use ethers_core::types::{Bytes as EthersBytes, H256};
use eyre::{eyre, Result};

use crate::bindings::endpoint;
use crate::eip712_structs;
use crate::engine::{PlaceOrder, PlaceOrderResponse};
use crate::math::ONE_X12;
use crate::trigger::{PlaceTriggerOrder, TriggerCriteria};
use crate::tx::get_eip712_digest;
use crate::utils::client_error::none_error;

use crate::core::execute::NadoExecute;
use crate::utils::nonce::order_nonce;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    PlaceOrderBuilder,
    NadoExecute,
    product_id: u32,
    price_x18: i128,
    linked_sender: [u8; 32],
    amount: i128,
    expiration: u64,
    order_type: eip712_structs::OrderType,
    reduce_only: bool,
    trigger_criteria: TriggerCriteria,
    times: u32,
    slippage_x6: u32,
    nonce: u64,
    appendix: u128,
    recv_time: u64,
    isolated: bool,
    margin: i128,
    spot_leverage: bool,
    borrow_margin: bool,
    mock_digest_and_signature: bool,
    builder_info: (u32, u32),
    id: u64;


    build_and_call!(self, execute, place_order => Option<PlaceOrderResponse>);

    pub async fn execute_trigger(&self) -> Result<Option<PlaceOrderResponse>> {
        self.nado.place_trigger_order(self.build_trigger()?).await
    }

    pub fn build(&self) -> Result<PlaceOrder> {
        self.assert_trigger_unset()?;
        let order = self.order()?;
        let id = self.id;
        let signature = self.get_signature(self.get_product_id()?, &order)?;
        let digest = self.get_digest(&order)?;

        Ok(PlaceOrder {
            order,
            signature,
            product_id: self.get_product_id()?,
            digest: Some(digest),
            id,
            spot_leverage: self.spot_leverage,
            borrow_margin: self.borrow_margin,
        })
    }

    pub fn build_trigger(&self) -> Result<PlaceTriggerOrder> {
        let order = self.order()?;
        let product_id = self.get_product_id()?;

        let trigger = self
            .trigger_criteria
            .clone()
            .ok_or(none_error("trigger_criteria"))?;
        let signature = self.get_signature(product_id, &order)?;
        let digest = self.get_digest(&order)?;
        Ok(PlaceTriggerOrder {
            order,
            signature: EthersBytes::from(signature),
            product_id,
            digest: Some(H256::from(digest)),
            spot_leverage: self.spot_leverage,
            borrow_margin: self.borrow_margin,
            trigger,
            id: self.id
        })
    }

    fn get_signature(&self, product_id: u32, order: &eip712_structs::Order) -> Result<Vec<u8>> {
        if self.should_mock_digest_and_signature() {
            let mut signature = vec![0; 65];
            signature.extend(self.nado.address()?);
            Ok(signature)
        } else {
            self.nado.signer().order_signature(product_id, order)
        }
    }

    fn get_digest(&self, order: &eip712_structs::Order) -> Result<[u8; 32]> {
        if self.should_mock_digest_and_signature() {
            Ok(random_digest())
        } else {
            self.encode_digest(order)
        }
    }

    fn encode_digest(&self, order: &eip712_structs::Order) -> Result<[u8; 32]> {
        let domain = self.nado.signer().order_domain(self.get_product_id()?)?;
        let encoded = get_eip712_digest(order, &domain);
        Ok(encoded)
    }

    fn assert_trigger_unset(&self) -> Result<()> {
        if self.trigger_criteria.is_some() {
            Err(eyre!("trigger_criteria set, use .build_triger to build trigger orders or clear trigger criteria"))
        } else {
            Ok(())
        }
    }

    pub fn order(&self) -> Result<eip712_structs::Order> {
        let mut builder = self.clone();

        if self.expiration.is_none() {
            builder = builder.expiration(u32::MAX as u64);
        }

        if self.order_type.is_none() {
            builder = builder.order_type(eip712_structs::OrderType::Default);
        }

        if self.reduce_only.is_none() {
            builder = builder.reduce_only(false);
        }

        builder.order_inner()
    }

    fn order_inner(&self) -> Result<eip712_structs::Order> {
        fields_to_vars!(self, amount, price_x18, reduce_only, (order_type: clone));

        let expiration = self.expiration.ok_or(none_error("expiration"))?;
        let mut appendix = self.appendix.unwrap_or(ORDER_VERSION as u128);

        if reduce_only {
            appendix |= 1 << 11;
        }
        if self.isolated.unwrap_or(false) {
            appendix |= 1 << 8;
            let margin = self.margin.ok_or(none_error("margin"))?;
            let margin_x6 = margin / ONE_X12;
            appendix |= (margin_x6 as u128) << 64;
        }
        appendix |= order_type.appendix_bit();
        let nonce = self.nonce.unwrap_or(order_nonce(self.recv_time));

        if let Some(trigger) = self.trigger_criteria.as_ref() {
            match trigger {
                TriggerCriteria::PriceTrigger {..} => { appendix |= 1 << 12;},
                TriggerCriteria::TimeTrigger {amounts, ..} => {
                    assert!(!self.isolated.unwrap_or(false), "time trigger orders cannot be isolated");
                    let times = self.times.ok_or(none_error("times"))?;
                    let slippage_x6 = self.slippage_x6.ok_or(none_error("slippage_x6"))?;

                    appendix |= (times as u128) << 96;
                    appendix |= (slippage_x6  as u128) << 64;
                    appendix |= match amounts {
                        Some(_) => 3 << 12,
                        None => 2 << 12,
                    };
                }
            }

        };
        if let Some((builder_id, builder_fee_rate)) = self.builder_info {
            appendix |= (builder_id as u128) << 48;
            appendix |= (builder_fee_rate as u128) << 38;
        }

        let sender = match self.linked_sender {
            Some(sender) => sender,
            None => self.nado.subaccount()?,
        };

        Ok(eip712_structs::Order::from_binding(&endpoint::Order {
            sender: sender.into(),
            price_x18,
            amount,
            expiration,
            nonce,
            appendix,
        }))
    }

    fn get_product_id(&self) -> Result<u32> {
        fields_to_vars!(self, product_id);
        Ok(product_id)
    }

    fn should_mock_digest_and_signature(&self) -> bool {
        self.mock_digest_and_signature.unwrap_or(false)
    }
);

