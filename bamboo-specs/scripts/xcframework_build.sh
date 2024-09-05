#!/bin/bash

set -e

rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add aarch64-apple-ios

arch -arm64 ./platform/apple/build.sh
