# Makefile for Behavioral Pathways

.PHONY: help test tests longitudinal-tests fmt lint check build clean

# Default target
help:
	@echo "Behavioral Pathways - Development"
	@echo ""
	@echo "Common tasks:"
	@echo "  make test               - Run tests"
	@echo "  make tests              - Run tests with coverage"
	@echo "  make longitudinal-tests - Run longitudinal tests"
	@echo "  make fmt                - Format code"
	@echo "  make lint               - Lint code"
	@echo "  make check              - Quick compile check"
	@echo "  make build              - Build release"
	@echo "  make clean              - Clean build artifacts"

# Run all tests (including ignored tests)
test:
	cargo nextest run --run-ignored all

# Run tests with coverage (including ignored tests)
tests:
	cargo llvm-cov nextest --run-ignored all

# Run ignored longitudinal tests
longitudinal-tests:
	cargo test --test longitudinal -- --ignored --nocapture

# Format code
fmt:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Quick compile check (faster than full build)
check:
	cargo check

# Run all checks before commit
check-all: fmt lint test
	@echo "All checks passed!"

# Build release
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean
