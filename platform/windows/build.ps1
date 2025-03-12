$ARCH_TO_FOLDER_MAP = @{
    "i686-pc-windows-msvc" = "x86"
    "x86_64-pc-windows-msvc" = "x64"
    "aarch64-pc-windows-msvc" = "arm64"
}

$FILES = @(
    "AdGuardFLM.dll",
    "AdGuardFLM.pdb"
)

Function CopyToTestsBuildFolder {
    Write-Output "Copying files...";
    foreach ($arch in $ARCH_TO_FOLDER_MAP.Keys) {
       foreach ($file in $FILES) {
        $folder = $ARCH_TO_FOLDER_MAP[$arch]
        $srcPath = "target\$arch\release\$file"
        $destPath = "platform\windows\build\bin\Release\$folder\$file"
        
        $destDir = Split-Path -Path $destPath -Parent
        if (!(Test-Path -Path $destDir)) {
            New-Item -ItemType Directory -Path $destDir -Force | Out-Null
        }
        
        Copy-Item -Path $srcPath -Destination $destPath
        if (!$?) {
            Write-Error "Failed to copy file from $srcPath to $destPath"
            exit 1
        }
        
        Write-Output "Copied: $srcPath to: $destPath"
       }
    }

    Write-Output "Copying files has been completed successfully";
}

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

Function ReplaceJsonVersion {
    param (
        [string]$Line,
        [string]$Pattern,
        [string]$PatternSubstitution
    )

    if ($Line -match $Pattern) {
        return $Line -replace $Pattern, "$PatternSubstitution"
    }

    return $Line
}

Function ReplaceRcVersion {
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

function SetAdapterVersion {
    
    $jsonFilePath = "platform\windows\AdGuard.FilterListManager\AdGuard.FilterListManager.schema.json";

    $jsonFileContent = Get-Content -Path $jsonFilePath -Raw
    $jsonFileJsonObject = ConvertFrom-Json -InputObject $jsonFileContent

    $jsonVersion = $jsonFileJsonObject.version.TrimEnd(".0");

    $csprojFilePath = "platform\windows\AdGuard.FilterListManager\AdGuard.FilterListManager.csproj";
    $csprojContent = Get-Content -Path $csprojFilePath;

    $updatedCsprojContent = $csprojContent | ForEach-Object {
        $_ = ReplaceJsonVersion -Line $_ -Pattern '^([\s|\t]*<Version>)(.+)(<\/Version>)' -PatternSubstitution "`${1}$jsonVersion`${3}"
        return $_;
    }

    Set-Content -Path $csprojFilePath -Value $updatedCsprojContent;
    Write-Host "Adapter version $jsonVersion is updated";
}

function SetNativeVersion {
    
    $versionFromToml = GetVersionFromToml -FilePath "crates\ffi\Cargo.toml";
    $rcVersion = $versionFromToml -replace '\.', ',' -replace '$', ',0';

    $rcFilePath = "crates\ffi\resources\AGWinFLM.rc";
    $rcContent = Get-Content -Path $rcFilePath;

    $updatedRcContent = $rcContent | ForEach-Object {
        $_ = ReplaceRcVersion -Line $_ -Pattern '^(.*FILEVERSION\s+)\d+,\d+,\d+,\d+' -NewVersion $rcVersion
        $_ = ReplaceRcVersion -Line $_ -Pattern '^(.*PRODUCTVERSION\s+)\d+,\d+,\d+,\d+' -NewVersion $rcVersion
        $_ = ReplaceRcVersion -Line $_ -Pattern '(.*"FileVersion",\s*")(\d+.\d+.\d+)' -NewVersion $versionFromToml
        $_ = ReplaceRcVersion -Line $_ -Pattern '(.*"ProductVersion",\s*")(\d+.\d+.\d+)' -NewVersion $versionFromToml
        return $_;
    }

    Set-Content -Path $rcFilePath -Value $updatedRcContent;
    Write-Host "Native version $rcVersion is updated";
}

Function RustBuild {
    $rust_version = (& cargo -V);
    try {
        SetNativeVersion;
        SetAdapterVersion;
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

    CopyToTestsBuildFolder;

    Write-Output "Executing method RustBuild has been completed successfully";
}