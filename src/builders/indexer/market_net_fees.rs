use crate::core::indexer::NadoIndexer;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::MarketNetFeesResponse;
use crate::serialize_utils::{WrappedU32, WrappedU64};
use crate::utils::client_error::none_error;

nado_builder!(
    MarketNetFeesBuilder,
    NadoIndexer,
    product_ids: Vec<u32>,
    start_time: u64,
    end_time: u64;

    build_and_call!(self, query, get_market_net_fees => MarketNetFeesResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, start_time, end_time);
        let product_ids = self
            .product_ids
            .clone()
            .ok_or(none_error("product_ids"))?;

        Ok(indexer::Query::MarketNetFees {
            product_ids: WrappedU32::wrap_vec_u32(&product_ids),
            start_time: WrappedU64(start_time),
            end_time: WrappedU64(end_time),
        })
    }
);
