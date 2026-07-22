use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract Verifier {
        struct Point {
            uint256 x;
            uint256 y;
        }
        function getPubkey(uint8 index) external view returns (Point);
        function getPubkeyAddress(uint8 index) external view returns (address);
        function checkIndividualSignature(bytes32 digest, bytes signature, uint8 signerIndex) external view returns (bool);
        function requireValidSignature(bytes32 message, bytes32 e, bytes32 s, uint8 signerBitmask) external;
        function requireValidTxSignatures(bytes txn, uint64 idx, bytes[] signatures) external view;
        function computeDigest(uint8 txType, bytes transactionBody) external pure returns (bytes32);
    }
}

pub use Verifier::Point;
