# `rgbldk` (CLI)

`rgbldk` is a command-line client that talks to a local RLN daemon (`rgbldkd`) over HTTP.

If you’re new to Bitcoin/Lightning: you can think of `rgbldkd` as a local service that exposes a JSON API, and `rgbldk` is a convenient terminal client for that API.

## Build

```bash
cargo build -p rgbldk-cli --bin rgbldk
```

Run it from the repo root:

```bash
./target/debug/rgbldk --help
```

## Connect to a daemon

By default the CLI connects to `http://127.0.0.1:8500`.

Override per-command:

```bash
./target/debug/rgbldk --connect http://127.0.0.1:8500 status
```

Or set a default for the shell:

```bash
export RGBLDK_CONNECT=http://127.0.0.1:8500
./target/debug/rgbldk status
```

## Regtest environment (Docker Compose)

This repo includes a self-contained regtest environment (bitcoind + esplora + two daemons) in `crates/cli/docker-compose.yaml`.

Start everything:

```bash
RPC_USER=bitcoin RPC_PASSWORD=bitcoin docker compose -f crates/cli/docker-compose.yaml up -d
```

Endpoints on the host:

- node A HTTP: `http://127.0.0.1:8501`
- node B HTTP: `http://127.0.0.1:8502`
- esplora: `http://127.0.0.1:3003`

Then:

```bash
export RGBLDK_CONNECT=http://127.0.0.1:8501
./target/debug/rgbldk status
./target/debug/rgbldk balances
```

## Docs

- CLI reference: [`../../docs/cli/README.md`](../../docs/cli/README.md)
- Commands list: [`../../docs/cli/commands.md`](../../docs/cli/commands.md)
- Regtest quickstart: [`../../docs/getting-started/regtest.md`](../../docs/getting-started/regtest.md)

