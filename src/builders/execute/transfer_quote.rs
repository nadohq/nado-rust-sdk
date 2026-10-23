use alloy_primitives::Address;
use eyre::Result;

use crate::eip712_structs::TransferQuote;
use crate::utils::client_error::none_error;

use crate::core::execute::NadoExecute;
use crate::{fields_to_vars, nado_builder};

nado_builder!(
    TransferQuoteBuilder,
    NadoExecute,
    recipient: [u8; 32],
    linked_sender: [u8; 32],
    amount: u128,
    nonce: u64,
    max_fee_x18: i128;

    pub async fn execute(&self) -> Result<()> {
        let tx = self.build().await?;
        match self.max_fee_x18 {
            Some(max_fee_x18) => self.nado.transfer_quote_v2(tx, max_fee_x18).await,
            None => self.nado.transfer_quote(tx).await,
        }
    }

    pub async fn build(&self) -> Result<TransferQuote> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);

        let address = Address::from_slice(&sender[0..20]).into();
        let nonce = self
            .nonce
            .unwrap_or(self.nado.next_tx_nonce(address).await?);
        fields_to_vars!(self, recipient, amount);

        Ok(TransferQuote {
            sender,
            recipient,
            amount,
            nonce
        })
    }
);
