
all: build-bz2

build-%: %
	cd $< && cargo build -r --target wasm32-wasip2
