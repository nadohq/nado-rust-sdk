// The provider constructors stay `async fn` to preserve the `.await` call sites
// across the SDK even though alloy's `connect_client` is synchronous.
#![allow(clippy::unused_async)]

use alloy::consensus::BlockHeader;
use alloy::eips::BlockNumberOrTag;
use alloy::network::{BlockResponse, EthereumWallet, Network, TransactionBuilder};
use alloy::primitives::{Address, B256};
use alloy::providers::fillers::{FillerControlFlow, NonceManager, TxFiller};
use alloy::providers::utils::Eip1559Estimation;
use alloy::providers::{DynProvider, Provider, ProviderBuilder, SendableTx};
use alloy::rpc::client::RpcClient;
use alloy::rpc::json_rpc::{ErrorPayload, RequestPacket, ResponsePacket, RpcError};
use alloy::rpc::types::TransactionReceipt;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use alloy::transports::layers::{RetryBackoffLayer, RetryPolicy};
use alloy::transports::{TransportError, TransportErrorKind, TransportFut, TransportResult};
use async_trait::async_trait;
use eyre::{eyre, Result};
use serde_json::{from_str, Value};
use std::future::IntoFuture;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;
use tower_service::Service;

pub type NadoProvider = DynProvider;
type HttpTransport = Http<Client>;

const MAX_RETRIES: u32 = 15;
const RETRY_BACKOFF_MS: u64 = 500;
const RETRY_CUPS: u64 = 330;
const PENDING_TX_POLL_INTERVAL: Duration = Duration::from_millis(500);

fn rate_limit_retry_layer() -> RetryBackoffLayer<EthersRateLimitRetryPolicy> {
    RetryBackoffLayer::new_with_policy(
        MAX_RETRIES,
        RETRY_BACKOFF_MS,
        RETRY_CUPS,
        EthersRateLimitRetryPolicy,
    )
}

#[derive(Clone, Copy, Debug, Default)]
struct EthersRateLimitRetryPolicy;

impl RetryPolicy for EthersRateLimitRetryPolicy {
    fn should_retry(&self, error: &TransportError) -> bool {
        match error {
            RpcError::Transport(TransportErrorKind::HttpError(http_error)) => {
                http_error.status == 429
            }
            RpcError::ErrorResp(payload) => is_ethers_rate_limit_error(payload),
            RpcError::DeserError { text, .. } => parse_error_payload(text)
                .is_some_and(|payload| is_ethers_rate_limit_error(&payload)),
            _ => false,
        }
    }

    fn backoff_hint(&self, error: &TransportError) -> Option<Duration> {
        let RpcError::ErrorResp(payload) = error else {
            return None;
        };
        let data = payload.try_data_as::<Value>()?.ok()?;
        let backoff_seconds = &data["rate"]["backoff_seconds"];
        if let Some(seconds) = backoff_seconds.as_u64() {
            Some(Duration::from_secs(seconds))
        } else {
            backoff_seconds
                .as_f64()
                .map(|seconds| Duration::from_secs(seconds as u64 + 1))
        }
    }
}

fn is_ethers_rate_limit_error(payload: &ErrorPayload) -> bool {
    payload.code == 429
        || payload.code == -32005
        || (payload.code == -32016 && payload.message.contains("rate limit"))
        || matches!(
            payload.message.as_ref(),
            "header not found" | "daily request count exceeded, request rate limited"
        )
}

fn parse_error_payload(text: &str) -> Option<ErrorPayload> {
    #[derive(serde::Deserialize)]
    struct Resp {
        error: ErrorPayload,
    }

    from_str::<Resp>(text).map(|resp| resp.error).ok()
}

#[derive(Clone, Copy, Debug, Default)]
struct PendingNonceManager;

#[async_trait]
impl NonceManager for PendingNonceManager {
    async fn get_next_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        P: Provider<N>,
        N: Network,
    {
        provider.get_transaction_count(address).pending().await
    }
}

const ETHERS_EIP1559_FEE_ESTIMATION_PAST_BLOCKS: u64 = 10;
const ETHERS_EIP1559_FEE_ESTIMATION_REWARD_PERCENTILE: f64 = 5.0;
const ETHERS_EIP1559_FEE_ESTIMATION_DEFAULT_PRIORITY_FEE: u128 = 3_000_000_000;
const ETHERS_EIP1559_FEE_ESTIMATION_PRIORITY_FEE_TRIGGER: u128 = 100_000_000_000;
const ETHERS_EIP1559_FEE_ESTIMATION_THRESHOLD_MAX_CHANGE: i128 = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EthersGasFillable {
    Legacy {
        gas_limit: u64,
        gas_price: u128,
    },
    Eip1559 {
        gas_limit: u64,
        max_fee_per_gas: u128,
        max_priority_fee_per_gas: u128,
    },
}

#[derive(Clone, Copy, Debug, Default)]
struct EthersGasFiller;

impl EthersGasFiller {
    async fn prepare_legacy<P, N>(
        &self,
        provider: &P,
        tx: &N::TransactionRequest,
    ) -> TransportResult<EthersGasFillable>
    where
        P: Provider<N>,
        N: Network,
    {
        let gas_price = match tx.gas_price() {
            Some(gas_price) => gas_price,
            None => provider.get_gas_price().await?,
        };
        let gas_limit = match tx.gas_limit() {
            Some(gas_limit) => gas_limit,
            None => {
                let mut tx_for_estimate = tx.clone();
                tx_for_estimate.set_gas_price(gas_price);
                provider.estimate_gas(tx_for_estimate).into_future().await?
            }
        };
        Ok(EthersGasFillable::Legacy {
            gas_limit,
            gas_price,
        })
    }

