# Health vs Readiness

RLN exposes two different “is it up?” checks:

- **Health (`/healthz`)**: “Is the HTTP server responding?”
- **Readiness (`/readyz`)**: “Is the node runtime ready to operate?”

## Why two endpoints?

- A process can be alive (health is OK) while still initializing (not ready).
- Integrators should gate workflows on readiness (or on specific events), not on health.

## Suggested usage

- Use `healthz` for container/process supervision.
- Use `readyz` for “should my app send commands now?”

