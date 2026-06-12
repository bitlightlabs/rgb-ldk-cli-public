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

Note: this generator starts local source-mode `rgbldkd` daemons with temporary passphrase files, `--auto-init-keystore`, and `--auto-unlock` before running the rest of the commands.


### Commands

```bash
$ DOCKER_BUILDKIT=1 docker compose -f crates/cli/docker-compose.yaml up -d bitcoind chain-init esplora wait-esplora

```

```bash
$ (cd ../rgb-ldk-node && cargo build --bin rgbldkd)

```

```bash
$ cargo build -p rgbldk-cli --bin rgbldk

```

```bash
$ export PATH="$PWD/target/debug:$PATH"

```

### Endpoints

```text
node_a=http://127.0.0.1:8501
node_b=http://127.0.0.1:8502
esplora=http://127.0.0.1:59968

```

## 1) Contexts (ctx)

Contexts let you name daemon endpoints (e.g. `node-a`, `node-b`) and switch the default target without repeatedly passing `--connect`.


### Add + show

**Run:**

```bash
$ rgbldk ctx add node-a --url http://127.0.0.1:8501 --use-now

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
> +---------+--------+-----------------------+
> | Current | Name   | URL                   |
> +==========================================+
> | *       | node-a | http://127.0.0.1:8501 |
> |---------+--------+-----------------------|
> |         | node-b | http://127.0.0.1:8502 |
> +---------+--------+-----------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx show

```

**Result:**

> ```text
> node-a -> http://127.0.0.1:8501
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
> +--------------------+------------------+
> | Field              | Value            |
> +=======================================+
> | api_crate_version  | 0.7.0-bitlight.1 |
> |--------------------+------------------|
> | api_version        | v1               |
> |--------------------+------------------|
> | core_crate_version | 0.7.0-bitlight.1 |
> +--------------------+------------------+
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
>   [OK] Best Block Height: Height: 1553
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
> | best_block_height | 1553  |
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
>   "node_id": "03a40c278b4fb9dc536162fdfe8d8343de0cdd36ba1d6d4b420683929cbed8e546"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:9735
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
>   [OK] Best Block Height: Height: 1553
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty node id

```

**Result:**

> ```text
> {
>   "node_id": "03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:9736
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
> {"keystore_path":"/tmp/rgbldk-cli-example-93422m1x/keystore-init-demo/keystore","mnemonic":"vast address puppy envelope pioneer subject bid churn dizzy upper metal involve base alpha trouble joy nice middle pupil diet play stadium come matter","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-93422m1x/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-93422m1x/keystore-migrate-demo/keys_seed.bak.20260611-115835","ok":true}
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
> +---------------------+----------------+-----------+-----------+
> | Node ID             | Address        | Connected | Persisted |
> +==============================================================+
> | 03e13839...9d7a1e24 | 127.0.0.1:9736 | true      | true      |
> +---------------------+----------------+-----------+-----------+
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
>   "address": "bcrt1qatwshxz8fku6h4rf6sqddeshm5e4qmnu6zsdph"
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
>   "address": "bcrt1qv4xv9mmvp0nadqcmnjq8u770m9k9pnr0c54sn9"
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
> bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qatwshxz8fku6h4rf6sqddeshm5e4qmnu6zsdph 1

```

**Result:**

> ```text
> 743ba3672a53793d4052eb51e5f10c3092de63056a29d8617cb32693c5b67c8e
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qv4xv9mmvp0nadqcmnjq8u770m9k9pnr0c54sn9 1

```

**Result:**

> ```text
> 06f752df9747cad2380df561b1d37c008298d16cb9dd8a5c514fa6291566daff
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "7e47fdfd43aed7743be74498b4970bf31c1a1d3bf07c3f60b2909d4cf943b117",
>   "797b22ec0eda3512120a8775340b6f4a9bac568b8cddba4222426d491fe5633d",
>   "2f613f39ee0f3976945c237c2360af4827455066389a20567b25eb0c01b12ca3",
>   "5bca928f55ba3e77cd1ffb50ebccd98f6ec28187a74faf72706f32b9c3a4eff7",
>   "542cf7798e5e78f54ba4fb35effb3c1d67dbb08544179745c32851237d145d0a",
>   "2af1c13bd2097a5151d573a463d96e8f56f4856f0e530d35ef482411d19729f1"
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
$ rgbldk wallet utxos

```

**Result:**

> ```text
> +--------------------------------------------------------------------+--------------+-----------+--------+--------------+----------+--------------+
> | Outpoint                                                           | Value (sats) | Status    | Height | Availability | Lock Ref | Locked Until |
> +=================================================================================================================================================+
> | 743ba3672a53793d4052eb51e5f10c3092de63056a29d8617cb32693c5b67c8e:1 |  100,000,000 | Confirmed |   1554 | Available    | -        |            - |
> +--------------------------------------------------------------------+--------------+-----------+--------+--------------+----------+--------------+
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

This demonstrates a BTC L1 transfer between the two nodes by using a channel open with `--push-msat` (gives the receiver an initial balance), inspecting the public network graph once the channel confirms, trying both splice directions on the pure-BTC channel, and then using a cooperative `channel close` to settle on-chain.


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
$ rgbldk channel open --node-id <node_id_a> --addr <node_a_p2p> --amount-sats 120000 --push-msat 30000000

```

**Result:**

> ```text
> {
>   "user_channel_id": "4d698d0429f760727de7c8f34fd6e149"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "1f62f51a97051a6a04798064877a33b852e1e06998614f123062b5901a7f5e84",
>   "7ac8337aeae6d121d72476c08556d1ca8ae5f0d8d1920ed436f5c35836559881",
>   "7655bd486717615c1a979569dc81424cc20825fc37680bb462b65b6f194e8d96",
>   "7556bdc41f3b5d2a7c2542228a8ddc0a186f3ad0a3f6c4d3e47c08751a387745",
>   "72781859d99cf990ebcfafd9021286e2c28a1bf56c9fb472702d2de048ce3f4c",
>   "16aa79da4487569b241315706c4b532bf90b492e9fee5a05c4c5c7b8ff71f618"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable |
> +==============================================================================+
> | 4d698d04...4fd6e149 | 03a40c27...bed8e546 | 120000          | true  | true   |
> +---------------------+---------------------+-----------------+-------+--------+
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
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable |
> +==============================================================================+
> | d972a527...2cac9df4 | 03e13839...9d7a1e24 | 120000          | true  | true   |
> +---------------------+---------------------+-----------------+-------+--------+
>
> ```

#### Network graph

**Run:**

```bash
$ rgbldk graph nodes

```

**Result:**

> ```text
> +--------------------------------------------------------------------+
> | Node ID                                                            |
> +====================================================================+
> | 03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24 |
> |--------------------------------------------------------------------|
> | 03a40c278b4fb9dc536162fdfe8d8343de0cdd36ba1d6d4b420683929cbed8e546 |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 03e13839...9d7a1e24.
> Known channels: 1.
> +------------------+
> | SCID             |
> +==================+
> | 1715238139396096 |
> +------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph channels

```

**Result:**

> ```text
> +------------------+
> | SCID             |
> +==================+
> | 1715238139396096 |
> +------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph channel <scid>

```

**Result:**

> ```text
> Channel 1715238139396096 connects 03a40c27...bed8e546 and 03e13839...9d7a1e24.
> Capacity: unknown.
>
> 03a40c27...bed8e546 -> 03e13839...9d7a1e24: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1781179149       |
> |-------------+------------------|
> | CLTV delta  | 72               |
> |-------------+------------------|
> | Min HTLC    | 1 msat           |
> |-------------+------------------|
> | Max HTLC    | 108,000,000 msat |
> |-------------+------------------|
> | Base fee    | 1,000 msat       |
> |-------------+------------------|
> | Fee rate    | 0 ppm            |
> +-------------+------------------+
>
> 03e13839...9d7a1e24 -> 03a40c27...bed8e546: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1781179149       |
> |-------------+------------------|
> | CLTV delta  | 72               |
> |-------------+------------------|
> | Min HTLC    | 1 msat           |
> |-------------+------------------|
> | Max HTLC    | 108,000,000 msat |
> |-------------+------------------|
> | Base fee    | 1,000 msat       |
> |-------------+------------------|
> | Fee rate    | 0 ppm            |
> +-------------+------------------+
>
> ```

