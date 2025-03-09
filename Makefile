build-runtime:
	cd runtime && wasm-pack build

start-ui: build-runtime
	cd ui && npm start

build-runtime-release:
	cd runtime && wasm-pack build --release

build-ui-release: build-runtime-release
	cd ui && npm run build

build-server-release:
	cd server && cargo build --release

build: build-server-release

start-server: build
	cd server && RUST_LOG=info cargo run -- --web-port 3000 --mongo-url ${MONGO_URL}

.PHONY: build-runtime start-ui build-runtime-release build-ui-release build-server-release build start-server