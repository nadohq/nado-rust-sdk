use alloy::sol;

sol! {
    #![sol(all_derives)]
    #[sol(rpc)]
    #[allow(missing_docs, clippy::too_many_arguments)]
    contract SpotEngine {
        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct Balance {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 amount;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct BalanceNormalized {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 amount_normalized;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct CoreRisk {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 amount;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 long_weight;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Config {
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

        #[derive(serde::Serialize, serde::Deserialize)]
        struct NlpLockedBalance {
            Balance balance;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 unlocked_at;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct NlpLockedBalances {
            Balance balance_unlocked;
            Balance balance_locked;
            NlpLockedBalance[] balances;
        }

        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct State {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 cumulative_deposits_multiplier_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 cumulative_borrows_multiplier_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 total_deposits_normalized;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 total_borrows_normalized;
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
        struct RiskStore {
            int32 long_weight_initial;
            int32 short_weight_initial;
            int32 long_weight_maintenance;
            int32 short_weight_maintenance;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price_x18;
        }

        function addOrUpdateProduct(uint32 product_id, uint32 quote_id, int128 size_increment, int128 min_size, Config config, RiskStore risk_store) external;
        function getBalance(uint32 product_id, bytes32 subaccount) external view returns (Balance);
        function getConfig(uint32 product_id) external view returns (Config);
        function getNlpLockedBalances(bytes32 subaccount) external view returns (NlpLockedBalances);
        function getProductIds() external view returns (uint32[]);
        function getRawBalance(uint32 product_id, bytes32 subaccount) external view returns (BalanceNormalized);
        function getRawState(uint32 product_id) external view returns (State);
        function getSlots() external pure returns (uint256 balances_slot, uint256 states_slot);
        function getStateAndBalance(uint32 product_id, bytes32 subaccount) external view returns (State, Balance);
        function getTokenBalance(uint32 product_id) external view returns (uint128 balance);
        function setBalance(uint32 product_id, bytes32 subaccount, int128 amount) external;
        function setState(uint32 product_id, State state) external;
        function updateBalance(uint32 product_id, bytes32 subaccount, int128 amount_delta) external;
        function updatePrice(uint32 product_id, int128 price_x18) external;
    }
}
pub use SpotEngine::*;

#[cfg(test)]
mod wire_golden {
    use super::SpotEngine::{Balance, State};
    use rkyv::Deserialize as _;

    /// Locks the JSON wire format of `Balance` to the pre-migration ethers Abigen output:
    /// snake_case key and `int128` as a quoted decimal string. Guards against serde drift —
    /// the deserialized content and JSON field names external consumers depend on must stay
    /// identical across the alloy migration.
    #[test]
    fn balance_serde_is_wire_stable() {
        let balance = Balance { amount: -1234 };
        let json = serde_json::to_string(&balance).unwrap();
        assert_eq!(json, r#"{"amount":"-1234"}"#);
        let back: Balance = serde_json::from_str(&json).unwrap();
        assert_eq!(back, balance);
    }

    /// Locks the rkyv round-trip for `State` (one of the rkyv-serialized spot structs). The
    /// archived bytes must validate via `check_bytes` and rkyv-deserialize back to an equal
    /// value, ensuring the rkyv content layout is preserved across the alloy migration.
    #[test]
    fn state_rkyv_roundtrips() {
        let state = State {
            cumulative_deposits_multiplier_x18: 1,
            cumulative_borrows_multiplier_x18: -2,
            total_deposits_normalized: 3,
            total_borrows_normalized: -4,
        };
        let bytes = rkyv::to_bytes::<_, 256>(&state).unwrap();
        let archived = rkyv::check_archived_root::<State>(&bytes).unwrap();
        let back: State = archived.deserialize(&mut rkyv::Infallible).unwrap();
        assert_eq!(back, state);
    }
}
