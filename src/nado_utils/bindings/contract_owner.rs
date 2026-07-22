use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract ContractOwner {
        function creditDepositV1(bytes32 subaccount) external;
        function wrapVaultAsset(bytes32 subaccount, uint32 productId) external;
        function addNlpPool(address owner, uint128 balanceWeightX18) external;
        function updateNlpPool(uint64 poolId, address owner, uint128 weight) external;
        function deleteNlpPool(uint64 poolId) external;
        function isDirectDepositV1Ready(address recipient, bool isFirstDeposit) external returns (bool);
        function isWrapVaultAssetReady(address recipient, uint32 productId, bool isFirstDeposit) external returns (bool);
        function forceRebalanceNlpPool(int128[] nlp_pool_rebalance_x18) external;
        function nlpProfitShare(uint64 pool_id, bytes32 recipient, uint128 amount) external;
    }
}
