# AdGuard filter list manager - windows C# adapter

### Requirements

- `Rust` - [See how to install](https://www.rust-lang.org/tools/install)
- `cargo` 1.75. Versions 1.76+ don't support Windows 7. [See for more info](https://blog.rust-lang.org/2023/08/24/Rust-1.72.0.html#future-windows-compatibility)
- `uniffi-bindgen-cs` - [See how to install](https://github.com/NordSecurity/uniffi-bindgen-cs)

First run

```cmd
rustup target add i686-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

### Build Rust part

Run from the repository folder

```cmd
cargo build --release --package adguard-flm-ffi --target i686-pc-windows-msvc  
cargo build --release --package adguard-flm-ffi --target x86_64-pc-windows-msvc  
cargo build --release --package adguard-flm-ffi --target aarch64-pc-windows-msvc 
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
