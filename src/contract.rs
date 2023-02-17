use crate::errors::ContractError;
use crate::responses::{GetBalanceResp, OwnerResp};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coins, ensure, Addr, BankMsg, Deps, DepsMut, Empty, Env, MessageInfo, Order, Response,
    StdError, StdResult, Uint128,
};
use cw_storage_plus::{Item, Map};
use sylvia::contract;

pub struct SeiContract<'a> {
    pub(crate) owner: Map<'a, &'a Addr, Empty>,
    pub(crate) balances: Map<'static, &'a Addr, Uint128>,
    pub(crate) coin_denom: Item<'a, String>,
}

#[cw_serde]
pub struct InstantiateMsgData {
    pub coin_denom: String,
}

#[contract]
impl SeiContract<'_> {
    pub const fn new() -> Self {
        Self {
            owner: Map::new("owner"),
            coin_denom: Item::new("coin_denom"),
            balances: Map::new("balances"),
        }
    }
    #[msg(instantiate)]
    pub fn instantiate(
        &self,
        _ctx: (DepsMut, Env, MessageInfo),
        data: InstantiateMsgData,
    ) -> StdResult<Response> {
        let (deps, _, info) = _ctx;
        let InstantiateMsgData { coin_denom } = data;
        // Set deployer as contract admin
        let owner_addr = deps.api.addr_validate(&info.sender.to_string())?;
        self.owner.save(deps.storage, &owner_addr, &Empty {})?;
        self.coin_denom.save(deps.storage, &coin_denom)?;
        Ok(Response::new())
    }

    #[msg(exec)]
    pub fn transfer(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        recipient_one: String,
        recipient_two: String,
    ) -> Result<Response, ContractError> {
        let (deps, env, info) = ctx;

        let contract_addr = &env.contract.address;
        let addr_one = deps.api.addr_validate(&recipient_one)?;
        let addr_two = deps.api.addr_validate(&recipient_two)?;

        // Validate funds sent
        let denom = self.coin_denom.load(deps.storage)?;
        let amount = cw_utils::must_pay(&info, &denom).map_err(|err| StdError::GenericErr {
            msg: err.to_string(),
        })?;

        let divided_amount = amount.checked_div(Uint128::new(2)).unwrap();

        // Since we use integer division, let remainders of sends with odd values be collected as a fee by the contract owner
        let fees = amount.u128() % 2;
        let owner_addr = deps.api.addr_validate(
            &self
                .get_owner((deps.as_ref(), env.clone()))
                .unwrap_or_default()
                .owner,
        )?;
        self.balances.update(deps.storage, &owner_addr, |balance| {
            Ok::<_, StdError>(
                balance
                    .unwrap_or_default()
                    .checked_add(Uint128::from(fees))?,
            )
        })?;

        // Record balances for recipients
        self.balances.update(deps.storage, &addr_one, |balance| {
            Ok::<_, StdError>(balance.unwrap_or_default().checked_add(divided_amount)?)
        })?;
        self.balances.update(deps.storage, &addr_two, |balance| {
            Ok::<_, StdError>(balance.unwrap_or_default().checked_add(divided_amount)?)
        })?;

        let message = BankMsg::Send {
            to_address: (contract_addr.to_string()),
            amount: coins(divided_amount.u128(), &denom),
        };

        let resp = Response::new()
            .add_message(message)
            .add_attribute("action", "transfer")
            .add_attribute("amount", amount.to_string())
            .add_attribute("per basis", divided_amount.to_string());
        Ok(resp)
    }

    #[msg(exec)]
    pub fn withdraw(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let (deps, env, info) = ctx;

        ensure!(
            amount != Uint128::zero(),
            ContractError::InvalidZeroAmount {}
        );

        // Check for sufficient balance to withdraw
        let balance = self
            .get_balance((deps.as_ref(), env), info.sender.to_string())
            .unwrap_or_default()
            .balance;
        ensure!(
            balance >= amount,
            ContractError::InsufficientBalance {
                sender: (info.sender)
            }
        );

        // Decrement user balance
        self.balances
            .update(deps.storage, &info.sender, |balance| {
                Ok::<_, StdError>(balance.unwrap_or_default().checked_sub(amount)?)
            })?;

        // Transfer tokens
        let denom = self.coin_denom.load(deps.storage)?;
        let message = BankMsg::Send {
            to_address: (info.sender.to_string()),
            amount: coins(amount.u128(), &denom),
        };

        let resp = Response::new()
            .add_message(message)
            .add_attribute("action", "withdraw")
            .add_attribute("amount", amount.to_string())
            .add_attribute("to", info.sender);
        Ok(resp)
    }

    #[msg(query)]
    pub fn get_owner(&self, ctx: (Deps, Env)) -> StdResult<OwnerResp> {
        let (deps, _) = ctx;
        let owner: Result<_, _> = self
            .owner
            .keys(deps.storage, None, None, Order::Ascending)
            .map(|addr| addr.map(String::from))
            .collect();
        Ok(OwnerResp { owner: owner? })
    }

    #[msg(query)]
    pub fn get_balance(&self, ctx: (Deps, Env), addr_str: String) -> StdResult<GetBalanceResp> {
        let (deps, _) = ctx;
        let addr = deps.api.addr_validate(&addr_str)?;
        let balance = self
            .balances
            .may_load(deps.storage, &addr)?
            .unwrap_or_default();
        Ok(GetBalanceResp {
            addr: addr.to_string(),
            balance,
        })
    }
}
