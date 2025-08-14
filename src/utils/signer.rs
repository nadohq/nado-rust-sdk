use core::fmt::Debug;
use std::str::FromStr;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{H160, U256};
use ethers::types::Signature;
use ethers_core::types::transaction::eip712::{EIP712Domain, Eip712};
use ethers_signers::Signer;
use ethers_signers::Wallet;
use eyre::Result;

use crate::tx::{domain, get_eip712_digest};

use crate::core::base::NadoBase;

pub struct NadoSigner<'a, V: NadoBase> {
    nado: &'a V,
}

impl<'a, V: NadoBase> NadoSigner<'a, V> {
    pub fn new(nado: &'a V) -> Self {
        Self { nado }
    }

    pub fn endpoint_digest<T: Eip712 + Send + Sync + Debug>(&self, tx: &T) -> Result<[u8; 32]> {
        let domain = self.endpoint_domain()?;
        let digest: [u8; 32] = get_eip712_digest(tx, &domain).into();
        Ok(digest)
    }

    pub fn endpoint_signature<T: Eip712 + Send + Sync + Debug>(
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

    pub fn endpoint_signature_concat<T: Eip712 + Send + Sync + Debug>(
        &self,
        endpoint_tx: &T,
    ) -> Result<Vec<u8>> {
        let mut ret = self.endpoint_signature_base(endpoint_tx)?;
        ret.extend(self.nado.address()?);
        Ok(ret)
    }

    fn endpoint_signature_base<T: Eip712 + Send + Sync + Debug>(
        &self,
        endpoint_tx: &T,
    ) -> Result<Vec<u8>> {
        let domain = self.endpoint_domain()?;
        let signature = self.sign_with_domain(endpoint_tx, domain)?;
        Ok(signature.to_vec())
    }

    pub fn order_signature<T: Eip712 + Send + Sync + Debug>(
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

    pub fn order_signature_concat<T: Eip712 + Send + Sync + Debug>(
        &self,
        product_id: u32,
        order_tx: &T,
    ) -> Result<Vec<u8>> {
        let mut ret = self.order_signature_base(product_id, order_tx)?;
        ret.extend(self.nado.address()?);
        Ok(ret)
    }

    fn order_signature_base<T: Eip712 + Send + Sync + Debug>(
        &self,
        product_id: u32,
        order_tx: &T,
    ) -> Result<Vec<u8>> {
        let domain = self.order_domain(product_id)?;
        let signature = self.sign_with_domain(order_tx, domain)?;
        Ok(signature.to_vec())
    }

    fn sign_with_domain<T: Eip712 + Send + Sync + Debug>(
        &self,
        payload: &T,
        domain: EIP712Domain,
    ) -> Result<Signature> {
        let encoded = get_eip712_digest(payload, &domain);
        Ok(self.nado.wallet()?.sign_hash(encoded)?)
    }

    fn endpoint_domain(&self) -> Result<EIP712Domain> {
        self.domain(self.nado.endpoint_addr())
    }

    pub fn order_domain(&self, product_id: u32) -> Result<EIP712Domain> {
        self.domain(H160::from_low_u64_be(product_id as u64))
    }

    fn domain(&self, verifying_contract: H160) -> Result<EIP712Domain> {
        Ok(domain(self.nado.chain_id()?, verifying_contract))
    }
}

pub fn wallet_with_chain_id(private_key: &str, chain_id: U256) -> Result<Wallet<SigningKey>> {
    let wallet = Wallet::from_str(private_key)?;
    Ok(wallet.with_chain_id(chain_id.as_u64()))
}
