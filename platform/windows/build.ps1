Function RenameOutFile {
    param([string]$profile_name)
    Move-Item -Path "target\$profile_name\release\filter_list_manager_ffi.dll" -Destination "target\$profile_name\release\AdGuardFLM.dll" -Force
    Move-Item -Path "target\$profile_name\release\filter_list_manager_ffi.pdb" -Destination "target\$profile_name\release\AdGuardFLM.pdb" -Force
}

Function GetVersionFromToml {
    param (
        [string]$FilePath
    )

    $content = Get-Content -Path $FilePath
    foreach ($line in $content) {
        if ($line -match '^\s*version\s*=\s*"(.*?)"\s*$') {
            return $matches[1]
        }
    }

    throw "Version not found in $FilePath"
}

Function ReplaceVersion {
    param (
        [string]$Line,
        [string]$Pattern,
        [string]$NewVersion
    )

    if ($Line -match $Pattern) {
        return $Line -replace $Pattern, "`${1}$NewVersion"
    }

    return $Line
}

function SetVersion {
    
    $versionFromToml = GetVersionFromToml -FilePath "crates\ffi\Cargo.toml";
    $rcVersion = $versionFromToml -replace '\.', ',' -replace '$', ',0';

    $rcFilePath = "crates\ffi\resources\AGWinFLM.rc";
    $rcContent = Get-Content -Path $rcFilePath;

    $updatedRcContent = $rcContent | ForEach-Object {
        $_ = ReplaceVersion -Line $_ -Pattern '^(.*FILEVERSION\s+)\d+,\d+,\d+,\d+' -NewVersion $rcVersion
        $_ = ReplaceVersion -Line $_ -Pattern '^(.*PRODUCTVERSION\s+)\d+,\d+,\d+,\d+' -NewVersion $rcVersion
        $_ = ReplaceVersion -Line $_ -Pattern '(.*"FileVersion",\s*")(\d+.\d+.\d+)' -NewVersion $versionFromToml
        $_ = ReplaceVersion -Line $_ -Pattern '(.*"ProductVersion",\s*")(\d+.\d+.\d+)' -NewVersion $versionFromToml
        return $_;
    }
    Set-Content -Path $rcFilePath -Value $updatedRcContent;

    Write-Host "Version $rcVersion is updated";
}

Function RustBuild {
    $rust_version = (& cargo -V);
    if (!$rust_version.StartsWith("cargo 1.75")) {
        Write-Output "Only Rust 1.75 supported! Versions 1.76+ don't support Windows 7. Current version is $rust_version";
        exit 1;
    }

    try {
        SetVersion;
    }
    catch {
        Write-Host $_
        exit 1;
    }

    $env:RUSTFLAGS = "-Ctarget-feature=+crt-static";

    Write-Output "Start executing method RustBuild";
    & cargo build --release --lib --package adguard-flm-ffi --target i686-pc-windows-msvc --features rusqlite-bundled
    RenameOutFile "i686-pc-windows-msvc"
    & cargo build --release --lib --package adguard-flm-ffi --target x86_64-pc-windows-msvc --features rusqlite-bundled
    RenameOutFile "x86_64-pc-windows-msvc"
    & cargo build --release --lib --package adguard-flm-ffi --target aarch64-pc-windows-msvc --features rusqlite-bundled
    RenameOutFile "aarch64-pc-windows-msvc"
    Write-Output "Executing method RustBuild has been completed successfully";
}