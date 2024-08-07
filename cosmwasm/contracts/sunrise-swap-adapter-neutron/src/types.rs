use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
}

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Fee {
    pub amount: String,
    pub recipient: String,
    pub refund_recipient: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GeneralMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<Fee>,
}