fn random_digest() -> [u8; 32] {
    let mut arr = [0u8; 32];
    rand::Rng::fill(&mut rand::thread_rng(), &mut arr[..]);
    arr
}

const ORDER_VERSION: u8 = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eip712_structs::OrderType;
    use crate::nado_client::NadoClient;
    use crate::prelude::ClientMode;

    #[test]
    fn test_isolated_order_missing_margin_returns_error() {
        let client = NadoClient::new(ClientMode::Test);
        let builder = PlaceOrderBuilder::new(&client)
            .product_id(1)
            .amount(100)
            .price_x18(2000)
            .expiration(1000)
            .reduce_only(false)
            .order_type(OrderType::Default)
            .linked_sender([1u8; 32])
            .isolated(true);

        // Missing margin should return Err, not panic
        let result = builder.order_inner();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "margin is None!");
    }

    #[test]
    fn test_isolated_order_with_margin_succeeds() {
        let client = NadoClient::new(ClientMode::Test);
        let builder = PlaceOrderBuilder::new(&client)
            .product_id(1)
            .amount(100)
            .price_x18(2000)
            .expiration(1000)
            .reduce_only(false)
            .order_type(OrderType::Default)
            .linked_sender([1u8; 32])
            .isolated(true)
            .margin(10_000_000_000_000); // 10 * 10^12

        let order = builder.order_inner().unwrap();
        // Bit 8 must be set for isolated
        assert_ne!(order.appendix & (1 << 8), 0);
        // Margin shifted by 64
        let expected_margin_x6 = 10_000_000_000_000u128 / ONE_X12 as u128;
        assert_eq!((order.appendix >> 64) & 0xFFFF_FFFF, expected_margin_x6);
    }
}

