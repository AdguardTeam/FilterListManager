#!/usr/bin/env bash

# Stage three deploy-ready Apple payloads for Artifact Keeper from the
# already-built AdGuardFLM xcframework. The directory layout under
# <stage-output-dir> matches the remote path each deploy workflow publishes:
#   apple-xcframework : adguard-flm/<version>/AdGuardFLM-<version>.zip  (generic -> apple-bin)
#                       adguard-flm/<version>/AdGuardFLM-pod-<version>.zip
#   apple-podspec     : AdGuardFLM/<version>/AdGuardFLM.podspec.json    (generic -> pods)
#   apple-swiftpm     : adguard/flm/<version>.zip                       (swift   -> swift-pkgs)

set -euo pipefail

if [[ $# -ne 4 ]]; then
  echo "Usage: $0 <version> <xcframework-zip> <ak-base-url> <stage-output-dir>" >&2
  exit 1
fi

VERSION="$1"
XCF_ZIP="$2"
AK_BASE="$3"
STAGE="$4"

# An empty value would still produce a podspec and a Package.swift, only
# with broken paths or URLs such as "/api/v1/...", so fail here instead.
for arg in VERSION XCF_ZIP AK_BASE STAGE; do
  if [[ -z "${!arg}" ]]; then
    echo "Error: ${arg} is empty" >&2
    exit 1
  fi
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APPLE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

XCF_DIR="$STAGE/apple-xcframework/adguard-flm/${VERSION}"
POD_DIR="$STAGE/apple-podspec/AdGuardFLM/${VERSION}"
SPM_DIR="$STAGE/apple-swiftpm/adguard/flm"
mkdir -p "$XCF_DIR" "$POD_DIR" "$SPM_DIR"

XCF_FILENAME="AdGuardFLM-${VERSION}.zip"

cp "$XCF_ZIP" "$XCF_DIR/$XCF_FILENAME"

CHECKSUM="$(shasum -a 256 "$XCF_DIR/$XCF_FILENAME" | awk '{print $1}')"

XCF_URL="${AK_BASE}/api/v1/repositories/apple-bin-virtual/download/adguard-flm/${VERSION}/${XCF_FILENAME}"

# The pod needs the Swift wrapper next to the binary: unlike core-libs, FLM's
# API lives in AdGuardFLMLib, not in the framework. Same contents as the
# Bamboo pod archive: Sources, the xcframework, CHANGELOG and LICENSE.
POD_FILENAME="AdGuardFLM-pod-${VERSION}.zip"
POD_URL="${AK_BASE}/api/v1/repositories/apple-bin-virtual/download/adguard-flm/${VERSION}/${POD_FILENAME}"
POD_TMP="$(mktemp -d)"
cp -r "$APPLE_DIR/AdGuardFLM/Sources/AdGuardFLMLib" "$POD_TMP/Sources"
(cd "$POD_TMP" && unzip -q "$XCF_DIR/$XCF_FILENAME")
echo "#${VERSION}" > "$POD_TMP/CHANGELOG"
echo "Confidential. Property of Adguard Software Ltd. https://adguard.com" > "$POD_TMP/LICENSE"
(cd "$POD_TMP" && zip -4yr "$XCF_DIR/$POD_FILENAME" Sources AdGuardFLM.xcframework CHANGELOG LICENSE >/dev/null)
rm -rf "$POD_TMP"

cat > "$POD_DIR/AdGuardFLM.podspec.json" <<EOF
{
  "name": "AdGuardFLM",
  "version": "${VERSION}",
  "summary": "AdGuard Filter List Manager",
  "description": "AdGuard Filter List Manager library",
  "homepage": "https://github.com/AdguardTeam/FilterListManager",
  "license": {
    "type": "proprietary",
    "file": "LICENSE"
  },
  "authors": {
    "Adguard Software Ltd": "devteam@adguard.com"
  },
  "platforms": {
    "osx": "10.15",
    "ios": "11.2"
  },
  "source": {
    "http": "${POD_URL}"
  },
  "source_files": "Sources/**/*",
  "preserve_paths": ["AdGuardFLM.xcframework"],
  "vendored_frameworks": "AdGuardFLM.xcframework",
  "xcconfig": {
    "LD_RUNPATH_SEARCH_PATHS": "@loader_path/../Frameworks"
  },
  "requires_arc": true,
  "dependencies": {
    "SwiftProtobuf": ["~> 1.0"]
  }
}
EOF

SWIFT_PROTOBUF_VERSION="1.28.2"
PKG_TMP="$(mktemp -d)"
mkdir -p "$PKG_TMP/AdGuardFLM"

cp -r "$APPLE_DIR/AdGuardFLM/Sources" "$PKG_TMP/AdGuardFLM/Sources"

cat > "$PKG_TMP/AdGuardFLM/Package.swift" <<EOF
// swift-tools-version: 5.4
import PackageDescription

let package = Package(
    name: "AdGuardFLM",
    platforms: [
        .iOS("11.2"), .macOS("10.15")
    ],
    products: [
        .library(name: "AdGuardFLMLib", targets: ["AdGuardFLMLib"])
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "${SWIFT_PROTOBUF_VERSION}")
    ],
    targets: [
        .target(
            name: "AdGuardFLMLib",
            dependencies: [
                .product(name: "SwiftProtobuf", package: "swift-protobuf"),
                .target(name: "AdGuardFLM")
            ]
        ),
        .binaryTarget(
            name: "AdGuardFLM",
            url: "${XCF_URL}",
            checksum: "${CHECKSUM}"
        ),
    ]
)
EOF

(cd "$PKG_TMP" && zip -4r "$SPM_DIR/${VERSION}.zip" AdGuardFLM >/dev/null)
rm -rf "$PKG_TMP"

echo "Staged Apple deploy payloads under ${STAGE}"
echo "  xcframework: ${XCF_DIR}/${XCF_FILENAME}"
echo "  pod archive: ${XCF_DIR}/${POD_FILENAME}"
echo "  podspec:     ${POD_DIR}/AdGuardFLM.podspec.json"
echo "  swiftpm:     ${SPM_DIR}/${VERSION}.zip"
