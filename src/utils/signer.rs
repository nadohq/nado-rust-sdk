use core::fmt::Debug;
use std::str::FromStr;

use alloy::signers::local::PrivateKeySigner;
use alloy::signers::{Signer, SignerSync};
use alloy_primitives::Signature;
use alloy_primitives::{Address, B256, U256};
use eyre::Result;

use crate::eip712_alloy::NadoEip712;
use crate::tx::{domain, get_eip712_digest, EIP712Domain};

use crate::core::base::NadoBase;

pub struct NadoSigner<'a, V: NadoBase> {
    nado: &'a V,
}

impl<'a, V: NadoBase> NadoSigner<'a, V> {
    pub fn new(nado: &'a V) -> Self {
        Self { nado }
    }

    pub fn endpoint_digest<T: NadoEip712 + Send + Sync + Debug>(&self, tx: &T) -> Result<[u8; 32]> {
        let domain = self.endpoint_domain()?;
        Ok(get_eip712_digest(tx, &domain))
    }

    pub fn endpoint_signature<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        endpoint_tx: &T,
    ) -> Result<Vec<u8>> {
        let signature = if self.nado.is_rest_client() {
            self.endpoint_signature_base(endpoint_tx)?
        } else {
            self.endpoint_signature_concat(endpoint_tx)?
        };
        Ok(signature)
    }

    pub fn endpoint_signature_concat<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        endpoint_tx: &T,
    ) -> Result<Vec<u8>> {
        let mut ret = self.endpoint_signature_base(endpoint_tx)?;
        ret.extend(self.nado.address()?);
        Ok(ret)
    }

    fn endpoint_signature_base<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        endpoint_tx: &T,
    ) -> Result<Vec<u8>> {
        let domain = self.endpoint_domain()?;
        let signature = self.sign_with_domain(endpoint_tx, domain)?;
        Ok(signature.as_bytes().to_vec())
    }

    pub fn order_signature<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        product_id: u32,
        order_tx: &T,
    ) -> Result<Vec<u8>> {
        let signature = if self.nado.is_rest_client() {
            self.order_signature_base(product_id, order_tx)?
        } else {
            self.order_signature_concat(product_id, order_tx)?
        };
        Ok(signature)
    }

    pub fn order_signature_concat<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        product_id: u32,
        order_tx: &T,
    ) -> Result<Vec<u8>> {
        let mut ret = self.order_signature_base(product_id, order_tx)?;
        ret.extend(self.nado.address()?);
        Ok(ret)
    }

    fn order_signature_base<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        product_id: u32,
        order_tx: &T,
    ) -> Result<Vec<u8>> {
        let domain = self.order_domain(product_id)?;
        let signature = self.sign_with_domain(order_tx, domain)?;
        Ok(signature.as_bytes().to_vec())
    }

    fn sign_with_domain<T: NadoEip712 + Send + Sync + Debug>(
        &self,
        payload: &T,
        domain: EIP712Domain,
    ) -> Result<Signature> {
        let encoded = get_eip712_digest(payload, &domain);
        Ok(self.nado.wallet()?.sign_hash_sync(&B256::from(encoded))?)
    }

    fn endpoint_domain(&self) -> Result<EIP712Domain> {
        self.domain(self.nado.endpoint_addr())
    }

    pub fn order_domain(&self, product_id: u32) -> Result<EIP712Domain> {
        self.domain(Address::left_padding_from(
            &(product_id as u64).to_be_bytes(),
        ))
    }

    fn domain(&self, verifying_contract: Address) -> Result<EIP712Domain> {
        Ok(domain(self.nado.chain_id()?, verifying_contract))
    }
}

pub fn wallet_with_chain_id(private_key: &str, chain_id: U256) -> Result<PrivateKeySigner> {
    let wallet = PrivateKeySigner::from_str(private_key)?;
    Ok(wallet.with_chain_id(Some(chain_id.to::<u64>())))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wallet_with_chain_id_accepts_prefixed_and_unprefixed_keys() {
        let bare = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let prefixed = format!("0x{bare}");
        let chain_id = U256::from(31_337u64);
        let expected_address =
            Address::from_str("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap();

        let bare_wallet = wallet_with_chain_id(bare, chain_id).unwrap();
        let prefixed_wallet = wallet_with_chain_id(&prefixed, chain_id).unwrap();

        assert_eq!(bare_wallet.address(), expected_address);
        assert_eq!(bare_wallet.address(), prefixed_wallet.address());
        assert_eq!(bare_wallet.chain_id(), Some(31_337));
        assert_eq!(prefixed_wallet.chain_id(), Some(31_337));
    }
}
