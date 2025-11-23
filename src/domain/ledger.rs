use tracing::info;

use crate::domain::{Currency, Money, errors::DomainError, events::{LedgerEvent, LedgerEventPayload}, types::{AccountId, EventId}};

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

    pub fn withdraw(&mut self, account_id: AccountId, amount: Money) -> Result<EventId, DomainError> {
        if !self.account_exists(account_id) {
            return Err(DomainError::AccountNotFound)
        }

        info!("Withdrawing {} from {}", amount, account_id);

        let balance = self.balance_for_account(account_id)?;

        if balance.amount() < amount.amount() {
            return Err(DomainError::InsufficientFunds { required_minor: amount.amount(), available_minor: balance.amount() });
        }

        let event = LedgerEvent::withdraw(account_id, amount);
        let id = event.id;

        self.events.push(event);

        Ok(id)
    }

    pub fn balance_for_account(&self, account_id: AccountId) -> Result<Money, DomainError> {
        let events = self.events_for_account(account_id)?;

        // Only supporting GBP for now
        let mut balance = Money::zero(Currency::Gbp);

        for event in events {
            match event.payload {
                LedgerEventPayload::Deposit { amount } => {
                    balance = balance.checked_add(amount).map_err(DomainError::InvalidMoney)?
                }
                LedgerEventPayload::Withdraw { amount } => {
                    balance = balance.checked_sub(amount).map_err(DomainError::InvalidMoney)?
                }
                _ => {}
            }
        }

        Ok(balance)
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

    #[test]
    fn withdrawal_reduces_balance_when_sufficient_funds() {
        let mut ledger = Ledger::new();
        let account = ledger.open_account();

        let ten = Money::new_minor(10_00, Currency::Gbp).unwrap();
        let four = Money::new_minor(4_00, Currency::Gbp).unwrap();

        ledger.deposit(account, ten).unwrap();
        ledger.withdraw(account, four).unwrap();

        let balance = ledger.balance_for_account(account).unwrap();
        assert_eq!(balance.amount(), 6_00);
    }

    #[test]
    fn withdrawal_fails_when_insufficient_funds() {
        let mut ledger = Ledger::new();
        let account = ledger.open_account();

        let five = Money::new_minor(5_00, Currency::Gbp).unwrap();
        let ten = Money::new_minor(10_00, Currency::Gbp).unwrap();

        ledger.deposit(account, five).unwrap();

        let err = ledger.withdraw(account, ten).unwrap_err();
        match err {
            DomainError::InsufficientFunds {
                required_minor,
                available_minor,
            } => {
                assert_eq!(required_minor, 10_00);
                assert_eq!(available_minor, 5_00);
            }
            other => panic!("expected InsufficientFunds, got {other:?}"),
        }
    }
}
