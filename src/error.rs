use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

    #[error("Insufficient funds provided to play")]
    InsufficientFunds,

    #[error("Proxy address is not valid")]
    InvalidProxyAddress,
    
    #[error("Invalid nois receive address")]
    UnauthorizedReceive,
    
    #[error("Invalid randomness received")]
    InvalidRandomness,
    
    #[error("Job ID already present")]
    JobIdAlreadyPresent
}