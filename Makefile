ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 0
STOP_BLOCK ?= +8193

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml output_era -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml output_era -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/rpc,sf/substreams/sink,sf/substreams/v1"

.PHONY: package
package:
	substreams pack ./substreams.yaml
