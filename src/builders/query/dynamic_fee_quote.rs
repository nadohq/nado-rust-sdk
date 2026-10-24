use crate::core::query::NadoQuery;
use crate::{build_and_call, fields_to_vars, nado_builder};
use eyre::Result;

use crate::engine;
use crate::engine::{DynamicFeeKind, DynamicFeeQuoteResponse};
use crate::utils::client_error::none_error;

nado_builder!(
    DynamicFeeQuoteBuilder,
    NadoQuery,
    kind: DynamicFeeKind,
    subaccount: [u8; 32],
    product_id: u32,
    recipient: [u8; 32];

    build_and_call!(self, query, get_dynamic_fee_quote => DynamicFeeQuoteResponse);

    pub fn build(&self) -> Result<engine::Query> {
        fields_to_vars!(self, kind, subaccount);
        Ok(engine::Query::DynamicFeeQuote {
            kind,
            sender: subaccount,
            product_id: self.product_id,
            recipient: self.recipient,
        })
    }
);
