WASM_DIR := ../../target/wasm32-wasip2/release/

$(WASM_DIR)/sevenz_7z.wasm:
	cd ../golden/servez-7z && cargo build -r --target wasm32-wasip2

run-sevenz-7z: $(WASM_DIR)/sevenz_7z.wasm
	cargo run -r -- --wasm $< --function sammyne:sevenz7z/api@1.0.0#zip '[1,2,3]'

run-argon2-hash: $(WASM_DIR)/argon2_component.wasm
	cargo run -r -- --wasm $< --function sammyne:argon2/api@1.0.0#hash '[1,2,3]' '"hello-world-hello-world"'

run-pulldown-cmark-parse: $(WASM_DIR)/pulldown_cmark.wasm
	cargo run -r -- --wasm $< --function sammyne:argon2/api@1.0.0#hash '[1,2,3]' '"hello-world-hello-world"'