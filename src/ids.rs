use derive_more::{Constructor, Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Constructor,
    Copy,
    Debug,
    Deserialize,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct ClientID(u16);
#[derive(
    Clone,
    Constructor,
    Copy,
    Debug,
    Deserialize,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct TransactionID(u32);
