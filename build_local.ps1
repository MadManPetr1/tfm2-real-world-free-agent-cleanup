# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

param(
    [string]$SdkDir = $env:TFM2_MOD_SDK
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($SdkDir)) {
    throw "Pass -SdkDir <path-to-v0.5.4-mod-sdk> or set TFM2_MOD_SDK."
}

$sdk = (Resolve-Path -LiteralPath $SdkDir).Path
$depsDir = Join-Path $sdk "deps"
$nativeDir = Join-Path $sdk "native"
$manifest = Join-Path $PSScriptRoot "Cargo.toml"
$targetDir = Join-Path $PSScriptRoot "target"
$baseVersion = (Get-Content -LiteralPath (Join-Path $sdk "base_version.txt") -Raw).Trim()
if ($baseVersion -ne "0.5.4") {
    throw "Real World Free Agent Cleanup 0.2.0 must be built with the 0.5.4 Mod SDK; found $baseVersion."
}

$pinned = Select-String -LiteralPath (Join-Path $sdk "rust-toolchain.toml") `
    -Pattern '^\s*channel\s*=\s*"([^"]+)"' |
    ForEach-Object { $_.Matches[0].Groups[1].Value } |
    Select-Object -First 1
if (-not $pinned) {
    throw "Could not read the SDK's pinned Rust toolchain."
}
$env:RUSTUP_TOOLCHAIN = $pinned

# The supported SDK baseline ships Rust object code as LLVM bitcode. MSVC link.exe cannot
# consume those archive members, so expose rust-lld under its COFF driver name.
$sysroot = (& rustup run $pinned rustc --print sysroot | Select-Object -First 1).Trim()
if ([string]::IsNullOrWhiteSpace($sysroot) -or -not (Test-Path -LiteralPath $sysroot)) {
    throw "Could not locate the SDK's pinned Rust sysroot."
}
$rustLld = Join-Path $sysroot "lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe"
if (-not (Test-Path -LiteralPath $rustLld -PathType Leaf)) {
    throw "rust-lld.exe is missing from the SDK's pinned Rust toolchain."
}
$linkerDir = Join-Path ([System.IO.Path]::GetTempPath()) "tfm2-mod-sdk-linker\$pinned"
$lldLink = Join-Path $linkerDir "lld-link.exe"
New-Item -ItemType Directory -Path $linkerDir -Force | Out-Null
if (-not (Test-Path -LiteralPath $lldLink -PathType Leaf)) {
    try {
        New-Item -ItemType HardLink -Path $lldLink -Target $rustLld -ErrorAction Stop | Out-Null
    }
    catch {
        Copy-Item -LiteralPath $rustLld -Destination $lldLink
    }
}
$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = $lldLink

function Find-SdkRlib([string]$pattern) {
    $matches = @(Get-ChildItem -LiteralPath $depsDir -Filter $pattern)
    if ($matches.Count -ne 1) {
        throw "Expected exactly one SDK dependency matching $pattern; found $($matches.Count)."
    }
    return $matches[0].FullName
}

$modApi = Find-SdkRlib "libmod_api-*.rlib"
$gameCore = Find-SdkRlib "libgame_core-*.rlib"
$arrayvec = Find-SdkRlib "libarrayvec-*.rlib"

$flags = @(
    "-L", "dependency=$depsDir",
    "--extern", "mod_api=$modApi",
    "--extern", "game_core=$gameCore",
    "--extern", "arrayvec=$arrayvec",
    "-L", "native=$nativeDir"
)
$env:CARGO_ENCODED_RUSTFLAGS = $flags -join [char]31

cargo rustc --release --manifest-path $manifest --target-dir $targetDir --lib -- --crate-type cdylib
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$builtDll = Join-Path $targetDir "release\tfm2_real_world_free_agent_cleanup.dll"
$outputDll = Join-Path $PSScriptRoot "tfm2_real_world_free_agent_cleanup.dll"
Copy-Item -LiteralPath $builtDll -Destination $outputDll -Force
Write-Host "Build successful: $outputDll"
