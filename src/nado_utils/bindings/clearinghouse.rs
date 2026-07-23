use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract Clearinghouse {
        struct LiquidateSubaccount {
            bytes32 sender;
            bytes32 liquidatee;
            uint32 productId;
            bool isEncodedSpread;
            int128 amount;
            uint64 nonce;
        }
        struct MintNlp {
            bytes32 sender;
            uint128 quoteAmount;
            uint64 nonce;
        }
        struct BurnNlp {
            bytes32 sender;
            uint128 nlpAmount;
            uint64 nonce;
        }
        struct NlpPool {
            uint64 poolId;
            bytes32 subaccount;
            address owner;
            uint128 balanceWeightX18;
        }

        function getWithdrawPool() external view returns (address);
        function getHealth(bytes32 subaccount, uint8 healthType) external returns (int128 health);
        function getInsurance() external view returns (int128);
        function getSpreads() external view returns (uint256);

        function setSpreads(uint256 spreads) external;
        function setDecimals(uint32 productId, uint8 dec) external;
        function initialize(
            address endpoint,
            address quote,
            address clearinghouseLiq,
            uint256 spreads,
            address withdrawPool
        ) external;
        function addEngine(address engine, address offchainExchange, uint8 engineType) external;

        function liqFinalizeSubaccount(LiquidateSubaccount txn) external returns (bool);
        function liqSettleAgainstLiquidator(LiquidateSubaccount txn) external;
        function liqLiquidationPayment(LiquidateSubaccount txn) external;

        function mintNlp(
            MintNlp txn,
            int128 oraclePriceX18,
            NlpPool[] nlpPools,
            int128[] nlpPoolRebalanceX18
        ) external;
        function burnNlp(
            BurnNlp txn,
            int128 oraclePriceX18,
            NlpPool[] nlpPools,
            int128[] nlpPoolRebalanceX18
        ) external;
    }
}
