.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	substreams pack substreams.yaml

.PHONY: run
run: package
	substreams run -e solana.substreams.pinax.network:443 \
		solana-transfers-v0.1.0.spkg \
		map_transfers \
		--start-block 385870151 \
		--stop-block 385870157

.PHONY: clean
clean:
	cargo clean
	rm -f *.spkg
