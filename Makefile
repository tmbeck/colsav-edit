.PHONY: all build check test fmt fmt-check clippy clean release install

# Default: full CI-style check
all: fmt-check clippy test

# Build (debug)
build:
	cargo build --workspace

# Type-check only (faster than build)
check:
	cargo check --workspace

# Run all tests
test:
	cargo test --workspace

# Run a single test by name (usage: make test-one T=round_trip_colony01)
test-one:
	cargo test --workspace -- $(T)

# Run only library tests
test-lib:
	cargo test -p colonization-sav

# Run only binary tests
test-bin:
	cargo test -p colsav

# Format all code
fmt:
	cargo fmt --all

# Check formatting (CI mode, no changes)
fmt-check:
	cargo fmt --all -- --check

# Lint with clippy (pedantic, workspace-level config)
clippy:
	cargo clippy --workspace

# Lint with clippy, deny warnings (strict CI mode)
clippy-strict:
	cargo clippy --workspace -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Build optimized release binary
release:
	cargo build --release -p colsav

# Install the colsav binary to ~/.cargo/bin
install:
	cargo install --path colsav

# Quick validation: syntax-check all Rust files
syntax:
	cargo check --workspace 2>&1

# Run the TUI with a test save file
tui:
	cargo run -p colsav -- tui -f saves/COLONY01.SAV

# Dump info from a test save
info:
	cargo run -p colsav -- info -f saves/COLONY01.SAV
