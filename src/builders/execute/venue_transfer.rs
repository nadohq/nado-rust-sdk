use alloy_primitives::Address;
use eyre::Result;

use crate::eip712_structs::VenueTransfer;
use crate::utils::client_error::none_error;

use crate::core::execute::NadoExecute;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    VenueTransferBuilder,
    NadoExecute,
    from_venue_id: u32,
    to_venue_id: u32,
    recipient: [u8; 32],
    amount: u128,
    nonce: u64;

    build_and_call!(self, execute, venue_transfer => (), async_build);

    pub async fn build(&self) -> Result<VenueTransfer> {
        // Nonces key on the signing wallet (the operator).
        let address = Address::from(self.nado.address()?).into();
        let nonce = match self.nonce {
            Some(nonce) => nonce,
            None => self.nado.next_tx_nonce(address).await?,
        };
        fields_to_vars!(self, from_venue_id, amount);

        Ok(VenueTransfer {
            fromVenueId: from_venue_id,
            toVenueId: self.to_venue_id.unwrap_or(0),
            recipient: self.recipient.unwrap_or([0u8; 32]),
            productId: 0,
            amount,
            nonce,
            appendix: 0,
        })
    }
);
