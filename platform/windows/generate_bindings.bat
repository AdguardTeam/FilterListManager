@echo off

uniffi-bindgen-cs --config .\crates\ffi\uniffi.toml  --out-dir .\platform\windows\build\ .\crates\ffi\src\flm_ffi.udl 

xcopy .\platform\windows\build\flm_ffi.cs .\platform\windows\AdGuard.FilterListManager\flm_ffi.cs.txt /Y
