# CI AdGuard FLM - windows C# adapter

Unit tests are in `AdGuard.FilterListManager.Test\AdGuard.FilterListManager.Test.csproj`
Examine `windows_adapter.yaml` file to see how we build it on Bamboo.

### Nuget

In `crates\ffi\src\platforms\windows\AdGuard.FilterListManager`:

The spec file `AdGuard.FilterListManager.nuspec` is being used for deploying on Bamboo agents
The scema file `AdGuard.FilterListManager.schema.json` should be incremented each release we want to deploy.

### How to Release a New Version

Versions should be deployed from the master branch.

1. If there are any breaking changes that require a pull request, you should create it and re-generate the C# bindings. See the section [Build C# Adapter](README_WIN.md#build-c-adapter).
2. Otherwise, the Bamboo plan "Build Filter List Manager Windows" can create an actual NuGet package for you. Refer to the `bamboo-specs\windows_adapter.yaml` file.
3. With each build on the master branch, the adapter schema in `AdGuard.FilterListManager.schema.json` should be incremented.
4. After the plan has finished, you can obtain a new version of the `Adguard.FilterListManager` NuGet package in the local Artifactory store.
5. Note that the version in the actual FLM Rust .dll file is specified in the `crates\ffi\resources\AGWinFLM.rc` file and can be modified in `Scripts\build_adapter.ps1`; the default value comes from `crates\ffi\Cargo.toml`.
6. The version of the adapter .dll file specified in `AdGuard.FilterListManager.csproj` in the `<Version>` section should be automatically updated based on the version from the `AdGuard.FilterListManager.schema.json` file. This is handled by the `crates\ffi\src\platforms\windows\Scripts\build_adapter.ps1` script.

NOTE. If a signature error occurs during assembly you can use [this solution](https://www.notion.so/adguard/sn-Vr-7f55f6d2080546c1a3fd69d509e926a2) or just remove signing from [cs proj](AdGuard.FilterListManager/AdGuard.FilterListManager.csproj#39) only for test.
