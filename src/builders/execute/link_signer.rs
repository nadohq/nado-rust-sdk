use crate::core::execute::NadoExecute;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

use crate::eip712_structs::LinkSigner;
use crate::utils::client_error::none_error;

nado_builder!(
    LinkSignerBuilder,
    NadoExecute,
    signer: [u8; 32],
    nonce: u64;

    build_and_call!(self, execute, link_signer => (), async_build);

    pub async fn build(&self) -> Result<LinkSigner> {
        let sender = self.nado.subaccount()?;
        let address = self.nado.address()?;
        let nonce = self
            .nonce
            .unwrap_or(self.nado.next_tx_nonce(address).await?);
        fields_to_vars!(self, signer);
        Ok(LinkSigner {
            sender,
            signer,
            nonce,
        })
    }
);
