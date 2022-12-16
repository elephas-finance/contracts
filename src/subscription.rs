use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// data for a subscription details
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// #[serde(rename_all = "snake_case")]
pub struct Subscription {
    /// frequency of subscription renewal
    pub frequency: u64,
    /// subscription rate
    pub rate: Uint128,
}
