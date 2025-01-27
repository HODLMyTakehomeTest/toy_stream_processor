use serde::Deserialize;

use crate::{
    ids::{ClientID, TransactionID},
    positive_decimal::PositiveDecimal,
};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Transaction {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Dispute(Dispute),
    Resolve(Resolve),
    Chargeback(Chargeback),
}

impl Transaction {
    pub fn client(&self) -> ClientID {
        match self {
            Transaction::Deposit(deposit) => deposit.client,
            Transaction::Withdrawal(withdrawal) => withdrawal.client,
            Transaction::Dispute(dispute) => dispute.client,
            Transaction::Resolve(resolve) => resolve.client,
            Transaction::Chargeback(chargeback) => chargeback.client,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Deposit {
    pub client: ClientID,
    pub tx: TransactionID,
    pub amount: PositiveDecimal,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Withdrawal {
    pub client: ClientID,
    pub tx: TransactionID,
    pub amount: PositiveDecimal,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Dispute {
    pub client: ClientID,
    pub tx: TransactionID,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Resolve {
    pub client: ClientID,
    pub tx: TransactionID,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Chargeback {
    pub client: ClientID,
    pub tx: TransactionID,
}