    async fn prepare_1559<P, N>(
        &self,
        provider: &P,
        tx: &N::TransactionRequest,
    ) -> TransportResult<EthersGasFillable>
    where
        P: Provider<N>,
        N: Network,
    {
        let (max_fee_per_gas, max_priority_fee_per_gas) =
            if let (Some(max_fee_per_gas), Some(max_priority_fee_per_gas)) =
                (tx.max_fee_per_gas(), tx.max_priority_fee_per_gas())
            {
                (max_fee_per_gas, max_priority_fee_per_gas)
            } else {
                let estimate = estimate_ethers_eip1559_fees(provider).await?;
                let max_fee_per_gas = tx.max_fee_per_gas().unwrap_or(estimate.max_fee_per_gas);
                let max_priority_fee_per_gas = tx
                    .max_priority_fee_per_gas()
                    .map(|tip| tip.min(max_fee_per_gas))
                    .unwrap_or(estimate.max_priority_fee_per_gas);
                (max_fee_per_gas, max_priority_fee_per_gas)
            };
        let gas_limit = match tx.gas_limit() {
            Some(gas_limit) => gas_limit,
            None => {
                let mut tx_for_estimate = tx.clone();
                tx_for_estimate.set_max_fee_per_gas(max_fee_per_gas);
                tx_for_estimate.set_max_priority_fee_per_gas(max_priority_fee_per_gas);
                provider.estimate_gas(tx_for_estimate).into_future().await?
            }
        };
        Ok(EthersGasFillable::Eip1559 {
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
        })
    }
}

impl<N> TxFiller<N> for EthersGasFiller
where
    N: Network,
{
    type Fillable = EthersGasFillable;

    fn status(&self, tx: &N::TransactionRequest) -> FillerControlFlow {
        if tx.gas_price().is_some() && tx.gas_limit().is_some() {
            return FillerControlFlow::Finished;
        }

        if tx.max_fee_per_gas().is_some()
            && tx.max_priority_fee_per_gas().is_some()
            && tx.gas_limit().is_some()
        {
            return FillerControlFlow::Finished;
        }

        FillerControlFlow::Ready
    }

    fn fill_sync(&self, _tx: &mut SendableTx<N>) {}

    async fn prepare<P>(
        &self,
        provider: &P,
        tx: &N::TransactionRequest,
    ) -> TransportResult<Self::Fillable>
    where
        P: Provider<N>,
    {
        if tx.gas_price().is_some() {
            self.prepare_legacy(provider, tx).await
        } else {
            match self.prepare_1559(provider, tx).await {
                Ok(estimate) => Ok(estimate),
                Err(RpcError::UnsupportedFeature(_)) => self.prepare_legacy(provider, tx).await,
                Err(err) => Err(err),
            }
        }
    }

    async fn fill(
        &self,
        fillable: Self::Fillable,
        mut tx: SendableTx<N>,
    ) -> TransportResult<SendableTx<N>> {
        if let Some(builder) = tx.as_mut_builder() {
            match fillable {
                EthersGasFillable::Legacy {
                    gas_limit,
                    gas_price,
                } => {
                    builder.set_gas_limit(gas_limit);
                    builder.set_gas_price(gas_price);
                }
                EthersGasFillable::Eip1559 {
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                } => {
                    builder.set_gas_limit(gas_limit);
                    builder.set_max_fee_per_gas(max_fee_per_gas);
                    builder.set_max_priority_fee_per_gas(max_priority_fee_per_gas);
                }
            }
        }
        Ok(tx)
    }
}

async fn estimate_ethers_eip1559_fees<P, N>(provider: &P) -> TransportResult<Eip1559Estimation>
where
    P: Provider<N>,
    N: Network,
{
    let base_fee_per_gas = provider
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .ok_or(RpcError::NullResp)?
        .header()
        .as_ref()
        .base_fee_per_gas()
        .ok_or(RpcError::UnsupportedFeature("eip1559"))?
        .into();
    let fee_history = provider
        .get_fee_history(
            ETHERS_EIP1559_FEE_ESTIMATION_PAST_BLOCKS,
            BlockNumberOrTag::Latest,
            &[ETHERS_EIP1559_FEE_ESTIMATION_REWARD_PERCENTILE],
        )
        .await?;
    Ok(ethers_eip1559_default_estimator(
        base_fee_per_gas,
        &fee_history.reward.unwrap_or_default(),
    ))
}

fn ethers_eip1559_default_estimator(
    base_fee_per_gas: u128,
    rewards: &[Vec<u128>],
) -> Eip1559Estimation {
    let max_priority_fee_per_gas =
        if base_fee_per_gas < ETHERS_EIP1559_FEE_ESTIMATION_PRIORITY_FEE_TRIGGER {
            ETHERS_EIP1559_FEE_ESTIMATION_DEFAULT_PRIORITY_FEE
        } else {
            estimate_ethers_priority_fee(rewards)
                .max(ETHERS_EIP1559_FEE_ESTIMATION_DEFAULT_PRIORITY_FEE)
        };
    let potential_max_fee = ethers_base_fee_surged(base_fee_per_gas);
    let max_fee_per_gas = if max_priority_fee_per_gas > potential_max_fee {
        max_priority_fee_per_gas + potential_max_fee
    } else {
        potential_max_fee
    };
    Eip1559Estimation {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    }
}

fn estimate_ethers_priority_fee(rewards: &[Vec<u128>]) -> u128 {
    let mut rewards = rewards
        .iter()
        .map(|reward| reward[0])
        .filter(|reward| *reward > 0)
        .collect::<Vec<_>>();
    if rewards.is_empty() {
        return 0;
    }
    if rewards.len() == 1 {
        return rewards[0];
    }

    rewards.sort_unstable();
    let mut rewards_copy = rewards.clone();
    rewards_copy.rotate_left(1);

    let mut percentage_change = rewards
        .iter()
        .zip(rewards_copy.iter())
        .map(|(a, b)| {
            let a = i128::try_from(*a).expect("priority fee overflow");
            let b = i128::try_from(*b).expect("priority fee overflow");
            ((b - a) * 100) / a
        })
        .collect::<Vec<_>>();
    percentage_change.pop();

    let max_change = percentage_change.iter().max().unwrap();
    let max_change_index = percentage_change
        .iter()
        .position(|change| change == max_change)
        .unwrap();
    let values = if *max_change >= ETHERS_EIP1559_FEE_ESTIMATION_THRESHOLD_MAX_CHANGE
        && max_change_index >= rewards.len() / 2
    {
        &rewards[max_change_index..]
    } else {
        &rewards
    };
    values[values.len() / 2]
}

