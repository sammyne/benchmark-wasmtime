WASM_DIR := ../../target/wasm32-wasip2/release

$(WASM_DIR)/sevenz_7z.wasm:
	cd ../golden/servez-7z && cargo build -r --target wasm32-wasip2

run-sevenz-7z: $(WASM_DIR)/sevenz_7z.wasm
	cargo run -r -- --wasm $< --function sammyne:sevenz7z/api@1.0.0#zip '[1,2,3]'

run-argon2-hash: $(WASM_DIR)/argon2_component.wasm
	cargo run -r -- --wasm $< --function sammyne:argon2/api@1.0.0#hash '[1,2,3]' '"hello-world-hello-world"'

run-pulldown-cmark-parse: $(WASM_DIR)/pulldown_cmark_component.wasm
	cargo run -r -- --wasm $< --function sammyne:pulldown-cmark/api@1.0.0#parse '"Hello world, this is a ~~complicated~~ *very simple* example."'

run-host-caller: $(WASM_DIR)/host_caller.wasm
	cargo run -r -- --wasm $< --with-host --function sammyne:host-caller/api@1.0.0#multi-sleep 2000 3

run-rust-python: $(WASM_DIR)/rust_python.wasm
	cargo run -r -- --wasm $< --function sammyne:rust-python/api@0.1.0#run '"print(\"Hello world\")"'