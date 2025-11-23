use tracing::info;

use crate::domain::{Money, errors::DomainError, events::LedgerEvent, types::{AccountId, EventId}};

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

    pub fn events_for_account(&self, account_id: AccountId) -> Result<Vec<LedgerEvent>, DomainError> {
        info!("Looking up account {}", account_id);

        if !self.account_exists(account_id) {
            return Err(DomainError::AccountNotFound)
        }

        Ok(self.events
            .iter()
            .filter(|e| e.account_id == account_id)
            .cloned()
            .collect())
    }

    pub fn open_account(&mut self) -> AccountId {
        let account_id = AccountId::new_v4();

        info!("Creating new account {}", account_id);

        let event = LedgerEvent::account_opened(account_id);

        self.events.push(event);

        account_id
    }

    pub fn account_exists(&self, account_id: AccountId) -> bool {
        self.events
            .iter()
            .any(|e| e.account_id == account_id)
    }

    pub fn deposit(&mut self, account_id: AccountId, amount: Money) -> Result<EventId, DomainError> {
        if !self.account_exists(account_id) {
            return Err(DomainError::AccountNotFound)
        }

        info!("Depositing {} to {}", amount, account_id);

        let event = LedgerEvent::deposit(account_id, amount);
        let id = event.id;

        self.events.push(event);

        Ok(id)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Currency, Money, events::LedgerEventPayload};

    #[test]
    fn deposit_into_existing_account_appends_event() {
        let mut ledger = Ledger::new();
        let account_id = ledger.open_account();

        let amount = Money::new_minor(10_00, Currency::Gbp).unwrap(); // £10
        ledger.deposit(account_id, amount).unwrap();

        let events = ledger.events_for_account(account_id).unwrap();
        assert_eq!(events.len(), 2);

        // Last event should be a Deposit
        match &events.last().unwrap().payload {
            LedgerEventPayload::Deposit { amount, .. } => {
                assert_eq!(amount.amount(), 10_00);
            }
            other => panic!("expected Deposit event, got {other:?}"),
        }
    }

    #[test]
    fn deposit_into_unknown_account_fails() {
        let mut ledger = Ledger::new();
        let fake_account = AccountId::new_v4();

        let amount = Money::new_minor(5_00, Currency::Gbp).unwrap();
        let err = ledger.deposit(fake_account, amount).unwrap_err();

        matches!(err, DomainError::AccountNotFound);
    }
}
