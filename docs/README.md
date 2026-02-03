# RLN — Public Home

RLN is a local, single-user Lightning node control plane with RGB support. It ships as:

- `rgbldkd`: the daemon (HTTP control plane), distributed as a prebuilt binary/Docker image (`bitlightlabs/rln-ldk-node`).
- `rgbldk`: the CLI client (talks to `rgbldkd` over HTTP).

These docs are written for **integrators and node operators** who want to run RLN locally (regtest/testnet) and build apps against its HTTP API.

## What You Can Do

- Run a local regtest node (daemon + chain source) and iterate quickly.
- Open channels, send/receive Bolt11 payments, and send spontaneous (keysend) payments.
- Build an app on top of a simple HTTP control plane and an explicit event queue (ACK model).

## Start Here

- **Run `rgbldkd` via Docker image:** [Docker guide](getting-started/docker.md)
- **Quickstart (regtest):** [Regtest quickstart](getting-started/regtest.md)
- **CLI overview:** [CLI overview](cli/README.md)
- **HTTP API overview:** [HTTP API overview](api/README.md)
- **Full HTTP API reference:** [HTTP API reference](api/http-api.md)

## Core Concepts

- Health vs readiness: [Health vs readiness](concepts/health-readiness.md)
- Event queue (ACK model): [Events (ACK model)](concepts/events-ack-model.md)
- Identifiers (payment IDs, channel IDs): [Identifiers](concepts/identifiers.md)
- Architecture (high level): [Architecture](concepts/architecture.md)

## Tutorials

- Open a channel end-to-end: [Open a channel](tutorials/open-a-channel.md)
- Receive a payment (invoice): [Receive a payment](tutorials/receive-a-payment.md)
- Send a payment (invoice/keysend): [Send a payment](tutorials/send-a-payment.md)
- Consume events reliably: [Consume events](tutorials/consume-events.md)

## Operations

- Troubleshooting: [Troubleshooting](troubleshooting/README.md)
- Logging & observability: [Logging & observability](ops/logging-observability.md)
- Releases & versioning: [Versioning & upgrades](releases/versioning.md)

## Security & Contributing

- Security model (scope & assumptions): [Security model](security/README.md)
- Threat model (what RLN does/does not protect against): [Threat model](security/threat-model.md)
- Contributing: [Contributing](contributing.md)
