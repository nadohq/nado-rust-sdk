use alloy::contract as alloy_contract;
use alloy_sol_types::sol;

use crate::bindings::perp_engine::Balance as PerpEngineBalance;
use crate::bindings::perp_engine::State as PerpEngineState;
use crate::bindings::spot_engine::Balance as SpotEngineBalance;
use crate::bindings::spot_engine::State as SpotEngineState;

// Keep Querier bytes32 Rust fields as arrays so nado_utils wire structs stay
// base-compatible while ABI encoding still follows Solidity bytesN rules.
mod array_sol_types {
    pub use alloy::sol_types::*;

    pub mod sol_data {
        pub use alloy::sol_types::sol_data::*;

        use alloy::sol_types::abi::token::WordToken;
        use alloy::sol_types::private::SolTypeValue;
        use alloy::sol_types::{EventTopic, SolType, Word};

        pub struct FixedBytes<const N: usize>;

        impl<const N: usize> SolTypeValue<FixedBytes<N>> for [u8; N]
        where
            ByteCount<N>: SupportedFixedBytes,
        {
            fn stv_to_tokens(&self) -> WordToken {
                let mut word = Word::ZERO;
                word[..N].copy_from_slice(self);
                word.into()
            }

            fn stv_eip712_data_word(&self) -> Word {
                SolTypeValue::<FixedBytes<N>>::stv_to_tokens(self).0
            }

            fn stv_abi_encode_packed_to(&self, out: &mut Vec<u8>) {
                out.extend_from_slice(self);
            }
        }

        impl<const N: usize> SolType for FixedBytes<N>
        where
            ByteCount<N>: SupportedFixedBytes,
        {
            type RustType = [u8; N];
            type Token<'a> = WordToken;

            const SOL_NAME: &'static str = <ByteCount<N>>::NAME;
            const ENCODED_SIZE: Option<usize> = Some(32);
            const PACKED_ENCODED_SIZE: Option<usize> = Some(N);

            fn valid_token(token: &Self::Token<'_>) -> bool {
                token.0[N..].iter().all(|byte| *byte == 0)
            }

            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                token.0[..N].try_into().unwrap()
            }
        }

        impl<const N: usize> EventTopic for FixedBytes<N>
        where
            ByteCount<N>: SupportedFixedBytes,
        {
            fn topic_preimage_length(_: &Self::RustType) -> usize {
                32
            }

            fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
                out.extend(SolTypeValue::<Self>::stv_to_tokens(rust).0);
            }

            fn encode_topic(rust: &Self::RustType) -> WordToken {
                SolTypeValue::<Self>::stv_to_tokens(rust)
            }
        }
    }

    pub mod private {
        pub use alloy::sol_types::private::*;

        pub type FixedBytes<const N: usize> = [u8; N];
    }
}

