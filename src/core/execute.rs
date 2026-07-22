use alloy::rpc::types::TransactionReceipt;
use alloy_primitives::{Address, U256};
use async_trait::async_trait;
use ethers_core::types::Bytes as EthersBytes;
use eyre::{eyre, Result};

use crate::bindings::endpoint;
use crate::bindings::querier::Querier;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::builders::execute::slow_mode::SubmitSlowModeTxParams;
use crate::core::query::NadoQuery;
use crate::eip712_structs::{
    BurnNlp, Cancellation, CancellationProducts, DependencyUpdate, LinkSigner, LiquidateSubaccount,
    MintNlp, TransferQuote, WithdrawCollateral, WithdrawCollateralV2,
};
use crate::engine::{
    CancelOrdersResponse, Execute, ExecuteResponseData, PlaceOrder, PlaceOrderResponse,
};
use crate::provider::{
    signer_provider_from_signer_sync, wait_for_transaction_receipt, NadoProvider,
};
use crate::trigger;
use crate::utils::constants::ETHERS_DEFAULT_PENDING_TX_RETRIES;
use crate::utils::deposit::erc20_client;
use crate::utils::nonce::order_nonce;
use crate::utils::response::match_cancel_orders_response;

macro_rules! map_response_type {
    ($response_data:expr, $enum_variant:path => $output_type:ty) => {
        match $response_data {
            Some(data) => match data {
                $enum_variant(response) => Ok(Some(response as $output_type)),
                _ => Err(eyre!("Unexpected response type")),
            },
            None => Ok(None),
        }
    };
}

#[async_trait]
pub trait NadoExecute: NadoQuery {
    async fn execute(&self, execute: Execute) -> Result<Option<ExecuteResponseData>>;

    async fn execute_trigger(
        &self,
        execute: trigger::Execute,
    ) -> Result<Option<ExecuteResponseData>>;

    async fn submit_slow_mode_tx(
        &self,
        params: SubmitSlowModeTxParams,
    ) -> Result<Option<TransactionReceipt>>;

