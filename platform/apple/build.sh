#!/bin/bash
set -exf

BUILD_ROOT_PATH="platform/apple/build"
FWK_BUILD_ROOT="${BUILD_ROOT_PATH}/framework"

export CARGO_TARGET_DIR="${BUILD_ROOT_PATH}/target"

LIBRARY_ARTIFACT_NAME="libfilter_list_manager_ffi.dylib"

FRAMEWORK_NAME=AdGuardFLM.framework

build_framework() {
    local OUTPUT_DIR="$1"
    local ARCH="$2"
    local SWIFT_SDK="$3"
    local SWIFT_TARGET="$4"
    export MACOSX_DEPLOYMENT_TARGET="$5"
    export IPHONEOS_DEPLOYMENT_TARGET="$5"
    local OPTION="$6"
    local FRAMEWORK_NAME=AdGuardFLM.framework
    local INFO_PLIST=Info.plist.ios
    case $OPTION in
    macos)
    local VERSION_SUFFIX=/Versions/A
    INFO_PLIST=Info.plist.macos
    ;;
    simulator)
    local TARGET_SUFFIX=-simulator
    ;;
    *)
    ;;
    esac

    cargo run -p uniffi-bindgen generate \
      crates/ffi/src/flm_ffi.udl \
      -l swift \
      -o "${CARGO_TARGET_DIR}/${ARCH}/release" \
      --no-format

    FRAMEWORK_PATH=${OUTPUT_DIR}/${FRAMEWORK_NAME}
    FRAMEWORK_VERSIONED_PATH=${OUTPUT_DIR}/${FRAMEWORK_NAME}${VERSION_SUFFIX}

    mkdir -p ${FRAMEWORK_PATH}
    mkdir -p ${FRAMEWORK_VERSIONED_PATH}/Modules/AdguardFLM.swiftmodule
    mkdir -p ${FRAMEWORK_VERSIONED_PATH}/Headers
    mkdir -p ${FRAMEWORK_VERSIONED_PATH}/Resources
    if [ -n "${VERSION_SUFFIX}" ]; then
      ln -shf A ${FRAMEWORK_PATH}/Versions/Current
      ln -shf Versions/Current/Modules ${FRAMEWORK_PATH}/Modules
      ln -shf Versions/Current/Headers ${FRAMEWORK_PATH}/Headers
      ln -shf Versions/Current/Resources ${FRAMEWORK_PATH}/Resources
      ln -shf Versions/Current/AdGuardFLM ${FRAMEWORK_PATH}/AdGuardFLM
    fi
    cp "${CARGO_TARGET_DIR}/${ARCH}/release/AdguardFLMFFI.h" ${FRAMEWORK_PATH}/Headers
    cp ${BUILD_ROOT_PATH}/../module.modulemap ${FRAMEWORK_PATH}/Modules
    cp ${BUILD_ROOT_PATH}/../AdGuardFLM.h ${FRAMEWORK_PATH}/Headers
    cp ${BUILD_ROOT_PATH}/../${INFO_PLIST} ${FRAMEWORK_PATH}/Resources/Info.plist

    mkdir -p "${CARGO_TARGET_DIR}/${ARCH}/release/deps"
    xcrun -sdk $SWIFT_SDK swiftc -target ${SWIFT_TARGET}${MACOSX_DEPLOYMENT_TARGET}${TARGET_SUFFIX} -emit-library -static -module-name AdGuardFLM \
        -emit-module-path ${FRAMEWORK_PATH}/Modules/AdguardFLM.swiftmodule/${SWIFT_TARGET}${TARGET_SUFFIX}.swiftmodule \
        -emit-module-interface-path ${FRAMEWORK_PATH}/Modules/AdguardFLM.swiftmodule/${SWIFT_TARGET}${TARGET_SUFFIX}.swiftinterface \
        -enable-library-evolution \
        -import-underlying-module \
        ${CARGO_TARGET_DIR}/${ARCH}/release/FilterListManager.swift \
        -Xcc -fmodule-map-file="${FRAMEWORK_PATH}/Modules/module.modulemap" \
        -I "${CARGO_TARGET_DIR}/${ARCH}/release" \
        -no-verify-emitted-module-interface \
        -o "${CARGO_TARGET_DIR}/${ARCH}/release/deps/libAdGuardFLM.a"
    nm -gU "${CARGO_TARGET_DIR}/${ARCH}/release/deps/libAdGuardFLM.a" | awk '/ T / {print $3}' > "${CARGO_TARGET_DIR}/${ARCH}/release"/deps/libAdGuardFLM.syms

    export SWIFT_LIB_DIR=$(xcrun -sdk $SWIFT_SDK swiftc -target ${SWIFT_TARGET}${MACOSX_DEPLOYMENT_TARGET}${TARGET_SUFFIX} -print-target-info | jq -r '.paths.runtimeLibraryImportPaths[0]')
    RUSTFLAGS="--cfg swift_lib_dir=\"$SWIFT_LIB_DIR\"" cargo build --release --package adguard-flm-ffi --target $ARCH

    cp "${CARGO_TARGET_DIR}/${ARCH}/release/${LIBRARY_ARTIFACT_NAME}" ${FRAMEWORK_VERSIONED_PATH}/AdGuardFLM
    install_name_tool -id @rpath/${FRAMEWORK_NAME}${VERSION_SUFFIX}/AdGuardFLM ${FRAMEWORK_VERSIONED_PATH}/AdGuardFLM
}

