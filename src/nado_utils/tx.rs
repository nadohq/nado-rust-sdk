use std::fmt::Debug;

use ethers::prelude::*;
use ethers::types::transaction::eip712::Eip712;
use ethers_core::types::transaction::eip712::EIP712Domain;
use ethers_core::utils::keccak256;
use serde::{Deserialize, Serialize};

use crate::bindings::endpoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
    MintVlp = 21,
    BurnVlp = 22,
    RebalanceVlp = 23,
    MatchOrdersWithAmount = 24,
    UpdateTierFeeRates = 25,
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
            21 => Self::MintVlp,
            22 => Self::BurnVlp,
            23 => Self::RebalanceVlp,
            24 => Self::MatchOrdersWithAmount,
            25 => Self::UpdateTierFeeRates,
            _ => panic!("Invalid TxType"),
        }
    }
}

pub fn get_eip712_digest<T: Eip712 + Send + Sync + Debug>(
    payload: &T,
    domain: &EIP712Domain,
) -> H256 {
    let domain_separator = domain.separator();
    let struct_hash = payload.struct_hash().unwrap();
    let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();
    H256::from(keccak256(digest_input))
}

pub fn domain(chain_id: U256, verifying_contract: H160) -> EIP712Domain {
    EIP712Domain {
        name: Some("Nado".to_string()),
        version: Some("0.0.1".to_string()),
        chain_id: Some(chain_id),
        verifying_contract: Some(verifying_contract),
        salt: None,
    }
}

pub fn domain2(chain_id: u64, verifying_contract: [u8; 20]) -> EIP712Domain {
    domain(U256::from(chain_id), verifying_contract.into())
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NadoTx {
    LiquidateSubaccount(endpoint::LiquidateSubaccount),
    LinkSigner(endpoint::LinkSigner),

    DepositCollateral(endpoint::DepositCollateral),
    WithdrawCollateral(endpoint::WithdrawCollateral),

    SettlePnl(endpoint::SettlePnl),
    PerpTick(endpoint::PerpTick),
    SpotTick(endpoint::SpotTick),
    UpdatePrice(endpoint::UpdatePrice),
    ManualAssert(endpoint::ManualAssert),

    MatchOrders(endpoint::MatchOrders),
    ExecuteSlowMode,
    TransferQuote(endpoint::TransferQuote),
    RebalanceXWithdraw(endpoint::RebalanceXWithdraw),
    DumpFees,
    AssertCode(endpoint::AssertCode),
    WithdrawInsurance(endpoint::WithdrawInsurance),
    CreateIsolatedSubaccount(endpoint::CreateIsolatedSubaccount),
    DelistProduct(endpoint::DelistProduct),
    MintVlp(endpoint::MintVlp),
    BurnVlp(endpoint::BurnVlp),
    RebalanceVlp(endpoint::RebalanceVlp),
    UpdateTierFeeRates(endpoint::UpdateTierFeeRates),
    Other,
}

impl NadoTx {
    pub fn tx_type(&self) -> TxType {
        match self {
            NadoTx::LiquidateSubaccount(_) => TxType::LiquidateSubaccount,
            NadoTx::DepositCollateral(_) => TxType::DepositCollateral,
            NadoTx::WithdrawCollateral(_) => TxType::WithdrawCollateral,
            NadoTx::SpotTick(_) => TxType::SpotTick,
            NadoTx::UpdatePrice(_) => TxType::UpdatePrice,
            NadoTx::SettlePnl(_) => TxType::SettlePnl,
            NadoTx::MatchOrders(_) => TxType::MatchOrders,
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
            NadoTx::MintVlp(_) => TxType::MintVlp,
            NadoTx::BurnVlp(_) => TxType::BurnVlp,
            NadoTx::RebalanceVlp(_) => TxType::RebalanceVlp,
            NadoTx::UpdateTierFeeRates(_) => TxType::UpdateTierFeeRates,
            NadoTx::Other => panic!("Other is not a valid tx type"),
        }
    }
}
