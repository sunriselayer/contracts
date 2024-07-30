use crate::msgs::SendToEvmMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;
use neutron_sdk::{
    bindings::{
        msg::{IbcFee, NeutronMsg},
        query::NeutronQuery,
    },
    query::min_ibc_fee::query_min_ibc_fee,
    sudo::msg::RequestPacketTimeoutHeight,
    NeutronResult,
};
use serde_json_wasm::to_string;

// IBC Channels to axelarnet
// https://docs.axelar.dev/resources/contract-addresses/testnet#ibc-channels
const IBC_CHANNEL: &str = "channel-8";

// Axelar GMP Account
// https://docs.axelar.dev/dev/cosmos-gmp#messages-from-native-cosmos
const AXELAR_GATEWAY: &str = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5s";

// Neutron Fee Denom
const FEE_DENOM: &str = "untrn";

#[cfg(not(feature = "library"))]
pub fn execute_send_to_evm(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: SendToEvmMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut response = Response::new();

    let coin = one_coin(&info).unwrap();

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let msg = NeutronMsg::IbcTransfer {
        source_port: "transfer".to_string(),
        source_channel: IBC_CHANNEL.to_string(),
        token: coin,
        sender: env.contract.address.to_string(),
        receiver: AXELAR_GATEWAY.to_string(),
        timeout_height: RequestPacketTimeoutHeight {
            revision_height: Some(0),
            revision_number: Some(0),
        },
        timeout_timestamp: env.block.time.plus_seconds(604_800u64).nanos(),
        memo: to_string(&msg).unwrap(),
        fee,
    };

    response = response.add_message(msg);

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
