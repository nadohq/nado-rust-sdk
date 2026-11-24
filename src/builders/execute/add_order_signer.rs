use crate::core::execute::NadoExecute;
use crate::{build_and_call, fields_to_vars, nado_builder};
use alloy_primitives::Address;
use eyre::Result;

use crate::eip712_structs::AddOrderSigner;
use crate::utils::client_error::none_error;

nado_builder!(
    AddOrderSignerBuilder,
    NadoExecute,
    signer: [u8; 32],
    nonce: u64,
    linked_sender: [u8; 32];

    build_and_call!(self, execute, add_order_signer => (), async_build);

    pub async fn build(&self) -> Result<AddOrderSigner> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let address = Address::from_slice(&sender[0..20]).into();
        let nonce = self
            .nonce
            .unwrap_or(self.nado.next_tx_nonce(address).await?);
        fields_to_vars!(self, signer);

        Ok(AddOrderSigner {
            sender,
            signer,
            nonce,
        })
    }
);
