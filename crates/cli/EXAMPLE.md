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
esplora=http://127.0.0.1:64456

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
> | api_crate_version  | 0.7.0-bitlight.2 |
> |--------------------+------------------|
> | api_version        | v1               |
> |--------------------+------------------|
> | core_crate_version | 0.7.0-bitlight.2 |
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
>   [OK] Best Block Height: Height: 101
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
> | best_block_height | 101   |
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
>   "node_id": "0295d9bf062f5683a0853c09475c95ffffd170cd21ccfe61e8cdb28a7869a4f50c"
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
>   [OK] Best Block Height: Height: 101
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty node id

```

**Result:**

> ```text
> {
>   "node_id": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5"
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
> {"keystore_path":"/tmp/rgbldk-cli-example-7zbj7kj4/keystore-init-demo/keystore","mnemonic":"high fee whip fruit bunker develop soon fence early extra mix job tooth great champion advice love eager lion second unlock night inside cinnamon","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-7zbj7kj4/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-7zbj7kj4/keystore-migrate-demo/keys_seed.bak.20260717-085145","ok":true}
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
> | 0288e7d9...bd4827a5 | 127.0.0.1:9736 | true      | true      |
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
>   "address": "bcrt1qjekcy4x6d9p9kx99p275f2wwlgqrp27864f60v"
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
>   "address": "bcrt1qsfq9rlvshh2wkys7x7c8jzare65z30lvsx4dg6"
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
> bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qjekcy4x6d9p9kx99p275f2wwlgqrp27864f60v 1

```

**Result:**

> ```text
> 0edf35baf1015f90e7c7e1647fe2c959ed1dab297a301ffdd150d95ac5c1c65a
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qsfq9rlvshh2wkys7x7c8jzare65z30lvsx4dg6 1

```

**Result:**

> ```text
> 9722c7495b74f5b87ae46c13048afad428b3c5c6450cabd14e6b6957512ac040
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "3ae365dce9220eaddc6330518a8a0edbd5e54460cc6dcad1492cf60f6fc2e443",
>   "35366c786b2d4a4dce5e886233fcb8fcbac19f1272083c6425d99572a9bdc0ff",
>   "2b7bf4f3c4a86a5ad7a2122c9b9a0e4d0c904e67aa3de86337c8003666cde2af",
>   "5ed6ef2aa6f222a3e05fdc8a421dfa111f8e7db7bbed8f09d76ea3c93e074b8d",
>   "5f9c3d955f131b903f36edf83354d8c8b41587054cc3cc7040fa9dc82ba2ca46",
>   "3abdd3812aba2b813153a6a1ba1ad9382ae2548e2dc799308da674c8041b9123"
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
> | 0edf35baf1015f90e7c7e1647fe2c959ed1dab297a301ffdd150d95ac5c1c65a:1 |  100,000,000 | Confirmed |    102 | Available    | -        |            - |
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
>   "user_channel_id": "40b916fd38a2297d99bdfabb8faebdbd"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "0f13d35dbc98e64b32274bd2b855168da1f2541d0c2ce5757385a27ae9fb0afc",
>   "1f211757a07d07870dd470da5c518c15d0aa59fa2ce0fc9e8c81ce87ecbea794",
>   "64d02a4d98fa992a7cdf005c82c94743c442977ae2420161f915f459f1d2c5a1",
>   "17792fa0e31f4acd1f7ea6831008d620b62b304ac19ee291c355104226035d66",
>   "264b9c48654326acfd16fe2574ebe0e1bc03618cc229cdec0751e20ef851bcbb",
>   "7f4af716ffd93a632e4ae7a01918ee50d88fff5bd5170a150b5061ae18903dda"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable |
> +================================================================================================================================+
> | 40b916fd...8faebdbd | 0295d9bf...69a4f50c | 118747255865344 | 1099657707521 | 1099555602434 | 120,000         | true  | true   |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
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
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable |
> +================================================================================================================================+
> | 0bd4d448...9e8e5ecc | 0288e7d9...bd4827a5 | 118747255865344 | 1099555602434 | 1099657707521 | 120,000         | true  | true   |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
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
> | 0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5 |
> |--------------------------------------------------------------------|
> | 0295d9bf062f5683a0853c09475c95ffffd170cd21ccfe61e8cdb28a7869a4f50c |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 0288e7d9...bd4827a5.
> Known channels: 1.
> +-----------------+
> | SCID            |
> +=================+
> | 118747255865344 |
> +-----------------+
>
> ```

**Run:**

```bash
$ rgbldk graph channels

```

**Result:**

> ```text
> +-----------------+
> | SCID            |
> +=================+
> | 118747255865344 |
> +-----------------+
>
> ```

**Run:**

```bash
$ rgbldk graph channel <scid>

```

**Result:**

> ```text
> Channel 118747255865344 connects 0288e7d9...bd4827a5 and 0295d9bf...69a4f50c.
> Capacity: unknown.
>
> 0288e7d9...bd4827a5 -> 0295d9bf...69a4f50c: no gossip update available.
>
> 0295d9bf...69a4f50c -> 0288e7d9...bd4827a5: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1784278339       |
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
>   "address": "bcrt1qrmns3kyeglup3l6t5upltq3qplrgvtnl2n0mpj"
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
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable |
> +================================================================================================================================+
> | 0bd4d448...9e8e5ecc | 0288e7d9...bd4827a5 | 118747255865344 | 1099555602434 | 1099657707521 | 120,000         | true  | true   |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
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
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable |
> +================================================================================================================================+
> | 40b916fd...8faebdbd | 0295d9bf...69a4f50c | 118747255865344 | 1099657707521 | 1099555602434 | 120,000         | true  | true   |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "2f89bf7547a2cbf2e40339092cd9e300a77c7f61e2c5fe84a8a742e2130b25a0",
>   "06057c99f59269e4c6a87b809fa5217e5eeddb282fc79eb8c1f47a9309a9fe95",
>   "153d259cb4a0b8c427f22c854d9ee6665708a2263bbc57ef1095c181edc1a576",
>   "53763b64f143c0332cee3817fcd0980c0b0db456c73792d4c4b74cce87d9097b",
>   "0f5101c8ae2e119aafe348158a489a41d1967e0f5a56d75b82ed73d849479d11",
>   "226c3e491fce961dc40748c0fe6a272c531537363a97b4aac7104e154fa7483c"
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
> BTC balance change: on-chain total 0 sats, spendable 0 sats, anchor reserve 0 sats, lightning -30,000 sats.
>
> ```

**Run:**

```bash
$ rgbldk wallet balance

```

**Result:**

> ```text
> +--------------------------+------------+
> | Asset                    | Balance    |
> +=======================================+
> | BTC On-chain (total)     | 1.0003 BTC |
> |--------------------------+------------|
> | BTC On-chain (spendable) | 1.0003 BTC |
> |--------------------------+------------|
> | BTC Anchor reserve       |     0 sats |
> |--------------------------+------------|
> | BTC Lightning (total)    |     0 sats |
> +--------------------------+------------+
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
> BTC balance change: on-chain total 0 sats, spendable 0 sats, anchor reserve 0 sats, lightning -87,654 sats.
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
>   "address": "bcrt1qg0nhwwyvxcw9kulr22yulh4vkfktlk7ha5ssy2"
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
> rgb(wpkh([23598573/86h/827167h/0h]tpubDD6AH5MbMxv3nuezMwW6pxHB69NhJ45SgyUK9YJPkMJoLzgWiHmmnwKWyGTqvA3k8F6nvyLWsao8StCdVhqyiGx7zm5X3ths6W1vJmB4aPK/<0;1>/*),seals(),noise(ff2ee36fb0381eeaf6428ff39c0a80bff68cc200e418a596d98018f68b55d357))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | 23598573    | m/86'/827167'/0' | tpubDD6AH5MbMxv3nuezMwW6pxHB69NhJ45SgyUK9YJPkMJoLzgWiHmmnwKWyGTqvA3k8F6nvyLWsao8StCdVhqyiGx7zm5X3ths6W1vJmB4aPK | wpkh([23598573/86'/827167'/0']tpubDD6AH5MbMxv3nuezMwW6pxHB69NhJ45SgyUK9YJPkMJoLzgWiHmmnwKWyGTqvA3k8F6nvyLWsao8StCdVhqyiGx7zm5X3ths6W1vJmB4aPK/<0;1>/*)#pejl4d6k |
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
> | Public key      | 03a343af63f2fed77bf43d026560e3cbd0b78b9f908d85a2070eea6e5223934bb9 |
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
> 3045022100be17586ae5d315af540c3bc10feaba7667aded87fcd9552afe45fc7ef12a9c3102205e7c3150d1e28a8cf2775dd4dd06343db5a588cb220dbd4e6df4e023e5fddb7a
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +--------+
> | Issuer |
> +========+
> +--------+
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
>   "address": "bcrt1q8mt3jwtz0amemdgnzlnc3s5dltlfw3tlr6svz4"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> 1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> 4fdf37b087b448b8133414010132f95c034e0ec74c9783503623484965e9e382
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "2f9e8ab5463cd66bd0bce7707e9e2f23bbbe042004e7e8e229771e1f2ac334d8"
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
> +--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------+
> | Outpoint                                                           | Value (sats) | Status    | Height | Availability | Allocations | Roles                      |
> +==================================================================================================================================================================+
> | 1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f:1 |   10,000,000 | Confirmed |    120 | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------|
> | 4fdf37b087b448b8133414010132f95c034e0ec74c9783503623484965e9e382:0 |      110,000 | Confirmed |    120 | Available    | -           | FeeSupport, BlindingTarget |
> +--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------+
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
> | 1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f:1 |   10,000,000 |    120 | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | 4fdf37b087b448b8133414010132f95c034e0ec74c9783503623484965e9e382:0 |      110,000 |    120 | Available |        |
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
>   "reservation_id": "manual-reservation-1784278478-xEm1izuq9AIWcvNrFLaY6qmdctrQsin8",
>   "outpoint": "1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f:1",
>   "reserved_until_unix_secs": "1784278538"
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
>   "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
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
>       "detail": "1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f:1"
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
> | DemoAsset | DEMO   |         0 |    100 | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
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
$ rgbldk rgb contracts export --contract-id contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo --out /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.
> Consignment key: contract_export_99dd0f072689ad5dd0ea0f79f7cbc22d6bb73e7e7f9581f7d68bda27b3c5e6e0
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f:1
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
>         "detail": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_49869e787b0140dc7a7bd73c49ebfca26fe467f6faa3de6677787c42a5f0dd3d",
>     "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>     "ok": true
>   },
>   "out": "/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_49869e787b0140dc7a7bd73c49ebfca26fe467f6faa3de6677787c42a5f0dd3d.
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.download.zip.
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
>   "address": "bcrt1qy0tjgqmydj5uzj5zqp4yhzpvcj72xk2p7kkrec"
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
> 885fd64d88d44726c40d19a7d95983b7f22fe6a68f88bad900c6e4bd2baca7c4
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "40f67508d12b4e39e6c8ba3b100c7e921b9539fb593fdcfbc162d27efd95aff2"
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
$ rgbldk rgb contracts import --contract-id contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo --file /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_32e00874966f143d5232804d347b50542c40e1e9664bef246dcd49aae37da0f2
> Imported contract contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.
> Consignment key: contract_import_32e00874966f143d5232804d347b50542c40e1e9664bef246dcd49aae37da0f2
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
> | DemoAsset | DEMO   |         0 |    100 | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
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
> The node knows contract contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.
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
>   "address": "bcrt1qw8pa9kkngutgpxcuhjne548ehhedm7nqgw0erp"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> c36fe7aeb2750dccbfe8b027c2eced9e291808c4d3ded94227948fade9aecd00
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "622b053bd3e33cbf909e9a4a7f01552c7d4b93a6067f4f45a785aa97df73407d"
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
>   "address": "bcrt1qevtjgvsq0mec5qzm8ddtx75zanakcw4welder0"
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
>   "address": "bcrt1qnn9zqjhtwrfx9se7ndssfleuvgwncqkkkxqahs"
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
>   "address": "bcrt1q8jpskxguj3vvvhm7hxd4c9ce6r9wch2p25x0xz"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"b9b895da3a67591e06ac91e1b416c3f18cee18b2ce55f83ef37ce74086eb2e05","status":"broadcast","outputs":[{"address":"bcrt1qevtjgvsq0mec5qzm8ddtx75zanakcw4welder0","value_sats":"30000","vout":0},{"address":"bcrt1qnn9zqjhtwrfx9se7ndssfleuvgwncqkkkxqahs","value_sats":"28000","vout":2}],"change":{"address":"bcrt1q8jpskxguj3vvvhm7hxd4c9ce6r9wch2p25x0xz","value_sats":"941828","vout":1},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "3486e14f71ac97f74dd86b78b41300219812cf350363346a1a9888ddef36898d"
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
>   "address": "bcrt1qf5mym2yfaykwssnc39mhhvrdeelawd4hzmnz6m"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> bdf41c2ee8b834ea03ccc8b33daa383152b7e305e6d4dd494480beefa19076b5
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "38c133674b333480cfd6997243d916bde2201e8de489482b3efa325adafaa27b"
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
> BTC balance change: on-chain total +500,000 sats, spendable +941,828 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qlpqs75a506t7v65m57xytr5z9v22w83pkym95u"
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
>   "address": "bcrt1qng5vsf4e3nwz5hcdftnnskdwh743jjpyma265s"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO 1a68befeaa8b62fff2314cfbe3becc334f1b547eededdfe74e61ac78172fb40f:1.
> Broadcast transaction: 1831da67761acf7e3eb2feee66adefb84a465cce707fe4570fd5a6348ad32fba
> Replacement output: bcrt1qlpqs75a506t7v65m57xytr5z9v22w83pkym95u with 10,040,000 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo_1831da67761acf7e3eb2feee66adefb84a465cce707fe4570fd5a6348ad32fba
> Change output: bcrt1qng5vsf4e3nwz5hcdftnnskdwh743jjpyma265s with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "6abce1fb24ddb0c34631708f9cc65b6e47fd3969da4899d63e802ca12914712d"
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
> BTC balance change: on-chain total 0 sats, spendable +459,746 sats, anchor reserve 0 sats, lightning 0 sats.
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
>   "address": "bcrt1qfsxfymsy4npw6hq47g3vjl0mfu8ydzuvc63rn4"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO b9b895da3a67591e06ac91e1b416c3f18cee18b2ce55f83ef37ce74086eb2e05:2.
> Broadcast transaction: 12f224e39bea54260382edd1ade4d5257e746eaec9521d4639e61d44f3574545
> Sent 27,889 sats to bcrt1qfsxfymsy4npw6hq47g3vjl0mfu8ydzuvc63rn4.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "6d4d8fbed5b93e01ace892a19a29bd8276be3758efda6883b2547d3deeeb4fcb"
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
> BTC balance change: on-chain total +27,889 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
>   "invoice": "contract:tb@ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo/90@at:AfYKutFM-nHg3FH6g-kOhbI2m6-2AdL0hdH-siO9X40N-VCIiCg/?expiry=2026-07-17T09:55:50.903599+00:00",
>   "blinding_utxo_used": "885fd64d88d44726c40d19a7d95983b7f22fe6a68f88bad900c6e4bd2baca7c4:0"
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
> | Contract         | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:AfYKutFM-nHg3FH6g-kOhbI2m6-2AdL0hdH-siO9X40N-VCIiCg    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1784282150                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo/90@at:AfYKutFM-nHg3FH6g-kOhbI2m6-2AdL0hdH-siO9X40N-VCIiCg/?expiry=2026-07-17T09:55:50.903599+00:00
> scope=contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:AfYKutFM-nHg3FH6g-kOhbI2m6-2AdL0hdH-siO9X40N-VCIiCg
> data=90
> expiry=2026-07-17T09:55:50.903599+00:00
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
>   "txid": "ff8a598873f84f469a3ac1a696460762035600fa41124f6628e34108380f2693",
>   "consignment_key": "rgb_consignment_contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo_ff8a598873f84f469a3ac1a696460762035600fa41124f6628e34108380f2693"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo_ff8a598873f84f469a3ac1a696460762035600fa41124f6628e34108380f2693.
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
> Accepted contract contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "25aa8f5ef1f0a966e0360602739af9b5b54e706755ddf2e35f5a72ca20802b22",
>   "130d1970afb6ef3de0e37ca065b9768733beccc13a48666f3080196bd6c52d3f",
>   "26ab8acb73e650145819b211d8fed8597731a7c4b2a5700319d017dd39f54976",
>   "09292d251be5287f58c2be7eb57b322d93b596a68e26ecb3c028bcba78eba60a",
>   "0b7fa4502a52ceed66d3009908a113b4fd185699be5826762885a97e789feb3d",
>   "608b4e817c3778ee2774ac8109f14de87789701ab3c1b16876327e7fcd0972c9"
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
$ rgbldk rgb contracts balance contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
> |-----------+-----------------------------------------------------------|
> | Mined     | 90                                                        |
> |-----------+-----------------------------------------------------------|
> | Tentative | 0                                                         |
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
> +------------------------------------------------------------------+-----------+-----------------------------------------------------------+--------+------------------------------------------------------------------+----------------------------------------------------------------------------------+
> | Payment ID                                                       | Status    | Contract                                                  | Amount | Txid                                                             | Consignment                                                                      |
> +=========================================================================================================================================================================================================================================================================================================+
> | ca7e9a9df6d2bd866520d33bef13e247710b559308a536d68769a013f7aad16f | succeeded | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |     90 | ff8a598873f84f469a3ac1a696460762035600fa41124f6628e34108380f2693 | transfer_import_f39393926677ecde0aa76c908d0c6305a380256008de5f3cc0e4da2ace522c89 |
> +------------------------------------------------------------------+-----------+-----------------------------------------------------------+--------+------------------------------------------------------------------+----------------------------------------------------------------------------------+
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
$ rgbldk rgb contracts balance contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
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
>   "user_channel_id": "9952853075557233da3c29cc462de631"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "0c3d874c9be06066e591504730402e5f098719de91a904e95e871abb9681152b",
>   "4848f9e0ef6087eb62195042197eb6db39a856fc75e999b044feac40e4a3bd71",
>   "03b5f995138e3beb075f556f23fdf6ccc2aaf03710076ea01f1b83cd60c7f498",
>   "5ce4fe78149467ac5e59c448e44cf9c78e85d0d2a83f47dadca5b6512cbd33a5",
>   "13ed80c05b44d17648b59a920b5756c03d3a3c199ce0d5dd1083ca58e3dd0ffb",
>   "2cf97040d99da839645ab4aec65153b43eddd058eed6a77a923e4a09c08a20f8"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+------+-----------+----------+-----------------+-------+--------+---------------------+-----------+------------+
> | User Channel ID     | Counterparty        | SCID | Out Alias | In Alias | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote |
> +===========================================================================================================================================================+
> | 99528530...462de631 | 0288e7d9...bd4827a5 | -    | -         | -        | 100,000         | false | false  | contract...-EZ1zAOo | 10        | 0          |
> +---------------------+---------------------+------+-----------+----------+-----------------+-------+--------+---------------------+-----------+------------+
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
>   "invoice": "contract:tb@ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo/7@at:sSqRm8Nj-iJkOZS7o-8t5Zb1pa-dB77wVeo-ilG1~sto-qzs7KQ/?expiry=2026-07-17T09:56:32.140552+00:00",
>   "blinding_utxo_used": "2bfeee79ee88e8a0f8ae6d6baf117f7fffe342cbba5c689412da9a1f8bc8cfc6:2"
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
>   "txid": "3599cdd1516829d59367936f51456b271cdc9fe378afeffa601538a97f16030b",
>   "consignment_key": "rgb_consignment_contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo_3599cdd1516829d59367936f51456b271cdc9fe378afeffa601538a97f16030b"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo_3599cdd1516829d59367936f51456b271cdc9fe378afeffa601538a97f16030b.
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
> Accepted contract contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "22a7c6a69fb34176e0bdfbe9947319958339e3d4bf7f8dbce9af8d09f7fd24d0",
>   "11801b20e6873c35013d18e1c52e72e90525245d42d75fc728fc76f0a37ecf80",
>   "14cce7f1225a263597470de1cebe56cc251b53b9f4799f92ebf1fb9c843cf648",
>   "0c5022565af058782985b33d4c7392588df17e9317357558083d267b5dcf084e",
>   "0f13ecb6f0605dafa9fa8654eeeb21f7bafdcb3d46dd72bfaee9e6bcfdba5ea1",
>   "13fe182f4af0752103b8a50e8908718e3ad864549a6590e453f214b30ccc6972"
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
$ rgbldk rgb contracts balance contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |
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

## 8) RGB/BTC swaps (single-hop lifecycle + multi-hop offer)

Create a single-hop swap offer on node-a, preview and accept it on node-b, wait for the maker to observe the acceptance, execute the circular payment, and poll both nodes until the swap is actually `Settled`. The two-node example also creates and cancels an unexecuted offer, and covers multi-hop offer encoding/cancellation. A real multi-hop settlement needs an intermediary node and is intentionally outside this two-node environment.


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
$ rgbldk --color never --output json --pretty swap create --counterparty-node-id <node_id_b> --channel-scid <channel_scid> --contract-id <contract_id> --asset-amount 2 --btc-amount-msat 10000000 --btc-carrier-amount-msat 5000000 --maker-gives-rgb --expiry-secs 3600

```

**Result:**

> ```text
> {
>   "swap_string": "rgb-swap:v1:eyJwYXltZW50X2hhc2hfaGV4IjoiOTJlZDIwZGRkOWVhYTZhMGU5MDE1NDY4YTJkMDViZjA5Y2UyNGJjZWNiNDYzN2M3NjBhZmE5N2Q1NGMyZTc1NiIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAyODhlN2Q5NzRjZWRjOGZhMWE3MmNlOGIzOTM5YWVjM2ZlNjU0YTZiMDkzZjEwZWU4ZTE1NWEwZmNiZDQ4MjdhNSIsImNoYW5uZWxfc2NpZCI6MTQ2MjM1MDQ2NTU5NzQ1LCJjb250cmFjdF9pZCI6ImNvbnRyYWN0OnR0SmNjc2pILVdGZWhpaU4tNHUwfkt1Ui1QeDlKMzFtLVg3R0ZSWHEtRVoxekFPbyIsImFzc2V0X2Ftb3VudCI6MiwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwMCwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOjUwMDAwMDAsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiZXhwaXJ5X3NlY3MiOjM2MDAsImNyZWF0ZWRfYXRfdW5peF9zZWNzIjoxNzg0Mjc4NjA5fQ",
>   "payment_hash": "92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756",
>   "info": {
>     "payment_hash": "92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5",
>     "channel_scid": "146235046559745",
>     "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>     "asset_amount": "2",
>     "btc_amount_msat": "10000000",
>     "btc_carrier_amount_msat": "5000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1784278609",
>     "is_multihop": false,
>     "last_error": null
>   }
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
$ rgbldk --color never --output json --pretty swap decode --swap-string <swap_string>

```

**Result:**

> ```text
> {
>   "payment_hash": "92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756",
>   "role": "Taker",
>   "status": "Offered",
>   "counterparty_node_id": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5",
>   "channel_scid": "146235046559745",
>   "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1784278609",
>   "is_multihop": false,
>   "last_error": null
> }
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty swap accept --swap-string <swap_string>

```

**Result:**

> ```text
> {
>   "payment_hash": "92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756",
>   "role": "Taker",
>   "status": "Accepted",
>   "counterparty_node_id": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5",
>   "channel_scid": "146235046559745",
>   "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1784278609",
>   "is_multihop": false,
>   "last_error": null
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
$ rgbldk swap get 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Accepted                                                         |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 0288e7d9...bd4827a5                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 146235046559745                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-EZ1zAOo                                              |
> |---------------------+------------------------------------------------------------------|
> | Asset amount        | 2                                                                |
> |---------------------+------------------------------------------------------------------|
> | BTC amount (msat)   | 10,000,000                                                       |
> |---------------------+------------------------------------------------------------------|
> | BTC carrier (msat)  | 5,000,000                                                        |
> |---------------------+------------------------------------------------------------------|
> | Maker gives         | RGB                                                              |
> |---------------------+------------------------------------------------------------------|
> | Expiry (secs)       | 3600                                                             |
> |---------------------+------------------------------------------------------------------|
> | Created (unix secs) | 1784278609                                                       |
> |---------------------+------------------------------------------------------------------|
> | Multihop            | false                                                            |
> |---------------------+------------------------------------------------------------------|
> | Last error          | -                                                                |
> +---------------------+------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty swap execute --payment-hash <payment_hash>

```

**Result:**

> ```text
> {
>   "ok": true,
>   "payment_hash": "92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756",
>   "status": "InFlight"
> }
>
> ```

**Run:**

```bash
$ rgbldk swap get 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 0288e7d9...bd4827a5                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 146235046559745                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-EZ1zAOo                                              |
> |---------------------+------------------------------------------------------------------|
> | Asset amount        | 2                                                                |
> |---------------------+------------------------------------------------------------------|
> | BTC amount (msat)   | 10,000,000                                                       |
> |---------------------+------------------------------------------------------------------|
> | BTC carrier (msat)  | 5,000,000                                                        |
> |---------------------+------------------------------------------------------------------|
> | Maker gives         | RGB                                                              |
> |---------------------+------------------------------------------------------------------|
> | Expiry (secs)       | 3600                                                             |
> |---------------------+------------------------------------------------------------------|
> | Created (unix secs) | 1784278609                                                       |
> |---------------------+------------------------------------------------------------------|
> | Multihop            | false                                                            |
> |---------------------+------------------------------------------------------------------|
> | Last error          | -                                                                |
> +---------------------+------------------------------------------------------------------+
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
$ rgbldk swap get 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Taker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 0288e7d9...bd4827a5                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 146235046559745                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-EZ1zAOo                                              |
> |---------------------+------------------------------------------------------------------|
> | Asset amount        | 2                                                                |
> |---------------------+------------------------------------------------------------------|
> | BTC amount (msat)   | 10,000,000                                                       |
> |---------------------+------------------------------------------------------------------|
> | BTC carrier (msat)  | 5,000,000                                                        |
> |---------------------+------------------------------------------------------------------|
> | Maker gives         | RGB                                                              |
> |---------------------+------------------------------------------------------------------|
> | Expiry (secs)       | 3600                                                             |
> |---------------------+------------------------------------------------------------------|
> | Created (unix secs) | 1784278609                                                       |
> |---------------------+------------------------------------------------------------------|
> | Multihop            | false                                                            |
> |---------------------+------------------------------------------------------------------|
> | Last error          | -                                                                |
> +---------------------+------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk swap ls

```

**Result:**

> ```text
> +------------------------------------------------------------------+-------+---------+-----------------------------------------------------------+-------+-------------+
> | Payment Hash                                                     | Role  | Status  | Contract                                                  | Asset | Maker Gives |
> +======================================================================================================================================================================+
> | 92ed20ddd9eaa6a0e9015468a2d05bf09ce24bcecb4637c760afa97d54c2e756 | Taker | Settled | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo | 2     | RGB         |
> +------------------------------------------------------------------+-------+---------+-----------------------------------------------------------+-------+-------------+
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
$ rgbldk --color never --output json --pretty swap create --counterparty-node-id <node_id_b> --channel-scid <channel_scid> --contract-id <contract_id> --asset-amount 1 --btc-amount-msat 1000000 --btc-carrier-amount-msat 1000000 --maker-gives-rgb --expiry-secs 3600

```

**Result:**

> ```text
> {
>   "swap_string": "rgb-swap:v1:eyJwYXltZW50X2hhc2hfaGV4IjoiYjQyMDljMjZlNzE2MzlmYTcwOTk5YWFmYWNiMTQ2ZWNjOTk5NzU0MDE3OWUyMTRjY2NmODc0M2ZlZjBlYzk2MiIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAyODhlN2Q5NzRjZWRjOGZhMWE3MmNlOGIzOTM5YWVjM2ZlNjU0YTZiMDkzZjEwZWU4ZTE1NWEwZmNiZDQ4MjdhNSIsImNoYW5uZWxfc2NpZCI6MTQ2MjM1MDQ2NTU5NzQ1LCJjb250cmFjdF9pZCI6ImNvbnRyYWN0OnR0SmNjc2pILVdGZWhpaU4tNHUwfkt1Ui1QeDlKMzFtLVg3R0ZSWHEtRVoxekFPbyIsImFzc2V0X2Ftb3VudCI6MSwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJidGNfY2Fycmllcl9hbW91bnRfbXNhdCI6MTAwMDAwMCwibWFrZXJfZ2l2ZXNfcmdiIjp0cnVlLCJleHBpcnlfc2VjcyI6MzYwMCwiY3JlYXRlZF9hdF91bml4X3NlY3MiOjE3ODQyNzg2MjN9",
>   "payment_hash": "b4209c26e71639fa70999aafacb146ecc9997540179e214cccf8743fef0ec962",
>   "info": {
>     "payment_hash": "b4209c26e71639fa70999aafacb146ecc9997540179e214cccf8743fef0ec962",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5",
>     "channel_scid": "146235046559745",
>     "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1784278623",
>     "is_multihop": false,
>     "last_error": null
>   }
> }
>
> ```

**Run:**

```bash
$ rgbldk swap cancel <payment_hash>

```

**Result:**

> ```text
> Swap cancelled.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty swap create-multihop --rgb-hop <node_id_b>:<channel_scid> --btc-hop <node_id_a>:<channel_scid> --contract-id <contract_id> --asset-amount 1 --btc-amount-msat 1000000 --btc-carrier-amount-msat 1000000 --maker-gives-rgb --expiry-secs 3600

```

**Result:**

> ```text
> {
>   "swap_string": "rgb-swap:v2:eyJwYXltZW50X2hhc2hfaGV4IjoiYmYxZTY5NmQ5ZWE5NDg3NGE3NGU2YzI4NGM4NGZhZDMxNjRiMzY5YTExMGY4ZDY2M2Y5YjUzNmE3YTVmNTQ5OCIsInJnYl9wYXRoIjpbeyJub2RlX2lkX2hleCI6IjAyODhlN2Q5NzRjZWRjOGZhMWE3MmNlOGIzOTM5YWVjM2ZlNjU0YTZiMDkzZjEwZWU4ZTE1NWEwZmNiZDQ4MjdhNSIsImNoYW5uZWxfc2NpZCI6MTQ2MjM1MDQ2NTU5NzQ1fV0sImJ0Y19wYXRoIjpbeyJub2RlX2lkX2hleCI6IjAyOTVkOWJmMDYyZjU2ODNhMDg1M2MwOTQ3NWM5NWZmZmZkMTcwY2QyMWNjZmU2MWU4Y2RiMjhhNzg2OWE0ZjUwYyIsImNoYW5uZWxfc2NpZCI6MTQ2MjM1MDQ2NTU5NzQ1fV0sImNvbnRyYWN0X2lkIjoiY29udHJhY3Q6dHRKY2NzakgtV0ZlaGlpTi00dTB-S3VSLVB4OUozMW0tWDdHRlJYcS1FWjF6QU9vIiwiYXNzZXRfYW1vdW50IjoxLCJidGNfYW1vdW50X21zYXQiOjEwMDAwMDAsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImV4cGlyeV9zZWNzIjozNjAwLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6MTc4NDI3ODYyNX0",
>   "payment_hash": "bf1e696d9ea94874a74e6c284c84fad3164b369a110f8d663f9b536a7a5f5498",
>   "info": {
>     "payment_hash": "bf1e696d9ea94874a74e6c284c84fad3164b369a110f8d663f9b536a7a5f5498",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5",
>     "channel_scid": "146235046559745",
>     "contract_id": "contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1784278625",
>     "is_multihop": true,
>     "last_error": null
>   }
> }
>
> ```

**Run:**

```bash
$ rgbldk swap cancel <multihop_payment_hash>

```

**Result:**

> ```text
> Swap cancelled.
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

## 9) Payments (BTC Lightning L2 + keysend + BOLT12 + RGB Lightning)

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
>   "invoice": "lnbcrt100n1p49nmnydq8v3jk6mcnp4q2yw0kt5emwglgd89n5t8yu6asl7v49xkzflzrhgu926pl9afqn62pp5p3084zpspuegxehh9ls7vmhr8wa6wg9qajk93g8jnj0nr4fnwj9ssp5p0kk8cx7acv9a935mfm5dhp2yxjw6wnqpeuuzxv322rfkhua63fs9qyysgqcqzp2xqrrssrzjq22an0cx9atg8gy98sy5why4lllazuxdy8x0uc0gekeg57rf5n6scqqqqyqq3vcqqsqqqqlgqqqqqqqqfq2c3czw0v66he80xfjet77aj5x2x7vy40ysn87x0aup92gw060xp93ueqj6ahnmgtu9uuvy7zxeex9lvu4zws329t5seqt44tdn3vhwsqhw7gvc"
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
> | payment_hash | 0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5 |
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
>   "payment_id": "0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b",
>   "preimage": "14db95da2095c74b9dfae09f10363cf3b0ff6a7b9958906cfd9489248bc44509",
>   "amount_sats": "10",
>   "destination": "0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b
>
> ```

**Run:**

```bash
$ rgbldk pay get 0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"0c5e7a88300f328366f72fe1e66ee33bbba720a0ecac58a0f29c9f31d533748b","preimage":"14db95da2095c74b9dfae09f10363cf3b0ff6a7b9958906cfd9489248bc44509","secret":"0bed63e0deee185e9634da7746dc2a21a4ed3a600e79c1199152869b5f9dd453"} |
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
>   "invoice": "lnbcrt130n1p49nmnwdq0v3jk6medwdjkueqnp4q2yw0kt5emwglgd89n5t8yu6asl7v49xkzflzrhgu926pl9afqn62pp5usntpqe4hauwwmlaknkxxcx3daq8yt3f02s4nkdkftplsq7a968ssp5862qutpctqzh68kcz0q5yn2py5zdj7lh4qxjrw8zktzsl2uvx7us9qyysgqcqzp2xqrrssrzjq22an0cx9atg8gy98sy5why4lllazuxdy8x0uc0gekeg57rf5n6scqqqqyqq3vcqqsqqqqlgqqqqqqqqfq26c2lusmakwmntaexmf4g6ejhfz6qez92jacf9mqhwha7pvgkunka8scn968h4vp62xengasaq94j8y9jx64fkmw6zgr79t4deeq36cqhvxujv"
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
>   "payment_id": "e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f
>
> ```

**Run:**

```bash
$ rgbldk pay get e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"e426b08335bf78e76ffdb4ec6360d16f40722e297aa159d9b64ac3f803dd2e8f","preimage":"7e85d3c1cc814aaecdee44fca26ad3dcf2a146c5818b8da535d9693ef6253a21","secret":"3e940e2c3858057d1ed813c1424d412504d97bf7a80d21b8e2b2c50fab8c37b9"} |
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
>   "invoice": "lnbcrt1p49nmnndqdv3jk6medweshynp4q2yw0kt5emwglgd89n5t8yu6asl7v49xkzflzrhgu926pl9afqn62pp5an99n52ytcwme778w8yc07m239hs5uaeg4r2e7zhftnjur720a2qsp59sndls6px0endhg3ydqa7qhur442ew3h0etlmdnxj5v3hh4k249s9qyysgqcqzp2xqrrssrzjq22an0cx9atg8gy98sy5why4lllazuxdy8x0uc0gekeg57rf5n6scqqqqyqq3vcqqsqqqqlgqqqqqqqqfq5435gn9gzy3v46uplxwzqkg4ssk3mwag04m8wlr3za9sh5ygzvhquy6pwefwep96kyjywx96v35sxylkg8s364jqut2wf28u5tszf3sq8fr470"
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
>   "payment_id": "ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54
>
> ```

**Run:**

```bash
$ rgbldk pay get ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"ecca59d1445e1dbcfbc771c987fb6a896f0a73b94546acf8574ae72e0fca7f54","preimage":"f36ec4d44dd868e76c278a252f173794d64c35d5b0e04161bd08cbe42f5b157c","secret":"2c26dfc34133f336dd112341df02fc1d6aacba377e57fdb66695191bdeb6554b"} |
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
>   "invoice": "lnbcrt140n1p49nmnedqcv3jk6meddphkcepdvdkxz6tdnp4q2yw0kt5emwglgd89n5t8yu6asl7v49xkzflzrhgu926pl9afqn62pp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp5dph8x5gehh89ql82dh2xdv942hl502c92qxwehljz06tq98kfnqs9qyysgqcqzp2xqrrssrzjq22an0cx9atg8gy98sy5why4lllazuxdy8x0uc0gekeg57rf5n6scqqqqyqq3vcqqsqqqqlgqqqqqqqqfq9z23n4e5u6dxxrvh8f633y26ps348ynx77lm89r2e5ksrk7r9qpnr88j8jtrhjee6yg9vqzny9y43v4kvflty26x5w6cmr7cg7uryksqwynqd7"
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
> | destination  | 0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5 |
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
> | payment_hash    | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":"4242424242424242424242424242424242424242424242424242424242424242","secret":"686e735119bdce507cea6dd466b0b555ff47ab05500cecdff213f4b014f64cc1"} |
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
>   "invoice": "lnbcrt150n1p49nm5rdqhv3jk6meddphkcepdveskjmqnp4q2yw0kt5emwglgd89n5t8yu6asl7v49xkzflzrhgu926pl9afqn62pp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp5n3s86paum6kxlj99jt77gxscd66tpycg458ja2z3mdlflvl6qepq9qyysgqcqzp2xqrrssrzjq22an0cx9atg8gy98sy5why4lllazuxdy8x0uc0gekeg57rf5n6scqqqqyqq3vcqqsqqqqlgqqqqqqqqfqpprd7hv3ly04g0mrnxn99vgt2v94srha7vsflth8g0t0uxrr7kcr7qef4l5e095tdhh3g5s559usg2yjgq4tuxuvqfpqxssww88fw3cq8fm7v4"
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
> | payment_hash    | 4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24                                                                                                                |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                        |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                                        |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                          |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                           |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"9c607d07bcdeac6fc8a592fde41a186eb4b09308ad0f2ea851db7e9fb3fa0642"} |
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
>   "invoice": "lnbcrt120n1p49nm5ddq0v3jk6medvfskx6cnp4q22an0cx9atg8gy98sy5why4lllazuxdy8x0uc0gekeg57rf5n6scpp5d4d7af0kcdzdd4z9caj9cz2lu520zywkgc96c7vfptyha3jr6ccqsp5c9f2pacgwpppy96gqqaw55q25scsp4hx92swgs7dxc0d2n6yl0fs9qyysgqcqzp2xqrrssrzjq2yw0kt5emwglgd89n5t8yu6asl7v49xkzflzrhgu926pl9afqn62qqqqyqq3gcqqyqqqqlgqqqqqqqqfq3f4yzu7a9vx6jd8r7tmfxqzfv5p5p7l67d6swqgz45p0lthvaewysz4llneq2p83hkj8n5krnwthzqppxl523dsp3vkuxxk57a2gqzsq42yg7s"
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
>   "payment_id": "6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630",
>   "preimage": "fc2b1680b78296609e6515d258364cafadc08ba7c43cfa7142c1aeaf9cd7276f",
>   "amount_sats": "12",
>   "destination": "0295d9bf062f5683a0853c09475c95ffffd170cd21ccfe61e8cdb28a7869a4f50c",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630
>
> ```

**Run:**

```bash
$ rgbldk pay get 6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"6d5beea5f6c344d6d445c7645c095fe514f111d6460bac79890ac97ec643d630","preimage":"fc2b1680b78296609e6515d258364cafadc08ba7c43cfa7142c1aeaf9cd7276f","secret":"c152a0f7087042121748003aea500aa43100d6e62aa0e443cd361ed54f44fbd3"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx5k0u55gvyqqqqpkqqqqpqqqq9tv7mk82knnaeuf8rcv45h32cy5q68tqxh8r5hhuesd7vuhp9e9mqyps6uhtm4wr2kxq5yfuqxlfxcjwgux0cm9pfv0206d5y9xf0jg26msqwnjvdypudhyyjawhvvnnv8w2f00rxqgl94g8nl76gc7kvj2hguly7l8snhawl4z0ukahrjv4v2dcd4sj7pqr90sa297unxrwszgv8kn3kt40wqp7hfn5v0h00x9snnpyr7whdgl4g8h4fkna92q0ed4y2au0y6ezk0hsqxu36e8n02n4gsmcdaxgll6pvggrhrqgr9qtk0p7k2zv0twnj69epswy6yk2tw9vdwfny2taw6s9ul7q"
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
> | offer_id                  | 94a0aece1ccb777ec9470e2611d54ef18bf9e8c1cbbeaccea37d33c5206c33d2   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 03b8c081940bb3c3eb284c7add3968b90c1c4d12ca5b8ac6b9332297d76a05e7fc |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1784282277                                                         |
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
>   "payment_id": "d8a7fe6933d56510593ae1b48dd5f162e306f4012b14d0dfb8d88277422686bd"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait d8a7fe6933d56510593ae1b48dd5f162e306f4012b14d0dfb8d88277422686bd --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> d8a7fe6933d56510593ae1b48dd5f162e306f4012b14d0dfb8d88277422686bd
>
> ```

**Run:**

```bash
$ rgbldk pay get d8a7fe6933d56510593ae1b48dd5f162e306f4012b14d0dfb8d88277422686bd

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | d8a7fe6933d56510593ae1b48dd5f162e306f4012b14d0dfb8d88277422686bd                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 11886968095dbf9ea724a31c867c159ef29057c2b79595b72498e6a6e5021d6c                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                               |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"94a0aece1ccb777ec9470e2611d54ef18bf9e8c1cbbeaccea37d33c5206c33d2","payer_note":null,"payment_hash":"11886968095dbf9ea724a31c867c159ef29057c2b79595b72498e6a6e5021d6c","quantity":null} |
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
>   "refund": "lnr1qqsph6glehs0rwpkf8urqdmk2y32s6avt8ea28v5h0szavw9xm349mg2qq8qg6jeljk4qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxtlfddrgdk9z3wv2jgh4ev43mhs7vk8lqs5snf66gek7xp6lnv3pty9hyetxw4hxgttyv4kk7kkzqyqqqmqqqqqsqqqrez37j0j35vr30djnus9ek3zcxhvdlxtcgptkltfvhcyvd8su9neszqaemux2lz4r49nu84vh58pydw6xxeakpqyq5czthp4sw4nkfc5zp5q8fa4ytuaxmvp5mz3d7k05lqf5cy6usrmm0efc92exr982tmakk4kueye0tluervw3t006xh4uheejlumszv0yf9ryvcrkfuu3e8k5l4e6p05tzqdnjguc4txrkkv9eeynq9dvqqqlhh234lnm5v4jf4h7q3azx0yhad45qprt3alz5jjr826jvr2hfpc",
>   "payment_id": "3f68ebc47005ab0b890fc797f67ec72b7c9ee6e2cdc6a07e9f5029a35a646a25"
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
> | absolute_expiry_unix_secs | 1784282285                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 032fe96b4686d8a28b98a922f5cb2b1dde1e658ff042909a75a466de3075f9b221 |
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
> lni1qqsph6glehs0rwpkf8urqdmk2y32s6avt8ea28v5h0szavw9xm349mg2qq8qg6jeljk4qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxtlfddrgdk9z3wv2jgh4ev43mhs7vk8lqs5snf66gek7xp6lnv3pty9hyetxw4hxgttyv4kk7kkzqyqqqmqqqqqsqqqrez37j0j35vr30djnus9ek3zcxhvdlxtcgptkltfvhcyvd8su9neszqaemux2lz4r49nu84vh58pydw6xxeakpqyq5czthp4sw4nkfc5zp5q8fa4ytuaxmvp5mz3d7k05lqf5cy6usrmm0efc92exr982tmakk4kueye0tluervw3t006xh4uheejlumszv0yf9ryvcrkfuu3e8k5l4e6p05tzqdnjguc4txrkkv9eeynq9dvqqqlhh234lnm5v4jf4h7q3azx0yhad45qprt3alz5jjr826jvr2hfpaqacpg3e7ewn8derap5ukw3vunntkrlej556cf8ugwars4tg8uh4yz0fgreadgnr4dl7rce63m2lulsgyymlfy0tnmk6ah62nc7sqx6sh960uszqenvq3w3wc597cddnng893ndwhud3yrprhudjuf53cv99308d7kksqgs0fxzj4qgqn2v84klxfan4y4vxhm3r2ep23skqefqhm0ceuxzn6ksy6ry2mn9tp7e6scc9smcwqm8c3dcsv028nqphnseal23ntks03uem50ftfkz9jqysw4r8hzkq4tqw4htxd0qq72sz325q4y3gs36tdwz2fn992t63c02z9wx3wylcmp2gycff46y6ecsnae8cns84z7hp72jf6n5vl3sx4zrsqqqqqqqqqqqqqq9gqqqqqqqqqqqqgayjedltzjqqqqqq9yq349nm5l4qsdndvm9arw2ychtlj4qtzc4tutmdf88uxt7xvrxkjdkgfcts86g0d2qggwrtsrqgqqpvppqtsdtp5wtr7xks0fq9rzz2zd3ze6t6y5pt4gja6zk9t493mc8ryteuzqfhklzwzpmfr89va0genvqgew5k8tad9evpn3uufyzsuscnvt0nghx7yx9ckwjpkp39yqrd2zxtru5gglwpu2epkc55zur6n5j96yh6q
> payment_id: d9b59b2f46e513175fe5502c58aaf8bdb5273f0cbf198335a4db21385c0fa43d
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
$ rgbldk pay wait 3f68ebc47005ab0b890fc797f67ec72b7c9ee6e2cdc6a07e9f5029a35a646a25 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 3f68ebc47005ab0b890fc797f67ec72b7c9ee6e2cdc6a07e9f5029a35a646a25
>
> ```

**Run:**

```bash
$ rgbldk pay get 3f68ebc47005ab0b890fc797f67ec72b7c9ee6e2cdc6a07e9f5029a35a646a25

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | 3f68ebc47005ab0b890fc797f67ec72b7c9ee6e2cdc6a07e9f5029a35a646a25                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | d9b59b2f46e513175fe5502c58aaf8bdb5273f0cbf198335a4db21385c0fa43d                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                          |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"d9b59b2f46e513175fe5502c58aaf8bdb5273f0cbf198335a4db21385c0fa43d","quantity":null} |
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
>   "refund": "lnr1qqsw6u5ha4356a74h29073x4yqjkc77acv6k6pvgmpx4cp4mh3jeskg2qq8qg6jelje4qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqssxduq3u26xaw72gx0qvjdhvxjkd47q8lthr26pfu3k3yqehaax338tyfhyetxw4hxgttpvfskuer0dckkgetddadvyqgqqpkqqqqpqqqqyusgzkdz4z0hxqsks62lqfklgf7m7xhzkaphzn0uktfh6t7y2yuqqypmk3aa6jypsrtkmtg03egpghfex3vgvhlcq8vm5tsm4fy2uvl3yssqwjs60t3m59pw5dcwyzx6eazlfhrgjzrjewuhtg4el0kgvhsn0chv2tfc3nqcrv8zl4kx2fgnp5w9alhacud2m59nyuzxeyptgu5fahr96t0mlrwhfp6q8haypvj0p5ay9fy0a37f7ju9vgxg20r7rrhcjgq6husvyea9jauu4c44kr5r5vyk9xre79ws",
>   "payment_id": "bc460126a56a5d96bd96178ec4ddd820bc1545b2da7f3a2b7fa2e6cebadefa11"
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
> +---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------+
> | ID                  | Payment Hash        | Status    | Kind         | Dir      | Amount (msat)   | Fee (msat) | HTLC locked |
> +==============================================================================================================================+
> | 5ac6c1c5...ba35df0e | -                   | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 0c5e7a88...d533748b | 0c5e7a88...d533748b | Succeeded | Bolt11       | Outbound | 10,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 5fc93d2b...7b47e39a | -                   | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 00cdaee9...aee76fc3 | -                   | Succeeded | Onchain      | Inbound  | 1,000,000,000   | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 052eeb86...da95b8b9 | -                   | Succeeded | Onchain      | Outbound | 58,000,000      | 172,000    | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | ba2fd38a...67da3118 | -                   | Succeeded | Onchain      | Outbound | 40,000,000      | 254,000    | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | e426b083...03dd2e8f | e426b083...03dd2e8f | Succeeded | Bolt11       | Outbound | 13,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | ecca59d1...0fca7f54 | ecca59d1...0fca7f54 | Succeeded | Bolt11       | Outbound | 11,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 425ed4e4...6e55384c | 425ed4e4...6e55384c | Succeeded | Bolt11       | Outbound | 14,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | b57690a1...2e1cf4bd | -                   | Succeeded | Onchain      | Inbound  | 500,000,000     | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 6d5beea5...c643d630 | 6d5beea5...c643d630 | Succeeded | Bolt11       | Inbound  | 12,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | d8a7fe69...422686bd | 11886968...e5021d6c | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 3f68ebc4...5a646a25 | d9b59b2f...5c0fa43d | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | bc460126...badefa11 | -                   | Failed    | Bolt12Refund | Outbound | 1,111           | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4113d54b...cc0b3a24 | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 454557f3...e324f212 | -                   | Succeeded | Onchain      | Inbound  | 27,889,000      | 111,000    | false       |
> +---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------+
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
$ rgbldk rgb ln invoice estimate-carrier

```

**Result:**

> ```text
> +---------------------------------+-----------------+
> | Field                           | Value           |
> +===================================================+
> | Receive available               | yes             |
> |---------------------------------+-----------------|
> | Can create RGB invoice          | yes             |
> |---------------------------------+-----------------|
> | Blocking reason                 | -               |
> |---------------------------------+-----------------|
> | Required carrier (msat)         | 330,000         |
> |---------------------------------+-----------------|
> | Required carrier reason         | admission_floor |
> |---------------------------------+-----------------|
> | Available inbound (msat)        | 88,294,124      |
> |---------------------------------+-----------------|
> | Suggested action                | -               |
> |---------------------------------+-----------------|
> | Minimum viable carrier (msat)   | 1,000           |
> |---------------------------------+-----------------|
> | Minimum viable reason           | minimum_viable  |
> |---------------------------------+-----------------|
> | Default create carrier (msat)   | 330,000         |
> |---------------------------------+-----------------|
> | Default create reason           | admission_floor |
> |---------------------------------+-----------------|
> | Admission threshold (msat)      | 330,000         |
> |---------------------------------+-----------------|
> | Minimum allowed carrier (msat)  | 1,000           |
> |---------------------------------+-----------------|
> | Holder reserve threshold (msat) | 354,000         |
> |---------------------------------+-----------------|
> | Estimate only                   | yes             |
> +---------------------------------+-----------------+
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason | Suggested action | Min msat | Min reason     | Default msat | Default reason  |
> +==============================================================================================================================================================================================================================================================================================================+
> | c6cfc88b1f9ada1294685cbacb42e3ff7f7f11af6b6daef8a0e888ee79eefe2a | 502727c25c7d7a6ab2eb165b6baf5fd6 | yes    | 88,294,124   | 10,045     | yes     | yes     | yes        | 330,000       | 88,294,124     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------+
> This is a current-state estimate only. Invoice creation and payment claim can still see different channel state; unsafe incoming payments are failed backwards instead of being claimed.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln invoice create-for-hash --contract-id <contract_id> --asset-amount 2 --payment-hash <payment_hash> --desc "rgb ln hold demo" --btc-carrier-amount-msat 3000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt30u1p49nm4tdq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp55lx6t87zq0479fcs7czcr8qfcrvqqq34htullq2qtzzzaxr9xawq9qrsgqxqrrsscqpjlp5kmf9cukgcav90gv2ydutkn72hyflraya7kvha3s4zh4pr8tnqr4q7qpzsmpz9acvufysljdx5g7vqz4dsxhckzq690q490agrfhh6m6ppx5x6p0qh8yd3pvs3msdnrn5zvksqde207s2qpz8p35vle69as6qc4gqd77xwj",
>   "btc_carrier_amount_msat": "3000000"
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
> | Destination           | 0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo          |
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
>   "invoice": "lnbcrt50u1p49nm4ddqjwfnkygrvdcsxgetddupp5xs9fdvedc803wn6hkg07zltx3zulug2kxamv49sr8uwt4prm9kdqsp5llszsu2utss69rnlcgpmu9ruavpv5rqd70kcp067j7yqrdqkfe0s9qrsgqxqrrsscqpjlp5kmf9cukgcav90gv2ydutkn72hyflraya7kvha3s4zh4pr8tnqr4q7qp9ujxfpwlqp3943azut3aawgl7rgrls9yqr8xrp063eclna96s47wrnk4tp0m2v6zxzkxayyaatg4u3q2fcy87rt99vj4rdc3r3fe7zkcqfwzu9x",
>   "btc_carrier_amount_msat": "5000000"
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
> | Payment hash          | 340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 0288e7d974cedc8fa1a72ce8b3939aec3fe654a6b093f10ee8e155a0fcbd4827a5 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo          |
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
>   "payment_id": "340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a
>
> ```

**Run:**

```bash
$ rgbldk pay get 340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"340a96b32dc1df174f57b21fe17d6688b9fe21563776ca96033f1cba847b2d9a","preimage":"2594760a3b44de21f169fe5d0c4e63946e922b28b3e8aba095427a8aefdfa810","rgb":{"asset_amount":"5","contract_id":"contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo","direction":"Outbound","is_swap":false},"secret":"ffe028715c5c21a28e7fc203be147ceb02ca0c0df3ed80bf5e978801b4164e5f"} |
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
$ rgbldk rgb ln invoice estimate-carrier

```

**Result:**

> ```text
> +---------------------------------+-----------------+
> | Field                           | Value           |
> +===================================================+
> | Receive available               | yes             |
> |---------------------------------+-----------------|
> | Can create RGB invoice          | yes             |
> |---------------------------------+-----------------|
> | Blocking reason                 | -               |
> |---------------------------------+-----------------|
> | Required carrier (msat)         | 330,000         |
> |---------------------------------+-----------------|
> | Required carrier reason         | admission_floor |
> |---------------------------------+-----------------|
> | Available inbound (msat)        | 14,045,876      |
> |---------------------------------+-----------------|
> | Suggested action                | -               |
> |---------------------------------+-----------------|
> | Minimum viable carrier (msat)   | 1,000           |
> |---------------------------------+-----------------|
> | Minimum viable reason           | minimum_viable  |
> |---------------------------------+-----------------|
> | Default create carrier (msat)   | 330,000         |
> |---------------------------------+-----------------|
> | Default create reason           | admission_floor |
> |---------------------------------+-----------------|
> | Admission threshold (msat)      | 330,000         |
> |---------------------------------+-----------------|
> | Minimum allowed carrier (msat)  | 1,000           |
> |---------------------------------+-----------------|
> | Holder reserve threshold (msat) | 354,000         |
> |---------------------------------+-----------------|
> | Estimate only                   | yes             |
> +---------------------------------+-----------------+
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason | Suggested action | Min msat | Min reason     | Default msat | Default reason  |
> +==============================================================================================================================================================================================================================================================================================================+
> | c6cfc88b1f9ada1294685cbacb42e3ff7f7f11af6b6daef8a0e888ee79eefe2a | 9952853075557233da3c29cc462de631 | yes    | 14,045,876   | 84,954     | yes     | yes     | yes        | 330,000       | 14,045,876     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------+
> This is a current-state estimate only. Invoice creation and payment claim can still see different channel state; unsafe incoming payments are failed backwards instead of being claimed.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb ln invoice create --contract-id <contract_id> --asset-amount 3 --desc "rgb ln demo back" --btc-carrier-amount-msat 4000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt40u1p49nm44dq6wfnkygrvdcsxgetddusxyctrdvpp5m647q54rly9q7ft9zsxsuj46jrkmrcm0pa8ej9h6p68ad8xprytssp56v4yj2enap2cksvjhlute9sv3uudpg0awtn29esyrhfx59tg2llq9qrsgqxqrrsscqpjlp5kmf9cukgcav90gv2ydutkn72hyflraya7kvha3s4zh4pr8tnqr4q7qpr9t65xerm29kzpjse0tl6sgauuj5rt9nv7dh8h6dlm7jkja3q2xxppktj7jhjtgx9s5m6470n0eluzwzxjdkfr5g9v2gytshnm3jhztqqkydeaw",
>   "btc_carrier_amount_msat": "4000000"
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
> | Payment hash          | deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 0295d9bf062f5683a0853c09475c95ffffd170cd21ccfe61e8cdb28a7869a4f50c |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo          |
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
>   "payment_id": "deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917
>
> ```

**Run:**

```bash
$ rgbldk pay get deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"deabe052a3f90a0f2565140d0e4aba90edb1e36f0f4f9916fa0e8fd69cc11917","preimage":"a7cbdee649cb4d8237a8717cbae03b620da0a6a023f58aab17d1e9f72f711e1c","rgb":{"asset_amount":"3","contract_id":"contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo","direction":"Outbound","is_swap":false},"secret":"d32a492b33e8558b4192bff8bc960c8f38d0a1fd72e6a2e6041dd26a156857fe"} |
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
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote |
> +===============================================================================================================================================================================+
> | 99528530...462de631 | 0288e7d9...bd4827a5 | 146235046559745 | 1099657576452 | 1099656527873 | 100,000         | true  | true   | contract...-EZ1zAOo | 6         | 4          |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+
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
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote |
> +===============================================================================================================================================================================+
> | 502727c2...6baf5fd6 | 0295d9bf...69a4f50c | 146235046559745 | 1099656527873 | 1099657576452 | 100,000         | true  | true   | contract...-EZ1zAOo | 4         | 6          |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+
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
>   "payment_id": "bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714
>
> ```

**Run:**

```bash
$ rgbldk pay get bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"bfae8da3a0b64ac7c2ce01d722be3405febfd48a4933271041c643bf07776714","preimage":"640118b9ea5005d50791b170a27f531daa1d8dbecbd9c6f70cd21b96dcd169a6"} |
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
>   "payment_id": "3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"3312d595c049c337950109274dbaa19f6df82cb855743c7bad07e9747db3338b","preimage":"e38760b6fab079c63a16c7678606ca506926414f6f6178352ab2ef31fd46c618"} |
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

## 10) Events (next/handled)

Demonstrate the event queue API: fetch the next event (`events next`) and acknowledge it (`events handled`) so the daemon can advance the queue.


### Queue

Tip: `events watch` blocks until it receives events. This example uses a short timeout so the generator does not hang if the queue is empty.


**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=a3ee11926a80b008f3bedd6dbf3a5b76eaf0a51db898ca8c8da7fd34022771dd:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=a3ee11926a80b008f3bedd6dbf3a5b76eaf0a51db898ca8c8da7fd34022771dd:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=0bd4d448c768029a3207455a9e8e5ecc
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
> ChannelReady user_channel_id=40b916fd38a2297d99bdfabb8faebdbd
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

## 11) BTC on-chain settlement (L1, node-a → node-b via channel push+close) + Channel force-close

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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "0d062341262aa5a46ef5db6c0fe277a5fdef72e46b375d1f2e046fdf53dafbba",
>   "7490dd15666543bc81a9ed1e67b18a9f94dd0fa3677a9a35d3f835bce50406ce",
>   "152badd881054650d422a09a2c2018fce424f4a7c3f66308cf6e2cc7e165f547",
>   "0cabfea83f48b9207239aca300a2952cd44514f16f3ba19d69984a92b5cafe45",
>   "5d04d75ee6bcdf855de1b32be6195d27a4434698c9ae66afd8fc198976c8be22",
>   "1ae459030252894cd086d87afb5e50e8f250c3f45615f8f23094f4e5ea6c68b8"
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
> BTC balance change: on-chain total +86,941 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +11,047 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (total)     | 1.01546404 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.01459463 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    86,348 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |     7 |         0 |        0 |     7 |
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
> | BTC On-chain (total)     | 99,978,121 sats |
> |--------------------------+-----------------|
> | BTC On-chain (spendable) | 99,967,074 sats |
> |--------------------------+-----------------|
> | BTC Anchor reserve       |          0 sats |
> |--------------------------+-----------------|
> | BTC Lightning (total)    |     11,047 sats |
> +--------------------------+-----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo | contract:ttJccsjH-WFehiiN-4u0~KuR-Px9J31m-X7GFRXq-EZ1zAOo |    83 |         0 |        0 |    83 |
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
>   "user_channel_id": "59aaf75af483ba23153229d386387693"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q4357fvm9k2w4046rnre0l0mfr3l69ntnz9frv6

```

**Result:**

> ```text
> [
>   "27b6c94aa093c55c6e66bd350eb957dd253d619c899bd1d3aaa034c9a947a40b",
>   "2e92a70f5c65acc0709b95e542a29bee3f0ee305c4af54e9e0466f99a4b7c7cf",
>   "7283cbc3ab32d510c3e7c4b66cd1708240012677ca845e9fac063570fea3b3b8",
>   "714c567ba429d3d3b4f00d99ce1dda75f79de1f24289c8c7e3fdc909a8df87b8",
>   "6e4be067895e57762a042f016edd8ff9945f6d1f1e37fa39c0d177a9e6b777f7",
>   "4477a1a780d806eeb4005f8afdfb8ea4b564846cf791f9e6a0c2b3f14c9d393d"
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

## 12) Cleanup

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
