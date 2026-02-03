# Tutorial: Consume Events Reliably

This tutorial shows how to build a robust event consumer.

## Principles

- ACK only after durable processing.
- Make handlers idempotent.
- Prefer one consumer per node.

## Minimal loop (TypeScript)

See [Events (ACK model)](../concepts/events-ack-model.md) for a full skeleton.

## Suggested event routing

- `PaymentReceived` → credit user balance
- `PaymentSuccessful` → mark outgoing payment complete
- `PaymentFailed` → mark outgoing payment failed and notify
- `ChannelReady` → update channel state and enable routing
