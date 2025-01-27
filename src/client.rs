use std::collections::HashMap;

use rust_decimal::Decimal;
use thiserror::Error;

use crate::{ids::TransactionID, positive_decimal::PositiveDecimal};

#[derive(Debug)]
pub struct Client {
    total: Decimal,
    held: Decimal,
    locked: bool,
    deposits: HashMap<TransactionID, Deposit>,
}

#[derive(Debug)]
struct Deposit {
    amount: PositiveDecimal,
    disputed: bool,
}

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error("account is locked, no transactions allowed")]
    AccountLocked,
    #[error("duplicate transaction id")]
    DuplicateTransactionID,
    #[error("insufficient funds")]
    InsufficientFunds,
    #[error("deposit not found")]
    DepositNotFound,
    #[error("deposit already disputed")]
    AlreadyDisputed,
    #[error("deposit not disputed")]
    NotDisputed,
}

impl Client {
    pub fn new() -> Self {
        Self {
            total: Decimal::ZERO,
            held: Decimal::ZERO,
            locked: false,
            deposits: HashMap::new(),
        }
    }

    pub fn available(&self) -> Decimal {
        self.total - self.held
    }

    pub fn held(&self) -> Decimal {
        self.held
    }

    pub fn total(&self) -> Decimal {
        self.total
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    fn ensure_not_locked(&self) -> Result<(), ProcessingError> {
        match self.locked {
            true => Err(ProcessingError::AccountLocked),
            false => Ok(()),
        }
    }

    pub fn deposit(
        &mut self,
        tx: TransactionID,
        amount: PositiveDecimal,
    ) -> Result<(), ProcessingError> {
        // ensure not locked
        self.ensure_not_locked()?;

        // verify that the transaction id is unique
        if self.deposits.contains_key(&tx) {
            return Err(ProcessingError::DuplicateTransactionID);
        }

        // insert amount into deposits
        self.deposits.insert(
            tx,
            Deposit {
                amount,
                disputed: false,
            },
        );

        self.total += Decimal::from(amount);

        Ok(())
    }

    pub fn withdrawal(
        &mut self,
        _tx: TransactionID,
        amount: PositiveDecimal,
    ) -> Result<(), ProcessingError> {
        // ensure not locked
        self.ensure_not_locked()?;

        // insert amount into deposits
        // no need to check for negative amount since TransactionAmount is guaranteed to be positive
        let decimal_amount: Decimal = amount.into();

        // ensure sufficient funds
        if decimal_amount > self.total {
            return Err(ProcessingError::InsufficientFunds);
        }

        // make withdrawal
        self.total -= decimal_amount;

        Ok(())
    }

    pub fn dispute(&mut self, tx: TransactionID) -> Result<(), ProcessingError> {
        // ensure not locked
        self.ensure_not_locked()?;

        // get the deposit
        let deposit = self
            .deposits
            .get_mut(&tx)
            .ok_or(ProcessingError::DepositNotFound)?;

        // throw error if already disputed
        if deposit.disputed {
            return Err(ProcessingError::AlreadyDisputed);
        }

        // hold the disputed amount
        self.held += Decimal::from(deposit.amount);
        deposit.disputed = true;

        Ok(())
    }

    pub fn resolve(&mut self, tx: TransactionID) -> Result<(), ProcessingError> {
        // ensure not locked
        self.ensure_not_locked()?;

        // get the deposit
        let deposit = self
            .deposits
            .get_mut(&tx)
            .ok_or(ProcessingError::DepositNotFound)?;

        // throw error if the deposit is not disputed
        if !deposit.disputed {
            return Err(ProcessingError::NotDisputed);
        }

        // release the disputed amount
        self.held -= Decimal::from(deposit.amount);
        deposit.disputed = false;

        Ok(())
    }

    pub fn chargeback(&mut self, tx: TransactionID) -> Result<(), ProcessingError> {
        // ensure not locked
        self.ensure_not_locked()?;

        // get the deposit
        let deposit = self
            .deposits
            .get_mut(&tx)
            .ok_or(ProcessingError::DepositNotFound)?;

        // throw error if the deposit is not disputed
        if !deposit.disputed {
            return Err(ProcessingError::NotDisputed);
        }

        // release the disputed amount
        self.held -= Decimal::from(deposit.amount);
        self.total -= Decimal::from(deposit.amount);
        deposit.disputed = false;

        // lock the account
        self.locked = true;

        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_chargeback_success_single_transaction() {
        let mut client = Client::new();

        client
            .deposit(TransactionID::new(1), dec!(10.0).try_into().unwrap())
            .expect("deposit should succeed");
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(10.0));
        assert_eq!(client.locked(), false);

        client
            .dispute(TransactionID::new(1))
            .expect("dispute should succeed");
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.held(), dec!(10.0));
        assert_eq!(client.available(), dec!(0.0));
        assert_eq!(client.locked(), false);

        client
            .chargeback(TransactionID::new(1))
            .expect("chargeback should succeed");
        assert_eq!(client.total(), dec!(0.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(0.0));
        assert_eq!(client.locked(), true);

        client
            .deposit(TransactionID::new(2), dec!(1000.0).try_into().unwrap())
            .expect_err("deposit should fail on locked account");
        assert_eq!(client.total(), dec!(0.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(0.0));
        assert_eq!(client.locked(), true);
    }

    #[test]
    fn test_chargeback_success_multiple_transaction() {
        let mut client = Client::new();

        client
            .deposit(TransactionID::new(1), dec!(10.0).try_into().unwrap())
            .expect("deposit should succeed");
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(10.0));
        assert_eq!(client.locked(), false);

        client
            .deposit(TransactionID::new(2), dec!(1000.0).try_into().unwrap())
            .expect("deposit should succeed");
        assert_eq!(client.total(), dec!(1010.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(1010.0));
        assert_eq!(client.locked(), false);

        client
            .dispute(TransactionID::new(1))
            .expect("dispute should succeed");
        assert_eq!(client.total(), dec!(1010.0));
        assert_eq!(client.held(), dec!(10.0));
        assert_eq!(client.available(), dec!(1000.0));
        assert_eq!(client.locked(), false);

        client
            .chargeback(TransactionID::new(1))
            .expect("chargeback should succeed");
        assert_eq!(client.total(), dec!(1000.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(1000.0));
        assert_eq!(client.locked(), true);

        client
            .deposit(TransactionID::new(2), dec!(1000.0).try_into().unwrap())
            .expect_err("deposit should fail on locked account");
        assert_eq!(client.total(), dec!(1000.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(1000.0));
        assert_eq!(client.locked(), true);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut client = Client::new();

        client
            .deposit(TransactionID::new(1), dec!(10.0).try_into().unwrap())
            .expect("deposit should succeed");
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.available(), dec!(10.0));

        client
            .withdrawal(TransactionID::new(2), dec!(20.0).try_into().unwrap())
            .expect_err("withdrawal should fail due to insufficient funds");
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.available(), dec!(10.0));
    }

    #[test]
    fn test_dispute_nonexistent_transaction() {
        let mut client = Client::new();

        client
            .dispute(TransactionID::new(1))
            .expect_err("dispute should fail for non-existent transaction");
        assert_eq!(client.total(), dec!(0.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(0.0));
    }

    #[test]
    fn test_dispute_already_disputed() {
        let mut client = Client::new();

        client
            .deposit(TransactionID::new(1), dec!(10.0).try_into().unwrap())
            .expect("deposit should succeed");
        
        client
            .dispute(TransactionID::new(1))
            .expect("first dispute should succeed");
        
        client
            .dispute(TransactionID::new(1))
            .expect_err("second dispute should fail");
        
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.held(), dec!(10.0));
        assert_eq!(client.available(), dec!(0.0));
    }

    #[test]
    fn test_resolve_nonexistent_transaction() {
        let mut client = Client::new();

        client
            .resolve(TransactionID::new(1))
            .expect_err("resolve should fail for non-existent transaction");
        assert_eq!(client.total(), dec!(0.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(0.0));
    }

    #[test]
    fn test_resolve_undisputed_transaction() {
        let mut client = Client::new();

        client
            .deposit(TransactionID::new(1), dec!(10.0).try_into().unwrap())
            .expect("deposit should succeed");
        
        client
            .resolve(TransactionID::new(1))
            .expect_err("resolve should fail for undisputed transaction");
        
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(10.0));
    }

    #[test]
    fn test_chargeback_nonexistent_transaction() {
        let mut client = Client::new();

        client
            .chargeback(TransactionID::new(1))
            .expect_err("chargeback should fail for non-existent transaction");
        assert_eq!(client.total(), dec!(0.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(0.0));
        assert_eq!(client.locked(), false);
    }

    #[test]
    fn test_chargeback_undisputed_transaction() {
        let mut client = Client::new();

        client
            .deposit(TransactionID::new(1), dec!(10.0).try_into().unwrap())
            .expect("deposit should succeed");
        
        client
            .chargeback(TransactionID::new(1))
            .expect_err("chargeback should fail for undisputed transaction");
        
        assert_eq!(client.total(), dec!(10.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(10.0));
        assert_eq!(client.locked(), false);
    }

    #[test]
    fn test_locked_account_operations() {
        let mut client = Client::new();

        // First set up an account and lock it through chargeback
        client
            .deposit(TransactionID::new(1), dec!(100.0).try_into().unwrap())
            .expect("initial deposit should succeed");
        client.dispute(TransactionID::new(1)).expect("dispute should succeed");
        client.chargeback(TransactionID::new(1)).expect("chargeback should succeed");
        assert_eq!(client.locked(), true);

        // Try deposit
        client
            .deposit(TransactionID::new(2), dec!(50.0).try_into().unwrap())
            .expect_err("deposit should fail on locked account");

        // Try withdrawal
        client
            .withdrawal(TransactionID::new(3), dec!(10.0).try_into().unwrap())
            .expect_err("withdrawal should fail on locked account");

        // Try dispute
        client
            .deposit(TransactionID::new(4), dec!(20.0).try_into().unwrap())
            .expect_err("deposit for dispute setup should fail");
        client
            .dispute(TransactionID::new(4))
            .expect_err("dispute should fail on locked account");

        // Try resolve
        client
            .resolve(TransactionID::new(1))
            .expect_err("resolve should fail on locked account");

        // Try chargeback
        client
            .chargeback(TransactionID::new(1))
            .expect_err("chargeback should fail on locked account");

        // Verify account state remains unchanged
        assert_eq!(client.total(), dec!(0.0));
        assert_eq!(client.held(), dec!(0.0));
        assert_eq!(client.available(), dec!(0.0));
        assert_eq!(client.locked(), true);
    }
}
