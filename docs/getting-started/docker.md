# Run `rgbldkd` via Docker (`bitlightlabs/rln-ldk-node`)

This page shows how to run the daemon from the public Docker image `bitlightlabs/rln-ldk-node`, and then connect with the CLI (`rgbldk`).

## 1) Pull the image

```bash
docker pull bitlightlabs/rln-ldk-node:latest
```

If you have a specific version tag, replace `:latest` with your tag.

## 2) Run the daemon (HTTP API on localhost)

The daemon defaults to binding `127.0.0.1:8500` (loopback). Inside a container this makes the published port unreachable, so you must pass:

- `--allow-non-loopback-listen`
- `--listen 0.0.0.0:8500`

Example (persist data in a named volume, publish API to localhost only):

```bash
docker run --rm \
  -p 127.0.0.1:8500:8500 \
  -v rgbldk-data:/home/rgbldk/.ldk-node \
  bitlightlabs/rln-ldk-node:latest \
  rgbldkd server \
  --allow-non-loopback-listen \
  --listen 0.0.0.0:8500 \
  --data-dir /home/rgbldk/.ldk-node \
  --log-to-stdout --log-level info
```

## 3) Configure a chain source

`rgbldkd server` supports exactly one of:

- `--esplora-url <url>`
- `--electrum-url <host:port>`
- `--bitcoind-rpc <host:port> --bitcoind-rpc-user <u> --bitcoind-rpc-password <p> [--bitcoind-rest <host:port>]`

Example: regtest Esplora running on your host (macOS/Windows):

```bash
docker run --rm -p 127.0.0.1:8500:8500 bitlightlabs/rln-ldk-node:latest \
  rgbldkd server \
  --allow-non-loopback-listen --listen 0.0.0.0:8500 \
  --network regtest \
  --esplora-url http://host.docker.internal:3003 \
  --log-to-stdout --log-level info
```

Example: regtest Esplora running on your host (Linux):

```bash
docker run --rm -p 127.0.0.1:8500:8500 --add-host=host.docker.internal:host-gateway \
  bitlightlabs/rln-ldk-node:latest \
  rgbldkd server \
  --allow-non-loopback-listen --listen 0.0.0.0:8500 \
  --network regtest \
  --esplora-url http://host.docker.internal:3003 \
  --log-to-stdout --log-level info
```

## 4) Verify from the host

```bash
curl -sSf http://127.0.0.1:8500/api/v1/healthz
```

Then use the CLI:

```bash
RGBLDK_CONNECT=http://127.0.0.1:8500 rgbldk status
```