    async fn place_order(&self, place_order: PlaceOrder) -> Result<Option<PlaceOrderResponse>> {
        let execute = Execute::PlaceOrder(place_order);
        let execute_response_data = self.execute(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn place_trigger_order(
        &self,
        place_trigger_order: trigger::PlaceTriggerOrder,
    ) -> Result<Option<PlaceOrderResponse>> {
        let execute = trigger::Execute::PlaceOrder(place_trigger_order);
        let execute_response_data = self.execute_trigger(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn cancel_orders(&self, tx: Cancellation) -> Result<Option<CancelOrdersResponse>> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::CancelOrders {
            tx,
            signature,
            required_unfilled_amount: None,
            id: None,
        };
        let execute_response_data = self.execute(execute).await?;
        match_cancel_orders_response(execute_response_data)
    }

    async fn cancel_trigger_orders(&self, tx: Cancellation) -> Result<()> {
        let signature: EthersBytes = self.endpoint_signature(&tx)?.into();
        let execute = trigger::Execute::CancelOrders { tx, signature };
        self.execute_trigger(execute).await?;
        Ok(())
    }

    async fn cancel_product_orders(
        &self,
        tx: CancellationProducts,
    ) -> Result<Option<CancelOrdersResponse>> {
        let signature = self.endpoint_signature(&tx)?;
        let digest = Some(self.signer().endpoint_digest(&tx)?);
        let execute = Execute::CancelProductOrders {
            tx,
            signature,
            digest,
            id: None,
        };
        let execute_response_data = self.execute(execute).await?;
        match_cancel_orders_response(execute_response_data)
    }

    async fn cancel_product_trigger_orders(&self, tx: CancellationProducts) -> Result<()> {
        let signature: EthersBytes = self.endpoint_signature(&tx)?.into();
        let execute = trigger::Execute::CancelProductOrders { tx, signature };
        self.execute_trigger(execute).await?;
        Ok(())
    }

    async fn update_dependency(
        &self,
        old_digest: [u8; 32],
        new_digest: [u8; 32],
    ) -> Result<Option<PlaceOrderResponse>> {
        let tx = DependencyUpdate {
            sender: self.subaccount()?,
            oldDigest: old_digest,
            newDigest: new_digest,
            nonce: order_nonce(None),
        };
        let signature: EthersBytes = self.endpoint_signature(&tx)?.into();
        let execute = trigger::Execute::UpdateDependency { tx, signature };
        let execute_response_data = self.execute_trigger(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn cancel_and_place(
        &self,
        cancel_tx: Cancellation,
        place_order: PlaceOrder,
        place_requires_unfilled: bool,
    ) -> Result<Option<PlaceOrderResponse>> {
        let cancel_signature = self.endpoint_signature(&cancel_tx)?;
        let execute = Execute::CancelAndPlace {
            cancel_tx,
            cancel_signature,
            place_order,
            place_requires_unfilled: Some(place_requires_unfilled),
            required_unfilled_amount: None,
        };
        let execute_response_data = self.execute(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn liquidate_subaccount(&self, tx: LiquidateSubaccount) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::LiquidateSubaccount { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn withdraw_collateral(
        &self,
        tx: WithdrawCollateral,
        spot_leverage: Option<bool>,
    ) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::WithdrawCollateral {
            tx,
            signature,
            spot_leverage,
            sequencer_risk_check: None,
        };
        self.execute(execute).await?;
        Ok(())
    }

    async fn withdraw_collateral_v2(
        &self,
        tx: WithdrawCollateralV2,
        spot_leverage: Option<bool>,
    ) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::WithdrawCollateralV2 {
            tx,
            signature,
            spot_leverage,
            sequencer_risk_check: None,
        };
        self.execute(execute).await?;
        Ok(())
    }

    async fn mint_nlp(&self, tx: MintNlp, spot_leverage: Option<bool>) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::MintNlp {
            tx,
            signature,
            spot_leverage,
        };
        self.execute(execute).await?;
        Ok(())
    }

    async fn burn_nlp(&self, tx: BurnNlp) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::BurnNlp { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn transfer_quote(&self, tx: TransferQuote) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::TransferQuote { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn link_signer(&self, tx: LinkSigner) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::LinkSigner { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn deposit_collateral(
        &self,
        deposit_collateral_params: DepositCollateralParams,
    ) -> Result<Option<TransactionReceipt>>;

    async fn approve_endpoint_allowance(
        &self,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        self.approve(self.endpoint_addr(), product_id, amount).await
    }

    #[deprecated(
        since = "0.2.2",
        note = "please use `approve_endpoint_allowance` or `approve` instead"
    )]
    async fn approve_allowance(
        &self,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        self.approve(self.endpoint_addr(), product_id, amount).await
    }

    async fn approve(
        &self,
        spender: Address,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        self.approve_with_gas_price(spender, product_id, amount, None)
            .await
    }

    async fn approve_with_gas_price(
        &self,
        spender: Address,
        product_id: u32,
        amount: u128,
        gas_price: Option<u128>,
    ) -> Result<Option<TransactionReceipt>> {
        let erc20_client = erc20_client(self, product_id).await?;
        let mut call = erc20_client.approve(spender, U256::from(amount));
        if let Some(price) = gas_price {
            call = call.gas_price(price);
        }
        let pending_tx = call.send().await?;
        let receipt = wait_for_transaction_receipt(
            erc20_client.provider(),
            *pending_tx.tx_hash(),
            ETHERS_DEFAULT_PENDING_TX_RETRIES,
        )
        .await?;
        println!("approved: {amount} of product id: {product_id}");
        Ok(receipt)
    }

    async fn get_token_allowance(&self, product_id: u32) -> Result<U256> {
        let erc20_client = erc20_client(self, product_id).await?;
        let owner = self.wallet()?.address();
        let allowance = erc20_client
            .allowance(owner, self.endpoint_addr())
            .call()
            .await?;
        Ok(allowance)
    }

    async fn get_token_balance(&self, product_id: u32) -> Result<U256> {
        let erc20_client = erc20_client(self, product_id).await?;
        let balance = erc20_client
            .balanceOf(self.wallet()?.address())
            .call()
            .await?;
        Ok(balance)
    }

    async fn get_token_decimals(&self, product_id: u32) -> Result<u8> {
        let erc20_client = erc20_client(self, product_id).await?;
        Ok(erc20_client.decimals().call().await?)
    }

    async fn mint_mock_erc20(
        &self,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        let erc20_client = erc20_client(self, product_id).await?;
        let pending_tx = erc20_client
            .mint(self.wallet()?.address(), U256::from(amount))
            .send()
            .await?;
        let receipt = wait_for_transaction_receipt(
            erc20_client.provider(),
            *pending_tx.tx_hash(),
            ETHERS_DEFAULT_PENDING_TX_RETRIES,
        )
        .await?;
        println!("minted: {amount} of product id: {product_id}");
        Ok(receipt)
    }

    fn endpoint(&self) -> Result<endpoint::EndpointInstance<NadoProvider>> {
        let provider = signer_provider_from_signer_sync(&self.node_url(), self.wallet()?.clone())?;
        Ok(endpoint::Endpoint::new(self.endpoint_addr(), provider))
    }

    fn querier(&self) -> Result<Querier::QuerierInstance<NadoProvider>> {
        let provider = signer_provider_from_signer_sync(&self.node_url(), self.wallet()?.clone())?;
        Ok(Querier::new(self.querier_addr(), provider))
    }
}
