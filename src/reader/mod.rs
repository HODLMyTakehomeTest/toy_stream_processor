use std::io;

use csv::{ReaderBuilder, Trim};
use raw_transaction::RawTransaction;
use tracing::warn;

use crate::transaction::Transaction;

mod raw_transaction;

pub struct TransactionReader<R> {
    iter: csv::DeserializeRecordsIntoIter<R, RawTransaction>,
}

impl<R> TransactionReader<R>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        let reader = ReaderBuilder::new().trim(Trim::All).from_reader(reader);
        let iter = reader.into_deserialize();

        Self { iter }
    }

    pub fn next(&mut self) -> Option<Transaction> {
        // loop until we are able to return a valid transaction
        loop {
            // try to get the next raw transaction
            let raw_transaction_result = self.iter.next()?;

            // in case we fail, print a warning and continue
            let raw_transaction = match raw_transaction_result {
                Ok(raw_transaction) => raw_transaction,
                Err(err) => {
                    warn!("skipping invalid transaction: {}", err);
                    continue;
                }
            };

            // try to convert the raw transaction to a transaction
            // in case we fail, print a warning and continue
            match raw_transaction.try_into() {
                Ok(transaction) => return Some(transaction),
                Err(err) => {
                    warn!("skipping invalid transaction: {}", err);
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{
        ids::{ClientID, TransactionID},
        positive_decimal::PositiveDecimal,
        transaction::{Deposit, Withdrawal},
    };

    use super::*;

    #[test]
    fn test_reader_valid() {
        let csv = r#"type, client, tx, amount
deposit,1,1,1.1
withdrawal,2,3,4.5678"#;

        let csv_bytes = csv.as_bytes();
        let mut reader = TransactionReader::new(csv_bytes);

        assert_eq!(
            reader.next(),
            Some(Transaction::Deposit(Deposit {
                amount: PositiveDecimal::new(dec!(1.1)).unwrap(),
                client: ClientID::new(1),
                tx: TransactionID::new(1)
            }))
        );

        assert_eq!(
            reader.next(),
            Some(Transaction::Withdrawal(Withdrawal {
                amount: PositiveDecimal::new(dec!(4.5678)).unwrap(),
                client: ClientID::new(2),
                tx: TransactionID::new(3)
            }))
        );

        assert_eq!(reader.next(), None);
    }

    #[test]
    fn test_skip_invalid() {
        let csv = "type, client, tx, amount\ninvalid,999,999,9.9999\ndeposit,-1,999,9.9999\ndeposit,-1,-999\ndeposit,1,1,1.1";
        let csv_bytes = csv.as_bytes();
        let mut reader = TransactionReader::new(csv_bytes);

        assert_eq!(
            reader.next(),
            Some(Transaction::Deposit(Deposit {
                amount: PositiveDecimal::new(dec!(1.1)).unwrap(),
                client: ClientID::new(1),
                tx: TransactionID::new(1)
            }))
        );
        assert_eq!(reader.next(), None);
    }
}
