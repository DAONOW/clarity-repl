use std::collections::BTreeMap;
use crate::clarity::costs::{ExecutionCost, LimitedCostTracker};
use serde_json::Value;

pub mod interpreter;
pub mod session;
pub mod settings;

pub use interpreter::ClarityInterpreter;
pub use session::Session;
pub use settings::SessionSettings;

#[derive(Default)]
pub struct ExecutionResult {
    pub contract: Option<(String, BTreeMap<String, Vec<String>>)>,
    pub result: Option<String>,
    pub events: Vec<Value>,
    pub cost: Option<CostSynthesis>,
}

#[derive(Clone)]
pub struct CostSynthesis {
    pub total: ExecutionCost,
    pub limit: ExecutionCost,
    pub memory: u64,
    pub memory_limit: u64,
}

impl CostSynthesis {

    pub fn from_cost_tracker(cost_tracker: &LimitedCostTracker) -> CostSynthesis {
        CostSynthesis {
            total: cost_tracker.get_total(),
            limit: cost_tracker.get_limit(),
            memory: cost_tracker.memory,
            memory_limit: cost_tracker.memory_limit,
        }
    }
}