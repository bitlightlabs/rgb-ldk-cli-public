# Regtest Quickstart (Docker Compose + local binaries)

This guide starts a regtest chain source using Docker Compose, then runs RLN (`rgbldkd`) via the public Docker image and verifies it via CLI.

## Prerequisites

- Docker + Docker Compose
- Rust toolchain (for building the CLI `rgbldk`)

## 1) Start regtest chain source (bitcoind + esplora)

The repo provides a regtest stack in `docker-compose.yml`:

```bash
RPC_USER=bitcoin RPC_PASSWORD=bitcoin docker compose up -d bitcoin-core electrs mining
```

Verify Esplora is reachable:

```bash
curl -sSf http://127.0.0.1:3003/blocks/tip/height
curl -sSf http://127.0.0.1:3003/fee-estimates
```

## 2) Build the CLI (`rgbldk`)

```bash
cargo build -p rgbldk-cli --bin rgbldk
```

Add the local binary dir to your PATH for this shell:

```bash
export PATH="$PWD/target/debug:$PATH"
```

## 3) Run the daemon (`rgbldkd`) via Docker (`bitlightlabs/rln-ldk-node`)

Start RLN on regtest using Esplora (macOS/Windows):

```bash
docker run --rm \
  -p 127.0.0.1:8500:8500 \
  -v rgbldk-data:/home/rgbldk/.ldk-node \
  bitlightlabs/rln-ldk-node:latest \
  rgbldkd server \
  --allow-non-loopback-listen \
  --network regtest \
  --esplora-url http://host.docker.internal:3003 \
  --listen 0.0.0.0:8500 \
  --data-dir /home/rgbldk/.ldk-node \
  --log-to-stdout \
  --log-level info
```

Linux: add `--add-host=host.docker.internal:host-gateway` to the `docker run` command.

You should see a line like:

```text
rgbldkd listening on http://0.0.0.0:8500
```

## 4) Verify with the CLI (`rgbldk`)

```bash
rgbldk version
rgbldk healthz
rgbldk readyz
rgbldk status
```

If you run the daemon on a different port, point the CLI at it:

```bash
rgbldk --connect http://127.0.0.1:8500 status
RGBLDK_CONNECT=http://127.0.0.1:8500 rgbldk status
```

## Next steps

- CLI: [CLI overview](../cli/README.md)
- Open a channel: [Open a channel](../tutorials/open-a-channel.md)
- HTTP API overview: [HTTP API overview](../api/README.md)
