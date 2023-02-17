use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, schemars::JsonSchema, Debug, Default)]
pub struct OwnerResp {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, schemars::JsonSchema, Debug, Default)]
pub struct GetBalanceResp {
    pub addr: String,
    pub balance: Uint128,
}
