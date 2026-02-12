use eyre::Result;

use crate::eip712_structs::OrderType;
use crate::engine::OrderResponse;
use crate::math::{f64_to_x18, to_i128_x18, to_u128_x6};
use crate::trigger::{PriceRequirement, TriggerCriteria};

use crate::prelude::*;
use crate::utils::private_key::private_key;

pub async fn execute_sanity_check() -> Result<()> {
    println!("setting up nado client...");
    let client = NadoClient::new(ClientMode::Local)
        .with_signer(private_key())
        .await
        .unwrap();

    println!("depositing collateral...");
    deposit(&client).await;

    println!("placing and cancelling orders...");
    place_orders(&client).await;

    println!("placing trigger orders...");
    trigger(&client).await;

    println!("withdrawing collateral...");
    withdraw_collateral(&client).await;

    println!("sanity check complete!");
    Ok(())
}

async fn deposit(client: &NadoClient) {
    let product_id = 0;
    let amount = 100000;
    client
        .deposit_collateral_builder()
        .product_id(product_id)
        .amount(to_u128_x6(amount))
        .mints_tokens(true)
        .deposit_and_await_balance()
        .await
        .unwrap();
}

async fn place_orders(client: &NadoClient) {
    let product_id = 1;
    let place_order_response = client
        .place_order_builder()
        .product_id(product_id)
        .price_x18(to_i128_x18(34000))
        .amount(f64_to_x18(0.05))
        .execute()
        .await
        .unwrap()
        .unwrap();

    let open_orders = client
        .get_subaccount_orders(client.subaccount().unwrap(), product_id)
        .await
        .unwrap();

    assert!(orders_contain_digest(
        place_order_response.digest,
        &open_orders.orders
    ));

    let cancel_orders_response = client
        .cancellation_builder()
        .digests(vec![open_orders.orders[0].digest])
        .product_ids(vec![product_id])
        .execute()
        .await
        .unwrap()
        .unwrap();

    assert!(orders_contain_digest(
        place_order_response.digest,
        &cancel_orders_response.cancelled_orders
    ));

    let place_order_response = client
        .place_order_builder()
        .product_id(2)
        .price_x18(to_i128_x18(34000))
        .amount(f64_to_x18(0.05))
        .execute()
        .await
        .unwrap()
        .unwrap();

    let cancel_product_orders = client
        .cancellation_products_builder()
        .product_ids(vec![1, 2])
        .execute()
        .await
        .unwrap()
        .unwrap();

    assert!(orders_contain_digest(
        place_order_response.digest,
        &cancel_product_orders.cancelled_orders
    ));

    // place order
    let digest_to_cancel = client
        .place_order_builder()
        .product_id(2)
        .price_x18(to_i128_x18(34000))
        .amount(f64_to_x18(0.05))
        .execute()
        .await
        .unwrap()
        .unwrap()
        .digest;

    let cancel_tx = client
        .cancellation_builder()
        .product_ids(vec![2])
        .digests(vec![digest_to_cancel])
        .build()
        .unwrap();

    let place = client
        .place_order_builder()
        .product_id(2)
        .price_x18(to_i128_x18(35000))
        .amount(f64_to_x18(0.05))
        .build()
        .unwrap();

    let placed_order_digest = client
        .cancel_and_place(cancel_tx, place, false)
        .await
        .unwrap()
        .unwrap()
        .digest;

    let open_orders = client
        .get_subaccount_orders(client.subaccount().unwrap(), 2)
        .await
        .unwrap();

    assert!(!orders_contain_digest(
        digest_to_cancel,
        &open_orders.orders
    ));

    assert!(orders_contain_digest(
        placed_order_digest,
        &open_orders.orders
    ));

    client
        .cancellation_products_builder()
        .product_ids(vec![2])
        .execute()
        .await
        .unwrap();
}

fn orders_contain_digest(digest: [u8; 32], orders: &[OrderResponse]) -> bool {
    orders.iter().any(|order| order.digest == digest)
}

async fn trigger(client: &NadoClient) {
    let product_id = 1;
    let trigger_order_builder = client
        .place_order_builder()
        .product_id(product_id)
        .amount(f64_to_x18(-0.01))
        .price_x18(to_i128_x18(39000))
        .order_type(OrderType::FillOrKill)
        .trigger_criteria(TriggerCriteria::PriceTrigger {
            price_requirement: PriceRequirement::OraclePriceAbove(to_i128_x18(37000)),
            dependency: None,
        });

    let mut trigger_order = trigger_order_builder.build_trigger().unwrap();
    let digest = trigger_order.digest.unwrap();
    client.place_trigger_order(trigger_order).await.unwrap();

    let trigger_order_query = client
        .list_trigger_orders_builder()
        .product_ids(vec![product_id])
        .pending(true);

    let mut trigger_orders = trigger_order_query.query().await.unwrap();
    assert!(trigger_orders.contains_digest(digest));

    client
        .cancellation_builder()
        .product_ids(vec![product_id])
        .digests(vec![digest.into()])
        .execute_trigger()
        .await
        .unwrap();

    trigger_orders = trigger_order_query.query().await.unwrap();
    assert!(!trigger_orders.contains_digest(digest));

    trigger_order = trigger_order_builder.build_trigger().unwrap();
    let digest = trigger_order.digest.unwrap();

    client.place_trigger_order(trigger_order).await.unwrap();

    trigger_orders = trigger_order_query.query().await.unwrap();
    assert!(trigger_orders.contains_digest(digest));

    client
        .cancellation_products_builder()
        .product_ids(vec![product_id])
        .execute_trigger()
        .await
        .unwrap();

    trigger_orders = trigger_order_query.query().await.unwrap();
    assert!(!trigger_orders.contains_digest(digest));
}

async fn withdraw_collateral(client: &NadoClient) {
    let info = client
        .get_subaccount_info(client.subaccount().unwrap())
        .await
        .unwrap();
    let pre_balance = info.get_spot_balance(0).unwrap();

    let builder = client
        .withdraw_collateral_builder()
        .product_id(0)
        .spot_leverage(false)
        .amount(to_u128_x6(5));

    builder.execute().await.unwrap();

    let info = client
        .get_subaccount_info(client.subaccount().unwrap())
        .await
        .unwrap();
    let post_withdraw_balance = info.get_spot_balance(0).unwrap();

    assert!(post_withdraw_balance.balance.amount < pre_balance.balance.amount);

    //withdraw half via slow mode
    let tx = builder.build_endpoint_tx().await.unwrap();

    client
        .submit_slow_mode_tx_builder()
        .withdraw_collateral_tx(tx)
        .mints_fee(true)
        .execute_and_sleep()
        .await
        .unwrap();

    let info = client
        .get_subaccount_info(client.subaccount().unwrap())
        .await
        .unwrap();
    let post_slow_mode_balance = info.get_spot_balance(0).unwrap();

    assert!(post_slow_mode_balance.balance.amount < post_withdraw_balance.balance.amount);
}
