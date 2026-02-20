# FFI for AdGuard FLM

This crate provides FFI bindings over filter-list-manager and build
configurations to interface with other programming languages.

FFI transport is implemented as a [C language interface (outer side)][native_interface] using [protocol buffers][protobuf] for serialization.
On the [Rust side (inner side)][rust_interface] there is a dispatcher that handles FFI function calls from foreign languages.

## How to build

### Re-generate protobuf and headers
You may need to regenerate protobuf files **for Rust** and `flm_native_interface.h` header:\
`cargo run -p ffi-native-assets-generator`.\
It's better to run this operation from the workspace root.

### Build library
`cargo build -p adguard-flm-ffi` from workspace root

### Platforms

[Apple Readme](./src/platforms/apple/README.md)\
[Windows Readme](./src/platforms/windows/README_WIN.md)\
[Android Readme](./src/platforms/android/README.md)

## FFI-specific symbols

### Native interface
Look at these symbols for a better understanding of the FFI interface:\
[Native interface][native_interface]

### Library facade
You can check the Rust facade [here][library_facade] for library function signatures.

### Models
[Source](./src/models/mod.rs)

### Errors
`OuterError` - flattened enum from `adguard_flm::FLMError`.\
[Source](./src/outer_error.rs)

## Methods explanation and examples

Check the documentation of [filter-list-manager][github_flm_core_readme] core crate for more information.

[protobuf]: https://protobuf.dev
[native_interface]: ./src/platforms/flm_native_interface.h
[rust_interface]: ./src/native_interface/mod.rs
[library_facade]: ./src/lib.rs
[github_flm_core_readme]: https://github.com/AdguardTeam/FilterListManager/blob/master/crates/filter-list-manager/README.md
