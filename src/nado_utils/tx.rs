use std::fmt::Debug;

use alloy::sol_types::SolValue;
use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::Eip712Domain as AlloyEip712Domain;
use ethers_core::types::{H160, U256 as EthersU256};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::bindings::endpoint;
use crate::eip712_alloy::NadoEip712;

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq,
)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum TxType {
    LiquidateSubaccount = 0,
    DepositCollateral = 1,
    WithdrawCollateral = 2,
    SpotTick = 3,
    UpdatePrice = 4,
    SettlePnl = 5,
    MatchOrders = 6,
    DepositInsurance = 7,
    ExecuteSlowMode = 8,
    DumpFees = 9,
    PerpTick = 10,
    ManualAssert = 11,
    UpdateProduct = 12,
    LinkSigner = 13,
    UpdateFeeTier = 14,
    TransferQuote = 15,
    RebalanceXWithdraw = 16,
    AssertCode = 17,
    WithdrawInsurance = 18,
    CreateIsolatedSubaccount = 19,
    DelistProduct = 20,
    MintNlp = 21,
    BurnNlp = 22,
    MatchOrdersWithAmount = 23,
    UpdateTierFeeRates = 24,
    AddNlpPool = 25,
    UpdateNlpPool = 26,
    DeleteNlpPool = 27,
    AssertProduct = 28,
    CloseIsolatedSubaccount = 29,
    UpdateBuilder = 30,
    ClaimBuilderFee = 31,
    WithdrawCollateralV2 = 32,
    ForceRebalanceNlpPool = 33,
    NlpProfitShare = 34,
}

