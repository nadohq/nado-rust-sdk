use crate::core::query::NadoQuery;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

use crate::engine;
use crate::engine::MaxNlpMintableResponse;
use crate::utils::client_error::none_error;
use crate::utils::wrapped_option_utils::optional_bool_to_string;

nado_builder!(
    MaxNlpMintableBuilder,
    NadoQuery,
    subaccount: [u8; 32],
    spot_leverage: bool;

    build_and_call!(self, query, get_max_nlp_mintable => MaxNlpMintableResponse);

    pub fn build(&self) -> Result<engine::Query> {
        fields_to_vars!(self, subaccount);
        Ok(engine::Query::MaxNlpMintable {
            sender: subaccount,
            spot_leverage: optional_bool_to_string(self.spot_leverage),
        })
    }
);
