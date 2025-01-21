# AdGuard filter list manager - windows C# adapter

### Requirements

- `Rust` - [See how to install](https://www.rust-lang.org/tools/install)
- `cargo` 1.75. Versions 1.76+ don't support Windows 7. [See for more info](https://blog.rust-lang.org/2023/08/24/Rust-1.72.0.html#future-windows-compatibility)
- `uniffi-bindgen-cs` - [See how to install](https://github.com/NordSecurity/uniffi-bindgen-cs)

First run

```cmd
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.8.0+v0.25.0
rustup target add i686-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

### Build Rust part

Run from the repository folder

```cmd
cargo build --release --package adguard-flm-ffi --target i686-pc-windows-msvc --features rusqlite-bundled
cargo build --release --package adguard-flm-ffi --target x86_64-pc-windows-msvc --features rusqlite-bundled
cargo build --release --package adguard-flm-ffi --target aarch64-pc-windows-msvc --features rusqlite-bundled
```

The result files will be in `target\[x86_64-pc-windows-msvc|i686-pc-windows-msvc|aarch64-pc-windows-msvc]\release`.

### Build C# adapter

Run this script to generate bindings for the current version:

```cmd
platform\windows\generate_bindings.bat

```

It saves the result to `platform\windows\AdGuard.FilterListManager\flm_ffi.cs.txt`.
Git will show the differences with the previous version. Change the adapter code according to this. We cannot use the generated bindings directly because this tool doesn't support early versions of C# from .NET 4.5 that we use. Resharper or Rider can help with the code refactoring here.
When the refactoring is done, check the tests and the adapter is ready to pack&deploy.

Go to `platform\windows` and build `AdGuard.FilterListManager\AdGuard.FilterListManager.csproj`. Unit tests are in `AdGuard.FilterListManager.Test\AdGuard.FilterListManager.Test.csproj`

### Nuget

In `platform\windows\AdGuard.FilterListManager`:

The spec file `AdGuard.FilterListManager.nuspec` is being used for deploying on Bamboo agents
The scema file `AdGuard.FilterListManager.schema.json` should be incremented each release we want to deploy.

### How to Release a New Version

Versions should be deployed from the master branch.

1. If there are any breaking changes that require a pull request, you should create it and re-generate the C# bindings. See the section [Build C# Adapter](#build-c-adapter).
2. Otherwise, the Bamboo plan "Build Filter List Manager Windows" can create an actual NuGet package for you. Refer to the `bamboo-specs\windows_adapter.yaml` file.
3. With each build on the master branch, the adapter schema in `AdGuard.FilterListManager.schema.json` should be incremented.
4. After the plan has finished, you can obtain a new version of the `Adguard.FilterListManager` NuGet package in the local Artifactory store.
5. Note that the version in the actual FLM Rust .dll file is specified in the `crates\ffi\resources\AGWinFLM.rc` file and can be modified in `build.ps1`; the default value comes from `crates\ffi\Cargo.toml`.
6. The version of the adapter .dll file specified in `AdGuard.FilterListManager.csproj` in the `<Version>` section should be automatically updated based on the version from the `AdGuard.FilterListManager.schema.json` file. This is handled by the `platform\windows\build.ps1` script.

