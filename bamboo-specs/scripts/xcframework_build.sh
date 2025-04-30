#!/bin/bash

set -e

./crates/ffi/src/platforms/apple/Scripts/configure.sh

./crates/ffi/src/platforms/apple/Scripts/build.sh
