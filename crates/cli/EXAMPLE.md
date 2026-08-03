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
esplora=http://127.0.0.1:3003

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
>   "node_id": "038302c28c8511d7d41136a5d19ca7f44cb5d3e645ca5bb5d69021b0238bb2323d"
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
>   "node_id": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347"
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
> {"keystore_path":"/tmp/rgbldk-cli-example-26j0ei36/keystore-init-demo/keystore","mnemonic":"disease bar confirm achieve brown strong side vocal tank climb ladder rookie snake chunk anxiety legal onion flash green wrestle enable pulse side prosper","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-26j0ei36/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-26j0ei36/keystore-migrate-demo/keys_seed.bak.20260803-072011","ok":true}
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
> | 038f0c59...fce55347 | 127.0.0.1:9736 | true      | true      |
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
>   "address": "bcrt1qvcppzgyzzq6rnyvksa0d7r0jqcac9g6tvhhlat"
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
>   "address": "bcrt1q87mcwpwwtlypze7fur8m0c46xnmz33qs6q8eqj"
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
> bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qvcppzgyzzq6rnyvksa0d7r0jqcac9g6tvhhlat 1

```

**Result:**

> ```text
> 29f3cff4b2237e97ee40ba108b935ac51e75556da81b7fa225c658a36c9cd261
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1q87mcwpwwtlypze7fur8m0c46xnmz33qs6q8eqj 1

```

**Result:**

> ```text
> 5d7dc848667229be98eaddf495a2e683835512af4cd422223e29b99e5c49fc91
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "3a914fb8727f7fa2777a424de67f674716229dbc443cd80380cdc35e2783f007",
>   "6be2527ecee5e2bf82ee727944c6d7638a522c86d061f41d03abbfd2d7f70664",
>   "611c96bfadd0ee568a53d87848a1e329915dcf8be08c0cd1a6bde345d84a7480",
>   "196af7197de94d714aee5eaf6f4f85d001306d243caff22e8033f6015e0be5f9",
>   "60a46d5e1b3f3c944da11815584f97246e07120bb3e6c589911f0a70b2dd7005",
>   "4c9f968d4c11394ed9d7b7d775b191ca4389e6b82fb22efe14c0b4be9cb3b227"
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
> | 29f3cff4b2237e97ee40ba108b935ac51e75556da81b7fa225c658a36c9cd261:1 |  100,000,000 | Confirmed |    102 | Available    | -        |            - |
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

This demonstrates a BTC L1 transfer between the two nodes by using a channel open with `--push-msat` (gives the receiver an initial balance), inspecting the public network graph once the channel confirms, trying both splice directions on the pure-BTC channel, and then using a cooperative `channel close` to settle on-chain. While the close is in flight, `channel closing` lists the channel until funds land back in the wallet (absent from both `channel ls` and `channel closing` means fully settled).


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
>   "user_channel_id": "bbd5598910904d025c3f20d61631b5ee"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "678952eb3f28594bd4316b6333fc182e8c0161ee0f6d7490bea6416d8575259f",
>   "564f19e5759bb798b879f72c364fb46b8d4b477f5436f0434b015ea54e03ef08",
>   "2e1ec8bf77ea3efd2aaf69a24c7b030c05b342d8f50b5029ae0d6db8de582f77",
>   "3714294a36614e4a8d6f182c814a9ca7f413416825608d0450ad23319a1224ad",
>   "1bb7d9700c3efaf6279d3c8e6dd245e3336369b6c6b1d804eef13d50c178289b",
>   "30c54dee672afe14ef9306da1fd2f2ea32c0a363bf8f53b684d4a4479d8ec065"
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
> | bbd55989...1631b5ee | 038302c2...8bb2323d | 118747255865345 | 1099626643463 | 1099583389703 | 120,000         | true  | true   |
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
> | 2454ad24...1e1b0175 | 038f0c59...fce55347 | 118747255865345 | 1099583389703 | 1099626643463 | 120,000         | true  | true   |
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
> | 038302c28c8511d7d41136a5d19ca7f44cb5d3e645ca5bb5d69021b0238bb2323d |
> |--------------------------------------------------------------------|
> | 038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347 |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 038f0c59...fce55347.
> Known channels: 1.
> +-----------------+
> | SCID            |
> +=================+
> | 118747255865345 |
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
> | 118747255865345 |
> +-----------------+
>
> ```

**Run:**

```bash
$ rgbldk graph channel <scid>

```

**Result:**

> ```text
> Channel 118747255865345 connects 038302c2...8bb2323d and 038f0c59...fce55347.
> Capacity: unknown.
>
> 038302c2...8bb2323d -> 038f0c59...fce55347: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1785741624       |
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
> 038f0c59...fce55347 -> 038302c2...8bb2323d: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1785741624       |
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
>   "address": "bcrt1qfxkfewueecz0v0k6hfaq0uzqd05kgazyfnh2w3"
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
> | 2454ad24...1e1b0175 | 038f0c59...fce55347 | 118747255865345 | 1099583389703 | 1099626643463 | 120,000         | true  | true   |
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
> | bbd55989...1631b5ee | 038302c2...8bb2323d | 118747255865345 | 1099626643463 | 1099583389703 | 120,000         | true  | true   |
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

#### Closing observability

Immediately after close initiation, poll `channel closing` on both peers. Entries progress through `negotiating` → `broadcasting` → `confirming` (and optionally `sweeping`) until they disappear.


**Run:**

```bash
$ rgbldk channel closing

```

**Result:**

> ```text
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
> | User Channel ID | Channel ID          | Counterparty        | Status       | Source  | Closing Txid | BTC (sats) |
> +==================================================================================================================+
> | -               | 2b56c893...40c7ba2e | 038302c2...8bb2323d | broadcasting | unknown | -            | 87,654     |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
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
$ rgbldk channel closing

```

**Result:**

> ```text
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
> | User Channel ID | Channel ID          | Counterparty        | Status       | Source  | Closing Txid | BTC (sats) |
> +==================================================================================================================+
> | -               | 2b56c893...40c7ba2e | 038f0c59...fce55347 | broadcasting | unknown | -            | 30,000     |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "66efdcc8b5d5e93fb15c27ce6f8a9c7bf5e62643d5ce4c23c22f96ac51d683ac",
>   "0a80f0f3acac76ff2b0dfb247ca24fc1324b939f77c2ea36a61014e5a459d025",
>   "275d0a552724ee41b6dbcae8306cce27233493fbf4ffa23729c0e751a0b55a65",
>   "7d7ac1c38a171062777bd6c766e4dc9cd4e7d730ddd1a2f0d424150ca5dee71c",
>   "03e736897152276e6dd0668bcc2453049d05de4bc6efec9f50fb0fb35e47c9ae",
>   "1f1ca09b642951174aa29528c6771c520b3d7f39f4e063567077d6933bbc3662"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "6799483325bd3afeca7dbc5e157ca0a078a312b4f9f0cba904eeff98f037ae3d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "734019f48fb232ae4c53a094c7b9a96860e8b06528da9ca50084e24d835bd416"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel closing

```

**Result:**

> ```text
> No channels currently closing.
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
$ rgbldk channel closing

```

**Result:**

