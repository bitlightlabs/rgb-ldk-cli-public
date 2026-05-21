import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, writeFile, readFile, rm, mkdir } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import net from "node:net";
import { spawn } from "node:child_process";

import { ControlHttpClient, HttpError, NodeHttpClient, u64 } from "../dist/index.js";

const RGBLDKD_BIN = process.env.RGBLDKD_BIN;
const BITCOIND_BIN = process.env.BITCOIND_BIN || "bitcoind";
const DOCKER_BIN = process.env.DOCKER_BIN || "docker";

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

async function sleep(ms) {
  await new Promise((r) => setTimeout(r, ms));
}

async function waitUntil(fn, { timeoutMs = 60_000, intervalMs = 250 } = {}) {
  const deadline = Date.now() + timeoutMs;
  // eslint-disable-next-line no-constant-condition
  while (true) {
    try {
      return await fn();
    } catch (e) {
      if (Date.now() > deadline) throw e;
      await sleep(intervalMs);
    }
  }
}

async function rpcCall({ url, username, password, method, params = [] }) {
  const body = { jsonrpc: "1.0", id: "ts-sdk", method, params };
  const resp = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Basic ${Buffer.from(`${username}:${password}`).toString("base64")}`,
    },
    body: JSON.stringify(body),
  });
  const json = await resp.json();
  if (json.error) throw new Error(`bitcoind rpc error: ${JSON.stringify(json.error)}`);
  return json.result;
}

async function waitElectrsUtxo(esploraUrl, address, { timeoutMs = 120_000 } = {}) {
  await waitUntil(
    async () => {
      const r = await fetch(`${esploraUrl}/address/${address}/utxo`);
      if (!r.ok) throw new Error(`electrs address utxo not ok: ${r.status}`);
      const utxos = await r.json();
      if (!Array.isArray(utxos) || utxos.length === 0) throw new Error("no utxos yet");
      return utxos;
    },
    { timeoutMs, intervalMs: 500 },
  );
}

async function waitElectrsTx(esploraUrl, txid, { timeoutMs = 120_000 } = {}) {
  await waitUntil(
    async () => {
      const r = await fetch(`${esploraUrl}/tx/${txid}`);
      if (!r.ok) throw new Error(`electrs tx not ok: ${r.status}`);
      return r;
    },
    { timeoutMs, intervalMs: 500 },
  );
}

async function waitElectrsTxConfirmed(esploraUrl, txid, { timeoutMs = 120_000 } = {}) {
  await waitUntil(
    async () => {
      const r = await fetch(`${esploraUrl}/tx/${txid}/status`);
      if (!r.ok) throw new Error(`electrs tx status not ok: ${r.status}`);
      const st = await r.json();
      if (!st || st.confirmed !== true) throw new Error("tx not confirmed yet");
      return st;
    },
    { timeoutMs, intervalMs: 500 },
  );
}

async function waitForEventType(nodeApi, type, { timeoutMs = 120_000 } = {}) {
  const deadline = Date.now() + timeoutMs;
  // eslint-disable-next-line no-constant-condition
  while (true) {
    const remaining = deadline - Date.now();
    if (remaining <= 0) throw new Error(`timed out waiting for event ${type}`);
    let ev;
    try {
      ev = await nodeApi.eventsWaitNext({ timeoutMs: Math.min(30_000, remaining) });
    } catch (e) {
      // `events/wait_next` is a long-poll endpoint; treat client-side timeouts as "no event yet".
      if (e && typeof e === "object" && "message" in e && e.message === "Request aborted") {
        continue;
      }
      throw e;
    }
    await nodeApi.eventsHandled();
    if (ev?.type === type) return ev;
  }
}

async function waitForAnyEventType(nodeApi, types, { timeoutMs = 120_000 } = {}) {
  const want = new Set(types);
  const deadline = Date.now() + timeoutMs;
  // eslint-disable-next-line no-constant-condition
  while (true) {
    const remaining = deadline - Date.now();
    if (remaining <= 0) throw new Error(`timed out waiting for events: ${types.join(", ")}`);
    let ev;
    try {
      ev = await nodeApi.eventsWaitNext({ timeoutMs: Math.min(30_000, remaining) });
    } catch (e) {
      if (e && typeof e === "object" && "message" in e && e.message === "Request aborted") {
        continue;
      }
      throw e;
    }
    await nodeApi.eventsHandled();
    if (ev?.type && want.has(ev.type)) return ev;
  }
}

async function ensurePeerConnected(nodeApi, nodeId, address) {
  await nodeApi.peersConnect({ node_id: nodeId, address, persist: true }).catch(() => {});
  await waitUntil(async () => {
    const peers = await nodeApi.peers();
    const p = peers.find((x) => x.node_id === nodeId);
    if (!p || !p.is_connected) throw new Error("peer not connected yet");
  }, { timeoutMs: 60_000, intervalMs: 500 });
}

async function startBitcoind({ workDir }) {
  const rpcPort = await getFreePort();
  const username = "rpcuser";
  const password = "rpcpass";
  const rpcUrl = `http://127.0.0.1:${rpcPort}/`;
  const datadir = join(workDir, "bitcoind");
  await mkdir(datadir, { recursive: true });

  const child = spawn(
    BITCOIND_BIN,
    [
      "-regtest",
      `-datadir=${datadir}`,
      "-server=1",
      "-listen=0",
      "-txindex=1",
      "-rest=1",
      // Allow Dockerized indexers (electrs) to connect from the host VM network.
      `-rpcbind=0.0.0.0`,
      `-rpcallowip=0.0.0.0/0`,
      `-rpcport=${rpcPort}`,
      `-rpcuser=${username}`,
      `-rpcpassword=${password}`,
      "-fallbackfee=0.0002",
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
      { timeoutMs: 5_000, intervalMs: 50 },
    ).catch(() => {});
    if (child.exitCode === null) child.kill("SIGKILL");
  };

  const ctx = { url: rpcUrl, username, password };

  try {
    await waitUntil(
      async () => {
        await rpcCall({ ...ctx, method: "getblockchaininfo" });
      },
      { timeoutMs: 20_000, intervalMs: 200 },
    );

    // Create a wallet for mining/spending.
    await rpcCall({ ...ctx, method: "createwallet", params: ["wallet"] }).catch(() => {});
    const miningAddr = await rpcCall({ ...ctx, method: "getnewaddress", params: ["miner"] });
    await rpcCall({ ...ctx, method: "generatetoaddress", params: [101, miningAddr] });

    return {
      rpcHostPort: `127.0.0.1:${rpcPort}`,
      dockerRpcHostPort: `host.docker.internal:${rpcPort}`,
      rpcUsername: username,
      rpcPassword: password,
      rpc: ctx,
      miningAddr,
      stop,
      stderr: () => stderr,
    };
  } catch (e) {
    await stop();
    e.message = `${e.message}\n\nbitcoind stderr:\n${stderr}`;
    throw e;
  }
}

