use crate::eip712_structs::{Cancellation, ListTriggerOrders};
use crate::eip712_structs::{CancellationProducts, Order};
use crate::engine::{ExecuteResponse, PlaceOrder, Status};
use crate::serialize_utils::{deserialize_i128, serialize_i128, WrappedBytes32, WrappedI128};
use ethers::types::{Bytes, H256};
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelReason {
    UserRequested,
    LinkedSignerChanged,
    Expired,
    AccountHealth,
    IsolatedSubaccountClosed,
    DependentOrderCancelled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecuteStatus {
    pub execute_response: ExecuteResponse,
    pub place_time: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderStatus {
    Cancelled(CancelReason),
    Triggered(ExecuteResponse),
    InternalError(String),
    Triggering,
    WaitingPrice,
    WaitingDependency,
    // TWAP-specific statuses
    TwapExecuting {
        current_execution: u32,
        total_executions: u32,
    },
    TwapCompleted {
        executed: u32,
        total: u32,
    },
}

impl TriggerOrderStatus {
    pub fn byte(&self) -> i32 {
        (match self {
            Self::Cancelled(_) => TriggerOrderStatusType::Cancelled,
            Self::Triggered(_) => TriggerOrderStatusType::Triggered,
            Self::InternalError(_) => TriggerOrderStatusType::InternalError,
            Self::Triggering => TriggerOrderStatusType::Triggering,
            Self::WaitingPrice => TriggerOrderStatusType::WaitingPrice,
            Self::WaitingDependency => TriggerOrderStatusType::WaitingDependency,
            Self::TwapExecuting { .. } => TriggerOrderStatusType::TwapExecuting,
            Self::TwapCompleted { .. } => TriggerOrderStatusType::TwapCompleted,
        }) as i32
    }

    pub fn byte_from_str(s: &str) -> Result<i32> {
        match s {
            "cancelled" => Ok(0),
            "triggered" => Ok(1),
            "internal_error" => Ok(2),
            "triggering" => Ok(3),
            "waiting_price" => Ok(4),
            "waiting_dependency" => Ok(5),
            "twap_executing" => Ok(6),
            "twap_completed" => Ok(7),
            _ => Err(eyre::eyre!("Invalid status string")),
        }
    }

    pub fn data(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_status_data(data: &[u8]) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TriggerCriteria {
    PriceTrigger {
        price_requirement: PriceRequirement,
        dependency: Option<Dependency>,
    },
    TimeTrigger {
        interval: u64,
        amounts: Option<Vec<WrappedI128>>,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub digest: H256,
    pub on_partial_fill: bool, // trigger on partial fill
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PriceRequirement {
    OraclePriceAbove(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),
    OraclePriceBelow(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),

    LastPriceAbove(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),
    LastPriceBelow(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),

    MidPriceAbove(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),

    MidPriceBelow(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    PriceTrigger = 0,
    TimeTrigger = 1,
    None = 2,
}

impl TriggerType {
    pub fn from_appendix(appendix: u128) -> Self {
        match appendix >> 12 & 3 {
            1 => Self::PriceTrigger,
            2 | 3 => Self::TimeTrigger,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderStatusType {
    Cancelled = 0,
    Triggered = 1,
    InternalError = 2,
    Triggering = 3,
    WaitingPrice = 4,
    WaitingDependency = 5,
    TwapExecuting = 6,
    TwapCompleted = 7,
}

impl TriggerOrderStatusType {
    pub fn pending() -> Vec<Self> {
        vec![
            Self::WaitingPrice,
            Self::WaitingDependency,
            Self::TwapExecuting,
        ]
    }
    pub fn pending_byte() -> Vec<i32> {
        Self::pending().into_iter().map(|x| x as i32).collect()
    }
}

impl TriggerCriteria {
    pub fn byte(&self) -> i32 {
        match self {
            Self::PriceTrigger { .. } => 0,
            Self::TimeTrigger { .. } => 1,
        }
    }

    pub fn price_str(&self) -> Option<String> {
        match self {
            Self::PriceTrigger {
                price_requirement, ..
            } => Some(price_requirement.price_str()),
            _ => None,
        }
    }

    pub fn criteria_price_byte(&self) -> Option<i32> {
        match self {
            Self::PriceTrigger {
                price_requirement, ..
            } => Some(price_requirement.byte()),
            _ => None,
        }
    }

    pub fn dependency(&self) -> Option<[u8; 32]> {
        match self {
            Self::PriceTrigger { dependency, .. } => dependency.clone().map(|x| x.digest.0),
            _ => None,
        }
    }
}

impl PriceRequirement {
    pub fn byte(&self) -> i32 {
        match self {
            Self::OraclePriceAbove(_) => 0,
            Self::OraclePriceBelow(_) => 1,
            Self::LastPriceAbove(_) => 2,
            Self::LastPriceBelow(_) => 3,
            Self::MidPriceAbove(_) => 4,
            Self::MidPriceBelow(_) => 5,
        }
    }

    pub fn price(&self) -> i128 {
        match self {
            Self::OraclePriceAbove(price) => *price,
            Self::OraclePriceBelow(price) => *price,
            Self::LastPriceAbove(price) => *price,
            Self::LastPriceBelow(price) => *price,
            Self::MidPriceAbove(price) => *price,
            Self::MidPriceBelow(price) => *price,
        }
    }

    pub fn price_str(&self) -> String {
        // for postgresql indexing, which doesn't support i128
        format!("{:0>40}", self.price())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceTriggerOrder {
    pub order: Order,
    pub signature: Bytes,
    pub product_id: u32,
    pub spot_leverage: Option<bool>,
    pub borrow_margin: Option<bool>,
    pub trigger: TriggerCriteria,
    pub digest: Option<H256>,
    pub id: Option<u64>,
}

impl PlaceTriggerOrder {
    pub fn to_place_order(&self) -> PlaceOrder {
        PlaceOrder {
            order: self.order.clone(),
            signature: self.signature.to_vec(),
            product_id: self.product_id,
            digest: None,
            id: self.id,
            spot_leverage: self.spot_leverage,
            borrow_margin: self.borrow_margin,
        }
    }

    pub fn order_data(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_order_data(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }

    pub fn expiration(&self) -> u64 {
        self.order.expiration
    }

    pub fn is_bid(&self) -> bool {
        self.order.amount.is_positive()
    }

    pub fn trigger_amount(&self, index: u32) -> Option<i128> {
        if !self.order.is_twap_random() {
            None
        } else {
            match &self.trigger {
                TriggerCriteria::TimeTrigger { amounts, .. } => {
                    Some(amounts.as_ref().unwrap()[index as usize].0)
                }
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceTriggerOrdersItemResponse {
    pub digest: Option<[u8; 32]>,
    pub error: Option<String>,
}

pub type PlaceTriggerOrdersResponse = Vec<PlaceTriggerOrdersItemResponse>;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
// #[ts(export)]·
// #[ts(export_to = "tsBindings/msg/")]
pub enum Execute {
    PlaceOrder(PlaceTriggerOrder),
    PlaceOrders {
        orders: Vec<PlaceTriggerOrder>,
        stop_on_failure: Option<bool>,
    },
    CancelOrders {
        tx: Cancellation,
        signature: Bytes,
    },
    CancelProductOrders {
        tx: CancellationProducts,
        signature: Bytes,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Query {
    ListTriggerOrders {
        tx: ListTriggerOrders,
        signature: Bytes,
        product_ids: Option<Vec<u32>>,
        trigger_types: Option<Vec<TriggerType>>,
        status_types: Option<Vec<TriggerOrderStatusType>>,
        max_update_time: Option<u64>,
        max_digest: Option<WrappedBytes32>,
        digests: Option<Vec<WrappedBytes32>>,
        limit: Option<u32>,
        reduce_only: Option<bool>,
    },
    ListTwapExecutions {
        digest: WrappedBytes32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriggerOrderInfo {
    pub order: PlaceTriggerOrder,
    pub status: TriggerOrderStatus,
    pub placed_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListTriggerOrdersResponse {
    pub orders: Vec<TriggerOrderInfo>,
}

impl ListTriggerOrdersResponse {
    pub fn contains_digest(&self, digest: H256) -> bool {
        self.orders
            .iter()
            .any(|order| order.order.digest == Some(digest))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TwapExecutionStatusData {
    Pending,
    Executed {
        executed_time: i64,
        execute_response: ExecuteResponse,
    },
    Failed(String),
    Cancelled(CancelReason),
}

impl TwapExecutionStatusData {
    pub fn data(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_status_data(data: &[u8]) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListTwapExecutionsResponse {
    pub executions: Vec<TwapExecutionDetail>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwapExecutionDetail {
    pub execution_id: u32,
    pub scheduled_time: i64,
    pub status: TwapExecutionStatusData,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum QueryResponseData {
    ListTriggerOrders(ListTriggerOrdersResponse),
    ListTwapExecutions(ListTwapExecutionsResponse),
    Error(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
#[serde(rename_all = "snake_case")]
pub struct QueryResponse {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<QueryResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
}
