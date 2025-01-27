# Rust Coding Test [![Cargo Build & Test](https://github.com/HODLMyTakehomeTest/toy_stream_processor/actions/workflows/ci.yml/badge.svg)](https://github.com/HODLMyTakehomeTest/toy_stream_processor/actions/workflows/ci.yml)
## Scoring checklist
### Basics
Basics are covered by github actions.
- [x] Code compiles
- [x] Code is properly formatted
- [x] Code reads and writes data as expected:
CLI interface: `cargo run -- transactions.csv > accounts.csv`

### Completeness
The following cases are covered:
- [x] Deposit
    - [x] Don't allow deposit if account is locked (covered by test :white_check_mark:)
    - [x] Don't allow duplicate transaction ID (covered by test :white_check_mark:)
    - [x] Don't allow negative deposits (blocked using `PositiveDecimal` NewType)
    - [x] Balances are correctly updated (covered by test :white_check_mark:)
- [x] Withdrawal
    - [x] Don't allow withdrawal if account is locked (covered by test :white_check_mark:)
    - [x] Don't allow negative withdrawals (blocked using `PositiveDecimal` NewType)
    - [x] Don't allow withdrawal if insufficient funds (covered by test :white_check_mark:)
    - [x] Balances are correctly updated (covered by test :white_check_mark:)
- [x] Dispute
    - [x] Don't allow dispute if account is locked (covered by test :white_check_mark:)
    - [x] Ignore disputes for non-existent transactions (covered by test :white_check_mark:)
    - [x] Don't allow disputes for already disputed transactions (covered by test :white_check_mark:)
    - [x] Deposit is marked as disputed (indirectly covered by test :white_check_mark:)
    - [x] Balances are correctly updated (covered by test :white_check_mark:)
- [x] Resolve
    - [x] Don't allow resolve if account is locked (covered by test :white_check_mark:)
    - [x] Ignore resolves for non-existent transactions (covered by test :white_check_mark:)
    - [x] Don't allow resolves transactions that are not disputed (covered by test :white_check_mark:)
    - [x] Deposit is marked as not disputed (indirectly covered by test :white_check_mark:)
    - [x] Balances are correctly updated (covered by test :white_check_mark:)
- [x] Chargeback
    - [x] Don't allow chargeback if account is locked (covered by test :white_check_mark:)
    - [x] Ignore chargebacks for non-existent transactions (covered by test :white_check_mark:)
    - [x] Don't allow chargebacks transactions that are not disputed (covered by test :white_check_mark:)
    - [x] Deposit is marked as not disputed (indirectly covered by test :white_check_mark:)
    - [x] Balances are correctly updated (covered by test :white_check_mark:)
    - [x] Account is locked (indirectly covered by test :white_check_mark:)

### Efficiency
- Are you doing something dangerous? **no**
    - I've used `unwrap` once in the code (excluding tests), and I documented by it's fine to do so
- I've introduced a few NewTypes to make the code more readable and less error-prone
    - `ClientID` - To avoid passing the wrong ID type
    - `TransactionID` - To avoid passing the wrong ID type
    - `PositiveDecimal` - To avoid having to validate that an amount is positive and non-zero over and over again
- I'm handling errors in two ways:
    - I use `thiserror` for the 'library' part of the code where we might be interested in the type of error
    - I use `anyhow` for the 'main' part of the code where we don't care about the type of error

### Maintainability
- [x] The code is well-documented
- [x] The code uses typesystem to avoid errors
- [x] The code is well-tested
- [x] Contains optional logging using `tracing` crate
    - The gradularity of logging is controlled by `RUST_LOG` environment variable
    - Logging output goes to `stderr` to avoid polluting `stdout`
- [x] Every struct has only one responsibility
    - `TransactionReader` - To read transactions from a CSV file
    - `Processor` - To keep track of clients and pass transactions to them
    - `Client` - Handles transactions for a single client
    - `main.rs` - Glues everything together

# Assumptions / remarks
## Only deposits are allowed to be disputed
It's not clear from the problem statement if only deposits are allowed to be disputed.
I've assumed that it is because I don't think it makes a lot of sense to dispute withdrawals.

## Once a disput has been resolved, it can be disuted again
It was not clear from the problem statement if a disputed transaction can be disputed again.
I've assumed that it is possible.

## Once an account is locked, no other transactions can be processed
The only thing the problem statement said is that an account is locked if a chargeback has been processed.
It did not mention which operations you can perform on a locked account.
I've assumed that it's impossible to perform any operations on a locked account.

## Disputing a transaction that would leave the account in a negative state after cashback is allowed
The problem statement does not say anything about this.
I've assumed that it's allowed since dispute does not actually reverse the transaction, it only holds the disputed ammount.
It's up to the person who's approving the chargeback to decide whether they want to reverse this transaction or not.
