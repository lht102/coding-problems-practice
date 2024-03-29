SHELL := /bin/sh

.PHONY: fmt
fmt:
	go fmt ./...
	cargo fmt

.PHONY: test
test:
	go test ./...
	cargo test -- --nocapture

.PHONY: lint
lint:
	golangci-lint run --fix
	cargo clippy --fix --allow-dirty