impl TxType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::LiquidateSubaccount,
            1 => Self::DepositCollateral,
            2 => Self::WithdrawCollateral,
            3 => Self::SpotTick,
            4 => Self::UpdatePrice,
            5 => Self::SettlePnl,
            6 => Self::MatchOrders,
            7 => Self::DepositInsurance,
            8 => Self::ExecuteSlowMode,
            9 => Self::DumpFees,
            10 => Self::PerpTick,
            11 => Self::ManualAssert,
            12 => Self::UpdateProduct,
            13 => Self::LinkSigner,
            14 => Self::UpdateFeeTier,
            15 => Self::TransferQuote,
            16 => Self::RebalanceXWithdraw,
            17 => Self::AssertCode,
            18 => Self::WithdrawInsurance,
            19 => Self::CreateIsolatedSubaccount,
            20 => Self::DelistProduct,
            21 => Self::MintNlp,
            22 => Self::BurnNlp,
            23 => Self::MatchOrdersWithAmount,
            24 => Self::UpdateTierFeeRates,
            25 => Self::AddNlpPool,
            26 => Self::UpdateNlpPool,
            27 => Self::DeleteNlpPool,
            28 => Self::AssertProduct,
            29 => Self::CloseIsolatedSubaccount,
            30 => Self::UpdateBuilder,
            31 => Self::ClaimBuilderFee,
            32 => Self::WithdrawCollateralV2,
            33 => Self::ForceRebalanceNlpPool,
            34 => Self::NlpProfitShare,
            _ => panic!("Invalid TxType"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EIP712Domain(AlloyEip712Domain);

impl EIP712Domain {
    pub fn separator(&self) -> [u8; 32] {
        self.0.separator().0
    }

    pub(crate) fn as_alloy(&self) -> &AlloyEip712Domain {
        &self.0
    }
}

pub fn get_eip712_digest<T: NadoEip712 + Send + Sync + Debug>(
    payload: &T,
    domain: &EIP712Domain,
) -> [u8; 32] {
    payload.alloy_signing_hash(domain.as_alloy())
}

pub trait IntoAlloyU256 {
    fn into_alloy_u256(self) -> U256;
}

impl IntoAlloyU256 for U256 {
    fn into_alloy_u256(self) -> U256 {
        self
    }
}

impl IntoAlloyU256 for EthersU256 {
    fn into_alloy_u256(self) -> U256 {
        let mut bytes = [0u8; 32];
        self.to_big_endian(&mut bytes);
        U256::from_be_bytes(bytes)
    }
}

pub trait IntoAlloyAddress {
    fn into_alloy_address(self) -> Address;
}

impl IntoAlloyAddress for Address {
    fn into_alloy_address(self) -> Address {
        self
    }
}

impl IntoAlloyAddress for H160 {
    fn into_alloy_address(self) -> Address {
        Address::from_slice(self.as_bytes())
    }
}

pub fn domain(
    chain_id: impl IntoAlloyU256,
    verifying_contract: impl IntoAlloyAddress,
) -> EIP712Domain {
    EIP712Domain(AlloyEip712Domain {
        name: Some("Nado".into()),
        version: Some("0.0.1".into()),
        chain_id: Some(chain_id.into_alloy_u256()),
        verifying_contract: Some(verifying_contract.into_alloy_address()),
        salt: None,
    })
}

pub fn domain2(chain_id: u64, verifying_contract: [u8; 20]) -> EIP712Domain {
    domain(U256::from(chain_id), Address::from(verifying_contract))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NadoTx {
    LiquidateSubaccount(endpoint::LiquidateSubaccount),
    LinkSigner(endpoint::LinkSigner),

    DepositCollateral(endpoint::DepositCollateral),
    WithdrawCollateral(endpoint::WithdrawCollateral),
    WithdrawCollateralV2(endpoint::WithdrawCollateralV2),

    SettlePnl(endpoint::SettlePnl),
    PerpTick(endpoint::PerpTick),
    SpotTick(endpoint::SpotTick),
    UpdatePrice(endpoint::UpdatePrice),
    ManualAssert(endpoint::ManualAssert),

    MatchOrders(endpoint::MatchOrders),
    MatchOrdersWithAmount(endpoint::MatchOrdersWithAmount),
    ExecuteSlowMode,
    TransferQuote(endpoint::TransferQuote),
    RebalanceXWithdraw(endpoint::RebalanceXWithdraw),
    DumpFees,
    AssertCode(endpoint::AssertCode),
    WithdrawInsurance(endpoint::WithdrawInsurance),
    CreateIsolatedSubaccount(endpoint::CreateIsolatedSubaccount),
    DelistProduct(endpoint::DelistProduct),
    MintNlp(endpoint::MintNlp),
    BurnNlp(endpoint::BurnNlp),
    UpdateTierFeeRates(endpoint::UpdateTierFeeRates),
    UpdateFeeTier(endpoint::UpdateFeeTier),
    AddNlpPool(endpoint::AddNlpPool),
    UpdateNlpPool(endpoint::UpdateNlpPool),
    DeleteNlpPool(endpoint::DeleteNlpPool),
    AssertProduct(endpoint::AssertProduct),
    CloseIsolatedSubaccount(endpoint::CloseIsolatedSubaccount),
    UpdateBuilder(endpoint::UpdateBuilder),
    ClaimBuilderFee(endpoint::ClaimBuilderFee),
    ForceRebalanceNlpPool(endpoint::ForceRebalanceNlpPool),
    NlpProfitShare(endpoint::NlpProfitShare),
    Other,
}

impl NadoTx {
    pub fn tx_type(&self) -> TxType {
        match self {
            NadoTx::LiquidateSubaccount(_) => TxType::LiquidateSubaccount,
            NadoTx::DepositCollateral(_) => TxType::DepositCollateral,
            NadoTx::WithdrawCollateral(_) => TxType::WithdrawCollateral,
            NadoTx::WithdrawCollateralV2(_) => TxType::WithdrawCollateralV2,
            NadoTx::SpotTick(_) => TxType::SpotTick,
            NadoTx::UpdatePrice(_) => TxType::UpdatePrice,
            NadoTx::SettlePnl(_) => TxType::SettlePnl,
            NadoTx::MatchOrders(_) => TxType::MatchOrders,
            NadoTx::MatchOrdersWithAmount(_) => TxType::MatchOrdersWithAmount,
            NadoTx::ExecuteSlowMode => TxType::ExecuteSlowMode,
            NadoTx::ManualAssert(_) => TxType::ManualAssert,
            NadoTx::PerpTick(_) => TxType::PerpTick,
            NadoTx::LinkSigner(_) => TxType::LinkSigner,
            NadoTx::TransferQuote(_) => TxType::TransferQuote,
            NadoTx::RebalanceXWithdraw(_) => TxType::RebalanceXWithdraw,
            NadoTx::AssertCode(_) => TxType::AssertCode,
            NadoTx::WithdrawInsurance(_) => TxType::WithdrawInsurance,
            NadoTx::DelistProduct(_) => TxType::DelistProduct,
            NadoTx::DumpFees => TxType::DumpFees,
            NadoTx::CreateIsolatedSubaccount(_) => TxType::CreateIsolatedSubaccount,
            NadoTx::MintNlp(_) => TxType::MintNlp,
            NadoTx::BurnNlp(_) => TxType::BurnNlp,
            NadoTx::UpdateTierFeeRates(_) => TxType::UpdateTierFeeRates,
            NadoTx::UpdateFeeTier(_) => TxType::UpdateFeeTier,
            NadoTx::AddNlpPool(_) => TxType::AddNlpPool,
            NadoTx::UpdateNlpPool(_) => TxType::UpdateNlpPool,
            NadoTx::DeleteNlpPool(_) => TxType::DeleteNlpPool,
            NadoTx::AssertProduct(_) => TxType::AssertProduct,
            NadoTx::CloseIsolatedSubaccount(_) => TxType::CloseIsolatedSubaccount,
            NadoTx::UpdateBuilder(_) => TxType::UpdateBuilder,
            NadoTx::ClaimBuilderFee(_) => TxType::ClaimBuilderFee,
            NadoTx::ForceRebalanceNlpPool(_) => TxType::ForceRebalanceNlpPool,
            NadoTx::NlpProfitShare(_) => TxType::NlpProfitShare,
            NadoTx::Other => panic!("Other is not a valid tx type"),
        }
    }
}

pub fn assert_product(
    product_id: u32,
    is_spot: bool,
    quote_id: u32,
    size_increment: i128,
    min_size: i128,
    others_hash: [u8; 32],
) -> Bytes {
    Bytes::from(
        [
            vec![TxType::AssertProduct as u8],
            (endpoint::AssertProduct {
                product_id,
                is_spot,
                quote_id,
                size_increment,
                min_size,
                others_hash: others_hash.into(),
            },)
                .abi_encode_params(),
        ]
        .concat(),
    )
}
