import test from "node:test";
import assert from "node:assert/strict";

import { HTTP_STATUS_LOCKED, HttpError, NodeHttpClient, isLockedHttpError } from "../dist/index.js";

test("status() returns locked shape when daemon locked", async () => {
  const fetch = async () =>
    new Response(JSON.stringify({ ok: true, locked: true, running: false, checks: [] }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1", { fetch });
  const status = await client.status();
  assert.equal(status.locked, true);
  assert.equal("is_running" in status, false);
});

test("statusUnlocked() throws when locked", async () => {
  const fetch = async () =>
    new Response(JSON.stringify({ ok: true, locked: true, running: false }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1", { fetch });
  await assert.rejects(() => client.statusUnlocked(), /Daemon is locked/);
});

test("isLockedHttpError() detects HTTP 423", async () => {
  const fetch = async () =>
    new Response(JSON.stringify({ error: "daemon is locked" }), {
      status: HTTP_STATUS_LOCKED,
      headers: { "Content-Type": "application/json" },
    });
  const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1", { fetch });
  try {
    await client.walletNewAddress();
    assert.fail("expected throw");
  } catch (e) {
    assert.ok(e instanceof HttpError);
    assert.ok(isLockedHttpError(e));
    assert.equal(e.status, HTTP_STATUS_LOCKED);
  }
});

