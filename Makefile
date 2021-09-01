rootPath = $(realpath .)
internalsPath = $(rootPath)/internals
buildPath = $(rootPath)/build
binPath = $(buildPath)/bin

.PHONY: build
build:
	@echo "\n>>> make build"
	ROOT_PATH=$(shell pwd) ./internals/ci/build.sh

dev:
	@echo "\n>>> make dev"
	ROOT_PATH=$(shell pwd) ./internals/ci/dev.sh
.PHONY: dev

test:
	@echo "\n>>> make test"
	ROOT_PATH=$(shell pwd) ./internals/ci/test.sh
.PHONY: test
