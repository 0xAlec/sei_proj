use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("{sender} has insufficient balance for the requested withdrawal")]
    InsufficientBalance { sender: Addr },
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("Cannot withdraw 0 tokens")]
    InvalidZeroAmount,
}
