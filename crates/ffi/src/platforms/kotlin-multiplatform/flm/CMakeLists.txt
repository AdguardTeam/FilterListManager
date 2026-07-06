# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html.
# For more examples on how to use CMake, see https://github.com/android/ndk-samples.

# Sets the minimum CMake version required for this project.
cmake_minimum_required(VERSION 3.22.1)

# Include ExternalProject module for building Rust
include(ExternalProject)

# Declares the project name. The project name can be accessed via ${ PROJECT_NAME},
# Since this is the top level CMakeLists.txt, the project name is also accessible
# with ${CMAKE_PROJECT_NAME} (both CMake variables are in-sync within the top level
# build script scope).
project("filter_list_manager_jni")

# Set paths for the Rust library
set(RUST_CRATE_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../../../..)

# Define the Rust library output path
set(RUST_LIB_NAME libfilter_list_manager_ffi.so)
set(EXTERNAL_JNI_LIBS_DIR ${CMAKE_CURRENT_SOURCE_DIR}/src/androidMain/jniLibs)

# Set build flags based on CMAKE_BUILD_TYPE
set(CARGO_BUILD_FLAGS --no-default-features --features=rusqlite-bundled,rustls-tls)

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    list(APPEND CARGO_BUILD_FLAGS --release)
elseif(CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo")
    list(APPEND CARGO_BUILD_FLAGS --release)
    # Add debug info equivalent to debug=true in Cargo.toml
    set(CARGO_BUILD_ENV RUSTFLAGS=-Cdebuginfo=2)
endif()

# Add external project for building the Rust library
ExternalProject_Add(
        rust_filter_list_manager_ffi
        SOURCE_DIR "${RUST_CRATE_ROOT}"
        CONFIGURE_COMMAND ""
        BUILD_COMMAND ${CMAKE_COMMAND} -E env ${CARGO_BUILD_ENV} cargo ndk -t ${ANDROID_ABI} -o "${EXTERNAL_JNI_LIBS_DIR}" build -p adguard-flm-ffi ${CARGO_BUILD_FLAGS}
        BUILD_IN_SOURCE TRUE
        INSTALL_COMMAND ""
        BUILD_BYPRODUCTS "${EXTERNAL_JNI_LIBS_DIR}/${ANDROID_ABI}/${RUST_LIB_NAME}"
)

# Add imported target
add_library(filter_list_manager_ffi SHARED IMPORTED)
set_target_properties(filter_list_manager_ffi PROPERTIES
        IMPORTED_LOCATION "${EXTERNAL_JNI_LIBS_DIR}/${ANDROID_ABI}/${RUST_LIB_NAME}"
        IMPORTED_NO_SONAME TRUE # Prevents RPATH/SONAME logic
)
target_link_directories(filter_list_manager_ffi INTERFACE "${EXTERNAL_JNI_LIBS_DIR}/${ANDROID_ABI}")

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.
add_library(${CMAKE_PROJECT_NAME} SHARED
        # List C/C++ source files with relative paths to this CMakeLists.txt.
        src/androidMain/cpp/filter_list_manager_jni.cpp)

# Include directories for the JNI library
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE
        ${RUST_CRATE_ROOT}/src/platforms
)

# Link against the imported Rust library
target_link_libraries(${CMAKE_PROJECT_NAME} filter_list_manager_ffi)
