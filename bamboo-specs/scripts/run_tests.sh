#!/bin/bash

set -e

rustup component add rustfmt

# Test the crates
echo "Testing the crates..."
cargo test --workspace --lib --all-features -- --test-threads=1
cargo fmt --all -- --check
