use eyre::Result;

use crate::indexer;
use crate::indexer::NlpInterestPaymentsResponse;
use crate::serialize_utils::WrappedU32;
use crate::utils::client_error::none_error;

use crate::core::indexer::NadoIndexer;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    NlpInterestPaymentsBuilder,
    NadoIndexer,
    max_idx: u64,
    max_time: u64,
    limit: u32;

    build_and_call!(self, query, get_nlp_interest_payments => NlpInterestPaymentsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, limit);
        Ok(indexer::Query::NlpInterestPayments {
            max_idx: wrapped_option_u64(self.max_idx),
            max_time: wrapped_option_u64(self.max_time),
            limit: WrappedU32(limit),
        })
    }
);
