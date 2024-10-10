#!/bin/bash
set -exf

./platform/apple/configure.sh
./platform/apple/build.sh

SUFFIX="" # Always release build
VER="$(sed -ne 's/^ *version = \"\(.*\)\"/\1/p' crates/ffi/Cargo.toml)"
VER="${VER}${SUFFIX}"

ARCH_NAME="AdGuardFLM-${VER}.zip"

cd platform/apple/build/framework

zip -4yr "../${ARCH_NAME}" AdGuardFLM.xcframework

echo '// swift-tools-version:5.3
import PackageDescription

let package = Package(
  name: "AdGuardFLM",
  platforms: [
    .iOS("11.2"), .macOS("10.15")
  ],
  products: [
    .library(name: "AdGuardFLM", targets: ["AdGuardFLM"]),
  ],
  targets: [
    .binaryTarget(
      name: "AdGuardFLM",
      path: "'${ARCH_NAME}'"
    ),
  ]
)
' > ../Package.swift
