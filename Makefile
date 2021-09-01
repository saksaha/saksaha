rootPath = $(realpath .)
internalsPath = $(rootPath)/internals
buildPath = $(rootPath)/build
binPath = $(buildPath)/bin

.PHONY: check_rust
check_rust:
	@echo "\n>> make check_rust"
	internals/build/check_rust.sh

clean:
	@echo "\n>> make clean"
	cd $(buildPath) && rm -rf ./*

.PHONY: build
build: check_rust
	@echo "\n>>> make build"
	env PROJECT_ROOT=$(rootPath) $(buildPath)/build

create_build:
	@echo "\n>>> make create_build"
	cd internals/build && go build -o $(buildPath)/build
.PHONY: create_build

dev: build
	@echo "\n>>> make dev"
	$(binPath)/sak $(filter-out $@,$(MAKECMDGOALS))
.PHONY: dev

test:
	@echo "\n>>> make test"
	go test -v ./...
.PHONY: test
