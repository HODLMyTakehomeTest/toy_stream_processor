# Rust Coding Test
## Scoring checklist
### Basics
- [x] Code compiles
- [x] Code reads and writes data as expected:
CLI interface: `cargo run -- transactions.csv > accounts.csv`
- [x] Code is properly formatted

Features


Assumptions




# Issues
- No dispute/resolve/cashback ID

# Assumptions
- Only deposits are allowed to be disputed
- Once a disput has been resolved/cashbacked, it can be disuted again
- Once an account is locked, no other transactions can be processed
- dispute a transaction if the that would mean the available would become negative, since we're not actually reversing the transaction
