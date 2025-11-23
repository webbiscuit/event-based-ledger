use thiserror::Error;

use crate::domain::MoneyError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("account not found")]
    AccountNotFound,

    #[error("invalid money value: {0}")]
    InvalidMoney(#[from] MoneyError),

    #[error(
        "insufficient funds: required {required_minor} (minor units), available {available_minor}"
    )]
    InsufficientFunds {
        required_minor: i64,
        available_minor: i64,
    },
}