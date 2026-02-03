# HTTP API (Reference)

This is the public-facing reference for `rgbldkd`’s HTTP JSON API.

Notes:

- Default listen: `http://127.0.0.1:8500`
- Prefix: `/api/v1/*` (legacy unprefixed routes may also exist)
- Auth: not implemented (loopback-only)

This document is based on the **main-branch** API. If you are consuming from another branch, verify types with the crate `rgbldk_api`.

## DTO Definitions (TypeScript)

> `u64/u32` are returned as JSON numbers. Frontends that need strict safety should use `BigInt` or a big-int JSON parser.

```ts
export type OkResponse = { ok: boolean };
export type ErrorResponse = { error: string };

export type StatusDto = {
  is_running: boolean;
  is_listening: boolean;
  best_block_height: number; // u32
};

export type BalancesDto = {
  total_onchain_balance_sats: number; // u64
  spendable_onchain_balance_sats: number; // u64
  total_anchor_channels_reserve_sats: number; // u64
  total_lightning_balance_sats: number; // u64
};

export type PeerDetailsDto = {
  node_id: string;
  address: string;
  is_persisted: boolean;
  is_connected: boolean;
};

export type PeerConnectRequest = { node_id: string; address: string; persist?: boolean };
export type PeerDisconnectRequest = { node_id: string };

export type OpenChannelRequest = {
  node_id: string;
  address: string;
  channel_amount_sats: number;
  push_to_counterparty_msat?: number | null;
  announce?: boolean | null;
};
export type OpenChannelResponse = { user_channel_id: string };

export type CloseChannelRequest = {
  user_channel_id: string;
  counterparty_node_id: string;
};

export type RgbChannelBalanceDto = {
  asset_id: string; // hex 32 bytes
  local_amount: number; // u64
  remote_amount: number; // u64
};

export type ChannelDetailsExtendedDto = {
  channel_id: string; // hex 32 bytes
  user_channel_id: string; // hex 16 bytes (32 chars), big-endian
  counterparty_node_id: string; // pubkey hex
  channel_point: string | null; // "txid:vout" if known
  channel_value_sats: number; // u64
  outbound_capacity_msat: number; // u64
  inbound_capacity_msat: number; // u64
  is_channel_ready: boolean;
  is_usable: boolean;
  is_announced: boolean;
  rgb_balance?: RgbChannelBalanceDto; // only present for RGB-enabled channels
};

export type Bolt11DecodeRequest = { invoice: string };
export type Bolt11DecodeResponse = {
  payment_hash: string;
  destination: string; // pubkey hex
  amount_msat: number | null;
  expiry_secs: number;
};

export type Bolt11ReceiveRequest = {
  amount_msat: number;
  description: string;
  expiry_secs: number;
};
export type Bolt11ReceiveVarRequest = { description: string; expiry_secs: number };
export type Bolt11ReceiveResponse = { invoice: string };

export type Bolt11SendRequest = { invoice: string };
export type Bolt11SendUsingAmountRequest = { invoice: string; amount_msat: number };
export type SendResponse = { payment_id: string };

// Synchronous “pay and wait” endpoint.
export type Bolt11PayRequest = { invoice: string; amount_msat?: number | null };
export type Bolt11PayResponse = {
  payment_id: string; // hex 32 bytes
  preimage: string; // hex 32 bytes
  amount_sats: number; // u64
  destination: string; // pubkey hex
  fee_paid_msat: number | null; // u64 | null
};

export type CustomTlvDto = { type: number; value_hex: string };
export type SpontaneousSendRequest = {
  counterparty_node_id: string;
  amount_msat: number;
  custom_tlvs?: CustomTlvDto[];
};

export type PaymentDetailsDto = {
  id: string;
  direction: "Inbound" | "Outbound";
  status: "Pending" | "Succeeded" | "Failed";
  amount_msat: number | null;
  kind: "Bolt11" | "Bolt11Jit" | "Bolt12Offer" | "Bolt12Refund" | "Spontaneous" | "Onchain";
  fee_paid_msat: number | null;
};

export type VersionResponse = {
  api_version: string;
  api_crate_version: string;
  core_crate_version: string;
};

export type NodeIdResponse = { node_id: string };
export type ListeningAddressesResponse = { addresses: string[] };
export type WalletNewAddressResponse = { address: string };

export type OutPointDto = { txid: string; vout: number };

export type EventDto =
  | { type: "PaymentSuccessful"; data: { payment_id: string | null; fee_paid_msat: number | null } }
  | { type: "PaymentFailed"; data: { payment_id: string | null } }
  | { type: "PaymentReceived"; data: { payment_id: string | null; amount_msat: number } }
  | { type: "ChannelPending"; data: { funding_txo: OutPointDto } }
  | { type: "ChannelReady"; data: { user_channel_id: string } }
  | { type: "ChannelClosed"; data: Record<string, never> }
  | { type: "Other"; data: { kind: string } };
```

## Error model

- `400 Bad Request`: `{ "error": "..." }`
- `404 Not Found`: `{ "error": "not found" }`

## Endpoints (Prefix: `/api/v1`)

### Health & status

- `GET /healthz` → `OkResponse`
- `GET /readyz` → `OkResponse` (200 when ready, 503 when not ready)
- `GET /version` → `VersionResponse`
- `GET /status` → `StatusDto`
- `GET /node_id` → `NodeIdResponse`
- `GET /listening_addresses` → `ListeningAddressesResponse`

### Wallet

- `POST /wallet/new_address` → `WalletNewAddressResponse`
- `POST /wallet/sync` → `OkResponse`
- `GET /balances` → `BalancesDto`

### Peers

- `GET /peers` → `PeerDetailsDto[]`
- `POST /peers/connect` (`PeerConnectRequest`) → `OkResponse`
- `POST /peers/disconnect` (`PeerDisconnectRequest`) → `OkResponse`

### Channels

- `GET /channels` → `ChannelDetailsExtendedDto[]`
- `POST /channel/open` → `OpenChannelResponse`
- `POST /channel/close` → `OkResponse`
- `POST /channel/force_close` → `OkResponse`

### Payments

- `POST /bolt11/decode` → `Bolt11DecodeResponse`
- `POST /bolt11/receive` → `Bolt11ReceiveResponse`
- `POST /bolt11/receive_var` → `Bolt11ReceiveResponse`
- `POST /bolt11/send` → `SendResponse`
- `POST /bolt11/send_using_amount` → `SendResponse`
- `POST /bolt11/pay` → `Bolt11PayResponse` (waits for completion)
- `POST /spontaneous/send` → `SendResponse`
- `GET /payment/:payment_id` → `PaymentDetailsDto`

### Events

- `POST /events/wait_next` → `EventDto` (long-poll)
- `POST /events/handled` → `OkResponse`

For semantics, see [Integration patterns](./integration-patterns.md) and [Events (ACK model)](../concepts/events-ack-model.md).
