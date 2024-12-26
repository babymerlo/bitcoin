# Rust

- [x] CBOR
- [x] trait associated types
- [x] attributes like #[serde(...)]

# BTC

- [x] Blockchain state -> all UTXO
- [x] inputs -> ref old outputs + sign + trx validation
- [x] output -> withdraw
- [x] trx -> inputs + outputs + fee + metadata
- [x] Find hash of block header which is less then target - Proof of Work. Adjust nonce

### Difficulty

- [x] Adjust target - manage difficulty
- [x] Diff = TargetMax / CurrentTarget
- [x] Hashcash, Byzantine Fault Tolerance (BFT)

### Mempool

- [x] Queue of trx
- [x] Sorted by fee
- [x] BTC Node -> validate trx before add to mempool
- [x] Miner -> validate trx before add to block. also get trx with highest fee
- [x] Validation -> verify sign of transaction input, verify balance, check double spending (uniq)
- [x] validation process - add trx to mempool which use already marked utxo (pzdc)
