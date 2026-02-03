# HTTP API Overview

`rgbldkd` exposes a local HTTP JSON API (intended for single-user, loopback-only use).

## Where to start

- Full reference: [HTTP API reference](./http-api.md)
- Integration patterns (recommended): [Integration patterns](./integration-patterns.md)

## Common endpoints

- `GET /api/v1/healthz` — process health
- `GET /api/v1/readyz` — readiness (node runtime running or not)
- `GET /api/v1/status` — summary state (running/listening/best height)
- `POST /api/v1/events/wait_next` + `POST /api/v1/events/handled` — event queue (ACK model)
