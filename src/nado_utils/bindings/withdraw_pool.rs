use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract WithdrawPool {
        function submitFastWithdrawal(uint64 idx, bytes transaction, bytes[] signatures) external;
        function submitWithdrawal(address token, address sendTo, uint128 amount, uint64 idx) external;
        function checkProductBalances(uint32[] productIds) external view returns (uint256[]);
        function checkMarkedIdxs(uint64[] idxs) external view returns (bool[]);
        function fees(uint32) external view returns (int128);
        function fastWithdrawalFeeAmount(address token, uint32 productId, uint128 amount) external view returns (int128);
        function minIdx() external view returns (uint64);
        function markedIdxs(uint64) external view returns (bool);
        function removeLiquidity(uint32 productId, uint128 amount, address sendTo) external;
    }
}
