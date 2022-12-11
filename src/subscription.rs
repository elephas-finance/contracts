use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// data for a subscription details
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Subscription {
    /// frequency of subscription renewal
    pub frequency: u64,
    /// subscription rate
    pub rate: u16,
}
