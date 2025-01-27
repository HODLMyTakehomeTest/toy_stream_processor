use derive_more::{Constructor, Display, From, Into};
use serde::{Deserialize, Serialize};

/// A strongly-typed identifier for a client in the transaction processing system.
///
/// This type wraps a `u16` to provide type safety and prevent mixing up client IDs with other numeric values.
/// It implements serialization/deserialization and common traits for comparison and display.
///
/// # Examples
/// ```
/// let client_id = ClientID::new(1234);
/// ```
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
    PartialEq,
    Serialize,
)]
pub struct ClientID(u16);

/// A strongly-typed identifier for a transaction in the processing system.
///
/// This type wraps a `u32` to provide type safety and prevent mixing up transaction IDs with other numeric values.
/// Implements serialization/deserialization and common traits for comparison and display.
///
/// # Examples
/// ```
/// let tx_id = TransactionID::new(5678);
/// ```
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
    PartialEq,
    Serialize,
)]
pub struct TransactionID(u32);
