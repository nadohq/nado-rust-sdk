use crate::core::query::NadoQuery;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

use crate::engine;
use crate::engine::NlpLockedBalancesResponse;
use crate::utils::client_error::none_error;

nado_builder!(
    NlpLockedBalancesBuilder,
    NadoQuery,
    subaccount: [u8; 32];

    build_and_call!(self, query, get_nlp_locked_balances => NlpLockedBalancesResponse);

    pub fn build(&self) -> Result<engine::Query> {
        fields_to_vars!(self, subaccount);
        Ok(engine::Query::NlpLockedBalances {
            subaccount,
        })
    }
);
