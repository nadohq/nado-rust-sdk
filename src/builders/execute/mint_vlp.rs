use crate::core::execute::NadoExecute;
use crate::{fields_to_vars, nado_builder};
use ethers::types::H160;
use eyre::Result;

use crate::eip712_structs::MintVlp;
use crate::utils::client_error::none_error;

nado_builder!(
    MintVlpBuilder,
    NadoExecute,
    quote_amount: u128,
    nonce: u64,
    linked_sender: [u8; 32],
    spot_leverage: bool;

    // we do not use the build_and_call macro here because of extra required argument
    pub async fn execute(&self) -> Result<()> {
        self.nado
            .mint_vlp(self.build().await?, self.spot_leverage)
            .await
    }

    pub async fn build(&self) -> Result<MintVlp> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let address = H160::from_slice(&sender[0..20]).0;
        let nonce = self
            .nonce
            .unwrap_or(self.nado.next_tx_nonce(address).await?);
        fields_to_vars!(
            self,
            quote_amount,
        );

        Ok(MintVlp {
            sender,
            quoteAmount: quote_amount,
            nonce,
        })
    }
);
