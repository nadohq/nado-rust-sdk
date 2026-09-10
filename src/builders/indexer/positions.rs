use eyre::Result;

use crate::core::indexer::NadoIndexer;
use crate::indexer;
use crate::indexer::PositionsResponse;
use crate::serialize_utils::{WrappedBytes32, WrappedU32};
use crate::utils::client_error::none_error;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    PositionsBuilder,
    NadoIndexer,
    subaccount: [u8; 32],
    product_id: u32,
    isolated: bool,
    open: bool,
    idx: u64,
    limit: u32;

    build_and_call!(self, query, get_positions => PositionsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, subaccount);
        Ok(indexer::Query::Positions {
            subaccount: WrappedBytes32(subaccount),
            product_id: self.product_id.map(WrappedU32),
            isolated: self.isolated,
            idx: wrapped_option_u64(self.idx),
            limit: self.limit.map(WrappedU32),
            open: self.open,
        })
    }
);
