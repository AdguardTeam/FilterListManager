#!/bin/bash

set -exf

REQUIRED="1.29"
CURRENT=$(protoc-gen-swift --version | awk '{print $2}')

echo "protoc-gen-swift detected: $CURRENT"
echo "Required minimum version: $REQUIRED"

# compare semantic versions
ver_lt() { [ "$1" = "$(printf '%s\n' "$1" "$2" | sort -V | head -n1)" ] && [ "$1" != "$2" ]; }

if ver_lt "$CURRENT" "$REQUIRED"; then
    echo "protoc $CURRENT is lower than required $REQUIRED – aborting generation"
    exit 1
fi

echo "protoc version is acceptable – continuing"

PROTO_GENERATED_PATH="crates/ffi/src/platforms/apple/AdGuardFLM/Sources/AdGuardFLMLib/GeneratedProto"

mkdir -p $PROTO_GENERATED_PATH

protoc \
    --swift_opt=Visibility=public \
    --swift_out=$PROTO_GENERATED_PATH \
    -I `find crates/ffi/src/protobuf`
