@echo off

uniffi-bindgen-cs --config .\crates\ffi\uniffi.toml  --out-dir .\platform\windows\uniffi\build\ .\crates\ffi\src\flm_ffi.udl 

xcopy .\platform\windows\uniffi\build\flm_ffi.cs .\platform\windows\uniffi\AdGuard.FilterListManager\flm_ffi.cs.txt /Y
