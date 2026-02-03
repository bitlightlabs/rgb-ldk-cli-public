# Threat Model

This is a pragmatic, product-facing threat model to help integrators deploy RLN safely.

## Assets

- Wallet keys / signing operations (on-chain funds).
- Lightning channel funds and payment state.
- Node identity and peer connectivity.
- Application state derived from events (payments, channel status).

## Trust boundaries

- **HTTP API boundary:** local machine boundary in the intended deployment.
- **Chain source boundary:** Esplora/Electrum endpoints are external dependencies.
- **Peer-to-peer boundary:** Lightning peers are remote and untrusted by default.

## Out of scope (today)

- Remote unauthenticated API exposure.
- Strong guarantees against a compromised local OS user account.
- Multi-user access control.

## Recommended mitigations

- Keep `rgbldkd` on loopback.
- Run RLN under a dedicated OS user.
- Protect data directories with filesystem permissions.
- Validate your chain source URLs and network (avoid mainnet/regtest mismatch).

