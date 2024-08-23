#!/bin/bash

set -e

rustup component add rustfmt clippy

# Test the crates
echo "Testing the crates..."
cargo clippy --all-targets --all-features
cargo test --lib --tests -- --test-threads=1
cargo fmt --all -- --check
