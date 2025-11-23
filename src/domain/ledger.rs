use tracing::info;

use crate::domain::{events::LedgerEvent, types::AccountId};

#[derive(Debug, Default)]
pub struct Ledger {
    events: Vec<LedgerEvent>,
}

impl Ledger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn events(&self) -> &[LedgerEvent] {
        &self.events
    }

    pub fn events_for_account(&self, account_id: AccountId) -> Vec<LedgerEvent> {
        info!("Looking up account {}", account_id);

        self.events
            .iter()
            .filter(|e| e.account_id == account_id)
            .cloned()
            .collect()
    }

    pub fn open_account(&mut self) -> AccountId {
        let account_id = AccountId::new_v4();

        info!("Creating new account {}", account_id);

        let event = LedgerEvent::account_opened(account_id);

        self.events.push(event);

        account_id
    }

}