#![allow(non_snake_case)]

use alloy_primitives::{Address, FixedBytes};
use alloy_sol_types::SolStruct;

use crate::eip712_structs::{
    BurnNlp, Cancellation, CancellationProducts, DependencyUpdate, LeaderboardAuthentication,
    LinkSigner, LiquidateSubaccount, ListTriggerOrders, MintNlp, NadoAuthentication, Order,
    SocialAuthentication, StreamAuthentication, TransferQuote, WithdrawCollateral,
    WithdrawCollateralV2,
};

pub mod sol_types {
    alloy_sol_types::sol! {
        struct Order { bytes32 sender; int128 priceX18; int128 amount; uint64 expiration; uint64 nonce; uint128 appendix; }
        struct Cancellation { bytes32 sender; uint32[] productIds; bytes32[] digests; uint64 nonce; }
        struct CancellationProducts { bytes32 sender; uint32[] productIds; uint64 nonce; }
        struct LinkSigner { bytes32 sender; bytes32 signer; uint64 nonce; }
        struct LiquidateSubaccount { bytes32 sender; bytes32 liquidatee; uint32 productId; bool isEncodedSpread; int128 amount; uint64 nonce; }
        struct WithdrawCollateral { bytes32 sender; uint32 productId; uint128 amount; uint64 nonce; }
        struct WithdrawCollateralV2 { bytes32 sender; uint32 productId; uint128 amount; uint64 nonce; address sendTo; uint128 appendix; }
        struct MintNlp { bytes32 sender; uint128 quoteAmount; uint64 nonce; }
        struct BurnNlp { bytes32 sender; uint128 nlpAmount; uint64 nonce; }
        struct TransferQuote { bytes32 sender; bytes32 recipient; uint128 amount; uint64 nonce; }
        struct ListTriggerOrders { bytes32 sender; uint64 recvTime; }
        struct DependencyUpdate { bytes32 sender; bytes32 oldDigest; bytes32 newDigest; uint64 nonce; }
        struct StreamAuthentication { bytes32 sender; uint64 expiration; }
        struct NadoAuthentication { string method; bytes32 sender; bytes32 payloadHash; uint64 nonce; }
        struct LeaderboardAuthentication { bytes32 sender; uint64 expiration; uint32[] contestIds; }
        struct SocialAuthentication { bytes32 sender; uint64 expiration; string provider; }
    }
}

pub trait NadoEip712 {
    type Sol: SolStruct;
    fn to_sol(&self) -> Self::Sol;
    fn alloy_signing_hash(&self, domain: &alloy_sol_types::Eip712Domain) -> [u8; 32] {
        self.to_sol().eip712_signing_hash(domain).0
    }
}

#[inline]
fn fb(b: [u8; 32]) -> FixedBytes<32> {
    b.into()
}

impl NadoEip712 for Order {
    type Sol = sol_types::Order;
    fn to_sol(&self) -> Self::Sol {
        sol_types::Order {
            sender: fb(self.sender),
            priceX18: self.priceX18,
            amount: self.amount,
            expiration: self.expiration,
            nonce: self.nonce,
            appendix: self.appendix,
        }
    }
}

impl NadoEip712 for Cancellation {
    type Sol = sol_types::Cancellation;
    fn to_sol(&self) -> Self::Sol {
        sol_types::Cancellation {
            sender: fb(self.sender),
            productIds: self.productIds.clone(),
            digests: self.digests.iter().map(|d| fb(*d)).collect(),
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for CancellationProducts {
    type Sol = sol_types::CancellationProducts;
    fn to_sol(&self) -> Self::Sol {
        sol_types::CancellationProducts {
            sender: fb(self.sender),
            productIds: self.productIds.clone(),
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for LinkSigner {
    type Sol = sol_types::LinkSigner;
    fn to_sol(&self) -> Self::Sol {
        sol_types::LinkSigner {
            sender: fb(self.sender),
            signer: fb(self.signer),
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for LiquidateSubaccount {
    type Sol = sol_types::LiquidateSubaccount;
    fn to_sol(&self) -> Self::Sol {
        sol_types::LiquidateSubaccount {
            sender: fb(self.sender),
            liquidatee: fb(self.liquidatee),
            productId: self.productId,
            isEncodedSpread: self.isEncodedSpread,
            amount: self.amount,
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for WithdrawCollateral {
    type Sol = sol_types::WithdrawCollateral;
    fn to_sol(&self) -> Self::Sol {
        sol_types::WithdrawCollateral {
            sender: fb(self.sender),
            productId: self.productId,
            amount: self.amount,
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for WithdrawCollateralV2 {
    type Sol = sol_types::WithdrawCollateralV2;
    fn to_sol(&self) -> Self::Sol {
        sol_types::WithdrawCollateralV2 {
            sender: fb(self.sender),
            productId: self.productId,
            amount: self.amount,
            nonce: self.nonce,
            sendTo: Address::from(self.sendTo),
            appendix: self.appendix,
        }
    }
}

impl NadoEip712 for MintNlp {
    type Sol = sol_types::MintNlp;
    fn to_sol(&self) -> Self::Sol {
        sol_types::MintNlp {
            sender: fb(self.sender),
            quoteAmount: self.quoteAmount,
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for BurnNlp {
    type Sol = sol_types::BurnNlp;
    fn to_sol(&self) -> Self::Sol {
        sol_types::BurnNlp {
            sender: fb(self.sender),
            nlpAmount: self.nlpAmount,
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for TransferQuote {
    type Sol = sol_types::TransferQuote;
    fn to_sol(&self) -> Self::Sol {
        sol_types::TransferQuote {
            sender: fb(self.sender),
            recipient: fb(self.recipient),
            amount: self.amount,
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for ListTriggerOrders {
    type Sol = sol_types::ListTriggerOrders;
    fn to_sol(&self) -> Self::Sol {
        sol_types::ListTriggerOrders {
            sender: fb(self.sender),
            recvTime: self.recvTime,
        }
    }
}

impl NadoEip712 for DependencyUpdate {
    type Sol = sol_types::DependencyUpdate;
    fn to_sol(&self) -> Self::Sol {
        sol_types::DependencyUpdate {
            sender: fb(self.sender),
            oldDigest: fb(self.oldDigest),
            newDigest: fb(self.newDigest),
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for StreamAuthentication {
    type Sol = sol_types::StreamAuthentication;
    fn to_sol(&self) -> Self::Sol {
        sol_types::StreamAuthentication {
            sender: fb(self.sender),
            expiration: self.expiration,
        }
    }
}

impl NadoEip712 for NadoAuthentication {
    type Sol = sol_types::NadoAuthentication;
    fn to_sol(&self) -> Self::Sol {
        sol_types::NadoAuthentication {
            method: self.method.clone(),
            sender: fb(self.sender),
            payloadHash: fb(self.payloadHash),
            nonce: self.nonce,
        }
    }
}

impl NadoEip712 for LeaderboardAuthentication {
    type Sol = sol_types::LeaderboardAuthentication;
    fn to_sol(&self) -> Self::Sol {
        sol_types::LeaderboardAuthentication {
            sender: fb(self.sender),
            expiration: self.expiration,
            contestIds: self.contestIds.clone(),
        }
    }
}

impl NadoEip712 for SocialAuthentication {
    type Sol = sol_types::SocialAuthentication;
    fn to_sol(&self) -> Self::Sol {
        sol_types::SocialAuthentication {
            sender: fb(self.sender),
            expiration: self.expiration,
            provider: self.provider.clone(),
        }
    }
}
