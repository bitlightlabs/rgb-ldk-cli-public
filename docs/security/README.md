# Security Model (Scope & Assumptions)

RLN is designed as a **local, single-user** node control plane.

## Current scope (main branch)

- The daemon is intended to listen on loopback (`127.0.0.1`).
- There is **no authentication** on the HTTP API.
- The API surface is meant for local developer workflows and local app integration (same machine).

## Practical implications

- Do **not** expose `rgbldkd` directly to untrusted networks.
- If you need remote access, put an authenticated reverse proxy in front (and treat it as a separate security boundary).

## What RLN tries to protect

- Correctness and safety of node operations under expected local usage.
- Reliable event delivery semantics (ACK model) so integrators can avoid silent event loss.

## What RLN does not protect (by design today)

- Multi-tenant separation.
- Network attacker scenarios against the HTTP API (since it is not designed for remote bind).

For more detail, see [Threat model](./threat-model.md).
