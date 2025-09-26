use crate::core::query::NadoQuery;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

use crate::serialize_utils::WrappedBytes32;
use crate::trigger;
use crate::trigger::ListTwapExecutionsResponse;
use crate::utils::client_error::none_error;

nado_builder!(
    ListTwapExecutionsBuilder,
    NadoQuery,
    digest: [u8; 32];

    build_and_call!(self, query, list_twap_executions => ListTwapExecutionsResponse);

    pub fn build(&self) -> Result<trigger::Query> {
        fields_to_vars!(self, digest);

        Ok(trigger::Query::ListTwapExecutions {
            digest: WrappedBytes32(digest),
        })
    }
);
