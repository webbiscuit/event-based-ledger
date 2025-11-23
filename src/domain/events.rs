use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::types::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LedgerEventPayload {
    // A new account was opened
    AccountOpened { account_id: AccountId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub id: Uuid,
    pub created_at: OffsetDateTime,
    pub payload: LedgerEventPayload
}

impl LedgerEvent {
    pub fn account_opened(account_id: AccountId)  -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            payload: LedgerEventPayload::AccountOpened { account_id }
        }
    }
}
