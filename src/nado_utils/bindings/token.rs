use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract EngineToken {
        function decimals() external view returns (uint8);
        function initialize(uint8 decimals_) external;
    }
}
