DOCKER_REPOSITORY ?= vincenttr
DOCKER_PACKAGE_NAME ?= fairy-lights-designer
DOCKER_PACKAGE_VERSION ?= 1.0.1
DOCKER_IMAGE_TAG ?= $(DOCKER_REPOSITORY)/$(DOCKER_PACKAGE_NAME):$(DOCKER_PACKAGE_VERSION)
DOCKER_IMAGE_LATEST_TAG ?= $(DOCKER_REPOSITORY)/$(DOCKER_PACKAGE_NAME):latest

build-runtime:
	cd runtime && wasm-pack build

start-ui: build-runtime
	cd ui && npm start

start-server: build
	cd server && RUST_LOG=info cargo run -- --web-port 3000 --mongo-url ${MONGO_URL}

build-runtime-release:
	cd runtime && wasm-pack build --release

build-ui-release: build-runtime-release
	cd ui && npm run build

build-server-release: build-ui-release
	cd server && cargo build --release

build: build-server-release

docker-publish: docker-build
	docker push "$(DOCKER_IMAGE_TAG)"
	docker push "$(DOCKER_IMAGE_LATEST_TAG)"

docker-build: build
	docker build --pull -t "$(DOCKER_IMAGE_TAG)" -t "$(DOCKER_IMAGE_LATEST_TAG)" -f docker/Dockerfile server/target/release

.PHONY: build-runtime start-ui build-runtime-release build-ui-release build-server-release build start-server docker-publish docker-build