#### Splice commands

These local source-mode daemons run with `--rgb-enabled`, so the channel above is treated as an RGB channel. Current node behavior rejects splicing RGB channels, so the two commands below are shown as controlled examples of the current API error instead of successful mutations.


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
$ rgbldk channel splice-in --user-channel-id <user_channel_id> --counterparty-node-id <node_id_a> --splice-amount-sats 20000

```

**Result:**

> ```text
> HTTP 400: ChannelSplicingFailed
> message=Failed to splice channel.
> (exit 1)
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
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q0ask0aav6val662s8acd6k2edzr5m39dgvkqeq"
> }
>
> ```

**Run:**

```bash
$ rgbldk channel splice-out --user-channel-id <user_channel_id> --counterparty-node-id <node_id_b> --address <wallet_address> --splice-amount-sats 10000

```

**Result:**

> ```text
> HTTP 400: ChannelSplicingFailed
> message=Failed to splice channel.
> (exit 1)
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable |
> +==============================================================================+
> | d972a527...2cac9df4 | 03e13839...9d7a1e24 | 120000          | true  | true   |
> +---------------------+---------------------+-----------------+-------+--------+
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
> +---------------------+---------------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable |
> +==============================================================================+
> | 4d698d04...4fd6e149 | 03a40c27...bed8e546 | 120000          | true  | true   |
> +---------------------+---------------------+-----------------+-------+--------+
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "0a6f9638358f9fcdc048e8de07c45c8024cfc54fccb269e9bc5189c2b3bca49d",
>   "5aceaa7ff3c9e65f13f7cca0f1470799479448091f1920b9ef40eea603fcec86",
>   "3f86aeb4b763ef232082e1a5e17b2202a2c2a21c3c42c36d61721982e9f3d3dc",
>   "28cfc6ab3ed0907c09f4a6b42eaa193f10cb42b3bc22b54a07916490ea0b3376",
>   "7e2977d780e12d9e1618433c0aff984fc45c92c368a9dabc9bb8c6aa934a5fa3",
>   "3e944823f2aa6a0968956c60b13259a71c2f48c06aed36655faf98c8faec0420"
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
> BTC balance change: on-chain total +30,000 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (total)     |  1.0003 BTC |
> |--------------------------+-------------|
> | BTC On-chain (spendable) |     1.0 BTC |
> |--------------------------+-------------|
> | BTC Anchor reserve       |      0 sats |
> |--------------------------+-------------|
> | BTC Lightning (total)    | 30,000 sats |
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
> BTC balance change: on-chain total +87,989 sats, spendable +87,989 sats, anchor reserve 0 sats, lightning -87,654 sats.
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
> | BTC On-chain (total)     | 99,967,074 sats |
> |--------------------------+-----------------|
> | BTC On-chain (spendable) | 99,967,074 sats |
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
>   "address": "bcrt1qa2x9p3e58a9sa2jx6ce67g6jpuwfe20l9x9lwg"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb descriptor

