use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract GasInfo {
        function getPricesInWei() external pure returns (
            uint256, uint256, uint256, uint256, uint256, uint256
        );
        function getL1Fee(bytes _data) external view returns (uint256);
    }
}
