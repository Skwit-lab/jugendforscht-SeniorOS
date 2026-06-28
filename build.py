import subprocess
import argparse
import os
import sys

def run(cmd, cwd=None):
    print(f"→ {cmd}")
    result = subprocess.run(cmd, shell=True, cwd=cwd)
    if result.returncode != 0:
        print("❌ Fehler beim Ausführen:", cmd)
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="SeniorOS Live-Build")
    parser.add_argument("--no-cache", action="store_true", help="Docker ohne Cache bauen")
    parser.add_argument("--skip-build", action="store_true", help="ISO-Build überspringen")
    args = parser.parse_args()

    image_name = "senioros-builder"
    config_path = os.path.join(os.getcwd(), "SeniorOS-Beta-v0.1.26b")

    if not os.path.exists(config_path):
        print(f"❌ Config-Ordner nicht gefunden: {config_path}")
        sys.exit(1)

    print("═══════════════════════════════════════")
    print("  SeniorOS Live-Build mit Docker")
    print("═══════════════════════════════════════\n")

    # Step 1: Docker-Image bauen
    print("[1/2] Docker-Image bauen ...")
    build_cmd = f"docker build -t {image_name} ."
    if args.no_cache:
        build_cmd += " --no-cache"
    run(build_cmd)
    print(f"✅ Docker-Image '{image_name}' erstellt.\n")

    if args.skip_build:
        print("⏭️  ISO-Build übersprungen (--skip-build).")
        sys.exit(0)

    # Step 2: Live-ISO bauen
    print("[2/2] Debian Live-ISO bauen (dauert ~30-60 Min) ...\n")

    docker_cmd = (
        f"docker run --privileged --rm "
        f"-v \"{config_path}:/build\" "
        f"-w /build "
        f"{image_name} "
        f"bash -c \"chmod +x auto/config && lb clean && lb build\""
    )

    run(docker_cmd)

    iso_path = os.path.join(config_path, "live-image-amd64.iso")
    print("\n═══════════════════════════════════════")
    print("  ✅ ISO erfolgreich gebaut!")
    print(f"  📁 {iso_path}")
    print("═══════════════════════════════════════")

if __name__ == "__main__":
    main()
