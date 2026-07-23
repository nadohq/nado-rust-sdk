use alloy::sol;

sol! {
    #![sol(all_derives)]
    #[sol(rpc)]
    #[allow(missing_docs, clippy::too_many_arguments)]
    contract OffchainExchange {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Order {
            bytes32 sender;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 amount;
            uint64 expiration;
            uint64 nonce;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 appendix;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedOrder {
            Order order;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct MatchOrders {
            uint32 product_id;
            SignedOrder taker;
            SignedOrder maker;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct MatchOrdersWithSigner {
            MatchOrders match_orders;
            address taker_linked_signer;
            address maker_linked_signer;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 taker_amount_delta;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct CreateIsolatedSubaccount {
            Order order;
            uint32 product_id;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct UpdateTierFeeRates {
            uint32 tier;
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 maker_rate_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 taker_rate_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Builder {
            address owner;
            uint32 default_fee_tier;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 lowest_fee_rate;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 highest_fee_rate;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct FeeRates {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 maker_rate_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 taker_rate_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct MarketInfo {
            uint32 quote_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 min_size;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 size_increment;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 collected_fees;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct MarketInfoStore {
            int64 min_size;
            int64 size_increment;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 collected_fees;
        }

        function assertProduct(bytes transaction) external;
        function claimBuilderFee(bytes32 sender, uint32 builder_id) external;
        function createIsolatedSubaccount(CreateIsolatedSubaccount txn, address linked_signer) external returns (bytes32);
        function dropDigest(bytes32 digest) external;
        function dropOrder(uint32 product_id, Order order) external;
        function dropOrderChecked(uint32 product_id, Order order) external;
        function dumpFees() external;
        function filledAmounts(bytes32 digest) external view returns (int128);
        function getAllFeeTiers(address[] users) external view returns (uint32[]);
        function getBuilder(uint32 builder_id) external view returns (Builder);
        function getClaimableBuilderFee(uint32 quote_id, uint32 builder_id) external view returns (int128);
        function getCollectedFees(uint32 product_id) external view returns (int128 collected_maker_fees, int128 collected_taker_fees, int128 collected_maker_builder_fees, int128 collected_taker_builder_fees);
        function getCustomFeeAddresses(uint32 start_at, uint32 limit) external view returns (address[]);
        function getDigest(uint32 product_id, Order order) external view returns (bytes32);
        function getEndpoint() external view returns (address);
        function getIsolatedSubaccountByDigest(bytes32 digest) external view returns (bytes32);
        function getIsolatedSubaccounts(bytes32 subaccount) external view returns (bytes32[]);
        function getIsolatedSubaccountsMask(address wallet) external view returns (uint256);
        function getMarginByDigest(bytes32 digest) external view returns (int128);
        function getMarketInfo(uint32 product_id) external view returns (MarketInfo m);
        function getMinSize(uint32 product_id) external view returns (int128);
        function getOrderFilledAmounts(uint32 product_id, Order order1, Order order2) external view returns (int128, int128);
        function getParentSubaccount(bytes32 subaccount) external view returns (bytes32);
        function getRawMarketInfo(uint32 product_id) external view returns (MarketInfoStore);
        function getSizeIncrement(uint32 product_id) external view returns (int128);
        function getSlots() external pure returns (uint256 filled_amounts_slot, uint256 taker_fees_slot, uint256 maker_fees_slot, uint256 market_info_slot, uint256 collected_builder_fee_slot, uint256 digest_to_subaccount_slot, uint256 digest_to_margin_slot);
        function getTierFeeRateX18(uint32 tier, uint32 product_id) external view returns (FeeRates);
        function getUserFeeTier(address sender) external view returns (uint32);
        function incrementFees(uint32 product_id, int128 maker_fee, int128 taker_fee, uint32 quote_id, uint32 maker_builder_id, uint32 taker_builder_id, int128 maker_builder_fee, int128 taker_builder_fee) external;
        function initialize(address _clearinghouse, address _endpoint) external;
        function isIsolatedSubaccountActive(bytes32 parent, bytes32 subaccount) external view returns (bool);
        function matchOrders(MatchOrdersWithSigner txn) external;
        function modifyFilledAmount(bytes32 taker_digest, bytes32 maker_digest, int128 taker_delta) external returns (int128, int128);
        function orderVersion() external pure returns (uint128);
        function owner() external view returns (address);
        function renounceOwnership() external;
        function setFeeTier(address user, uint32 tier) external;
        function setFilledAmount(bytes32 digest, int128 filled_amount) external;
        function transferOwnership(address new_owner) external;
        function tryCloseIsolatedSubaccount(bytes32 subaccount) external;
        function updateBuilder(bytes transaction) external;
        function updateCollectedFees(uint32 product_id, int128 collected_fees) external;
        function updateFeeTier(address user, uint32 new_tier) external;
        function updateMarket(uint32 product_id, uint32 quote_id, int128 size_increment, int128 min_size) external;
        function updateTierFeeRates(UpdateTierFeeRates txn) external;
    }
}
pub use OffchainExchange::*;

#[cfg(test)]
mod wire_golden {
    use super::OffchainExchange::Order;

    /// Locks the JSON wire format of `Order` to the pre-migration ethers Abigen
    /// output: snake_case keys, `bytes32` as lowercase 0x-hex, and `int128` /
    /// `uint128` as quoted decimal strings. This guards against accidental serde
    /// drift during the alloy migration — the deserialized content (and the JSON
    /// field names external consumers depend on) must stay identical.
    #[test]
    fn order_serde_is_wire_stable() {
        let order = Order {
            sender: [0x11u8; 32].into(),
            price_x18: 1000,
            amount: -50,
            expiration: 7,
            nonce: 9,
            appendix: 3,
        };
        let json = serde_json::to_string(&order).unwrap();
        assert_eq!(
            json,
            r#"{"sender":"0x1111111111111111111111111111111111111111111111111111111111111111","price_x18":"1000","amount":"-50","expiration":7,"nonce":9,"appendix":"3"}"#
        );
        let back: Order = serde_json::from_str(&json).unwrap();
        assert_eq!(back, order);
    }
}
