FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    live-build \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

CMD ["lb clean"]
CMD ["lb config"]
CMD ["lb build"]