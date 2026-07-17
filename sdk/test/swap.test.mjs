import test from "node:test";
import assert from "node:assert/strict";

import { NodeHttpClient } from "../dist/client.js";
import { u64 } from "../dist/u64.js";

const info = {
  payment_hash: "11".repeat(32),
  role: "Maker",
  status: "Offered",
  counterparty_node_id: "02".repeat(33),
  channel_scid: "18446744073709551615",
  contract_id: "contract:demo",
  asset_amount: "9007199254740993",
  btc_amount_msat: "2000",
  btc_carrier_amount_msat: "330000",
  maker_gives_rgb: true,
  expiry_secs: 3600,
  created_at_unix_secs: "1780000000",
  is_multihop: false,
  last_error: null,
};

test("swap methods encode requests and decode u64 fields losslessly", async () => {
  const requests = [];
  const fetch = async (url, init) => {
    requests.push({ url, init });
    if (url.endsWith("/swap/offers")) {
      return new Response(JSON.stringify({ swap_string: "rgb-swap:v1:demo", payment_hash: info.payment_hash, info }), {
        status: 200,
      });
    }
    if (url.endsWith("/swap/" + info.payment_hash)) {
      return new Response(JSON.stringify(info), { status: 200 });
    }
    return new Response(JSON.stringify([]), { status: 200 });
  };

  const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1", { fetch });
  const offer = await client.swapCreateOffer({
    counterparty_node_id: info.counterparty_node_id,
    channel_scid: u64(7),
    contract_id: info.contract_id,
    asset_amount: u64(1),
    btc_amount_msat: u64(2),
    btc_carrier_amount_msat: u64(3),
    maker_gives_rgb: true,
    expiry_secs: 3600,
  });

  assert.equal(offer.swap_string, "rgb-swap:v1:demo");
  assert.equal(offer.info.channel_scid.toString(), "18446744073709551615");
  assert.equal(offer.info.asset_amount.toString(), "9007199254740993");

  const fetched = await client.swapGet(info.payment_hash);
  assert.equal(fetched.created_at_unix_secs.toString(), "1780000000");
  assert.equal(requests[0].init.method, "POST");
  assert.equal(JSON.parse(requests[0].init.body).channel_scid, "7");
});

test("swapGet returns null for an unknown swap", async () => {
  const fetch = async () => new Response(JSON.stringify({ error: "unknown swap" }), { status: 404 });
  const client = new NodeHttpClient("http://127.0.0.1:8500/api/v1", { fetch });
  assert.equal(await client.swapGet("00".repeat(32)), null);
});
