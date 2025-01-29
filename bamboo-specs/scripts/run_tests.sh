#!/bin/bash

set -e

rustup component add rustfmt
rustup component add clippy

# Test the crates
echo "Testing the crates..."
cargo fmt --all -- --check
cargo clippy
cargo test --workspace --lib --all-features -- --test-threads=1
