import test from "node:test";
import assert from "node:assert/strict";

import { NmhClient, createChromeNativeMessagingTransport } from "../dist/nmh.js";

function makeEmitter() {
  const listeners = new Set();
  return {
    addListener(fn) {
      listeners.add(fn);
    },
    removeListener(fn) {
      listeners.delete(fn);
    },
    emit(msg) {
      for (const fn of [...listeners]) fn(msg);
    },
  };
}

test("chrome transport correlates id and returns result", async () => {
  const onMessage = makeEmitter();
  const onDisconnect = makeEmitter();
  const posted = [];

  const port = {
    postMessage(msg) {
      posted.push(msg);
      queueMicrotask(() => {
        onMessage.emit({
          ok: true,
          id: msg.id,
          result: msg.method === "version"
            ? { host: "rgbldk-nmh", protocol: "native-messaging/v1" }
            : { ok: true },
        });
      });
    },
    disconnect() {},
    onMessage,
    onDisconnect,
  };

  const runtime = {
    connectNative(_hostName) {
      return port;
    },
    lastError: undefined,
  };

  const transport = createChromeNativeMessagingTransport("com.bitlight.rgbldk", runtime);
  const client = new NmhClient(transport);
  const v = await client.version();
  assert.equal(typeof v.host, "string");
  assert.equal(typeof v.protocol, "string");
  assert.equal(posted[0].method, "version");
});

test("chrome transport rejects all pending on disconnect", async () => {
  const onMessage = makeEmitter();
  const onDisconnect = makeEmitter();

  const port = {
    postMessage(_msg) {},
    disconnect() {},
    onMessage,
    onDisconnect,
  };

  const runtime = {
    connectNative(_hostName) {
      return port;
    },
    lastError: { message: "boom" },
  };

  const transport = createChromeNativeMessagingTransport("com.bitlight.rgbldk", runtime);
  const p = transport.request({ method: "status", params: {} }, { timeoutMs: 1000 });
  queueMicrotask(() => onDisconnect.emit({}));
  await assert.rejects(() => p, /boom/);
});
