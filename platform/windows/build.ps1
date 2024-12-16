Function RenameOutFile {
    param([string]$profile_name)
    Move-Item -Path "target\$profile_name\release\filter_list_manager_ffi.dll" -Destination "target\$profile_name\release\AdGuardFLM.dll" -Force
    Move-Item -Path "target\$profile_name\release\filter_list_manager_ffi.pdb" -Destination "target\$profile_name\release\AdGuardFLM.pdb" -Force
}


Function RustBuild {
    Write-Output "Start executing method RustBuild";
    & cargo build --release --lib --package adguard-flm-ffi --target i686-pc-windows-msvc --features rusqlite-bundled
    RenameOutFile "i686-pc-windows-msvc"
    & cargo build --release --lib --package adguard-flm-ffi --target x86_64-pc-windows-msvc --features rusqlite-bundled
    RenameOutFile "x86_64-pc-windows-msvc"
    & cargo build --release --lib --package adguard-flm-ffi --target aarch64-pc-windows-msvc --features rusqlite-bundled
    RenameOutFile "aarch64-pc-windows-msvc"
    Write-Output "Executing method RustBuild has been completed successfully";
}