async function startElectrs({ workDir, bitcoind }) {
  const esploraPort = await getFreePort();
  const name = `rgbldk-sdk-electrs-${process.pid}-${esploraPort}`;
  const dbDir = join(workDir, "electrs-db");
  await mkdir(dbDir, { recursive: true });

  const child = spawn(
    DOCKER_BIN,
    [
      "run",
      "--rm",
      "--name",
      name,
      "-p",
      `${esploraPort}:3002`,
      "-v",
      `${dbDir}:/data`,
      "mempool/electrs:latest",
      "-v",
      "--network",
      "regtest",
      "--jsonrpc-import",
      "--daemon-rpc-addr",
      bitcoind.dockerRpcHostPort,
      "--cookie",
      `${bitcoind.rpcUsername}:${bitcoind.rpcPassword}`,
      "--db-dir",
      "/data/db",
      "--daemon-dir",
      "/data/daemon",
      "--http-addr",
      "0.0.0.0:3002",
      "--electrum-rpc-addr",
      "0.0.0.0:60401",
    ],
    { stdio: ["ignore", "pipe", "pipe"] },
  );

  let stdout = "";
  let stderr = "";
  child.stdout?.setEncoding("utf8");
  child.stdout?.on("data", (s) => {
    stdout += s;
  });
  child.stderr?.setEncoding("utf8");
  child.stderr?.on("data", (s) => {
    stderr += s;
  });

  const stop = async () => {
    spawn(DOCKER_BIN, ["rm", "-f", name], { stdio: "ignore" });
    if (child.exitCode !== null) return;
    child.kill("SIGINT");
    await waitUntil(
      async () => {
        if (child.exitCode === null) throw new Error("still running");
      },
      { timeoutMs: 10_000, intervalMs: 100 },
    ).catch(() => {});
    if (child.exitCode === null) child.kill("SIGKILL");
  };

  const baseUrl = `http://127.0.0.1:${esploraPort}`;

  try {
    await waitUntil(
      async () => {
        const r = await fetch(`${baseUrl}/blocks/tip/height`);
        if (!r.ok) throw new Error(`electrs http not ok: ${r.status}`);
        const txt = await r.text();
        const height = Number(txt.trim());
        if (!Number.isFinite(height)) throw new Error("invalid tip height");
        return height;
      },
      { timeoutMs: 120_000, intervalMs: 500 },
    );
    return {
      esploraUrl: baseUrl,
      stop,
      logs: () => `electrs stdout:\n${stdout}\n\nelectrs stderr:\n${stderr}`,
    };
  } catch (e) {
    await stop();
    e.message = `${e.message}\n\n${stdout}\n\n${stderr}`;
    throw e;
  }
}

