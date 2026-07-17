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

    import { NodeHttpClient, u64 } from "rgb-ldk-node-sdk";

    // You can use either the root router (/) or the recommended versioned prefix (/api/v1).
    // Default rgbldkd listen: http://127.0.0.1:8500
    const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1");
    const status = await client.status();
    if ("locked" in status && status.locked) {
      // The daemon is serving the locked HTTP API. Unlock via the CLI control plane:
      // rgbldk node unlock --passphrase-stdin
      console.log("daemon locked", status.checks ?? []);
      throw new Error("daemon locked");
    }
    const { address } = await client.walletNewAddress();
    const inv = await client.bolt11Receive({ amount_msat: u64(1000), description: "test", expiry_secs: 600 });
    const sent = await client.bolt11Send({ invoice: inv.invoice }); // or client.bolt11Pay({ invoice: inv.invoice })
    const payment = await client.getPayment(sent.payment_id);

    // BOLT12 offer (receive + pay)
    const { offer } = await client.bolt12OfferReceive({ amount_msat: u64(1000), description: "coffee", expiry_secs: 600 });
    const p = await client.bolt12OfferSend({ offer });
    const waited = await client.paymentWait(p.payment_id, { timeout_secs: 60 });
    if (!waited.ok) {
      // For example, if it's awaiting an invoice, you may cancel it:
      await client.paymentAbandon(p.payment_id);
    }

    // Atomic BTC/RGB swap offers are out-of-band and settle asynchronously.
    const swapOffer = await client.swapCreateOffer({
      counterparty_node_id: "<node_id>",
      channel_scid: u64(123),
      contract_id: "contract:...",
      asset_amount: u64(10),
      btc_amount_msat: u64(1000),
      btc_carrier_amount_msat: u64(330000),
      maker_gives_rgb: true,
      expiry_secs: 3600,
    });
    await client.swapAccept({ swap_string: swapOffer.swap_string });
    await client.swapExecute({ swap_string: swapOffer.swap_string });
    const swap = await client.swapGet(swapOffer.payment_hash);
    // Only `Settled` proves that the circular payment completed.

Notes
-----

- The client uses global fetch by default. In Node, pass a fetch implementation: new NodeHttpClient(baseUrl, { fetch: (await import('node-fetch')).default })
- For long-polling events, you can pass timeoutMs to abort: client.eventsWaitNext({ timeoutMs: 30000 })
- u64 fields (for example `*_msat` / `*_sats`) are represented as an opaque `U64` wrapper backed by `bigint`.
- `swapExecute` only initiates a swap; poll `swapGet` and inspect `status` (`Settled` or `Failed`) for the final result.
- When the daemon is locked, most endpoints return HTTP 423. Catch `HttpError` and check `err.status === 423` (or use `isLockedHttpError(err)`).

Native Messaging (unlock bridge)
--------------------------------

For browser extensions and desktop apps, you can call `unlock` via the Native Messaging Host `rgbldk-nmh`.

Browser (Chrome):

    import { NmhClient, createChromeNativeMessagingTransport } from "rgb-ldk-node-sdk/nmh";
    const nmh = new NmhClient(createChromeNativeMessagingTransport("com.bitlight.rgbldk"));
    await nmh.unlock("your passphrase");

Desktop (Node/Electron):

    import { NmhClient } from "rgb-ldk-node-sdk/nmh";
    import { createNodeNativeMessagingTransport } from "rgb-ldk-node-sdk/nmh-node";
    const nmh = new NmhClient(createNodeNativeMessagingTransport("/abs/path/to/rgbldk-nmh", ["--data-dir", "/tmp/ldk_node"]));
    await nmh.unlock("your passphrase");

Hosted nodes (no local CLI)
---------------------------

If the node is running on a server and users do not have access to the server's CLI, enable the daemon's authenticated HTTP control server (`--control-http-listen` + `--control-http-token`) and call it via `ControlHttpClient`:

    import { ControlHttpClient } from "rgb-ldk-node-sdk/control-http";
    const ctl = new ControlHttpClient("https://example.com:8550", "your-token");
    // If the server uses --keystore-passphrase-file, do not send passphrase from clients:
    await ctl.unlockUsingServerSecret();

The daemon requires `--control-http-allow-unlock` to enable the unlock endpoint.
