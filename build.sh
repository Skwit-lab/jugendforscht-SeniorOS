#!/bin/bash
set -euo pipefail

IMAGE_NAME="senioros-builder"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONFIG_DIR="$SCRIPT_DIR/SeniorOS-Beta-v0.1.26b"

if [ ! -d "$CONFIG_DIR" ]; then
    echo "❌ Config-Ordner nicht gefunden: $CONFIG_DIR"
    exit 1
fi

echo "═══════════════════════════════════════"
echo "  SeniorOS Live-Build mit Docker"
echo "═══════════════════════════════════════"
echo ""

# Step 1
echo "[1/2] Docker-Image bauen ..."
docker build -t "$IMAGE_NAME" "$SCRIPT_DIR"
echo "✅ Docker-Image '$IMAGE_NAME' erstellt."
echo ""

# Step 2
echo "[2/2] Debian Live-ISO bauen (dauert ~30-60 Min) ..."
echo ""

docker run --privileged --rm \
    -v "$CONFIG_DIR:/build" \
    -w /build \
    "$IMAGE_NAME" \
    bash -c "chmod +x auto/config && lb build"

echo ""
echo "═══════════════════════════════════════"
echo "  ✅ ISO erfolgreich gebaut!"
echo "  📁 $CONFIG_DIR/live-image-amd64.iso"
echo "═══════════════════════════════════════"