async function startRgbldkdNode({
  workDir,
  name,
  esploraUrl,
  apiPort,
  controlPort,
  p2pPort,
}) {
  const dataDir = join(workDir, name);
  const passphraseFile = join(workDir, `${name}.passphrase.txt`);
  await writeFile(passphraseFile, `0123456789abcdef-passphrase-${name}\n`, "utf8");

  const controlToken = `0123456789abcdef-control-${name}`;

  const child = spawn(
    RGBLDKD_BIN,
    [
      "server",
      "--network",
      "regtest",
      "--listen",
      `127.0.0.1:${apiPort}`,
      "--control-http-listen",
      `127.0.0.1:${controlPort}`,
      "--control-http-allow-unlock",
      "--control-http-allow-lock",
      "--control-http-token",
      controlToken,
      "--data-dir",
      dataDir,
      "--keystore-passphrase-file",
      passphraseFile,
      "--auto-init-keystore",
      "--rgb-enabled",
      "--esplora-url",
      esploraUrl,
      "--ldk-listen",
      `127.0.0.1:${p2pPort}`,
      "--log-to-stdout",
      "--log-level",
      "info",
    ],
    { stdio: ["ignore", "pipe", "pipe"] },
  );

  let stdout = "";
  let stderr = "";
  child.stdout?.setEncoding("utf8");
  child.stdout?.on("data", (s) => {
    stdout += s;
  });
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
      { timeoutMs: 5_000, intervalMs: 50 },
    ).catch(() => {});
    if (child.exitCode === null) child.kill("SIGKILL");
  };

  const api = new NodeHttpClient(`http://127.0.0.1:${apiPort}/api/v1`);
  const control = new ControlHttpClient(`http://127.0.0.1:${controlPort}`, controlToken);

  return {
    apiPort,
    controlPort,
    p2pPort,
    api,
    control,
    stop,
    logs: () => `stdout:\n${stdout}\n\nstderr:\n${stderr}`,
  };
}

