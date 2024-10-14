$protosDir = Join-Path $PSScriptRoot "..\..\..\protobuf" | Resolve-Path
$outDir = Join-Path $PSScriptRoot "..\AdGuard.FilterListManagerProtobuf\AdGuard.FilterListManagerProtobuf\ProtobufGenerated" | Resolve-Path

protoc --csharp_out="$outDir" -I="$protosDir" filters.proto flm_interface.proto misc_models.proto configuration.proto outer_error.proto
Write-Host "protoc --csharp_out=$outDir -I=$protosDir"