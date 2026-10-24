use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract ContractOwner {
        function creditDepositV1(bytes32 subaccount) external;
        function wrapVaultAsset(bytes32 subaccount, uint32 productId) external;
        function isDirectDepositV1Ready(address recipient, bool isFirstDeposit) external returns (bool);
        function isWrapVaultAssetReady(address recipient, uint32 productId, bool isFirstDeposit) external returns (bool);
        function updateVenue(uint32 venueId, address operator) external;
        function getContractOwnerExt() external view returns (address);
    }
}