fn ethers_base_fee_surged(base_fee_per_gas: u128) -> u128 {
    if base_fee_per_gas <= 40_000_000_000 {
        base_fee_per_gas * 2
    } else if base_fee_per_gas <= 100_000_000_000 {
        base_fee_per_gas * 16 / 10
    } else if base_fee_per_gas <= 200_000_000_000 {
        base_fee_per_gas * 14 / 10
    } else {
        base_fee_per_gas * 12 / 10
    }
}

#[derive(Clone, Debug)]
struct SequentialFallbackService<S> {
    transports: Vec<(Option<String>, S)>,
}

impl<S> SequentialFallbackService<S> {
    #[cfg(test)]
    fn new(transports: Vec<S>) -> Self {
        Self {
            transports: transports
                .into_iter()
                .map(|transport| (None, transport))
                .collect(),
        }
    }

    fn with_urls(transports: Vec<(String, S)>) -> Self {
        Self {
            transports: transports
                .into_iter()
                .map(|(url, transport)| (Some(url), transport))
                .collect(),
        }
    }
}

impl<S> Service<RequestPacket> for SequentialFallbackService<S>
where
    S: Service<
            RequestPacket,
            Response = ResponsePacket,
            Error = TransportError,
            Future = TransportFut<'static>,
        > + Clone
        + Send
        + Sync
        + 'static,
{
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        let transports = self.transports.clone();
        Box::pin(async move {
            let mut last_error: Option<TransportError> = None;
            for (idx, (url, mut transport)) in transports.into_iter().enumerate() {
                if idx != 0 {
                    if let Some(error) = &last_error {
                        let err_str = error.to_string();
                        if err_str.contains(": G ") {
                            break;
                        }
                        println!(
                            "http_ensemble: using backup node_url: {}\n{err_str}",
                            url.as_deref().unwrap_or("<unknown>")
                        );
                    }
                }
                match transport.call(req.clone()).await {
                    Ok(response) => {
                        if let Some(error) = response.as_error() {
                            let error = TransportError::ErrorResp(error.clone());
                            let should_stop = error.to_string().contains(": G ");
                            last_error = Some(error);
                            if should_stop {
                                break;
                            }
                            continue;
                        }
                        return Ok(response);
                    }
                    Err(error) => {
                        let should_stop = error.to_string().contains(": G ");
                        last_error = Some(error);
                        if should_stop {
                            break;
                        }
                    }
                }
            }
            Err(last_error
                .unwrap_or_else(|| TransportErrorKind::custom_str("list of providers was empty")))
        })
    }
}

fn build_client(node_urls: &[String]) -> Result<RpcClient> {
    if node_urls.len() == 1 {
        let url = &node_urls[0];
        return Ok(RpcClient::builder()
            .layer(rate_limit_retry_layer())
            .http(url.parse()?));
    }
    let service = build_fallback_service(node_urls)?;
    Ok(RpcClient::builder()
        .layer(rate_limit_retry_layer())
        .transport(service, false))
}

fn build_plain_client(node_urls: &[String]) -> Result<RpcClient> {
    if node_urls.len() == 1 {
        let url = &node_urls[0];
        return Ok(RpcClient::builder().http(url.parse()?));
    }
    Ok(RpcClient::builder().transport(build_fallback_service(node_urls)?, false))
}

fn build_fallback_service(
    node_urls: &[String],
) -> Result<SequentialFallbackService<HttpTransport>> {
    let mut transports = Vec::with_capacity(node_urls.len());
    for url in node_urls {
        transports.push((url.clone(), Http::new(url.parse()?)));
    }
    Ok(SequentialFallbackService::with_urls(transports))
}

pub async fn read_provider(node_url: &str) -> Result<NadoProvider> {
    let client = build_client(&[node_url.to_string()])?;
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client);
    Ok(provider.erased())
}

pub async fn read_provider_no_retry(node_url: &str) -> Result<NadoProvider> {
    let client = build_plain_client(&[node_url.to_string()])?;
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client);
    Ok(provider.erased())
}

pub fn read_provider_no_retry_sync(node_url: &str) -> Result<NadoProvider> {
    let client = build_plain_client(&[node_url.to_string()])?;
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client);
    Ok(provider.erased())
}

pub async fn read_ensemble_provider(node_urls: &[String]) -> Result<NadoProvider> {
    let client = build_client(node_urls)?;
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client);
    Ok(provider.erased())
}

pub async fn read_ensemble_provider_no_retry(node_urls: &[String]) -> Result<NadoProvider> {
    let client = build_plain_client(node_urls)?;
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client);
    Ok(provider.erased())
}

pub async fn signer_provider(node_url: &str, private_key: &str) -> Result<NadoProvider> {
    let signer: PrivateKeySigner = private_key.parse()?;
    signer_provider_inner_with_chain_id(&[node_url.to_string()], signer).await
}

pub async fn signer_provider_from_key(node_url: &str, key_bytes: &[u8]) -> Result<NadoProvider> {
    let signer = PrivateKeySigner::from_slice(key_bytes)?;
    signer_provider_inner_with_chain_id(&[node_url.to_string()], signer).await
}

pub async fn signer_provider_from_signer(
    node_url: &str,
    signer: PrivateKeySigner,
) -> Result<NadoProvider> {
    signer_provider_inner_from_signer(&[node_url.to_string()], signer).await
}

pub fn signer_provider_from_signer_sync(
    node_url: &str,
    signer: PrivateKeySigner,
) -> Result<NadoProvider> {
    let client = build_client(&[node_url.to_string()])?;
    signer_provider_inner_from_client_preserving_existing_chain_id(client, signer)
}

pub async fn signer_ensemble_provider(
    node_urls: &[String],
    private_key: &str,
) -> Result<NadoProvider> {
    let signer: PrivateKeySigner = private_key.parse()?;
    signer_provider_inner_with_chain_id(node_urls, signer).await
}

