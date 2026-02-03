# Logging & Observability

## Logging

`rgbldkd` supports `--log-level <off|error|warn|info|debug|trace>` and `--log-to-stdout`.

Recommended for local dev:

```bash
rgbldkd server --log-to-stdout --log-level info
```

## Health endpoints

- `GET /api/v1/healthz` for “process is alive”
- `GET /api/v1/readyz` for “node runtime is ready”

## Suggested metrics (future)

RLN does not currently expose Prometheus metrics on main. If you need observability, treat:

- logs as the primary signal
- health/ready as liveness gates
- events as the workflow stream

