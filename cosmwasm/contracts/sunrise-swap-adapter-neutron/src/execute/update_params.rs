use crate::msgs::UpdateParamsMsg;
use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use neutron_sdk::{
    bindings::{msg::NeutronMsg, query::NeutronQuery},
    NeutronError, NeutronResult,
};

// #[cfg(not(feature = "library"))]
pub fn execute_update_params(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    info: MessageInfo,
    msg: UpdateParamsMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    let mut response = Response::new();
    let mut params: Params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(NeutronError::Std(StdError::generic_err("Unauthorized")));
    }

    if let Some(authority) = msg.authority {
        params.authority = deps.api.addr_validate(&authority)?;
    }

    PARAMS.save(deps.storage, &params)?;
    response = response
        .add_attribute("action", "update_params")
        .add_attribute("authority", params.authority.to_string());

    Ok(response)
}
