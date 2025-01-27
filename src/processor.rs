use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    client::{Client, ProcessingError},
    ids::ClientID,
    transaction::Transaction,
};

pub struct Processor {
    clients: HashMap<ClientID, Client>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn handle_transaction(&mut self, transaction: Transaction) -> Result<(), ProcessingError> {
        let client = self.clients.entry(transaction.client()).or_default();

        match transaction {
            Transaction::Deposit(deposit) => client.deposit(deposit.tx, deposit.amount),
            Transaction::Withdrawal(withdrawal) => {
                client.withdrawal(withdrawal.tx, withdrawal.amount)
            }
            Transaction::Dispute(dispute) => client.dispute(dispute.tx),
            Transaction::Resolve(resolve) => client.resolve(resolve.tx),
            Transaction::Chargeback(chargeback) => client.chargeback(chargeback.tx),
        }
    }

    pub fn status_entries<'a>(&'a self) -> impl Iterator<Item = ProcessorStatusEntry> + 'a {
        self.clients
            .iter()
            .map(|(client_id, client)| ProcessorStatusEntry {
                client: *client_id,
                available: client.available(),
                held: client.held(),
                total: client.total(),
                locked: client.locked(),
            })
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ProcessorStatusEntry {
    pub client: ClientID,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{
        ids::TransactionID,
        transaction::{Deposit, Dispute, Resolve, Withdrawal},
    };

    use super::*;

    #[test]
    fn basic_transactions_test() {
        let mut processor = Processor::new();
        processor
            .handle_transaction(Transaction::Deposit(Deposit {
                tx: TransactionID::new(1),
                client: ClientID::new(1),
                amount: dec!(10.0).try_into().unwrap(),
            }))
            .unwrap();

        processor
            .handle_transaction(Transaction::Withdrawal(Withdrawal {
                tx: TransactionID::new(2),
                client: ClientID::new(1),
                amount: dec!(5.0).try_into().unwrap(),
            }))
            .unwrap();

        processor
            .handle_transaction(Transaction::Dispute(Dispute {
                tx: TransactionID::new(1),
                client: ClientID::new(1),
            }))
            .unwrap();

        processor
            .handle_transaction(Transaction::Resolve(Resolve {
                tx: TransactionID::new(1),
                client: ClientID::new(1),
            }))
            .unwrap();

        processor
            .handle_transaction(Transaction::Deposit(Deposit {
                tx: TransactionID::new(3),
                client: ClientID::new(2),
                amount: dec!(1000.0).try_into().unwrap(),
            }))
            .unwrap();

        let mut entries = processor
            .status_entries()
            .map(|e| (e.client, e))
            .collect::<HashMap<ClientID, _>>();

        assert_eq!(
            Some(ProcessorStatusEntry {
                client: ClientID::new(1),
                available: dec!(5.0),
                held: dec!(0.0),
                total: dec!(5.0),
                locked: false,
            }),
            entries.remove(&ClientID::new(1))
        );

        assert_eq!(
            Some(ProcessorStatusEntry {
                client: ClientID::new(2),
                available: dec!(1000.0),
                held: dec!(0.0),
                total: dec!(1000.0),
                locked: false,
            }),
            entries.remove(&ClientID::new(2))
        );

        assert!(entries.is_empty());
    }
}
