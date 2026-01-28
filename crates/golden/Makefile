
OUT_DIR := $(shell pwd)/out
BUILD_DIR := $(shell pwd)/../../target/wasm32-wasip2/release


pack: $(OUT_DIR) build-fast-argon2 build-fast-pulldown-cmark build-fast-sevenz-7z

build-fast-argon2: argon2 $(OUT_DIR)
	cd $< && cargo build -r --target wasm32-wasip2
	cp "$(BUILD_DIR)/argon2_component.wasm" "$(OUT_DIR)/$<.wasm"

build-fast-pulldown-cmark: pulldown-cmark $(OUT_DIR)
	cd $< && cargo build -r --target wasm32-wasip2
	cp "$(BUILD_DIR)/pulldown_cmark_component.wasm" "$(OUT_DIR)/$<.wasm"

build-fast-sevenz-7z: sevenz-7z $(OUT_DIR)
	cd $< && cargo build -r --target wasm32-wasip2
	cp "$(BUILD_DIR)/sevenz_7z.wasm" "$(OUT_DIR)/$<.wasm"

# build-fast-rust-python: rust-python $(OUT_DIR)
# 	cd $< && cargo build -r --target wasm32-wasip2
# 	cp "$(BUILD_DIR)/rust_python.wasm" "$(OUT_DIR)/$@.wasm"

build-dev-%: % $(OUT_DIR)
	cd $< && cargo build --target wasm32-wasip2

$(OUT_DIR):
	mkdir -p $@

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)
