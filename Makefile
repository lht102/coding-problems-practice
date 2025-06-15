SHELL := /bin/sh

.PHONY: fmt
fmt:
	go fmt ./...
	cargo fmt
	uv run ruff format

.PHONY: test
test:
	go test ./...
	cargo test -- --nocapture
	uv run python -m unittest

.PHONY: lint
lint:
	golangci-lint run --fix
	cargo clippy --fix --allow-dirty
	uv run ruff check
