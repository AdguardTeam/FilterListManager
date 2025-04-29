# AdGuard Filter List Manager - Windows C# Adapter

### Requirements

- `Rust` - [See how to install](https://www.rust-lang.org/tools/install)
- `cargo` comes with Rust, the current version is 1.85.
- `protoc` the current version is 30.1 - [See how to install](https://grpc.io/docs/protoc-installation/)
- `Visual studio build tools` - [See how to install](https://visualstudio.microsoft.com/ru/downloads/#build-tools-for-visual-studio-2022) 

Make sure that all these tools are available in your `PATH` environment variable.

First run (from the repository folder)

```cmd
rustup target add i686-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

### Build Rust part & C# bindings generation

Run from the repository folder

```powershell
. ./crates/ffi/src/platforms/windows/Scripts/build_adapter.ps1
RustBuild
```

If you only want to generate protobuf files you can run 

```powershell
./crates/ffi/src/platforms/windows/Scripts/generate_protobuf.ps1
```

The result files will be in `target\[x86_64-pc-windows-msvc|i686-pc-windows-msvc|aarch64-pc-windows-msvc]\release`.
Check the protobuf files in and correct them if needed.

### Build C# adapter

There are two solutions: 
- `AdGuard.FilterListManagerOpen.sln` - this solution can be build in any environment. Please, use this solution for all cases.
Examine `AdGuard.FilterListManager.SampleApp\AdGuard.FilterListManager.SampleApp.csproj` - you can check the FLM adapter using this project. Projects `AdGuard.FilterListManagerOpen.csproj` & `AdGuard.FilterListManager.SampleApp.csproj` in this solution use utils libraries from `/libs` folder. This allows to build the project without our internal CI services.

- `AdGuard.FilterListManager.sln` - the solution we in AdGuard use to assemble projects for our products. It contains some additional logic like signing and uses our internal CI services. This solution is not recommended for general use.

## Internal usage only

[See here](README_CI.md)