merge_framework() {
    local OUTPUT_DIR="$1"
    shift
    local OPTION="$1"
    shift
    local DIRS=("$@")
    local FRAMEWORK_DIRS=()
    mkdir -p "$OUTPUT_DIR"

    case ${OPTION} in
    macos)
    local VERSION_SUFFIX=Versions/A
    ;;
    esac

    for dir in "${DIRS[@]}"; do
    FRAMEWORK_DIRS+=("$dir"/AdGuardFLM.framework/${VERSION_SUFFIX}/AdGuardFLM)
    cp -Rf "$dir"/AdGuardFLM.framework "$OUTPUT_DIR"/
    done
    lipo -create "${FRAMEWORK_DIRS[@]}" -output "${OUTPUT_DIR}/AdGuardFLM.framework/${VERSION_SUFFIX}/AdGuardFLM"
}

make_dsym() {
    dsymutil $1 -o $2
    strip -S $1
}

# Build desktop framework
build_framework ${FWK_BUILD_ROOT}/x86_64-macos x86_64-apple-darwin macosx x86_64-apple-macos 10.15 macos
build_framework ${FWK_BUILD_ROOT}/arm64-macos aarch64-apple-darwin macosx arm64-apple-macos 11.0 macos
merge_framework ${FWK_BUILD_ROOT}/universal-macos macos ${FWK_BUILD_ROOT}/x86_64-macos ${FWK_BUILD_ROOT}/arm64-macos

# Build iPhone simulator framework
build_framework ${FWK_BUILD_ROOT}/x86_64-ios-sim x86_64-apple-ios iphonesimulator x86_64-apple-ios 13.0 simulator
build_framework ${FWK_BUILD_ROOT}/arm64-ios-sim aarch64-apple-ios-sim iphonesimulator arm64-apple-ios 13.0 simulator
merge_framework ${FWK_BUILD_ROOT}/universal-ios-sim simulator ${FWK_BUILD_ROOT}/x86_64-ios-sim ${FWK_BUILD_ROOT}/arm64-ios-sim

# Build iPhone framework
build_framework ${FWK_BUILD_ROOT}/arm64-ios aarch64-apple-ios iphoneos arm64-apple-ios 13.0 ios

# Make .dSYMs
make_dsym ${FWK_BUILD_ROOT}/universal-macos/AdGuardFLM.framework/AdGuardFLM ${FWK_BUILD_ROOT}/universal-macos/AdGuardFLM.framework.dSYM
make_dsym ${FWK_BUILD_ROOT}/universal-ios-sim/AdGuardFLM.framework/AdGuardFLM ${FWK_BUILD_ROOT}/universal-ios-sim/AdGuardFLM.framework.dSYM
make_dsym ${FWK_BUILD_ROOT}/arm64-ios/AdGuardFLM.framework/AdGuardFLM ${FWK_BUILD_ROOT}/arm64-ios/AdGuardFLM.framework.dSYM

# Make xcframework
rm -rf ${FWK_BUILD_ROOT}/AdGuardFLM.xcframework
xcodebuild -create-xcframework \
  -framework ${FWK_BUILD_ROOT}/universal-macos/AdGuardFLM.framework \
  -debug-symbols $(readlink -f ${FWK_BUILD_ROOT})/universal-macos/AdGuardFLM.framework.dSYM \
  -framework ${FWK_BUILD_ROOT}/universal-ios-sim/AdGuardFLM.framework \
  -debug-symbols $(readlink -f ${FWK_BUILD_ROOT})/universal-ios-sim/AdGuardFLM.framework.dSYM \
  -framework ${FWK_BUILD_ROOT}/arm64-ios/AdGuardFLM.framework \
  -debug-symbols $(readlink -f ${FWK_BUILD_ROOT})/arm64-ios/AdGuardFLM.framework.dSYM \
  -output ${FWK_BUILD_ROOT}/AdGuardFLM.xcframework
