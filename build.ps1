param(
    [switch]$NoCache,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$imageName = "senioros-builder"
$configPath = Join-Path $PWD "SeniorOS-Beta-v0.1.26b"

if (-not (Test-Path $configPath)) {
    Write-Host "❌ Config-Ordner nicht gefunden: $configPath" -ForegroundColor Red
    exit 1
}

Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  SeniorOS Live-Build mit Docker" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Step 1: Docker-Image bauen
Write-Host "[1/2] Docker-Image bauen ..." -ForegroundColor Cyan
$buildArgs = @("build", "-t", $imageName, ".")
if ($NoCache) {
    $buildArgs += "--no-cache"
}
$process = Start-Process -FilePath "docker" -ArgumentList $buildArgs -NoNewWindow -Wait -PassThru
if ($process.ExitCode -ne 0) {
    Write-Host "❌ Docker-Image Build fehlgeschlagen." -ForegroundColor Red
    exit 1
}
Write-Host "✅ Docker-Image '$imageName' erstellt." -ForegroundColor Green
Write-Host ""

if ($SkipBuild) {
    Write-Host "⏭️  ISO-Build übersprungen (--SkipBuild)." -ForegroundColor Yellow
    exit 0
}

# Step 2: Live-ISO bauen
Write-Host "[2/2] Debian Live-ISO bauen (dauert ~30-60 Min) ..." -ForegroundColor Cyan
Write-Host ""

docker run --privileged --rm `
    -v "${configPath}:/build" `
    -w /build `
    $imageName `
    bash -c "chmod +x auto/config && lb build"

if ($LASTEXITCODE -eq 0) {
    $isoPath = Join-Path $configPath "live-image-amd64.iso"
    Write-Host ""
    Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "  ✅ ISO erfolgreich gebaut!" -ForegroundColor Green
    Write-Host "  📁 $isoPath" -ForegroundColor White
    Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
} else {
    Write-Host ""
    Write-Host "❌ Build fehlgeschlagen." -ForegroundColor Red
    Write-Host "  Tipps:" -ForegroundColor Yellow
    Write-Host "  • Internetverbindung prüfen"
    Write-Host "  • Docker Desktop muss laufen (mit WSL2-Backend)"
    Write-Host "  • Genug Speicherplatz? (min. 5 GB frei)"
    Write-Host "  • Mit 'build.ps1 -NoCache' frisch bauen"
    exit 1
}