> ```text
> No channels currently closing.
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
> No balance change.
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
>   "address": "bcrt1qnavw65x90vfa87qr9ks56qk4a07nf54vjsyjas"
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
> rgb(wpkh([a35a51ec/86h/827167h/0h]tpubDCbMYNMsBa6Le8jaA8H4AHcQhiTpvLobJ78asGADBUtB8TBMa2WunRB4WwBev8bRrmHk9c7Womtk5gBMTppspLiyo33GhSNbyE9wPW8vAs9/<0;1>/*),seals(),noise(0d30bc692b3973758762ee49caab5d87e4137c0649a25df45d03612b33e9bcfc))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | a35a51ec    | m/86'/827167'/0' | tpubDCbMYNMsBa6Le8jaA8H4AHcQhiTpvLobJ78asGADBUtB8TBMa2WunRB4WwBev8bRrmHk9c7Womtk5gBMTppspLiyo33GhSNbyE9wPW8vAs9 | wpkh([a35a51ec/86'/827167'/0']tpubDCbMYNMsBa6Le8jaA8H4AHcQhiTpvLobJ78asGADBUtB8TBMa2WunRB4WwBev8bRrmHk9c7Womtk5gBMTppspLiyo33GhSNbyE9wPW8vAs9/<0;1>/*)#9wepp8jf |
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
> | Public key      | 023eac4aad5ba0b1caffddbfa2b79cdf0acde2e57b940da9a74be5be7603ddc91d |
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
> 304402204796327d91922d27dd9218632abd0cc292f7e97439967649ff9529190978d5fd022061635c8edc27c02fc68238f9c1e7998221d2f3b9eb9a3b2e92bdd63883964291
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
>   "address": "bcrt1qftyaxmvhvgpwp82qwgyxxu8l55xamyn5glq3u7"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> d386cf4af6f8acd399ea3ec72b82ecda2abacf93fdb1518299bec4cfd5471741
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> 091bcb957d7bad1278f9d9d933f01329cc4e8e0c688609d92dada855b9f1b9e8
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "11f1ac739380327d7acee585a20a406550a5cbeadfd07c1f560fa7b05bb3134c"
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
> | 091bcb957d7bad1278f9d9d933f01329cc4e8e0c688609d92dada855b9f1b9e8:1 |      110,000 | Confirmed |    122 | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------|
> | d386cf4af6f8acd399ea3ec72b82ecda2abacf93fdb1518299bec4cfd5471741:0 |   10,000,000 | Confirmed |    122 | Available    | -           | FeeSupport, BlindingTarget |
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
> | 091bcb957d7bad1278f9d9d933f01329cc4e8e0c688609d92dada855b9f1b9e8:1 |      110,000 |    122 | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | d386cf4af6f8acd399ea3ec72b82ecda2abacf93fdb1518299bec4cfd5471741:0 |   10,000,000 |    122 | Available |        |
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
>   "reservation_id": "manual-reservation-1785741650-InsxEDaArm0RUNwj0af6GjP2Xo2IODyd",
>   "outpoint": "091bcb957d7bad1278f9d9d933f01329cc4e8e0c688609d92dada855b9f1b9e8:1",
>   "reserved_until_unix_secs": "1785741710"
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
>   "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
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
>       "detail": "091bcb957d7bad1278f9d9d933f01329cc4e8e0c688609d92dada855b9f1b9e8:1"
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
> | DemoAsset | DEMO   |         0 |    100 | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
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
$ rgbldk rgb contracts export --contract-id contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY --out /home/astral/work/rgb-ldk-cli/target/rgbldk-example/contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.
> Consignment key: contract_export_b63d86e5f0f672a2e7f3231419352f2f90f2d815c5382d825b5d3a918d2ceede
> Saved to /home/astral/work/rgb-ldk-cli/target/rgbldk-example/contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=091bcb957d7bad1278f9d9d933f01329cc4e8e0c688609d92dada855b9f1b9e8:1
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip

```

**Result:**

