#!/bin/bash
set -exf

./platform/apple/build.sh

SUFFIX="" # Always release build
VER="$(sed -ne 's/^ *version = "\(.*\)"/\1/p' Cargo.toml)"
VER="${VER}${SUFFIX}"

ARCHIVE_NAME="AdGuardFLM-${VER}.zip"

cd platform/apple/build/framework

find ../ -name "*.zip" -delete
zip -4yr ../"${ARCHIVE_NAME}" AdGuardFLM.xcframework

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
      path: "'${ARCHIVE_NAME}'"
    ),
  ]
)
' > ../Package.swift
