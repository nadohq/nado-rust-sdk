use std::time::Duration;

use tokio::time::sleep;

use nado_sdk::indexer::Limit;
use nado_sdk::math::to_u128_x6;
use nado_sdk::prelude::*;
use nado_sdk::serialize_utils::WrappedU32;
use nado_sdk::tx::TxType;

const QUOTE_PRODUCT_ID: u32 = 0;

#[tokio::main]
async fn main() {
    let client = NadoClient::new(ClientMode::Test)
        .with_signer(private_key())
        .await
        .unwrap();
    let subaccount = client.subaccount().unwrap();

    let latest_withdrawal_idx = |client: &NadoClient| {
        let client = client.clone();
        async move {
            client
                .get_events_builder()
                .subaccounts(vec![subaccount])
                .product_ids(vec![QUOTE_PRODUCT_ID])
                .event_types(vec![TxType::WithdrawCollateral])
                .limit(Limit::Raw(WrappedU32(1)))
                .desc(true)
                .query()
                .await
                .ok()
                .and_then(|res| res.events.first().map(|e| e.submission_idx as u64))
        }
    };

    let prior_idx = latest_withdrawal_idx(&client).await;

    client
        .withdraw_collateral_builder()
        .product_id(QUOTE_PRODUCT_ID)
        .amount(to_u128_x6(20))
        .execute()
        .await
        .unwrap();
    println!("withdraw_collateral submitted: 20 quote, product {QUOTE_PRODUCT_ID}");

    let mut idx = None;
    for _ in 0..30 {
        match latest_withdrawal_idx(&client).await {
            Some(i) if Some(i) != prior_idx => {
                idx = Some(i);
                break;
            }
            _ => sleep(Duration::from_secs(2)).await,
        }
    }
    let idx = idx.expect("withdrawal not indexed within 60s");
    println!("submission_idx: {idx}");

    for _ in 0..30 {
        match client.get_fast_withdrawal_signature(idx).await {
            Ok(res) => {
                let signatures = res
                    .signatures
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                println!("submitFastWithdrawal(uint64,bytes,bytes[]) args:");
                println!("  idx: {idx}");
                println!("  transaction: {}", res.tx_bytes);
                println!("  signatures: [{signatures}]");
                return;
            }
            Err(_) => sleep(Duration::from_secs(2)).await,
        }
    }
    panic!("fast withdrawal signatures not available within 60s");
}
