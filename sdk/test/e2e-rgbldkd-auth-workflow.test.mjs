import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import net from "node:net";
import { spawn } from "node:child_process";

import { ControlHttpClient, HttpError, NodeHttpClient } from "../dist/index.js";

async function getFreePort() {
  const server = net.createServer();
  server.unref();
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const addr = server.address();
  const port = typeof addr === "object" && addr ? addr.port : 0;
  await new Promise((resolve) => server.close(resolve));
  return port;
}

async function waitUntil(fn, { timeoutMs = 10_000, intervalMs = 100 } = {}) {
  const deadline = Date.now() + timeoutMs;
  // eslint-disable-next-line no-constant-condition
  while (true) {
    try {
      return await fn();
    } catch (e) {
      if (Date.now() > deadline) throw e;
      await new Promise((r) => setTimeout(r, intervalMs));
    }
  }
}

function isHttpErrorWithStatus(e, status) {
  return e instanceof HttpError && e.status === status;
}

const RGBLDKD_BIN = process.env.RGBLDKD_BIN;

test(
  "e2e: start rgbldkd, unlock via control http, enforce main API bearer token",
  { skip: !RGBLDKD_BIN, timeout: 30_000 },
  async () => {
    const workDir = await mkdtemp(join(tmpdir(), "rgbldk-sdk-e2e-"));
    try {
      const dataDir = join(workDir, "data");
      const passphraseFile = join(workDir, "passphrase.txt");
      const apiToken = "0123456789abcdef-api-token";
      const controlToken = "0123456789abcdef-control-token";

      await writeFile(passphraseFile, "0123456789abcdef-passphrase\n", "utf8");

      const apiPort = await getFreePort();
      const controlPort = await getFreePort();

      const child = spawn(
        RGBLDKD_BIN,
        [
          "server",
          "--listen",
          `127.0.0.1:${apiPort}`,
          "--data-dir",
          dataDir,
          "--network",
          "regtest",
          "--keystore-passphrase-file",
          passphraseFile,
          "--auto-init-keystore",
          "--control-http-listen",
          `127.0.0.1:${controlPort}`,
          "--control-http-allow-unlock",
          "--control-http-allow-lock",
          "--control-http-token",
          controlToken,
          "--http-token",
          apiToken,
        ],
        { stdio: ["ignore", "pipe", "pipe"] },
      );

      let stderr = "";
      child.stderr?.setEncoding("utf8");
      child.stderr?.on("data", (s) => {
        stderr += s;
      });

      const stop = async () => {
        if (child.exitCode !== null) return;
        child.kill("SIGINT");
        await waitUntil(
          async () => {
            if (child.exitCode === null) throw new Error("still running");
          },
          { timeoutMs: 3_000, intervalMs: 50 },
        ).catch(() => {});
        if (child.exitCode === null) child.kill("SIGKILL");
      };

      try {
        const apiNoAuth = new NodeHttpClient(`http://127.0.0.1:${apiPort}/api/v1`);
        const apiAuth = new NodeHttpClient(`http://127.0.0.1:${apiPort}/api/v1`, {
          headers: { Authorization: `Bearer ${apiToken}` },
        });
        const control = new ControlHttpClient(`http://127.0.0.1:${controlPort}`, controlToken);

        // Wait until locked HTTP is serving /status.
        const lockedStatus = await waitUntil(() => apiNoAuth.status(), { timeoutMs: 10_000 });
        assert.equal(lockedStatus.locked, true);

        await control.unlockUsingServerSecret();

        // After unlock, the main node API is live and must require the bearer token.
        await waitUntil(
          async () => {
            await assert.rejects(
              () => apiNoAuth.status(),
              (e) => isHttpErrorWithStatus(e, 401),
            );
          },
          { timeoutMs: 10_000 },
        );

        const s = await waitUntil(() => apiAuth.statusUnlocked(), { timeoutMs: 10_000 });
        assert.equal("is_running" in s, true);
        assert.notEqual(s.locked, true);

        // A representative POST should also be protected.
        await assert.rejects(
          () => apiNoAuth.walletNewAddress(),
          (e) => isHttpErrorWithStatus(e, 401),
        );
        const addr = await apiAuth.walletNewAddress();
        assert.ok(typeof addr.address === "string" && addr.address.length > 0);

        await control.lock();

        const lockedAgain = await waitUntil(() => apiNoAuth.status(), { timeoutMs: 10_000 });
        assert.equal(lockedAgain.locked, true);
      } catch (e) {
        e.message = `${e.message}\n\nrgbldkd stderr:\n${stderr}`;
        throw e;
      } finally {
        await stop();
      }
    } finally {
      await rm(workDir, { recursive: true, force: true });
    }
  },
);
