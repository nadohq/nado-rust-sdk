use alloy::sol;

sol! {
    // ContractOwner's slow-mode admin functions, served by its fallback:
    // bind this at the ContractOwner proxy address, not at the delegate's.
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract ContractOwnerExt {
        function addNlpPool(address owner, uint128 balanceWeightX18) external;
        function updateNlpPool(uint64 poolId, address owner, uint128 weight) external;
        function deleteNlpPool(uint64 poolId) external;
        function forceRebalanceNlpPool(int128[] nlp_pool_rebalance_x18) external;
        function nlpProfitShare(uint64 pool_id, bytes32 recipient, uint128 amount) external;
        function updateTierFeeRates(uint32[] tier, uint32[] productId, int128[] makerRateX18, int128[] takerRateX18) external;
        function updateBuilders(uint32[] builderIds, address[] owners, uint32[] defaultFeeTiers, int128[] lowestFeeRates, int128[] highestFeeRates) external;
    }
}
