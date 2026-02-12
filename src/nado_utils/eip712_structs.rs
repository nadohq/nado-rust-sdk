#![allow(non_snake_case)]

use crate::bindings::{endpoint, offchain_exchange};
use crate::isolated::is_isolated_subaccount;
use crate::math::{mul_x18, ONE_X18, ONE_X6};
use crate::serialize_utils::{
    deserialize_bytes20, deserialize_bytes32, deserialize_i128, deserialize_u128, deserialize_u64,
    deserialize_vec_bytes32, serialize_bytes20, serialize_bytes32, serialize_i128, serialize_u128,
    serialize_u64, serialize_vec_bytes32,
};
use ethers::prelude::*;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Debug;

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
    Default,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct Order {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub priceX18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub amount: i128, // positive: bid

    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub expiration: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
    #[serde(
        serialize_with = "serialize_u128",
        deserialize_with = "deserialize_u128"
    )]
    pub appendix: u128,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OrderType {
    #[default]
    Default,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

impl fmt::Display for OrderType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::Default => write!(fmt, "default"),
            OrderType::ImmediateOrCancel => write!(fmt, "ioc"),
            OrderType::FillOrKill => write!(fmt, "fok"),
            OrderType::PostOnly => write!(fmt, "post_only"),
        }
    }
}

impl OrderType {
    pub fn taker_only(&self) -> bool {
        matches!(self, OrderType::ImmediateOrCancel | OrderType::FillOrKill)
    }

    pub fn appendix_bit(&self) -> u128 {
        (match self {
            OrderType::Default => 0,
            OrderType::ImmediateOrCancel => 1,
            OrderType::FillOrKill => 2,
            OrderType::PostOnly => 3,
        }) << 9
    }
}

impl Order {
    pub fn new(
        sender: [u8; 32],
        priceX18: i128,
        amount: i128,
        expiration: u64,
        nonce: u64,
        appendix: u128,
    ) -> Self {
        Self {
            sender,
            priceX18,
            amount,
            expiration,
            nonce,
            appendix,
        }
    }

    pub fn to_offchain_exchange_binding(&self) -> offchain_exchange::Order {
        offchain_exchange::Order {
            sender: self.sender,
            price_x18: self.priceX18,
            amount: self.amount,
            expiration: self.expiration,
            nonce: self.nonce,
            appendix: self.appendix,
        }
    }

    pub fn to_binding(&self) -> endpoint::Order {
        endpoint::Order {
            sender: self.sender,
            price_x18: self.priceX18,
            amount: self.amount,
            expiration: self.expiration,
            nonce: self.nonce,
            appendix: self.appendix,
        }
    }

    pub fn to_signed_binding(&self, signature: &Bytes) -> endpoint::SignedOrder {
        endpoint::SignedOrder {
            order: self.to_binding(),
            signature: signature.clone(),
        }
    }

    pub fn to_offchain_exchange_signed_binding(
        &self,
        signature: &Bytes,
    ) -> offchain_exchange::SignedOrder {
        offchain_exchange::SignedOrder {
            order: self.to_offchain_exchange_binding(),
            signature: signature.clone(),
        }
    }

    pub fn from_binding(order: &endpoint::Order) -> Self {
        Self {
            sender: order.sender,
            priceX18: order.price_x18,
            amount: order.amount,
            expiration: order.expiration,
            nonce: order.nonce,
            appendix: order.appendix,
        }
    }

    pub fn reduce_only(&self) -> bool {
        (self.appendix >> 11 & 1) == 1
    }
    pub fn recv_time(&self) -> u64 {
        self.nonce >> 20
    }

    pub fn is_trigger_order(&self) -> bool {
        (self.appendix >> 12 & 3) > 0
    }

    pub fn is_price_trigger(&self) -> bool {
        (self.appendix >> 12 & 3) == 1
    }

    pub fn is_twap(&self) -> bool {
        (self.appendix >> 12 & 3) >= 2
    }

    pub fn is_twap_random(&self) -> bool {
        (self.appendix >> 12 & 3) == 3
    }

    pub fn twap_times(&self) -> Option<u32> {
        if self.is_twap() {
            Some((self.appendix >> 96) as u32)
        } else {
            None
        }
    }

