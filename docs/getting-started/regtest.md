# Regtest Quickstart (Docker Compose)

This guide starts a self-contained regtest environment (bitcoind + esplora + two nodes) using Docker Compose, then verifies it via the CLI.

## Prerequisites

- Docker + Docker Compose
- Rust toolchain (for building the CLI `rgbldk`)

## 1) Start the regtest environment

The repo provides a regtest stack in `crates/cli/docker-compose.yaml`:

```bash
RPC_USER=bitcoin RPC_PASSWORD=bitcoin docker compose -f crates/cli/docker-compose.yaml up -d
```

Verify Esplora is reachable:

```bash
curl -sSf http://127.0.0.1:3003/blocks/tip/height
curl -sSf http://127.0.0.1:3003/fee-estimates
```

The two node endpoints on the host are:

```text
node_a=http://127.0.0.1:8501
node_b=http://127.0.0.1:8502
```

## 2) Build the CLI (`rgbldk`)

```bash
cargo build -p rgbldk-cli --bin rgbldk
```

Add the local binary dir to your PATH for this shell:

```bash
export PATH="$PWD/target/debug:$PATH"
```

## 3) Verify with the CLI (`rgbldk`)

```bash
RGBLDK_CONNECT=http://127.0.0.1:8501 rgbldk node version
RGBLDK_CONNECT=http://127.0.0.1:8501 rgbldk node health
RGBLDK_CONNECT=http://127.0.0.1:8501 rgbldk node ready
RGBLDK_CONNECT=http://127.0.0.1:8501 rgbldk node status
```

## Next steps

- CLI: [CLI overview](../cli/README.md)
- Open a channel: [Open a channel](../tutorials/open-a-channel.md)
- HTTP API overview: [HTTP API overview](../api/README.md)
