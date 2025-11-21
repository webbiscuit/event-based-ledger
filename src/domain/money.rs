use core::fmt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    GBP,
    USD,
    EUR,
}

impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::USD => "$"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Minor units (e.g. pence or cents). Always >= 0.
    amount: i64,
    currency: Currency,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MoneyError {
    #[error("amount must be non-negative (got {0})")]
    Negative(i64),

    #[error("cannot operate on different currencies: {0:?} vs {1:?}")]
    CurrencyMismatch(Currency, Currency),
}

impl Money {
    /// Create a Money value in minor units (e.g. pence).
    pub fn new_minor(amount: i64, currency: Currency) -> Result<Self, MoneyError> {
        if amount < 0 {
            return Err(MoneyError::Negative(amount));
        }
        Ok(Self { amount, currency })
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn checked_add(self, other: Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }

    pub fn checked_sub(self, other: Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(self.currency, other.currency));
        }
        if self.amount < other.amount {
            return Err(MoneyError::Negative(self.amount - other.amount));
        }
        Ok(Self {
            amount: self.amount - other.amount,
            currency: self.currency,
        })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = 2;

        let divisor = 10_i64.pow(digits);
        let major = self.amount / divisor;
        let minor = self.amount % divisor;

        write!(
            f,
            "{}{}.{:02}",
            self.currency.symbol(),
            major,
            minor
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_minor_rejects_negative() {
        let result = Money::new_minor(-1, Currency::GBP);
        assert_eq!(result, Err(MoneyError::Negative(-1)));
    }

    #[test]
    fn new_minor_creates_valid_money() {
        let m = Money::new_minor(123, Currency::GBP).unwrap();
        assert_eq!(m.amount(), 123);
        assert_eq!(m.currency(), Currency::GBP);
    }

    #[test]
    fn checked_add_same_currency_succeeds() {
        let a = Money::new_minor(100, Currency::GBP).unwrap();
        let b = Money::new_minor(50, Currency::GBP).unwrap();

        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum.amount(), 150);
        assert_eq!(sum.currency(), Currency::GBP);
    }

    #[test]
    fn checked_add_different_currency_fails() {
        let gbp = Money::new_minor(100, Currency::GBP).unwrap();
        let usd = Money::new_minor(50, Currency::USD).unwrap();

        let result = gbp.checked_add(usd);
        assert_eq!(
            result,
            Err(MoneyError::CurrencyMismatch(Currency::GBP, Currency::USD))
        );
    }

    #[test]
    fn checked_sub_cannot_go_negative() {
        let a = Money::new_minor(100, Currency::GBP).unwrap();
        let b = Money::new_minor(150, Currency::GBP).unwrap();

        let result = a.checked_sub(b);
        assert!(matches!(result, Err(MoneyError::Negative(_))));
    }

    #[test]
    fn display_formats_gbp_properly() {
        let m = Money::new_minor(3400, Currency::GBP).unwrap();
        assert_eq!(m.to_string(), "£34.00");
    }

    #[test]
    fn display_formats_small_amounts() {
        let m = Money::new_minor(99, Currency::GBP).unwrap();
        assert_eq!(m.to_string(), "£0.99");
    }

    #[test]
    fn display_formats_usd() {
        let m = Money::new_minor(1250, Currency::USD).unwrap();
        assert_eq!(m.to_string(), "$12.50");
    }

    #[test]
    fn display_formats_eur() {
        let m = Money::new_minor(12345, Currency::EUR).unwrap();
        assert_eq!(m.to_string(), "€123.45");
    }

    #[test]
    fn display_formats_euro_zero() {
        let m = Money::new_minor(0, Currency::EUR).unwrap();
        assert_eq!(m.to_string(), "€0.00");
    }
}
