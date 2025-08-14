use eyre::Result;

use crate::core::indexer::NadoIndexer;
use crate::indexer;
use crate::indexer::{LeaderboardResponse, LeaderboardType};
use crate::serialize_utils::{WrappedU32, WrappedU64};
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, nado_builder};

nado_builder!(
    LeaderboardBuilder,
    NadoIndexer,
    contest_id: u32,
    rank_type: LeaderboardType,
    start: u64,
    limit: u64;

    build_and_call!(self, query, get_leaderboard => LeaderboardResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, contest_id, (rank_type: clone));
        Ok(indexer::Query::Leaderboard {
            contest_id: WrappedU32(contest_id),
            rank_type,
            start: self.start.map(|s| WrappedU64(s)),
            limit: self.limit.map(|l| WrappedU64(l)),
        })
    }

);