function u64ToBigInt(v) {
  if (typeof v === "bigint") return v;
  if (typeof v === "number") return BigInt(v);
  if (typeof v === "string") return BigInt(v);
  if (v && typeof v === "object" && "toString" in v) return BigInt(v.toString());
  throw new Error(`unexpected u64-like value: ${String(v)}`);
}

async function mine(bitcoind, blocks) {
  await rpcCall({ ...bitcoind.rpc, method: "generatetoaddress", params: [blocks, bitcoind.miningAddr] });
}

async function fund(bitcoind, address, sats) {
  const btc = sats / 1e8;
  await rpcCall({ ...bitcoind.rpc, method: "sendtoaddress", params: [address, btc] });
}

async function blockCount(bitcoind) {
  return rpcCall({ ...bitcoind.rpc, method: "getblockcount" });
}

function isHttpStatus(e, status) {
  return e instanceof HttpError && e.status === status;
}

test(
  "e2e: open RGB channel -> multiple RGB LN payments -> close channel (amount correctness)",
  {
    skip: !(RGBLDKD_BIN && process.env.RGBLDK_E2E),
    timeout: 5 * 60_000,
  },
  async () => {
    const workDir = await mkdtemp(join(tmpdir(), "rgbldk-sdk-e2e-rgb-"));
    const bitcoind = await startBitcoind({ workDir });
    const electrs = await startElectrs({ workDir, bitcoind });

    const nodeA = await startRgbldkdNode({
      workDir,
      name: "A",
      esploraUrl: electrs.esploraUrl,
      apiPort: await getFreePort(),
      controlPort: await getFreePort(),
      p2pPort: await getFreePort(),
    });
    const nodeB = await startRgbldkdNode({
      workDir,
      name: "B",
      esploraUrl: electrs.esploraUrl,
      apiPort: await getFreePort(),
      controlPort: await getFreePort(),
      p2pPort: await getFreePort(),
    });

    const stopAll = async () => {
      await Promise.allSettled([nodeA.stop(), nodeB.stop(), electrs.stop(), bitcoind.stop()]);
      await rm(workDir, { recursive: true, force: true });
    };

    try {
      // Wait until both daemons serve locked status on main API.
      await waitUntil(async () => {
        const s = await nodeA.api.status();
        assert.equal(s.locked, true);
      });
      await waitUntil(async () => {
        const s = await nodeB.api.status();
        assert.equal(s.locked, true);
      });

      // Unlock both nodes via control HTTP (server-managed passphrase file).
      await nodeA.control.unlockUsingServerSecret();
      await nodeB.control.unlockUsingServerSecret();

      // Wait until main API becomes unlocked and usable.
      await waitUntil(async () => {
        const s = await nodeA.api.statusUnlocked();
        assert.ok("is_running" in s);
      });
      await waitUntil(async () => {
        const s = await nodeB.api.statusUnlocked();
        assert.ok("is_running" in s);
      });

      // Fund wallets.
      const addrA = await nodeA.api.walletNewAddress();
      const rgbAddrA = await nodeA.api.rgbNewAddress();
      const addrB = await nodeB.api.walletNewAddress();
      const rgbAddrB = await nodeB.api.rgbNewAddress();
      await fund(bitcoind, addrA.address, 200_000_000); // 2 BTC (LN wallet)
      await fund(bitcoind, rgbAddrA.address, 50_000_000); // 0.5 BTC (RGB wallet)
      await fund(bitcoind, addrB.address, 200_000_000); // 2 BTC (LN wallet)
      await fund(bitcoind, rgbAddrB.address, 50_000_000); // 0.5 BTC (RGB wallet)
      await mine(bitcoind, 6);

      // Wait for electrs to index our funding transactions.
      await waitElectrsUtxo(electrs.esploraUrl, addrA.address);
      await waitElectrsUtxo(electrs.esploraUrl, rgbAddrA.address);
      await waitElectrsUtxo(electrs.esploraUrl, addrB.address);
      await waitElectrsUtxo(electrs.esploraUrl, rgbAddrB.address);

      await waitUntil(() => nodeA.api.walletSync(), { timeoutMs: 120_000 });
      await waitUntil(() => nodeB.api.walletSync(), { timeoutMs: 120_000 });

      // RGB runtime sync.
      await waitUntil(() => nodeA.api.rgbSync(), { timeoutMs: 120_000 });
      await waitUntil(() => nodeB.api.rgbSync(), { timeoutMs: 120_000 });

      // Assert the LN wallets actually have spendable funds before proceeding.
      await waitUntil(async () => {
        const b = await nodeA.api.balances();
        assert.ok(u64ToBigInt(b.btc.onchain_spendable_sats) > 0n);
      });
      await waitUntil(async () => {
        const b = await nodeB.api.balances();
        assert.ok(u64ToBigInt(b.btc.onchain_spendable_sats) > 0n);
      });

      // Import issuer (required for contract issuance).
      const issuerName = "RGB20-Simplest-v0-rLosfg";
      const issuerBytes = await readFile(
        new URL("./fixtures/issuers/RGB20-Simplest-v0-rLosfg.issuer", import.meta.url),
      );
      const issuerA = await nodeA.api.rgbIssuersImport(issuerName, issuerBytes, "raw");
      assert.equal(issuerA.ok, true);
      const issuerB = await nodeB.api.rgbIssuersImport(issuerName, issuerBytes, "raw");
      assert.equal(issuerB.ok, true);

      // Issue an RGB contract on A.
      const issue = await waitUntil(
        async () => {
          await nodeA.api.rgbSync();
          return nodeA.api.rgbContractsIssue({
            issuer_name: issuerName,
            contract_name: "TEST",
            ticker: "TST",
            precision: 0,
            issued_supply: u64(1_000),
          });
        },
        { timeoutMs: 120_000, intervalMs: 1000 },
      );
      assert.equal(issue.ok, true);
      const { contract_id: contractId } = issue;

      await nodeA.api.rgbSync();

      // Share contract to B via export+download+import.
      const exportResp = await nodeA.api.rgbContractsExport({ contract_id: contractId });
      assert.equal(exportResp.ok, true);
      const archive = await nodeA.api.rgbConsignmentDownload(exportResp.consignment_key, "raw");
      const importResp = await nodeB.api.rgbContractsImport(contractId, archive, "raw");
      assert.equal(importResp.ok, true);

      await nodeB.api.rgbSync();
      await waitUntil(() => nodeA.api.walletSync(), { timeoutMs: 60_000 });
      await waitUntil(() => nodeB.api.walletSync(), { timeoutMs: 60_000 });

      // Open RGB-enabled channel from A -> B.
      const nodeBId = (await nodeB.api.nodeId()).node_id;
      const nodeBListen = (await nodeB.api.listeningAddresses()).addresses[0];
      assert.ok(nodeBListen, "nodeB must have at least one listening address");

      const nodeAId = (await nodeA.api.nodeId()).node_id;
      const nodeAListen = (await nodeA.api.listeningAddresses()).addresses[0];
      assert.ok(nodeAListen, "nodeA must have at least one listening address");

      await nodeA.api.peersConnect({ node_id: nodeBId, address: nodeBListen, persist: true });
      await nodeB.api.peersConnect({ node_id: nodeAId, address: nodeAListen, persist: true });

      const colorContext = `http://127.0.0.1:${nodeA.apiPort}/api/v1/rgb/consignments/{txid}?format=zip`;
      const open = await nodeA.api.channelOpen({
        node_id: nodeBId,
        address: nodeBListen,
        channel_amount_sats: u64(1_000_000),
        announce: false,
        rgb: {
          contract_id: contractId,
          asset_amount: u64(500),
          color_context_data: colorContext,
        },
      });

      // Drain events so the channel handshake can progress (required in default raw events mode).
      const evPendingA = await waitForEventType(nodeA.api, "ChannelPending", { timeoutMs: 120_000 });
      const evPendingB = await waitForEventType(nodeB.api, "ChannelPending", { timeoutMs: 120_000 });
      const fundingTxid = evPendingA.data.funding_txo.txid;
      assert.ok(fundingTxid, "missing funding txid");
      assert.ok(evPendingB.data.funding_txo.txid, "missing funding txid on nodeB");

      await waitElectrsTx(electrs.esploraUrl, fundingTxid, { timeoutMs: 120_000 });

      // Mine confirmations for funding tx.
      await mine(bitcoind, 6);

      await waitElectrsTxConfirmed(electrs.esploraUrl, fundingTxid, { timeoutMs: 120_000 });

      // Wait for both nodes to catch up to chain tip.
      const tip = await blockCount(bitcoind);
      await waitUntil(async () => {
        const s = await nodeA.api.statusUnlocked();
        assert.ok(s.best_block_height >= tip);
      }, { timeoutMs: 180_000, intervalMs: 1000 });
      await waitUntil(async () => {
        const s = await nodeB.api.statusUnlocked();
        assert.ok(s.best_block_height >= tip);
      }, { timeoutMs: 180_000, intervalMs: 1000 });

      await waitUntil(() => nodeA.api.walletSync(), { timeoutMs: 120_000 });
      await waitUntil(() => nodeB.api.walletSync(), { timeoutMs: 120_000 });

      const evReadyA = await waitForEventType(nodeA.api, "ChannelReady", { timeoutMs: 180_000 });
      const evReadyB = await waitForEventType(nodeB.api, "ChannelReady", { timeoutMs: 180_000 });
      assert.equal(evReadyA.data.user_channel_id, open.user_channel_id);
      assert.ok(evReadyB.data.user_channel_id);

      // Assert channel is usable on both nodes.
      const chA = (await nodeA.api.channels()).find((c) => c.user_channel_id === open.user_channel_id);
      assert.ok(chA, "channel not found on nodeA");
      const chB = (await nodeB.api.channels()).find((c) => c.channel_id === chA.channel_id);
      assert.ok(chB, "channel not found on nodeB");
      assert.equal(chA.is_channel_ready, true);
      assert.equal(chA.is_usable, true);
      assert.equal(chB.is_channel_ready, true);

      await ensurePeerConnected(nodeA.api, nodeBId, nodeBListen);
      await ensurePeerConnected(nodeB.api, nodeAId, nodeAListen);

      // Baseline balances.
      await nodeA.api.rgbSync();
      await nodeB.api.rgbSync();
      const balA0 = await nodeA.api.rgbContractBalance(contractId);
      const balB0 = await nodeB.api.rgbContractBalance(contractId);
      const a0 = u64ToBigInt(balA0.balance.total);
      const b0 = u64ToBigInt(balB0.balance.total);
      assert.ok(a0 + b0 > 0n, "expected some initial asset balance");

      const chA0 = (await nodeA.api.channels()).find((c) => c.user_channel_id === open.user_channel_id);
      assert.ok(chA0?.rgb_balance, "expected rgb_balance on nodeA channel");
      const chB0 = (await nodeB.api.channels()).find((c) => c.channel_id === chA0.channel_id);
      assert.ok(chB0?.rgb_balance, "expected rgb_balance on nodeB channel");
      const chanALocal0 = u64ToBigInt(chA0.rgb_balance.local_amount);
      const chanBLocal0 = u64ToBigInt(chB0.rgb_balance.local_amount);
      const chanTotal0 = chanALocal0 + chanBLocal0;

      // Multiple RGB LN payments A -> B.
      const sends = [100n, 50n, 25n];
      let sentTotal = 0n;
      for (const amt of sends) {
        await ensurePeerConnected(nodeA.api, nodeBId, nodeBListen);
        await ensurePeerConnected(nodeB.api, nodeAId, nodeAListen);
        await nodeA.api.rgbSync();
        await nodeB.api.rgbSync();

        const inv = await nodeB.api.rgbLnInvoiceCreate({
          contract_id: contractId,
          asset_amount: u64(Number(amt)),
          description: `rgb ln ${amt.toString()}`,
          expiry_secs: 3600,
          btc_carrier_amount_msat: u64(5_000_000),
        });
        const pay = await nodeA.api.rgbLnPay({ invoice: inv.invoice });
        const [recvEv, outcome] = await Promise.all([
          waitForEventType(nodeB.api, "PaymentReceived", { timeoutMs: 180_000 }),
          waitForAnyEventType(nodeA.api, ["PaymentSuccessful", "PaymentFailed"], { timeoutMs: 180_000 }),
        ]);
        assert.equal(outcome.type, "PaymentSuccessful");
        assert.ok(recvEv.data.rgb, "expected rgb context on PaymentReceived");
        assert.equal(recvEv.data.rgb.contract_id, contractId);
        sentTotal += amt;

        await nodeA.api.rgbSync();
        await nodeB.api.rgbSync();

        await waitUntil(async () => {
          const chA = (await nodeA.api.channels()).find((c) => c.user_channel_id === open.user_channel_id);
          assert.ok(chA?.rgb_balance);
          const chB = (await nodeB.api.channels()).find((c) => c.channel_id === chA.channel_id);
          assert.ok(chB?.rgb_balance);
          const aLocal = u64ToBigInt(chA.rgb_balance.local_amount);
          const bLocal = u64ToBigInt(chB.rgb_balance.local_amount);
          assert.equal(bLocal, chanBLocal0 + sentTotal);
          assert.equal(aLocal + bLocal, chanTotal0);
        }, { timeoutMs: 60_000, intervalMs: 500 });
      }

      // Close channel.
      await nodeA.api.channelClose({ user_channel_id: open.user_channel_id, counterparty_node_id: nodeBId });
      await mine(bitcoind, 6);

      const evClosedA = await waitForEventType(nodeA.api, "ChannelClosed", { timeoutMs: 180_000 });
      const evClosedB = await waitForEventType(nodeB.api, "ChannelClosed", { timeoutMs: 180_000 });
      assert.ok(evClosedA.data.channel_id);
      assert.ok(evClosedB.data.channel_id);

      await waitUntil(async () => {
        const chsA = await nodeA.api.channels();
        if (chsA.find((c) => c.user_channel_id === open.user_channel_id)) {
          throw new Error("channel still present on nodeA");
        }
        const chsB = await nodeB.api.channels();
        if (chsB.find((c) => c.counterparty_node_id === nodeAId)) {
          throw new Error("channel still present on nodeB");
        }
      }, { timeoutMs: 120_000, intervalMs: 1000 });

      // Contract should remain present on both nodes.
      const contractsA = await nodeA.api.rgbContracts();
      const contractsB = await nodeB.api.rgbContracts();
      assert.ok(contractsA.contracts.some((c) => c.contract_id === contractId));
      assert.ok(contractsB.contracts.some((c) => c.contract_id === contractId));

      // Sanity: calling a random endpoint without unlock should not happen; ensure lock works.
      await nodeA.control.lock();
      await assert.rejects(() => nodeA.api.walletNewAddress(), (e) => isHttpStatus(e, 423));
    } catch (e) {
      e.message = `${e.message}\n\nnodeA logs:\n${nodeA.logs()}\n\nnodeB logs:\n${nodeB.logs()}\n\n${electrs.logs()}\n\nbitcoind stderr:\n${bitcoind.stderr()}`;
      throw e;
    } finally {
      await stopAll();
    }
  },
);
