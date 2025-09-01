use crate::bindings::endpoint::Endpoint;
use crate::bindings::mock_erc20::MockERC20;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::core::execute::NadoExecute;
use crate::provider::NadoProvider;
use crate::revert::parse_provider_error;
use crate::utils::constants::DEFAULT_PENDING_TX_RETIRES;
use ethers_contract::ContractError;
use ethers_core::types::TransactionReceipt;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Provider;
use eyre::{eyre, Result};
use std::sync::Arc;
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
            println!("sleeping for {}s (erc20)", sleep_secs);
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
            println!("sleeping for {}s (erc20)", sleep_secs);
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    endpoint_deposit_call(nado, &deposit_collateral_params).await
}

pub fn provider_with_signer<V: NadoExecute>(nado: &V) -> Result<Arc<NadoProvider>> {
    let provider = Provider::new_client(&nado.node_url(), 15, 500)?;
    let wallet = nado.wallet()?.clone();
    Ok(Arc::new(SignerMiddleware::new(
        provider.interval(Duration::from_millis(500)),
        wallet,
    )))
}

pub async fn erc20_client<V: NadoExecute + Sync>(
    nado: &V,
    product_id: u32,
) -> Result<MockERC20<NadoProvider>> {
    let token_address = nado.get_token_address(product_id).await?;
    let client = provider_with_signer(nado)?;
    let remote_quote = MockERC20::new(token_address, client.clone());
    Ok(remote_quote)
}

pub async fn endpoint_deposit_call<V: NadoExecute>(
    nado: &V,
    deposit_collateral_params: &DepositCollateralParams,
) -> Result<Option<TransactionReceipt>> {
    let product_id = deposit_collateral_params.product_id;
    let amount = deposit_collateral_params.amount;
    let subaccount = deposit_collateral_params.subaccount;

    let endpoint: Endpoint<NadoProvider> = nado.endpoint()?;
    let mut tx = if let Some(referral_code) = deposit_collateral_params.referral_code.clone() {
        endpoint.deposit_collateral_with_referral(subaccount, product_id, amount, referral_code)
    } else {
        let mut subaccount_name = [0u8; 12];
        subaccount_name[..12].copy_from_slice(&subaccount[20..]);
        endpoint.deposit_collateral(subaccount_name, product_id, amount)
    };

    if let Some(gas_price) = deposit_collateral_params.gas_price {
        tx = tx.gas_price(gas_price);
    }

    let pending_tx = tx.send().await;
    let pending_tx = match pending_tx {
        Ok(tx) => tx,
        Err(e) => return Err(eyre!(parse_provider_error(e))),
    };
    let tx_receipt = pending_tx
        .retries(DEFAULT_PENDING_TX_RETIRES)
        .log_msg("pending tx")
        .await;
    match tx_receipt {
        Ok(receipt) => Ok(receipt),
        Err(e) => Err(eyre!(parse_provider_error(
            ContractError::<NadoProvider>::ProviderError { e }
        ))),
    }
}
