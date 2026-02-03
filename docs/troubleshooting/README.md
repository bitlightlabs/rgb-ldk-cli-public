# Troubleshooting

## CLI cannot connect

- Verify daemon is running and prints its listen URL.
- Try `curl -sSf http://127.0.0.1:8500/api/v1/healthz`.
- If you changed ports, set `RGBLDK_CONNECT`.

## `FeerateEstimationUpdateFailed` or wallet sync failures

Root causes:

- Chain source URL is wrong/unreachable.
- Network mismatch (e.g., regtest node pointing at mainnet esplora).

Actions:

- Verify Esplora endpoints:
  - `/fee-estimates`
  - `/blocks/tip/height`
- Ensure `rgbldkd --network` matches the chain source network.

## Events seem “stuck”

- If you call `events/wait_next` but never call `events/handled`, the same event will repeat forever.
- Make sure your app ACKs after successful processing.

