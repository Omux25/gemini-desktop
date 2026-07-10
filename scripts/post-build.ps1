#!/usr/bin/env pwsh
# post-build.ps1 — Renames Tauri build artifacts to clean, user-friendly names.
# Called automatically by `npm run release`.

param(
    [string]$Version
)

if (-not $Version) {
    $tauriConf = Get-Content "src-tauri/tauri.conf.json" -Raw | ConvertFrom-Json
    $Version = $tauriConf.version
}

$nsisDir = "src-tauri/target/release/bundle/nsis"

$mappings = @(
    @{ From = "Gemini Desktop_${Version}_x64-setup.exe";          To = "Gemini-Desktop-Setup.exe" }
    @{ From = "Gemini Desktop_${Version}_x64-setup.nsis.zip";     To = "Gemini-Updater-Core.zip" }
    @{ From = "Gemini Desktop_${Version}_x64-setup.nsis.zip.sig"; To = "Gemini-Updater-Core.zip.sig" }
)

Write-Host "`n=== Post-Build: Renaming Artifacts ===" -ForegroundColor Cyan

foreach ($m in $mappings) {
    $src = Join-Path $nsisDir $m.From
    $dst = Join-Path $nsisDir $m.To
    if (Test-Path $src) {
        Copy-Item $src $dst -Force
        Write-Host "  [OK] $($m.From) -> $($m.To)" -ForegroundColor Green
    } else {
        Write-Host "  [SKIP] $($m.From) not found" -ForegroundColor Yellow
    }
}

# Also copy the portable exe
$releaseBin = "src-tauri/target/release/Gemini-Desktop.exe"
$portableDst = "Gemini-Desktop-Portable.exe"
if (Test-Path $releaseBin) {
    Copy-Item $releaseBin $portableDst -Force
    Write-Host "  [OK] Gemini-Desktop.exe -> $portableDst" -ForegroundColor Green
}

Write-Host "`n=== Post-Build Complete ===" -ForegroundColor Cyan
Write-Host "Ready to publish! Artifacts are in: $nsisDir" -ForegroundColor White
