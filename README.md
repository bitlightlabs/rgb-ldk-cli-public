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

## Buying from LSPs (client side)

A node keeps a list of LSPs it buys channels from. `lsp add` connects to the LSP
and asks which LSPS protocols it supports; the entry is saved only if that works.
Every purchase names the LSP it buys from with `--lsp <pubkey>`.

```bash
rgbldk lsp add --pubkey <lsp_pubkey> --address 203.0.113.5:9735 --trust-0conf
rgbldk lsp ls
rgbldk lsps1 info                                   # all LSPS1 LSPs, or --lsp <pubkey>
rgbldk lsps1 order create --lsp <lsp_pubkey> --lsp-balance-sat 100000 --channel-expiry-blocks 4380
rgbldk lsp rm <lsp_pubkey>
```

- `--trust-0conf`: use channels from this LSP before the funding confirms. JIT
  channels (LSPS2) need it.
- `--trust-no-reserve`: don't keep an on-chain reserve (25,000 sats per channel)
  for force-close fees on channels from this LSP. Needed when the node has no
  on-chain BTC.
- `--token`: if the LSP requires one.

Running `lsp add` again with the same pubkey replaces the entry, so pass every
flag again: an omitted `--trust-*` flag or `--token` is reset. `lsp rm` is refused
while the LSP has paid orders whose channel is not open yet. JIT invoices bought
from a removed LSP can no longer be paid: the node stops accepting its 0conf
channels.

## LSPS1 service (selling channels)

To run a node as an LSPS1 (bLIP-51) channel seller, start `rgbldkd` with
`--lsps1-service-config <path.json>`. The regtest compose file already does this
for node A (`crates/cli/lsps1-service.json`, mounted read-only). Omit the flag to
leave the service off. The file is strict: an unknown field stops the node from
starting. Amounts are plain JSON numbers here (`20000`), not the decimal strings
(`"20000"`) the HTTP API uses; a string also stops the node from starting.

Every field is optional. Limits and pricing can be changed at runtime via
`PUT /lsps1/options` and `PUT /lsps1/pricing`; once one of them has been used,
its persisted value wins over the file on every later start. Until then the file
is read on every start. `advertise_service` is always read from the file.

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
| `advertise_service` | bool | `false` | Announce this node as an LSP to peers. Shared with LSPS2: `true` in either file turns it on. Re-read on every start. |
| `min_required_channel_confirmations` | u16 | `0` | Confirmations before the client may use the channel. |
| `min_funding_confirms_within_blocks` | u16 | `6` | Blocks the LSP promises to confirm funding within. |
| `supports_zero_channel_reserve` | bool | `false` | Not supported yet: the node turns it off at startup. |
| `max_channel_expiry_blocks` | u32 | `52560` | Max lease length a client may request (~1 year). |
| `min_initial_client_balance_sat` | u64 | `0` | Min sats pushed to the client side. |
| `max_initial_client_balance_sat` | u64 | `0` | Max sats pushed to the client side. |
| `min_initial_lsp_balance_sat` | u64 | `20000` | Min inbound liquidity per order. |
| `max_initial_lsp_balance_sat` | u64 | `500000000` | Max inbound liquidity per order. |
| `min_channel_balance_sat` | u64 | `20000` | Min total channel capacity. |
| `max_channel_balance_sat` | u64 | `500000000` | Max total channel capacity. |
| `bolt11_invoice_expiry_secs` | u32 | `3600` | Lifetime of a BOLT11 order invoice. |
| `min_onchain_payment_confirmations` | u16 | `1` | Confirmations required for an onchain deposit. |
| `default_btc_capacity_ppm_per_year` | u32 | `10000` | Seed lease rate for BTC capacity (ppm/year). |
| `default_onchain_cost_sat` | u64 | `10000` | Seed onchain cost charged per order. |
| `default_min_fee_sat` | u64 | `1000` | Seed minimum total fee per order. |
| `auto_close_expired_channels` | bool | `true` | Cooperatively close leased channels after expiry. |
| `channel_expiry_grace_blocks` | u32 | `144` | Grace blocks before an expired lease is closed. |
| `late_deposit_refund_window_secs` | u64 | `2592000` | Window for auto-refunding a late deposit (30 days). |
| `onchain_open_timeout_blocks` | u32 | `144` | Blocks an on-chain-paid order keeps retrying the channel open while the client is unreachable, before it is refunded. |

The `default_*` fields set the pricing until the first `PUT /lsps1/pricing`;
after that the persisted pricing is used.

## LSPS2 service (JIT channels)

With LSPS2 (bLIP-52) a client with no channel can still receive: it buys a JIT
invoice from the LSP, and when the payment arrives the LSP opens a 0conf channel
to the client and forwards the payment minus an opening fee. Both BTC and RGB
payments work.

