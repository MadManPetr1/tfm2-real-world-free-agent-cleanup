# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

param(
    [string]$SdkDir = $env:TFM2_MOD_SDK,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

& (Join-Path $PSScriptRoot "validate_repo.ps1")

if (-not $SkipBuild) {
    & (Join-Path $root "build_local.ps1") -SdkDir $SdkDir
    if ($LASTEXITCODE -ne 0) {
        throw "Release build failed with exit code $LASTEXITCODE."
    }
}

$dll = Join-Path $root "real_world_free_agent_cleanup.dll"
if (-not (Test-Path -LiteralPath $dll -PathType Leaf)) {
    throw "real_world_free_agent_cleanup.dll is missing."
}

$modInfo = Get-Content -LiteralPath (Join-Path $root "mod.mod_info") -Raw | ConvertFrom-Json
$buildRoot = Join-Path $root "builds"
$releaseRoot = Join-Path $buildRoot "real-world-free-agent-cleanup-v$($modInfo.version)"
$runtimeRoot = Join-Path $releaseRoot "real_world_free_agent_cleanup"
$archive = Join-Path $buildRoot "real-world-free-agent-cleanup-v$($modInfo.version).zip"

if (Test-Path -LiteralPath $releaseRoot) {
    Remove-Item -LiteralPath $releaseRoot -Recurse -Force
}
if (Test-Path -LiteralPath $archive) {
    Remove-Item -LiteralPath $archive -Force
}
New-Item -ItemType Directory -Path $runtimeRoot -Force | Out-Null

foreach ($name in @(
    "real_world_free_agent_cleanup.dll",
    "mod.mod_info",
    "mod.override_info",
    "thumbnail.png",
    "README.md",
    "CHANGELOG.md",
    "LICENSE",
    "NOTICE.md"
)) {
    Copy-Item -LiteralPath (Join-Path $root $name) -Destination (Join-Path $runtimeRoot $name)
}

Compress-Archive -LiteralPath $runtimeRoot -DestinationPath $archive -CompressionLevel Optimal
Write-Host "Release package created: $archive"
