OUT_DIR = $(shell pwd)/out
BUILD_DIR = $(shell pwd)/../../../target/wasm32-wasip2/release

CLI_DIR := $(shell pwd)/../../cli

.PHONY: all
all: $(OUT_DIR)/cmd.wasm $(OUT_DIR)/reactor.wasm

$(OUT_DIR)/cmd.wasm: $(OUT_DIR)
	cargo build -r --target wasm32-wasip2 --bin pulldown-cmark-cmd
	cp $(BUILD_DIR)/pulldown-cmark-cmd.wasm $@

$(OUT_DIR)/reactor.wasm: $(OUT_DIR)
	cargo build -r --target wasm32-wasip2
	cp $(BUILD_DIR)/pulldown_cmark_reactor.wasm $@

run-reactor: $(OUT_DIR)/reactor.wasm
	cd $(CLI_DIR) && cargo run -r -- --wasm $< --function sammyne:pulldown-cmark/api@1.0.0#parse '"Hello world, this is a ~~complicated~~ *very simple* example."'

run-cmd: $(OUT_DIR)/cmd.wasm
	cd $(CLI_DIR) && cargo run -r -- 

$(OUT_DIR):
	mkdir -p $@

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)
