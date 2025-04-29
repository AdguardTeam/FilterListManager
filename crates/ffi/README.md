# FFI for AdGuard FLM

This crate is a set of bindings over filter-list-manager and build
configurations for FFI bindings to interface with other programming languages.

FFI transport are implemented as [C language interface (outer side)][native_interface] using [protocol buffers][protobuf] for serialisation.
On [rust side (inner side)][rust_interface] there is a dispatcher that passes the FFI function call to foreign language.

## How to build

### Re-generate protobuf and headers
You may need regenerate protobuf files **for rust** and `flm_native_interface.h` header:\
`cargo run -p ffi-native-assets-generator`.\
It's better run this operation from the workspace root.

### Build library
`cargo run -p adguard-flm-ffi` from workspace root

### Platforms

[Apple Readme](./src/platforms/apple/README.md)\
[Windows Readme](./src/platforms/windows/README_WIN.md)

## FFI-specific symbols

Look at these symbols for better understanding FFI interface.

### Library facade
You can check rust facade [here][library_facade] for library functions signatures.

### Models

[Source](./src/models/mod.rs)

### Errors

`OuterError` - flattened enum from `adguard_flm::FLMError`.\
[Source](./src/outer_error.rs)

[protobuf]: https://protobuf.dev
[native_interface]: ./src/platforms/flm_native_interface.h
[rust_interface]: ./src/native_interface/mod.rs
[library_facade]: ./src/lib.rs
