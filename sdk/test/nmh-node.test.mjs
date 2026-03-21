import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { NmhClient } from "../dist/nmh.js";
import { createNodeNativeMessagingTransport } from "../dist/nmh-node.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const fixture = path.join(__dirname, "fixtures", "fake-nmh.mjs");

test("node transport can call version()", async () => {
  const t = createNodeNativeMessagingTransport(process.execPath, [fixture]);
  const c = new NmhClient(t);
  try {
    const v = await c.version({ timeoutMs: 2000 });
    assert.equal(v.host, "fake-nmh");
  } finally {
    c.close();
  }
});

test("node transport unlock() propagates ok=false as error", async () => {
  const t = createNodeNativeMessagingTransport(process.execPath, [fixture]);
  const c = new NmhClient(t);
  try {
    await assert.rejects(() => c.unlock("wrong", { timeoutMs: 2000 }), /bad passphrase/);
  } finally {
    c.close();
  }
});

test("node transport unlock() works with correct passphrase", async () => {
  const t = createNodeNativeMessagingTransport(process.execPath, [fixture]);
  const c = new NmhClient(t);
  try {
    const r = await c.unlock("correct", { timeoutMs: 2000 });
    assert.equal(r.ok, true);
    assert.equal(r.running, true);
  } finally {
    c.close();
  }
});
