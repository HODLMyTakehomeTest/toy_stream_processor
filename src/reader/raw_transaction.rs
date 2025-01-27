use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use thiserror::Error;

use crate::{
    ids::{ClientID, TransactionID},
    positive_decimal::PositiveDecimal,
    transaction::{Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal},
};

// Introduced RawTransaction to workaround a rust-csv issue
// Tagged enums are not supported
// https://github.com/BurntSushi/rust-csv/issues/211
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct RawTransaction {
    #[serde(rename = "type")]
    pub transaction_type: RawTransactionType,
    pub client: ClientID,
    pub tx: TransactionID,
    pub amount: Option<PositiveDecimal>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RawTransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl Display for RawTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // It's safe to unwrap here as to_variant_name only fails when using incompatible enum types
        write!(f, "{}", to_variant_name(self).unwrap())
    }
}

#[derive(Debug, Error)]
pub enum RawTransactionConvertError {
    #[error("missing amount for transaction type: '{transaction_type}'")]
    MissingAmount {
        transaction_type: RawTransactionType,
    },
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = RawTransactionConvertError;

    fn try_from(value: RawTransaction) -> Result<Self, Self::Error> {
        // Map the raw transaction type to a transaction
        Ok(match value.transaction_type {
            RawTransactionType::Deposit => Transaction::Deposit(Deposit {
                amount: get_transaction_amount(&value)?,
                client: value.client,
                tx: value.tx,
            }),
            RawTransactionType::Withdrawal => Transaction::Withdrawal(Withdrawal {
                amount: get_transaction_amount(&value)?,
                client: value.client,
                tx: value.tx,
            }),
            RawTransactionType::Dispute => Transaction::Dispute(Dispute {
                client: value.client,
                tx: value.tx,
            }),
            RawTransactionType::Resolve => Transaction::Resolve(Resolve {
                client: value.client,
                tx: value.tx,
            }),
            RawTransactionType::Chargeback => Transaction::Chargeback(Chargeback {
                client: value.client,
                tx: value.tx,
            }),
        })
    }
}

fn get_transaction_amount(
    raw_transaction: &RawTransaction,
) -> Result<PositiveDecimal, RawTransactionConvertError> {
    let amount =
        raw_transaction
            .amount
            .ok_or_else(|| RawTransactionConvertError::MissingAmount {
                transaction_type: raw_transaction.transaction_type,
            })?;

    Ok(amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    macro_rules! deserialize_test {
        ($test_name:ident, $csv_str:literal, $expected_raw_transaction:expr) => {
            #[test]
            fn $test_name() {
                let csv = concat!("type, client, tx, amount\n", $csv_str);

                let csv_bytes = csv.as_bytes();
                let mut reader = ::csv::ReaderBuilder::new()
                    .trim(::csv::Trim::All)
                    .from_reader(csv_bytes);

                let transaction: RawTransaction = reader
                    .deserialize()
                    .next()
                    .unwrap()
                    .expect("valid raw transaction");

                assert_eq!(transaction, $expected_raw_transaction);
            }
        };
    }

    deserialize_test!(
        test_deserialize_deposit,
        "deposit, 1, 1, 1.1",
        RawTransaction {
            transaction_type: RawTransactionType::Deposit,
            client: ClientID::new(1),
            tx: TransactionID::new(1),
            amount: Some(PositiveDecimal::new(dec!(1.1)).unwrap()),
        }
    );

    deserialize_test!(
        test_deserialize_withdrawal,
        "withdrawal, 2, 2, 2.22",
        RawTransaction {
            transaction_type: RawTransactionType::Withdrawal,
            client: ClientID::new(2),
            tx: TransactionID::new(2),
            amount: Some(PositiveDecimal::new(dec!(2.22)).unwrap()),
        }
    );

    deserialize_test!(
        test_deserialize_dispute,
        "dispute, 3, 3,",
        RawTransaction {
            transaction_type: RawTransactionType::Dispute,
            client: ClientID::new(3),
            tx: TransactionID::new(3),
            amount: None,
        }
    );

    deserialize_test!(
        test_deserialize_resolve,
        "resolve, 4, 4,",
        RawTransaction {
            transaction_type: RawTransactionType::Resolve,
            client: ClientID::new(4),
            tx: TransactionID::new(4),
            amount: None,
        }
    );

    deserialize_test!(
        test_deserialize_chargeback,
        "chargeback, 5, 5,",
        RawTransaction {
            transaction_type: RawTransactionType::Chargeback,
            client: ClientID::new(5),
            tx: TransactionID::new(5),
            amount: None,
        }
    );
}
