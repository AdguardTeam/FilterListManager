# How to run tests with coverage

## Info

Will use [grcov](https://github.com/mozilla/grcov)

## Prerequisites

```shell
# Install llvm-tools
rustup component add llvm-tools
cargo install grcov
```

## Usage

```shell
# Remove old data
find . -iname '*.profraw' -delete && (rm -r ./target/debug/coverage 2&>1 || true)

# Export ENVS
export CARGO_INCREMENTAL=0 # Might not be necessary.
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"

# ... run_tests ...
# After tests *.profraw files will be generated

# -s flag will looking for .profraw files
grcov ./crates/filter-list-manager  --binary-path ./target/debug -t html -s .  -o ./target/debug/coverage

open ./target/debug/coverage/index.html
```
