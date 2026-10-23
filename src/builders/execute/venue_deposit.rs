use alloy_primitives::Address;
use eyre::Result;

use crate::eip712_structs::VenueDeposit;
use crate::utils::client_error::none_error;

use crate::core::execute::NadoExecute;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    VenueDepositBuilder,
    NadoExecute,
    sender: [u8; 32],
    venue_id: u32,
    recipient: [u8; 32],
    amount: u128,
    nonce: u64;

    build_and_call!(self, execute, venue_deposit => (), async_build);

    pub async fn build(&self) -> Result<VenueDeposit> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.sender.unwrap_or(default_sender);

        let address = Address::from_slice(&sender[0..20]).into();
        let nonce = match self.nonce {
            Some(nonce) => nonce,
            None => self.nado.next_tx_nonce(address).await?,
        };
        fields_to_vars!(self, venue_id, amount);

        Ok(VenueDeposit {
            sender,
            venueId: venue_id,
            recipient: self.recipient.unwrap_or([0u8; 32]),
            productId: 0,
            amount,
            nonce,
            appendix: 0,
        })
    }
);
