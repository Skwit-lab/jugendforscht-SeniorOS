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

    docker_cmd = (
        f"docker run --privileged --rm "
        f"-v \"{config_path}:/build\" "
        f"-w /build "
        f"{image_name} "
        f"bash -c \"chmod +x auto/config && lb clean\""
    )

    run(docker_cmd)

if __name__ == "__main__":
    main()