> ```text
> {
>   "bytes": 2991,
>   "export": {
>     "checks": [
>       {
>         "name": "rgb_enabled",
>         "ok": true
>       },
>       {
>         "detail": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_03a3f2cea3639b397d30f553a62b128d5dd8817489155f2e36eb16beafa9feb0",
>     "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>     "ok": true
>   },
>   "out": "/home/astral/work/rgb-ldk-cli/target/rgbldk-example/contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_03a3f2cea3639b397d30f553a62b128d5dd8817489155f2e36eb16beafa9feb0.
> Saved to /home/astral/work/rgb-ldk-cli/target/rgbldk-example/contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.download.zip.
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
>   "address": "bcrt1qgt7l8ezrdztqjukzmskp6v3zkh5srzgekgahwe"
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
> 54e3ccd0e0439a9bdea792000183a1db6e804ca12f83dc9a4177b319cb3ffce4
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "2ab20d9c482954fe65edc28ae7651c344b1d9d802e92879697594d37c79232e8"
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
$ rgbldk rgb contracts import --contract-id contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY --file /home/astral/work/rgb-ldk-cli/target/rgbldk-example/contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_b539609b73007975ac9e74e20637af819573a02d51a3beed9476d282fe0157f0
> Imported contract contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.
> Consignment key: contract_import_b539609b73007975ac9e74e20637af819573a02d51a3beed9476d282fe0157f0
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
> | DemoAsset | DEMO   |         0 |    100 | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
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
> The node knows contract contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.
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
>   "invoice": "contract:tb@XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY/90@at:7tTDQRHg-23U1W9ju-W6xnUtWz-8YifTkfo-Fme7XtRG-jykpSg/?expiry=2026-08-03T08:20:56.831821381+00:00",
>   "blinding_utxo_used": "54e3ccd0e0439a9bdea792000183a1db6e804ca12f83dc9a4177b319cb3ffce4:0"
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
> | Contract         | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:7tTDQRHg-23U1W9ju-W6xnUtWz-8YifTkfo-Fme7XtRG-jykpSg    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1785745256                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY/90@at:7tTDQRHg-23U1W9ju-W6xnUtWz-8YifTkfo-Fme7XtRG-jykpSg/?expiry=2026-08-03T08:20:56.831821381+00:00
> scope=contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:7tTDQRHg-23U1W9ju-W6xnUtWz-8YifTkfo-Fme7XtRG-jykpSg
> data=90
> expiry=2026-08-03T08:20:56.831821381+00:00
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
>   "txid": "44632c79a54eb836acea369d4960578117559608696d9f3a889cd55184717eed",
>   "consignment_key": "rgb_consignment_contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY_44632c79a54eb836acea369d4960578117559608696d9f3a889cd55184717eed"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY_44632c79a54eb836acea369d4960578117559608696d9f3a889cd55184717eed.
> Saved to /home/astral/work/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-a-to-b-prechannel.zip.
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
> Accepted contract contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "5107226d52e28ab5a3d05e73450e8c5c2acf0b8092d7f3f76478ac156e841cdc",
>   "64ba82c71aa1269b9363513376f5ef641a896e2aa7396c293265f3e1eaaa00d9",
>   "0b8e1f4863fddba7e2a5c0647fa1fae873bccb406d8510c9b338fbd0963a7b4d",
>   "03c19a24a3bfe0d6bd35507e8009706e8d8b24e8d7edf375513a842585f35d8d",
>   "17a22abc2de56d5e6d3cfb930caa4324dcbcd222d9a0be0c7ead7dfb3afaeb65",
>   "6ee8c37ec4699c5c249751951879ffce3fbea71f73603c6941d5d13a76226370"
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
$ rgbldk rgb contracts balance contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
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
> | 00348bf36ab878bd76860c19c46d634e4b8b0afdce41c49c3295403296282dc1 | succeeded | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |     90 | 44632c79a54eb836acea369d4960578117559608696d9f3a889cd55184717eed | transfer_import_f26585fc0dc12e1f4e57cc34a9fa889999acafcd0024464064fbe2db16dc45ec |
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
$ rgbldk rgb contracts balance contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
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
>   "user_channel_id": "16d8c5522048305ecff6fcf47df3f829"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "5ff250248abc62f2a989f1931906663aee87f6610e962a5dc2e7c49dc166a306",
>   "413d33769e5bf25ce951ea013a36fe4ebf0fb1a34a8807f1447789ae1bff4353",
>   "09b38dafbaead3f4a81036133555991c4ca49c2aa3c6c1b2188eb15c9442dc23",
>   "16295f65fb74cf145afe70d1b53ed3be4266617428c85c12484d1289d3523992",
>   "6d463d3ced057ec06b6ae871adff097ccda4c08e2fbe7d99998faf196e184c02",
>   "441243fe11f7ce75756b7c9ce05d29d8817f60c8a0df84696a0d1f1656b47668"
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
> | 16d8c552...7df3f829 | 038f0c59...fce55347 | -    | -         | -        | 100,000         | false | false  | contract...-Rd_o8pY | 10        | 0          |
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
>   "invoice": "contract:tb@XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY/7@at:AKel1Txl-irtIxd2V-N5LTxV3c-~SU98stA-7fBtxUh8-jRMTJQ/?expiry=2026-08-03T08:21:22.375992726+00:00",
>   "blinding_utxo_used": "d386cf4af6f8acd399ea3ec72b82ecda2abacf93fdb1518299bec4cfd5471741:0"
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
>   "txid": "b5de5e7a18c14ed25cd25c76e65b36483ee9baae7266a7278b987981e3c4e60d",
>   "consignment_key": "rgb_consignment_contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY_b5de5e7a18c14ed25cd25c76e65b36483ee9baae7266a7278b987981e3c4e60d"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY_b5de5e7a18c14ed25cd25c76e65b36483ee9baae7266a7278b987981e3c4e60d.
> Saved to /home/astral/work/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-b-to-a.zip.
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
> Accepted contract contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1cbd7b33640467f97b924219c2a75e8ab10d2efcac65d83b9777ad5e78e144ea",
>   "6f2019909a93f19a2c3895ccc063eb68be6dcb7f3e0c6e1b03acbf6271debcd4",
>   "335b66a3b69d12f6ae7e0a7c9df3077008da3d2f4193fd9fd9c15c0752c68cd8",
>   "20afccf90d14f1aefbfd070ce852a9e0592c7b5990f1f8c7fc7d9ace0e45eb55",
>   "2d087abe01b1b83812d67775863930046d5625bc0df8fa89935acc3b693adfa2",
>   "7bb73903a0e9f46ebf3b2029487e6c22473c6d0a5ca009ba5301191debe91dab"
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
$ rgbldk rgb contracts balance contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |
> |-----------+-----------------------------------------------------------|
> | Mined     | 0                                                         |
> |-----------+-----------------------------------------------------------|
> | Tentative | 7                                                         |
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

#### RGB UTXO lifecycle (fund/top-up/sweep)

`fund` creates empty RGB wallet outputs, `top-up` increases a single-asset RGB UTXO's bitcoin capacity, and `sweep` spends an empty RGB wallet output back to the BTC wallet. These low-level UTXO tools are different from `rgb onchain send`, which moves contract allocations between wallets.

Run this lifecycle on node-b (which now holds 90 units) so node-a's remaining 10-unit UTXO stays eligible for the RGB channel open that follows.


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
>   "address": "bcrt1q4huz8tsscz354dgtd25elm8pswqxeyq3lla5n6"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> b3bf9774960b46d9facde0de34e55ee90c6ad921b4b7c06c8560ca31c4e817c1
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "2cb8afc92ddc15ee26e7e6417dc24d9501316e695de6c64b60c14f2bde516eaa"
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
>   "address": "bcrt1qtcwq0wj9enluzv4559lrp56cqu86447s6pvcn3"
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
>   "address": "bcrt1q4regvfma7p5xuedkgcs6tgrvwmz3pp5atdxyev"
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
>   "address": "bcrt1qf6m4y8n3rmh0e7qkzch2w9e8ly40llp6pfpmva"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"7b6c7eafe8d99efcc9b9bc28a436096cf2a593f46bea5854790c3ca396a07446","status":"broadcast","outputs":[{"address":"bcrt1qtcwq0wj9enluzv4559lrp56cqu86447s6pvcn3","value_sats":"30000","vout":2},{"address":"bcrt1q4regvfma7p5xuedkgcs6tgrvwmz3pp5atdxyev","value_sats":"28000","vout":1}],"change":{"address":"bcrt1qf6m4y8n3rmh0e7qkzch2w9e8ly40llp6pfpmva","value_sats":"941828","vout":0},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1ba8e29b6efa8ba57cf2b17aecbf7d4f335d110e33107b4fee4b38bc9e959ffb"
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
>   "address": "bcrt1qw97dsnutfyvtarzwny2drvv4xw542qlld9v790"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> e8959377ade1b53d1581682b0f3a4e54682284fd91cfd61f33531e50227109db
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1c16a7ddde3c0ffb1aaa7459cd62cec251908b85f319c7852aafa169077fd237"
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
>   "address": "bcrt1qjsqujq3jcd9natx2lfkkpvln7j22dfakdjxnag"
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
>   "address": "bcrt1qxzprmyddj4gawcmdtlktjjj50fkdtjjk02k8xl"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO b5de5e7a18c14ed25cd25c76e65b36483ee9baae7266a7278b987981e3c4e60d:1.
> Broadcast transaction: 1fa804c4e36ba4e3f0e3134f5d78b72056638f2fe2e947bf3320f84176d8a70c
> Replacement output: bcrt1qjsqujq3jcd9natx2lfkkpvln7j22dfakdjxnag with 10,039,800 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY_1fa804c4e36ba4e3f0e3134f5d78b72056638f2fe2e947bf3320f84176d8a70c
> Change output: bcrt1qxzprmyddj4gawcmdtlktjjj50fkdtjjk02k8xl with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "534138c4eb4637183e9ad2d34c15d77d457302506b3a2b6cbffe2ae7a7d7815e",
>   "5451a38c33fa0425d0640565547e2dd0a7d4bee2a2a7ead7aa689dd870528cf2",
>   "0d342d36fdb900ab36455d3bb9e06115cd2e323a966197349ddadb07a5658d83",
>   "3e9a647332f396e766ba6ddd3fb73bece2fd18e93e94d7d187d7725d7e183cfc",
>   "5ad01536126e673b1ad4857762fc0f098319d748c0bcf32dfa61329639aa55f8",
>   "1abc46c18a2b68ac0ce10a4391dabbf842cc79c5d1a4d08a300d262288150a7b"
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
> BTC balance change: on-chain total -40,254 sats, spendable -500,000 sats, anchor reserve 0 sats, lightning 0 sats.
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
>   "address": "bcrt1qrjjcahr4urp79wgy9ylh2v7rz5tt3z29pl5rah"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO 7b6c7eafe8d99efcc9b9bc28a436096cf2a593f46bea5854790c3ca396a07446:1.
> Broadcast transaction: 09607b9822bba70276f9c7797bde4f0b218e06d4fc03a40d5419fb57fd1d610e
> Sent 27,889 sats to bcrt1qrjjcahr4urp79wgy9ylh2v7rz5tt3z29pl5rah.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "4b50908e93397d485203d84fd32f3669622d43ce2c8de88ec39aa3bc3450ecee",
>   "724826032818ca0e1b32829dc43e947fe31087a0e755068ed28c90fcfadb1a16",
>   "65b5882e5ada629d1ba915d03b43ed10f6e790b0d69d7ae3add928e65543c3bd",
>   "65c1bf407bd08ccaea01001d11e9d6cf24d79de2cf5a7827650327b484113b84",
>   "116be7a5a8c5fbff790e18f1b9cfe65262268e6414c8d222632d18da98cbc174",
>   "7db677891304efd7ebca6b73f038a41d151838845766e34b3ad93ec3868640c9"
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
>   "swap_string": "rgb-swap:v1:eyJwYXltZW50X2hhc2hfaGV4IjoiYzc4MjRlMTExZWZhNzJhZjUzNTRmNjcxNTBjYTZiODc0ZjYxODkxZTM0NmJkYWU5NjE4OGQ5OGZlMzZlNWYxMyIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAzOGYwYzU5ZmQ0MzAwMWFmZTIxNTY0NThhYzMwZDA1ZGI5YzhiYTIyZjY2ZDljMGNmMmIwMGQxYjZmY2U1NTM0NyIsImNoYW5uZWxfc2NpZCI6MTQ5NTMzNTgxNDQzMDczLCJjb250cmFjdF9pZCI6ImNvbnRyYWN0OlhLRDY1bnpCLVlFZ21TcmUtOFpfTzFuMi12b1JMQU1QLWdENXlqZzktUmRfbzhwWSIsImFzc2V0X2Ftb3VudCI6MiwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwMCwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOjUwMDAwMDAsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiZXhwaXJ5X3NlY3MiOjM2MDAsImNyZWF0ZWRfYXRfdW5peF9zZWNzIjoxNzg1NzQxNjkzfQ",
>   "payment_hash": "c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13",
>   "info": {
>     "payment_hash": "c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347",
>     "channel_scid": "149533581443073",
>     "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>     "asset_amount": "2",
>     "btc_amount_msat": "10000000",
>     "btc_carrier_amount_msat": "5000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1785741693",
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
>   "payment_hash": "c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13",
>   "role": "Taker",
>   "status": "Offered",
>   "counterparty_node_id": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347",
>   "channel_scid": "149533581443073",
>   "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1785741693",
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
>   "payment_hash": "c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13",
>   "role": "Taker",
>   "status": "Accepted",
>   "counterparty_node_id": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347",
>   "channel_scid": "149533581443073",
>   "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1785741693",
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
$ rgbldk swap get c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Accepted                                                         |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 038f0c59...fce55347                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 149533581443073                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-Rd_o8pY                                              |
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
> | Created (unix secs) | 1785741693                                                       |
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
>   "payment_hash": "c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13",
>   "status": "InFlight"
> }
>
> ```

