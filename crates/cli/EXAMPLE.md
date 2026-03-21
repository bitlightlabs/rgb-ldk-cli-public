# `rgbldk` CLI examples (regtest, docker-compose)

This file is generated from a real local test run on regtest using `crates/cli/docker-compose.yaml`.

Daemon mode:
- If `../rgb-ldk-node` exists: build and start two local `rgbldkd` daemons from source.
- Otherwise: use the docker image(s) defined in `crates/cli/docker-compose.yaml`.

Notes:
- Outputs contain IDs (txids, invoices, payment IDs, node IDs, channel IDs) that differ per run.
- `rgbldk` is a client CLI talking to `rgbldkd` over HTTP.


Mutual transfers covered in this example (BTC/RGB × L1/L2):
- BTC L1: `BTC on-chain settlement (L1, node-b → node-a ...)` and `BTC on-chain settlement (L1, node-a → node-b ...)`
- BTC L2: `BTC Lightning transfer (L2, node-a → node-b)` and `BTC Lightning transfer (L2, node-b → node-a)`
- RGB L1: `RGB on-chain transfer (L1, node-a → node-b)` and `RGB on-chain transfer (L1, node-b → node-a)`
- RGB L2: `RGB Lightning transfer (L2, node-a → node-b)` and `RGB Lightning transfer (L2, node-b → node-a)`


## 0) Start environment

This section starts a self-contained regtest environment (bitcoind + esplora + two `rgbldkd` nodes) and shows the endpoints you will use in the rest of the examples.

In source mode, docker-compose starts bitcoind + esplora, and the script starts the two `rgbldkd` daemons locally. In docker-image mode, docker-compose starts all services (including `node-a` and `node-b`).

Note: Starting from Phase 0, `rgbldkd` starts locked. This generator initializes a local keystore and unlocks the daemons (source mode only) before running the rest of the commands.


### Commands

```bash
$ DOCKER_BUILDKIT=1 docker compose -f crates/cli/docker-compose.yaml up -d bitcoind chain-init esplora wait-esplora

```

```bash
$ (cd ../rgb-ldk-node && cargo build -p ldk-node --bin rgbldkd)

```

```bash
$ cargo build -p rgbldk-cli --bin rgbldk

```

```bash
$ export PATH="$PWD/target/debug:$PATH"

```

### Endpoints

```text
node_a=http://127.0.0.1:59187
node_b=http://127.0.0.1:8502
esplora=http://127.0.0.1:59183

```

## 1) Contexts (ctx)

Contexts let you name daemon endpoints (e.g. `node-a`, `node-b`) and switch the default target without repeatedly passing `--connect`.


### Add + show

**Run:**

```bash
$ rgbldk ctx add node-a --url http://127.0.0.1:59187 --use-now

```

**Result:**

> ```text
> Context "node-a" created and set as active.
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk ctx add node-b --url http://127.0.0.1:8502

```

**Result:**

> ```text
> Context "node-b" created.
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk ctx ls

```

**Result:**

> ```text
> +---------+--------+------------------------+
> | Current | Name   | URL                    |
> +===========================================+
> | *       | node-a | http://127.0.0.1:59187 |
> |---------+--------+------------------------|
> |         | node-b | http://127.0.0.1:8502  |
> +---------+--------+------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx show

```

**Result:**

> ```text
> node-a -> http://127.0.0.1:59187
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk ctx show

```

**Result:**

> ```text
> node-b -> http://127.0.0.1:8502
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

### Remove

**Run:**

```bash
$ rgbldk ctx add demo --url http://127.0.0.1:1

```

**Result:**

> ```text
> Context "demo" created.
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk ctx rm demo

```

**Result:**

> ```text
> Context "demo" removed.
>
> ```

## 2) Node basics (version/health/ready/status)

Use these commands to verify the CLI can reach the daemon, and to understand *why* a node is (not) ready via sub-checks.


### Health + status + identity

**Run:**

```bash
$ rgbldk node version

```

**Result:**

> ```text
> +--------------------+-----------+
> | Field              | Value     |
> +================================+
> | api_crate_version  | 0.7.0+git |
> |--------------------+-----------|
> | api_version        | v1        |
> |--------------------+-----------|
> | core_crate_version | 0.7.0+git |
> +--------------------+-----------+
>
> ```

**Run:**

```bash
$ rgbldk node health

```

**Result:**

> ```text
> [OK] node health
>   [OK] HTTP Server: Responding
>
> ```

**Run:**

```bash
$ rgbldk node ready

```

**Result:**

> ```text
> [OK] node ready
>   [OK] Lightning Node: Running
>   [OK] P2P Listener: Listening
>   [OK] Best Block Height: Height: 222
>
> ```

**Run:**

```bash
$ rgbldk node status

```

**Result:**

> ```text
> +-------------------+-------+
> | Field             | Value |
> +===========================+
> | is_running        | true  |
> |-------------------+-------|
> | p2p_is_listening  | true  |
> |-------------------+-------|
> | best_block_height | 222   |
> +-------------------+-------+
>
> ```

**Run:**

```bash
$ rgbldk wallet balance

```

**Result:**

> ```text
> +--------------------------+---------+
> | Asset                    | Balance |
> +====================================+
> | BTC On-chain (total)     |  0 sats |
> |--------------------------+---------|
> | BTC On-chain (spendable) |  0 sats |
> |--------------------------+---------|
> | BTC Anchor reserve       |  0 sats |
> |--------------------------+---------|
> | BTC Lightning (total)    |  0 sats |
> +--------------------------+---------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty node id

