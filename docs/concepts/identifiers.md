# Identifiers & Formats

RLN uses several IDs that appear in CLI output and API responses.

## Node ID

- A secp256k1 public key (hex, compressed form commonly 66 chars).
- Used to identify peers and destinations.

## Payment ID

- Hex-encoded 32 bytes (64 hex chars).
- Returned by payment send endpoints and used by `GET /payment/:payment_id`.

## Channel IDs

You may see both:

- `channel_id`: internal channel identifier (32 bytes hex).
- `user_channel_id`: a user-facing identifier used by close/force-close APIs (16 bytes hex, 32 chars, big-endian).

In this repo’s CLI/API, channel close operations are keyed by `user_channel_id` + `counterparty_node_id`.

