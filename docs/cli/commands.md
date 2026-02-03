# CLI Commands (Reference)

This page mirrors the current command tree in `crates/cli/main.rs` on the main branch.

## Global flags

- `--connect <url>`: daemon base URL (default `http://127.0.0.1:8500`, env `RGBLDK_CONNECT`)
- `--output text|json`: output mode (default `text`)

## Node

- `rgbldk version`
- `rgbldk healthz`
- `rgbldk readyz`
- `rgbldk status`
- `rgbldk balances`

## Peers

- `rgbldk peers list`

## Wallet

- `rgbldk wallet new-address`
- `rgbldk wallet sync`

## Channels

- `rgbldk channel open --node-id <pubkey> --addr <host:port> --amount-sats <u64> [--push-msat <u64>] [--private]`
- `rgbldk channel close --user-channel-id <hex> --counterparty-node-id <pubkey>`
- `rgbldk channel force-close --user-channel-id <hex> --counterparty-node-id <pubkey>`

## Bolt11

- `rgbldk bolt11 receive --amount-msat <u64> --desc <string> [--expiry-secs <u32>]`
- `rgbldk bolt11 receive-var --desc <string> [--expiry-secs <u32>]`
- `rgbldk bolt11 send --invoice <bolt11>`
- `rgbldk bolt11 send-using-amount --invoice <bolt11> --amount-msat <u64>`

## Spontaneous (keysend)

- `rgbldk spontaneous send --node-id <pubkey> --amount-msat <u64> [--tlv <type:hex> ...]`

## Payments

- `rgbldk payment get <payment_id_hex>`

## Events

- `rgbldk events next`
- `rgbldk events handled`

For event semantics, see [Events (ACK model)](../concepts/events-ack-model.md).
