# FFI for AdGuard FLM

Foreign function interface crate for filter list manager library.

For generating FFI bindings, the [uniffi-rs][uniffi-rs] library is used.

[uniffi-rs]: https://github.com/mozilla/uniffi-rs

## Build

### Apple

```shell
rustup target add x86_64-apple-darwin aarch64-apple-darwin # for macOS universal framework
rustup target add x86_64-apple-ios aarch64-apple-ios-sim # for iPhone simulator universal framework
rustup target add aarch64-apple-ios # for iOS framework
```

#### Building Apple XCFramework

```shell
cd ${REPO_ROOT}
./platform/apple/build.sh
```

### Windows

tbd...

## Usage recommendations and motivation

This crate is a set of bindings over filter-list-manager and build
configurations for FFI bindings to interface with other programming languages.
Currently, the FFI wrapper uses a Mutex wrapper around FLM, keep this in mind.

For example:

1. You want to update filters with `update_filters` method.
2. Client has bad internet connection, and we have a lot of filters.
3. You run filters update process in another thread in your app.
4. The main thread wants to read something from FLM.
5. Main thread will be blocked until update process will be completed.

Specifically for this case, the solution will be to use the second parameter of
the `update_filters` method - `loose_timeout`.

## FFI-specific symbols

Look at these symbols for better understanding FFI interface.

### Functions

[Top Level Functions](./src/top_level.rs)

### Models

[Models](./src/models/mod.rs)

### Errors

`OuterError` - flattened enum from `adguard_flm::FLMError`.
[Outer Error](./src/outer_error.rs)