```

**Result:**

> ```text
> {
>   "node_id": "02d4c816f521749c8ab9882d827b56e1e45d7e08a7ac5ff99b0d25ab6a68982de6"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:59190
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk node ready

```

**Result:**

> ```text
> [OK] node ready
>   [OK] Lightning Node: Running
>   [OK] P2P Listener: Listening
>   [OK] Best Block Height: Height: 222
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty node id

```

**Result:**

> ```text
> {
>   "node_id": "038f76a17610c973e12aa7ccf6be61f5665d2ffa52d894453a85e063703b112a42"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:59192
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

### Local-only keystore + lock/unlock

These commands do not require the daemon HTTP API. They operate on local files (keystore) and a local control socket / control HTTP server.


#### Keystore init (generate mnemonic)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --yes --data-dir <data_dir> keystore init --mode generate-mnemonic --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-i_dkhta6/keystore-init-demo/keystore","mnemonic":"accident expect category lonely tool hip torch elbow security antique ozone autumn across coyote bacon maximum discover pave offer social mix foil east mountain","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-i_dkhta6/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-i_dkhta6/keystore-migrate-demo/keys_seed.bak.20260321-150922","ok":true}
>
> ```

#### Node unlock/lock (dummy local control socket)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --yes --data-dir <data_dir> node unlock --passphrase-stdin

```

**Result:**

> ```text
> {"ok":true}
>
> ```

**Run:**

```bash
$ rgbldk --yes --data-dir <data_dir> node lock

```

**Result:**

> ```text
> [OK] Locked
>
> ```

#### Node unlock-hosted (dummy control HTTP server)

**Run:**

```bash
$ printf '%s\n' '<control_token>' | rgbldk node unlock-hosted --control-connect http://127.0.0.1:<port> --control-token-stdin

```

**Result:**

> ```text
> {"ok":true}
>
> ```

## 3) Peers (ls)

List currently known peers. This is usually empty before you open channels or connect peers explicitly.


### List peers

**Run:**

```bash
$ rgbldk peer ls

```

**Result:**

> ```text
> +---------+---------+-----------+-----------+
> | Node ID | Address | Connected | Persisted |
> +===========================================+
> +---------+---------+-----------+-----------+
>
> ```

Tip: tables truncate long IDs by default to keep the output readable on an 80-column terminal. Use `--no-truncate` when you need to copy the full value.

**Show full IDs (no truncation):**


**Run:**

```bash
$ rgbldk --no-truncate peer ls

```

**Result:**

> ```text
> +---------+---------+-----------+-----------+
> | Node ID | Address | Connected | Persisted |
> +===========================================+
> +---------+---------+-----------+-----------+
>
> ```

### Connect + disconnect

**Run:**

```bash
$ rgbldk peer connect <node_id_b> <node_b_p2p> --persist

```

**Result:**

> ```text
> Peer connected.
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk peer connect <node_id_a> <node_a_p2p> --persist

```

**Result:**

> ```text
> Peer connected.
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk peer ls

```

**Result:**

> ```text
> +---------------------+-----------------+-----------+-----------+
> | Node ID             | Address         | Connected | Persisted |
> +===============================================================+
> | 038f76a1...3b112a42 | 127.0.0.1:59192 | true      | true      |
> +---------------------+-----------------+-----------+-----------+
>
> ```

Keep the peer connected for the rest of this example so channel opening and payments are reliable. A `peer disconnect` example is included in the cleanup section.


## 4) Wallet (address/sync) + fund on regtest

Generate on-chain addresses for both nodes, fund them from the regtest miner wallet, and then `wallet sync` to make balances visible to the node wallet.


### Generate addresses

**Run:**

```bash
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q9yxmg24ham6mppeupjzhmpg6zjngusx54fythz"
> }
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qyujwz2g96fqdgycmxg64cqcrkepxgc4tx2zynd"
> }
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

### Fund + sync

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin getnewaddress

```

**Result:**

> ```text
> bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1q9yxmg24ham6mppeupjzhmpg6zjngusx54fythz 1

```

**Result:**

> ```text
> 7f1cb83ee210095609e2d6e6cb504305e2390ffd9eb2bd102737c39f607a61f3
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qyujwz2g96fqdgycmxg64cqcrkepxgc4tx2zynd 1

```

**Result:**

> ```text
> 006a00d66b32834168426f5e22c2eaf1cc4d8366ccf07a5699ca0f739cc78bd5
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "42e5905baee28f63c6f0e689fbe636524d4c2a3496d34f073cfbb9067695c7ea",
>   "2eaf7d546fa9835ddba498a12e0a5867026cd27375903e235084346e8c88f70e",
>   "0707ee1feb84d6cba3e12f2da4b68d821ff0c8bb9f9d1223e7c5427e8f32b255",
>   "32c01724744dffa994ba523e12309c79e7bd42c38f0eab4518402b51301211f4",
>   "334d92345770aaab9b0d4e98aabbaac9437048ef322b19621b65fb0f478538dc",
>   "046ca80b12f6247d9a3b30ac493f8ca6494f5ac595b1106a3516f39f8cfb39ef"
> ]
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> No balance change.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```

**Result:**

> ```text
> Wallet synced.
> No balance change.
>
> ```

**Run:**

```bash
$ rgbldk wallet balance

```

**Result:**

> ```text
> +--------------------------+---------+
> | Asset                    | Balance |
> +====================================+
> | BTC On-chain (total)     | 1.0 BTC |
> |--------------------------+---------|
> | BTC On-chain (spendable) | 1.0 BTC |
> |--------------------------+---------|
> | BTC Anchor reserve       |  0 sats |
> |--------------------------+---------|
> | BTC Lightning (total)    |  0 sats |
> +--------------------------+---------+
>
> ```

**Run:**

```bash
$ rgbldk wallet balance --sats

```

**Result:**

> ```text
> +--------------------------+----------------+
> | Asset                    | Balance        |
> +===========================================+
> | BTC On-chain (total)     | 100000000 sats |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 100000000 sats |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |         0 sats |
> +--------------------------+----------------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet balance

```

**Result:**

> ```text
> +--------------------------+---------+
> | Asset                    | Balance |
> +====================================+
> | BTC On-chain (total)     | 1.0 BTC |
> |--------------------------+---------|
> | BTC On-chain (spendable) | 1.0 BTC |
> |--------------------------+---------|
> | BTC Anchor reserve       |  0 sats |
> |--------------------------+---------|
> | BTC Lightning (total)    |  0 sats |
> +--------------------------+---------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet balance --sats

```

**Result:**

> ```text
> +--------------------------+----------------+
> | Asset                    | Balance        |
> +===========================================+
> | BTC On-chain (total)     | 100000000 sats |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 100000000 sats |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |         0 sats |
> +--------------------------+----------------+
>
> ```

## 5) BTC on-chain settlement (L1, node-b → node-a via channel push+close)

This demonstrates a BTC L1 transfer between the two nodes by using a channel open with `--push-msat` (gives the receiver an initial balance) and then a cooperative `channel close` to settle on-chain.


### Commands

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk channel open --node-id <node_id_a> --addr <node_a_p2p> --amount-sats 60000 --push-msat 2000000 --private

```

**Result:**

> ```text
> {
>   "user_channel_id": "44e0e660cef121e9c6649f2084bcfeee"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "6b550bf9bf99f24bf348a67558dc917ef67dc784591882f3211483c3c62f315b",
>   "5365d22698648eed6e3aa6f90268aa8d468ead01298cfc9e6173f0e345a89009",
>   "14e54710fd664f8cb15e8172a01727a504188e8bfba863eddac320a9ac52b224",
>   "69b2e4a17e681e7476e7fddcd9764d047f7ea0f562b9d9ea9f8b7407d8b557c8",
>   "4a6f3778d42610058bba0d108224a8b7976f516cd721aaf5d847d97798abefbb",
>   "42b2c701b49b1ac8d9e33a84df21f0a9521e3d6c0161d7da81404862369e5b64"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel close --user-channel-id <user_channel_id> --counterparty-node-id <node_id_a>

```

**Result:**

> ```text
> Channel close initiated.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "28dc08710d5057cb02325764b2c79a7a4f269334da328f874897dcd8593288fd",
>   "796eec2f03709e043d9e75ca3366a4eafafa1f22ade75e4a5ccfd14caee4810a",
>   "6d6fd4c6c68b1ac7a1ed92f79b9b400ec95a989a5795c233b260a9d5e603ca93",
>   "1e0c905471a66f4e4e12cdfce9a3341d19dc2f746cacb9da7555433710ec4f96",
>   "0e2b8e4525040792618a0463662f072b69b1541d1ab07f3551f5eea5dbdb0dc2",
>   "105480156c0ab1dd0246d963275b876b8be68964081cefc43e6efe30e5134d65"
> ]
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +2,000 sats, spendable +2,000 sats, anchor reserve 0 sats, lightning -2,000 sats.
>
> ```

**Run:**

```bash
$ rgbldk wallet balance

```

**Result:**

> ```text
> +--------------------------+-------------+
> | Asset                    | Balance     |
> +========================================+
> | BTC On-chain (total)     | 1.00002 BTC |
> |--------------------------+-------------|
> | BTC On-chain (spendable) | 1.00002 BTC |
> |--------------------------+-------------|
> | BTC Anchor reserve       |      0 sats |
> |--------------------------+-------------|
> | BTC Lightning (total)    |      0 sats |
> +--------------------------+-------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +55,989 sats, spendable +55,989 sats, anchor reserve 0 sats, lightning -55,396 sats.
>
> ```

**Run:**

```bash
$ rgbldk wallet balance

```

**Result:**

> ```text
> +--------------------------+-----------------+
> | Asset                    | Balance         |
> +============================================+
> | BTC On-chain (total)     | 99,995,074 sats |
> |--------------------------+-----------------|
> | BTC On-chain (spendable) | 99,995,074 sats |
> |--------------------------+-----------------|
> | BTC Anchor reserve       |          0 sats |
> |--------------------------+-----------------|
> | BTC Lightning (total)    |          0 sats |
> +--------------------------+-----------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

## 6) RGB (sync/issuers/contracts)

Demonstrate basic RGB wallet sync and a minimal issuer/contract workflow: import an issuer archive, issue a contract, export a consignment, and import it into the other node.


### Sync + inventory

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q9yxmg24ham6mppeupjzhmpg6zjngusx54fythz"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +--------------------------+
> | issuer_name              |
> +==========================+
> | RGB20-Simplest-v0-rLosfg |
> |--------------------------|
> | demo-issuer              |
> +--------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts ls

```

**Result:**

> ```text
> +------+--------+-----------+---------------+----------+-------------+
> | name | ticker | precision | issued_supply | asset_id | contract_id |
> +====================================================================+
> +------+--------+-----------+---------------+----------+-------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos ls

```

**Result:**

> ```text
> 7f1cb83ee210095609e2d6e6cb504305e2390ffd9eb2bd102737c39f607a61f3:0
> a7943bbcfc436874fa8a298824740ae1dade9d5c1c668c72f1b8d8c57dbedcb3:0
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos summary

```

**Result:**

> ```text
> +--------------------------------------------------------------------+-------------+------------------+----------+----------------+
> | outpoint                                                           | value_sats  | confirmed_height | reserved | reserved_until |
> +=================================================================================================================================+
> | 7f1cb83ee210095609e2d6e6cb504305e2390ffd9eb2bd102737c39f607a61f3:0 | 100,000,000 |              223 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | a7943bbcfc436874fa8a298824740ae1dade9d5c1c668c72f1b8d8c57dbedcb3:0 |       2,000 |              235 | false    |              - |
> +--------------------------------------------------------------------+-------------+------------------+----------+----------------+
>
> ```

### Issue + share

Issuing a contract requires the RGB wallet to have a spendable UTXO. Fund the RGB wallet address and sync before issuing.


**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address> 0.1

```

**Result:**

> ```text
> 125b6e59bb3833a759bb97e228c90dd8ef85c520c0520e82170e55fb1c637987
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "64b21635a776c556c201d9a03f66d1ddb17ddf7bda2d722d88b8f6c74714f38c"
> ]
>
> ```

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

#### UTXOs (reserve/release)

**Run:**

```bash
$ rgbldk rgb utxos summary

```

**Result:**

> ```text
> +--------------------------------------------------------------------+-------------+------------------+----------+----------------+
> | outpoint                                                           | value_sats  | confirmed_height | reserved | reserved_until |
> +=================================================================================================================================+
> | 125b6e59bb3833a759bb97e228c90dd8ef85c520c0520e82170e55fb1c637987:0 |  10,000,000 |                - | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | 7f1cb83ee210095609e2d6e6cb504305e2390ffd9eb2bd102737c39f607a61f3:0 | 100,000,000 |              223 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | a7943bbcfc436874fa8a298824740ae1dade9d5c1c668c72f1b8d8c57dbedcb3:0 |       2,000 |              235 | false    |              - |
> +--------------------------------------------------------------------+-------------+------------------+----------+----------------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb utxos reserve --ttl-secs 60

```

**Result:**

> ```text
> {
>   "reservation_id": "Ry2NYZlWmO28MljM6J6yVvAcAyZmS19S",
>   "outpoint": "125b6e59bb3833a759bb97e228c90dd8ef85c520c0520e82170e55fb1c637987:0",
>   "reserved_until_unix_secs": "1774105876"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos release --reservation-id <reservation_id>

```

**Result:**

> ```text
> released=true
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers import --name demo-issuer --file ../rgb-ldk-node/tests/issuers/RGB20-Simplest-v0-rLosfg.issuer

```

**Result:**

> ```text
> [OK] RGB issuer import
>   [OK] RGB Enabled
>   [OK] Issuer Name Valid: demo-issuer
>   [OK] Upload Size Ok: 6152 bytes
>   [OK] Archive Decoded: 6152 bytes
>   [OK] Issuer Stored: /tmp/lrgb-issuers/demo-issuer.issuer
>   [OK] Issuer Loaded: demo-issuer
> issuer_name=demo-issuer
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +--------------------------+
> | issuer_name              |
> +==========================+
> | RGB20-Simplest-v0-rLosfg |
> |--------------------------|
> | demo-issuer              |
> +--------------------------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb contracts issue --issuer-name demo-issuer --contract-name "DemoAsset" --ticker DEMO --precision 0 --issued-supply 100

```

**Result:**

> ```text
> {
>   "ok": true,
>   "contract_id": "contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I",
>   "asset_id": "0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72",
>   "issued_supply": "100",
>   "checks": [
>     {
>       "name": "rgb_enabled",
>       "ok": true
>     },
>     {
>       "name": "issuer_found",
>       "ok": true,
>       "detail": "demo-issuer"
>     },
>     {
>       "name": "issuer_imported",
>       "ok": true,
>       "detail": "demo-issuer"
>     },
>     {
>       "name": "utxo_selected",
>       "ok": true,
>       "detail": "125b6e59bb3833a759bb97e228c90dd8ef85c520c0520e82170e55fb1c637987:0"
>     },
>     {
>       "name": "params_built",
>       "ok": true,
>       "detail": "issued_supply=100"
>     }
>   ]
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts ls

```

**Result:**

> ```text
> +-----------+--------+-----------+---------------+------------------------------------------------------------------+-----------------------------------------------------------+
> | name      | ticker | precision | issued_supply | asset_id                                                         | contract_id                                               |
> +===============================================================================================================================================================================+
> | DemoAsset | DEMO   |         0 |           100 | 0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72 | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> +-----------+--------+-----------+---------------+------------------------------------------------------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> |-------------+-----------------------------------------------------------|
> | mined       | 100                                                       |
> |-------------+-----------------------------------------------------------|
> | tentative   | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | offchain    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | archived    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | total       | 100                                                       |
> +-------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I --out /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I.raw --format raw --direct

```

**Result:**

> ```text
> exported=contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I
> consignment_key=contract_export_98894e9c879c442552e4392f97ec2a39280216a94ebd5a33946b0d0a5409a77c
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I.raw
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=125b6e59bb3833a759bb97e228c90dd8ef85c520c0520e82170e55fb1c637987:0
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip

```

**Result:**

> ```text
> {
>   "bytes": 2990,
>   "export": {
>     "checks": [
>       {
>         "name": "rgb_enabled",
>         "ok": true
>       },
>       {
>         "detail": "contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_466cdabf68a4dd5330373c27b5256b38dee6cdb3c379d98d512eb9610689df1f",
>     "contract_id": "contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I",
>     "ok": true
>   },
>   "out": "/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> consignment_key=contract_export_466cdabf68a4dd5330373c27b5256b38dee6cdb3c379d98d512eb9610689df1f
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I.download.zip
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qyujwz2g96fqdgycmxg64cqcrkepxgc4tx2zynd"
> }
>
> ```

Fund the RGB wallet on node-b as well. This ensures node-b has spendable RGB wallet UTXOs for later on-chain sends.


**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_b> 0.1

```

**Result:**

> ```text
> 1137b09c68d7e0f3d2c293b54babdc522f187f66b0f78fcc92feca73102ef6a3
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "17438ea328ccd7a6eeee8b0a9603f48291590fd0efdab5ec64f7189d8a6c5b4d"
> ]
>
> ```

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts import --contract-id contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I --file /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_3dfad3551dea36e7c9f73640c6edd00dc372c81e29135bde603e78a527c0f39f
> contract_id=contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I
> consignment_key=contract_import_3dfad3551dea36e7c9f73640c6edd00dc372c81e29135bde603e78a527c0f39f
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts ls

```

**Result:**

> ```text
> +-----------+--------+-----------+---------------+------------------------------------------------------------------+-----------------------------------------------------------+
> | name      | ticker | precision | issued_supply | asset_id                                                         | contract_id                                               |
> +===============================================================================================================================================================================+
> | DemoAsset | DEMO   |         0 |           100 | 0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72 | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> +-----------+--------+-----------+---------------+------------------------------------------------------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> |-------------+-----------------------------------------------------------|
> | mined       | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | tentative   | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | offchain    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | archived    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | total       | 0                                                         |
> +-------------+-----------------------------------------------------------+
>
> ```

#### RGB on-chain transfer (L1, node-a → node-b)

Send most of the issued supply from node-a to node-b before opening an RGB-enabled channel. This leaves node-a with exactly 10 units, so the channel funding transaction can spend that UTXO without creating additional RGB change outputs.


**Run:**

```bash
$ rgbldk rgb onchain invoice-create --contract-id <contract_id> --amount 90

```

**Result:**

> ```text
> {
>   "invoice": "contract:tb@DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I/90@at:5PrC_K1J-jj3Le9PX-TNn_fZ5T-Y9wm1Ay6-4vw_bQM6-oFNTMA/?expiry=2026-03-21T16:10:31.171701+00:00",
>   "blinding_utxo_used": "3eee98f9f3ca3097cba732726d3c44b7bc506bbb4c69ba0177f54101a650e06a:0"
> }
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I/90@at:5PrC_K1J-jj3Le9PX-TNn_fZ5T-Y9wm1Ay6-4vw_bQM6-oFNTMA/?expiry=2026-03-21T16:10:31.171701+00:00
> scope=contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:5PrC_K1J-jj3Le9PX-TNn_fZ5T-Y9wm1Ay6-4vw_bQM6-oFNTMA
> data=90
> expiry=2026-03-21T16:10:31.171701+00:00
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain send --invoice '<invoice>' --sats-for-fee-and-outputs 10000 --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {
>   "txid": "0b32c8bfb221ff714c2fbd2af6d6b084875d524b82081e82ab37050927fe957d",
>   "consignment_key": "rgb_consignment_contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I_0b32c8bfb221ff714c2fbd2af6d6b084875d524b82081e82ab37050927fe957d"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> consignment_key=rgb_consignment_contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I_0b32c8bfb221ff714c2fbd2af6d6b084875d524b82081e82ab37050927fe957d
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-a-to-b-prechannel.zip
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain receive --file <a-to-b.zip> --format zip --payment-id <payment_id>

```

**Result:**

> ```text
> asset_id=0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72
> amount=90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "419e1b381a26b91a7d498d4f7616c43d126f641d296610e33e8046e5bf8e7243",
>   "673780b12718de70900b29d4b3c8790d5fc54f1153d154a63a71c2fb41715153",
>   "43433b32c045a9d7d20ed96d6312062f066113821e07abef298142c20ff9b572",
>   "20bffe3241b39077d553c5b72f4b3d197fc47b3400779a2496c6d886a2beafd7",
>   "15590519a7361c958c44fa7936e57246a58eea8a6baff7567e6d000171fa59da",
>   "6058b42bd204859584209fb9bfdce364bac7db0707a8e97a9ae2b3ed115c8738"
> ]
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +9,999,345 sats, spendable +9,999,345 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> |-------------+-----------------------------------------------------------|
> | mined       | 90                                                        |
> |-------------+-----------------------------------------------------------|
> | tentative   | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | offchain    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | archived    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | total       | 90                                                        |
> +-------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total -250 sats, spendable -250 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> |-------------+-----------------------------------------------------------|
> | mined       | 10                                                        |
> |-------------+-----------------------------------------------------------|
> | tentative   | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | offchain    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | archived    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | total       | 10                                                        |
> +-------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

## 7) Channel (ls/open)

Open a private Lightning channel from node-a to node-b and wait for confirmations so the channel becomes usable.


If you provide `--rgb-context` when opening the channel, the daemon will share it with the counterparty as `color_context_data`. In a cross-host setup, this is typically an `http(s)` URL template that lets the receiver fetch the *funding consignment* out-of-band.


### Open + confirm (RGB-enabled, consignment over HTTP)

**Run:**

```bash
$ rgbldk channel open --node-id <node_id_b> --addr <node_b_p2p> --amount-sats 100000 --push-msat 20000000 --private --rgb-asset-id <asset_id_hex> --rgb-asset-amount 10 --rgb-context 'http://<A_HOST>:8501/api/v1/rgb/consignments/{txid}?format=zip'

```

**Result:**

> ```text
> {
>   "user_channel_id": "1c829b98985004e62adc22e3b5d1666a"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "3021f566adf1a20f9d6d0c26e03c5b5cd355e9e9e50ca8743aa385b0715c0ea3",
>   "1f220eb3ec55e24915b972fbcef425732a43bfacd9e5b3a31decf0ebc34fce98",
>   "4f071a78b4ec62ffa0751ee4e050982bb17b1fdb754072ed6f366a0c9761a35f",
>   "0476b7be6be3f683a48ff71ba5364efb9ee834bbd690e220cf8212af61ffd925",
>   "1d8f9c0a94bfb883cdb6e1bfc43a36163c0065fce4ba781f172d4e24db2ca7d9",
>   "001a59ae4f68f85b72cedf81dcdedb517fb3f3dd8eed01fb1e3e5fb30c8fee3f"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+-------+--------+---------------------+-----------+------------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable | RGB Asset           | RGB Local | RGB Remote |
> +=============================================================================================================================+
> | 1c829b98...b5d1666a | 038f76a1...3b112a42 | 100000          | false | false  | 0e2bebb4...e8e31b72 | 10        | 0          |
> +---------------------+---------------------+-----------------+-------+--------+---------------------+-----------+------------+
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total -101,176 sats, spendable -101,176 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```

**Result:**

> ```text
> Wallet synced.
> No balance change.
>
> ```

Because we passed `--rgb-context` during `channel open`, node-b can fetch the funding consignment directly from node-a over HTTP (no manual download/upload step needed). You can verify that node-b has cached the funding consignment by querying its consignment endpoint by the funding txid:


**Run:**

```bash
$ curl -sSf -o /dev/null -w '%{http_code}\n' 'http://<B_HOST>:8502/api/v1/rgb/consignments/<funding_txid>?format=zip'

```

**Result:**

> ```text
> 200
>
> ```

### RGB on-chain transfer (L1, node-b → node-a)

This demonstrates an RGB L1 transfer using the `rgb onchain` commands. Because the sender's consignment cache is not shared with the receiver in a multi-node setup, we download the consignment from the sender and upload it to the receiver via `--file`.

Note: a node-a → node-b RGB L1 transfer is shown earlier (before opening the RGB-enabled channel) to make the channel funding transaction deterministic.


**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain invoice-create --contract-id <contract_id> --amount 7

```

**Result:**

> ```text
> {
>   "invoice": "contract:tb@DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I/7@at:9OPDrQCg-e_m7SIpK-ML1krVRS-JDVUzOtQ-Vpk6ERje-2RERhA/?expiry=2026-03-21T16:10:52.491426+00:00",
>   "blinding_utxo_used": "743a88b6c9ac5e7677da0e4d24fbf45b39dbab329eff97d39df758d39b1e6bbb:2"
> }
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> No balance change.
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain send --invoice '<invoice>' --sats-for-fee-and-outputs 10000 --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {
>   "txid": "ccb800a4d8bbab4f6974763a2da3c5a00c6caf0d9513ba2f88623479497fcfcd",
>   "consignment_key": "rgb_consignment_contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I_ccb800a4d8bbab4f6974763a2da3c5a00c6caf0d9513ba2f88623479497fcfcd"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> consignment_key=rgb_consignment_contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I_ccb800a4d8bbab4f6974763a2da3c5a00c6caf0d9513ba2f88623479497fcfcd
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-b-to-a.zip
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain receive --file <b-to-a.zip> --format zip --payment-id <payment_id>

```

**Result:**

> ```text
> asset_id=0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72
> amount=7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "197c9b31d6cbf82b03f3dfbacf0e34754d218a682e680292669eea5a93a32669",
>   "15b7c0a64bbaeb1b908a1e2b2fed0b1f8d3dc1b5f44918a9c4cd19dee7eadf9e",
>   "3b7cd9598d240d56ad8f9ed7829d777982b4bb37f6c001fa7397f401cccbd37f",
>   "695e248813038bb5611d616804035272c2ac75e33ad156cce5a486a2c5905caf",
>   "44d3cc58077f2b38975ce45c35ec538bb629f5ff132700f4f7d69471ef246723",
>   "39578160347994e46562a77d0ab64c450349e8319169f470a0a3eab558454936"
> ]
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> No balance change.
>
> ```

**Run:**

```bash
$ rgbldk rgb sync

```

**Result:**

> ```text
> [OK] RGB sync
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |
> |-------------+-----------------------------------------------------------|
> | mined       | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | tentative   | 7                                                         |
> |-------------+-----------------------------------------------------------|
> | offchain    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | archived    | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | total       | 7                                                         |
> +-------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

## 8) Payments (BTC Lightning L2 + keysend + BOLT12 + RGB Lightning)

Create Bolt11 invoices on node-b and pay them from node-a. Use `pay wait` to block until the payment reaches a terminal state, and `pay get` to inspect details. Also demonstrate a spontaneous (keysend) payment.


### BTC Lightning transfer (L2, node-a → node-b)

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay invoice create --desc demo --amount-msat 10000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt100n1p5madg8dq8v3jk6mcnp4qw8hdgtkzryh8cf25lx0d0np74n96tl62tvfg3f6shsxxupmzy4yypp5s0ze92gjxlkruh3nq4943u9a3ge7z2zkxsy05qfkggtcfe6vfhcqsp5hh55tw4m9v7uzgd00kfp5s8dwexmr8j9r6639d4f0rhmjtlj73cq9qyysgqcqpcxqrrssrzjqt2vs9h4y96fez4e3qkcy76ku8j96lsg57k9l7vmp5j6k6ngnqk7vqqqqyqqsqsqqsqqqqlgqqqqqqqqfqr6lfmztqyr9atd20ewx2t83gqae68yyytxppuy8jz8nfnfea2awn246lhmw7sakfpkjyjmjrf7866er625u2p07f4mtyvxd2zf695ngp5wv5u7"
> }
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay invoice pay --invoice <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "83c592a91237ec3e5e33054b58f0bd8a33e128563408fa0136421784e74c4df0",
>   "preimage": "38a8424713fc1a40c2894b8b7dfdbada3ce22395743b172db45812a21f9f2d0f",
>   "amount_sats": "10",
>   "destination": "038f76a17610c973e12aa7ccf6be61f5665d2ffa52d894453a85e063703b112a42",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 83c592a91237ec3e5e33054b58f0bd8a33e128563408fa0136421784e74c4df0 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 83c592a91237ec3e5e33054b58f0bd8a33e128563408fa0136421784e74c4df0
>
> ```

**Run:**

```bash
$ rgbldk pay get 83c592a91237ec3e5e33054b58f0bd8a33e128563408fa0136421784e74c4df0

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 83c592a91237ec3e5e33054b58f0bd8a33e128563408fa0136421784e74c4df0                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"83c592a91237ec3e5e33054b58f0bd8a33e128563408fa0136421784e74c4df0","preimage":"38a8424713fc1a40c2894b8b7dfdbada3ce22395743b172db45812a21f9f2d0f","secret":"bde945babb2b3dc121af7d921a40ed764db19e451eb512b6a978efb92ff2f470"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 10,000 msat                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                        |
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay invoice create --desc demo-var

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt1p5madgwdqdv3jk6medweshynp4qw8hdgtkzryh8cf25lx0d0np74n96tl62tvfg3f6shsxxupmzy4yypp58hjyzhh7u8et8ts082az9rzvg38eahlj0afxu95ylmmn0zfmwmxssp5y9wjn75rgs6lvdqpq6tpqkfej0zrz85ymy7l69n5dv3aclv90yjq9qyysgqcqpcxqrrssrzjqt2vs9h4y96fez4e3qkcy76ku8j96lsg57k9l7vmp5j6k6ngnqk7vqqqqyqqsqsqqsqqqqlgqqqqqqqqfqlsew23n8d4m3tw8sva0nmdr5fwsxkj44guqg5sv8jpggpsv2j2ey5hu3844z8k0jfdlf84ekqppw6zjd6rlg4ufckqage0eysa4relgq4dyftu"
> }
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay invoice pay --invoice <invoice> --amount-msat 11000

```

**Result:**

> ```text
> {
>   "payment_id": "3de4415efee1f2b3ae0f3aba228c4c444f9edff27f526e1684fef737893b76cd",
>   "preimage": "cf2167043ff8049575601f33168099f0a89aadb78e33d44cff8f76cf579f831e",
>   "amount_sats": "11",
>   "destination": "038f76a17610c973e12aa7ccf6be61f5665d2ffa52d894453a85e063703b112a42",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 3de4415efee1f2b3ae0f3aba228c4c444f9edff27f526e1684fef737893b76cd --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 3de4415efee1f2b3ae0f3aba228c4c444f9edff27f526e1684fef737893b76cd
>
> ```

**Run:**

```bash
$ rgbldk pay get 3de4415efee1f2b3ae0f3aba228c4c444f9edff27f526e1684fef737893b76cd

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 3de4415efee1f2b3ae0f3aba228c4c444f9edff27f526e1684fef737893b76cd                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"3de4415efee1f2b3ae0f3aba228c4c444f9edff27f526e1684fef737893b76cd","preimage":"cf2167043ff8049575601f33168099f0a89aadb78e33d44cff8f76cf579f831e","secret":"215d29fa834435f63401069610593993c4311e84d93dfd16746b23dc7d857924"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 11,000 msat                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                        |
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

### BTC Lightning transfer (L2, node-b → node-a)

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay invoice create --desc demo-back --amount-msat 12000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt120n1p5madgndq0v3jk6medvfskx6cnp4qt2vs9h4y96fez4e3qkcy76ku8j96lsg57k9l7vmp5j6k6ngnqk7vpp5pvdfzwhkxdj0uv9zd37m00nyddk94k38jzdu82g9kk40sm6y6q5qsp5zr8wkm384yfx7gzrv5lpcqgc8x9z5sa7f0em9zzsnyad4m8jw6kq9qyysgqcqpcxqrrssrzjqw8hdgtkzryh8cf25lx0d0np74n96tl62tvfg3f6shsxxupmzy4yyqqqqyqqd8cqqcqqqqlgqqqqqqqqfq7vscqwfyx7envfsmpwd23pq65mtvelkpyly8npr5jd9g9xt60psh2rr8nctcqrwfch5x89e3fgwl9e464gyxj4z5p7d8ahzhwvqe0rqpxlkaxa"
> }
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay invoice pay --invoice <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "0b1a913af63364fe30a26c7db7be646b6c5ada27909bc3a905b5aaf86f44d028",
>   "preimage": "a344f034981336f9b761203ad004d26b7aa9a5016b7480df56bad15d5f794c16",
>   "amount_sats": "12",
>   "destination": "02d4c816f521749c8ab9882d827b56e1e45d7e08a7ac5ff99b0d25ab6a68982de6",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 0b1a913af63364fe30a26c7db7be646b6c5ada27909bc3a905b5aaf86f44d028 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 0b1a913af63364fe30a26c7db7be646b6c5ada27909bc3a905b5aaf86f44d028
>
> ```

**Run:**

```bash
$ rgbldk pay get 0b1a913af63364fe30a26c7db7be646b6c5ada27909bc3a905b5aaf86f44d028

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 0b1a913af63364fe30a26c7db7be646b6c5ada27909bc3a905b5aaf86f44d028                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"0b1a913af63364fe30a26c7db7be646b6c5ada27909bc3a905b5aaf86f44d028","preimage":"a344f034981336f9b761203ad004d26b7aa9a5016b7480df56bad15d5f794c16","secret":"10ceeb6e27a9126f2043653e1c0118398a2a43be4bf3b28850993adaecf276ac"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 12,000 msat                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                        |
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

### BOLT12 offer

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay offer create --desc offer-demo --amount-msat 5555

```

**Result:**

> ```text
> {
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczxn0kr9ggv7qk5eqt02gt5nj9tnzpdsfa4dc0yt4lq3favtluekrf94d4x3xpducphk6xl0c9sj24ut9gcvg5njrcsyrkd8cswwpu27ux0yqn08svjp6gzqv7dd0553yhnf6a8z58ecgg3ef2jwsn84cdf92g20cfya3537jhgqqq6fp8psnyxg9tzrzt7xp0c3ydyx20fu7le87mm636ujgpjk2aehj8dusel2hd38dhvwv84z9sk4xpry0m9q9ej44l9h9vpcdgq9nhd9cxy4u7za8akrkga6xv70rg488svff3wkg6trldm5cc38gt7nyjmjtc4zgl7waetlmnhzcss8lwlmlpklzyh02j9qsqthpr5gw0xrtpr648vdxs55q0dtspw92qr"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay offer decode <offer>

```

**Result:**

> ```text
> +---------------------------+--------------------------------------------------------------------+
> | Field                     | Value                                                              |
> +================================================================================================+
> | offer_id                  | 54cf9e7b9d9a3afa67baa17c868751aee1bb55cdcaf7e5b5670dd8c87b59863e   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 03fddfdfc36f88977aa450400bb8474439e61ac23d54ec69a14a01ed5c02e2a803 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1774109482                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | paths_count               | 1                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | expects_quantity          | false                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hashes              | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> +---------------------------+--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay offer pay --offer <offer>

```

**Result:**

> ```text
> {
>   "payment_id": "e8479cb388884f130fb1e2e6f8f529b4d7f4b1ac243278fc726b5dd96a80552c"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait e8479cb388884f130fb1e2e6f8f529b4d7f4b1ac243278fc726b5dd96a80552c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> e8479cb388884f130fb1e2e6f8f529b4d7f4b1ac243278fc726b5dd96a80552c
>
> ```

**Run:**

```bash
$ rgbldk pay get e8479cb388884f130fb1e2e6f8f529b4d7f4b1ac243278fc726b5dd96a80552c

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | e8479cb388884f130fb1e2e6f8f529b4d7f4b1ac243278fc726b5dd96a80552c                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"54cf9e7b9d9a3afa67baa17c868751aee1bb55cdcaf7e5b5670dd8c87b59863e","payer_note":null,"payment_hash":"df7a96b87223fed1b06076125637a245e21694c02d44c7d1e8d1716b794fa432","quantity":null} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 5,555 msat                                                                                                                                                                                          |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                              |
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

### BOLT12 refund (initiate/request-payment) + abandon

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay refund initiate --amount-msat 4321 --payer-note refund-demo

```

**Result:**

> ```text
> {
>   "refund": "lnr1qqswgarahu9u6uwy5sqfesn0prqddvsvd7qprkkm74gfke2jzjzpgss2qq8qg6d7cvc9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxg3sdfnz79zq6t2ruxyh7rdmvcxcdd5w2lmpxgzw322kp0snz8u6ty9hyetxw4hxgttyv4kk7kh3qw8hdgtkzryh8cf25lx0d0np74n96tl62tvfg3f6shsxxupmzy4yyqekf5tcrvlzef8a0jzczjq9evvdqd5e0cu6cldl8ytsp27ghp93g5pqx3wq2tx85zwqy6a5gut9psjxjpa5gp36h4pxkusyseyv87a7v0x2qqdz5thh0j85n5yn3hu6t92gxy5mexlh34tmp59ggzy45qhe6p7g8hqrja8rcws0pslcsxy2480squasqdd7p3yl7zxnehfgrcqya26cq2es2pnp0q7zgmn3vjcwrm9fyh8tkxly0xhx0wq3ny677xqgpc82f0gczd2kz7kjxjc79sgf8u7h929x9dc558ehfy4h8jc4qrz6uke0m69cj4jxz7lf29hxfg",
>   "payment_id": "a7e5261d606aa34b3e2b3cfc1360aafec283c4e78c2c45621d16631fb3f2013e"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay refund decode <refund>

```

**Result:**

> ```text
> +---------------------------+--------------------------------------------------------------------+
> | Field                     | Value                                                              |
> +================================================================================================+
> | description               |                                                                    |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 4,321                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1774109488                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 0322306a662f1440d2d43e1897f0dbb660d86b68e57f613204e8a9560be1311f9a |
> |---------------------------+--------------------------------------------------------------------|
> | payer_note                | refund-demo                                                        |
> |---------------------------+--------------------------------------------------------------------|
> | quantity                  | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | paths_count               | 1                                                                  |
> +---------------------------+--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk pay refund request-payment <refund>

```

**Result:**

> ```text
> lni1qqswgarahu9u6uwy5sqfesn0prqddvsvd7qprkkm74gfke2jzjzpgss2qq8qg6d7cvc9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxg3sdfnz79zq6t2ruxyh7rdmvcxcdd5w2lmpxgzw322kp0snz8u6ty9hyetxw4hxgttyv4kk7kh3qw8hdgtkzryh8cf25lx0d0np74n96tl62tvfg3f6shsxxupmzy4yyqekf5tcrvlzef8a0jzczjq9evvdqd5e0cu6cldl8ytsp27ghp93g5pqx3wq2tx85zwqy6a5gut9psjxjpa5gp36h4pxkusyseyv87a7v0x2qqdz5thh0j85n5yn3hu6t92gxy5mexlh34tmp59ggzy45qhe6p7g8hqrja8rcws0pslcsxy2480squasqdd7p3yl7zxnehfgrcqya26cq2es2pnp0q7zgmn3vjcwrm9fyh8tkxly0xhx0wq3ny677xqgpc82f0gczd2kz7kjxjc79sgf8u7h929x9dc558ehfy4h8jc4qrz6uke0m69cj4jxz7lf29hxf2s06qf0qt2vs9h4y96fez4e3qkcy76ku8j96lsg57k9l7vmp5j6k6ngnqk7vq4gtew4dmynmc0scvh8advgs05jdta0n857uj4wexxnuthsg4gt7qpq9zaawpdv0gyf5gr9pr6m3cd6dq5zt95htsjv458g6w8w9yxwx2azqq452pmmz28tyw8g05fss2e0rmgst98deus456a6m8uc234eg8jglx3adzwp3f0pj3qadzglqtwumy2xfe4apn7zma0m8mcanyevn4sp8as8htpqx7al9y8r820eqqrmr7yf29vhze4q8rnn48ktrn0ucccfjpzh4q0juyzkav6nuvhjvh9zh3ckngd0nq5tp4xnfn5u9fay0z86wq38mq64hy3kjvuxuvntl8el937s5jddsavu5zr7l83p8zzu05ek0k45c6g5qy32udt0pdrtwg3cq3js3jch8rfsn2peyf8ssvekvxf648xk4glrpf2x4gsuqqqq86qqqqqqqqrgqqqqqqqqqqqqzqqqqqqqt46cnqqqpfqydxlt2g4gyr8uunkwlsyr4kpefdgj2rqakq8rpjurnrjmkckn8q4art2m2fwq82szzrs6uqczqqqtqggz9ldm4wy66gmmdgkmj8w9x6phj5uzj4tvwvcxayy5fhzge0lrpga0qsxjdg23hxck5xarahregt9qjzrvu0cp5c7z28gy63vdwwyen2824u5qyapdklur96ne7h9enqmxy5vautfpfr645d57ts3rr7fs87ctz
> payment_id: cfce4ecefc083ad8394b51250c1db00e30cb8398e5bb62d3382bd1ad5b525c03
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk pay wait a7e5261d606aa34b3e2b3cfc1360aafec283c4e78c2c45621d16631fb3f2013e --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> a7e5261d606aa34b3e2b3cfc1360aafec283c4e78c2c45621d16631fb3f2013e
>
> ```

**Run:**

```bash
$ rgbldk pay get a7e5261d606aa34b3e2b3cfc1360aafec283c4e78c2c45621d16631fb3f2013e

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | a7e5261d606aa34b3e2b3cfc1360aafec283c4e78c2c45621d16631fb3f2013e                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"cfce4ecefc083ad8394b51250c1db00e30cb8398e5bb62d3382bd1ad5b525c03","quantity":null} |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 4,321 msat                                                                                                                     |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                         |
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay refund initiate --amount-msat 1111 --payer-note refund-abandon-demo

```

**Result:**

> ```text
> {
>   "refund": "lnr1qqsv2wl49yagewlfwae08ueasfdl8ekq7ksypwx5yfsx084lwrt3y2s2qq8qg6d7cv64qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqssylswr20wjs8gzynln7wjqnd0netl85frc56ckafutjdvemckl06jtyfhyetxw4hxgttpvfskuer0dckkgetddad0zqu0w6shvyxfw0sj4f7v76lxratxt5hl55kcj3zn4p0qvdcrkyf2ggp8jr0x7srd60f7mj44e6kxj6aw9xw5qxpuhs6fj5j6qzr0y04m9cgzqfz2wf4dh72efep24mkxjqull4cpyuf25qnm4xuz8yd09xl2kc6gsqq6t7wlygdgmzcjfcnevcda0dlvrdjps59qjr6d7edf7spuwwg699s4m86mfnrz43aqp6pmzj0c37cnfhm4geyk3d9yd5l4eggqfm7er0tw39cx7a84k0wuprh3jsrquy5tgyd4cg52ggwecdcj7sep8gckee8p49d63snswycrgplj8q63zzsswrerncslgqp0zmzcrt44rdrnfhrkr74lr6u74rr5ahg",
>   "payment_id": "aae6f6ecc6398cab024402ca8e53cbce85596ca10db3047b5c1a024c86aeb4a3"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay abandon <payment_id>

```

**Result:**

> ```text
> [OK] Abandon payment
>
> ```

### Payments list

**Run:**

```bash
$ rgbldk pay ls

```

**Result:**

> ```text
> +---------------------+-----------+--------------+----------+-----------------+------------+
> | ID                  | Status    | Kind         | Dir      | Amount (msat)   | Fee (msat) |
> +==========================================================================================+
> | 3de4415e...893b76cd | Succeeded | Bolt11       | Outbound | 11,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | aae6f6ec...86aeb4a3 | Failed    | Bolt12Refund | Outbound | 1,111           | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | b3dcbe7d...bc3b94a7 | Succeeded | Onchain      | Inbound  | 2,000,000       | 2,011,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 8779631c...596e5b12 | Succeeded | Onchain      | Inbound  | 10,000,000,000  | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | f3617a60...3eb81c7f | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 7d95fe27...bfc8320b | Succeeded | Onchain      | Outbound | 0               | 250,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | bb6b1e9b...b6883a74 | Succeeded | Onchain      | Outbound | 100,000,000     | 1,176,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | ac35dd84...c5d2f7d8 | Succeeded | Onchain      | Outbound | 0               | 655,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | e8479cb3...6a80552c | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 83c592a9...e74c4df0 | Succeeded | Bolt11       | Outbound | 10,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | a7e5261d...b3f2013e | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 0b1a913a...6f44d028 | Succeeded | Bolt11       | Inbound  | 12,000          | -          |
> +---------------------+-----------+--------------+----------+-----------------+------------+
>
> ```

### RGB Lightning transfer (L2, node-a → node-b)

Create an RGB LN invoice on node-b and pay it from node-a over the RGB-enabled channel.


**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln invoice create --asset-id <asset_id_hex> --asset-amount 5 --desc "rgb ln demo" --btc-carrier-amount-msat 5000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt50u1p5madfgdqjwfnkygrvdcsxgetddupp5m9tlwgh43uftz8fgj82umrdc3srhj0zvezljxueffm4uka3ky6assp55g4c8djerwu6crqfcdvgrx2tv86dqmmg6u7uuq8t3gnhzjhr4wfs9qrsgqxqrrsscqpjlp5pc47hdq6jzfw66rs6qdzduytk7a7vhrjlmq744un80f5368rrdeq7qp9wtzdgy4693984tf88s9vhl4ft3xl3fe2f24jqt0f6mzpslg44f8842g80e3ht3q50yl2ecd9neggn5vjuc3kfvzudqhfm476egd9qqqq8k4hf3"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb ln invoice decode <invoice>

```

**Result:**

> ```text
> +---------------------+--------------------------------------------------------------------+
> | Field               | Value                                                              |
> +==========================================================================================+
> | payment_hash        | d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb   |
> |---------------------+--------------------------------------------------------------------|
> | destination         | 038f76a17610c973e12aa7ccf6be61f5665d2ffa52d894453a85e063703b112a42 |
> |---------------------+--------------------------------------------------------------------|
> | carrier_amount_msat | 5,000,000                                                          |
> |---------------------+--------------------------------------------------------------------|
> | expiry_secs         | 3600                                                               |
> |---------------------+--------------------------------------------------------------------|
> | asset_id            | 0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72   |
> |---------------------+--------------------------------------------------------------------|
> | asset_amount        | 5                                                                  |
> +---------------------+--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln pay --invoice <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb
>
> ```

**Run:**

```bash
$ rgbldk pay get d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb                                                                                                                                                                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"d957f722f58f12b11d2891d5cd8db88c07793c4cc8bf2373294eebcb763626bb","preimage":"b37bab05a3ea38a98cc309cb2a57eb02ccd262cca0a5d671b269a851267caa7b","rgb":{"asset_amount":"5","asset_id":"0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72","direction":"Outbound","is_swap":false},"secret":"a22b83b6591bb9ac0c09c35881994b61f4d06f68d73dce00eb8a27714ae3ab93"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 5,000,000 msat                                                                                                                                                                                                                                                                                                                                                                                |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                                                                                                                                                                        |
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

#### RGB Lightning transfer (L2, node-b → node-a)

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln invoice create --asset-id <asset_id_hex> --asset-amount 3 --desc "rgb ln demo back" --btc-carrier-amount-msat 4000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt40u1p5madfwdq6wfnkygrvdcsxgetddusxyctrdvpp5va9xal34usvwc04p8l4yh54ng568hf6av90sahqj93kdphl4y79ssp5re7pu2agvk4fufwlhppu29fqa8na4gpwrmwtlg4qv6j7h0evlv2q9qrsgqxqrrsscqpjlp5pc47hdq6jzfw66rs6qdzduytk7a7vhrjlmq744un80f5368rrdeq7qpr3twk9rgc8f8ztrpezdtld790mm0uvgtwc5ncpsjupls2e0ghauarwm50x55qmrrwf6te8hdwu3nucxdm8lvsr7qtjn9qprccxkfxjgspngss7m"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb ln invoice decode <invoice>

```

**Result:**

> ```text
> +---------------------+--------------------------------------------------------------------+
> | Field               | Value                                                              |
> +==========================================================================================+
> | payment_hash        | 674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b   |
> |---------------------+--------------------------------------------------------------------|
> | destination         | 02d4c816f521749c8ab9882d827b56e1e45d7e08a7ac5ff99b0d25ab6a68982de6 |
> |---------------------+--------------------------------------------------------------------|
> | carrier_amount_msat | 4,000,000                                                          |
> |---------------------+--------------------------------------------------------------------|
> | expiry_secs         | 3600                                                               |
> |---------------------+--------------------------------------------------------------------|
> | asset_id            | 0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72   |
> |---------------------+--------------------------------------------------------------------|
> | asset_amount        | 3                                                                  |
> +---------------------+--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln pay --invoice <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b
>
> ```

**Run:**

```bash
$ rgbldk pay get 674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b                                                                                                                                                                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"674a6efe35e418ec3ea13fea4bd2b345347ba75d615f0edc122c6cd0dff5278b","preimage":"f72e061ec88d06df8c5bedfb975c46081c0513a02f43dbd074d0a88b307cd441","rgb":{"asset_amount":"3","asset_id":"0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72","direction":"Outbound","is_swap":false},"secret":"1e7c1e2ba865aa9e25dfb843c51520e9e7daa02e1edcbfa2a066a5ebbf2cfb14"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 4,000,000 msat                                                                                                                                                                                                                                                                                                                                                                                |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                                                                                                                                                                        |
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

To confirm the asset movement, list channels on both nodes again. For RGB-enabled channels, `channel ls` includes extra columns (`RGB Asset`, `RGB Local`, `RGB Remote`) showing the current in-channel balances.


**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+-------+--------+---------------------+-----------+------------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable | RGB Asset           | RGB Local | RGB Remote |
> +=============================================================================================================================+
> | 1c829b98...b5d1666a | 038f76a1...3b112a42 | 100000          | true  | true   | 0e2bebb4...e8e31b72 | 8         | 2          |
> +---------------------+---------------------+-----------------+-------+--------+---------------------+-----------+------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+-------+--------+---------------------+-----------+------------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable | RGB Asset           | RGB Local | RGB Remote |
> +=============================================================================================================================+
> | 3134fc0a...7aaafaec | 02d4c816...68982de6 | 100000          | true  | true   | 0e2bebb4...e8e31b72 | 2         | 8          |
> +---------------------+---------------------+-----------------+-------+--------+---------------------+-----------+------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

### Keysend

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay keysend send --node-id <node_id_b> --amount-msat 1234 --tlv 70001:02

```

**Result:**

> ```text
> {
>   "payment_id": "0418866ae20669ffe646a427e6111d50c07a2a5996e2218ac8ed78dac41f7495"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 0418866ae20669ffe646a427e6111d50c07a2a5996e2218ac8ed78dac41f7495 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 0418866ae20669ffe646a427e6111d50c07a2a5996e2218ac8ed78dac41f7495
>
> ```

**Run:**

```bash
$ rgbldk pay get 0418866ae20669ffe646a427e6111d50c07a2a5996e2218ac8ed78dac41f7495

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 0418866ae20669ffe646a427e6111d50c07a2a5996e2218ac8ed78dac41f7495                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"0418866ae20669ffe646a427e6111d50c07a2a5996e2218ac8ed78dac41f7495","preimage":"be3b9af38e8c72f007dd1d21d3af387f14e73bfb30932c7e1aa0e0d86914519d"} |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 1,234 msat                                                                                                                                                        |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                            |
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

#### Keysend (reverse direction)

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay keysend send --node-id <node_id_a> --amount-msat 2345 --tlv 70002:03

```

**Result:**

> ```text
> {
>   "payment_id": "c9c62a1cb3b299ce89412a041a739a07bb9eedb2a0c97f5fb08fe2ca542a9e26"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait c9c62a1cb3b299ce89412a041a739a07bb9eedb2a0c97f5fb08fe2ca542a9e26 --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> c9c62a1cb3b299ce89412a041a739a07bb9eedb2a0c97f5fb08fe2ca542a9e26
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get c9c62a1cb3b299ce89412a041a739a07bb9eedb2a0c97f5fb08fe2ca542a9e26

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | c9c62a1cb3b299ce89412a041a739a07bb9eedb2a0c97f5fb08fe2ca542a9e26                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"c9c62a1cb3b299ce89412a041a739a07bb9eedb2a0c97f5fb08fe2ca542a9e26","preimage":"4ffea1e0c155103eae7a2b79f675d785642200371bc0281aefaf9406f5d8c126"} |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 2,345 msat                                                                                                                                                        |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | -                                                                                                                                                                 |
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

## 9) Events (next/handled)

Demonstrate the event queue API: fetch the next event (`events next`) and acknowledge it (`events handled`) so the daemon can advance the queue.


### Queue

Tip: `events watch` blocks until it receives events. This example uses a short timeout so the generator does not hang if the queue is empty.


**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:59187 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=3eee98f9f3ca3097cba732726d3c44b7bc506bbb4c69ba0177f54101a650e06a:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=3eee98f9f3ca3097cba732726d3c44b7bc506bbb4c69ba0177f54101a650e06a:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:59187 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=8da83270648f98fe335d5f023f40eca7
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:59187 events handled

```

**Result:**

> ```text
> Marked handled.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=44e0e660cef121e9c6649f2084bcfeee
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events handled

```

**Result:**

> ```text
> Marked handled.
>
> ```

## 10) BTC on-chain settlement (L1, node-a → node-b via channel push+close) + Channel force-close

Demonstrate graceful close vs force-close. Force-close is destructive and requires `--yes` for non-interactive safety.


### BTC on-chain settlement (L1, node-a → node-b via channel push+close)

**Run:**

```bash
$ rgbldk channel close --user-channel-id <user_channel_id> --counterparty-node-id <node_id_b>

```

**Result:**

> ```text
> Channel close initiated.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "7370c0517a0416f754fac4cd3d64c33d5b6a78eb37ce5c131b3ad0f60bd53788",
>   "6737ffce6e3012628033bd2747fe5b75e1babfc95315f6ace265fca1cab3f08e",
>   "625921880ba6135c8655a6abb4281439830a50b11c5d8930a473514a1375234e",
>   "21ddaeeab7bc14417d0b87d05893d083ba78ecaf7183c706045b89da386f937a",
>   "28c1858b5b002a4fbef8705613b79c88651e1dcb54f58fcc13377b60cb165bee",
>   "5409028a5bb026f06d4a5d10f0f99d8b791e64d9e2fe85d44eeb66076e6f312b"
> ]
>
> ```

**Run:**

```bash
$ rgbldk wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +76,968 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +21,020 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk wallet balance

```

**Result:**

> ```text
> +--------------------------+----------------+
> | Asset                    | Balance        |
> +===========================================+
> | BTC On-chain (total)     | 1.09976887 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.09899919 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    76,375 sats |
> +--------------------------+----------------+
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Asset                                                     | Contract                                                  | Mined | Tentative | Offchain | Total |
> +=====================================================================================================================================================================+
> | 0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72 | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |     7 |         8 |        0 |    15 |
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet balance

```

**Result:**

> ```text
> +--------------------------+----------------+
> | Asset                    | Balance        |
> +===========================================+
> | BTC On-chain (total)     | 1.10015189 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.09994169 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    21,020 sats |
> +--------------------------+----------------+
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Asset                                                     | Contract                                                  | Mined | Tentative | Offchain | Total |
> +=====================================================================================================================================================================+
> | 0e2bebb41a9092ed6870d01a26f08bb7bbe65c72fec1ead7933bd348e8e31b72 | contract:DivrtBqQ-ku1ocNA-aJvCLt7-vmXHL_w-erXkzvT-SOjjG3I |    83 |         2 |        0 |    85 |
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
>
> ```

### Force-close (example)

**Run:**

```bash
$ rgbldk channel open --node-id <node_id_b> --addr <node_b_p2p> --amount-sats 50000 --push-msat 1000000 --private

```

**Result:**

> ```text
> {
>   "user_channel_id": "bb85bfa702b518962f7ee0f34469326c"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7u3mgnwtgaskhvqmclwr4ft345cwugk9sujl3y

```

**Result:**

> ```text
> [
>   "2de3a19be04538db85e671446c829b4a36867f2afb76caabf455ccd8f43112e4",
>   "764e37cff365652adf90b49b1a565277b38ae0ed8fdac84326a24618860566d9",
>   "2abe75137a5e7225e739981f855cbf3cb184e20c459a11193cd96c8626d1c7d4",
>   "74a4cf5eac15bac4bd8f682e3ae3ee21d78a67cd80bbce57a8078634d9ec888a",
>   "3ed9aced395dea4ba4f1cb0bd4ee6fa34b916d7452f91999a20ca20710f1577a",
>   "47ec26fc018c8947a6f247cd4bef9e6d1c2a36731b51448e3912a6c4a2052f04"
> ]
>
> ```

**Run:**

```bash
$ rgbldk --yes channel force-close --user-channel-id <user_channel_id> --counterparty-node-id <node_id_b>

```

**Result:**

> ```text
> Channel force-close initiated.
>
> ```

## 11) Cleanup

Tear down the docker-compose stack and remove volumes.


### Disconnect peers

**Run:**

```bash
$ rgbldk peer disconnect <node_id_b>

```

**Result:**

> ```text
> Peer disconnected.
>
> ```

**Run:**

```bash
$ rgbldk peer ls

```

**Result:**

> ```text
> +---------+---------+-----------+-----------+
> | Node ID | Address | Connected | Persisted |
> +===========================================+
> +---------+---------+-----------+-----------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk peer disconnect <node_id_a>

```

**Result:**

> ```text
> Peer disconnected.
>
> ```

**Run:**

```bash
$ rgbldk peer ls

```

**Result:**

> ```text
> +---------+---------+-----------+-----------+
> | Node ID | Address | Connected | Persisted |
> +===========================================+
> +---------+---------+-----------+-----------+
>
> ```

**Run:**

```bash
$ rgbldk ctx use node-a

```

**Result:**

> ```text
> Switched to context "node-a".
> Next: run `rgbldk node status` to verify the connection.
>
> ```

```bash
$ docker compose -f crates/cli/docker-compose.yaml down -v

```
