use serde::{Deserialize, Serialize};

use alloy_primitives::Address;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub node_url: String,
    pub quote: Address,
    pub clearinghouse: Address,
    pub endpoint: Address,
    pub spot_engine: Address,
    pub perp_engine: Address,
    pub querier: Address,
    pub offchain_exchange: Address,
    pub verifier: Address,
    pub contract_owner: Address,
}
