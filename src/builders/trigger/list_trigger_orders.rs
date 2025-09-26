use crate::core::query::NadoQuery;
use crate::utils::nonce::default_recv_time;
use crate::{build_and_call, nado_builder};
use ethers::types::Bytes;
use eyre::Result;

use crate::eip712_structs;
use crate::trigger;
use crate::trigger::ListTriggerOrdersResponse;
use crate::utils::wrapped_option_utils::{wrapped_option_bytes32, wrapped_option_vec_bytes32};

nado_builder!(
    ListTriggerOrdersBuilder ,
    NadoQuery,
    recv_time: u64,
    product_ids: Vec<u32>,
    trigger_types: Vec<trigger::TriggerType>,
    status_types: Vec<trigger::TriggerOrderStatusType>,
    linked_sender: [u8; 32],
    pending: bool,
    digests: Vec<[u8; 32]>,
    max_digest: [u8; 32],
    max_update_time: u64,
    limit: u32,
    reduce_only: bool;

    build_and_call!(self, query, list_trigger_orders => ListTriggerOrdersResponse);

    pub fn build(&self) -> Result<trigger::Query> {
        let tx = self.list_trigger_orders()?;
        let signature = Bytes::from(self.nado.endpoint_signature(&tx)?);
        let mut status_types = self.status_types.clone();
        if self.pending.unwrap_or(false) {
            assert!(status_types.is_none(), "should not specify status_types when pending is true");
            status_types = Some(trigger::TriggerOrderStatusType::pending());
        }

        Ok(trigger::Query::ListTriggerOrders {
            tx,
            signature,
            product_ids: self.product_ids.clone(),
            trigger_types: self.trigger_types.clone(),
            status_types,
            digests: wrapped_option_vec_bytes32(self.digests.clone()),
            max_update_time: self.max_update_time,
            max_digest: wrapped_option_bytes32(self.max_digest),
            limit: self.limit,
            reduce_only: self.reduce_only,
        })
    }


    fn list_trigger_orders(&self) -> Result<eip712_structs::ListTriggerOrders> {
        let default_sender =  self.nado.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let recv_time = self.recv_time.unwrap_or(default_recv_time());
        Ok(eip712_structs::ListTriggerOrders {
            sender,
            recvTime: recv_time,
        })
    }
);
