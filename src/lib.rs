pub mod contract;
pub mod errors;
#[cfg(test)]
mod multitest;
pub mod responses;

#[cfg(not(feature = "library"))]
mod entry_points {
    use crate::contract::{ContractExecMsg, ContractQueryMsg, InstantiateMsg, SeiContract};
    use crate::errors::ContractError;

    use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    const CONTRACT: SeiContract = SeiContract::new();

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        msg.dispatch(&CONTRACT, (deps, env, info))
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: ContractQueryMsg) -> Result<Binary, ContractError> {
        msg.dispatch(&CONTRACT, (deps, env))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ContractExecMsg,
    ) -> Result<Response, ContractError> {
        msg.dispatch(&CONTRACT, (deps, env, info))
    }
}

#[cfg(not(feature = "library"))]
pub use crate::entry_points::*;
