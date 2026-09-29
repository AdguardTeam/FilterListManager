# CI AdGuard FLM - windows C# adapter

Unit tests are in `AdGuard.FilterListManager.Test\AdGuard.FilterListManager.Test.csproj`
Examine `.github/workflows/build-windows.yml` to see how CI builds it.

### Nuget

In `crates\ffi\src\platforms\windows\AdGuard.FilterListManager`:

The spec file `AdGuard.FilterListManager.nuspec` is used by CI to pack the NuGet package.

### How to Release a New Version

Versions should be deployed from the master branch.

1. If there are any breaking changes that require a pull request, you should create it and re-generate the C# bindings. See the section [Build C# Adapter](README_WIN.md#build-c-adapter).
2. Otherwise, the `Publish release` workflow (`.github/workflows/publish-release.yml`) builds and signs the NuGet package in its `build-windows` job and pushes it in `windows-deploy`, once every adapter of the release has been built.
3. After the workflow has finished, you can obtain a new version of the `Adguard.FilterListManager` NuGet package in the local Artifactory store.
4. All versions are resolved from the same source: the `FLM_VERSION` environment variable, or, if unset, `git describe --tags --match='v*' --abbrev=0` (a leading `v` is stripped). Set `FLM_VERSION` in the build environment to override; otherwise tag the release commit `vX.Y.Z`.
   - The native FLM Rust `.dll` version (`AGWinFLM.rc`) is generated from `AGWinFLM.rc.in` by the ffi crate's `build.rs` into `OUT_DIR` (no file is patched in place).
   - The C# adapter package `<Version>` is resolved at build time by `AdGuard.FilterListManager\Directory.Build.targets` (the `.csproj` files carry no `<Version>`; nothing is patched in place).
5. The FFI crate version (`adguard-flm-ffi`) is written to `AdGuard.FilterListManager\metadata.json` at build time by `Scripts\build_adapter.ps1` (`SetMetadataVersion` / `ResolveVersion`). This file is packed into the NuGet package so consumers can read the build version.

NOTE. If a signature error occurs during assembly you can use [this solution](https://www.notion.so/adguard/sn-Vr-7f55f6d2080546c1a3fd69d509e926a2) or just remove signing from [cs proj](AdGuard.FilterListManager/AdGuard.FilterListManager.csproj#39) only for test.
