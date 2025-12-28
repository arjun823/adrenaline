param([switch]$Verbose = $false)

$ErrorActionPreference = "Stop"
$env:PYO3_USE_ABI3_FORWARD_COMPATIBILITY = 1

$BinaryName = "adrenaline.exe"
$SourcePath = "target\release\$BinaryName"
$DistDir = "dist"
$DestPath = "$DistDir\$BinaryName"

Write-Host "Adrenaline Compiler Build" -ForegroundColor Cyan
Write-Host "Building release binary..." -ForegroundColor Green

if ($Verbose) {
    cargo build --release -v
} else {
    cargo build --release
}

if ($LASTEXITCODE -ne 0) { exit 1 }

New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

if (-not (Test-Path $SourcePath)) {
    Write-Host "Error: Binary not found at $SourcePath" -ForegroundColor Red
    exit 1
}

Copy-Item -Force $SourcePath $DestPath

Remove-Item -Force "$DistDir\adrenaline.pdb" -ErrorAction SilentlyContinue
Remove-Item -Force "$DistDir\adrenaline.d" -ErrorAction SilentlyContinue

Write-Host "Build complete! Binary ready at: $DestPath" -ForegroundColor Green
