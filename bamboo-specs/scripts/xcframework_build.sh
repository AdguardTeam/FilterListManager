#!/bin/bash

set -e

arch -arm64 ./platform/apple/Scripts/configure.sh

arch -arm64 ./platform/apple/Scripts/build.sh
