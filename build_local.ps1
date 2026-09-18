# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

param([string]$SdkDir = $env:TFM2_MOD_SDK)

$ErrorActionPreference = "Stop"
if (-not $SdkDir) { throw 'Pass -SdkDir <game-root>/mod-sdk-stable or set TFM2_MOD_SDK.' }
$sdk = (Resolve-Path -LiteralPath $SdkDir).Path
$api = Join-Path $sdk 'mod-api-stable'
if (-not (Test-Path -LiteralPath (Join-Path $api 'Cargo.toml'))) {
    throw 'SdkDir must identify a Stable SDK, not a classic SDK.'
}
$vendor = Join-Path $PSScriptRoot 'vendor'
New-Item -ItemType Directory -Path $vendor -Force | Out-Null
Copy-Item -LiteralPath $api -Destination $vendor -Recurse -Force
$manifest = Join-Path $PSScriptRoot 'Cargo.toml'
cargo build --locked --release --manifest-path $manifest
if ($LASTEXITCODE -ne 0) { throw "Build failed: $LASTEXITCODE" }
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target/release/tfm2_real_world_free_agent_cleanup.dll') `
    -Destination (Join-Path $PSScriptRoot 'tfm2_real_world_free_agent_cleanup.dll') -Force
Write-Host 'Built RWFAC with the Stable API.'
