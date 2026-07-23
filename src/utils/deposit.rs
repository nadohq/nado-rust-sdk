use crate::bindings::mock_erc20::MockERC20;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::core::execute::NadoExecute;
use crate::provider::{
    signer_provider_from_signer_sync, wait_for_transaction_receipt, NadoProvider,
};
use crate::revert::parse_alloy_error;
use crate::utils::constants::DEFAULT_PENDING_TX_RETIRES;
use alloy::rpc::types::TransactionReceipt;
use alloy_primitives::Address;
use eyre::{eyre, Result};
use std::time::Duration;

pub async fn deposit_collateral<V: NadoExecute + Sync>(
    nado: &V,
    deposit_collateral_params: DepositCollateralParams,
) -> Result<Option<TransactionReceipt>> {
    let amount = deposit_collateral_params.amount;
    let product_id = deposit_collateral_params.product_id;

    if deposit_collateral_params.mints_tokens {
        nado.mint_mock_erc20(product_id, amount).await?;
        if let Some(sleep_secs) = deposit_collateral_params.erc20_sleep_secs {
            println!("sleeping for {sleep_secs}s (erc20)");
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    if deposit_collateral_params.approves_allowance {
        nado.approve_with_gas_price(
            nado.endpoint_addr(),
            product_id,
            amount,
            deposit_collateral_params.gas_price,
        )
        .await?;
        if let Some(sleep_secs) = deposit_collateral_params.erc20_sleep_secs {
            println!("sleeping for {sleep_secs}s (erc20)");
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    endpoint_deposit_call(nado, &deposit_collateral_params).await
}

pub async fn erc20_client<V: NadoExecute + Sync>(
    nado: &V,
    product_id: u32,
) -> Result<MockERC20::MockERC20Instance<NadoProvider>> {
    let token_address = nado.get_token_address(product_id).await?;
    let provider = signer_provider_from_signer_sync(&nado.node_url(), nado.wallet()?.clone())?;
    Ok(MockERC20::new(Address::from(token_address), provider))
}

pub async fn endpoint_deposit_call<V: NadoExecute>(
    nado: &V,
    deposit_collateral_params: &DepositCollateralParams,
) -> Result<Option<TransactionReceipt>> {
    let product_id = deposit_collateral_params.product_id;
    let amount = deposit_collateral_params.amount;
    let subaccount = deposit_collateral_params.subaccount;

    let endpoint = nado.endpoint()?;
    // `depositCollateral` and `depositCollateralWithReferral` produce distinct alloy
    // `CallBuilder` types, so each branch sends independently.
    let pending_tx = if let Some(referral_code) = deposit_collateral_params.referral_code.clone() {
        let mut call = endpoint.depositCollateralWithReferral(
            subaccount.into(),
            product_id,
            amount,
            referral_code,
        );
        if let Some(gas_price) = deposit_collateral_params.gas_price {
            call = call.gas_price(gas_price);
        }
        call.send()
            .await
            .map_err(|e| eyre!(parse_alloy_error(&e)))?
    } else {
        let mut subaccount_name = [0u8; 12];
        subaccount_name[..12].copy_from_slice(&subaccount[20..]);
        let mut call = endpoint.depositCollateral(subaccount_name.into(), product_id, amount);
        if let Some(gas_price) = deposit_collateral_params.gas_price {
            call = call.gas_price(gas_price);
        }
        call.send()
            .await
            .map_err(|e| eyre!(parse_alloy_error(&e)))?
    };
    println!("pending tx: {:?}", pending_tx.tx_hash());
    wait_for_transaction_receipt(
        endpoint.provider(),
        *pending_tx.tx_hash(),
        DEFAULT_PENDING_TX_RETIRES,
    )
    .await
}
