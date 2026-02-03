# Versioning & Upgrades

RLN is pre-release. Expect changes to CLI output and HTTP API over time.

## Recommendations for integrators

- Pin a git tag or commit SHA for deployments.
- Read `GET /api/v1/version` at startup and log it.
- Treat the HTTP API as the source of truth; avoid scraping CLI output.

## Recommendations for contributors

- When changing DTOs, update:
  - [HTTP API reference](../api/http-api.md)
  - CLI help and examples
