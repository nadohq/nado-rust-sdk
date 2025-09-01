use crate::core::query::NadoQuery;
use crate::{build_and_call, nado_builder};
use eyre::Result;

use crate::engine;
use crate::engine::NlpPoolInfoResponse;

nado_builder!(
    NlpPoolInfoBuilder,
    NadoQuery,
    ;

    build_and_call!(self, query, get_nlp_pool_info => NlpPoolInfoResponse);

    pub fn build(&self) -> Result<engine::Query> {
        Ok(engine::Query::NlpPoolInfo {
        })
    }
);
