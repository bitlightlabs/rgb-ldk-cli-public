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

## LSPS1 service (selling channels)

To run a node as an LSPS1 (bLIP-51) channel seller, start `rgbldkd` with
`--lsps1-service-config <path.json>`. The regtest compose file already does this
for node A (`crates/cli/lsps1-service.json`, mounted read-only). Omit the flag to
leave the service off.

Every field is optional. The file only **seeds** the config on first start —
after that the persisted value wins, and you change limits and pricing at runtime
via `PUT /lsps1/options` and `PUT /lsps1/pricing`. The one exception is
`advertise_service`, read from the file on every start.

Minimal example (`crates/cli/lsps1-service.json`):

```json
{
  "advertise_service": true,
  "min_initial_lsp_balance_sat": 10000,
  "max_initial_lsp_balance_sat": 100000000,
  "min_channel_balance_sat": 10000,
  "max_channel_balance_sat": 100000000
}
```

| Field | Type | Default | Meaning |
|-------|------|---------|---------|
| `require_token` | string | *(none)* | Require this token on incoming orders; unset means no token check. |
| `advertise_service` | bool | `false` | Announce the LSPS1 service to peers. Re-read on every start. |
| `min_required_channel_confirmations` | u16 | `0` | Confirmations before the client may use the channel. |
| `min_funding_confirms_within_blocks` | u16 | `6` | Blocks the LSP promises to confirm funding within. |
| `supports_zero_channel_reserve` | bool | `false` | Offer zero-reserve channels. |
| `max_channel_expiry_blocks` | u32 | `52560` | Max lease length a client may request (~1 year). |
| `min_initial_client_balance_sat` | u64 | `0` | Min sats pushed to the client side. |
| `max_initial_client_balance_sat` | u64 | `0` | Max sats pushed to the client side. |
| `min_initial_lsp_balance_sat` | u64 | `20000` | Min inbound liquidity per order. |
| `max_initial_lsp_balance_sat` | u64 | `500000000` | Max inbound liquidity per order. |
| `min_channel_balance_sat` | u64 | `20000` | Min total channel capacity. |
| `max_channel_balance_sat` | u64 | `500000000` | Max total channel capacity. |
| `bolt11_invoice_expiry_secs` | u32 | `3600` | Lifetime of a BOLT11 order invoice. |
| `min_onchain_payment_confirmations` | u16 | `1` | Confirmations required for an onchain deposit. |
| `max_fulfill_retries` | u32 | `3` | Channel-open retries before an order fails. |
| `default_btc_capacity_ppm_per_year` | u32 | `10000` | Seed lease rate for BTC capacity (ppm/year). |
| `default_onchain_cost_sat` | u64 | `10000` | Seed onchain cost charged per order. |
| `default_min_fee_sat` | u64 | `1000` | Seed minimum total fee per order. |
| `auto_close_expired_channels` | bool | `true` | Cooperatively close leased channels after expiry. |
| `channel_expiry_grace_blocks` | u32 | `144` | Grace blocks before an expired lease is closed. |
| `late_deposit_refund_window_secs` | u64 | `2592000` | Window for auto-refunding a late deposit (30 days). |

The `default_*` fields only seed pricing on first start; adjust them later with
`PUT /lsps1/pricing`.

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
