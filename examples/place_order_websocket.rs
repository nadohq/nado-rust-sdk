// Example demonstrating how to place orders using websockets with the SDK builder.
// This example shows direct websocket communication while leveraging the SDK's
// PlaceOrderBuilder to construct and sign orders.
//
// Run with: cargo run --example place_order_websocket --features ws
//
// Required environment variables:
// - RUST_SDK_PRIVATE_KEY: Your wallet private key (set in .env file)
//
// Optional environment variables:
// - NETWORK: "test" or "prod" (defaults to "test")

use eyre::Result;
use futures_util::{SinkExt, StreamExt};
use nado_sdk::engine::{EngineMessage, Execute, ExecuteResponse, Status};
use nado_sdk::math::to_i128_x18;
use nado_sdk::prelude::*;
use nado_sdk::utils::private_key::private_key;
use rustls::crypto::ring;
use std::env;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};

const TESTNET_WS: &str = "wss://gateway.test.nado.xyz/v1/ws";
const PROD_WS: &str = "wss://gateway.prod.nado.xyz/v1/ws";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Ensure rustls picks a crypto provider when both ring and aws-lc are compiled in.
    ring::default_provider()
        .install_default()
        .expect("failed to install rustls ring provider");

    let network = env::var("NETWORK").unwrap_or_else(|_| "test".to_string());
    let ws_url = match network.as_str() {
        "prod" => PROD_WS,
        "test" => TESTNET_WS,
        _ => {
            eprintln!("Invalid NETWORK: {network}. Using 'test' as default.");
            TESTNET_WS
        }
    };

    println!("Connecting to {network} ({ws_url})");

    let (ws_stream, _) = connect_async(ws_url).await?;
    let (mut write, mut read) = ws_stream.split();

    println!("Connected successfully!");

    // Create NadoClient to leverage SDK's signing and building utilities
    let client_mode = match network.as_str() {
        "prod" => ClientMode::Prod,
        "test" => ClientMode::Test,
        _ => ClientMode::Test,
    };

    let client = NadoClient::new(client_mode)
        .with_signer(private_key())
        .await
        .unwrap();

    println!("Address: 0x{}", hex::encode(client.address()?));
    println!("Subaccount: 0x{}", hex::encode(client.subaccount()?));

    // Build order using SDK's PlaceOrderBuilder
    const BTC_PERP: u32 = 2;
    let place_order = client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(-1)) // Sell 1 BTC
        .price_x18(to_i128_x18(95_000))
        .build()?;

    println!(
        "\nPlacing order:\n  Product: {} (BTC_PERP)\n  Price: ${}\n  Amount: {} BTC\n  Digest: 0x{}",
        BTC_PERP,
        place_order.order.priceX18 as f64 / 1e18,
        place_order.order.amount as f64 / 1e18,
        hex::encode(place_order.digest.unwrap())
    );

    // Send order via websocket (bypassing SDK's HTTP client)
    let place_order_msg =
        serde_json::to_string(&EngineMessage::Execute(Execute::PlaceOrder(place_order)))?;
    write.send(Message::Text(place_order_msg)).await?;

    // Receive response
    let response = timeout(Duration::from_secs(10), read.next())
        .await?
        .ok_or_else(|| eyre::eyre!("No response"))??;

    let execute_response: ExecuteResponse = serde_json::from_str(&response.to_string())?;

    match execute_response.status {
        Status::Success => {
            println!("✓ Order placed successfully!");
            if let Some(sig) = execute_response.signature {
                println!("  Response signature: 0x{}", hex::encode(sig));
            }
        }
        Status::Failure => {
            println!(
                "✗ Order failed: {}",
                execute_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            );
        }
    }

    Ok(())
}
