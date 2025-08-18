#!/usr/bin/env pwsh

# Import the build_adapter.ps1 script to access its functions
$scriptDir = $PSScriptRoot
$buildAdapterPath = Join-Path $scriptDir "build_adapter.ps1"

# Source the build_adapter.ps1 script to make its functions available
. $buildAdapterPath

# Call the GenerateProtobufBindings function
GenerateProtobufBindings

Write-Output "Protobuf bindings generation completed via generate_protobuf.ps1"