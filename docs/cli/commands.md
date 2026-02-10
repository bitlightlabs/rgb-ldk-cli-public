# CLI Commands (Reference)

This page mirrors the current command tree in `crates/cli/src/main.rs` on the main branch.

## Global flags

- `--connect <url>`: daemon base URL (default `http://127.0.0.1:8500`)
- `--output auto|text|json`: output mode (default `auto`)
  - `auto`: text if interactive TTY; json otherwise
- `--pretty`: pretty-print JSON output (only for json output)
- `--color auto|always|never`: color mode (default `auto`)
- `--yes`: assume yes for prompts (destructive actions)
- `--no-truncate`: do not truncate long IDs in tables

## Ctx

- `rgbldk ctx ls`
- `rgbldk ctx show`
- `rgbldk ctx add <name> --url <url> [--use-now]`
- `rgbldk ctx use <name>`
- `rgbldk ctx rm <name>`

## Node

- `rgbldk node version`
- `rgbldk node health`
- `rgbldk node ready`
- `rgbldk node status`
- `rgbldk node id`
- `rgbldk node listen`

## Wallet

- `rgbldk wallet balance [--sats]`
- `rgbldk wallet address`
- `rgbldk wallet sync`

## Peer

- `rgbldk peer ls`
- `rgbldk peer connect <node_id> <addr> [--persist]`
- `rgbldk peer disconnect <node_id>`

## Channel

- `rgbldk channel ls`
- `rgbldk channel open --node-id <pubkey> --addr <host:port> --amount-sats <u64> [--push-msat <u64>] [--private]`
- `rgbldk channel close --user-channel-id <hex> --counterparty-node-id <pubkey>`
- `rgbldk channel force-close --user-channel-id <hex> --counterparty-node-id <pubkey>`

## Pay

### BOLT11 (invoices)

- `rgbldk pay invoice create --desc <string> [--amount-msat <u64>] [--expiry-secs <u32>]`
- `rgbldk pay invoice pay --invoice <bolt11> [--amount-msat <u64>]`

### BOLT12 (offers)

- `rgbldk pay offer create --desc <string> [--amount-msat <u64>] [--expiry-secs <u32> | --no-expiry] [--quantity <u64>]`
- `rgbldk pay offer decode --offer <lno...>`
- `rgbldk pay offer pay --offer <lno...> [--amount-msat <u64>] [--quantity <u64>] [--payer-note <string>]`

### BOLT12 (refunds)

- `rgbldk pay refund initiate --amount-msat <u64> [--expiry-secs <u32>] [--quantity <u64>] [--payer-note <string>]`
- `rgbldk pay refund decode --refund <lnr...>`
- `rgbldk pay refund request-payment --refund <lnr...>`

### Keysend

- `rgbldk pay keysend send --node-id <pubkey> --amount-msat <u64> [--tlv <type:hex> ...]`

### Payment tracking

- `rgbldk pay ls`
- `rgbldk pay get <payment_id_hex>`
- `rgbldk pay wait <payment_id_hex> [--timeout-secs <u32>]`
- `rgbldk pay abandon <payment_id_hex>`

## Events

- `rgbldk events next`
- `rgbldk events handled`
- `rgbldk events watch [--count <u64>]`

For event semantics, see [Events (ACK model)](../concepts/events-ack-model.md).
