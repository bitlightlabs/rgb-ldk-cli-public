rgb-ldk-node-sdk
=================

TypeScript SDK for the rgb-ldk-node HTTP API. Works in modern browsers (React/Vue/etc.) and Node (with a fetch polyfill).

Install
-------

- Local build
  - cd sdk
  - npm install
  - npm run build

- Using as a dependency (after publishing)
  - npm install rgb-ldk-node-sdk

Usage
-----

    import { NodeHttpClient } from "rgb-ldk-node-sdk";

    // You can use either the root router (/) or the recommended versioned prefix (/api/v1).
    // Default rgbldkd listen: http://127.0.0.1:8500
    const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1");
    const status = await client.status();
    const { address } = await client.walletNewAddress();
    const inv = await client.bolt11Receive({ amount_msat: 1000, description: "test", expiry_secs: 600 });
    const sent = await client.bolt11Send({ invoice: inv.invoice }); // or client.bolt11Pay({ invoice: inv.invoice })
    const payment = await client.getPayment(sent.payment_id);

    // BOLT12 offer (receive + pay)
    const { offer } = await client.bolt12OfferReceive({ amount_msat: 1000, description: "coffee", expiry_secs: 600 });
    const p = await client.bolt12OfferSend({ offer });
    const waited = await client.paymentWait(p.payment_id, { timeout_secs: 60 });
    if (!waited.ok) {
      // For example, if it's awaiting an invoice, you may cancel it:
      await client.paymentAbandon(p.payment_id);
    }

Notes
-----

- The client uses global fetch by default. In Node, pass a fetch implementation: new NodeHttpClient(baseUrl, { fetch: (await import('node-fetch')).default })
- For long-polling events, you can pass timeoutMs to abort: client.eventsWaitNext({ timeoutMs: 30000 })
- Large integer values (for example `*_msat` fields) may be returned as `bigint` to avoid precision loss in JavaScript.
