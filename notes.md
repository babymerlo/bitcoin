# Rust

- [x] CBOR
- [x] trait associated types
- [x] attributes like #[serde(...)]

# BTC
- [x] Blockchain state -> all UTXO
- [x] inputs -> ref old outputs + sign + trx validation
- [x] output -> withdraw
- [x] trx -> inputs + outputs + fee + metadata

- [x] Find hash of block header which is less then target - Proof of Work. Nonce

### Difficulty
- [x] Adjust target - manage difficulty
- [x] Diff = TargetMax / CurrentTarget
- [x] Hashcash, Byzantine Fault Tolerance (BFT)