use crate::msgs::SendToEvmMsg;
use crate::types::{Fee, GeneralMessage};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;
use ethabi::ethereum_types::H160;
use ethabi::{encode, Address, Token};
use neutron_sdk::{
    bindings::{
        msg::{IbcFee, NeutronMsg},
        query::NeutronQuery,
    },
    query::min_ibc_fee::query_min_ibc_fee,
    sudo::msg::RequestPacketTimeoutHeight,
    NeutronError, NeutronResult,
};
use serde_json_wasm::to_string;

// IBC Channels to axelarnet
// https://docs.axelar.dev/resources/contract-addresses/testnet#ibc-channels
const IBC_CHANNEL: &str = "channel-8";

// Axelar GMP Account
// https://docs.axelar.dev/dev/cosmos-gmp#messages-from-native-cosmos
// https://github.com/axelarnetwork/axelar-docs/issues/435
const AXELAR_GATEWAY: &str = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5";
// Axelar relayer address to use as the fee recipient
// https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/native-integration/README.md#relayer-service-for-cosmos---evm
const AXELAR_FEE_RECIPIENT: &str = "axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd";

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

    // info.funds used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin = one_coin(&info).unwrap();

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview

    // build payload
    let addr = match msg.recipient.parse::<H160>() {
        Ok(address) => Ok(Token::Address(Address::from(address))),
        Err(error) => Err(NeutronError::SerdeJSONWasm(error.to_string())),
    }?;
    let payload = encode(&vec![addr]);

    let fee: Option<Fee> = Some(Fee {
        amount: msg.fee,
        recipient: AXELAR_FEE_RECIPIENT.to_string(),
    });

    // The type field denotes the message type
    // 1: pure message
    // 2: message with token
    let gmp_message = GeneralMessage {
        destination_chain: msg.destination_chain,
        destination_address: msg.destination_contract,
        payload,
        type_: 2,
        fee,
        refund_address: msg.refund_recipient,
    };

    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let ibc_msg = NeutronMsg::IbcTransfer {
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
        memo: to_string(&gmp_message).unwrap(),
        fee,
    };

    response = response.add_message(ibc_msg);

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
