Function RenameOutFile {
    param([string]$profile_name)
    Rename-Item -Path "target\$profile_name\release\filter_list_manager_ffi.dll" -NewName "AdGuardFLM.dll"
    Rename-Item -Path "target\$profile_name\release\filter_list_manager_ffi.pdb" -NewName "AdGuardFLM.pdb"
}


Function RustBuild {

    $rust_version = (& cargo -V);
    if (!$rust_version.StartsWith("cargo 1.75")){
        Write-Output "Only Rust 1.75 supported! Versions 1.76+ don't support Windows 7. Current version is $rust_version";
        exit 1;
    }

    Write-Output "Start executing method RustBuild";
    & cargo build --release --package adguard-flm-ffi --target i686-pc-windows-msvc
    RenameOutFile "i686-pc-windows-msvc"
    & cargo build --release --package adguard-flm-ffi --target x86_64-pc-windows-msvc
    RenameOutFile "x86_64-pc-windows-msvc"
    & cargo build --release --package adguard-flm-ffi --target aarch64-pc-windows-msvc 
    RenameOutFile "aarch64-pc-windows-msvc"
    Write-Output "Executing method RustBuild has been completed successfully";
}