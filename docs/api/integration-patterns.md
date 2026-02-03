# Integration Patterns

This page describes recommended patterns for integrating with `rgbldkd` safely and predictably.

## 1) Use the event queue (ACK model) for automation

The events API is a long-poll + ACK queue:

- `POST /api/v1/events/wait_next` blocks until an event exists and returns the head of the queue.
- Until you call `POST /api/v1/events/handled`, `wait_next` returns the same event again.

This lets you implement “at least once” processing: only ACK after your app successfully handles the event.

See [Events (ACK model)](../concepts/events-ack-model.md) for a detailed model and a TypeScript skeleton.

## 2) Prefer idempotent handlers

Events can be redelivered (e.g., crash before ACK). Design your handlers to be idempotent:

- Persist a small “seen set” by `payment_id` / `user_channel_id`.
- If you receive a duplicate, treat it as success and ACK.

## 3) Use `status` for polling dashboards, not for workflows

- Dashboards can poll `GET /api/v1/status` and `GET /api/v1/balances` at a low rate.
- Workflows (payments, channel readiness) should use events.

## 4) Handle timeouts gracefully

Long polling will hold connections. For web backends:

- Set a reasonable request timeout (e.g., 30–60s).
- On timeout, retry `wait_next` immediately.

## 5) Error handling

- Treat HTTP 400 as “bug / invalid input / invalid state” and surface the error.
- Treat network errors as retryable.
