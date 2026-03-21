import test from "node:test";
import assert from "node:assert/strict";

import { HttpError } from "../dist/client.js";
import { ControlHttpClient } from "../dist/control-http.js";

test("control http client sends bearer token and parses ok response", async () => {
  let auth;
  const fetch = async (url, init) => {
    auth = init.headers.Authorization;
    assert.equal(url, "http://127.0.0.1:8550/control/status");
    return new Response(JSON.stringify({ ok: true, locked: true, running: false }), { status: 200 });
  };
  const c = new ControlHttpClient("http://127.0.0.1:8550", "tkn", { fetch });
  const s = await c.status();
  assert.equal(auth, "Bearer tkn");
  assert.equal(s.locked, true);
});

test("control http client throws HttpError on 401", async () => {
  const fetch = async () =>
    new Response(JSON.stringify({ ok: false, error: "unauthorized" }), { status: 401 });
  const c = new ControlHttpClient("http://127.0.0.1:8550", "bad", { fetch });
  await assert.rejects(
    () => c.status(),
    (e) => e instanceof HttpError && e.status === 401,
  );
});

test("control http client can unlock without passphrase body", async () => {
  let body;
  const fetch = async (_url, init) => {
    body = init.body;
    return new Response(JSON.stringify({ ok: true, locked: false, running: true }), { status: 200 });
  };
  const c = new ControlHttpClient("http://127.0.0.1:8550", "tkn", { fetch });
  const r = await c.unlockUsingServerSecret();
  assert.equal(body, "{}");
  assert.equal(r.running, true);
});
