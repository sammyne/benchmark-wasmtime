OUT_DIR = $(shell pwd)/out
BUILD_DIR = $(shell pwd)/../../../target/wasm32-wasip2/release

CLI_DIR := $(shell pwd)/../../cli

NAME = $(shell grep '^name = ' Cargo.toml | head -1 | cut -d'"' -f2 | sed 's/-cmd$$//')

CMD_NAME = $(NAME)-cmd
REACTOR_NAME = $(subst -,_,$(NAME))_reactor

.PHONY: all
all: $(OUT_DIR)/cmd.wasm $(OUT_DIR)/reactor.wasm

$(OUT_DIR)/cmd.wasm: $(BUILD_DIR)/$(CMD_NAME).wasm $(OUT_DIR)
	cp $< $@

$(BUILD_DIR)/$(CMD_NAME).wasm: src/main.rs
	cargo build -r --target wasm32-wasip2 --bin $(CMD_NAME)

$(OUT_DIR)/reactor.wasm: $(BUILD_DIR)/$(REACTOR_NAME).wasm $(OUT_DIR)
	cp $< $@

$(BUILD_DIR)/$(REACTOR_NAME).wasm:
	cargo build -r --target wasm32-wasip2

run-reactor: $(OUT_DIR)/reactor.wasm
	cd $(CLI_DIR) && cargo run -r -- --wasm $< --function sammyne:sevenz7z/api@1.0.0#zip '[1,2,3]' '"hello-world-hello-world"'

run-cmd: $(OUT_DIR)/cmd.wasm
	cd $(CLI_DIR) && cargo run -r -- --wasm $< -f wasi:cli/run@0.2.0 123

$(OUT_DIR):
	mkdir -p $@

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)
