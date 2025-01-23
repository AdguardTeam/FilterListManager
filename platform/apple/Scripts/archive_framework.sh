#!/bin/bash

set -exf

BUILD_FRAMEWORK_FOLDER_PATH="platform/apple/build/framework"

zip -r "${BUILD_FRAMEWORK_FOLDER_PATH}/AdGuardFLM.xcframework.zip" "${BUILD_FRAMEWORK_FOLDER_PATH}/AdGuardFLM.xcframework"
