//! Wire codec for `MatchOrdersV2` (TxType 40). Mirrors
//! `contracts/libraries/MatchOrdersV2Helper.sol`: the tx is the type byte
//! followed by eleven raw 32-byte words, with fields in declaration order,
//! high bits first, starting a new word only when they don't fit.
//!
//! ```text
//! words 0-4  taker: sender, priceX18 << 128 | amount,
//!            expiration << 192 | nonce << 128 | appendix, r, vs
//! words 5-9  maker, same layout
//! word 10    takerAmountDelta << 128 | productId << 96 | controlFlags << 32
//! ```
//!
//! The length and the low 32 bits of word 10 are checked, so every match has
//! exactly one encoding.

use alloy_primitives::{Bytes, B256};

use crate::bindings::endpoint;
use crate::tx::TxType;

/// An OTC match: no protocol fee on either side; builder fees still apply.
pub const MATCH_FLAG_OTC: u64 = 1;
/// `EndpointTx` reverts on any other `controlFlags` bit.
pub const KNOWN_MATCH_FLAGS: u64 = MATCH_FLAG_OTC;

const WORDS: usize = 11;
pub const TX_LENGTH: usize = 1 + WORDS * 32;

pub fn encode(m: &endpoint::MatchOrdersV2) -> Bytes {
    let mut out = Vec::with_capacity(TX_LENGTH);
    out.push(TxType::MatchOrdersV2 as u8);
    for side in [&m.taker, &m.maker] {
        let o = &side.order;
        out.extend_from_slice(o.sender.as_slice());
        out.extend_from_slice(&o.price_x18.to_be_bytes());
        out.extend_from_slice(&o.amount.to_be_bytes());
        out.extend_from_slice(&o.expiration.to_be_bytes());
        out.extend_from_slice(&o.nonce.to_be_bytes());
        out.extend_from_slice(&o.appendix.to_be_bytes());
        out.extend_from_slice(side.signature.r.as_slice());
        out.extend_from_slice(side.signature.vs.as_slice());
    }
    out.extend_from_slice(&m.taker_amount_delta.to_be_bytes());
    out.extend_from_slice(&m.product_id.to_be_bytes());
    out.extend_from_slice(&m.control_flags.to_be_bytes());
    out.extend_from_slice(&[0u8; 4]);
    debug_assert_eq!(out.len(), TX_LENGTH);
    Bytes::from(out)
}

/// `None` for anything `EndpointTx` would reject as malformed: a wrong length,
/// non-zero spare bits, or a type byte other than `MatchOrdersV2`. Unknown
/// `control_flags` bits decode fine; callers that need executability check
/// [`endpoint::MatchOrdersV2::has_unknown_flags`].
pub fn decode(tx: &[u8]) -> Option<endpoint::MatchOrdersV2> {
    if tx.len() != TX_LENGTH || tx[0] != TxType::MatchOrdersV2 as u8 {
        return None;
    }
    let mut r = Reader(&tx[1..]);
    let taker = r.signed_order();
    let maker = r.signed_order();
    let taker_amount_delta = i128::from_be_bytes(r.take());
    let product_id = u32::from_be_bytes(r.take());
    let control_flags = u64::from_be_bytes(r.take());
    if r.take::<4>() != [0u8; 4] {
        return None;
    }
    Some(endpoint::MatchOrdersV2 {
        taker,
        maker,
        taker_amount_delta,
        product_id,
        control_flags,
    })
}

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let (head, rest) = self.0.split_at(N);
        self.0 = rest;
        head.try_into().unwrap()
    }

    fn signed_order(&mut self) -> endpoint::SignedOrderV2 {
        endpoint::SignedOrderV2 {
            order: endpoint::Order {
                sender: B256::from(self.take::<32>()),
                price_x18: i128::from_be_bytes(self.take()),
                amount: i128::from_be_bytes(self.take()),
                expiration: u64::from_be_bytes(self.take()),
                nonce: u64::from_be_bytes(self.take()),
                appendix: u128::from_be_bytes(self.take()),
            },
            signature: endpoint::CompactSignature {
                r: B256::from(self.take::<32>()),
                vs: B256::from(self.take::<32>()),
            },
        }
    }
}

impl endpoint::SignedOrderV2 {
    /// The legacy 65-byte `r ‖ s ‖ v` form that `OffchainExchange` verifies.
    /// The zero signature of an `X_ACCOUNT` order expands to zeros with
    /// `v = 27`, which is fine because `X_ACCOUNT` skips the signature check.
    pub fn to_signed_order(&self) -> endpoint::SignedOrder {
        let mut signature = Vec::with_capacity(65);
        signature.extend_from_slice(self.signature.r.as_slice());
        signature.extend_from_slice(self.signature.vs.as_slice());
        let v = 27 + (signature[32] >> 7);
        signature[32] &= 0x7f;
        signature.push(v);
        endpoint::SignedOrder {
            order: self.order.clone(),
            signature: Bytes::from(signature),
        }
    }
}

