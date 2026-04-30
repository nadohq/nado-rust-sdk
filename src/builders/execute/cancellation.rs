use eyre::Result;

use crate::eip712_structs;
use crate::engine::{CancelOrdersResponse, Execute};
use crate::utils::client_error::none_error;
use crate::utils::response::match_cancel_orders_response;

use crate::core::execute::NadoExecute;
use crate::utils::nonce::order_nonce;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    CancellationBuilder,
    NadoExecute,
    product_ids: Vec<u32>,
    linked_sender: [u8; 32],
    digests: Vec<[u8; 32]>,
    required_unfilled_amount: i128,
    nonce: u64,
    recv_time: u64;

    build_and_call!(self, execute_trigger, cancel_trigger_orders => ());

    pub async fn execute(&self) -> Result<Option<CancelOrdersResponse>> {
        let tx = self.build()?;
        let signature = self.nado.endpoint_signature(&tx)?;
        let execute = Execute::CancelOrders {
            tx,
            signature,
            required_unfilled_amount: self.required_unfilled_amount,
            id: None,
        };
        match_cancel_orders_response(self.nado.execute(execute).await?)
    }

    pub fn build(&self) -> Result<eip712_structs::Cancellation> {
        let default_sender = self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let nonce = self.nonce.unwrap_or(order_nonce(self.recv_time));
        fields_to_vars!(self, (product_ids: clone), (digests: clone));

        Ok(eip712_structs::Cancellation {
            sender,
            productIds: product_ids,
            digests,
            nonce,
        })
    }
);
