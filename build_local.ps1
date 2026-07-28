param(
    [string]$SdkDir = $env:TFM2_MOD_SDK
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($SdkDir)) {
    throw "Pass -SdkDir <path-to-v0.5.2-mod-sdk> or set TFM2_MOD_SDK."
}

$sdk = (Resolve-Path -LiteralPath $SdkDir).Path
$depsDir = Join-Path $sdk "deps"
$nativeDir = Join-Path $sdk "native"
$manifest = Join-Path $PSScriptRoot "Cargo.toml"
$targetDir = Join-Path $PSScriptRoot "target"

$pinned = Select-String -LiteralPath (Join-Path $sdk "rust-toolchain.toml") `
    -Pattern '^\s*channel\s*=\s*"([^"]+)"' |
    ForEach-Object { $_.Matches[0].Groups[1].Value } |
    Select-Object -First 1
if (-not $pinned) {
    throw "Could not read the SDK's pinned Rust toolchain."
}
$env:RUSTUP_TOOLCHAIN = $pinned

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

$builtDll = Join-Path $targetDir "release\real_world_free_agent_cleanup.dll"
$outputDll = Join-Path $PSScriptRoot "real_world_free_agent_cleanup.dll"
Copy-Item -LiteralPath $builtDll -Destination $outputDll -Force
Write-Host "Build successful: $outputDll"
