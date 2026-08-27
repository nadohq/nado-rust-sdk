use crate::core::indexer::NadoIndexer;
use crate::indexer;
use crate::indexer::PortfolioHistoryResponse;
use crate::serialize_utils::{WrappedBytes32, WrappedU64};
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

nado_builder!(
    PortfolioHistoryBuilder,
    NadoIndexer,
    subaccount: [u8; 32],
    start_time: u64,
    end_time: u64;

    build_and_call!(self, query, get_portfolio_history => PortfolioHistoryResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, subaccount, start_time, end_time);
        Ok(indexer::Query::PortfolioHistory {
            subaccount: WrappedBytes32(subaccount),
            start_time: WrappedU64(start_time),
            end_time: WrappedU64(end_time),
        })
    }
);
