# Tutorial: Open a Channel (end-to-end)

This tutorial assumes:

- You have a running regtest chain source (Esplora) from [Regtest quickstart](../getting-started/regtest.md).
- You have at least two RLN nodes running (node A and node B).

## 1) Get node B’s identity and listening address

On node B:

- Read `node_id` via `GET /api/v1/node_id`.
- Ensure `GET /api/v1/listening_addresses` includes a reachable address.

## 2) Fund node A and sync wallet

On node A:

- `POST /api/v1/wallet/new_address`
- Send coins to the returned address using your regtest miner
- `POST /api/v1/wallet/sync`

## 3) Open channel

Call on node A:

`POST /api/v1/channel/open` with:

- `node_id`: node B’s pubkey hex
- `address`: node B’s `host:port`
- `channel_amount_sats`: capacity

## 4) Wait for readiness via events

Long-poll events on both nodes:

- `ChannelPending` (optional)
- `ChannelReady` (the channel is usable)

Only ACK after your app persists the state transition.

See [Consume events](./consume-events.md) for a robust event consumer.
