# AdGuard Filter List Manager - Apple Swift Adapter

## Build and Development

After changing rust interface you should:

1. Regenerate protobuf
2. Build xcframework
3. Run `archive_framework` script

### Requirements

- `Rust` - [See how to install](https://www.rust-lang.org/tools/install)
- `cargo` comes with Rust, the current version is 1.85.
- `protoc` the current version is 29.3 - [See how to install](https://grpc.io/docs/protoc-installation/)

Make sure that all these tools are available in your `PATH` environment variable.

First run (from the repository root folder)

```sh
./crates/ffi/src/platforms/apple/Scripts/configure.sh
```

### Available scripts

`Scripts/configure` - configure rust before first build\
`Scripts/generate_proto` - regenerate Swift-protobuf counterpart\
`Script/build` - build .xcframework for Apple OSs\
`Script/archive_framework` - archive framework, puts it to build folder

### Build Rust part

Run from the repository root folder

```sh
./crates/ffi/src/platforms/apple/Scripts/build.sh
```

The result xcframework will be in `crates/ffi/src/platforms/apple/build/framework/AdGuardFLM.xcframework`.

### Generate Swift proto objects

Run from the repository root folder

```sh
./crates/ffi/src/platforms/apple/Scripts/generate_proto.sh
```

The result files will be in `crates/ffi/src/platforms/apple/AdGuardFLM/Sources/AdGuardFLMLib/GeneratedProto`.

### Archive framework

Run from the repository root folder

```sh
./crates/ffi/src/platforms/apple/Scripts/archive_framework.sh
```

The result xcframework will be in `crates/ffi/src/platforms/apple/build/framework/AdGuardFLM.xcframework`.

### Test your build

1. Run **Build** steps 
2. Open `crates/ffi/src/platforms/apple/AdguardFLM` as local package in XCode
3. Make `Clean Build Folder` in XCode
4. Run AdGuardFLMLibTests
5. Make sure that you have tested your changeset properly
