use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{COUNTER, NOIS_PROXY, RANDOM_INT_OUTCOME};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| ContractError::InvalidProxyAddress)?;
    NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;
    COUNTER.save(deps.storage, &0)?;
    
    Ok(Response::default())

}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        RandomIntOutcome { job_id } => to_json_binary(&query::random_int_outcome(deps, job_id)?),
    }
}

mod query {
    use cosmwasm_std::StdError;
    use crate::state::RANDOM_INT_OUTCOME;
    use super::*;

    pub fn random_int_outcome(deps: Deps, job_id: String) -> Result<u8, StdError> {
        RANDOM_INT_OUTCOME.load(deps.storage, &job_id)
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        GetRandomInt {} => exec::get_random_int(deps, info),
        NoisReceive { callback } => exec::nois_receive(deps, info, callback),
    }
}

mod exec {
    use cosmwasm_std::{ensure_eq, WasmMsg};
    use nois::{int_in_range, NoisCallback, ProxyExecuteMsg};

    use super::*;

    pub fn get_random_int(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let nois_proxy = NOIS_PROXY.load(deps.storage)?;

        let job_id = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &job_id)?;

        let response = Response::new().add_message(WasmMsg::Execute {
            contract_addr: nois_proxy.into(),
            msg: to_json_binary(&ProxyExecuteMsg::GetNextRandomness { job_id: job_id.to_string() })?,
            funds: info.funds,
        });

        Ok(response)
    }

    pub fn nois_receive(
        deps: DepsMut,
        info: MessageInfo,
        callback: NoisCallback,
    ) -> Result<Response, ContractError> {
        let proxy = NOIS_PROXY.load(deps.storage)?;
        ensure_eq!(info.sender, proxy, ContractError::UnauthorizedReceive);

        let NoisCallback {
            job_id, randomness, ..
        } = callback;

        let randomness: [u8; 32] = randomness.to_array().map_err(|_| ContractError::InvalidRandomness)?;
        let random_int = int_in_range(randomness, 1, 10);

        RANDOM_INT_OUTCOME.save(deps.storage, &job_id, &random_int)?;
        Ok(Response::default())
    }
}