Start `rgbldkd` with `--lsps2-service-config <path.json>`. RGB JIT channels are
priced with the LSPS1 pricing and need `--lsps1-service-config` as well. The
regtest compose file runs both on node A (`crates/cli/lsps2-service.json`).

### Client side

The client adds the LSP with `--trust-0conf` first. Without it, `invoice` and
`rgb-invoice` fail with 409 `precondition_failed` and nothing is bought. A client
with no on-chain BTC also needs `--trust-no-reserve`. The client must be online
when the payment arrives.

```bash
# BTC
rgbldk lsps2 quote   --lsp <lsp_pubkey> --amount-msat 50000000
rgbldk lsps2 invoice --lsp <lsp_pubkey> --amount-msat 50000000 --max-fee 2500000

# RGB
rgbldk lsps2 rgb-quote --lsp <lsp_pubkey> --contract-id <contract_id> \
  --asset-amount 100 --lsp-asset-balance 150 --lsp-balance-sat 200000
rgbldk lsps2 rgb-invoice --lsp <lsp_pubkey> --contract-id <contract_id> \
  --asset-amount 100 --lsp-asset-balance 150 --lsp-balance-sat 200000 --max-fee-sat 1617
```

- BTC opening fee = `max(amount × proportional_ppm / 1,000,000, min_fee_msat)`,
  rounded up. Omit `--amount-msat` on `invoice` for a variable-amount invoice.
  `--max-fee` is then the highest `proportional_ppm` accepted; the LSP's
  `min_fee_msat` still applies, so a small payment can pay more than that rate.
- RGB fee = `max(min_fee_sat, onchain_cost_sat + btc_rent_sat + asset_rent_sat)`,
  with the yearly rent rates of the LSPS1 pricing applied for `min_lifetime`
  blocks. `--lsp-asset-balance` beyond `--asset-amount` stays in the channel as
  inbound capacity for later RGB payments. The payer pays
  `--btc-carrier-amount-msat` (default 354000) plus the fee; the client receives
  the carrier and the asset.
- `rgb-invoice` fails with 409 `fee_above_limit` if the LSP's fee is above
  `--max-fee-sat`; pass the `estimated_fee.fee_sat` from `rgb-quote`.

The LSP closes a JIT channel itself: if the opening fee never arrives,
`unpaid_channel_close_secs` after the offer expired (offers are valid 24 hours);
once paid, after `min_channel_lifetime` blocks, following the LSPS1
`auto_close_expired_channels` and grace settings. Closing goes through the LSPS1
service, so an LSP started without `--lsps1-service-config` closes no JIT
channel.

### Service config

Every field is optional. As with LSPS1, the file is read on every start until the
first `PUT /lsps2/options` (`rgbldk lsps2 options set --json <file>`); after that
the persisted options win, except `advertise_service`, which is always read from
the file. As with LSPS1, amounts are plain JSON numbers and an unknown field
stops the node from starting, so don't paste the `options get` output (decimal
strings) into this file.

```json
{
  "advertise_service": true
}
```

| Field | Type | Default | Meaning |
|-------|------|---------|---------|
| `require_token` | string | *(none)* | Require this token from clients. |
| `advertise_service` | bool | `false` | Announce this node as an LSP to peers. Shared with LSPS1: `true` in either file turns it on. Re-read on every start. |
| `channel_opening_fee_ppm` | u32 | `10000` | BTC JIT opening fee rate (1%). |
| `min_channel_opening_fee_msat` | u64 | `2000000` | Minimum BTC JIT opening fee. |
| `channel_over_provisioning_ppm` | u32 | `100000` | Extra BTC JIT capacity on top of the payment (10%), as inbound room for later payments. |
| `min_channel_lifetime` | u32 | `4032` | Blocks a paid channel stays open. Also the rent period of RGB JIT. |
| `max_client_to_self_delay` | u32 | `2016` | Largest `to_self_delay` a client may ask for. |
| `min_payment_size_msat` | u64 | `10000000` | Smallest BTC JIT payment. |
| `max_payment_size_msat` | u64 | `1000000000` | Largest BTC JIT payment. RGB limits come from the LSPS1 config. |
| `max_unpaid_channels_per_peer` | u32 | `1` | JIT channels one client may have open with the fee still outstanding, BTC and RGB together. |
| `max_unpaid_channels_total` | u32 | `10` | The same, across all clients. |
| `unpaid_channel_close_secs` | u64 | `86400` | Seconds after the offer expired when a channel whose fee never arrived is closed. |

Past either unpaid-channel limit, new purchases are refused. The limit is
checked again when the payment arrives, so a payment to an invoice bought
earlier can fail too.

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
