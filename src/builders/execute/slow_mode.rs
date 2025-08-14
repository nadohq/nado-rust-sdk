use crate::bindings::endpoint::endpoint;
use crate::tx::TxType;
use ethers::types::Bytes;
use eyre::Result;
use std::time::Duration;

use crate::core::execute::NadoExecute;
use crate::utils::client_error::none_error;
use crate::utils::constants::DEFAULT_SLOW_MODE_SLEEP_SECS;
use crate::{build_and_call, fields_to_vars, nado_builder};
use ethers::abi::AbiEncode;
use ethers::types::TransactionReceipt;

nado_builder!(
    SubmitSlowModeTxBuilder,
    NadoExecute,
    tx: Bytes,
    mints_fee: bool,
    approves_fee: bool,
    sleep_secs: u64,
    erc20_sleep_secs: u64,
    gas_price: u128;

    pub async fn execute_and_sleep(&self) -> Result<Option<TransactionReceipt>> {
        let tx_hash = self.execute().await?;
        self.sleep().await;
        Ok(tx_hash)
    }

    build_and_call!(self, execute, submit_slow_mode_tx => Option<TransactionReceipt>);

    async fn sleep(&self) {
        let sleep_secs = self.sleep_secs.unwrap_or(DEFAULT_SLOW_MODE_SLEEP_SECS);
        if self.nado.is_rest_client() {
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }


    pub fn withdraw_collateral_tx(&self, withdraw_collateral: endpoint::WithdrawCollateral) -> Self {
        let tx = withdraw_collateral_bytes(withdraw_collateral);
        self.tx(tx)
    }


    pub fn liquidate_subaccount_tx(&self, liquidate_subaccount_tx: endpoint::LiquidateSubaccount) -> Self {
        let tx = liquidate_subaccount_bytes(liquidate_subaccount_tx);
        self.tx(tx)
    }

    pub fn build(&self) -> Result<SubmitSlowModeTxParams> {
        fields_to_vars!(self, (tx: clone));
        let mints_fee = self.mints_fee.unwrap_or(false);
        let approves_fee = self.approves_fee.unwrap_or(true);

        Ok(SubmitSlowModeTxParams {
            tx,
            mints_fee,
            approves_fee,
            erc20_sleep_secs: self.erc20_sleep_secs,
            gas_price: self.gas_price,
        })
    }

);

pub struct SubmitSlowModeTxParams {
    pub tx: Bytes,
    pub mints_fee: bool,
    pub approves_fee: bool,
    pub erc20_sleep_secs: Option<u64>,
    pub gas_price: Option<u128>,
}

fn withdraw_collateral_bytes(withdraw_collateral: endpoint::WithdrawCollateral) -> Bytes {
    let withdraw_tx_bytes = AbiEncode::encode(endpoint::UnsignedWithdrawCollateralReturn(
        withdraw_collateral,
    ));

    Bytes::from([vec![TxType::WithdrawCollateral as u8], withdraw_tx_bytes].concat())
}

fn liquidate_subaccount_bytes(liquidate_subaccount: endpoint::LiquidateSubaccount) -> Bytes {
    let liquidate_subaccount_return =
        endpoint::UnsignedLiquidateSubaccountReturn(liquidate_subaccount);

    Bytes::from(
        [
            vec![TxType::LiquidateSubaccount as u8],
            AbiEncode::encode(liquidate_subaccount_return),
        ]
        .concat(),
    )
}