**Run:**

```bash
$ rgbldk swap get c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 038f0c59...fce55347                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 149533581443073                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-Rd_o8pY                                              |
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
> | Created (unix secs) | 1785741693                                                       |
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
$ rgbldk swap get c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Taker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 038f0c59...fce55347                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 149533581443073                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-Rd_o8pY                                              |
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
> | Created (unix secs) | 1785741693                                                       |
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
> | c7824e111efa72af5354f67150ca6b874f61891e346bdae96188d98fe36e5f13 | Taker | Settled | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY | 2     | RGB         |
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
>   "swap_string": "rgb-swap:v1:eyJwYXltZW50X2hhc2hfaGV4IjoiNDU0MjQ5NjJjZDA2N2ZlZjQ0MGNlMGQzNjRiNDZjODEyMzUxZWJlZjRiNjQ2MmExZjA2Yjc3MTY5N2I4YzgxOSIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAzOGYwYzU5ZmQ0MzAwMWFmZTIxNTY0NThhYzMwZDA1ZGI5YzhiYTIyZjY2ZDljMGNmMmIwMGQxYjZmY2U1NTM0NyIsImNoYW5uZWxfc2NpZCI6MTQ5NTMzNTgxNDQzMDczLCJjb250cmFjdF9pZCI6ImNvbnRyYWN0OlhLRDY1bnpCLVlFZ21TcmUtOFpfTzFuMi12b1JMQU1QLWdENXlqZzktUmRfbzhwWSIsImFzc2V0X2Ftb3VudCI6MSwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJidGNfY2Fycmllcl9hbW91bnRfbXNhdCI6MTAwMDAwMCwibWFrZXJfZ2l2ZXNfcmdiIjp0cnVlLCJleHBpcnlfc2VjcyI6MzYwMCwiY3JlYXRlZF9hdF91bml4X3NlY3MiOjE3ODU3NDE2OTZ9",
>   "payment_hash": "45424962cd067fef440ce0d364b46c812351ebef4b6462a1f06b771697b8c819",
>   "info": {
>     "payment_hash": "45424962cd067fef440ce0d364b46c812351ebef4b6462a1f06b771697b8c819",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347",
>     "channel_scid": "149533581443073",
>     "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1785741696",
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
>   "swap_string": "rgb-swap:v2:eyJwYXltZW50X2hhc2hfaGV4IjoiOGI4MWZjOGQwMTgyMTYyZWJkNWMzZDM2NTA1ZGFjNTgzMWRhN2JlMTkyM2EwMDcxMDQxMDlmNGE5ZDgwY2IwMiIsInJnYl9wYXRoIjpbeyJub2RlX2lkX2hleCI6IjAzOGYwYzU5ZmQ0MzAwMWFmZTIxNTY0NThhYzMwZDA1ZGI5YzhiYTIyZjY2ZDljMGNmMmIwMGQxYjZmY2U1NTM0NyIsImNoYW5uZWxfc2NpZCI6MTQ5NTMzNTgxNDQzMDczfV0sImJ0Y19wYXRoIjpbeyJub2RlX2lkX2hleCI6IjAzODMwMmMyOGM4NTExZDdkNDExMzZhNWQxOWNhN2Y0NGNiNWQzZTY0NWNhNWJiNWQ2OTAyMWIwMjM4YmIyMzIzZCIsImNoYW5uZWxfc2NpZCI6MTQ5NTMzNTgxNDQzMDczfV0sImNvbnRyYWN0X2lkIjoiY29udHJhY3Q6WEtENjVuekItWUVnbVNyZS04Wl9PMW4yLXZvUkxBTVAtZ0Q1eWpnOS1SZF9vOHBZIiwiYXNzZXRfYW1vdW50IjoxLCJidGNfYW1vdW50X21zYXQiOjEwMDAwMDAsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImV4cGlyeV9zZWNzIjozNjAwLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6MTc4NTc0MTY5Nn0",
>   "payment_hash": "8b81fc8d0182162ebd5c3d36505dac5831da7be1923a007104109f4a9d80cb02",
>   "info": {
>     "payment_hash": "8b81fc8d0182162ebd5c3d36505dac5831da7be1923a007104109f4a9d80cb02",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347",
>     "channel_scid": "149533581443073",
>     "contract_id": "contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1785741696",
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
>   "invoice": "lnbcrt100n1p48qsvpdq8v3jk6mcnp4qw8sck0agvqp4l3p2ezc4scdqhdeezaz9andnsx09vqdrdhuu4f5wpp5afthqdlcpp683676nvj60n6pr8xzp9445vauax3ww8gfsz8m9gjqsp5d293f5x5nv090cxfax4l5a3209gddwwac4m6h9meahaunpj4cyys9qyysgqcqzp2xqrrssrzjqwps9s5vs5ga04q3x6jar89873xtt5lxgh99hdwkjqsmqgutkger6qqqqyqqgqgqq5qqqqlgqqqqqqqqfq3syq4lsjjxwdmk7dfw45t7kqn2t36nnqjxfv3e42t3v4atw6g0hx6ks9nm8ty99kk5f8t456gvenggs86ef3pgrfehc9cyah93n3xhcp64v4s7"
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
> | payment_hash | ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347 |
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
>   "payment_id": "ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24",
>   "preimage": "c080c0561d09ca31f371880f2f5ed400949367665aa9d896bca5082a1ecb6dc9",
>   "amount_sats": "10",
>   "destination": "038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24
>
> ```

**Run:**

```bash
$ rgbldk pay get ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"ea577037f8087478ebda9b25a7cf4119cc2096b5a33bce9a2e71d09808fb2a24","preimage":"c080c0561d09ca31f371880f2f5ed400949367665aa9d896bca5082a1ecb6dc9","secret":"6a8b14d0d49b1e57e0c9e9abfa762a7950d6b9ddc577ab9779edfbc98655c109"} |
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
>   "invoice": "lnbcrt130n1p48qsvzdq0v3jk6medwdjkueqnp4qw8sck0agvqp4l3p2ezc4scdqhdeezaz9andnsx09vqdrdhuu4f5wpp55dsvx3dfm2ychpuj8a635776vt70rdwmcg5eqppe8s5k8fq79q9qsp50l8ulhpvynkymdg3gpplqjdz8rmqtl45peea9quvcy2z95javp7q9qyysgqcqzp2xqrrssrzjqwps9s5vs5ga04q3x6jar89873xtt5lxgh99hdwkjqsmqgutkger6qqqqyqqgqgqq5qqqqlgqqqqqqqqfqsgkk4vtem827n26p83tjaynjy7p2mfwllqfgnmj2g7awdp6tpc04jkzztte7puy6wzvscrtm35l9cskxvs4zkkc2etf7fkk2t5ntkpgqdj6mj4"
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
>   "payment_id": "a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a
>
> ```

**Run:**

