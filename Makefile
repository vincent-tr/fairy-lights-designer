build-runtime:
	cd runtime && wasm-pack build

start: build-runtime
	cd ui && npm start

build-runtime-release:
	cd runtime && wasm-pack build --release

build-ui-release: build-runtime-release
	cd ui && npm run build

build-server-release:
	cd server && cargo build --release

build: build-server-release

.PHONY: build-runtime start