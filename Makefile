build-runtime:
	cd runtime && wasm-pack build

start: build-runtime
	cd ui && npm start

build-runtime-release:
	cd runtime && wasm-pack build --release

build: build-runtime-release
	cd ui && npm run build

.PHONY: build-runtime start