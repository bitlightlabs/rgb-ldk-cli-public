# CLI Overview (`rgbldk`)

`rgbldk` is a client CLI that talks to `rgbldkd` over HTTP. It is designed for local developer workflows.

## Connection

By default the CLI connects to `http://127.0.0.1:8500`.

Override per-command:

```bash
rgbldk --connect http://127.0.0.1:8500 status
```

Or set the default:

```bash
export RGBLDK_CONNECT=http://127.0.0.1:8500
rgbldk status
```

## Output modes

- Default output is `text` (intended for humans).
- Use JSON for automation:

```bash
rgbldk --output json status
```

## Next

- Commands reference: [Commands reference](./commands.md)
- Examples: [Examples](./examples.md)
