use alloy::sol;

sol! {
    #![sol(all_derives)]
    #[sol(rpc)]
    #[allow(missing_docs, clippy::too_many_arguments)]
    contract Endpoint {
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
        struct MatchOrdersWithAmount {
            MatchOrders match_orders;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 taker_amount_delta;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Times {
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 perp_time;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 spot_time;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Cancellation {
            bytes32 sender;
            uint32[] product_ids;
            bytes32[] digests;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct CancellationProducts {
            bytes32 sender;
            uint32[] product_ids;
            uint64 nonce;
        }

        struct ChainlinkFullReport {
            bytes32[3] report_context;
            bytes report_blob;
            bytes32[] raw_rs;
            bytes32[] raw_ss;
            bytes32 raw_vs;
        }

        struct ChainlinkReportBlob {
            bytes32 feed_id;
            uint32 valid_from_timestamp;
            uint32 observations_timestamp;
            uint192 native_fee;
            uint192 link_fee;
            uint64 expires_at;
            int192 price;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedCancellation {
            Cancellation cancellation;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedCancellationProducts {
            CancellationProducts cancellation_products;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct AddNlpPool {
            address owner;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 balance_weight_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct AssertCode {
            string[] contract_names;
            bytes32[] code_hashes;
            #[serde(serialize_with = "crate::bindings::serde_helpers::serialize_u256", deserialize_with = "crate::bindings::serde_helpers::deserialize_u256")]
            uint256 spreads;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct AssertProduct {
            uint32 product_id;
            bool is_spot;
            uint32 quote_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 size_increment;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 min_size;
            bytes32 others_hash;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct BurnNlp {
            bytes32 sender;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 nlp_amount;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct ClaimBuilderFee {
            bytes32 sender;
            uint32 builder_id;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct CloseIsolatedSubaccount {
            bytes32 subaccount;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct CompactSignature {
            bytes32 r;
            bytes32 vs;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct CreateIsolatedSubaccount {
            Order order;
            uint32 product_id;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct DeleteNlpPool {
            uint64 pool_id;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct ForceRebalanceNlpPool {
            #[serde(serialize_with = "crate::serialize_utils::serialize_vec_i128", deserialize_with = "crate::serialize_utils::deserialize_vec_i128")]
            int128[] nlp_pool_rebalance_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct NlpProfitShare {
            uint64 pool_id;
            bytes32 recipient;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct DelistProduct {
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price_x18;
            bytes32[] subaccounts;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct DepositCollateral {
            bytes32 sender;
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct DepositInsurance {
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct LinkSigner {
            bytes32 sender;
            bytes32 signer;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct LiquidateSubaccount {
            bytes32 sender;
            bytes32 liquidatee;
            uint32 product_id;
            bool is_encoded_spread;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 amount;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct ManualAssert {
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 insurance;
            bytes[] spot_states;
            bytes[] perp_states;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct MintNlp {
            bytes32 sender;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 quote_amount;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct NlpPool {
            uint64 pool_id;
            bytes32 subaccount;
            address owner;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 balance_weight_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct PerpTick {
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 time;
            #[serde(serialize_with = "crate::serialize_utils::serialize_vec_i128", deserialize_with = "crate::serialize_utils::deserialize_vec_i128")]
            int128[] avg_price_diffs;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct RebalanceXWithdraw {
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
            address send_to;
        }

        struct Rebate {
            bytes32[] subaccounts;
            int128[] amounts;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SettlePnl {
            bytes32[] subaccounts;
            #[serde(serialize_with = "crate::bindings::serde_helpers::serialize_vec_u256_hex", deserialize_with = "crate::bindings::serde_helpers::deserialize_vec_u256_hex")]
            uint256[] product_ids;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedBurnNlp {
            BurnNlp tx;
            bytes signature;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 oracle_price_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_vec_i128", deserialize_with = "crate::serialize_utils::deserialize_vec_i128")]
            int128[] nlp_pool_rebalance_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedLinkSigner {
            LinkSigner tx;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedLiquidateSubaccount {
            LiquidateSubaccount tx;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedMintNlp {
            MintNlp tx;
            bytes signature;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 oracle_price_x18;
            #[serde(serialize_with = "crate::serialize_utils::serialize_vec_i128", deserialize_with = "crate::serialize_utils::deserialize_vec_i128")]
            int128[] nlp_pool_rebalance_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedTransferQuote {
            TransferQuote tx;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedWithdrawCollateral {
            WithdrawCollateral tx;
            bytes signature;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SignedWithdrawCollateralV2 {
            WithdrawCollateralV2 tx;
            CompactSignature signature;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 fee_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SlowModeConfig {
            uint64 timeout;
            uint64 tx_count;
            uint64 tx_up_to;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SlowModeTx {
            uint64 executable_at;
            address sender;
            bytes tx;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct SpotTick {
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 time;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct TransferQuote {
            bytes32 sender;
            bytes32 recipient;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct UpdateBuilder {
            uint32 builder_id;
            address owner;
            uint32 default_fee_tier;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 lowest_fee_rate;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 highest_fee_rate;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct UpdateFeeTier {
            address user;
            uint32 new_tier;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct UpdateNlpPool {
            uint64 pool_id;
            address owner;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 balance_weight_x18;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct UpdatePrice {
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_i128", deserialize_with = "crate::serialize_utils::deserialize_i128")]
            int128 price_x18;
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
        struct WithdrawCollateral {
            bytes32 sender;
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
            uint64 nonce;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct WithdrawCollateralV2 {
            bytes32 sender;
            uint32 product_id;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
            uint64 nonce;
            address send_to;
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 appendix;
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct WithdrawInsurance {
            #[serde(serialize_with = "crate::serialize_utils::serialize_u128", deserialize_with = "crate::serialize_utils::deserialize_u128")]
            uint128 amount;
            address send_to;
        }

        function addNlpPool(AddNlpPool p) external pure returns (AddNlpPool);
        function assertCode(AssertCode p) external pure returns (AssertCode);
        function assertProduct(AssertProduct p) external pure returns (AssertProduct);
        function chainlinkFullReport(ChainlinkFullReport p) external pure returns (ChainlinkFullReport);
        function chainlinkReportBlob(ChainlinkReportBlob p) external pure returns (ChainlinkReportBlob);
        function checkSlowModeTxInner(address sender, bytes transaction) external pure returns (bytes32);
        function checkSlowModeTxLinkSigner() external view returns (bytes32);
        function claimBuilderFee(ClaimBuilderFee p) external pure returns (ClaimBuilderFee);
        function clearinghouse() external view returns (address);
        function closeIsolatedSubaccount(CloseIsolatedSubaccount p) external pure returns (CloseIsolatedSubaccount);
        function createIsolatedSubaccount(CreateIsolatedSubaccount p) external pure returns (CreateIsolatedSubaccount);
        function deleteNlpPool(DeleteNlpPool p) external pure returns (DeleteNlpPool);
        function depositCollateral(bytes12 subaccount_name, uint32 product_id, uint128 amount) external;
        function depositCollateralWithReferral(bytes32 subaccount, uint32 product_id, uint128 amount, string referral) external;
        function executeSlowModeTransaction() external;
        function getEndpointTx() external view returns (address);
        function getHealthCheckFee() external pure returns (int128);
        function getLinkedSigner(bytes32 subaccount) external view returns (address);
        function getLiquidationFee() external pure returns (int128);
        function getNlpPools() external view returns (NlpPool[]);
        function getNonce(address sender) external view returns (uint64);
        function getNumSubaccounts() external view returns (uint64);
        function getOffchainExchange() external view returns (address);
        function getPriceX18(uint32 product_id) external returns (int128 price_x18);
        function getSequencer() external view returns (address);
        function getSequencerFee(uint32 product_id) external view returns (int128);
        function getSlots() external pure returns (uint256 n_submissions_slot);
        function getSlowModeTx(uint64 idx) external view returns (SlowModeTx, uint64, uint64);
        function getSubaccountById(uint64 subaccount_id) external view returns (bytes32);
        function getSubaccountId(bytes32 subaccount) external view returns (uint64);
        function getTakerSequencerFee() external pure returns (int128);
        function getTime() external view returns (uint128);
        function getTimes() external view returns (Times);
        function incrementSubmissions() external returns (uint64);
        function initialize(address _sanctions, address _sequencer, address _offchain_exchange, address _clearinghouse, address _verifier, address _endpoint_tx) external;
        function liquidationStart(bytes transaction) external;
        function manualAssert(ManualAssert p) external pure returns (ManualAssert);
        function matchOrders(MatchOrders p) external pure returns (MatchOrders);
        function matchOrdersWithAmount(MatchOrdersWithAmount p) external pure returns (MatchOrdersWithAmount);
        function nSubmissions() external view returns (uint64);
        function nlpPools(uint256) external view returns (uint64 pool_id, bytes32 subaccount, address owner, uint128 balance_weight_x18);
        function owner() external view returns (address);
        function perpTick(PerpTick p) external pure returns (PerpTick);
        function processSlowModeTransaction(address sender, bytes transaction) external;
        function processSlowModeTransactionImpl(address sender, bytes transaction) external;
        function processTransactionImpl(bytes transaction) external;
        function rebalanceXWithdraw(RebalanceXWithdraw p) external pure returns (RebalanceXWithdraw);
        function rebate(Rebate p) external pure returns (Rebate);
        function referralCodes(address) external view returns (string);
        function renounceOwnership() external;
        function resyncSlowModeTxs() external;
        function setInitialPrice(uint32 product_id, int128 initial_price_x18) external;
        function setPriceX18(uint32 product_id, int128 price_x18) external;
        function setSlowModeConfig(SlowModeConfig slow_mode_config) external;
        function setSlowModeTx(uint64 idx, SlowModeTx txn) external;
        function settlePnl(SettlePnl p) external pure returns (SettlePnl);
        function signedBurnNlp(SignedBurnNlp p) external pure returns (SignedBurnNlp);
        function signedCancellation(SignedCancellation p) external pure returns (SignedCancellation);
        function signedCancellationProducts(SignedCancellationProducts p) external pure returns (SignedCancellationProducts);
        function signedLinkSigner(SignedLinkSigner p) external pure returns (SignedLinkSigner);
        function signedLiquidateSubaccount(SignedLiquidateSubaccount p) external pure returns (SignedLiquidateSubaccount);
        function signedMintNlp(SignedMintNlp p) external pure returns (SignedMintNlp);
        function signedOrder(SignedOrder p) external pure returns (SignedOrder);
        function signedTransferQuote(SignedTransferQuote p) external pure returns (SignedTransferQuote);
        function signedWithdrawCollateral(SignedWithdrawCollateral p) external pure returns (SignedWithdrawCollateral);
        function signedWithdrawCollateralV2(SignedWithdrawCollateralV2 p) external pure returns (SignedWithdrawCollateralV2);
        function spotTick(SpotTick p) external pure returns (SpotTick);
        function submitSlowModeTransaction(bytes transaction) external;
        function submitSlowModeTransactionImpl(bytes transaction) external;
        function submitTransactions(bytes[] transactions) external;
        function submitTransactionsChecked(uint64 idx, bytes[] transactions, bytes32 e, bytes32 s, uint8 signer_bitmask) external;
        function submitTransactionsCheckedWithGasLimit(uint64 idx, bytes[] transactions, uint256 gas_limit) external;
        function transferOwnership(address new_owner) external;
        function transferQuote(TransferQuote p) external pure returns (TransferQuote);
        function unsignedBurnNlp(BurnNlp p) external pure returns (BurnNlp);
        function unsignedDelistProduct(DelistProduct p) external pure returns (DelistProduct);
        function unsignedDepositCollateral(DepositCollateral p) external pure returns (DepositCollateral);
        function unsignedDepositInsurance(DepositInsurance p) external pure returns (DepositInsurance);
        function unsignedLinkSigner(LinkSigner p) external pure returns (LinkSigner);
        function unsignedLiquidateSubaccount(LiquidateSubaccount p) external pure returns (LiquidateSubaccount);
        function unsignedMintNlp(MintNlp p) external pure returns (MintNlp);
        function unsignedTransferQuote(TransferQuote p) external pure returns (TransferQuote);
        function unsignedUpdateTierFeeRates(UpdateTierFeeRates p) external pure returns (UpdateTierFeeRates);
        function unsignedWithdrawCollateral(WithdrawCollateral p) external pure returns (WithdrawCollateral);
        function unsignedWithdrawInsurance(WithdrawInsurance p) external pure returns (WithdrawInsurance);
        function updateBuilder(UpdateBuilder p) external pure returns (UpdateBuilder);
        function updateFeeTier(UpdateFeeTier p) external pure returns (UpdateFeeTier);
        function updateNlpPool(UpdateNlpPool p) external pure returns (UpdateNlpPool);
        function updatePrice(UpdatePrice p) external pure returns (UpdatePrice);
        function upgradeEndpointTx(address _endpoint_tx) external;
        function forceRebalanceNlpPool(ForceRebalanceNlpPool p) external pure returns (ForceRebalanceNlpPool);
        function nlpProfitShare(NlpProfitShare p) external pure returns (NlpProfitShare);
    }
}
pub use Endpoint::*;
