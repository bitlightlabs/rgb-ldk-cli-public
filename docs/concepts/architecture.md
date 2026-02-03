# Architecture (High Level)

RLN exposes a local HTTP control plane over `rgbldkd`, powered by an embedded node runtime (LDK-based) and wallet sync via a chain source (Esplora/Electrum depending on config).

## Components

- **rgbldkd (daemon):** HTTP server exposing `/api/v1/*` endpoints.
- **Node runtime:** manages peers, channels, payments, and an internal event queue.
- **Wallet:** on-chain balance management and transaction building/signing.
- **Chain source:** Esplora or Electrum for fee estimates and chain sync.
- **rgbldk (CLI):** convenience client calling the HTTP API.

## Data flows (simplified)

- **Invoice receive:** create invoice → wait for `PaymentReceived` event → ACK.
- **Invoice pay:** send payment → observe `PaymentSuccessful/PaymentFailed` → ACK.
- **Channel open:** open channel → wait for confirmation(s) → `ChannelReady` event → ACK.

For reliable automation, the event queue is the primary coordination surface; see [Events (ACK model)](./events-ack-model.md).