```

**Result:**

> ```text
> RGB wallet descriptor:
> rgb(wpkh([ced04ff4/86h/827167h/0h]tpubDC6EcdagWMvWS8GKrAmkW18Pzv4def16wRUu2mCLKZ9meraedVcE6jbGEvqucnpHbj25PHiGy75X9L9MQBZs2eLWX7GuTBBFu7wDvTpZTSu/<0;1>/*),seals(),noise(f39a868a3a1e8b3608b0a15bd4ed75692e11ad32627d648e6cf309f045c451b4))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | ced04ff4    | m/86'/827167'/0' | tpubDC6EcdagWMvWS8GKrAmkW18Pzv4def16wRUu2mCLKZ9meraedVcE6jbGEvqucnpHbj25PHiGy75X9L9MQBZs2eLWX7GuTBBFu7wDvTpZTSu | wpkh([ced04ff4/86'/827167'/0']tpubDC6EcdagWMvWS8GKrAmkW18Pzv4def16wRUu2mCLKZ9meraedVcE6jbGEvqucnpHbj25PHiGy75X9L9MQBZs2eLWX7GuTBBFu7wDvTpZTSu/<0;1>/*)#m3dswg0a |
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb sign-message --message 7267626c646b206578616d706c65 --algorithm ecdsa --encoding hex

```

**Result:**

> ```text
> Message signed with ecdsa.
> +-----------------+--------------------------------------------------------------------+
> | Field           | Value                                                              |
> +======================================================================================+
> | Public key      | 03cedd2625b2c45eb2010a652dd9cfc48ba3636997a9cccbab17de7f63ee6f3721 |
> |-----------------+--------------------------------------------------------------------|
> | Derivation path | m/86'/827167'/0'                                                   |
> |-----------------+--------------------------------------------------------------------|
> | Encoding        | hex                                                                |
> |-----------------+--------------------------------------------------------------------|
> | Format          | Standard                                                           |
> |-----------------+--------------------------------------------------------------------|
> | Digest          | 2382eb4b6a524c95173ce0538ada9a17fa8e622d3a1b29bfe6f48c5bb3968ca8   |
> +-----------------+--------------------------------------------------------------------+
>
> Signature:
> 3045022100be9a696a0fa81cbe99263dd1f545bfe227b7e08c0dbca0d0f04ae5bbcd0e70ea0220612477dfc9126ea76f2e923c7c78916d535789b904105346af3ce7c00062e32f
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +-------------+
> | Issuer      |
> +=============+
> | demo-issuer |
> +-------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts ls

```

**Result:**

> ```text
> +------+--------+-----------+--------+-------------+
> | Name | Ticker | Precision | Issued | Contract ID |
> +==================================================+
> +------+--------+-----------+--------+-------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos ls

```

**Result:**

> ```text
> +----------+--------------+--------+--------+--------------+-------------+-------+
> | Outpoint | Value (sats) | Status | Height | Availability | Allocations | Roles |
> +================================================================================+
> +----------+--------------+--------+--------+--------------+-------------+-------+
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos summary

```

**Result:**

> ```text
> +----------+--------------+--------+-------+--------+
> | Outpoint | Value (sats) | Height | State | Assets |
> +===================================================+
> +----------+--------------+--------+-------+--------+
>
> ```

### Issue + share

Issuing a contract requires the RGB wallet to have spendable UTXOs. Fund two RGB wallet addresses and sync before issuing.


**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qu38m9ur6v0d3mst2zlf9exfm4m2wrg5tkugh5g"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> 1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> 83f30f51e2aa44c31c4aa8d92d553a7814e931a7be222894227ad44b93edee19
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "46158d5ff28d77ac61e0b185b650cce8fb4f74b82cf3a347fd872895a84b6d50"
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
$ rgbldk rgb utxos ls

```

**Result:**

> ```text
> +--------------------------------------------------------------------+--------------+------------+--------+--------------+-------------+----------------------------+
> | Outpoint                                                           | Value (sats) | Status     | Height | Availability | Allocations | Roles                      |
> +===================================================================================================================================================================+
> | 1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029:1 |   10,000,000 | In mempool |      - | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+------------+--------+--------------+-------------+----------------------------|
> | 83f30f51e2aa44c31c4aa8d92d553a7814e931a7be222894227ad44b93edee19:0 |      110,000 | In mempool |      - | Available    | -           | FeeSupport, BlindingTarget |
> +--------------------------------------------------------------------+--------------+------------+--------+--------------+-------------+----------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos summary

```

**Result:**

> ```text
> +--------------------------------------------------------------------+--------------+--------+-----------+--------+
> | Outpoint                                                           | Value (sats) | Height | State     | Assets |
> +=================================================================================================================+
> | 1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029:1 |   10,000,000 |      - | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | 83f30f51e2aa44c31c4aa8d92d553a7814e931a7be222894227ad44b93edee19:0 |      110,000 |      - | Available |        |
> +--------------------------------------------------------------------+--------------+--------+-----------+--------+
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos reserve --outpoint <rgb_outpoint> --ttl-secs 60

```

**Result:**

> ```text
> {
>   "reservation_id": "manual-reservation-1781179197-TbUTuXmENURExBgpjhraM9ZxhdIAT1dR",
>   "outpoint": "1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029:1",
>   "reserved_until_unix_secs": "1781179257"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos release --reservation-id <reservation_id>

```

**Result:**

> ```text
> Reservation released.
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
> Imported issuer demo-issuer.
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +-------------+
> | Issuer      |
> +=============+
> | demo-issuer |
> +-------------+
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
>   "contract_id": "contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE",
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
>       "detail": "1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029:1"
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
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
> | Name      | Ticker | Precision | Issued | Contract ID                                               |
> +=====================================================================================================+
> | DemoAsset | DEMO   |         0 |    100 | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> |-----------+-----------------------------------------------------------|
> | Mined     | 100                                                       |
> |-----------+-----------------------------------------------------------|
> | Tentative | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Offchain  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Archived  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Total     | 100                                                       |
> +-----------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE --out /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.
> Consignment key: contract_export_159f2b12b80bab5ea7aeff8b7d304df4f6cb36c28e69d7a35157e1763e899a77
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029:1
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip

```

**Result:**

> ```text
> {
>   "bytes": 2993,
>   "export": {
>     "checks": [
>       {
>         "name": "rgb_enabled",
>         "ok": true
>       },
>       {
>         "detail": "contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_0f2a7c0fb09e4d0a80fd40ff7b51b4285119e2327e888c949c73a51253a03502",
>     "contract_id": "contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE",
>     "ok": true
>   },
>   "out": "/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_0f2a7c0fb09e4d0a80fd40ff7b51b4285119e2327e888c949c73a51253a03502.
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.download.zip.
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
>   "address": "bcrt1qjvnfyzggg38amvdpxzmt8a6z7y3a2uf797dgzt"
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
> b4af85d6f0e61d7f430b27f1b05ac1c22e1fd42dcefdd6af886cfb7f3e86a69d
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "540df826c9546d3f86586e59f36d3221276a690643716ff015a7894b1e9b7a26"
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
$ rgbldk rgb contracts import --contract-id contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE --file /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_07f7ccecd1cce9b6b0ead0ae131e1cbbbc7ad078f8089514b24e4aa18fcf1779
> Imported contract contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.
> Consignment key: contract_import_07f7ccecd1cce9b6b0ead0ae131e1cbbbc7ad078f8089514b24e4aa18fcf1779
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts ls

```

**Result:**

> ```text
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
> | Name      | Ticker | Precision | Issued | Contract ID                                               |
> +=====================================================================================================+
> | DemoAsset | DEMO   |         0 |    100 | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> |-----------+-----------------------------------------------------------|
> | Mined     | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Tentative | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Offchain  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Archived  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Total     | 0                                                         |
> +-----------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts known <contract_id>

```

**Result:**

> ```text
> The node knows contract contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.
>
> ```

#### RGB UTXO lifecycle (fund/top-up/sweep)

`fund` creates empty RGB wallet outputs, `top-up` increases a single-asset RGB UTXO's bitcoin capacity, and `sweep` spends an empty RGB wallet output back to the BTC wallet. These low-level UTXO tools are different from `rgb onchain send`, which moves contract allocations between wallets.


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
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q28lkmyjuljhkqwwl7sfjd2uhuxnvaqjch7exed"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> 84d81d26a6057aed3ff3b056414a928214e1c58e5241ea1438f0dc9e84a6272a
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "10f19d9ecfc8f9774aa7027180696972d00fe62a47fb329d3e111dd4fb4caf9a"
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
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qnnswxgz3dkwvegvmvsuqa6acwjtwnsyxvftwlk"
> }
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qlv6a57gzqc6lqwu3hk0kwahl56vqnye9sllknd"
> }
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qk29rfmnm7phcn0am3xs4792plsvc53mae9lr8k"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"9ae4c5140a94871ce9ec1303cfab8f28526ad5252d3b8b9c931cc0aa9aea82e2","status":"broadcast","outputs":[{"address":"bcrt1qnnswxgz3dkwvegvmvsuqa6acwjtwnsyxvftwlk","value_sats":"30000","vout":0},{"address":"bcrt1qlv6a57gzqc6lqwu3hk0kwahl56vqnye9sllknd","value_sats":"28000","vout":1}],"change":{"address":"bcrt1qk29rfmnm7phcn0am3xs4792plsvc53mae9lr8k","value_sats":"941828","vout":2},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "42ebe2df85021a243824582208efd82b1e22d2b2c6914a6e581dbd5cc036bc37"
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
> BTC balance change: on-chain total -58,172 sats, spendable -1,000,000 sats, anchor reserve 0 sats, lightning 0 sats.
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
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q8apacw723sscv56ylzvtmslpu00m8sjh8tkk6a"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> df481c4b74b17fbba82eb8587234e484e7a661fa4e26be1430b7396ab7bdb134
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "742f5de749c372f43cd3a60b0cbf2edb3e18fd1ac1a17d34df501b08d297d924"
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
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q0yuq2eph6msxrrvecghzm62ps79v04fd055z70"
> }
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qfsu5xnvf5q55qlpcsrp58vu297caz5wggpp6yw"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO 1ece68b2001b72cb5e97c6600690d55919798318a012effc4c3234784fedc029:1.
> Broadcast transaction: 1b6d01cb6099e6fc3d6eda189a94349837a7c13f52ac07f8d1413c4b51b742d7
> Replacement output: bcrt1q0yuq2eph6msxrrvecghzm62ps79v04fd055z70 with 10,040,000 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE_1b6d01cb6099e6fc3d6eda189a94349837a7c13f52ac07f8d1413c4b51b742d7
> Change output: bcrt1qfsu5xnvf5q55qlpcsrp58vu297caz5wggpp6yw with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "4b0bd3bdd12a80043855e0cf7984ea1fb13668175ac94d070e38ab45ff3308be"
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
> BTC balance change: on-chain total -40,254 sats, spendable -40,254 sats, anchor reserve 0 sats, lightning 0 sats.
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
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qtwwm3lh8gk24yhzdylc02np8w32fa8ma3ewaky"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO 9ae4c5140a94871ce9ec1303cfab8f28526ad5252d3b8b9c931cc0aa9aea82e2:1.
> Broadcast transaction: bc7492dd9501dfc3b83f99f9c06e0a98632be411d375a9d77bf8dc03c8f6287d
> Sent 27,889 sats to bcrt1qtwwm3lh8gk24yhzdylc02np8w32fa8ma3ewaky.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "739137fd3c5c87dd586c1c504eea3037e03c3ad3c425ad349ede77756c500db0"
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
> BTC balance change: on-chain total 0 sats, spendable +27,889 sats, anchor reserve 0 sats, lightning 0 sats.
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
$ rgbldk ctx use node-b

```

**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
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
>   "invoice": "contract:tb@YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE/90@at:7KgzHvss-IZNvmYiY-n_sh1MYp-8TX9OffR-rWLaejjS-eklJzw/?expiry=2026-06-11T13:00:46.635798+00:00",
>   "blinding_utxo_used": "b4af85d6f0e61d7f430b27f1b05ac1c22e1fd42dcefdd6af886cfb7f3e86a69d:1"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain invoice-decode '<invoice>'

```

**Result:**

> ```text
> +------------------+-----------------------------------------------------------+
> | Field            | Value                                                     |
> +==============================================================================+
> | Contract         | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:7KgzHvss-IZNvmYiY-n_sh1MYp-8TX9OffR-rWLaejjS-eklJzw    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1781182846                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE/90@at:7KgzHvss-IZNvmYiY-n_sh1MYp-8TX9OffR-rWLaejjS-eklJzw/?expiry=2026-06-11T13:00:46.635798+00:00
> scope=contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:7KgzHvss-IZNvmYiY-n_sh1MYp-8TX9OffR-rWLaejjS-eklJzw
> data=90
> expiry=2026-06-11T13:00:46.635798+00:00
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
>   "txid": "fd1dcbf1bd60e5fd4bf013b743494d3018871ff22f02f64a6d8d68aa9ffadf48",
>   "consignment_key": "rgb_consignment_contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE_fd1dcbf1bd60e5fd4bf013b743494d3018871ff22f02f64a6d8d68aa9ffadf48"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE_fd1dcbf1bd60e5fd4bf013b743494d3018871ff22f02f64a6d8d68aa9ffadf48.
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-a-to-b-prechannel.zip.
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
> Accepted contract contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "142984f35edf4b82c2bbff427f96bee23909f7bc6c8c6e2fc34285d3b6c18da0",
>   "1eb93f4efc8d0766e55a5917a1c795fe69187726b4eae53f54253a0ba3016590",
>   "0d94cfb89f0766c7a8fd46f45b34d435b7803c2f4b31886a1396f4135a9742a0",
>   "002f259bd0ce7e6b7d1e0818306f405ab0c8b23e8cf6c161f8ab2ffdd528ce22",
>   "25ccc9c9b4b38a5064df43abd28656794229265c601a2a6d37d5818545098928",
>   "2797d2bb0692b7bb53afae05d3b710f00d62c09c54f5b98acdaa19d356307ddc"
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
$ rgbldk rgb contracts balance contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> |-----------+-----------------------------------------------------------|
> | Mined     | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Tentative | 90                                                        |
> |-----------+-----------------------------------------------------------|
> | Offchain  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Archived  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Total     | 90                                                        |
> +-----------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb onchain payments --contract-id <contract_id>

```

**Result:**

> ```text
> +------------------------------------------------------------------+-----------+-----------------------------------------------------------+--------+------+----------------------------------------------------------------------------------+
> | Payment ID                                                       | Status    | Contract                                                  | Amount | Txid | Consignment                                                                      |
> +=============================================================================================================================================================================================================================================+
> | 0869d2a67ce0768cb5179e03720ff438a77577d6e8b62193c73d0ef9209c7da7 | succeeded | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |     90 | -    | transfer_import_71b093c9afb39d8e512f4fa2b13c65b9da6ab52ee6d473af4efc438b9dd9fc95 |
> +------------------------------------------------------------------+-----------+-----------------------------------------------------------+--------+------+----------------------------------------------------------------------------------+
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
$ rgbldk rgb contracts balance contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> |-----------+-----------------------------------------------------------|
> | Mined     | 10                                                        |
> |-----------+-----------------------------------------------------------|
> | Tentative | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Offchain  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Archived  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Total     | 10                                                        |
> +-----------+-----------------------------------------------------------+
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
$ rgbldk channel open --node-id <node_id_b> --addr <node_b_p2p> --amount-sats 100000 --push-msat 20000000 --private --rgb-contract-id <contract_id> --rgb-asset-amount 10 --rgb-context 'http://<A_HOST>:8501/api/v1/rgb/consignments/{txid}?format=zip'

```

**Result:**

> ```text
> {
>   "user_channel_id": "682d5789320d19d61ff406e13bd1e1ea"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "1c5046454102c72692e8346bbe6f882b364f9d2ced7bc9dc81fa94b6123acdc2",
>   "79c0e3423fa5d099b178333d6ac3b946239b5453fc60ffdd8aab798386cbbc12",
>   "6a54c7c5b490738e56835671ea0427265de24554b222cf66c37d0cd643326e63",
>   "689306d2e5103adf394aff43cbdbafcc755113ef79c6941c9b9c623e9a60d640",
>   "0abbf0c41268046436d60917db5326015944a4428d3235d4014f85357a5284ce",
>   "2d2a49f05287493310834231a74facc48c0c6cc5c8367843cd53ecf8c9e1eda9"
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
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote |
> +=============================================================================================================================+
> | 682d5789...3bd1e1ea | 03e13839...9d7a1e24 | 100000          | false | false  | contract...-GxfVGkE | 10        | 0          |
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
>   "invoice": "contract:tb@YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE/7@at:28UbWHcK-YbF_PvmS-WM0zrdbH-wGL3ZdwU-fBkSKSvY-nYmJ_Q/?expiry=2026-06-11T13:01:14.141963+00:00",
>   "blinding_utxo_used": "83f30f51e2aa44c31c4aa8d92d553a7814e931a7be222894227ad44b93edee19:0"
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
>   "txid": "51ec0c79a4b9d1a0dd163908e8598113d5385413a1ad4c82acb8b1af0a32bbe8",
>   "consignment_key": "rgb_consignment_contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE_51ec0c79a4b9d1a0dd163908e8598113d5385413a1ad4c82acb8b1af0a32bbe8"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE_51ec0c79a4b9d1a0dd163908e8598113d5385413a1ad4c82acb8b1af0a32bbe8.
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-b-to-a.zip.
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
> Accepted contract contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "6eba610c1297bb3abd05e099527a7adfd0155e71834b8e84fc57c50a03797b0e",
>   "7d1cd2013d21cf70183544b733c1a7bdc1d184c0d48e1b3ea19017d0d08c5f8e",
>   "6a6cafae1bfe9dc727a6227bde39cbd79a451818f93bd5f7d4449b0e6e3ba936",
>   "1db042792bcbf3b5f835d19c6f6d78bb2d5b80a4e81092f89e9a7095e2e20348",
>   "3f1bfb65a7b925223970172a132421634b14a81eccf28ba515456f857f3fd84b",
>   "08c1fed6ff89100aa14ccf0056cbb57820132e87997d1d57e33b8af277c30387"
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
$ rgbldk rgb contracts balance contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |
> |-----------+-----------------------------------------------------------|
> | Mined     | 7                                                         |
> |-----------+-----------------------------------------------------------|
> | Tentative | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Offchain  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Archived  | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Total     | 7                                                         |
> +-----------+-----------------------------------------------------------+
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
>   "invoice": "lnbcrt100n1p4z4guhdq8v3jk6mcnp4q0snswwnvm5ze6paulnx8vkwjw4rsrkyvvn6xwa4v56tr0ya0g0zgpp5htxx2709e5g0035kp3mn60e4906auryhjxktthv3xf8ft8ysywassp5jjnr7hz592kg42gs9496yy0ejadyk2nx5h7dfvk5aqdz9vcyqvas9qyysgqcqzp2xqrrssrzjqwjqcfutf7uac5mpvt7larvrg00qehfkhgwk6j6zq6pe9897mrj5vqqqqyqq2nsqqyqqqqlgqqqqqqqqfq6zpp644ykv4vlf0je09ref3jxvfwe8hnsdnfmet89capyvh2p5vsyz94slwer03den9wwf5ttngte3y5vmmeq5fledf30eay00unwrqpayvkfa"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay invoice decode <invoice>

```

**Result:**

> ```text
> +--------------+--------------------------------------------------------------------+
> | Field        | Value                                                              |
> +===================================================================================+
> | payment_hash | bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24 |
> |--------------+--------------------------------------------------------------------|
> | amount_msat  | 10,000                                                             |
> |--------------+--------------------------------------------------------------------|
> | expiry_secs  | 3600                                                               |
> +--------------+--------------------------------------------------------------------+
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
>   "payment_id": "bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb",
>   "preimage": "2b9ca7c273864b72908ec03aa5f2410cbac9ea202fc18962e310bd8540c187ce",
>   "amount_sats": "10",
>   "destination": "03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb
>
> ```

**Run:**

```bash
$ rgbldk pay get bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"bacc6579e5cd10f7c6960c773d3f352bf5de0c9791acb5dd91324e959c9023bb","preimage":"2b9ca7c273864b72908ec03aa5f2410cbac9ea202fc18962e310bd8540c187ce","secret":"94a63f5c542aac8aa9102d4ba211f9975a4b2a66a5fcd4b2d4e81a22b304033b"} |
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
$ rgbldk --color never --output json --pretty pay invoice create --desc demo-send --amount-msat 13000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt130n1p4z4guldq0v3jk6medwdjkueqnp4q0snswwnvm5ze6paulnx8vkwjw4rsrkyvvn6xwa4v56tr0ya0g0zgpp5ckr9d7vwnrz4x3x70s4nft3js0v6zwy7wus2d2wn7m9tekr5srnqsp57vcslcwlqcvcup0xmwcjgzl5q8vf9nwpfjh2c0x4g3snuak58lpq9qyysgqcqzp2xqrrssrzjqwjqcfutf7uac5mpvt7larvrg00qehfkhgwk6j6zq6pe9897mrj5vqqqqyqq2nsqqyqqqqlgqqqqqqqqfqc5jwjeq0c8vmk3w886sp32xj98uhe6rp0yndgzjvsr244m20lkxrkafh8v85a3vv2mn4uuytst6et3sk7guptlyxz9j9punt9n9ytdgpmtwg9s"
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
$ rgbldk --color never --output json --pretty pay invoice send <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "c58656f98e98c55344de7c2b34ae3283d9a1389e7720a6a9d3f6cabcd87480e6"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait c58656f98e98c55344de7c2b34ae3283d9a1389e7720a6a9d3f6cabcd87480e6 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> c58656f98e98c55344de7c2b34ae3283d9a1389e7720a6a9d3f6cabcd87480e6
>
> ```

**Run:**

```bash
$ rgbldk pay get c58656f98e98c55344de7c2b34ae3283d9a1389e7720a6a9d3f6cabcd87480e6

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | c58656f98e98c55344de7c2b34ae3283d9a1389e7720a6a9d3f6cabcd87480e6                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"c58656f98e98c55344de7c2b34ae3283d9a1389e7720a6a9d3f6cabcd87480e6","preimage":"cf5e62d8d078f0cbe55afc3a4391bbd96cf14fabce20b978412d2c8f341bda9a","secret":"f3310fe1df06198e05e6dbb1240bf401d892cdc14caeac3cd544613e76d43fc2"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 13,000 msat                                                                                                                                                                                                                                   |
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
>   "invoice": "lnbcrt1p4z4gaydqdv3jk6medweshynp4q0snswwnvm5ze6paulnx8vkwjw4rsrkyvvn6xwa4v56tr0ya0g0zgpp5mh9wr6wv8g0vc2xrel0jfunxp3q9takygy3y3p2szn7lxgucj6sqsp50qr59p2lc09kjje53hq44gyqqwzcgk946c9levykm544nas6cm0q9qyysgqcqzp2xqrrssrzjqwjqcfutf7uac5mpvt7larvrg00qehfkhgwk6j6zq6pe9897mrj5vqqqqyqq2nsqqyqqqqlgqqqqqqqqfqc04xpaj05qxvfxvqdn7dfz99rvmaglw28vawywwd7rtf0gj0w64998rp5qqhyf7n8274n8cjufdjyxrzx7mmd9aplmlnfd7nthv8c9cp8l39u3"
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
$ rgbldk --color never --output json --pretty pay invoice send-using-amount --invoice <invoice> --amount-msat 11000

```

**Result:**

> ```text
> {
>   "payment_id": "ddcae1e9cc3a1ecc28c3cfdf24f2660c4055f6c4412248855014fdf3239896a0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ddcae1e9cc3a1ecc28c3cfdf24f2660c4055f6c4412248855014fdf3239896a0 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ddcae1e9cc3a1ecc28c3cfdf24f2660c4055f6c4412248855014fdf3239896a0
>
> ```

**Run:**

```bash
$ rgbldk pay get ddcae1e9cc3a1ecc28c3cfdf24f2660c4055f6c4412248855014fdf3239896a0

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | ddcae1e9cc3a1ecc28c3cfdf24f2660c4055f6c4412248855014fdf3239896a0                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"ddcae1e9cc3a1ecc28c3cfdf24f2660c4055f6c4412248855014fdf3239896a0","preimage":"55404a28f1a7f29472facac3e6b72a8db8b704986b68468473ca700b231cddcf","secret":"780742855fc3cb694b348dc15aa08003858458b5d60bfcb096dd2b59f61ac6de"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 11,000 msat                                                                                                                                                                                                                                   |
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
$ rgbldk pay invoice create-for-hash --desc demo-hold-claim --amount-msat 14000 --payment-hash <payment_hash>

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt140n1p4z4ga2dqcv3jk6meddphkcepdvdkxz6tdnp4q0snswwnvm5ze6paulnx8vkwjw4rsrkyvvn6xwa4v56tr0ya0g0zgpp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp50ygpt8cldtlkzz5a6tmn77hhffszxqdajuykn9whkw03nymtd5eq9qyysgqcqzp2xqrrssrzjqwjqcfutf7uac5mpvt7larvrg00qehfkhgwk6j6zq6pe9897mrj5vqqqqyqq2nsqqyqqqqlgqqqqqqqqfq80yfj64407rqg3cf5df0elahpddxslplc5ulqfzu9whpqlfxpcr9z7zwkp4u8mem0pndqh4mmax229y8lhy46prywget3mnldytn5ssqg72fe6"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay invoice decode <invoice>

```

**Result:**

> ```text
> +--------------+--------------------------------------------------------------------+
> | Field        | Value                                                              |
> +===================================================================================+
> | payment_hash | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24 |
> |--------------+--------------------------------------------------------------------|
> | amount_msat  | 14,000                                                             |
> |--------------+--------------------------------------------------------------------|
> | expiry_secs  | 3600                                                               |
> +--------------+--------------------------------------------------------------------+
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
$ rgbldk --color never --output json --pretty pay invoice send <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c"
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
$ rgbldk pay invoice claim-for-hash --payment-hash <payment_hash> --preimage <preimage> --claimable-amount-msat 14000

```

**Result:**

> ```text
> [OK] Claim
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
$ rgbldk pay wait 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c
>
> ```

**Run:**

```bash
$ rgbldk pay get 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":"4242424242424242424242424242424242424242424242424242424242424242","secret":"7910159f1f6aff610a9dd2f73f7af74a602301bd97096995d7b39f19936b6d32"} |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 14,000 msat                                                                                                                                                                                                                                   |
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
$ rgbldk pay invoice create-for-hash --desc demo-hold-fail --amount-msat 15000 --payment-hash <payment_hash>

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt150n1p4z4gandqhv3jk6meddphkcepdveskjmqnp4q0snswwnvm5ze6paulnx8vkwjw4rsrkyvvn6xwa4v56tr0ya0g0zgpp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp5tf87hk69qjcvn057vwvj3tagrlqqax7j3f098g5lmmztum3wqtvs9qyysgqcqzp2xqrrssrzjqwjqcfutf7uac5mpvt7larvrg00qehfkhgwk6j6zq6pe9897mrj5vqqqqyqq2nsqqyqqqqlgqqqqqqqqfqyp4sevh3d86jqq73eyjv7femsn5ms5d505d9tfvsrx33kl04j4a9a2qn042y6cy6kfazq50qunsnnnvcf8qz0aylyadfut2f9tk8wlqqrlmxky"
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
$ rgbldk --color never --output json --pretty pay invoice send <invoice>

```

**Result:**

> ```text
> {
>   "payment_id": "4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24"
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
$ rgbldk pay invoice fail-for-hash <payment_hash>

```

**Result:**

> ```text
> [OK] Fail
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
$ rgbldk pay get 4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24

```

**Result:**

> ```text
> +-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                           |
> +===================================================================================================================================================================================================+
> | id              | 4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24                                                                                                                |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                        |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                                        |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                          |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"5a4febdb4504b0c9be9e639928afa81fc00e9bd28a5e53a29fdec4be6e2e02d9"} |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 15,000 msat                                                                                                                                                                     |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | -                                                                                                                                                                               |
> +-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt120n1p4z4gaudq0v3jk6medvfskx6cnp4qwjqcfutf7uac5mpvt7larvrg00qehfkhgwk6j6zq6pe9897mrj5vpp50y0lp4sequx7amj5enn94mf8w2s5a8xz3tfludy54ff3dvh54urqsp5aulrhsgru7ty25njrjz9n2227hxqdaen80mrfx75phxcfq06s3gq9qyysgqcqzp2xqrrssrzjq0snswwnvm5ze6paulnx8vkwjw4rsrkyvvn6xwa4v56tr0ya0g0zgqqqqyqqfxsqqcqqqqlgqqqqqqqqfqtlr3xx6s09elh5gdawr48tr5zg88qwevcxwtrtk8zutpt55kadx4eydnsq4d65h99x27drcdp68qppld7rykzylw8y3asgpx0kgkyhcqrdnh3t"
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
>   "payment_id": "791ff0d619070deeee54cce65aed2772a14e9cc28ad3fe3494aa5316b2f4af06",
>   "preimage": "b0d262a841224217a0dc75d8ba40b1157b144d1b4f9ef9780d6fcaeab100e59b",
>   "amount_sats": "12",
>   "destination": "03a40c278b4fb9dc536162fdfe8d8343de0cdd36ba1d6d4b420683929cbed8e546",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 791ff0d619070deeee54cce65aed2772a14e9cc28ad3fe3494aa5316b2f4af06 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 791ff0d619070deeee54cce65aed2772a14e9cc28ad3fe3494aa5316b2f4af06
>
> ```

**Run:**

```bash
$ rgbldk pay get 791ff0d619070deeee54cce65aed2772a14e9cc28ad3fe3494aa5316b2f4af06

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 791ff0d619070deeee54cce65aed2772a14e9cc28ad3fe3494aa5316b2f4af06                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"791ff0d619070deeee54cce65aed2772a14e9cc28ad3fe3494aa5316b2f4af06","preimage":"b0d262a841224217a0dc75d8ba40b1157b144d1b4f9ef9780d6fcaeab100e59b","secret":"ef3e3bc103e7964552721c8459a94af5cc06f7333bf6349bd40dcd8481fa8450"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx52436vgvyqgqqcvqqqqpqqqqx3furhduhflyjwacntj22zjwvl2papxjlqmgjdvf8erk7c0n2x07qypcskfpwf6553g7h833ng3eqa4j4lscdjjjxznpt407l600qdjp53qqwnpx9eutj9grkgua6zcu4u30vaw289a9m2lsj0ezgfktqs8ca3j8zqllvugpg2fgjsp5kyr437c6psk00wmknj3hpczm2ral2525zntg5rnqssvps3p7cdaj6pah9flj0pzqehgerq8gqgxh5mtknw7h65m8vglpyead585ukscn6ut8nqdgs4vmvgn3vggryl6a60k7uau76reu8s4g3w34t6280yjtma2fjgge9gagw3yfl4yq"
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
> | offer_id                  | e066315c1c8b075ad500890371426b05da1a7122fa5e45cb1a952b4fdef32fdb   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 0327f5dd3edee779ed0f3c3c2a88ba355e9477924bdf549921192a3a874489fd48 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1781182931                                                         |
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
>   "payment_id": "6101da03cb534672acddb39195ba8c0e58c5d9dd0e539e2a16a2b6d7f16f20fa"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 6101da03cb534672acddb39195ba8c0e58c5d9dd0e539e2a16a2b6d7f16f20fa --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 6101da03cb534672acddb39195ba8c0e58c5d9dd0e539e2a16a2b6d7f16f20fa
>
> ```

**Run:**

```bash
$ rgbldk pay get 6101da03cb534672acddb39195ba8c0e58c5d9dd0e539e2a16a2b6d7f16f20fa

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | 6101da03cb534672acddb39195ba8c0e58c5d9dd0e539e2a16a2b6d7f16f20fa                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"e066315c1c8b075ad500890371426b05da1a7122fa5e45cb1a952b4fdef32fdb","payer_note":null,"payment_hash":"2d1b2e5d6cff9b0b60a1da28a431222fdaf7c62d6cfd45e3b13be61a850acdef","quantity":null} |
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
>   "refund": "lnr1qqstfr9z55svvhm807tt0qarnwp253x4huavagkxnkxrsxe9nq0a8rq2qq8qg632k8v4qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssy2exevxl0zzjgyl8wddkrt2u4pq6tfxewvqvyn69e80vdjfp2vh6ty9hyetxw4hxgttyv4kk7kkzqqqqvxqqqqqsqqqz3vp04tdwnq5k6jnneqnvxzaga8pcepcyxq6hm03yvkrglyf8th4qzqusmdt7vemk0fkmrpkqjzm35ye5rqaq6sndhr3fpyl208yum3n5ucq8fern738l2y06nwkauf82l2ccaag9nhpzdl6ly4ncx907crwr3z5u9q4ggkq099z2607r2e2x6905tunywnkwhql30s7yrt82j8967rd5afxsar4ck7yk9hclfym9nknzwldy24uz3vf5rnmjdy50595gqmu3hstdxs6fy8txa7tlr9tmjctpn2rjzkg",
>   "payment_id": "ddff86c1243d7ad0cfc4e1ce18512537094bc93c117604a2ba17e25e2a82f7cf"
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
> | absolute_expiry_unix_secs | 1781182937                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 022b26cb0df78852413e7735b61ad5ca841a5a4d97300c24f45c9dec6c921532fa |
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
> lni1qqstfr9z55svvhm807tt0qarnwp253x4huavagkxnkxrsxe9nq0a8rq2qq8qg632k8v4qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssy2exevxl0zzjgyl8wddkrt2u4pq6tfxewvqvyn69e80vdjfp2vh6ty9hyetxw4hxgttyv4kk7kkzqqqqvxqqqqqsqqqz3vp04tdwnq5k6jnneqnvxzaga8pcepcyxq6hm03yvkrglyf8th4qzqusmdt7vemk0fkmrpkqjzm35ye5rqaq6sndhr3fpyl208yum3n5ucq8fern738l2y06nwkauf82l2ccaag9nhpzdl6ly4ncx907crwr3z5u9q4ggkq099z2607r2e2x6905tunywnkwhql30s7yrt82j8967rd5afxsar4ck7yk9hclfym9nknzwldy24uz3vf5rnmjdy50595gqmu3hstdxs6fy8txa7tlr9tmjctpn2rjzkdqacp7zwpe6dnwst8g8hn7vcaje6f65wqwc33j0gemk4jnfvdun4apufqz8zg5lk4zchu8uy0etl59pyd7vdlp62tcvacc652eeemru6zqpcnqzq66jtd3hha3cs862tdmhce9y29epncp8h3j2csekn54h9h2rxzgzsqgszkdgwxkruz28tnxpq6c3dp9r8esmu7hl0sy7xyqa9fn6sylajzpw4dlzpfn8njzxfzs93px87dek35n5gulpr4kz376ylkt2cgl85vukhudm9mwukgjhjw0wnej995urz6mumhzd0ul82ez7j36t76ttsd2kej0aymacay7nsn2mmqg9kjc5lf9sdj3trjqsmplzca6lh99gg22vf2d2lguuxdzrsqqqqqqqqqqqqqq9gqqqqqqqqqqqqgayjedltzjqqqqqq9yq34z4g7v4qsgkez09u0g4jk2hkac9zpkzegjll70c25cl9jakgwqvdxm94eqrld2qggwrtsrqgqqpvppqwqt7naq9vrhcv535qcg8fkufmxnpff3g3u98dvragpww9j8k5ndeuzqe2k4fdv78hxwrkgamwzj3wq2augxjj9ep3f8sv7dtgqyhxeytgff3t7nv2yau35t058yph8e0e0lsnc4nj8zdl5jmc4nkcfk023vdts
> payment_id: 8b644f2f1e8acacabdbb82883616512fffcfc2a98f965db21c0634db2d7201fd
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
$ rgbldk pay wait ddff86c1243d7ad0cfc4e1ce18512537094bc93c117604a2ba17e25e2a82f7cf --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ddff86c1243d7ad0cfc4e1ce18512537094bc93c117604a2ba17e25e2a82f7cf
>
> ```

**Run:**

```bash
$ rgbldk pay get ddff86c1243d7ad0cfc4e1ce18512537094bc93c117604a2ba17e25e2a82f7cf

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | ddff86c1243d7ad0cfc4e1ce18512537094bc93c117604a2ba17e25e2a82f7cf                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"8b644f2f1e8acacabdbb82883616512fffcfc2a98f965db21c0634db2d7201fd","quantity":null} |
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
>   "refund": "lnr1qqszm70v9mfxeey344s2486rvzw502p0r6uqzmn3rqz4wa7g70dap9q2qq8qg632k804qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqssx6sw9lcwqlelcnvkyavrfwly3fgq9zwwfvcyepxpg3apcvedvzt7tyfhyetxw4hxgttpvfskuer0dckkgetddadvyqqqqcvqqqqpqqqqxayxmj0cuhrrm629w70rz30fvz6z9snx83zhw8ujyrtxhv5zqt0gqype5yuvw562rxkhjlcn4t2tln4sy76vs06d6k6r2dlpfyavsg6eqmgqwny0f9w7wv45kylh2h4k3wtyctdy37agtwduraq2797w90dkh7j2nf4attt2tw7f7h4rlcaskcwpylkqzsv5xvre59gjpkfw5w2jxm75lvzxf6vttj0r2lsdmuq8g6vyzvk4wg02e2d6a25urxxw04hy2fszwdx3x9sef9l90c97tq34er6vdf29agns",
>   "payment_id": "af44b5caaefc7ebaede3ad2933b4b75e1e96fec9444285b5229d6b9c43abddef"
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

### Async payments (role/configuration checks)

Async-payment commands require dedicated node roles and additional handshake state. In this simple two-node demo they intentionally surface the current configuration/state errors instead of hanging.


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
$ rgbldk pay async receive-offer

```

**Result:**

> ```text
> HTTP 400: OfferCreationFailed
> message=Failed to create offer.
> (exit 1)
>
> ```

**Run:**

```bash
$ rgbldk pay async set-static-invoice-server-paths --paths-hex deadbeef

```

**Result:**

> ```text
> HTTP 400: invalid blinded path encoding
> (exit 1)
>
> ```

**Run:**

```bash
$ rgbldk pay async blinded-paths-for-recipient --recipient-id-hex 010203

```

**Result:**

> ```text
> HTTP 503: node is not in async-payments Server role
> (exit 1)
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
> | 2a27a684...261dd884 | Succeeded | Onchain      | Inbound  | 1,000,000,000   | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 425ed4e4...6e55384c | Succeeded | Bolt11       | Outbound | 14,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | e282ea9a...14c5e49a | Succeeded | Onchain      | Outbound | 58,000,000      | 172,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 34b1bdb7...4b1c48df | Succeeded | Onchain      | Inbound  | 500,000,000     | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | ddff86c1...2a82f7cf | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 8e7cb6c5...67a33b74 | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 7d28f6c8...dd9274bc | Succeeded | Onchain      | Inbound  | 27,889,000      | 111,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | e6993d98...b0eae07e | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 6101da03...f16f20fa | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | d742b751...cb016d1b | Succeeded | Onchain      | Outbound | 40,000,000      | 254,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | ddcae1e9...239896a0 | Succeeded | Bolt11       | Outbound | 11,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | c58656f9...d87480e6 | Succeeded | Bolt11       | Outbound | 13,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | af44b5ca...43abddef | Failed    | Bolt12Refund | Outbound | 1,111           | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | bacc6579...9c9023bb | Succeeded | Bolt11       | Outbound | 10,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 791ff0d6...b2f4af06 | Succeeded | Bolt11       | Inbound  | 12,000          | -          |
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
$ rgbldk --color never --output json --pretty rgb ln invoice create-for-hash --contract-id <contract_id> --asset-amount 2 --payment-hash <payment_hash> --desc "rgb ln hold demo" --btc-carrier-amount-msat 3000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt30u1p4z4g7kdq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp56v4r958tlzynvjs5yuqj2nmzdhgyy5yqhhw0rmevzefjl2svyxrs9qrsgqxqrrsscqpjlp5vyemwlnlk4n7q75u4rvk6kn6xqgwgnu3lpz6jd93glh3k974rfqs7qpz28kseurajm357lcmt22qh22jvamnffclwfz7wlkngulvcshnpd0pvdfhrujlyhjtlu38r8kesk3unlammx2ljvzp5f7gr0lufhg5segqkat349"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb ln invoice decode <invoice>

```

**Result:**

> ```text
> +-----------------------+--------------------------------------------------------------------+
> | Field                 | Value                                                              |
> +============================================================================================+
> | Payment hash          | bb391415c05e39d77ca17381d3be3f7d0cd5e5332e5a579311adaa0aa62106e9   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE          |
> |-----------------------+--------------------------------------------------------------------|
> | Asset amount          | 2                                                                  |
> +-----------------------+--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln invoice create --contract-id <contract_id> --asset-amount 5 --desc "rgb ln demo" --btc-carrier-amount-msat 5000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt50u1p4z4g7cdqjwfnkygrvdcsxgetddupp5dxrmr2dv0uvj7zgffvy68pfc052v8duxcf7zm9maah929wj309nssp5g4yceckp6hu5n6jghzkfsg92ggk75uurgjfkrgmfkuzmf3rnn3zq9qrsgqxqrrsscqpjlp5vyemwlnlk4n7q75u4rvk6kn6xqgwgnu3lpz6jd93glh3k974rfqs7qp9u6kv7hmt83k7604w8e96guqezatzhr62wu7asgt5xcmhvpqq0g2842659ze3wwc52ckykcgrpulrpsfe552prtfd9xed3gkt3hdlwesqzf89ur"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb ln invoice decode <invoice>

```

**Result:**

> ```text
> +-----------------------+--------------------------------------------------------------------+
> | Field                 | Value                                                              |
> +============================================================================================+
> | Payment hash          | 6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 03e13839d366e82ce83de7e663b2ce93aa380ec46327a33bb56534b1bc9d7a1e24 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE          |
> |-----------------------+--------------------------------------------------------------------|
> | Asset amount          | 5                                                                  |
> +-----------------------+--------------------------------------------------------------------+
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
>   "payment_id": "6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967
>
> ```

**Run:**

```bash
$ rgbldk pay get 6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"6987b1a9ac7f192f09094b09a385387d14c3b786c27c2d977dedcaa2ba517967","preimage":"22d89717f8d14143616bb07a4bdbc7644c90b7d5298fc68d18df6a17ddd10251","rgb":{"asset_amount":"5","contract_id":"contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE","direction":"Outbound","is_swap":false},"secret":"45498ce2c1d5f949ea48b8ac9820aa422dea7383449361a369b705b4c4739c44"} |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 5,000,000 msat                                                                                                                                                                                                                                                                                                                                                                            |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                                                                                                                                                                    |
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
$ rgbldk --color never --output json --pretty rgb ln invoice create --contract-id <contract_id> --asset-amount 3 --desc "rgb ln demo back" --btc-carrier-amount-msat 4000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt40u1p4z4g7ldq6wfnkygrvdcsxgetddusxyctrdvpp5wy96h9yxq22m3nkkatcmms8lws3xqtkdm7tqat6e8uqdz3js9emqsp5hh0sxpdgt82z7xc3d6h96lye7wamqx9t290kgxr6t6jcufv6l98q9qrsgqxqrrsscqpjlp5vyemwlnlk4n7q75u4rvk6kn6xqgwgnu3lpz6jd93glh3k974rfqs7qprwy628zdp6mlxa7dsre7c473mc5ep3erh9sumu8vwl8n3h7ean9w3yp9tgx23u0ss0npw07hzmrlx06zr6usrs223xfjy5pwcvfcxlagq57dxh8"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb ln invoice decode <invoice>

```

**Result:**

> ```text
> +-----------------------+--------------------------------------------------------------------+
> | Field                 | Value                                                              |
> +============================================================================================+
> | Payment hash          | 710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 03a40c278b4fb9dc536162fdfe8d8343de0cdd36ba1d6d4b420683929cbed8e546 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE          |
> |-----------------------+--------------------------------------------------------------------|
> | Asset amount          | 3                                                                  |
> +-----------------------+--------------------------------------------------------------------+
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
>   "payment_id": "710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76
>
> ```

**Run:**

```bash
$ rgbldk pay get 710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"710bab94860295b8ced6eaf1bdc0ff7422602ecddf960eaf593f00d146502e76","preimage":"b65d71bed8afd9c4a4cb1fe1131b3de127c6e2c5f64d784c928473e8356c5c09","rgb":{"asset_amount":"3","contract_id":"contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE","direction":"Outbound","is_swap":false},"secret":"bddf0305a859d42f1b116eae5d7c99f3bbb018ab515f64187a5ea58e259af94e"} |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 4,000,000 msat                                                                                                                                                                                                                                                                                                                                                                            |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                                                                                                                                                                                                                                                                                                                                                    |
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote |
> +=============================================================================================================================+
> | 682d5789...3bd1e1ea | 03e13839...9d7a1e24 | 100000          | true  | true   | contract...-GxfVGkE | 8         | 2          |
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
> | User Channel ID     | Counterparty        | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote |
> +=============================================================================================================================+
> | 5d224561...0f018f66 | 03a40c27...bed8e546 | 100000          | true  | true   | contract...-GxfVGkE | 2         | 8          |
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
>   "payment_id": "0072e8e3d1b0147cb67c072476afa9eb82156159e2ed0c747054e8789f7a79ca"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 0072e8e3d1b0147cb67c072476afa9eb82156159e2ed0c747054e8789f7a79ca --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 0072e8e3d1b0147cb67c072476afa9eb82156159e2ed0c747054e8789f7a79ca
>
> ```

**Run:**

```bash
$ rgbldk pay get 0072e8e3d1b0147cb67c072476afa9eb82156159e2ed0c747054e8789f7a79ca

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 0072e8e3d1b0147cb67c072476afa9eb82156159e2ed0c747054e8789f7a79ca                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"0072e8e3d1b0147cb67c072476afa9eb82156159e2ed0c747054e8789f7a79ca","preimage":"304bbefd2a1a72c11b4a6d6bce5dbc25409984824a5fabb79ad53219a162339c"} |
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
>   "payment_id": "9e9ea37865aa30be705a93ae289dbc1feda127f0af391f3aa672ea07573d9c1b"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 9e9ea37865aa30be705a93ae289dbc1feda127f0af391f3aa672ea07573d9c1b --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 9e9ea37865aa30be705a93ae289dbc1feda127f0af391f3aa672ea07573d9c1b
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 9e9ea37865aa30be705a93ae289dbc1feda127f0af391f3aa672ea07573d9c1b

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 9e9ea37865aa30be705a93ae289dbc1feda127f0af391f3aa672ea07573d9c1b                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"9e9ea37865aa30be705a93ae289dbc1feda127f0af391f3aa672ea07573d9c1b","preimage":"f3b6d395c9d23acad1eb00c54d2a373fd4e109863eba06e6bc4623012087a4f6"} |
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
$ rgbldk --connect http://127.0.0.1:8501 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=0e7f12de3fe5da60ca47b20acdda5caab39bc4d610f033e667e1ae43561e70bf:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=0e7f12de3fe5da60ca47b20acdda5caab39bc4d610f033e667e1ae43561e70bf:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=d972a527a27857ea35685fd22cac9df4
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events handled

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
> ChannelReady user_channel_id=4d698d0429f760727de7c8f34fd6e149
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "342dcad957c6907ff5a823c86754ad9048159789752d5e31b4baa9126ae9f17c",
>   "50ae0b575ad14c81846aed6cb83acfbe6c120673ceea1bd2b092ba1c5720221b",
>   "5d2164587b5a5d1691d44e060fa8bdc511a236f74c344daff74e9733d5a658b4",
>   "61eab1d67c229ead1f3b417c55cbef19b537fe133c9f6a40b84b5bd38e0f8423",
>   "5d08b87ee71fb8998f3bdbeb605fa13a68c151a20868d7b60c0d5510a3b88585",
>   "540b3789367a5c49c1ec71bfbe1ce6156c1e2491d8956be5adf72ec0e9d42efc"
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
> BTC balance change: on-chain total +74,582 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +21,047 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (total)     | 1.01534045 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.01459463 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    71,812 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |     7 |         0 |        0 |     7 |
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet balance

```

**Result:**

> ```text
> +--------------------------+-----------------+
> | Asset                    | Balance         |
> +============================================+
> | BTC On-chain (total)     | 99,988,121 sats |
> |--------------------------+-----------------|
> | BTC On-chain (spendable) | 99,967,074 sats |
> |--------------------------+-----------------|
> | BTC Anchor reserve       |          0 sats |
> |--------------------------+-----------------|
> | BTC Lightning (total)    |     21,047 sats |
> +--------------------------+-----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE | contract:YTO3fn_1-Z_B6nKj-ZbVp6MB-DkT5H4R-ak0sUfv-GxfVGkE |    83 |         0 |        0 |    83 |
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
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
>   "user_channel_id": "05863a3a1273d0a768201a587574d03b"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q044mynz56qmlhn6t4j3shktrg90khe4cts7m7k

```

**Result:**

> ```text
> [
>   "4223c23e8f9d418128f0d543fdad986be47f4ac9e81756145b2be0113f8ff096",
>   "59a42c58f0c097bf49822d4c8ee1f0dc71a7c830511f5a9a138d1b4082190fb7",
>   "3482af029f656badffb366eae0fb56352810c191d6e3a6785f0b9fb6b302a6df",
>   "35c4add0f5c5621fee2a6983df0512975b2a4cdb757e1332bdeed634e4217ad1",
>   "256e74b6b66ec49c0cf927d075241ea4cdf9a57e164b5aa8b68078b7cf7e58df",
>   "2ef8ae83feff57720e6f892045dc3eeeb6a2efe10b40c0f48edb3c18cee9192f"
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