    pub fn twap_slippage_x6(&self) -> Option<u32> {
        if self.is_twap() {
            Some((self.appendix >> 64 & ((1 << 32) - 1)) as u32)
        } else {
            None
        }
    }

    pub fn order_price(&self, oracle_price: i128) -> i128 {
        let mut price = self.priceX18;
        if self.is_twap() {
            let slippage = self.twap_slippage_x6().unwrap() as i128 * ONE_X18 / ONE_X6;

            let twap_price = if self.amount.is_positive() {
                mul_x18(oracle_price, ONE_X18 + slippage)
            } else {
                mul_x18(oracle_price, ONE_X18 - slippage)
            };

            if self.amount.is_positive() {
                price = min(price, twap_price);
            } else {
                price = max(price, twap_price);
            }
        }
        price
    }

    pub fn is_isolated(&self) -> bool {
        (self.appendix >> 8 & 1) == 1
    }

    pub fn isolated_margin(&self) -> Option<i128> {
        if self.is_isolated() {
            Some((self.appendix >> 64) as i128 * ONE_X18 / ONE_X6)
        } else {
            None
        }
    }

    pub fn builder_info(&self) -> (u32, i128) {
        let builder_id = ((self.appendix >> 48) & ((1 << 16) - 1)) as u32;
        let builder_fee_rate =
            ((self.appendix >> 38) & ((1 << 10) - 1)) as i128 * (ONE_X18 / 100_000); // 0.1bps
        (builder_id, builder_fee_rate)
    }
    pub fn reserved_bits(&self) -> u128 {
        (self.appendix >> 14) & ((1 << 24) - 1)
    }

