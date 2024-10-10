#!/bin/bash

set -e

arch -arm64 ./platform/apple/configure.sh

arch -arm64 ./platform/apple/build.sh
