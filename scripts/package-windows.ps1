# Portable Windows x86_64 (MSVC) release. Run with PowerShell 7+ or Windows PowerShell.
param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+\.\d+\.\d+([-.][0-9A-Za-z.-]+)?$')]
    [string]$Version
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    & cargo build -p argus-ui --release --locked --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw "Windows x86_64 cargo build failed (exit $LASTEXITCODE)" }

    $binary = 'target/x86_64-pc-windows-msvc/release/argus-ui.exe'
    if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
        throw "Expected binary missing: $binary"
    }

    $name = "argus-ui-$Version-windows-x64"
    $stage = Join-Path 'dist' $name
    $archive = Join-Path 'dist' "$name.zip"
    if (Test-Path -LiteralPath $stage) {
        Remove-Item -LiteralPath $stage -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $stage | Out-Null
    Copy-Item -LiteralPath $binary -Destination (Join-Path $stage 'ArgusUI.exe')
    Copy-Item -Path README.md, LICENSE, CHANGELOG.md -Destination $stage
    @"
Argus UI $Version - Windows x86_64
=================================
Run ArgusUI.exe directly. No installer or WebView runtime is needed.
Settings: %LOCALAPPDATA%\Argus-UI\settings.conf

This executable is not code-signed with a trusted publisher certificate.
"@ | Set-Content -LiteralPath (Join-Path $stage 'README.txt') -Encoding utf8

    Compress-Archive -LiteralPath $stage -DestinationPath $archive -CompressionLevel Optimal -Force
    if (-not (Test-Path -LiteralPath $archive -PathType Leaf)) {
        throw "Expected Windows ZIP missing: $archive"
    }
    Write-Host "Built $archive"
}
finally {
    Pop-Location
}
