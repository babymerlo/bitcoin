.PHONY: block block_print tx_gen tx_print mining_test

block_gen:
	cargo run --bin block_gen block.cbor
block_print:
	cargo run --bin block_print block.cbor
tx_gen:
	cargo run --bin tx_gen tx.cbor
tx_print:
	cargo run --bin tx_print tx.cbor
mining_test:
	@echo "Running miner with ROUNDS=$(ROUNDS)"
	cargo run --bin miner ./block.cbor $(ROUNDS)
