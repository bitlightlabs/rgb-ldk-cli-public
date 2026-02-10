# Local Binary Quickstart (`rgbldkd` + `rgbldk`)

This guide runs the RLN daemon (`rgbldkd`) as a local executable and uses the CLI (`rgbldk`) to talk to it over HTTP.

`rgbldkd` is provided as a standalone executable. Make sure you have it available on your machine.

## Prerequisites

- `rgbldkd` executable in your `PATH` (or use an absolute path)
- A reachable chain source (Esplora/Electrum) for your chosen network
- Rust toolchain (to build `rgbldk`)

## 1) Build the CLI (`rgbldk`)

```bash
cargo build -p rgbldk-cli --bin rgbldk
export PATH="$PWD/target/debug:$PATH"
```

## 2) Run the daemon (`rgbldkd`)

Pick a network and a chain source. The HTTP API defaults to `http://127.0.0.1:8500`.

### Mainnet (Esplora)

```bash
rgbldkd server \
  --network bitcoin \
  --esplora-url https://blockstream.info/api \
  --listen 127.0.0.1:8500 \
  --data-dir ~/.rln
```

### Testnet (Esplora)

```bash
rgbldkd server \
  --network testnet \
  --esplora-url https://blockstream.info/testnet/api \
  --listen 127.0.0.1:8500 \
  --data-dir ~/.rln
```

Tip: use a dedicated `--data-dir` per network (e.g. `~/.rln-testnet`) to avoid mixing state.

## 3) Verify

Health check:

```bash
curl -sSf http://127.0.0.1:8500/api/v1/healthz
```

Then via CLI:

```bash
rgbldk node version
rgbldk node health
rgbldk node status
```
