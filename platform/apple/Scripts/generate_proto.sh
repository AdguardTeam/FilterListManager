#!/bin/bash

set -exf

PROTO_GENERATED_PATH="platform/apple/AdGuardFLM/Sources/AdGuardFLMLib/GeneratedProto"

mkdir -p $PROTO_GENERATED_PATH

protoc \
    --swift_opt=Visibility=public \
    --swift_out=$PROTO_GENERATED_PATH \
    -I `find crates/ffi/src/protobuf`