```bash
$ rgbldk pay get a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"a360c345a9da898b87923f751a7bda62fcf1b5dbc2299004393c2963a41e280a","preimage":"053778e95c96c0ee35f2a6e61b7b8a11ac2c4ed4d180fc49a709332e039e4ea5","secret":"7fcfcfdc2c24ec4db5114043f049a238f605feb40e73d2838cc11422d25d607c"} |
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
>   "invoice": "lnbcrt1p48qsvydqdv3jk6medweshynp4qw8sck0agvqp4l3p2ezc4scdqhdeezaz9andnsx09vqdrdhuu4f5wpp5jdnx6eewxwzuka4m5f3nt73umphh0906quy5phvj267w9wlwfs6ssp5v8qcpazsc89jea8832ze3aupesk4n3gy44sm3p7pg0zfnjqjrmrq9qyysgqcqzp2xqrrssrzjqwps9s5vs5ga04q3x6jar89873xtt5lxgh99hdwkjqsmqgutkger6qqqqyqqgqgqq5qqqqlgqqqqqqqqfqr55d34vvzkxwq3xda3n9589any97m3q6v8qc5uavwzaa5jatsp6rrsz8f7lh9cn4e3a75qv7uas0wmdjlm8955x3g7mkze63l3luwwgp4fvvqt"
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
>   "payment_id": "93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35
>
> ```

**Run:**

```bash
$ rgbldk pay get 93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"93666d672e3385cb76bba26335fa3cd86f7795fa070940dd9256bce2bbee4c35","preimage":"c7357cbf8d0d2ca074b4f1c9a8de3fb27623d43470219d255b4c951870c52d45","secret":"61c180f450c1cb2cf4e78a8598f781cc2d59c504ad61b887c143c499c8121ec6"} |
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
>   "invoice": "lnbcrt140n1p48qsv9dqcv3jk6meddphkcepdvdkxz6tdnp4qw8sck0agvqp4l3p2ezc4scdqhdeezaz9andnsx09vqdrdhuu4f5wpp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp5ahc5p5yxg2awtq8qmxemqhsvl9yw62u3ejutdwwkd6kxrutl5r0q9qyysgqcqzp2xqrrssrzjqwps9s5vs5ga04q3x6jar89873xtt5lxgh99hdwkjqsmqgutkger6qqqqyqqgqgqq5qqqqlgqqqqqqqqfq4cju7tk89v572zn243ehc49uv6mzc9elkeda44csnyw6ek94cf48m29q6zlars2xelmu9a4j47rdge4fqcrcvsvt3rx6f2tq563yqqcqvvjxcs"
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
> | destination  | 038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347 |
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
> [X] Details
>   [OK] Payment Id Valid
>   [X] Timed Out: Waited 60s
>       hint: Try again, or abandon the payment if it is awaiting an invoice (BOLT12).
> payment timed out
>
> ```

**Run:**

```bash
$ rgbldk pay get 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c

```

**Result:**

> ```text
> +-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                           |
> +===================================================================================================================================================================================================+
> | id              | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c                                                                                                                |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c                                                                                                                |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                        |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | … Pending                                                                                                                                                                       |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                          |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                           |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":null,"secret":"edf140d08642bae580e0d9b3b05e0cf948ed2b91ccb8b6b9d66eac61f17fa0de"} |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)   | 14,000 msat                                                                                                                                                                     |
> |-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat) | -                                                                                                                                                                               |
> +-----------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt150n1p48qswzdqhv3jk6meddphkcepdveskjmqnp4qw8sck0agvqp4l3p2ezc4scdqhdeezaz9andnsx09vqdrdhuu4f5wpp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp58xc759d97rup27ye5wvvyvx8mcdc5fx3pf97gwt87wjwervquelq9qyysgqcqzp2xqrrssrzjqwps9s5vs5ga04q3x6jar89873xtt5lxgh99hdwkjqsmqgutkger6qqqqyqqgqgqq5qqqqlgqqqqqqqqfqyx8jcluy0mhqeaukj8k0l3jurjfzup9plwu0d48qtf8lfxpy5z99069rd3x2hqn2ulegtjcez39vuzllr0ng5wke5xfhm8yq6cgtyhcqxc0lnp"
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
> | kind_details    | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"39b1ea15a5f0f8157899a398c230c7de1b8a24d10a4be43967f3a4ec8d80e67e"} |
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
>   "invoice": "lnbcrt120n1p48qsw9dq0v3jk6medvfskx6cnp4qwps9s5vs5ga04q3x6jar89873xtt5lxgh99hdwkjqsmqgutkger6pp5ex7anf9e53ddnxxe75n369y3zx3u8t0uuhzrmk7rhqhncrqtxlyqsp5tyfsk4tkppkv07xncn058xk98074ugejrg88cuy2q393u8msr45q9qyysgqcqzp2xqrrssrzjqw8sck0agvqp4l3p2ezc4scdqhdeezaz9andnsx09vqdrdhuu4f5wqqqqyqqqlsqqgqqqqlgqqqqqqqqfqwja9lh2pcg42cgzmuv6qrezng9zyrqc9ehnkq8gceydy386s952j03my9sj7m7zlhww35q43xfsdq8jxs38w5sgwuuvrz24cqkjj4fsqmkqlym"
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
>   "payment_id": "c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8",
>   "preimage": "775ca0701092b8a2e36333b19c22b166c267bd7c4ae480f0c48a056653d4c3ad",
>   "amount_sats": "12",
>   "destination": "038302c28c8511d7d41136a5d19ca7f44cb5d3e645ca5bb5d69021b0238bb2323d",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8
>
> ```

**Run:**

```bash
$ rgbldk pay get c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"c9bdd9a4b9a45ad998d9f5271d149111a3c3adfce5c43ddbc3b82f3c0c0b37c8","preimage":"775ca0701092b8a2e36333b19c22b166c267bd7c4ae480f0c48a056653d4c3ad","secret":"59130b5576086cc7f8d3c4df439ac53bfd5e23321a0e7c708a044b1e1f701d68"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx5uz06cgvyqgqqpkqqqqpqqqsy6yvv4z4pq49hrnmne5a2w5aeft9kyaj835auqtndtsm7dwfdy6uqyplrs9h7t7ay86xdwn9me69a0g4n0el8c8fakulz0500jvs554g0jcqwnf447s2h8eyffqw25fk9cpgmsudlwjkummrq9c20336zpa6sr2kdglcyptf3z8pcluszmkmttac6a4jg6ate29dgpy2ffy09y07v9dhu4pus5q85v7muac886mq37stva29zquju5z8dz4xdjv6ndjhk0l6tzgu0pfrn8hvpy8r3vug2zv494v9ym6pvggzlg8fltf0zmym2g3wm9cc5ff738dqx5yrk9lmky3dftyjquwptq9q"
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
> | offer_id                  | c2eb8333d368795fbb0edc1d74e51252b0f469273bf95fbe51fd35014a93b931   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 02fa0e9fad2f16c9b5222ed9718a253e89da035083b17fbb122d4ac92071c1580a |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1785745366                                                         |
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
>   "payment_id": "58a54721b950c2302ad869367db50370082ae88f5362d717f72d6381fc30acde"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 58a54721b950c2302ad869367db50370082ae88f5362d717f72d6381fc30acde --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 58a54721b950c2302ad869367db50370082ae88f5362d717f72d6381fc30acde
>
> ```

**Run:**

