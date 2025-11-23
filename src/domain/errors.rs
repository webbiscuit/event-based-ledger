use thiserror::Error;

use crate::domain::MoneyError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("account not found")]
    AccountNotFound,

    #[error("invalid money value: {0}")]
    InvalidMoney(#[from] MoneyError),
}