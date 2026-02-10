# rgb-ldk-cli

Tools for running and controlling an RLN node via a local HTTP JSON API.

Most users only need:

- `rgbldkd` (daemon): runs the node and serves the HTTP API.
- `rgbldk` (CLI): a terminal client for calling that API.
- TypeScript SDK: for web apps / Node.js services that want to call the API directly.

## What’s in this repo

- Rust HTTP API DTOs/types: [`crates/api`](crates/api)
- Rust CLI (`rgbldk`): [`crates/cli`](crates/cli)
- TypeScript SDK: [`sdk/`](sdk)
- Docs: [`docs/`](docs)

## Quick start (local regtest)

1) Start a local regtest environment (bitcoind + esplora + two daemons):

```bash
RPC_USER=bitcoin RPC_PASSWORD=bitcoin docker compose -f crates/cli/docker-compose.yaml up -d
```

This exposes:

- node A: `http://127.0.0.1:8501`
- node B: `http://127.0.0.1:8502`

2) Build the CLI:

```bash
cargo build -p rgbldk-cli --bin rgbldk
```

3) Call the daemon:

```bash
RGBLDK_CONNECT=http://127.0.0.1:8501 ./target/debug/rgbldk node status
RGBLDK_CONNECT=http://127.0.0.1:8501 ./target/debug/rgbldk wallet balance
```

## Docs

- Docs home: [`docs/README.md`](docs/README.md)
- Docker guide: [`docs/getting-started/docker.md`](docs/getting-started/docker.md)
- Local binary guide: [`docs/getting-started/local-binary.md`](docs/getting-started/local-binary.md)
- HTTP API reference: [`docs/api/http-api.md`](docs/api/http-api.md)
- CLI reference: [`docs/cli/README.md`](docs/cli/README.md)
- TypeScript SDK: [`sdk/README.md`](sdk/README.md)

## License

Dual-licensed under Apache-2.0 and MIT. See [`LICENSE-APACHE`](LICENSE-APACHE) and [`LICENSE-MIT`](LICENSE-MIT).
