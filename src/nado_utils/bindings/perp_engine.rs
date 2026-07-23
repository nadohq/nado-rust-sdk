use alloy::sol;

sol! {
    #![sol(all_derives)]
    #[sol(rpc)]
    #[allow(missing_docs, clippy::too_many_arguments)]
    contract PerpEngine {
        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct Balance {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 amount;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 v_quote_balance;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 last_cumulative_funding_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive(check_bytes)]
        struct State {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 cumulative_funding_long_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 cumulative_funding_short_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 available_settle;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 open_interest;
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

        function addOrUpdateProduct(uint32 product_id, int128 size_increment, int128 min_size, RiskStore risk_store) external;
        function balances(uint32, bytes32) external view returns (int128 amount, int128 v_quote_balance, int128 last_cumulative_funding_x18);
        function emitBalanceUpdate(uint32 product_id, bytes32 subaccount) external;
        function getAvailableSettle(uint32 product_id) external view returns (int128);
        function getBalance(uint32 product_id, bytes32 subaccount) external view returns (Balance);
        function getClearinghouse() external view returns (address);
        function getCoreRisk(bytes32 subaccount, uint32 product_id, uint8 health_type) external returns (CoreRisk);
        function getEndpoint() external view returns (address);
        function getEngineType() external pure returns (uint8);
        function getHealthContribution(bytes32 subaccount, uint8 health_type) external returns (int128 health);
        function getPositionPnl(uint32 product_id, bytes32 subaccount) external returns (int128);
        function getProductIds() external view returns (uint32[]);
        function getRawBalance(uint32 product_id, bytes32 subaccount) external view returns (Balance);
        function getRawState(uint32 product_id) external view returns (State);
        function getRisk(uint32 product_id) external returns (Risk);
        function getSettlementState(uint32 product_id, bytes32 subaccount) external returns (int128 available_settle, State state, Balance balance);
        function getSlots() external pure returns (uint256 balances_slot, uint256 states_slot);
        function getStateAndBalance(uint32 product_id, bytes32 subaccount) external view returns (State, Balance);
        function initialize(address _clearinghouse, address _offchainExchange, address, address _endpoint, address _admin) external;
        function manualAssert(bytes[] _states) external view;
        function migrationFlag() external view returns (uint128);
        function owner() external view returns (address);
        function perpPositionClosed(uint32 product_id, bytes32 subaccount) external view returns (bool);
        function renounceOwnership() external;
        function setBalance(uint32 product_id, bytes32 subaccount, Balance balance) external;
        function setState(uint32 product_id, State state) external;
        function settlePnl(bytes32 subaccount, uint256 product_ids) external returns (int128);
        function socializeSubaccount(bytes32 subaccount, int128 insurance) external returns (int128);
        function states(uint32) external view returns (int128 cumulative_funding_long_x18, int128 cumulative_funding_short_x18, int128 available_settle, int128 open_interest);
        function transferOwnership(address new_owner) external;
        function updateBalance(uint32 product_id, bytes32 subaccount, int128 amount_delta, int128 v_quote_delta) external;
        function updatePrice(uint32 product_id, int128 price_x18) external;
        function updateRisk(uint32 product_id, RiskStore risk_store) external;
        function updateStates(uint128 dt, int128[] avg_price_diffs) external;
    }
}
pub use PerpEngine::*;

#[cfg(test)]
mod wire_golden {
    use super::PerpEngine::{Balance, Risk};
    use rkyv::Deserialize as _;

    /// Locks the JSON wire format of `Balance` to the pre-migration ethers Abigen output:
    /// snake_case keys and `int128` as quoted decimal strings. This guards against accidental
    /// serde drift during the alloy migration — the deserialized content (and the JSON field
    /// names external consumers depend on) must stay identical.
    #[test]
    fn balance_serde_is_wire_stable() {
        let balance = Balance {
            amount: 1000,
            v_quote_balance: -50,
            last_cumulative_funding_x18: 7,
        };
        let json = serde_json::to_string(&balance).unwrap();
        assert_eq!(
            json,
            r#"{"amount":"1000","v_quote_balance":"-50","last_cumulative_funding_x18":"7"}"#
        );
        let back: Balance = serde_json::from_str(&json).unwrap();
        assert_eq!(back, balance);
    }

    /// Locks the rkyv round-trip for `Risk` (one of the rkyv-serialized perp structs). The
    /// archived bytes must validate via `check_bytes` and rkyv-deserialize back to an equal
    /// value, ensuring the rkyv content layout is preserved across the alloy migration.
    #[test]
    fn risk_rkyv_roundtrips() {
        let risk = Risk {
            long_weight_initial_x18: 1,
            short_weight_initial_x18: -2,
            long_weight_maintenance_x18: 3,
            short_weight_maintenance_x18: -4,
            price_x18: 123_456_789,
        };
        let bytes = rkyv::to_bytes::<_, 256>(&risk).unwrap();
        let archived = rkyv::check_archived_root::<Risk>(&bytes).unwrap();
        let back: Risk = archived.deserialize(&mut rkyv::Infallible).unwrap();
        assert_eq!(back, risk);
    }
}