```bash
$ rgbldk pay get 58a54721b950c2302ad869367db50370082ae88f5362d717f72d6381fc30acde

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | 58a54721b950c2302ad869367db50370082ae88f5362d717f72d6381fc30acde                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 1cb92ede2d9750ebc412c80ff5cc3d0c23b1e0e13f73b51a28db02d3a8a57b3f                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                               |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"c2eb8333d368795fbb0edc1d74e51252b0f469273bf95fbe51fd35014a93b931","payer_note":null,"payment_hash":"1cb92ede2d9750ebc412c80ff5cc3d0c23b1e0e13f73b51a28db02d3a8a57b3f","quantity":null} |
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
>   "refund": "lnr1qqsdwenslw7j7t8rqxuwcnccgmu256gtwxnez8z026w82crwav9ylqs2qq8qg6nsflv9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxw37dfl223zl69jmgpr9slfpqlpcq3jth7uev80kdc5xq37s87anty9hyetxw4hxgttyv4kk7kkzqqqqqmqqqqqsqqgzx6vcdwwcvggg3r2w9pkepwm65tyt9ep7vqpkg00txaz307905s6szql7j54y8gs06kf3wguw9u876yjleugr76d0wmkz2gfsp86ymfscugq8gg5np22t2m3w6djhhwu6557qltt9c2vx4traj7gylz6q55td9lxtyn3wcgvqquqfl98n2vhma8n62cav7nz4vtlhusdagspu3cwa0j0dxhhudwzmrcpyfmgegdtwm272nnwrham85u6m9c83fwan227eqjzwaag08c8v9fzj65fr5kw4qyqjxgf7zeq",
>   "payment_id": "691de8231c3050b9b987cad710a58b063233c998dfca2bb62ecb5da9819b4c91"
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
> | absolute_expiry_unix_secs | 1785745368                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 033a3e6a7ea5445fd165b4046587d2107c380464bbfb9961df66e286047d03fbb3 |
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
> lni1qqsdwenslw7j7t8rqxuwcnccgmu256gtwxnez8z026w82crwav9ylqs2qq8qg6nsflv9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxw37dfl223zl69jmgpr9slfpqlpcq3jth7uev80kdc5xq37s87anty9hyetxw4hxgttyv4kk7kkzqqqqqmqqqqqsqqgzx6vcdwwcvggg3r2w9pkepwm65tyt9ep7vqpkg00txaz307905s6szql7j54y8gs06kf3wguw9u876yjleugr76d0wmkz2gfsp86ymfscugq8gg5np22t2m3w6djhhwu6557qltt9c2vx4traj7gylz6q55td9lxtyn3wcgvqquqfl98n2vhma8n62cav7nz4vtlhusdagspu3cwa0j0dxhhudwzmrcpyfmgegdtwm272nnwrham85u6m9c83fwan227eqjzwaag08c8v9fzj65fr5kw4qyqjxgf7ze9qacpc7rzel4psqxh7y9tytzkrp5zah8yt5ghkdkwqeu4sp5dklnj4x3crdvxv4zcqzpcwycx0x70pdetpayt0hjpkarptnyc6ylmqt8pz5uaqzq50ulsdf3svkzae7ykn5qyhhw0kvzxf2qdjkm3y5v0k9fqp2taf25qg3ul4mpwnzscf8nnpqwz6q92duap7u6zzj9caq9eqq5kgn26f3s75e3yw9mtu4ft86acx5f48hk443egyhx47l65qeuqy2xy7q4vaq47ca36uv462vgvxmy6qwxvj0lmkncmn7ap0lme4nkfnfdk6xscz6nxm2lfddhtlae7rg62a04r254kfutqa9sf7rffsdd6rye6t3m9kymyr430nfywe46dzrsqqqqqqqqqqqqqq9gqqqqqqqqqqqqgayjedltzjqqqqqq9yq348qswf4qszyu2ruv289pfvj7xm0egktfxj3qws4wv82j56eaqkw4dp9l5lsr92qggwrtsrqgqqpvppq00zu6ka2wvsqkhdl5ydp5hhxmlvfc7xc70nj09kz9xm0wma8pfs9uzqlqel9t47jqhyrsftw7rh9ps5d253c3hhcux0m33jr9z738kv3e89y7hwt6dz8lgun3epxr5l0wphrw9ryccxurn55lwt0sqyfgcpmxg
> payment_id: 227143e31472852c978db7e5165a4d2881d0ab98754a9acf416755a12fe9f80c
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
$ rgbldk pay wait 691de8231c3050b9b987cad710a58b063233c998dfca2bb62ecb5da9819b4c91 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 691de8231c3050b9b987cad710a58b063233c998dfca2bb62ecb5da9819b4c91
>
> ```

**Run:**

```bash
$ rgbldk pay get 691de8231c3050b9b987cad710a58b063233c998dfca2bb62ecb5da9819b4c91

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | 691de8231c3050b9b987cad710a58b063233c998dfca2bb62ecb5da9819b4c91                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 227143e31472852c978db7e5165a4d2881d0ab98754a9acf416755a12fe9f80c                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                          |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"227143e31472852c978db7e5165a4d2881d0ab98754a9acf416755a12fe9f80c","quantity":null} |
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
>   "refund": "lnr1qqsryajsl2md64603gg4uceju82g8ysg42pwf2fty4kg2z6hdgahpag2qq8qg6nsfld9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqss8lmyty2fn9huul46e4qg3n3lyckaqztq6esdmremwnxyzxqzmycytyfhyetxw4hxgttpvfskuer0dckkgetddadvyqqqqpkqqqqpqqqs99dz2ukmx62sg0dxqn8upjast03e2k6nyvjd9gthkc909w98pc93qyp0yg76eq5g46tay7se45mg3757u3atdltsuh4ytge7g9dw6kqqrwsqwjwhup242d29t8mp7pcxccvel4caczexc69w9nqdf3wcd6u5v8qneeue22zns9vkneu549ldnh27ahvtqfsy5f5rfjg7y8xzpd30gctdllth3tndnlz9f3wyl6cp6wkvmp038jlpjr4vzn4s8zrkysl9u8trtqnzh8f0aw0zmsehlaauuqrdkz86sw6q",
>   "payment_id": "9f9124735b2da4439f19859bb416282eb12db9438e97ee5d458038585c619bc8"
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
> | 93666d67...bbee4c35 | 93666d67...bbee4c35 | Succeeded | Bolt11       | Outbound | 11,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 61d29c6c...f4cff329 | -                   | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 9f912473...5c619bc8 | -                   | Failed    | Bolt12Refund | Outbound | 1,111           | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | a360c345...a41e280a | a360c345...a41e280a | Succeeded | Bolt11       | Outbound | 13,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 43271eae...c049826f | -                   | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | c9bdd9a4...0c0b37c8 | c9bdd9a4...0c0b37c8 | Succeeded | Bolt11       | Inbound  | 12,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 58a54721...fc30acde | 1cb92ede...a8a57b3f | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 425ed4e4...6e55384c | 425ed4e4...6e55384c | Pending   | Bolt11       | Outbound | 14,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | ea577037...08fb2a24 | ea577037...08fb2a24 | Succeeded | Bolt11       | Outbound | 10,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4113d54b...cc0b3a24 | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 691de823...819b4c91 | 227143e3...2fe9f80c | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          | false       |
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
> | 4a16cb3b483b68e6ccdb7d79fb740efa41ec6b0f13e28dae77b1e03c443f23fc | 1f4723faec112b969df9eacb4260756e | yes    | 88,294,124   | 10,031     | yes     | yes     | yes        | 330,000       | 88,294,124     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt30u1p48qswtdq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp5p5ac5terh5fpznufrkypgtgjdczde78w73nnsazrfkkpvzqvdtas9qrsgqxqrrsscqpjlp5tjs04enuc9sysfj2k77x0ca4na47s39spslqp7w28q75thag72tq7qpzz7c0mke0cf3p7rlfq80lstqpk8647nvtz963gd7cy5p7vsgnustxwqg64vfes74xefssp3v3rs3sd0k24vvatzj22lden0rj3hnlh8cpng2zkd",
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
> | Destination           | 038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY          |
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
>   "invoice": "lnbcrt50u1p48qswtdqjwfnkygrvdcsxgetddupp5jv9smv9pk2pkz50kpt988vqsn595e5tqzuuqxssd59qa422jd35qsp5wtk972hrxcc9z73xyt97um80ztvvha5gu2a0764ejf8vavln7r2q9qrsgqxqrrsscqpjlp5tjs04enuc9sysfj2k77x0ca4na47s39spslqp7w28q75thag72tq7qp998hlrc2vdhjt6ur78sfa8kh976t46rldj9398k5j8r0qxl9nu8v5ay3s3qh89dhta0ut5fjyll3ndzwqws3svhv0y3dydyl56uem57sqecx53u",
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
> | Payment hash          | 930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 038f0c59fd43001afe2156458ac30d05db9c8ba22f66d9c0cf2b00d1b6fce55347 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY          |
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
>   "payment_id": "930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68
>
> ```

