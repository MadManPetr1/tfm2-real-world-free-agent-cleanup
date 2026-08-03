# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

param()

$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Add-Type -AssemblyName System.Drawing

foreach ($relativePath in @(
    "Cargo.toml",
    "Cargo.lock",
    "src/lib.rs",
    "mod.mod_info",
    "mod.override_info",
    "better_mod_menu_profile.json",
    "profile_icon.png",
    "thumbnail.png",
    "assets/thumbnail-master.png",
    "README.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "LICENSE",
    "NOTICE.md",
    "docs/PRESENTATION.md",
    "docs/RELEASING.md",
    "scripts/package_release.ps1"
)) {
    $path = Join-Path $root $relativePath
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Required file is missing: $relativePath"
    }
}

$profile = Get-Content -LiteralPath (Join-Path $root "better_mod_menu_profile.json") -Raw |
    ConvertFrom-Json
if ($profile.schema_version -ne 1 -or $profile.profile_icon -ne "profile_icon.png") {
    throw "better_mod_menu_profile.json must use schema version 1 and profile_icon.png."
}

$image = [System.Drawing.Image]::FromFile((Join-Path $root "thumbnail.png"))
try {
    if ($image.Width -ne 256 -or $image.Height -ne 256) {
        throw "thumbnail.png must be 256x256."
    }
}
finally {
    $image.Dispose()
}

$thumbnailMaster = [System.Drawing.Image]::FromFile((Join-Path $root "assets/thumbnail-master.png"))
try {
    if ($thumbnailMaster.Width -ne 128 -or $thumbnailMaster.Height -ne 128) {
        throw "assets/thumbnail-master.png must be 128x128."
    }
}
finally {
    $thumbnailMaster.Dispose()
}

$modInfo = Get-Content -LiteralPath (Join-Path $root "mod.mod_info") -Raw | ConvertFrom-Json
if ($modInfo.mod_id -ne "real_world_free_agent_cleanup") {
    throw "mod.mod_info must declare mod_id real_world_free_agent_cleanup."
}
$cargo = Get-Content -LiteralPath (Join-Path $root "Cargo.toml") -Raw
$cargoVersion = [regex]::Match($cargo, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if ($cargoVersion -ne $modInfo.version) {
    throw "Version mismatch: Cargo.toml is $cargoVersion, mod.mod_info is $($modInfo.version)."
}
if ($cargo -notmatch '(?m)^license\s*=\s*"MPL-2\.0"') {
    throw "Cargo.toml must declare MPL-2.0."
}

$source = Get-Content -LiteralPath (Join-Path $root "src/lib.rs") -Raw
if ($source -notmatch 'const MOD_ID: &str = "real_world_free_agent_cleanup";') {
    throw 'The Rust MOD_ID must remain "real_world_free_agent_cleanup".'
}
if ($source -notmatch 'CONTRACT_CORRECTIONS' -or
    $source -notmatch 'verified_stale_contract_ids') {
    throw "Contracted-player cleanup must remain an explicit, reviewed correction list."
}

Push-Location $root
try {
    cargo fmt --check
    if ($LASTEXITCODE -ne 0) {
        throw "cargo fmt --check failed."
    }
}
finally {
    Pop-Location
}

Write-Host "Repository validation passed for Real World Free Agent Cleanup v$($modInfo.version)."
