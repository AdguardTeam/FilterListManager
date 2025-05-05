#!/bin/bash

set -exf

pushd crates/ffi/src
buf generate --template platforms/apple/buf.gen.yaml
popd
