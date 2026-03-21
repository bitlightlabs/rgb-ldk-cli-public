import test from "node:test";
import assert from "node:assert/strict";
import http from "node:http";
import { once } from "node:events";
import { Buffer } from "node:buffer";

import { ControlHttpClient, HttpError, NodeHttpClient } from "../dist/index.js";

function readBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on("data", (c) => chunks.push(c));
    req.on("end", () => resolve(Buffer.concat(chunks)));
    req.on("error", reject);
  });
}

function sendJson(res, status, obj) {
  const body = JSON.stringify(obj);
  res.writeHead(status, { "Content-Type": "application/json" });
  res.end(body);
}

function sendBytes(res, status, bytes) {
  res.writeHead(status, { "Content-Type": "application/octet-stream" });
  res.end(Buffer.from(bytes));
}

function checkBearer(req, expectedToken) {
  const h = req.headers.authorization;
  if (!h) return { ok: false, error: "missing Authorization: Bearer token" };
  const s = Array.isArray(h) ? h.join(",") : h;
  if (s !== `Bearer ${expectedToken}`) return { ok: false, error: "unauthorized" };
  return { ok: true };
}

async function startMockDaemon({ apiToken, controlToken }) {
  let locked = true;

  const server = http.createServer(async (req, res) => {
    const url = new URL(req.url ?? "/", `http://${req.headers.host ?? "127.0.0.1"}`);
    const path = url.pathname;

    if (path.startsWith("/control/")) {
      const auth = checkBearer(req, controlToken);
      if (!auth.ok) return sendJson(res, 401, { error: auth.error });

      if (req.method === "GET" && path === "/control/status") {
        return sendJson(res, 200, { ok: true, locked, running: !locked });
      }
      if (req.method === "GET" && path === "/control/version") {
        return sendJson(res, 200, { ok: true, protocol: "control-http", daemon: "rgbldkd", daemon_version: "test" });
      }
      if (req.method === "POST" && path === "/control/unlock") {
        await readBody(req);
        locked = false;
        return sendJson(res, 200, { ok: true, locked, running: !locked });
      }
      if (req.method === "POST" && path === "/control/lock") {
        const raw = await readBody(req);
        let yes = false;
        try {
          const v = JSON.parse(raw.toString("utf8") || "{}");
          yes = v?.yes === true;
        } catch {
          // ignore
        }
        if (!yes) return sendJson(res, 400, { error: "refusing to lock without yes=true" });
        locked = true;
        return sendJson(res, 200, { ok: true, locked, running: !locked });
      }

      return sendJson(res, 404, { error: "not found" });
    }

    if (path.startsWith("/api/v1/")) {
      if (path !== "/api/v1/healthz" && path !== "/api/v1/readyz") {
        const auth = checkBearer(req, apiToken);
        if (!auth.ok) return sendJson(res, 401, { error: auth.error });
      }

      if (req.method === "GET" && path === "/api/v1/healthz") {
        return sendJson(res, 200, { ok: true, checks: [] });
      }
      if (req.method === "GET" && path === "/api/v1/readyz") {
        return sendJson(res, 200, { ok: true, checks: [] });
      }
      if (req.method === "GET" && path === "/api/v1/status") {
        return sendJson(res, 200, { ok: true, locked, running: !locked, checks: [] });
      }
      if (req.method === "POST" && path === "/api/v1/rgb/contracts/import") {
        await readBody(req);
        return sendJson(res, 200, { ok: true, contract_id: "contract:dummy", consignment_key: "dummy" });
      }
      if (req.method === "GET" && path.startsWith("/api/v1/rgb/consignments/")) {
        return sendBytes(res, 200, new Uint8Array([1, 2, 3, 4]));
      }
      if (req.method === "POST" && path === "/api/v1/wallet/new_address") {
        await readBody(req);
        return sendJson(res, 200, { address: "bcrt1qexample000000000000000000000000000000000" });
      }

      return sendJson(res, 404, { error: "not found" });
    }

    return sendJson(res, 404, { error: "not found" });
  });

  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  const addr = server.address();
  const port = typeof addr === "object" && addr ? addr.port : 0;
  const baseUrl = `http://127.0.0.1:${port}`;

  return {
    baseUrl,
    close: () => new Promise((resolve) => server.close(resolve)),
  };
}

test("full workflow: control unlock + main API bearer token (json/raw/binary)", async () => {
  const apiToken = "0123456789abcdef-api";
  const controlToken = "0123456789abcdef-control";
  const daemon = await startMockDaemon({ apiToken, controlToken });
  try {
    const apiNoAuth = new NodeHttpClient(`${daemon.baseUrl}/api/v1`);
    await assert.rejects(
      () => apiNoAuth.status(),
      (e) => e instanceof HttpError && e.status === 401
    );

    const api = new NodeHttpClient(`${daemon.baseUrl}/api/v1`, { headers: { Authorization: `Bearer ${apiToken}` } });
    const control = new ControlHttpClient(daemon.baseUrl, controlToken);

    const s1 = await api.status();
    assert.equal(s1.locked, true);

    await control.unlockUsingServerSecret();
    const s2 = await api.statusUnlocked();
    assert.equal(s2.locked, false);

    // requestRaw path should also carry Authorization from the client default headers
    const importResp = await api.rgbContractsImport("contract:dummy", new Uint8Array([9, 9, 9]));
    assert.equal(importResp.ok, true);

    // requestBinary path should also carry Authorization from the client default headers
    const bytes = await api.rgbConsignmentDownload("dummy");
    assert.deepEqual(Array.from(bytes), [1, 2, 3, 4]);

    await control.lock();
    await assert.rejects(() => api.statusUnlocked(), /Daemon is locked/);
  } finally {
    await daemon.close();
  }
});