impl endpoint::MatchOrdersV2 {
    pub fn is_otc(&self) -> bool {
        self.control_flags & MATCH_FLAG_OTC != 0
    }

    pub fn has_unknown_flags(&self) -> bool {
        self.control_flags & !KNOWN_MATCH_FLAGS != 0
    }

    pub fn to_match_orders(&self) -> endpoint::MatchOrders {
        endpoint::MatchOrders {
            product_id: self.product_id,
            taker: self.taker.to_signed_order(),
            maker: self.maker.to_signed_order(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> endpoint::MatchOrdersV2 {
        let order = |sender: u8, price_x18: i128, amount: i128| endpoint::Order {
            sender: B256::repeat_byte(sender),
            price_x18,
            amount,
            expiration: u64::MAX - 1,
            nonce: 0x1234_5678_9abc_def0,
            appendix: u128::MAX - 7,
        };
        endpoint::MatchOrdersV2 {
            taker: endpoint::SignedOrderV2 {
                order: order(
                    0xaa,
                    99_000_000_000_000_000_000,
                    -10_000_000_000_000_000_000,
                ),
                signature: endpoint::CompactSignature {
                    r: B256::repeat_byte(0x11),
                    vs: B256::repeat_byte(0x92),
                },
            },
            maker: endpoint::SignedOrderV2 {
                order: order(0xbb, 100_000_000_000_000_000_000, 5_000_000_000_000_000_000),
                signature: endpoint::CompactSignature {
                    r: B256::repeat_byte(0x33),
                    vs: B256::repeat_byte(0x44),
                },
            },
            taker_amount_delta: -1_000_000_000_000_000_000,
            product_id: 2,
            control_flags: MATCH_FLAG_OTC,
        }
    }

    // shared with MatchOrders.t.sol and generated by a third, independent
    // encoder, so the Rust and Solidity codecs can't drift together
    const GOLDEN: &str = concat!(
        "28",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "00000000000000055de6a779bbac0000ffffffffffffffff7538dcfb76180000",
        "fffffffffffffffe123456789abcdef0fffffffffffffffffffffffffffffff8",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "9292929292929292929292929292929292929292929292929292929292929292",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "00000000000000056bc75e2d6310000000000000000000004563918244f40000",
        "fffffffffffffffe123456789abcdef0fffffffffffffffffffffffffffffff8",
        "3333333333333333333333333333333333333333333333333333333333333333",
        "4444444444444444444444444444444444444444444444444444444444444444",
        "fffffffffffffffff21f494c589c000000000002000000000000000100000000",
    );

    #[test]
    fn matches_golden() {
        assert_eq!(alloy_primitives::hex::encode(encode(&sample())), GOLDEN);
    }

    #[test]
    fn round_trips() {
        let m = sample();
        let tx = encode(&m);
        assert_eq!(tx.len(), 353);
        assert_eq!(decode(&tx), Some(m));
    }

    #[test]
    fn rejects_non_canonical_encodings() {
        let tx = encode(&sample()).to_vec();

        let mut long = tx.clone();
        long.push(0);
        assert_eq!(decode(&long), None);
        assert_eq!(decode(&tx[..tx.len() - 1]), None);

        let mut spare = tx.clone();
        *spare.last_mut().unwrap() = 1;
        assert_eq!(decode(&spare), None);

        let mut wrong_type = tx;
        wrong_type[0] = TxType::MatchOrders as u8;
        assert_eq!(decode(&wrong_type), None);
    }

    // archive /events serve NadoTx as JSON; clients key off this shape
    #[test]
    fn serializes_under_match_orders_v2() {
        let json = serde_json::to_value(crate::tx::NadoTx::MatchOrdersV2(sample())).unwrap();
        let m = &json["match_orders_v2"];
        assert_eq!(m["product_id"], 2);
        assert_eq!(m["control_flags"], 1);
        assert_eq!(m["taker_amount_delta"], "-1000000000000000000");
        assert_eq!(m["taker"]["order"]["amount"], "-10000000000000000000");
        assert_eq!(
            m["maker"]["signature"]["r"],
            format!("0x{}", "33".repeat(32))
        );
    }

    #[test]
    fn expands_compact_signature_to_65_bytes() {
        let signed = sample().taker.to_signed_order();
        let mut expected = vec![0x11; 32];
        expected.extend(std::iter::repeat_n(0x92, 32));
        expected[32] = 0x12;
        expected.push(28);
        assert_eq!(signed.signature.to_vec(), expected);

        let maker = sample().maker.to_signed_order();
        assert_eq!(maker.signature[32], 0x44);
        assert_eq!(maker.signature[64], 27);
    }
}
