# Nutzen von Debian Bookworm als stabile Basis
FROM debian:bookworm

# Installieren von live-build und allen benötigten Tools
RUN apt-get update && apt-get install -y \
    live-build \
    debootstrap \
    squashfs-tools \
    xorriso \
    && rm -rf /var/lib/apt/lists/*

# Wir arbeiten in einem internen Ordner des Containers.
# Das verhindert Fehler mit dem Dateisystem deines Host-PCs.
WORKDIR /live-system

# Das ist die Automatisierung: Beim Starten wird radikal aufgeräumt,
# alles frisch konfiguriert, die ISO gebaut und am Ende in den Output-Ordner kopiert.
CMD lb clean --purge && \
    lb config \
        --binary-images iso-hybrid \
        --architectures amd64 \
        --distribution bookworm && \
    lb build && \
    mkdir -p /output && cp *.iso /output/