async fn signer_provider_inner_with_chain_id(
    node_urls: &[String],
    signer: PrivateKeySigner,
) -> Result<NadoProvider> {
    let client = build_client(node_urls)?;
    let chain_id = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_client(client.clone())
        .get_chain_id()
        .await?;
    Ok(signer_provider_inner_from_client(
        client,
        signer,
        Some(chain_id),
    ))
}

async fn signer_provider_inner_from_signer(
    node_urls: &[String],
    signer: PrivateKeySigner,
) -> Result<NadoProvider> {
    let client = build_client(node_urls)?;
    signer_provider_inner_from_client_preserving_signer_chain_id(client, signer).await
}

async fn signer_provider_inner_from_client_preserving_signer_chain_id(
    client: RpcClient,
    signer: PrivateKeySigner,
) -> Result<NadoProvider> {
    if signer.chain_id().is_some() {
        signer_provider_inner_from_client_preserving_existing_chain_id(client, signer)
    } else {
        let chain_id = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(client.clone())
            .get_chain_id()
            .await?;
        Ok(signer_provider_inner_from_client(
            client,
            signer,
            Some(chain_id),
        ))
    }
}

fn signer_provider_inner_from_client_preserving_existing_chain_id(
    client: RpcClient,
    signer: PrivateKeySigner,
) -> Result<NadoProvider> {
    let Some(chain_id) = signer.chain_id() else {
        return Err(eyre!(
            "signer is missing chain id; use signer_provider_from_signer(...).await to fetch it from the node"
        ));
    };
    Ok(signer_provider_inner_from_client(
        client,
        signer,
        Some(chain_id),
    ))
}

fn signer_provider_inner_from_client(
    client: RpcClient,
    signer: PrivateKeySigner,
    chain_id: Option<u64>,
) -> NadoProvider {
    let wallet = EthereumWallet::from(signer);
    let builder = ProviderBuilder::new()
        .disable_recommended_fillers()
        .filler(EthersGasFiller)
        .with_nonce_management(PendingNonceManager);
    match chain_id {
        Some(chain_id) => builder
            .with_chain_id(chain_id)
            .wallet(wallet)
            .connect_client(client)
            .erased(),
        None => builder.wallet(wallet).connect_client(client).erased(),
    }
}

pub async fn chain_id(provider: &NadoProvider) -> Result<u64> {
    Ok(provider.get_chain_id().await?)
}

/// Wait for a transaction receipt with the old ethers `PendingTransaction` semantics.
///
/// ethers first checks whether the tx is still known to the node. If it disappears
/// from the mempool before being mined, it returns `Ok(None)` after a small retry
/// budget. Polling RPC errors and missing receipts remain pending instead of consuming
/// that retry budget.
pub async fn wait_for_transaction_receipt(
    provider: &NadoProvider,
    tx_hash: B256,
    missing_tx_retries: usize,
) -> Result<Option<TransactionReceipt>> {
    wait_for_transaction_receipt_with_interval(
        provider,
        tx_hash,
        missing_tx_retries,
        PENDING_TX_POLL_INTERVAL,
    )
    .await
}

pub async fn wait_for_transaction_receipt_with_interval(
    provider: &NadoProvider,
    tx_hash: B256,
    missing_tx_retries: usize,
    poll_interval: Duration,
) -> Result<Option<TransactionReceipt>> {
    sleep(poll_interval).await;
    let mut retries_remaining = missing_tx_retries;
    loop {
        match provider.get_transaction_by_hash(tx_hash).await {
            Ok(Some(tx)) => {
                if tx.block_number.is_some() {
                    break;
                }
            }
            Ok(None) => {
                if retries_remaining == 0 {
                    return Ok(None);
                }
                retries_remaining -= 1;
            }
            Err(_e) => {}
        }
        sleep(poll_interval).await;
    }

    loop {
        match provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => return Ok(Some(receipt)),
            Ok(None) => {}
            Err(_e) => {}
        }
        sleep(poll_interval).await;
    }
}

