use crate::types::Params;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    SunriseSwap(SunriseSwapMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
}

#[cw_serde]
pub struct SunriseSwapMsg {
    pub sunrise_address: String,
    pub channel_id: String,
    pub memo: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Params {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
