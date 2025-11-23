use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::{Money, types::{AccountId, EventId}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LedgerEventPayload {
    // A new account was opened
    AccountOpened,
    // Add money to account
    Deposit { amount: Money },
    // Remove money from account
    Withdraw { amount: Money },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub id: EventId,
    pub account_id: AccountId,
    pub created_at: OffsetDateTime,
    pub payload: LedgerEventPayload
}

impl LedgerEvent {
    pub fn account_opened(account_id: AccountId)  -> Self {
        Self {
            id: EventId::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            account_id,
            payload: LedgerEventPayload::AccountOpened
        }
    }

    pub fn deposit(account_id: AccountId, amount: Money)  -> Self {
        Self {
            id: EventId::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            account_id,
            payload: LedgerEventPayload::Deposit { amount }
        }
    }

    pub fn withdraw(account_id: AccountId, amount: Money)  -> Self {
        Self {
            id: EventId::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            account_id,
            payload: LedgerEventPayload::Withdraw { amount }
        }
    }
}
