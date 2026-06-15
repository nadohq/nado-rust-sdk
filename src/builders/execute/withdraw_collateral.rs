use ethers::types::H160;
use eyre::Result;

use crate::bindings::endpoint;
use crate::eip712_structs;

use crate::core::execute::NadoExecute;
use crate::utils::client_error::none_error;
use crate::{fields_to_vars, nado_builder};

nado_builder!(
    WithdrawCollateralBuilder,
    NadoExecute,
    amount: u128,
    product_id: u32,
    nonce: u64,
    linked_sender: [u8; 32],
    send_to: H160,
    appendix: u128,
    spot_leverage: bool;

    // we do not use macro here because of extra required argument
    pub async fn execute(&self) -> Result<()> {
        if self.send_to.is_some() || self.appendix.is_some() {
            self.nado
                .withdraw_collateral_v2(self.build_v2().await?, self.spot_leverage)
                .await
        } else {
            self.nado
                .withdraw_collateral(self.build().await?, self.spot_leverage)
                .await
        }
    }

    pub async fn build_endpoint_tx(&self) -> Result<endpoint::WithdrawCollateral> {
        let tx = self.build().await?;
        Ok(
            endpoint::WithdrawCollateral {
                sender: tx.sender,
                amount: tx.amount,
                nonce: tx.nonce,
                product_id: tx.productId,
            }
        )
    }

    pub async fn build_endpoint_tx_v2(&self) -> Result<endpoint::WithdrawCollateralV2> {
        let tx = self.build_v2().await?;
        Ok(
            endpoint::WithdrawCollateralV2 {
                sender: tx.sender,
                amount: tx.amount,
                nonce: tx.nonce,
                product_id: tx.productId,
                send_to: H160::from_slice(&tx.sendTo),
                appendix: tx.appendix,
            }
        )
    }

    pub async fn build(&self) -> Result<eip712_structs::WithdrawCollateral> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let address = H160::from_slice(&sender[0..20]).0;
        let nonce = self
            .nonce
            .unwrap_or(self.nado.next_tx_nonce(address).await?);
        fields_to_vars!(self, amount, product_id);

        Ok(eip712_structs::WithdrawCollateral {
            sender,
            amount,
            nonce,
            productId: product_id,
        })
    }

    pub async fn build_v2(&self) -> Result<eip712_structs::WithdrawCollateralV2> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let address = H160::from_slice(&sender[0..20]).0;
        let nonce = self
            .nonce
            .unwrap_or(self.nado.next_tx_nonce(address).await?);
        let appendix = self.appendix.unwrap_or(0);
        let send_to = self.send_to.unwrap_or_else(H160::zero);
        fields_to_vars!(self, amount, product_id);

        Ok(eip712_structs::WithdrawCollateralV2 {
            sender,
            amount,
            nonce,
            productId: product_id,
            sendTo: send_to.0,
            appendix,
        })
    }
);
