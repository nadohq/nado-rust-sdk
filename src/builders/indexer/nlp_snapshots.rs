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
        let interval = if let (Some(granularity), Some(count)) = (self.granularity, self.count) {
            Some(Interval {
                count,
                granularity,
                max_time: wrapped_option_u64(self.max_time),
            })
        } else {
            None
        };

        Ok(indexer::Query::NlpSnapshots {
            interval,
            idx: wrapped_option_u64(self.idx),
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.map(WrappedU32),
        })
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nado_client::NadoClient;
    use crate::prelude::ClientMode;

    #[test]
    fn test_nlp_snapshots_builder_interval() {
        let client = NadoClient::new(ClientMode::Test);
        let builder = NlpSnapshotsBuilder::new(&client);

        // When both granularity and count are None, interval is None
        let query = builder.build().unwrap();
        match query {
            indexer::Query::NlpSnapshots { interval, .. } => assert!(interval.is_none()),
            _ => panic!("unexpected query variant"),
        }

        // When only granularity is set, interval is None
        let builder_gran = builder.granularity(3600);
        let query = builder_gran.build().unwrap();
        match query {
            indexer::Query::NlpSnapshots { interval, .. } => assert!(interval.is_none()),
            _ => panic!("unexpected query variant"),
        }

        // When both are set, interval is Some
        let builder_full = builder_gran.count(24);
        let query = builder_full.build().unwrap();
        match query {
            indexer::Query::NlpSnapshots { interval, .. } => {
                let interval = interval.expect("expected Some interval");
                assert_eq!(interval.granularity, 3600);
                assert_eq!(interval.count, 24);
            }
            _ => panic!("unexpected query variant"),
        }
    }
}