sol! {
    #![sol(
        all_derives,
        alloy_sol_types = array_sol_types,
        alloy_contract = alloy_contract
    )]
    #[sol(rpc)]
    #[allow(missing_docs, clippy::too_many_arguments)]
    contract Querier {
        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct BookInfo {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 size_increment;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price_increment_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 min_size;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 collected_fees;
        }

        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct HealthInfo {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 assets;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 liabilities;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 health;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct NlpPool {
            uint64 pool_id;
            bytes32 subaccount;
            address owner;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 balance_weight_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct NlpPoolInfo {
            NlpPool[] nlp_pools;
        }

        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct Risk {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 long_weight_initial_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 short_weight_initial_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 long_weight_maintenance_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 short_weight_maintenance_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Config {
            // serialize as "0x{hex}" string to match H160 bincode/serde format (backward compat)
            #[serde(serialize_with = "crate::bindings::serde_helpers::serialize_address", deserialize_with = "crate::bindings::serde_helpers::deserialize_address")]
            address token;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 interest_inflection_util_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 interest_floor_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 interest_small_cap_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 interest_large_cap_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 withdraw_fee_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 min_deposit_rate_x18;
        }

        // `all_derives` skips Debug/Default/PartialEq/Eq/Hash for structs that embed an
        // out-of-block (external) type such as `perp_engine::Balance`, so re-add them
        // explicitly here to match the derive set the ethers Abigen output carried. `Clone`
        // is still supplied by `all_derives`.
        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug, Default, PartialEq, Eq, Hash)]
        #[archive(check_bytes)]
        struct PerpBalance {
            uint32 product_id;
            PerpEngineBalance balance;
        }

        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug, Default, PartialEq, Eq, Hash)]
        #[archive(check_bytes)]
        struct PerpProduct {
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 oracle_price_x18;
            Risk risk;
            PerpEngineState state;
            BookInfo book_info;
        }

        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug, Default, PartialEq, Eq, Hash)]
        #[archive(check_bytes)]
        struct SpotBalance {
            uint32 product_id;
            SpotEngineBalance balance;
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq, Hash)]
        struct SpotProduct {
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 oracle_price_x18;
            Risk risk;
            Config config;
            SpotEngineState state;
            BookInfo book_info;
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq, Hash)]
        struct ProductInfo {
            SpotProduct[] spot_products;
            PerpProduct[] perp_products;
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq, Hash)]
        struct SubaccountInfo {
            bytes32 subaccount;
            bool exists;
            HealthInfo[] healths;
            int128[][] health_contributions;
            uint32 spot_count;
            uint32 perp_count;
            SpotBalance[] spot_balances;
            PerpBalance[] perp_balances;
            SpotProduct[] spot_products;
            PerpProduct[] perp_products;
        }

        function getAllProducts() external returns (ProductInfo);
        function getBookInfo(uint32 product_id) external view returns (BookInfo book_info);
        function getClearinghouse() external view returns (address);
        function getInsurance() external view returns (int128);
        function getNlpPoolInfo() external view returns (NlpPoolInfo);
        function getPerpBalance(bytes32 subaccount, uint32 product_id) external view returns (PerpBalance);
        function getPerpBalances(bytes32 subaccount, uint32[] product_ids) external view returns (PerpBalance[] perp_balances);
        function getPerpProduct(uint32 product_id) external returns (PerpProduct);
        function getPerpProducts(uint32[] product_ids) external returns (PerpProduct[] perp_products);
        function getSpotBalance(bytes32 subaccount, uint32 product_id) external view returns (SpotBalance);
        function getSpotBalances(bytes32 subaccount, uint32[] product_ids) external view returns (SpotBalance[] spot_balances);
        function getSpotProduct(uint32 product_id) external returns (SpotProduct);
        function getSpotProducts(uint32[] product_ids) external returns (SpotProduct[] spot_products);
        function getSubaccountInfo(bytes32 subaccount) external returns (SubaccountInfo);
        function initialize(address _clearinghouse) external;
    }
}
pub use Querier::*;

#[cfg(test)]
mod wire_golden {
    use super::Querier::{BookInfo, PerpBalance};
    use crate::bindings::perp_engine::Balance as PerpEngineBalance;
    use rkyv::Deserialize as _;

    /// Locks the JSON wire format of `BookInfo` to the pre-migration ethers Abigen output:
    /// snake_case keys and `int128` as quoted decimal strings. Guards against serde drift —
    /// the deserialized content and JSON field names external consumers depend on must stay
    /// identical across the alloy migration.
    #[test]
    fn book_info_serde_is_wire_stable() {
        let book_info = BookInfo {
            size_increment: 1,
            price_increment_x18: -2,
            min_size: 3,
            collected_fees: -4,
        };
        let json = serde_json::to_string(&book_info).unwrap();
        assert_eq!(
            json,
            r#"{"size_increment":"1","price_increment_x18":"-2","min_size":"3","collected_fees":"-4"}"#
        );
        let back: BookInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(back, book_info);
    }

    /// Locks the rkyv round-trip for `PerpBalance`, which embeds the cross-referenced
    /// `perp_engine::Balance` rkyv type. The archived bytes must validate via `check_bytes`
    /// and rkyv-deserialize back to an equal value, proving the nested rkyv content layout is
    /// preserved across the alloy migration.
    #[test]
    fn perp_balance_rkyv_roundtrips() {
        let perp_balance = PerpBalance {
            product_id: 7,
            balance: PerpEngineBalance {
                amount: 100,
                v_quote_balance: -200,
                last_cumulative_funding_x18: 300,
            },
        };
        let bytes = rkyv::to_bytes::<_, 256>(&perp_balance).unwrap();
        let archived = rkyv::check_archived_root::<PerpBalance>(&bytes).unwrap();
        let back: PerpBalance = archived.deserialize(&mut rkyv::Infallible).unwrap();
        assert_eq!(back, perp_balance);
    }
}