**Run:**

```bash
$ rgbldk pay get 930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"930b0db0a1b2836151f60aca73b0109d0b4cd160173803420da141daa9526c68","preimage":"73ca6493c850a47d24965282b63783f13708e77b9bebf42a9ee1116738b43f99","rgb":{"asset_amount":"5","contract_id":"contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY","direction":"Outbound","is_swap":false},"secret":"72ec5f2ae33630517a2622cbee6cef12d8cbf688e2baff6ab9924eceb3f3f0d4"} |
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
> | Available inbound (msat)        | 14,031,876      |
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
> | 4a16cb3b483b68e6ccdb7d79fb740efa41ec6b0f13e28dae77b1e03c443f23fc | 16d8c5522048305ecff6fcf47df3f829 | yes    | 14,031,876   | 84,968     | yes     | yes     | yes        | 330,000       | 14,031,876     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt40u1p48qswddq6wfnkygrvdcsxgetddusxyctrdvpp54n8v3hp2k9qwwmef5cvz6d6yftmqtszvwx0kpecfwyqw294fjv5ssp5ghp0mknznw6ecav0t9lyhhardrr3f85gg9sykkvwukqwq38ay58s9qrsgqxqrrsscqpjlp5tjs04enuc9sysfj2k77x0ca4na47s39spslqp7w28q75thag72tq7qpra4guqrfvt7hwf2dx079l4kt44vkxrsknqxv7n9l2hfe4vlt0m8hryxkmp04jh6w79czlhmv8t4s8vvqx5jjnyrjrqpmcv8s2ptadg3qqgqucv6",
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
> | Payment hash          | accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 038302c28c8511d7d41136a5d19ca7f44cb5d3e645ca5bb5d69021b0238bb2323d |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY          |
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
>   "payment_id": "accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329
>
> ```

**Run:**

```bash
$ rgbldk pay get accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"accec8dc2ab140e76f29a6182d37444af605c04c719f60e7097100e516a99329","preimage":"a7e12c1cfb2b840c5ceabd96236321f66866d74c8f364ac23e9b120619436403","rgb":{"asset_amount":"3","contract_id":"contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY","direction":"Outbound","is_swap":false},"secret":"45c2fdda629bb59c758f597e4bdfa368c7149e8841604b598ee580e044fd250f"} |
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
> | 16d8c552...7df3f829 | 038f0c59...fce55347 | 149533581443073 | 1099578802181 | 1099519885314 | 100,000         | true  | true   | contract...-Rd_o8pY | 6         | 4          |
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
> | 1f4723fa...4260756e | 038302c2...8bb2323d | 149533581443073 | 1099519885314 | 1099578802181 | 100,000         | true  | true   | contract...-Rd_o8pY | 4         | 6          |
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
>   "payment_id": "ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e
>
> ```

**Run:**

```bash
$ rgbldk pay get ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"ae33a0444d608cf2f99bf21328725c43f3b967bf33b65915cecb3c0f8de2988e","preimage":"06b8ccddcce7191b4facfb79e90c6ee1f10503080ef610c59512228e0c09a88a"} |
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
>   "payment_id": "91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798 --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"91bc245431b6d50213ca131b5dad4b0c046cf7e966adfee78d9cd13260609798","preimage":"295bad3cd1f631096bdfe5ce6e7c3ca524960b6fa2bb4cda53b2bf2a799b8435"} |
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
> ChannelPending funding_txo=2fbac740037a7bf9d235a06dd36bdc5fa7bc63de6e4dd9883037380b93c8562b:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=2fbac740037a7bf9d235a06dd36bdc5fa7bc63de6e4dd9883037380b93c8562b:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=2454ad240529f012e87e36a21e1b0175
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
> ChannelReady user_channel_id=bbd5598910904d025c3f20d61631b5ee
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

Demonstrate graceful close vs force-close, and how `channel closing` tracks both paths. Force-close is destructive and requires `--yes` for non-interactive safety.


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
$ rgbldk channel closing

```

**Result:**

> ```text
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
> | User Channel ID     | Channel ID          | Counterparty        | Status      | Source | Closing Txid | BTC (sats) | RGB Contract        | RGB Local | RGB Sweep |
> +==================================================================================================================================================================+
> | 16d8c552...7df3f829 | 4a16cb3b...443f23fc | 038f0c59...fce55347 | negotiating | coop   | -            | 86,348     | contract...-Rd_o8pY | 6         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 channel closing

```

**Result:**

> ```text
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
> | User Channel ID     | Channel ID          | Counterparty        | Status      | Source | Closing Txid | BTC (sats) | RGB Contract        | RGB Local | RGB Sweep |
> +==================================================================================================================================================================+
> | 1f4723fa...4260756e | 4a16cb3b...443f23fc | 038302c2...8bb2323d | negotiating | coop   | -            | 11,033     | contract...-Rd_o8pY | 4         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "48291d33c69d8022a7ee1c4576eedcb7989a5ec363a8db894d4b4fe482eb4b89",
>   "136381f15204dfb84cae69e884ce43d64845f9211f14de1092843d95d72d8995",
>   "429541ca2467b7462e2a0696ac5e85b240839f1ff01981e5157dcb4f7077df51",
>   "3d7027f412d2f7e90c7b6ac1b233cb9440aae9ef5018c06cc03dd1cdd248f955",
>   "2305a3e30d5ccec249aa0948e80b1d742ccd2d2fe28933525918356af6f8de59",
>   "1594f45309562b4931726c4aaefe9c74c713c60b384110a649e37ab76abf173e"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "370c013e8794cfeb109e6f9ffbd5b2229de525246c1e90b5553c91f0fdbacdf9"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "6a0b81d5454946aacf6ec9e402e158deec546b4cc4d9b8976b164f2f532727b7"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "2c4cf72dfaa0f94791b499bfac78b2359b713aae9be872037f88d4cc2df31551"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "3a4b7728d65cced27bcbb7241534a18964b689216ede6276f994e99c54f84259"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "6563d20568911e3d336c49cb4125c315b4629a84ee95191806ef67af16ec12e1"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "35b3266b77cf1d5f538515e6c631f150dadb58aec87538d51ed29418d1994624"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "0f11e7df23607732800b640ffc8008de8e0b86aefba369517a121aee26c07512"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "67ced61e026c106425f6322dac022f3318b41fa308f670462ee0ab1cefd5150e"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "718ca779339c5c3ab71680e6867a4e715d7ac1140a7ffdcba32c86501a30a5f2"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1363b42a4fbfef20f7bc5f352c079985019dcfced1519462a949bd322e74cdc2"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "6cedd0a09f5ce8850a6b4183cc013bbaebc3b12c3285ed3717b1d5f6c2b2a88f"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1782c5ae0712155ca0fef5a3ae844e2f4697fb907f1a800456c15f7f62082bba"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "73d1af16aa8c6d3ade238723f2257a3147b302e83d3a41fb8b00390a0fbcf0ec"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "49d792f6648fae6b485cbba351b76a860229b44c0957ffffaaf91b2e3353317b"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "6cafea1108a16a59ba3263d3ed6aba4e7fa298e66ab88450fd8f4ac79ff435b9"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "3ad5aad7f90da576ed82373098cd3302d4d74465cac711a26d24c6a0074225a3"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "0c30aa14ba9308a0ef88385351284c8c0114baf7620ec2ff391bf78e8e977b86"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "383cd918afeb1a6fe1376cf9991aa241f8105ff27bc1e3b02ededb75c1bd51de"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "5d1779e6349dd4f80fb40054535cd1448f5161bae33ba96a1ec4d35e4532f450"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "4cf5615ba8af91d46716e185a185567d6cdf798703623d43237e6ae0c01215e0"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1e9f7e68ad5714c1d1763f7eaa6e9e50866ed42e8a4644ca4233f295b90f2e1c"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "0fa1bb055fad8899994b9d2fd259b99ec35a00fe88afc58335422ad317345f40"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1999868878fcd85c0e6ffc22a3c46c06522bd29218825d39ff98203da7987c25"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "189121db1dbc27c28be206e3ef10d9cac9c134937c85b57b77e84ed0dca3c503"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "2f0841ac7d4737da42fa5e32203e4132c3d44a2ee7faeaa6ac4b923b969dee35"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "28bbe252e5d39e497f51278d7f75939cfb989e19697ca37b7209a2ec214a51cf"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "570bb106c915773d524390761dde91b79bd40b865c368e7252226ac507947a05"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "152668d47058203cfdc6a35a33ff9e26a4bcb6c38c512745dda9e205fa5c40e0"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "2a99b5fabacb92e518e8cdef68e94e8ee321aa3d62e95cc8ed85a67e9e47aa6d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "744515a3a331fecbc44afbe63abc99810e9a384683dd4234920ff002df5f83bf"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "7b0b8a300a1ecec954741bc7a57b32d5668455681c7c61a9f8725fa3dcd7fde4"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "6ef7d13517112d5fdcefb0c1d6ba93cc199ddfc46479ae7efebe72ccdf713891"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "631be09aa77eac55e861f4295c7dcc603590180c87d9ebcc77fb08a654ab5ab8"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "1541d6a89c3ed1bd723d8ac57b9a11500e2d87140e2729f1f34c559e76c5e24f"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "2f530a6e02cead9f216d20fbf12b6d5b75206c3380a491ba24e2a9279459774d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "18efca0fd0c767262774472e8a326d703abd29ee89454284116f75db03fe9ca4"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "793faa0c2063754da6515e15cdf031f701c33ee29621fb4f57afe35475a06742"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "68bfa3f9fda74071745e8ee14c8e3926f2dc84b3f0c30cdf3cd4620d76cfb5ec"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "441ea92d7b6f6dc9fb791b937123db6b8737b3caef7cf38f7af058f0e5991ede"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "64e066ab6ea0010c9534fa1a31d06a046e329fc4e95e8ecd69251ae83d06d199"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "226d8168fa2704e65f50e06767af69952fbba5c4c156f58007726340cfa43aeb"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "31b7f76b21f290018e81894b11bff9e4800cabd4d8d4100d897e707d366765d5"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "099d6953161614e90595271e5b9f4f8d5ce38dfc5b0508c51dff05c8aaf0fd2a"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "563c1a163db09fc161810d731c87a6ef4ce16990a4281d11b6f5f834aee4d5b5"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "27ed579f86943faa3de13f60cdd0a7a620c3849c15208e36389f70665125f0e7"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "3a9a619e992dd79a94e47938b13cf8fae2fd079095509f6785f2b431206418f3"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "03d8d01a0b94fe6637b49d0966af4ecd31e76ca78ca26d84ea452431c80f4e2e"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "20b8d2dd0ac56a1d292a58c6881eb81339dbbdb025c8cce37848432db4f5223a"
> ]
>
> ```

