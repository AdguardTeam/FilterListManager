# AdGuard filter list manager (protobuf version) - windows C# adapter

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
cargo build --release --package adguard-flm-ffi --lib --target i686-pc-windows-msvc --features rusqlite-bundled
cargo build --release --package adguard-flm-ffi --lib --target x86_64-pc-windows-msvc --features rusqlite-bundled
cargo build --release --package adguard-flm-ffi --lib --target aarch64-pc-windows-msvc --features rusqlite-bundled
```

The result files will be in `target\[x86_64-pc-windows-msvc|i686-pc-windows-msvc|aarch64-pc-windows-msvc]\release`.

### Build C# adapter

Go to `crates\ffi\src\platforms\windows` and build `AdGuard.FilterListManagerProtobuf\AdGuard.FilterListManagerProtobuf.csproj`. 

### Sample application 
Sample application is located in `AdGuard.FilterListManagerProtobuf.SampleAppapp`. This is simplest console application which invokes all the methods from `FFI method` one by one, having only purpose be sure that all is working without fails
