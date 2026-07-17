# rgb-ldk-cli

Tools for running and controlling an RLN node via a local HTTP JSON API.

Most users only need:

- `rgbldkd` (daemon): runs the node and serves the HTTP API.
- `rgbldk` (CLI): a terminal client for calling that API.
- TypeScript SDK: for web apps / Node.js services that want to call the API directly.

## What’s in this repo

- Rust CLI (`rgbldk`): [`crates/cli`](crates/cli)
- TypeScript SDK: [`sdk/`](sdk)

HTTP API DTOs + generated Rust client live in `rgb-ldk-api`:

- `rgbldk_http_dto`
- `rgbldk_http_client`

## Quick start (local regtest)

1) Start a local regtest environment (bitcoind + esplora + two daemons):

```bash
RPC_USER=bitcoin RPC_PASSWORD=bitcoin docker compose -f crates/cli/docker-compose.yaml up -d
```

This exposes:

- node A: `http://127.0.0.1:8501`
- node B: `http://127.0.0.1:8502`

2) Build the CLI:

```bash
cargo build -p rgbldk-cli --bin rgbldk
```

3) Call the daemon:

```bash
RGBLDK_CONNECT=http://127.0.0.1:8501 ./target/debug/rgbldk node status
RGBLDK_CONNECT=http://127.0.0.1:8501 ./target/debug/rgbldk wallet balance
```

## Offline debugging (invoice + consignment)

These commands do **not** talk to `rgbldkd`; they help debug issues like
`No owned allocations resolved from consignment` by checking whether an expected seal/outpoint is
present in a consignment.

```bash
# Parse an RGB invoice URI (pass `-` to read from stdin, or `@file` to read from a file)
./target/debug/rgbldk debug invoice 'contract:...'

# Inspect a consignment file (raw/gzip/zip auto-detected)
./target/debug/rgbldk debug consignment ./transfer.consignment.zip

# Check for a specific outpoint and/or invoice beneficiary in the consignment
./target/debug/rgbldk debug consignment ./transfer.consignment.zip \
  --outpoint cb6525f40318ba6d7a999c429bb2d60f2dab37731db5a14511fa11cd014429b8:0 \
  --invoice 'contract:...'
```

## SDK Docs

- TypeScript SDK: [`sdk/README.md`](sdk/README.md)

## Atomic BTC/RGB swaps

The CLI exposes the node's swap lifecycle, including multi-hop offers and the
asynchronous settlement status:

```bash
rgbldk swap create --counterparty-node-id <node_id> --channel-scid <scid> \
  --contract-id <contract_id> --asset-amount <amount> \
  --btc-amount-msat <amount> --btc-carrier-amount-msat <amount> \
  --maker-gives-rgb
rgbldk swap decode --swap-string 'rgb-swap:v1:...'
rgbldk swap accept --swap-string 'rgb-swap:v1:...'
rgbldk swap execute --swap-string 'rgb-swap:v1:...'
rgbldk swap get <payment_hash>
```

`swap execute` only initiates the circular payment. Poll `swap get` until the
status is `Settled` or `Failed`; a successful HTTP response is not settlement.

## License

Dual-licensed under Apache-2.0 and MIT. See [`LICENSE-APACHE`](LICENSE-APACHE) and [`LICENSE-MIT`](LICENSE-MIT).
