# Events: ACK Model

RLN uses an explicit ACK model for events:

1. `POST /api/v1/events/wait_next` returns the event at the head of the queue (blocking).
2. You process the event.
3. If processing succeeds, call `POST /api/v1/events/handled` to ACK.
4. Only after ACK does the daemon advance to the next event.

This provides “at least once delivery” semantics.

## Why ACK?

It prevents silent event loss:

- If your app crashes mid-processing, the next `wait_next` returns the same event again.

## TypeScript skeleton

```ts
async function eventLoop(baseUrl: string, onEvent: (ev: any) => Promise<void>) {
  while (true) {
    const ev = await fetch(`${baseUrl}/api/v1/events/wait_next`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: "{}",
    }).then(r => r.json());

    try {
      await onEvent(ev);
      await fetch(`${baseUrl}/api/v1/events/handled`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: "{}",
      });
    } catch {
      await new Promise(r => setTimeout(r, 500));
    }
  }
}
```

## Operational advice

- Run a **single** event consumer per node to avoid races.
- Make handlers idempotent (see [Integration patterns](../api/integration-patterns.md)).
