# `rgbldk` (CLI)

`rgbldk` is a command-line client that talks to a local RLN daemon (`rgbldkd`) over HTTP.

If you’re new to Bitcoin/Lightning: you can think of `rgbldkd` as a local service that exposes a JSON API, and `rgbldk` is a convenient terminal client for that API.

## Build

```bash
cargo build -p rgbldk-cli --bin rgbldk
```

Native Messaging Host (for browser extensions / desktop apps):

```bash
cargo build -p rgbldk-cli --bin rgbldk-nmh
```

Run it from the repo root:

```bash
./target/debug/rgbldk --help
```

## Connect to a daemon

By default the CLI connects to `http://127.0.0.1:8500`.

Override per-command:

```bash
./target/debug/rgbldk --connect http://127.0.0.1:8500 node status
```

Or set a default for the shell:

```bash
export RGBLDK_CONNECT=http://127.0.0.1:8500
./target/debug/rgbldk node status
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
./target/debug/rgbldk node status
./target/debug/rgbldk wallet balance
```

## Generate `EXAMPLE.md`

`crates/cli/EXAMPLE.md` is generated from a local regtest run. By default it will start bitcoind/esplora via docker-compose and run two `rgbldkd` daemons from a sibling checkout at `../rgb-ldk-node`. If that directory does not exist, it will ask before pulling remote docker images.

Starting from Phase 0, `rgbldkd` starts locked. The generator will initialize local keystores and unlock the daemons before running the rest of the example commands.

```bash
python3 crates/cli/scripts/gen_example_md.py
```

## Native Messaging Host

`rgbldk-nmh` is a local helper process intended for browser extensions (Native Messaging) and desktop apps. It reads length-prefixed JSON messages on stdin and writes length-prefixed JSON responses on stdout.

Supported methods:

- `version`
- `status` (control socket status)
- `unlock` (requires `params.passphrase`)
- `lock`