    pub fn version(&self) -> u8 {
        (self.appendix & 255) as u8
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct Cancellation {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    pub productIds: Vec<u32>,
    #[serde(
        serialize_with = "serialize_vec_bytes32",
        deserialize_with = "deserialize_vec_bytes32"
    )]
    pub digests: Vec<[u8; 32]>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl Cancellation {
    pub fn to_binding(&self) -> endpoint::Cancellation {
        endpoint::Cancellation {
            sender: self.sender,
            product_ids: self.productIds.clone(),
            digests: self.digests.clone(),
            nonce: self.nonce,
        }
    }

    pub fn to_signed_binding(&self, signature: &Bytes) -> endpoint::SignedCancellation {
        endpoint::SignedCancellation {
            cancellation: self.to_binding(),
            signature: signature.clone(),
        }
    }

    pub fn recv_time(&self) -> u64 {
        self.nonce >> 20
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct CancellationProducts {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    pub productIds: Vec<u32>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl CancellationProducts {
    pub fn to_binding(&self) -> endpoint::CancellationProducts {
        endpoint::CancellationProducts {
            sender: self.sender,
            product_ids: self.productIds.clone(),
            nonce: self.nonce,
        }
    }

    pub fn to_signed_binding(&self, signature: &Bytes) -> endpoint::SignedCancellationProducts {
        endpoint::SignedCancellationProducts {
            cancellation_products: self.to_binding(),
            signature: signature.clone(),
        }
    }

    pub fn recv_time(&self) -> u64 {
        self.nonce >> 20
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct LinkSigner {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub signer: [u8; 32],
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl LinkSigner {
    pub fn to_binding(&self) -> endpoint::LinkSigner {
        endpoint::LinkSigner {
            sender: self.sender,
            signer: self.signer,
            nonce: self.nonce,
        }
    }

    pub fn to_signed_binding(&self, signature: &Bytes) -> endpoint::SignedLinkSigner {
        endpoint::SignedLinkSigner {
            tx: self.to_binding(),
            signature: signature.clone(),
        }
    }

    pub fn recv_time(&self) -> u64 {
        self.nonce >> 20
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct LiquidateSubaccount {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub liquidatee: [u8; 32],
    pub productId: u32,
    pub isEncodedSpread: bool,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub amount: i128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl LiquidateSubaccount {
    pub fn to_binding(&self) -> endpoint::LiquidateSubaccount {
        endpoint::LiquidateSubaccount {
            sender: self.sender,
            liquidatee: self.liquidatee,
            product_id: self.productId,
            is_encoded_spread: self.isEncodedSpread,
            amount: self.amount,
            nonce: self.nonce,
        }
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct WithdrawCollateral {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    pub productId: u32,
    #[serde(
        serialize_with = "serialize_u128",
        deserialize_with = "deserialize_u128"
    )]
    // #[ts(type = "BigNumberish")]
    pub amount: u128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl WithdrawCollateral {
    pub fn to_binding(&self) -> endpoint::WithdrawCollateral {
        endpoint::WithdrawCollateral {
            sender: self.sender,
            product_id: self.productId,
            amount: self.amount,
            nonce: self.nonce,
        }
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct MintNlp {
    // #[ts(type = "typestring")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_u128",
        deserialize_with = "deserialize_u128"
    )]
    // #[ts(type = "BigNumberish")]
    pub quoteAmount: u128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl MintNlp {
    pub fn to_binding(&self) -> endpoint::MintNlp {
        endpoint::MintNlp {
            sender: self.sender,
            quote_amount: self.quoteAmount,
            nonce: self.nonce,
        }
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct BurnNlp {
    // #[ts(type = "typestring")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_u128",
        deserialize_with = "deserialize_u128"
    )]
    // #[ts(type = "BigNumberish")]
    pub nlpAmount: u128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl BurnNlp {
    pub fn to_binding(&self) -> endpoint::BurnNlp {
        endpoint::BurnNlp {
            sender: self.sender,
            nlp_amount: self.nlpAmount,
            nonce: self.nonce,
        }
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eip712,
    EthAbiType,
)]
#[eip712()]
#[archive(check_bytes)]
#[allow(non_snake_case)]
pub struct TransferQuote {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub recipient: [u8; 32],
    #[serde(
        serialize_with = "serialize_u128",
        deserialize_with = "deserialize_u128"
    )]
    // #[ts(type = "BigNumberish")]
    pub amount: u128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
}

impl TransferQuote {
    pub fn to_binding(&self) -> endpoint::TransferQuote {
        endpoint::TransferQuote {
            sender: self.sender,
            recipient: self.recipient,
            amount: self.amount,
            nonce: self.nonce,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Eip712,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
#[eip712()]
#[allow(non_snake_case)]
pub struct ListTriggerOrders {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub recvTime: u64,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct SignedListTriggerOrders {
    pub tx: ListTriggerOrders,
    pub signature: ethers::core::types::Bytes,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Eip712,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
#[eip712()]
#[allow(non_snake_case)]
pub struct StreamAuthentication {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub expiration: u64,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Eip712,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
#[eip712()]
#[allow(non_snake_case)]
pub struct TaskAuthentication {
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub sender: [u8; 20],
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub expiration: u64,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Eip712,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
#[eip712()]
#[allow(non_snake_case)]
pub struct LeaderboardAuthentication {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub expiration: u64,
}

pub fn to_bytes12(s: &str) -> [u8; 12] {
    let b = s.as_bytes();
    let mut out = [0u8; 12];
    let n = std::cmp::min(b.len(), 12);
    out[..n].copy_from_slice(&b[..n]);
    out
}

pub fn to_bytes32(address: H160, name: &str) -> [u8; 32] {
    concat_to_bytes32(address.into(), to_bytes12(name))
}

pub fn concat_to_bytes32(address: [u8; 20], name: [u8; 12]) -> [u8; 32] {
    let mut ret = [0; 32];
    ret[..20].clone_from_slice(&address);
    ret[20..].clone_from_slice(&name);
    ret
}

pub fn from_bytes32(b: [u8; 32]) -> (H160, String) {
    (
        H160::from_slice(&b[..20]),
        from_bytes12(b[20..].try_into().unwrap()),
    )
}

pub fn from_bytes12(b: [u8; 12]) -> String {
    String::from_utf8(b.to_vec()).unwrap()
}

pub fn from_bytes32_safe(b: [u8; 32]) -> (H160, String) {
    let subaccount_name_bytes = b[20..].try_into().unwrap();
    let subaccount_name = if is_isolated_subaccount(b) {
        format!("0x{}", hex::encode(subaccount_name_bytes))
    } else {
        from_bytes12_safe(subaccount_name_bytes)
    };
    (H160::from_slice(&b[..20]), subaccount_name)
}

pub fn from_bytes12_safe(b: [u8; 12]) -> String {
    match String::from_utf8(b.iter().copied().filter(|&c| c != 0).collect()) {
        Ok(s) => s,
        Err(_) => {
            format!("0x{}", hex::encode(b))
        }
    }
}
