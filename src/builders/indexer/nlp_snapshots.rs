use crate::core::indexer::NadoIndexer;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, nado_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::{Interval, NlpSnapshotsResponse};
use crate::serialize_utils::WrappedU32;

nado_builder!(
    NlpSnapshotsBuilder,
    NadoIndexer,
    max_time: u64,
    granularity: u64,
    count: u64,
    idx: u64,
    limit: u32;

    build_and_call!(self, query, get_nlp_snapshots => NlpSnapshotsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        let interval = self.count.zip(self.granularity).map(|(count, granularity)| Interval {
            count,
            granularity,
            max_time: wrapped_option_u64(self.max_time),
        });

        Ok(indexer::Query::NlpSnapshots {
            interval,
            idx: wrapped_option_u64(self.idx),
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.map(WrappedU32),
        })
    }
);
