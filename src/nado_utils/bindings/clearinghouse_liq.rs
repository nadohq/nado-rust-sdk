use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract ClearinghouseLiq {
        function getEndpoint() external view returns (address);
        function owner() external view returns (address);
        function isAboveInitial(bytes32 subaccount) external returns (bool);
        function isUnderInitial(bytes32 subaccount) external returns (bool);
        function transferOwnership(address newOwner) external;
        function renounceOwnership() external;
    }
}
