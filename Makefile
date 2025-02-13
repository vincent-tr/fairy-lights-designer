build-runtime:
	cd runtime && wasm-pack build

start: build-runtime
	cd ui && npm start

.PHONY: build-runtime start