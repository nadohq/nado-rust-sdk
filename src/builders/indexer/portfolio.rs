use crate::core::indexer::NadoIndexer;
use crate::indexer;
use crate::indexer::PortfolioResponse;
use crate::serialize_utils::WrappedBytes32;
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

nado_builder!(
    PortfolioBuilder,
    NadoIndexer,
    subaccount: [u8; 32];

    build_and_call!(self, query, get_portfolio => PortfolioResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, subaccount);
        Ok(indexer::Query::Portfolio {
            subaccount: WrappedBytes32(subaccount),
        })
    }
);
