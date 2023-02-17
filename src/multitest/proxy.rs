use cosmwasm_std::{Addr, Coin, StdResult, Uint128};
use cw_multi_test::{App, Executor};

use crate::{
    contract::{ExecMsg, InstantiateMsg, InstantiateMsgData, QueryMsg, SeiContract},
    errors::ContractError,
    responses::{GetBalanceResp, OwnerResp},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeiContractCodeId(u64);

impl SeiContractCodeId {
    pub fn store_code(app: &mut App) -> Self {
        let code_id = app.store_code(Box::new(SeiContract::new()));
        Self(code_id)
    }

    #[track_caller]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: &Addr,
        label: &str,
        admin: Option<String>,
    ) -> StdResult<SeiContractProxy> {
        let msg = InstantiateMsg {
            data: InstantiateMsgData {
                coin_denom: "sei".to_string(),
            },
        };

        app.instantiate_contract(self.0, sender.clone(), &msg, &[], label, admin)
            .map_err(|err| err.downcast().unwrap())
            .map(SeiContractProxy)
    }
}

#[derive(Debug)]
pub struct SeiContractProxy(Addr);

impl SeiContractProxy {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    #[track_caller]
    pub fn get_owner(&self, app: &App) -> StdResult<OwnerResp> {
        let msg = QueryMsg::GetOwner {};

        app.wrap().query_wasm_smart(self.0.clone(), &msg)
    }

    #[track_caller]
    pub fn get_balance(&self, app: &App, addr: String) -> StdResult<GetBalanceResp> {
        let msg = QueryMsg::GetBalance { addr_str: addr };
        app.wrap().query_wasm_smart(self.0.clone(), &msg)
    }

    #[track_caller]
    pub fn transfer(
        &self,
        app: &mut App,
        sender: &Addr,
        recipient_one: String,
        recipient_two: String,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        let msg = ExecMsg::Transfer {
            recipient_one,
            recipient_two,
        };
        app.execute_contract(sender.clone(), self.0.clone(), &msg, funds)
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn withdraw(
        &self,
        app: &mut App,
        sender: &Addr,
        amount: Uint128,
    ) -> Result<(), ContractError> {
        let msg = ExecMsg::Withdraw { amount: (amount) };
        app.execute_contract(sender.clone(), self.0.clone(), &msg, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }
}