#[cfg(test)]
mod tests {
    use super::{PendingNonceManager, SequentialFallbackService};
    use alloy::primitives::{Address, B256, U256};
    use alloy::providers::fillers::NonceManager;
    use alloy::providers::{Provider, ProviderBuilder};
    use alloy::rpc::client::RpcClient;
    use alloy::rpc::json_rpc::{
        ErrorPayload, Id, Request, RequestPacket, Response, ResponsePacket, ResponsePayload,
        RpcError,
    };
    use alloy::rpc::types::TransactionRequest;
    use alloy::signers::{local::PrivateKeySigner, Signer};
    use alloy::transports::layers::{RetryBackoffLayer, RetryPolicy};
    use alloy::transports::{TransportError, TransportErrorKind, TransportFut};
    use alloy_consensus::{Transaction, TxEnvelope};
    use alloy_rlp::Decodable;
    use serde_json::value::RawValue;
    use serde_json::{from_str, from_value, json, to_string, to_value, Value};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    };
    use std::task::{Context, Poll};
    use std::time::Duration;
    use tower_service::Service;

    #[derive(Clone, Debug)]
    struct MockTransport {
        calls: Arc<AtomicUsize>,
        requests: Arc<Mutex<Vec<String>>>,
        result: MockResult,
    }

    #[derive(Clone, Debug)]
    enum MockResult {
        Success(&'static str),
        RpcError(&'static str),
        RpcErrorWithCode(i64, &'static str),
        TransportError(&'static str),
    }

    impl MockTransport {
        fn success(value: &'static str) -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
                requests: Arc::new(Mutex::new(vec![])),
                result: MockResult::Success(value),
            }
        }

        fn rpc_error(message: &'static str) -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
                requests: Arc::new(Mutex::new(vec![])),
                result: MockResult::RpcError(message),
            }
        }

        fn rpc_error_with_code(code: i64, message: &'static str) -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
                requests: Arc::new(Mutex::new(vec![])),
                result: MockResult::RpcErrorWithCode(code, message),
            }
        }

        fn transport_error(message: &'static str) -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
                requests: Arc::new(Mutex::new(vec![])),
                result: MockResult::TransportError(message),
            }
        }

        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }

        fn request_jsons(&self) -> Vec<String> {
            self.requests.lock().unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    struct SendRawTxTransport {
        raw_txs: Arc<Mutex<Vec<String>>>,
        requests: Arc<Mutex<Vec<String>>>,
        transaction_count: &'static str,
    }

    impl Default for SendRawTxTransport {
        fn default() -> Self {
            Self::with_transaction_count("0x0")
        }
    }

    impl SendRawTxTransport {
        fn with_transaction_count(transaction_count: &'static str) -> Self {
            Self {
                raw_txs: Arc::new(Mutex::new(vec![])),
                requests: Arc::new(Mutex::new(vec![])),
                transaction_count,
            }
        }

        fn raw_txs(&self) -> Vec<String> {
            self.raw_txs.lock().unwrap().clone()
        }

        fn request_jsons(&self) -> Vec<String> {
            self.requests.lock().unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    struct PendingTxTransport {
        requests: Arc<Mutex<Vec<String>>>,
        result: PendingTxResult,
    }

    #[derive(Clone, Debug)]
    enum PendingTxResult {
        MissingTransaction,
        MinedMissingReceipt { tx_hash: B256 },
        MinedReceipt { tx_hash: B256 },
    }

    impl PendingTxTransport {
        fn missing_transaction() -> Self {
            Self {
                requests: Arc::new(Mutex::new(vec![])),
                result: PendingTxResult::MissingTransaction,
            }
        }

        fn mined_missing_receipt(tx_hash: B256) -> Self {
            Self {
                requests: Arc::new(Mutex::new(vec![])),
                result: PendingTxResult::MinedMissingReceipt { tx_hash },
            }
        }

        fn mined_receipt(tx_hash: B256) -> Self {
            Self {
                requests: Arc::new(Mutex::new(vec![])),
                result: PendingTxResult::MinedReceipt { tx_hash },
            }
        }

        fn request_methods(&self) -> Vec<String> {
            self.requests.lock().unwrap().clone()
        }
    }

    impl Service<RequestPacket> for PendingTxTransport {
        type Response = ResponsePacket;
        type Error = TransportError;
        type Future = TransportFut<'static>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: RequestPacket) -> Self::Future {
            let method = req.method_names().next().unwrap().to_string();
            self.requests.lock().unwrap().push(method.clone());
            let result = self.result.clone();
            Box::pin(async move { Ok(pending_tx_response(req, &method, result)) })
        }
    }

    impl Service<RequestPacket> for MockTransport {
        type Response = ResponsePacket;
        type Error = TransportError;
        type Future = TransportFut<'static>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: RequestPacket) -> Self::Future {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.requests.lock().unwrap().push(to_string(&req).unwrap());
            let result = self.result.clone();
            Box::pin(async move {
                match result {
                    MockResult::Success(value) => Ok(response_for(req, value)),
                    MockResult::RpcError(message) => Ok(error_response_for(req, message)),
                    MockResult::RpcErrorWithCode(code, message) => {
                        Ok(coded_error_response_for(req, code, message))
                    }
                    MockResult::TransportError(message) => {
                        Err(TransportErrorKind::custom_str(message))
                    }
                }
            })
        }
    }

    impl Service<RequestPacket> for SendRawTxTransport {
        type Response = ResponsePacket;
        type Error = TransportError;
        type Future = TransportFut<'static>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: RequestPacket) -> Self::Future {
            let raw_txs = self.raw_txs.clone();
            let requests = self.requests.clone();
            let transaction_count = self.transaction_count;
            let request_json = to_value(&req).unwrap();
            let method = req.method_names().next().unwrap().to_string();
            requests.lock().unwrap().push(to_string(&req).unwrap());
            Box::pin(async move {
                if method == "eth_getTransactionCount" {
                    return Ok(response_for(req, transaction_count));
                }
                if method == "eth_sendRawTransaction" {
                    let raw_tx = request_json["params"][0]
                        .as_str()
                        .expect("eth_sendRawTransaction raw tx param")
                        .to_string();
                    raw_txs.lock().unwrap().push(raw_tx);
                    return Ok(ResponsePacket::Single(Response {
                        id: req.as_single().unwrap().id().clone(),
                        payload: success_json_payload(format!(
                            "\"{:#x}\"",
                            B256::repeat_byte(0x44)
                        )),
                    }));
                }
                Ok(coded_error_response_for(req, -32601, "unexpected method"))
            })
        }
    }

    fn request(method: &str) -> RequestPacket {
        Request::new(method.to_string(), Id::Number(1), [(); 0])
            .serialize()
            .unwrap()
            .into()
    }

    fn batch_request(methods: &[&str]) -> RequestPacket {
        RequestPacket::Batch(
            methods
                .iter()
                .enumerate()
                .map(|(idx, method)| {
                    Request::new(method.to_string(), Id::Number((idx + 1) as u64), [(); 0])
                        .serialize()
                        .unwrap()
                })
                .collect(),
        )
    }

    fn response_for(req: RequestPacket, value: &str) -> ResponsePacket {
        match req {
            RequestPacket::Single(req) => ResponsePacket::Single(Response {
                id: req.id().clone(),
                payload: success_payload(value),
            }),
            RequestPacket::Batch(reqs) => ResponsePacket::Batch(
                reqs.into_iter()
                    .map(|req| Response {
                        id: req.id().clone(),
                        payload: success_payload(value),
                    })
                    .collect(),
            ),
        }
    }

    fn error_response_for(req: RequestPacket, message: &'static str) -> ResponsePacket {
        coded_error_response_for(req, -32603, message)
    }

    fn coded_error_response_for(
        req: RequestPacket,
        code: i64,
        message: &'static str,
    ) -> ResponsePacket {
        match req {
            RequestPacket::Single(req) => ResponsePacket::Single(Response {
                id: req.id().clone(),
                payload: error_payload(code, message),
            }),
            RequestPacket::Batch(reqs) => ResponsePacket::Batch(
                reqs.into_iter()
                    .map(|req| Response {
                        id: req.id().clone(),
                        payload: error_payload(code, message),
                    })
                    .collect(),
            ),
        }
    }

    fn success_payload(value: &str) -> ResponsePayload {
        ResponsePayload::Success(RawValue::from_string(format!("\"{value}\"")).unwrap())
    }

    fn success_json_payload(value: String) -> ResponsePayload {
        ResponsePayload::Success(RawValue::from_string(value).unwrap())
    }

    fn error_payload(code: i64, message: &'static str) -> ResponsePayload {
        ResponsePayload::Failure(ErrorPayload {
            code,
            message: message.into(),
            data: None,
        })
    }

    fn rpc_error_payload(code: i64, message: &'static str) -> TransportError {
        RpcError::ErrorResp(ErrorPayload {
            code,
            message: message.into(),
            data: None,
        })
    }

    fn single_success(response: ResponsePacket) -> String {
        let response = response.as_single().unwrap();
        from_str(response.payload.as_success().unwrap().get()).unwrap()
    }

    fn batch_success(response: ResponsePacket) -> Vec<String> {
        response
            .as_batch()
            .unwrap()
            .iter()
            .map(|response| from_str(response.payload.as_success().unwrap().get()).unwrap())
            .collect()
    }

    fn pending_tx_response(
        req: RequestPacket,
        method: &str,
        result: PendingTxResult,
    ) -> ResponsePacket {
        let payload = match (method, result) {
            ("eth_getTransactionByHash", PendingTxResult::MissingTransaction) => {
                success_json_payload("null".to_string())
            }
            ("eth_getTransactionByHash", PendingTxResult::MinedMissingReceipt { tx_hash })
            | ("eth_getTransactionByHash", PendingTxResult::MinedReceipt { tx_hash }) => {
                success_json_payload(mined_transaction_json(tx_hash))
            }
            ("eth_getTransactionReceipt", PendingTxResult::MinedMissingReceipt { .. }) => {
                success_json_payload("null".to_string())
            }
            ("eth_getTransactionReceipt", PendingTxResult::MinedReceipt { tx_hash }) => {
                success_json_payload(transaction_receipt_json(tx_hash))
            }
            _ => error_payload(-32601, "unexpected method"),
        };

        match req {
            RequestPacket::Single(req) => ResponsePacket::Single(Response {
                id: req.id().clone(),
                payload,
            }),
            RequestPacket::Batch(reqs) => ResponsePacket::Batch(
                reqs.into_iter()
                    .map(|req| Response {
                        id: req.id().clone(),
                        payload: error_payload(-32601, "unexpected batch"),
                    })
                    .collect(),
            ),
        }
    }

    fn mined_transaction_json(tx_hash: B256) -> String {
        format!(
            r#"{{"hash":"{tx_hash:#x}","nonce":"0x1","blockHash":"0x{}","blockNumber":"0x2","transactionIndex":"0x0","from":"0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266","to":"0x5fbdb2315678afecb367f032d93f642f64180aa3","value":"0x0","gasPrice":"0x3a29f0f8","gas":"0x1c9c380","input":"0x","r":"0xd309309a59a49021281cb6bb41d164c96eab4e50f0c1bd24c03ca336e7bc2bb7","s":"0x28a7f089143d0a1355ebeb2a1b9f0e5ad9eca4303021c1400d61bc23c9ac5319","v":"0x1c","chainId":"0x7a69","type":"0x0"}}"#,
            "22".repeat(32)
        )
    }

    fn transaction_receipt_json(tx_hash: B256) -> String {
        format!(
            r#"{{"transactionHash":"{tx_hash:#x}","blockHash":"0x{}","blockNumber":"0x2","logsBloom":"0x{}","gasUsed":"0x5208","contractAddress":null,"cumulativeGasUsed":"0x5208","transactionIndex":"0x0","from":"0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266","to":"0x5fbdb2315678afecb367f032d93f642f64180aa3","type":"0x0","effectiveGasPrice":"0x1","logs":[],"status":"0x1"}}"#,
            "22".repeat(32),
            "00".repeat(256)
        )
    }

    fn provider_for_pending_tx_transport(transport: PendingTxTransport) -> super::NadoProvider {
        let client = RpcClient::builder().transport(transport, false);
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(client)
            .erased()
    }

    fn test_signer() -> PrivateKeySigner {
        PrivateKeySigner::from_slice(&[1; 32]).unwrap()
    }

    #[tokio::test]
    async fn from_signer_provider_uses_existing_chain_id_without_rpc() {
        let transport = MockTransport::success("0x1");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let signer = test_signer().with_chain_id(Some(31_337));

        let _provider =
            super::signer_provider_inner_from_client_preserving_signer_chain_id(client, signer)
                .await
                .unwrap();

        assert_eq!(transport.calls(), 0);
    }

    #[test]
    fn sync_from_signer_provider_uses_existing_chain_id_without_rpc() {
        let transport = MockTransport::success("0x1");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let signer = test_signer().with_chain_id(Some(31_337));

        let _provider =
            super::signer_provider_inner_from_client_preserving_existing_chain_id(client, signer)
                .unwrap();

        assert_eq!(transport.calls(), 0);
    }

    #[tokio::test]
    async fn sync_from_signer_provider_signs_with_existing_chain_id() {
        let transport = SendRawTxTransport::default();
        let client = RpcClient::builder().transport(transport.clone(), false);
        let chain_id = 31_337;
        let signer = test_signer().with_chain_id(Some(chain_id));
        let provider =
            super::signer_provider_inner_from_client_preserving_existing_chain_id(client, signer)
                .unwrap();
        let tx = TransactionRequest::default()
            .to(Address::ZERO)
            .value(U256::from(1u8))
            .gas_limit(21_000)
            .gas_price(1)
            .nonce(0);

        let _pending = provider.send_transaction(tx).await.unwrap();

        let raw_txs = transport.raw_txs();
        assert_eq!(raw_txs.len(), 1);
        let raw_tx = raw_txs[0].strip_prefix("0x").unwrap_or(&raw_txs[0]);
        let tx_bytes = hex::decode(raw_tx).unwrap();
        let mut tx_bytes = tx_bytes.as_slice();
        let envelope = TxEnvelope::decode(&mut tx_bytes).unwrap();
        assert_eq!(envelope.chain_id(), Some(chain_id));
    }

    #[tokio::test]
    async fn sync_from_signer_provider_fills_missing_nonce_from_pending_count() {
        let transport = SendRawTxTransport::with_transaction_count("0x7");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let chain_id = 31_337;
        let signer = test_signer().with_chain_id(Some(chain_id));
        let signer_address = signer.address();
        let provider =
            super::signer_provider_inner_from_client_preserving_existing_chain_id(client, signer)
                .unwrap();
        let tx = TransactionRequest::default()
            .to(Address::ZERO)
            .value(U256::from(1u8))
            .gas_limit(21_000)
            .gas_price(1);

        let _pending = provider.send_transaction(tx).await.unwrap();

        let requests = transport.request_jsons();
        let count_request = requests
            .iter()
            .map(|request| from_str::<Value>(request).unwrap())
            .find(|request| request["method"] == "eth_getTransactionCount")
            .expect("expected nonce fill to call eth_getTransactionCount");
        assert_eq!(
            count_request["params"][0],
            to_value(signer_address).unwrap()
        );
        assert_eq!(count_request["params"][1], "pending");

        let raw_txs = transport.raw_txs();
        assert_eq!(raw_txs.len(), 1);
        let raw_tx = raw_txs[0].strip_prefix("0x").unwrap_or(&raw_txs[0]);
        let tx_bytes = hex::decode(raw_tx).unwrap();
        let mut tx_bytes = tx_bytes.as_slice();
        let envelope = TxEnvelope::decode(&mut tx_bytes).unwrap();
        assert_eq!(envelope.chain_id(), Some(chain_id));
        assert_eq!(envelope.nonce(), 7);
    }

    #[test]
    fn sync_from_signer_provider_rejects_raw_signer_without_chain_id() {
        let transport = MockTransport::success("0x7a69");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let signer = test_signer();

        let err = match super::signer_provider_inner_from_client_preserving_existing_chain_id(
            client, signer,
        ) {
            Ok(_) => panic!("raw signer without chain id should not build sync provider"),
            Err(err) => err,
        };

        assert!(err.to_string().contains("missing chain id"));
        assert_eq!(transport.calls(), 0);
    }

    #[tokio::test]
    async fn from_signer_provider_fetches_chain_id_for_raw_signer() {
        let transport = MockTransport::success("0x7a69");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let signer = test_signer();

        let _provider =
            super::signer_provider_inner_from_client_preserving_signer_chain_id(client, signer)
                .await
                .unwrap();

        assert_eq!(transport.calls(), 1);
    }

    #[tokio::test]
    async fn pending_nonce_manager_uses_pending_block_tag() {
        let transport = MockTransport::success("0x0");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(client);

        let nonce = PendingNonceManager
            .get_next_nonce(&provider, Address::ZERO)
            .await
            .unwrap();

        assert_eq!(nonce, 0);
        let requests = transport.request_jsons();
        assert_eq!(requests.len(), 1);
        let request: Value = from_str(&requests[0]).unwrap();
        assert_eq!(request["method"], "eth_getTransactionCount");
        assert_eq!(request["params"].as_array().unwrap().len(), 2);
        assert_eq!(
            request["params"][0],
            "0x0000000000000000000000000000000000000000"
        );
        assert_eq!(request["params"][1], "pending");
    }

    #[test]
    fn rate_limit_policy_matches_ethers_retry_cases() {
        let policy = super::EthersRateLimitRetryPolicy;

        assert!(policy.should_retry(&rpc_error_payload(429, "too many requests")));
        assert!(policy.should_retry(&rpc_error_payload(-32005, "project rate limit")));
        assert!(policy.should_retry(&rpc_error_payload(-32016, "rate limit exceeded")));
        assert!(policy.should_retry(&rpc_error_payload(-32603, "header not found")));
        assert!(policy.should_retry(&rpc_error_payload(
            -32603,
            "daily request count exceeded, request rate limited"
        )));
        assert!(policy.should_retry(&TransportErrorKind::http_error(429, String::new())));

        assert!(!policy.should_retry(&TransportErrorKind::http_error(503, String::new())));
        assert!(!policy.should_retry(&rpc_error_payload(
            -32603,
            "execution reverted: G 100 90192"
        )));
    }

    #[test]
    fn rate_limit_policy_reads_ethers_backoff_hint() {
        let policy = super::EthersRateLimitRetryPolicy;
        let payload: ErrorPayload = from_value(json!({
            "code": 429,
            "message": "too many requests",
            "data": {
                "rate": {
                    "backoff_seconds": 2.5
                }
            }
        }))
        .unwrap();

        assert_eq!(
            policy.backoff_hint(&RpcError::ErrorResp(payload)),
            Some(Duration::from_secs(3))
        );
    }

    #[test]
    fn rate_limit_policy_parses_error_response_from_deser_error_text() {
        let policy = super::EthersRateLimitRetryPolicy;
        let err = from_str::<u64>("not json").unwrap_err();
        let error = RpcError::DeserError {
            err,
            text:
                r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32005,"message":"project rate limit"}}"#
                    .to_string(),
        };

        assert!(policy.should_retry(&error));
    }

    #[tokio::test]
    async fn rate_limit_retry_layer_uses_ethers_retry_budget() {
        let transport = MockTransport::rpc_error_with_code(429, "too many requests");
        let client = RpcClient::builder()
            .layer(RetryBackoffLayer::new_with_policy(
                super::MAX_RETRIES,
                0,
                10_000,
                super::EthersRateLimitRetryPolicy,
            ))
            .transport(transport.clone(), false);
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(client);

        let _error = provider.get_chain_id().await.unwrap_err();

        assert_eq!(transport.calls(), (super::MAX_RETRIES + 1) as usize);
    }

    #[tokio::test]
    async fn plain_provider_does_not_retry_connectivity_errors() {
        let transport = MockTransport::transport_error("request timed out");
        let client = RpcClient::builder().transport(transport.clone(), false);
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_client(client);

        let _error = provider.get_chain_id().await.unwrap_err();

        assert_eq!(transport.calls(), 1);
    }

    #[tokio::test]
    async fn wait_for_receipt_returns_none_after_missing_transaction_retries() {
        let tx_hash = B256::repeat_byte(0x11);
        let transport = PendingTxTransport::missing_transaction();
        let provider = provider_for_pending_tx_transport(transport.clone());

        let receipt = super::wait_for_transaction_receipt_with_interval(
            &provider,
            tx_hash,
            2,
            Duration::from_millis(1),
        )
        .await
        .unwrap();

        assert!(receipt.is_none());
        assert_eq!(
            transport.request_methods(),
            vec![
                "eth_getTransactionByHash",
                "eth_getTransactionByHash",
                "eth_getTransactionByHash"
            ]
        );
    }

    #[tokio::test]
    async fn wait_for_receipt_keeps_polling_when_receipt_is_missing() {
        let tx_hash = B256::repeat_byte(0x22);
        let transport = PendingTxTransport::mined_missing_receipt(tx_hash);
        let provider = provider_for_pending_tx_transport(transport.clone());

        let timed_out = tokio::time::timeout(
            Duration::from_millis(50),
            super::wait_for_transaction_receipt_with_interval(
                &provider,
                tx_hash,
                2,
                Duration::from_millis(1),
            ),
        )
        .await
        .is_err();

        assert!(timed_out);
        let methods = transport.request_methods();
        assert_eq!(methods[0], "eth_getTransactionByHash");
        assert!(methods
            .iter()
            .skip(1)
            .all(|m| m == "eth_getTransactionReceipt"));
        assert!(methods.len() > 2);
    }

    #[tokio::test]
    async fn wait_for_receipt_returns_mined_receipt() {
        let tx_hash = B256::repeat_byte(0x33);
        let transport = PendingTxTransport::mined_receipt(tx_hash);
        let provider = provider_for_pending_tx_transport(transport.clone());

        let receipt = super::wait_for_transaction_receipt_with_interval(
            &provider,
            tx_hash,
            2,
            Duration::from_millis(1),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(receipt.transaction_hash, tx_hash);
        assert_eq!(
            transport.request_methods(),
            vec!["eth_getTransactionByHash", "eth_getTransactionReceipt"]
        );
    }

    #[tokio::test]
    async fn empty_provider_list_errors_at_request_time() {
        let mut service: SequentialFallbackService<MockTransport> =
            SequentialFallbackService::new(vec![]);

        let error = service.call(request("eth_chainId")).await.unwrap_err();

        assert!(error.to_string().contains("list of providers was empty"));
    }

    #[tokio::test]
    async fn primary_success_does_not_call_backup() {
        let primary = MockTransport::success("primary");
        let backup = MockTransport::success("backup");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let response = service.call(request("eth_chainId")).await.unwrap();

        assert_eq!(single_success(response), "primary");
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 0);
    }

    #[tokio::test]
    async fn backup_success_does_not_reorder_primary_on_next_request() {
        let primary = MockTransport::transport_error("primary down");
        let backup = MockTransport::success("backup");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let first = service.call(request("eth_chainId")).await.unwrap();
        let second = service.call(request("eth_blockNumber")).await.unwrap();

        assert_eq!(single_success(first), "backup");
        assert_eq!(single_success(second), "backup");
        assert_eq!(primary.calls(), 2);
        assert_eq!(backup.calls(), 2);
    }

    #[tokio::test]
    async fn primary_rpc_error_falls_back_to_backup() {
        let primary = MockTransport::rpc_error("primary rpc error");
        let backup = MockTransport::success("backup");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let response = service.call(request("eth_call")).await.unwrap();

        assert_eq!(single_success(response), "backup");
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 1);
    }

    #[tokio::test]
    async fn batch_rpc_error_falls_back_to_backup() {
        let primary = MockTransport::rpc_error("primary rpc error");
        let backup = MockTransport::success("backup");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let response = service
            .call(batch_request(&["eth_call", "eth_getBalance"]))
            .await
            .unwrap();

        assert_eq!(batch_success(response), vec!["backup", "backup"]);
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 1);
    }

    #[tokio::test]
    async fn all_rpc_errors_return_last_error() {
        let primary = MockTransport::rpc_error("primary rpc error");
        let backup = MockTransport::rpc_error("backup rpc error");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let error = service.call(request("eth_call")).await.unwrap_err();

        assert!(error.to_string().contains("backup rpc error"));
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 1);
    }

    #[tokio::test]
    async fn gas_simulation_transport_error_does_not_fall_back_to_backup() {
        let primary = MockTransport::transport_error("execution reverted: G 100 90192");
        let backup = MockTransport::success("backup");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let error = service.call(request("eth_estimateGas")).await.unwrap_err();

        assert!(error.to_string().contains(": G "));
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 0);
    }

    #[tokio::test]
    async fn gas_simulation_rpc_error_does_not_fall_back_to_backup() {
        let primary = MockTransport::rpc_error("execution reverted: G 100 90192");
        let backup = MockTransport::success("backup");
        let mut service = SequentialFallbackService::new(vec![primary.clone(), backup.clone()]);

        let error = service.call(request("eth_estimateGas")).await.unwrap_err();

        assert!(error.to_string().contains(": G "));
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 0);
    }

    #[tokio::test]
    async fn gas_rpc_error_after_transport_error_stops_before_later_backup() {
        let primary = MockTransport::transport_error("primary down");
        let backup = MockTransport::rpc_error("execution reverted: G 100 90192");
        let tertiary = MockTransport::success("tertiary");
        let mut service =
            SequentialFallbackService::new(vec![primary.clone(), backup.clone(), tertiary.clone()]);

        let error = service.call(request("eth_estimateGas")).await.unwrap_err();

        assert!(error.to_string().contains(": G "));
        assert_eq!(primary.calls(), 1);
        assert_eq!(backup.calls(), 1);
        assert_eq!(tertiary.calls(), 0);
    }
}