**Run:**

```bash
$ rgbldk channel closing

```

**Result:**

> ```text
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
> | User Channel ID     | Channel ID          | Counterparty        | Status      | Source | Closing Txid | BTC (sats) | RGB Contract        | RGB Local | RGB Sweep |
> +==================================================================================================================================================================+
> | 16d8c552...7df3f829 | 4a16cb3b...443f23fc | 038f0c59...fce55347 | negotiating | coop   | -            | 86,348     | contract...-Rd_o8pY | 6         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 channel closing

```

**Result:**

> ```text
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
> | User Channel ID     | Channel ID          | Counterparty        | Status      | Source | Closing Txid | BTC (sats) | RGB Contract        | RGB Local | RGB Sweep |
> +==================================================================================================================================================================+
> | 1f4723fa...4260756e | 4a16cb3b...443f23fc | 038302c2...8bb2323d | negotiating | coop   | -            | 11,033     | contract...-Rd_o8pY | 4         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
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
> +--------------------------+-------------+
> | Asset                    | Balance     |
> +========================================+
> | BTC On-chain (total)     |  1.0003 BTC |
> |--------------------------+-------------|
> | BTC On-chain (spendable) | 1.00005 BTC |
> |--------------------------+-------------|
> | BTC Anchor reserve       | 25,000 sats |
> |--------------------------+-------------|
> | BTC Lightning (total)    | 86,348 sats |
> +--------------------------+-------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |     7 |         0 |        0 |     7 |
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+--------+
> | RGB L2 Channel                                                   | Contract                                                  | Local | Remote |
> +===============================================================================================================================================+
> | 4a16cb3b483b68e6ccdb7d79fb740efa41ec6b0f13e28dae77b1e03c443f23fc | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |     6 |      4 |
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+--------+
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
> | BTC On-chain (total)     | 1.01396537 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.01371537 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |    25,000 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    11,033 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |    83 |         0 |        0 |    83 |
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+--------+
> | RGB L2 Channel                                                   | Contract                                                  | Local | Remote |
> +===============================================================================================================================================+
> | 4a16cb3b483b68e6ccdb7d79fb740efa41ec6b0f13e28dae77b1e03c443f23fc | contract:XKD65nzB-YEgmSre-8Z_O1n2-voRLAMP-gD5yjg9-Rd_o8pY |     4 |      6 |
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+--------+
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
>   "user_channel_id": "bd0ac053e02e58ad1fbb906ab9bbf219"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q355m9kzvz403qypkf4m3rrn5dezgweczjcp2cc

```

**Result:**

> ```text
> [
>   "29bf60c4d5bb72619d5b0033c2451b9195e954cbb6142f9eb7b0f6ca5d38dfe4",
>   "494bac35fec400084fd77bedda80f37a165ca7fe6f816a8c6be401df4320de4d",
>   "482ec9448e2886a94de30a33a809048dc179c61cc859206aaf5871e6e5aff3f7",
>   "1f696b235827c9e3db1b1d92dd15d675977418fab89fd4c029232417945ef9a1",
>   "5b09b05188d0cd45b632eab2fea4f767357d3783b6c2fa920a5bd2af82246706",
>   "5a4991cb2dd76eb30a470965c76637037c6a3d1db239c39e6fffeaf24e6f82b9"
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

Force-close also appears in `channel closing`. CSV delays may keep the entry around longer than a cooperative close; the example only snapshots the post-force-close view rather than waiting for full settlement.


**Run:**

```bash
$ rgbldk channel closing

```

**Result:**

> ```text
> +---------------------+---------------------+---------------------+--------------+---------+--------------+------------+---------------------+-----------+-----------+
> | User Channel ID     | Channel ID          | Counterparty        | Status       | Source  | Closing Txid | BTC (sats) | RGB Contract        | RGB Local | RGB Sweep |
> +====================================================================================================================================================================+
> | 16d8c552...7df3f829 | 4a16cb3b...443f23fc | 038f0c59...fce55347 | negotiating  | coop    | -            | 86,348     | contract...-Rd_o8pY | 6         | -         |
> |---------------------+---------------------+---------------------+--------------+---------+--------------+------------+---------------------+-----------+-----------|
> | -                   | 637d276a...5f2f554d | 038f0c59...fce55347 | broadcasting | unknown | -            | 46,654     | -                   | -         | -         |
> +---------------------+---------------------+---------------------+--------------+---------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 channel closing

```

**Result:**

> ```text
> +---------------------+---------------------+---------------------+--------------+---------+--------------+------------+---------------------+-----------+-----------+
> | User Channel ID     | Channel ID          | Counterparty        | Status       | Source  | Closing Txid | BTC (sats) | RGB Contract        | RGB Local | RGB Sweep |
> +====================================================================================================================================================================+
> | 1f4723fa...4260756e | 4a16cb3b...443f23fc | 038302c2...8bb2323d | negotiating  | coop    | -            | 11,033     | contract...-Rd_o8pY | 4         | -         |
> |---------------------+---------------------+---------------------+--------------+---------+--------------+------------+---------------------+-----------+-----------|
> | -                   | 637d276a...5f2f554d | 038302c2...8bb2323d | broadcasting | unknown | -            | 1,000      | -                   | -         | -         |
> +---------------------+---------------------+---------------------+--------------+---------+--------------+------------+---------------------+-----------+-----------+
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
