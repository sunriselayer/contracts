use crate::msgs::SunriseSwapMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use cw_utils::one_coin;
use neutron_sdk::{
    bindings::{
        msg::{IbcFee, NeutronMsg},
        query::NeutronQuery,
    },
    query::min_ibc_fee::query_min_ibc_fee,
    sudo::msg::RequestPacketTimeoutHeight,
    NeutronError, NeutronResult,
};

const FEE_DENOM: &str = "untrn";

#[cfg(not(feature = "library"))]
pub fn execute_sunrise_swap(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: SunriseSwapMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut response = Response::new();

    let coin = match one_coin(&info) {
        Ok(coin) => coin,
        Err(payment_error) => {
            return Err(NeutronError::Std(StdError::generic_err(
                payment_error.to_string(),
            )))
        }
    };

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let msg = NeutronMsg::IbcTransfer {
        source_port: "transfer".to_string(),
        source_channel: msg.channel_id,
        token: coin,
        sender: env.contract.address.to_string(),
        receiver: msg.sunrise_address,
        timeout_height: RequestPacketTimeoutHeight {
            revision_height: Some(0),
            revision_number: Some(0),
        },
        timeout_timestamp: env.block.time.plus_seconds(604_800u64).nanos(),
        memo: msg.memo,
        fee,
    };

    response = response.add_message(msg);

    // response = response.add_message(CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
    //     to_address: msg.sunrise_address,
    //     amount: vec![coin],
    // }));

    Ok(response)
}

fn min_ntrn_ibc_fee(fee: IbcFee) -> IbcFee {
    IbcFee {
        recv_fee: fee.recv_fee,
        ack_fee: fee
            .ack_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
        timeout_fee: fee
            .timeout_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
    }
}
