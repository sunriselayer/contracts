use crate::error::ContractError;
use crate::msgs::SendToSunriseMsg;
use cosmwasm_std::{CosmosMsg, DepsMut, Env, IbcTimeout, MessageInfo, Response};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_send_to_sunrise(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SendToSunriseMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let coin = one_coin(&info)?;

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    response = response.add_message(CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
        channel_id: msg.channel_id,
        to_address: msg.sunrise_address,
        amount: coin,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(604_800u64)),
        memo: Some(msg.memo),
    }));

    // response = response.add_message(CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
    //     to_address: msg.sunrise_address,
    //     amount: vec![coin],
    // }));

    Ok(response)
}
