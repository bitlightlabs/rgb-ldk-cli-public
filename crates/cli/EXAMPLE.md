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
$ (cd ../rgb-ldk-node && cargo build -p rgbldkd --bin rgbldkd)

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
>   "node_id": "0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda"
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
>   "node_id": "0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e"
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
> {"keystore_path":"/tmp/rgbldk-cli-example-_xrd8gw_/keystore-init-demo/keystore","mnemonic":"nominee happy rhythm sentence divide zero few foil slot diagram before order canal erosion pink dinner wrist paper balcony option peasant purchase chronic grab","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-_xrd8gw_/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-_xrd8gw_/keystore-migrate-demo/keys_seed.bak.20261116-081139","ok":true}
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
> | 0318a96c...cbff386e | 127.0.0.1:9736 | true      | true      |
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
>   "address": "bcrt1q4jp06a5mtkr05g50whpjx0xfjvz3vr6pwu8lru"
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
>   "address": "bcrt1qjrgdkvsqaqvt3dzvu2kn29nxe88pvthnxfwmdt"
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
> bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1q4jp06a5mtkr05g50whpjx0xfjvz3vr6pwu8lru 1

```

**Result:**

> ```text
> 230b4fae55c07365aa21f7060a039a6ff60832fe0f9410b01d9a2887a73b72c0
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qjrgdkvsqaqvt3dzvu2kn29nxe88pvthnxfwmdt 1

```

**Result:**

> ```text
> 1297258539c388c9b9685ed8e18abe2a0ce7c117656e9669bd86133bbf3d93db
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "3ab9a4f3eaa9ed223997734cb03d739c425feb2fc502e7f9809d939f0ebe0196",
>   "1f753f61f8728ef962ce31eae9266580b5c5b97d5cd237d2ca60bd0e8114be2e",
>   "76987c12a418ccb5af1ed624be2f04f1a020b8a43f3dd4256b5eab1aea3419fa",
>   "3ed2cae7c14f67b42cb7bc6da7478a21523348d12634549c508b119969e3d878",
>   "4e25432536e0e20023fe2bbdc7927a9ba6bf903d58da26294f492e7aab5f08e4",
>   "7540ca08af4fe8f9d4b94d2c37a3543204b8e236fcdec6f2f80fe415524d0d04"
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
> | 230b4fae55c07365aa21f7060a039a6ff60832fe0f9410b01d9a2887a73b72c0:0 |  100,000,000 | Confirmed |    102 | Available    | -        |            - |
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
$ rgbldk channel open --node-id <node_id_a> --addr <node_a_p2p> --amount-sats 120000 --push-msat 30000000 --announce

```

**Result:**

> ```text
> {
>   "user_channel_id": "95fb0e1599a9a9c7498264c41c3a622c"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "2dccc6caede499186ac45d9586611132d7b099daf2fd51547816f130c48900f4",
>   "378f118fbb7e011ef6416d0e5d854b957e4db7cc66f3cc2ada7d6070763fe7a7",
>   "52c20385a04fb3b1049f76f80ff00dadcf37144a51a949427a495e5decdda184",
>   "40e80d7320e7fbe3e22878aab6de65c2e9dc53d3e11be8420e04068a8b4956b5",
>   "4101c4ae551af3e4fe0928f087067199748f44df1181454baa910f09bcce0e7b",
>   "2741d2c1c23f4c46405b985be6669ef7cb67349a84b95d2125fb5bcabe795ec1"
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
> | 95fb0e15...1c3a622c | 0383300a...0b03dcda | 118747255865344 | 1099639029764 | 1099551997952 | 120,000         | true  | true   |
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
> | 06a67018...e446a3f4 | 0318a96c...cbff386e | 118747255865344 | 1099551997952 | 1099639029764 | 120,000         | true  | true   |
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
> | 0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda |
> |--------------------------------------------------------------------|
> | 0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 0318a96c...cbff386e.
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
> Channel 118747255865344 connects 0318a96c...cbff386e and 0383300a...0b03dcda.
> Capacity: unknown.
>
> 0318a96c...cbff386e -> 0383300a...0b03dcda: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1794816706       |
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
> 0383300a...0b03dcda -> 0318a96c...cbff386e: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1794816706       |
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
>   "address": "bcrt1q23geur67h7y6w06ll5tpu0qz9tfzqay8v97jmr"
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
> | 06a67018...e446a3f4 | 0318a96c...cbff386e | 118747255865344 | 1099551997952 | 1099639029764 | 120,000         | true  | true   |
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
> | 95fb0e15...1c3a622c | 0383300a...0b03dcda | 118747255865344 | 1099639029764 | 1099551997952 | 120,000         | true  | true   |
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
> | -               | 9a5813ca...914648a9 | 0383300a...0b03dcda | broadcasting | unknown | -            | 87,654     |
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
> | -               | 9a5813ca...914648a9 | 0318a96c...cbff386e | broadcasting | unknown | -            | 30,000     |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "7160ec654110f52dfe33366985f28fbdfc001ae0c57310ada09f8b1964ee337d",
>   "1f384e6c836d821bf61e38bf6118df38edc340a2056d672d05960a36ef287568",
>   "53e931ae7cf39403465336df6c3704b8d3f992278014f2b6c48faa2ec8720605",
>   "78d32e66d99413dac66d3efb1436865b4ed86cc1fd79d1e20525903467da639d",
>   "249168a7beaf70a4df92de7b6aea6873dfe2a8c77224c22f9ad3c8dd527ffc3b",
>   "39ea0c770ed13b04443ee74f678ff3047840d23da2f15586b70aece289fd1e29"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "0bbfd047aaaf60214205443768a23f92f7f4a3e898cdcc8da47d7b8da5c3663c"
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

## 5b) LSPS1 channel purchase (node-b buys inbound liquidity from node-a)

node-a runs the LSPS1 (bLIP-51) channel-selling service (started with `--lsps1-service-config`). node-b, the client, adds node-a to its LSP list, places a BTC channel order, pays the order on-chain to the deposit address, and node-a opens the ordered channel back to node-b. The operator side (`pricing`, `options`, `orders`) and the client side (`lsp add`, `info`, `order`) are both shown. Amounts are decimal-string sats; an order is complete once its `channel` field is populated (and the operator ledger shows `order_state: completed`).


### Operator: inspect and re-apply pricing / options (node-a)

**Run:**

```bash
$ rgbldk --color never --output json --pretty lsps1 pricing get

```

**Result:**

> ```text
> {
>   "pricing": {
>     "btc_capacity_ppm_per_year": 10000,
>     "onchain_cost_sat": "10000",
>     "min_fee_sat": "1000"
>   },
>   "assets": []
> }
>
> ```

**Run:**

```bash
$ rgbldk lsps1 pricing set --json pricing.json

```

**Result:**

> ```text
> {"pricing":{"btc_capacity_ppm_per_year":10000,"onchain_cost_sat":"10000","min_fee_sat":"1000"},"assets":[]}
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty lsps1 options get

```

**Result:**

> ```text
> {
>   "supported_options": {
>     "min_required_channel_confirmations": 0,
>     "min_funding_confirms_within_blocks": 1,
>     "supports_zero_channel_reserve": false,
>     "max_channel_expiry_blocks": 52560,
>     "min_initial_client_balance_sat": "0",
>     "max_initial_client_balance_sat": "0",
>     "min_initial_lsp_balance_sat": "10000",
>     "max_initial_lsp_balance_sat": "100000000",
>     "min_channel_balance_sat": "10000",
>     "max_channel_balance_sat": "100000000"
>   },
>   "service": {
>     "require_token": null,
>     "bolt11_invoice_expiry_secs": 3600,
>     "min_onchain_payment_confirmations": 1,
>     "auto_close_expired_channels": true,
>     "channel_expiry_grace_blocks": 144,
>     "late_deposit_refund_window_secs": "2592000",
>     "onchain_open_timeout_blocks": 144,
>     "force_close_after_blocks": 0,
>     "accept_new_orders": true,
>     "max_unpaid_orders_per_peer": 5,
>     "max_unpaid_orders_total": 100
>   }
> }
>
> ```

**Run:**

```bash
$ rgbldk lsps1 options set --json options.json

```

**Result:**

> ```text
> {"supported_options":{"min_required_channel_confirmations":0,"min_funding_confirms_within_blocks":1,"supports_zero_channel_reserve":false,"max_channel_expiry_blocks":52560,"min_initial_client_balance_sat":"0","max_initial_client_balance_sat":"0","min_initial_lsp_balance_sat":"10000","max_initial_lsp_balance_sat":"100000000","min_channel_balance_sat":"10000","max_channel_balance_sat":"100000000"},"service":{"require_token":null,"bolt11_invoice_expiry_secs":3600,"min_onchain_payment_confirmations":1,"auto_close_expired_channels":true,"channel_expiry_grace_blocks":144,"late_deposit_refund_window_secs":"2592000","onchain_open_timeout_blocks":144,"force_close_after_blocks":0,"accept_new_orders":true,"max_unpaid_orders_per_peer":5,"max_unpaid_orders_total":100}}
>
> ```

**Run:**

```bash
$ rgbldk lsps1 orders ls

```

**Result:**

> ```text
> No LSPS1 orders served.
>
> ```

### Client: add the LSP, inspect the offering, place an order (node-b)

`lsp add` connects to the LSP and asks which LSPS protocols it supports (LSPS0); the entry is saved only if that succeeds. `--trust-0conf` lets node-b use channels from node-a before the funding confirms, which the LSPS2 section below needs. `--trust-no-reserve` drops the on-chain reserve node-b would otherwise keep for force-close fees.


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
$ rgbldk lsp add --pubkey <node_id_a> --address <node_a_p2p> --trust-0conf --trust-no-reserve

```

**Result:**

> ```text
> LSP 0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda @ 127.0.0.1:9735 saved (LSPS1, LSPS2)
>
> ```

**Run:**

```bash
$ rgbldk lsp ls

```

**Result:**

> ```text
> +---------------------+----------------+--------------+-------+------------+-------+
> | Pubkey              | Address        | Protocols    | 0conf | No Reserve | Token |
> +==================================================================================+
> | 0383300a...0b03dcda | 127.0.0.1:9735 | LSPS1, LSPS2 | yes   | yes        | no    |
> +---------------------+----------------+--------------+-------+------------+-------+
>
> ```

**Run:**

```bash
$ rgbldk lsps1 info

```

**Result:**

> ```text
> {"lsps":[{"lsp_node_id":"0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda","info":{"supported_options":{"min_required_channel_confirmations":0,"min_funding_confirms_within_blocks":1,"supports_zero_channel_reserve":false,"max_channel_expiry_blocks":52560,"min_initial_client_balance_sat":"0","max_initial_client_balance_sat":"0","min_initial_lsp_balance_sat":"10000","max_initial_lsp_balance_sat":"100000000","min_channel_balance_sat":"10000","max_channel_balance_sat":"100000000"},"pricing":{"btc_capacity_ppm_per_year":10000,"onchain_cost_sat":"10000","min_fee_sat":"1000"},"rgb":{"rgb_assets":[]}},"error":null}]}
>
> ```

The order is paid on-chain: the client sends the order total to the deposit address the LSP returns (`payment.onchain.address`), with no pre-existing channel required. The LSP's watcher opens the ordered channel once the deposit confirms.


**Run:**

```bash
$ rgbldk lsps1 order create --lsp <node_id_a> --lsp-balance-sat 100000 --channel-expiry-blocks 4380

```

**Result:**

> ```text
> {
>   "order_id": "b281163c-0818-42b6-beac-a4b3818ef5b7",
>   "order": {
>     "lsp_balance_sat": "100000",
>     "client_balance_sat": "0",
>     "required_channel_confirmations": 0,
>     "funding_confirms_within_blocks": 1,
>     "channel_expiry_blocks": 4380,
>     "announce_channel": false
>   },
>   "payment": {
>     "bolt11": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1794820330",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "invoice": "lnbcrt100840n1p404wk6dqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5pp52cvztw534nmuuda27ccd4s0lfzljp3nyvm8vfrq3f869ep4skdhssp5klgusex5evd28eku4gqckpxdhxrlhrhxwdrq2a6myfra7qasvgws9qyysgqcqzp2xqrrssspkv3gesj9h8tq22dflxdxd2vxl4pjulwv00ny6jh80qvwmtz6cxde5evw9520nkrrrxm40ez6qqmusa9klq3teghlhd2482te22m8gpptn52z"
>     },
>     "onchain": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1794820330",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "address": "bcrt1qj3jpxt80l4593a6d44f0euesf52gtxhgev4s97",
>       "min_onchain_payment_confirmations": 1,
>       "refund_onchain_address": "bcrt1q90he0aw64u77q65pjaqcntrstzwe0rf6m86xj9"
>     }
>   },
>   "channel": null,
>   "rgb": null
> }
>
> ```

### Client: pay the order on-chain; node-a opens the channel

Send the order total (`10084` sat) to the deposit address from `payment.onchain.address`. Any wallet works; on regtest the miner pays it directly. Once the deposit confirms, node-a's watcher opens the ordered channel back to node-b.


**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qj3jpxt80l4593a6d44f0euesf52gtxhgev4s97 0.00010084

```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Run:**

```bash
$ rgbldk lsps1 order get <order_id>

```

**Result:**

> ```text
> order b281163c-0818-42b6-beac-a4b3818ef5b7 — channel opened
> {"order_id":"b281163c-0818-42b6-beac-a4b3818ef5b7","order":{"lsp_balance_sat":"100000","client_balance_sat":"0","required_channel_confirmations":0,"funding_confirms_within_blocks":1,"channel_expiry_blocks":4380,"announce_channel":false},"payment":{"bolt11":{"state":"expect_payment","expires_at_unix_secs":"1794820330","fee_total_sat":"10084","order_total_sat":"10084","invoice":"lnbcrt100840n1p404wk6dqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5pp52cvztw534nmuuda27ccd4s0lfzljp3nyvm8vfrq3f869ep4skdhssp5klgusex5evd28eku4gqckpxdhxrlhrhxwdrq2a6myfra7qasvgws9qyysgqcqzp2xqrrssspkv3gesj9h8tq22dflxdxd2vxl4pjulwv00ny6jh80qvwmtz6cxde5evw9520nkrrrxm40ez6qqmusa9klq3teghlhd2482te22m8gpptn52z"},"onchain":{"state":"paid","expires_at_unix_secs":"1794820330","fee_total_sat":"10084","order_total_sat":"10084","address":"bcrt1qj3jpxt80l4593a6d44f0euesf52gtxhgev4s97","min_onchain_payment_confirmations":1,"refund_onchain_address":"bcrt1q90he0aw64u77q65pjaqcntrstzwe0rf6m86xj9"}},"channel":{"funded_at_unix_secs":"1794816735","funding_outpoint":"a2f4991d134d946e5843964cfb703fb08104c3fd2bc62f0f058ce23deaaa7d9e:1","expires_at_unix_secs":"1797444735"},"rgb":null}
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+------+---------------+---------------+-----------------+-------+--------+-------+---------------------+
> | User Channel ID     | Counterparty        | SCID | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable | Lease | Lease Ends (height) |
> +===================================================================================================================================================+
> | baa8fe3c...77ea18bd | 0383300a...0b03dcda | -    | 1099667734532 | 1099569430531 | 100,000         | true  | true   | lsps1 | 4506                |
> +---------------------+---------------------+------+---------------+---------------+-----------------+-------+--------+-------+---------------------+
>
> ```

### Operator: the fulfilled order in the ledger (node-a)

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
$ rgbldk lsps1 orders ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------+---------+----------+---------+
> | Order ID            | Counterparty        | Order     | Payment | Paid Via | Failure |
> +======================================================================================+
> | b281163c...818ef5b7 | 0318a96c...cbff386e | completed | paid    | onchain  | -       |
> +---------------------+---------------------+-----------+---------+----------+---------+
>
> ```

**Run:**

```bash
$ rgbldk lsps1 orders get <order_id>

```

**Result:**

> ```text
> {"counts_toward_unpaid_quota":false,"order_id":"b281163c-0818-42b6-beac-a4b3818ef5b7","counterparty_node_id":"0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e","order_state":"completed","payment_state":"paid","paid_via":"onchain","lsp_balance_sat":"100000","client_balance_sat":"0","channel_expiry_blocks":4380,"announce_channel":false,"fee_total_sat":"10084","order_total_sat":"10084","onchain_address":"bcrt1qj3jpxt80l4593a6d44f0euesf52gtxhgev4s97","onchain_paid_sat":"10084","refund_onchain_address":"bcrt1q90he0aw64u77q65pjaqcntrstzwe0rf6m86xj9","refund_txid":null,"fulfill_retry_count":0,"created_at_unix_secs":"1794816730","payment_expires_at_unix_secs":"1794820330","funding_outpoint":"a2f4991d134d946e5843964cfb703fb08104c3fd2bc62f0f058ce23deaaa7d9e:1","funded_at_height":126,"channel_closed_at_unix_secs":null,"close_kind":null,"close_attempts":0,"last_close_error":null,"first_close_attempt_at_unix_secs":null,"next_close_attempt_at_unix_secs":null,"failure_reason":null,"cancelled_at_unix_secs":null,"cancel_reason":null,"rgb":null}
>
> ```

### Client: RGB channel orders

RGB channel orders (`lsps1 rgb-order`) require the LSP to advertise an RGB asset offering and hold that asset in inventory. This node-a instance sells BTC capacity only, so the call below is expected to be rejected — it documents the command shape and the asset-not-offered error a client sees against a BTC-only LSP.


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
$ rgbldk lsps1 rgb-order --lsp <node_id_a> --asset-id <contract_id> --lsp-asset-balance 100 --lsp-balance-sat 100000 --channel-expiry-blocks 4380

```

**Result:**

> ```text
> HTTP 400: invalid_input
> message=The LSP does not offer asset rgb:2WBcas9-Vm3S6huqv-Yd2yMUM3l-fWbTvHtnv-CvcQngbY5-tdEeVw.
> (exit 1)
>
> ```

### Retry-safe orders and operator cancel

`--client-request-id` makes a retry safe: running the same command again returns the first order instead of placing a second one. The operator can cancel an order while it is still awaiting payment, one by one or for a whole client.


**Run:**

```bash
$ rgbldk lsps1 order create --lsp <node_id_a> --lsp-balance-sat 100000 --channel-expiry-blocks 4380 --client-request-id example-order-1

```

**Result:**

> ```text
> {
>   "order_id": "588bcabf-6bad-488f-8928-53870dd2874d",
>   "client_request_id": "example-order-1",
>   "order": {
>     "lsp_balance_sat": "100000",
>     "client_balance_sat": "0",
>     "required_channel_confirmations": 0,
>     "funding_confirms_within_blocks": 1,
>     "channel_expiry_blocks": 4380,
>     "announce_channel": false
>   },
>   "payment": {
>     "bolt11": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1794820337",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "invoice": "lnbcrt100840n1p404whpdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5pp54r93t9axydtwestxa7csl3yfzydec9fk4gmcz6278z06dym9pw8ssp5wch40ep6rz0hs7dvrhq0yez63sh2qnnzfywp90esda9c7895rnfs9qyysgqcqzp2xqrrssrzjqvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxuqqqqyqqjnsqqsqqqqlgqqqqqqqqfq8clql8a3krgzw656895etu7lnrplev0xtgwzlgl442arz6kfn8xh9qr8zkutc2s88ppuj4fnl9r0dj6wgyz83zf9arcq9vh0p7gmp6cpwyykh4"
>     },
>     "onchain": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1794820337",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "address": "bcrt1q22u5vkl9nrn506lsvz5sz00l9rnv677zkc8qha",
>       "min_onchain_payment_confirmations": 1,
>       "refund_onchain_address": "bcrt1q82qvf2y67dtj4plyy6t58ul9nttne5n9zvhvfw"
>     }
>   },
>   "channel": null,
>   "rgb": null
> }
>
> ```

**Run:**

```bash
$ rgbldk lsps1 order create --lsp <node_id_a> --lsp-balance-sat 100000 --channel-expiry-blocks 4380 --client-request-id example-order-1

```

**Result:**

> ```text
> {
>   "order_id": "588bcabf-6bad-488f-8928-53870dd2874d",
>   "client_request_id": "example-order-1",
>   "order": {
>     "lsp_balance_sat": "100000",
>     "client_balance_sat": "0",
>     "required_channel_confirmations": 0,
>     "funding_confirms_within_blocks": 1,
>     "channel_expiry_blocks": 4380,
>     "announce_channel": false
>   },
>   "payment": {
>     "bolt11": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1794820337",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "invoice": "lnbcrt100840n1p404whpdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5pp54r93t9axydtwestxa7csl3yfzydec9fk4gmcz6278z06dym9pw8ssp5wch40ep6rz0hs7dvrhq0yez63sh2qnnzfywp90esda9c7895rnfs9qyysgqcqzp2xqrrssrzjqvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxuqqqqyqqjnsqqsqqqqlgqqqqqqqqfq8clql8a3krgzw656895etu7lnrplev0xtgwzlgl442arz6kfn8xh9qr8zkutc2s88ppuj4fnl9r0dj6wgyz83zf9arcq9vh0p7gmp6cpwyykh4"
>     },
>     "onchain": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1794820337",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "address": "bcrt1q22u5vkl9nrn506lsvz5sz00l9rnv677zkc8qha",
>       "min_onchain_payment_confirmations": 1,
>       "refund_onchain_address": "bcrt1q82qvf2y67dtj4plyy6t58ul9nttne5n9zvhvfw"
>     }
>   },
>   "channel": null,
>   "rgb": null
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
$ rgbldk lsps1 orders cancel <order_id> --reason example

```

**Result:**

> ```text
> {"counts_toward_unpaid_quota":false,"order_id":"588bcabf-6bad-488f-8928-53870dd2874d","counterparty_node_id":"0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e","order_state":"failed","payment_state":"expect_payment","paid_via":null,"lsp_balance_sat":"100000","client_balance_sat":"0","channel_expiry_blocks":4380,"announce_channel":false,"fee_total_sat":"10084","order_total_sat":"10084","onchain_address":"bcrt1q22u5vkl9nrn506lsvz5sz00l9rnv677zkc8qha","onchain_paid_sat":null,"refund_onchain_address":"bcrt1q82qvf2y67dtj4plyy6t58ul9nttne5n9zvhvfw","refund_txid":null,"fulfill_retry_count":0,"created_at_unix_secs":"1794816737","payment_expires_at_unix_secs":"1794820337","funding_outpoint":null,"funded_at_height":null,"channel_closed_at_unix_secs":null,"close_kind":null,"close_attempts":0,"last_close_error":null,"first_close_attempt_at_unix_secs":null,"next_close_attempt_at_unix_secs":null,"failure_reason":"cancelled","cancelled_at_unix_secs":"1794816737","cancel_reason":"example","rgb":null}
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
$ rgbldk lsps1 order create --lsp <node_id_a> --lsp-balance-sat 100000 --channel-expiry-blocks 4380 --client-request-id example-order-2

```

**Result:**

> ```text
> order 401b6c28-d5c3-4888-a4f2-bf38bf004504 — awaiting fulfillment
> {"order_id":"401b6c28-d5c3-4888-a4f2-bf38bf004504","client_request_id":"example-order-2","order":{"lsp_balance_sat":"100000","client_balance_sat":"0","required_channel_confirmations":0,"funding_confirms_within_blocks":1,"channel_expiry_blocks":4380,"announce_channel":false},"payment":{"bolt11":{"state":"expect_payment","expires_at_unix_secs":"1794820338","fee_total_sat":"10084","order_total_sat":"10084","invoice":"lnbcrt100840n1p404whzdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5pp53j8ym0vaqef8zkcsvec9xgeppqz63d4adz879q8537469a2av7wssp59uuys3x4u9xwjjrnsklmetlk463gk99upqrajy59330t0xqn4gjs9qyysgqcqzp2xqrrssrzjqvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxuqqqqyqqjnsqqsqqqqlgqqqqqqqqfqpen69jhtjqfttxuwaxejzcyqukkxwjzxepszer84y0e303ty2j4zjnrguwt704daqr9zdkwa29z2l45a75nxp7dhynnzdl90vxck0fsq4wkwhq"},"onchain":{"state":"expect_payment","expires_at_unix_secs":"1794820338","fee_total_sat":"10084","order_total_sat":"10084","address":"bcrt1qyq6ay3kynsjjsyd76r0chpawkve2kanjt5zzzf","min_onchain_payment_confirmations":1,"refund_onchain_address":"bcrt1q4l4whw253vw6f8qqdqewfqs0y063j762kpqcw8"}},"channel":null,"rgb":null}
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
$ rgbldk lsps1 orders cancel-peer <node_id_b>

```

**Result:**

> ```text
> {"cancelled":[{"counts_toward_unpaid_quota":false,"order_id":"401b6c28-d5c3-4888-a4f2-bf38bf004504","counterparty_node_id":"0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e","order_state":"failed","payment_state":"expect_payment","paid_via":null,"lsp_balance_sat":"100000","client_balance_sat":"0","channel_expiry_blocks":4380,"announce_channel":false,"fee_total_sat":"10084","order_total_sat":"10084","onchain_address":"bcrt1qyq6ay3kynsjjsyd76r0chpawkve2kanjt5zzzf","onchain_paid_sat":null,"refund_onchain_address":"bcrt1q4l4whw253vw6f8qqdqewfqs0y063j762kpqcw8","refund_txid":null,"fulfill_retry_count":0,"created_at_unix_secs":"1794816738","payment_expires_at_unix_secs":"1794820338","funding_outpoint":null,"funded_at_height":null,"channel_closed_at_unix_secs":null,"close_kind":null,"close_attempts":0,"last_close_error":null,"first_close_attempt_at_unix_secs":null,"next_close_attempt_at_unix_secs":null,"failure_reason":"cancelled","cancelled_at_unix_secs":"1794816738","cancel_reason":null,"rgb":null}],"skipped":[]}
>
> ```

**Run:**

```bash
$ rgbldk lsps1 orders ls

```

**Result:**

> ```text
> +---------------------+---------------------+-----------+----------------+----------+-----------+
> | Order ID            | Counterparty        | Order     | Payment        | Paid Via | Failure   |
> +===============================================================================================+
> | 401b6c28...bf004504 | 0318a96c...cbff386e | failed    | expect_payment | -        | cancelled |
> |---------------------+---------------------+-----------+----------------+----------+-----------|
> | 588bcabf...0dd2874d | 0318a96c...cbff386e | failed    | expect_payment | -        | cancelled |
> |---------------------+---------------------+-----------+----------------+----------+-----------|
> | b281163c...818ef5b7 | 0318a96c...cbff386e | completed | paid           | onchain  | -         |
> +---------------------+---------------------+-----------+----------------+----------+-----------+
>
> ```

## 5c) LSPS2 JIT channels (node-b receives through a channel node-a opens on payment)

node-a also runs the LSPS2 (bLIP-52) JIT channel service (started with `--lsps2-service-config`). node-b buys a JIT invoice from node-a. When a payer pays it, node-a opens a 0conf channel to node-b and forwards the payment minus the opening fee. node-b must be online when the payment arrives and must trust node-a for 0conf (`lsp add --trust-0conf`); otherwise `invoice`/`rgb-invoice` return 409 `precondition_failed` before anything is bought.


### Operator: inspect and re-apply LSPS2 options (node-a)

**Run:**

```bash
$ rgbldk --color never --output json --pretty lsps2 options get

```

**Result:**

> ```text
> {
>   "advertise_service": true,
>   "accept_new_channels": true,
>   "require_token": null,
>   "channel_opening_fee_ppm": 10000,
>   "channel_over_provisioning_ppm": 100000,
>   "min_channel_opening_fee_msat": "2000000",
>   "min_channel_lifetime": 4032,
>   "max_client_to_self_delay": 2016,
>   "min_payment_size_msat": "10000000",
>   "max_payment_size_msat": "1000000000",
>   "max_unpaid_channels_per_peer": 1,
>   "max_unpaid_channels_total": 10,
>   "unpaid_channel_close_secs": "86400"
> }
>
> ```

**Run:**

```bash
$ rgbldk lsps2 options set --json options.json

```

**Result:**

> ```text
> {"advertise_service":true,"accept_new_channels":true,"require_token":null,"channel_opening_fee_ppm":10000,"channel_over_provisioning_ppm":100000,"min_channel_opening_fee_msat":"2000000","min_channel_lifetime":4032,"max_client_to_self_delay":2016,"min_payment_size_msat":"10000000","max_payment_size_msat":"1000000000","max_unpaid_channels_per_peer":1,"max_unpaid_channels_total":10,"unpaid_channel_close_secs":"86400"}
>
> ```

### Client: BTC JIT quote and invoice (node-b)

`quote` lists node-a's fee offers; with `--amount-msat` it also returns the opening fee for that amount: `max(amount × proportional_ppm / 1,000,000, min_fee_msat)`. `invoice` buys the JIT channel and returns a BOLT11 invoice whose route hint goes through node-a. `--max-fee` caps the fee (total msat for a fixed amount).


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
$ rgbldk lsps2 quote --lsp <node_id_a> --amount-msat 50000000

```

**Result:**

> ```text
> {"offers":[{"min_fee_msat":"2000000","proportional_ppm":10000,"valid_until":"2026-11-17T08:12:18.514+00:00","min_lifetime":4032,"max_client_to_self_delay":2016,"min_payment_size_msat":"10000000","max_payment_size_msat":"1000000000"}],"fee_msat":"2000000"}
>
> ```

**Run:**

```bash
$ rgbldk lsps2 invoice --lsp <node_id_a> --amount-msat 50000000 --max-fee 2500000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt500u1p404whzdqqpp5pkmft2ludy5j2zn6an5ef33qvq2q29cqvhm5em7wqn7nlsr8rqxqsp5yhrula6eew7clrrfvc4d7hghvhtjvdkgturz833fg069w5wd5a4s9qyysgqcqzpvxqrrssrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqq9tsqq5qqqqqqqqqqqqqqfqm5gde35aap30gc7ptu9fz2vuyq73m4slhekpz6x02snemc6gzmryhz3f59797r65gm4wlmtlcjx99r35c9ryppgcuufe3y5qcw645wsp6s45r9",
>   "payment_hash": "0db695abfc6929250a7aece994c620601405170065f74cefce04fd3fc067180c",
>   "amount_msat": "50000000",
>   "fee_msat": "2000000",
>   "expires_at_unix_secs": "1794820338",
>   "min_lifetime": 4032
> }
>
> ```

The response carries the payment hash, the fee and the expiry. `pay get` follows the payment by that hash.


**Run:**

```bash
$ rgbldk pay get <payment_hash>

```

**Result:**

> ```text
> +------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                                                              |
> +=============================================================================================================================================================================================================================================================================================================+
> | id                     | 0db695abfc6929250a7aece994c620601405170065f74cefce04fd3fc067180c                                                                                                                                                                                                                   |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 0db695abfc6929250a7aece994c620601405170065f74cefce04fd3fc067180c                                                                                                                                                                                                                   |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Inbound                                                                                                                                                                                                                                                                            |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | … Pending                                                                                                                                                                                                                                                                          |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11Jit                                                                                                                                                                                                                                                                          |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                                                              |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816738                                                                                                                                                                                                                                                                         |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | 1794820338                                                                                                                                                                                                                                                                         |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"counterparty_skimmed_fee_msat":null,"payment_hash":"0db695abfc6929250a7aece994c620601405170065f74cefce04fd3fc067180c","preimage":"3d73be8bf452aeeb69f0696e9daaeca240b3157ea0741326a9693344fd885e9f","secret":"25c7cff759cbbd8f8c69662adf5d1765d72636c85f0623c62943f45751cda76b"} |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 50,000,000 msat                                                                                                                                                                                                                                                                    |
> |------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | -                                                                                                                                                                                                                                                                                  |
> +------------------------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
>
> ```

The invoice is paid by any node with a route to node-a; node-b then sees a `PaymentReceived` for the amount minus the fee, and `pay payments` lists it with kind `Bolt11Jit`. This walkthrough has no third node, so the invoice is not paid here.


The operator lists the JIT channels it sold, and the offers that were bought and not paid yet, with `jit-channels`.


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
$ rgbldk lsps2 jit-channels

```

**Result:**

> ```text
> +---------------------+------------------+------------+------+------------+--------------+-----------+----------------+
> | Counterparty        | State            | Channel ID | Kind | Fee (msat) | Unpaid Quota | Closed By | Close Attempts |
> +=====================================================================================================================+
> | 0318a96c...cbff386e | awaiting_payment | -          | BTC  | -          | false        | -         | 0              |
> +---------------------+------------------+------------+------+------------+--------------+-----------+----------------+
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

### Client: RGB JIT quote (node-b)

RGB JIT channels carry BTC and an asset, and are priced with the LSP's LSPS1 pricing, so the LSP must run LSPS1 too and offer the asset. `rgb-quote` returns the offering and a local fee estimate; `rgb-invoice` takes the same arguments plus `--max-fee-sat` and returns an RGB invoice. This node-a instance offers no RGB asset, so both calls below are expected to be rejected — they document the command shape.


**Run:**

```bash
$ rgbldk lsps2 rgb-quote --lsp <node_id_a> --contract-id <contract_id> --asset-amount 100 --lsp-asset-balance 150 --lsp-balance-sat 200000

```

**Result:**

> ```text
> HTTP 400: invalid_input
> message=the LSP does not offer asset contract:RDD~tWXQ-QrO45y4-iOwxuEP-98LEwNV-0rIPeXA-Rwf60ko
> (exit 1)
>
> ```

**Run:**

```bash
$ rgbldk lsps2 rgb-invoice --lsp <node_id_a> --contract-id <contract_id> --asset-amount 100 --lsp-asset-balance 150 --lsp-balance-sat 200000 --max-fee-sat 2000

```

**Result:**

> ```text
> HTTP 400: lsp_rejected
> message=asset_id is not offered by this LSP
> (exit 1)
>
> ```

### Client: remove the LSP (node-b)

`lsp rm` drops node-a from the list. Channels already bought stay open, but JIT invoices bought from node-a can no longer be paid. It is refused while an order is paid but its channel is not open yet.


**Run:**

```bash
$ rgbldk lsp rm <node_id_a>

```

**Result:**

> ```text
> LSP 0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda removed
>
> ```

**Run:**

```bash
$ rgbldk lsp ls

```

**Result:**

> ```text
> No LSPs configured.
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
>   "address": "bcrt1qlyg0q2wfm5rl8u8e4vl5508hzu9umyl8p6uv8t"
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
> rgb(wpkh([02472ed8/86h/827167h/0h]tpubDD72c5hpf3Y3jJJ3NHDLAR4Jxt11FCKhHnFyc2Qd5hXcmsUsJ7dVvhCwKYzy2aNAaw4E1LhgRhqeccDyj7G8hEPbnpuwC2TRYNzQYySHbRs/<0;1>/*),seals(),noise(842e25d662fd9931132b0e6176e00b173f078b5b8aeb61cf1d3cce1dce797035))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | 02472ed8    | m/86'/827167'/0' | tpubDD72c5hpf3Y3jJJ3NHDLAR4Jxt11FCKhHnFyc2Qd5hXcmsUsJ7dVvhCwKYzy2aNAaw4E1LhgRhqeccDyj7G8hEPbnpuwC2TRYNzQYySHbRs | wpkh([02472ed8/86'/827167'/0']tpubDD72c5hpf3Y3jJJ3NHDLAR4Jxt11FCKhHnFyc2Qd5hXcmsUsJ7dVvhCwKYzy2aNAaw4E1LhgRhqeccDyj7G8hEPbnpuwC2TRYNzQYySHbRs/<0;1>/*)#s90vd8fz |
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
> | Public key      | 02cd46a729ac78296d29d725933d7ecc204361fc785f1c22c71c8d552f7729f57f |
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
> 3045022100998cf6023ee654e252fa7d35e8a1a9de55f76253eaf86a3d8e36923abafeb9b8022035498eb9acb9fdfa6bc34eb9f05ad170a25dffcdcc2f8129123527004b2a2d76
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +--------------------------+
> | Issuer                   |
> +==========================+
> | RGB20-Simplest-v0-rLosfg |
> +--------------------------+
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
>   "address": "bcrt1qgx7h8s2h8ce59lzy5jx8phka4hhk7mzf2cml6a"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> 64ee8d5a4f4417bb7aa90b34d64b40642613cf1c751b5e40f8aa4580f62b355e
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> 6d69b3cde9f0839bf38ce6f4efbca54304703cdbb64afe3cc7bf8fe8e466f823
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "1d8d818d0bddd8785435326ef6332e5400eb19b040647770068ccf73c4858b57"
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
> | 64ee8d5a4f4417bb7aa90b34d64b40642613cf1c751b5e40f8aa4580f62b355e:0 |   10,000,000 | Confirmed |    128 | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------|
> | 6d69b3cde9f0839bf38ce6f4efbca54304703cdbb64afe3cc7bf8fe8e466f823:0 |      110,000 | Confirmed |    128 | Available    | -           | FeeSupport, BlindingTarget |
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
> | 64ee8d5a4f4417bb7aa90b34d64b40642613cf1c751b5e40f8aa4580f62b355e:0 |   10,000,000 |    128 | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | 6d69b3cde9f0839bf38ce6f4efbca54304703cdbb64afe3cc7bf8fe8e466f823:0 |      110,000 |    128 | Available |        |
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
>   "reservation_id": "manual-reservation-1794816744-j2smVTiHy0Uu15hfi7tmmcJnm81eDXGY",
>   "outpoint": "64ee8d5a4f4417bb7aa90b34d64b40642613cf1c751b5e40f8aa4580f62b355e:0",
>   "reserved_until_unix_secs": "1794816804"
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
> +--------------------------+
> | Issuer                   |
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
>   "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
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
>       "detail": "64ee8d5a4f4417bb7aa90b34d64b40642613cf1c751b5e40f8aa4580f62b355e:0"
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
> | DemoAsset | DEMO   |         0 |    100 | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
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
$ rgbldk rgb contracts export --contract-id contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM --out /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.
> Consignment key: contract_export_096096af0e68a42532eb5bb963bfbf99664915c8f8870fcad2e2aaf106bd0bcf
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=64ee8d5a4f4417bb7aa90b34d64b40642613cf1c751b5e40f8aa4580f62b355e:0
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
>         "detail": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_d8d855b5a82af1eb70398f172cb4e9d319024eb7e0d8046a51d41194e68daa5d",
>     "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>     "ok": true
>   },
>   "out": "/Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_d8d855b5a82af1eb70398f172cb4e9d319024eb7e0d8046a51d41194e68daa5d.
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.download.zip.
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
>   "address": "bcrt1qd52fml0d74ykcqez88v2yy75zk3jz2mfd4f3k5"
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
> 9e859df30b4847fce4f9263acc9acb32334daa945bd1cd70a240e3fb8029a182
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "2b1e9e3d03b5d2b5cd463c06178d571b4225ea47b1e85296a7e8ee51d853d6a3"
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
$ rgbldk rgb contracts import --contract-id contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM --file /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_ac0f414f2a9eed98e7313a5dfb383196b496e84a4e90fef181bba478cce34919
> Imported contract contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.
> Consignment key: contract_import_ac0f414f2a9eed98e7313a5dfb383196b496e84a4e90fef181bba478cce34919
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
> | DemoAsset | DEMO   |         0 |    100 | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
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
> The node knows contract contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.
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
>   "invoice": "contract:tb@k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM/90@at:FPpl999w-yaXY~b7g-qVFmg9Bv-JYImORY4-Tu9JrLDG-E5ubjA/?expiry=2026-11-16T09:12:29.548+00:00",
>   "blinding_utxo_used": "9e859df30b4847fce4f9263acc9acb32334daa945bd1cd70a240e3fb8029a182:1"
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
> | Contract         | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:FPpl999w-yaXY~b7g-qVFmg9Bv-JYImORY4-Tu9JrLDG-E5ubjA    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1794820349                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM/90@at:FPpl999w-yaXY~b7g-qVFmg9Bv-JYImORY4-Tu9JrLDG-E5ubjA/?expiry=2026-11-16T09:12:29.548+00:00
> scope=contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:FPpl999w-yaXY~b7g-qVFmg9Bv-JYImORY4-Tu9JrLDG-E5ubjA
> data=90
> expiry=2026-11-16T09:12:29.548+00:00
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
>   "txid": "d1b5fe3df66a85dd2d5d5cf9b80528f087197b3e7ebfbd992a6ef6fd299f02df",
>   "consignment_key": "rgb_consignment_contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM_d1b5fe3df66a85dd2d5d5cf9b80528f087197b3e7ebfbd992a6ef6fd299f02df"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM_d1b5fe3df66a85dd2d5d5cf9b80528f087197b3e7ebfbd992a6ef6fd299f02df.
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-a-to-b-prechannel.zip.
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
> Accepted contract contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "4cb9e6109b553516d4d3804c6636bbd0754b8ee7dd6f348d87e8353edeaf5205",
>   "36b853dca011131a0e8f7466920868e943a5bc1a45f8ccb53a55605eb6bf7356",
>   "09d420f8a0be0a18c8a942fcd2d646d9410261497ee7f7990a0a6fefc939c472",
>   "5717140ea86ba4527ae987614291c9b824c762476e8a8c19e73999efd31cfef6",
>   "34a405ff3985297ef0d03d3f18f324e0b098cbcb26522895dd75f3335fe54f2c",
>   "3b4bac3ab84fd3818d667ba62befdfa9f69ebc706d0191cf268ddee9b3cc53db"
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
$ rgbldk rgb contracts balance contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
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
> | 8e93468879b27838e511663aab1dfbb52985eeff2d64261b2ce78a8d3853e147 | succeeded | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |     90 | d1b5fe3df66a85dd2d5d5cf9b80528f087197b3e7ebfbd992a6ef6fd299f02df | transfer_import_bf69906321fcf853505a122b85c567d98a2172c86ae5b9bc3343fe1faf8758e1 |
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
$ rgbldk rgb contracts balance contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
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
>   "user_channel_id": "5d9bbe711f1c32343bd6df05425c5188"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "6dcee9d8e545e2d75a2715821f578443db15513b8b82ff660e187279e908f8c2",
>   "0e269c8fdcafc864b60a5cffd6dbd60729662560226b320c145cb854da1cea16",
>   "622010bad821f8a3c49513d83a8e4e7df72ac76a7296530698f00aa4be817563",
>   "02d1eb13710404e9e4d03d7be507cfc212a5a0c546b74270c7c078bae2978909",
>   "75565acb6e56531eac1ca52e0afaeb75403b0deeea124f8a048f66c9035cb7bf",
>   "375070f0ff91e3cd2fe7527e0035dbee1bb573d65a7cfd8f87a001ae2cf85c45"
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
> | b281163c...818ef5b7 | 0318a96c...cbff386e | 140737488420865 | 1099569430531 | 1099667734532 | 100,000         | true  | true   |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------|
> | 5d9bbe71...425c5188 | 0318a96c...cbff386e | -               | -             | -             | 100,000         | false | false  |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+
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

Because we passed `--rgb-context` during `channel open`, in a multi-host deployment the acceptor fetches the funding consignment directly from the opener over HTTP (no manual download/upload). On this single-host demo both daemons share one consignment cache directory, so that receiver-side HTTP fetch isn't exercised here; the RGB balance checks below are the real proof the channel carries its asset.


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
$ rgbldk rgb onchain invoice-create --contract-id <contract_id> --amount 7 --use-witness-utxo

```

**Result:**

> ```text
> {
>   "invoice": "contract:tb@k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM/7@wout:AQAAAAAA-AAAQbqBE-ad_30DTO-84qS535b-F4LiAU7y-LS3v/?expiry=2026-11-16T09:12:53.647+00:00",
>   "blinding_utxo_used": null
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
>   "txid": "d2a6891a38ec5d6d8e8e582972f7c4454f92b30c22af15f18bcd3eef22fbb78f",
>   "consignment_key": "rgb_consignment_contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM_d2a6891a38ec5d6d8e8e582972f7c4454f92b30c22af15f18bcd3eef22fbb78f"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM_d2a6891a38ec5d6d8e8e582972f7c4454f92b30c22af15f18bcd3eef22fbb78f.
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/rgb-onchain-b-to-a.zip.
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
> Accepted contract contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "03efc09abc6e4060d0392d8817f49200e670c2b4d37e8ac6f83860a30f74d3d1",
>   "46eb9da702fa7ab5d5425b5440e9313d156c42e7b202f87b95aefbe7c2973e71",
>   "00604be38c00e5f6a86c6c56f1d29eb93eb6dd11802339ff217370c6e9c10a8d",
>   "0506dcd4c748bd305621ac3b6208999f96c4db52ae094a92f199575cd306ec87",
>   "02b41f7971f418fea1ad72605c535ed332c8c488adf12ef8bd82b8a4df711075",
>   "4e1c79d3d980bfcce768a08bc0eb191d4bee2d14d7a15891348487f20aeb3649"
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
$ rgbldk rgb contracts balance contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |
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
>   "address": "bcrt1qdd7afk7xdm4s44s002kqtll4pn4rhnff4zzv9l"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> c782f7268732346a80dbade525cb9d1a38dd19c11e1cb1cfa44e920ceb438f05
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "025361c8bbe2bf1560ccde9292d61189a426d66d3ac367d271eb4c2a58dcc1b5"
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
>   "address": "bcrt1qpaf7g9w09mkme3teqnl2lg955y46yvgyuf220q"
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
>   "address": "bcrt1q08hzyuw8hv4xxzvc4px2ghy8y0ue860sgcw486"
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
>   "address": "bcrt1qyaw2d0nr4yp99xntfq76kzvzzhpz7r6tzz4lkw"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"dd948d2e00b6ed7a668c4d44d7df42cb7b7f48e38d9a83b1559d6312ff06f5f0","status":"broadcast","outputs":[{"address":"bcrt1qpaf7g9w09mkme3teqnl2lg955y46yvgyuf220q","value_sats":"30000","vout":2},{"address":"bcrt1q08hzyuw8hv4xxzvc4px2ghy8y0ue860sgcw486","value_sats":"28000","vout":0}],"change":{"address":"bcrt1qyaw2d0nr4yp99xntfq76kzvzzhpz7r6tzz4lkw","value_sats":"941828","vout":1},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "01c584c409d999b026b2aa73065c7edcadaa3bd31e2f9f2f00720d3dcd27cec2"
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
>   "address": "bcrt1qvv37atnet3e67ugv4ku86q9vpgducuv9de9ly3"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> 4cad4f8bd694939cdcaa76efbf8eb7b2af2fd631cc9df8e130ac92ed54d4625a
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "0a7bd3dcfa454a90658f59435e388badbcac383591bb96a8e10bc50473ae338c"
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
>   "address": "bcrt1qjwwr2vtwnyzun3ttxnk5n32xchcfjlrlzdyhwp"
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
>   "address": "bcrt1qusr2zrv65phjktvcyw05cmhv6tddpmn9ammskk"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO d2a6891a38ec5d6d8e8e582972f7c4454f92b30c22af15f18bcd3eef22fbb78f:2.
> Broadcast transaction: be4eaa8d8ba3c28daec04858dd03177a118a0f0bd206c8e47a1d9d85d90f6a7e
> Replacement output: bcrt1qjwwr2vtwnyzun3ttxnk5n32xchcfjlrlzdyhwp with 10,029,800 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM_be4eaa8d8ba3c28daec04858dd03177a118a0f0bd206c8e47a1d9d85d90f6a7e
> Change output: bcrt1qusr2zrv65phjktvcyw05cmhv6tddpmn9ammskk with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "0211f525ed2276644a5bde05f777094ebee8dcc4f0893bfa2a5b8b289560f246",
>   "434f1f0dc63fff9700719e9a097f2787c0c96b375700e5ff35326f34a027d530",
>   "1e58751fee3e1446c3ac42164f6fa8cdde47106c5c066c78fe895d00b8f557ae",
>   "44a73430ceb30b4052a66c51802f3999ca2ca46a585cb16c68ae9c6aeb6d2335",
>   "57f316951bc6e50502d5e5d6408e1eec259f8d1747f678f3cd83559c9b5831e4",
>   "36fa4b5799c298fddbeb4900ebe1e12498f55343eece8df406efb9a1f473b57b"
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
>   "address": "bcrt1qxxqk2fcygru4ea4audl8kx6wkd6f66n86m3drz"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO dd948d2e00b6ed7a668c4d44d7df42cb7b7f48e38d9a83b1559d6312ff06f5f0:0.
> Broadcast transaction: 6c7331969ac996977a744749e66cae31d22fc08f0681e7debd2a0d60d8e68d96
> Sent 27,889 sats to bcrt1qxxqk2fcygru4ea4audl8kx6wkd6f66n86m3drz.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "619d1949815a434fee1d78ec61fb43af8e5a214fb4c7acc39fa0b9dc2f0bfb80",
>   "3b43116b23efbbd93663437fa91caabb07c9b89526a85bc2f70293d85de0be95",
>   "4767e2fd591dbb17c3632a9ba674588af564cb7cc02f5d3bfbf2c1406d1d2d08",
>   "36574c8c82cddc84d1876c958cfdcbcf89a522a2ca5fc016ab1ff9bc47011585",
>   "1df0a4f740089e4604b7db7fef8d62307642742ca9af3a2c5ac7590f5461de65",
>   "4810e791ba7eeb0f8a4d4b0ccee7eece3a0a7799d819bf5596e6afb1ab3162c1"
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
>   "swap_string": "rgb-swap:v1:eyJxdW90ZSI6eyJleGVjdXRpb25faWQiOiJmNzhhYmY2YzZlZjFiOGFiMTE1MmZhNmYxYzI0ZDhhZjQwMWU2MGQ0NjI0YTViMDE2YTc0MmM4NTAwYWI5MzE0IiwibWFrZXIiOiIwMzgzMzAwYTgyZTA1Mjc2MmFmZmIyZDQ5MWQzMzQ4NWIzMmY4ZWYxNmU3ZmI5N2Q4MjU5OTI1OTQyMGIwM2RjZGEiLCJ0YWtlciI6IjAzMThhOTZjMmFlYjIzYTgyYzZlN2M4ZGJkNmY4ODhmMzZjNzI4YThjYWE3ZjFjM2M5Y2M5OWYxYjdjYmZmMzg2ZSIsInRha2VyX291dGJvdW5kX2NoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSIsImFzc2V0X2lkIjpbMTQ3LDE4MSwyMTQsMjEsMTgzLDEyMSwxOTYsODQsMTQzLDkyLDE0NywxMjcsMTYxLDE0MiwyNDMsNzYsMTksMTk1LDE3OSwyMzgsMTcwLDEwOCwyNCwyMTgsMTMsMTU2LDgsNjMsNzUsMTg0LDkyLDE5NV0sImFzc2V0X2Ftb3VudCI6IjIiLCJidGNfYW1vdW50X21zYXQiOiIxMDAwMDAwMCIsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoiNTAwMDAwMCIsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiY3JlYXRlZF9hdF91bml4X3NlY3MiOiIxNzk0ODE2Nzg2IiwiZXhwaXJ5X3NlY3MiOjM2MDAsInNldHRsZW1lbnRfbW9kZSI6IkNpcmN1bGFySHRsYyIsIm1ha2VyX3RvX3Rha2VyX2ZlZV9wYXllciI6Ik1ha2VyIiwidGFrZXJfdG9fbWFrZXJfZmVlX3BheWVyIjoiTWFrZXIiLCJjb25zdHJhaW50cyI6eyJtYWtlcl90b190YWtlciI6eyJmaXhlZF9yb3V0ZSI6W3sibm9kZV9pZCI6IjAzMThhOTZjMmFlYjIzYTgyYzZlN2M4ZGJkNmY4ODhmMzZjNzI4YThjYWE3ZjFjM2M5Y2M5OWYxYjdjYmZmMzg2ZSIsImNoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSJ9XSwicmVxdWlyZWRfbm9kZXMiOlsiMDMxOGE5NmMyYWViMjNhODJjNmU3YzhkYmQ2Zjg4OGYzNmM3MjhhOGNhYTdmMWMzYzljYzk5ZjFiN2NiZmYzODZlIl0sInJlcXVpcmVkX2NoYW5uZWxzIjpbIjE1NjEzMDY1MTIwOTcyOSJdLCJyZXF1aXJlZF9sc3BzIjpbXSwibWF4X2ZlZV9tc2F0IjoiMCJ9LCJ0YWtlcl90b19tYWtlciI6eyJmaXhlZF9yb3V0ZSI6W3sibm9kZV9pZCI6IjAzODMzMDBhODJlMDUyNzYyYWZmYjJkNDkxZDMzNDg1YjMyZjhlZjE2ZTdmYjk3ZDgyNTk5MjU5NDIwYjAzZGNkYSIsImNoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSJ9XSwicmVxdWlyZWRfbm9kZXMiOlsiMDM4MzMwMGE4MmUwNTI3NjJhZmZiMmQ0OTFkMzM0ODViMzJmOGVmMTZlN2ZiOTdkODI1OTkyNTk0MjBiMDNkY2RhIl0sInJlcXVpcmVkX2NoYW5uZWxzIjpbIjE1NjEzMDY1MTIwOTcyOSJdLCJyZXF1aXJlZF9sc3BzIjpbXSwibWF4X2ZlZV9tc2F0IjoiMCJ9LCJtYXhfdG90YWxfY2x0dl9kZWx0YSI6MjAxNn19LCJwYXltZW50X2hhc2hfaGV4IjoiZjc4YWJmNmM2ZWYxYjhhYjExNTJmYTZmMWMyNGQ4YWY0MDFlNjBkNDYyNGE1YjAxNmE3NDJjODUwMGFiOTMxNCIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAzMThhOTZjMmFlYjIzYTgyYzZlN2M4ZGJkNmY4ODhmMzZjNzI4YThjYWE3ZjFjM2M5Y2M5OWYxYjdjYmZmMzg2ZSIsImNoYW5uZWxfc2NpZCI6MTU2MTMwNjUxMjA5NzI5LCJjb250cmFjdF9pZCI6ImNvbnRyYWN0Oms3WFdGYmQ1LXhGU1BYSk4tfm9ZN3pUQi1QRHNfNnFiLUJqYURad0ktUDB1NFhNTSIsImFzc2V0X2Ftb3VudCI6MiwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwMCwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOjUwMDAwMDAsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiZXhwaXJ5X3NlY3MiOjM2MDAsImNyZWF0ZWRfYXRfdW5peF9zZWNzIjoxNzk0ODE2Nzg2fQ",
>   "payment_hash": "f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314",
>   "info": {
>     "payment_hash": "f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e",
>     "channel_scid": "156130651209729",
>     "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>     "asset_amount": "2",
>     "btc_amount_msat": "10000000",
>     "btc_carrier_amount_msat": "5000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1794816786",
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
>   "payment_hash": "f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314",
>   "role": "Taker",
>   "status": "Offered",
>   "counterparty_node_id": "0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda",
>   "channel_scid": "156130651209729",
>   "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1794816786",
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
>   "payment_hash": "f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314",
>   "role": "Taker",
>   "status": "Accepted",
>   "counterparty_node_id": "0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda",
>   "channel_scid": "156130651209729",
>   "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1794816786",
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
$ rgbldk swap get f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Accepted                                                         |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 0318a96c...cbff386e                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 156130651209729                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-P0u4XMM                                              |
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
> | Created (unix secs) | 1794816786                                                       |
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
>   "payment_hash": "f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314",
>   "status": "InFlight"
> }
>
> ```

**Run:**

```bash
$ rgbldk swap get f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 0318a96c...cbff386e                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 156130651209729                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-P0u4XMM                                              |
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
> | Created (unix secs) | 1794816786                                                       |
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
$ rgbldk swap get f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Taker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 0383300a...0b03dcda                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 156130651209729                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-P0u4XMM                                              |
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
> | Created (unix secs) | 1794816786                                                       |
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
> | f78abf6c6ef1b8ab1152fa6f1c24d8af401e60d4624a5b016a742c8500ab9314 | Taker | Settled | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM | 2     | RGB         |
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
>   "swap_string": "rgb-swap:v1:eyJxdW90ZSI6eyJleGVjdXRpb25faWQiOiJjYzQxNTE4NDBiY2YxMDc1YmExYzI2M2ZmN2NhYTZlZDYwZGRlN2M5YTFkZWMzM2YzMGMzMDJjNzcxMDNhNDUwIiwibWFrZXIiOiIwMzgzMzAwYTgyZTA1Mjc2MmFmZmIyZDQ5MWQzMzQ4NWIzMmY4ZWYxNmU3ZmI5N2Q4MjU5OTI1OTQyMGIwM2RjZGEiLCJ0YWtlciI6IjAzMThhOTZjMmFlYjIzYTgyYzZlN2M4ZGJkNmY4ODhmMzZjNzI4YThjYWE3ZjFjM2M5Y2M5OWYxYjdjYmZmMzg2ZSIsInRha2VyX291dGJvdW5kX2NoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSIsImFzc2V0X2lkIjpbMTQ3LDE4MSwyMTQsMjEsMTgzLDEyMSwxOTYsODQsMTQzLDkyLDE0NywxMjcsMTYxLDE0MiwyNDMsNzYsMTksMTk1LDE3OSwyMzgsMTcwLDEwOCwyNCwyMTgsMTMsMTU2LDgsNjMsNzUsMTg0LDkyLDE5NV0sImFzc2V0X2Ftb3VudCI6IjEiLCJidGNfYW1vdW50X21zYXQiOiIxMDAwMDAwIiwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOiIxMDAwMDAwIiwibWFrZXJfZ2l2ZXNfcmdiIjp0cnVlLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6IjE3OTQ4MTY3ODgiLCJleHBpcnlfc2VjcyI6MzYwMCwic2V0dGxlbWVudF9tb2RlIjoiQ2lyY3VsYXJIdGxjIiwibWFrZXJfdG9fdGFrZXJfZmVlX3BheWVyIjoiTWFrZXIiLCJ0YWtlcl90b19tYWtlcl9mZWVfcGF5ZXIiOiJNYWtlciIsImNvbnN0cmFpbnRzIjp7Im1ha2VyX3RvX3Rha2VyIjp7ImZpeGVkX3JvdXRlIjpbeyJub2RlX2lkIjoiMDMxOGE5NmMyYWViMjNhODJjNmU3YzhkYmQ2Zjg4OGYzNmM3MjhhOGNhYTdmMWMzYzljYzk5ZjFiN2NiZmYzODZlIiwiY2hhbm5lbF9zY2lkIjoiMTU2MTMwNjUxMjA5NzI5In1dLCJyZXF1aXJlZF9ub2RlcyI6WyIwMzE4YTk2YzJhZWIyM2E4MmM2ZTdjOGRiZDZmODg4ZjM2YzcyOGE4Y2FhN2YxYzNjOWNjOTlmMWI3Y2JmZjM4NmUiXSwicmVxdWlyZWRfY2hhbm5lbHMiOlsiMTU2MTMwNjUxMjA5NzI5Il0sInJlcXVpcmVkX2xzcHMiOltdLCJtYXhfZmVlX21zYXQiOiIwIn0sInRha2VyX3RvX21ha2VyIjp7ImZpeGVkX3JvdXRlIjpbeyJub2RlX2lkIjoiMDM4MzMwMGE4MmUwNTI3NjJhZmZiMmQ0OTFkMzM0ODViMzJmOGVmMTZlN2ZiOTdkODI1OTkyNTk0MjBiMDNkY2RhIiwiY2hhbm5lbF9zY2lkIjoiMTU2MTMwNjUxMjA5NzI5In1dLCJyZXF1aXJlZF9ub2RlcyI6WyIwMzgzMzAwYTgyZTA1Mjc2MmFmZmIyZDQ5MWQzMzQ4NWIzMmY4ZWYxNmU3ZmI5N2Q4MjU5OTI1OTQyMGIwM2RjZGEiXSwicmVxdWlyZWRfY2hhbm5lbHMiOlsiMTU2MTMwNjUxMjA5NzI5Il0sInJlcXVpcmVkX2xzcHMiOltdLCJtYXhfZmVlX21zYXQiOiIwIn0sIm1heF90b3RhbF9jbHR2X2RlbHRhIjoyMDE2fX0sInBheW1lbnRfaGFzaF9oZXgiOiJjYzQxNTE4NDBiY2YxMDc1YmExYzI2M2ZmN2NhYTZlZDYwZGRlN2M5YTFkZWMzM2YzMGMzMDJjNzcxMDNhNDUwIiwiY291bnRlcnBhcnR5X25vZGVfaWRfaGV4IjoiMDMxOGE5NmMyYWViMjNhODJjNmU3YzhkYmQ2Zjg4OGYzNmM3MjhhOGNhYTdmMWMzYzljYzk5ZjFiN2NiZmYzODZlIiwiY2hhbm5lbF9zY2lkIjoxNTYxMzA2NTEyMDk3MjksImNvbnRyYWN0X2lkIjoiY29udHJhY3Q6azdYV0ZiZDUteEZTUFhKTi1-b1k3elRCLVBEc182cWItQmphRFp3SS1QMHU0WE1NIiwiYXNzZXRfYW1vdW50IjoxLCJidGNfYW1vdW50X21zYXQiOjEwMDAwMDAsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImV4cGlyeV9zZWNzIjozNjAwLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6MTc5NDgxNjc4OH0",
>   "payment_hash": "cc4151840bcf1075ba1c263ff7caa6ed60dde7c9a1dec33f30c302c77103a450",
>   "info": {
>     "payment_hash": "cc4151840bcf1075ba1c263ff7caa6ed60dde7c9a1dec33f30c302c77103a450",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e",
>     "channel_scid": "156130651209729",
>     "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1794816788",
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
>   "swap_string": "rgb-swap:v3:eyJxdW90ZSI6eyJleGVjdXRpb25faWQiOiJmOThjZWRlNGRlYmQwMjg0ODliNTU5N2Y5YjU4ODBiMGQyMmY4YmYzODRkYWQ4MmYzNDFiMWU2MjVhMmZkYmViIiwibWFrZXIiOiIwMzgzMzAwYTgyZTA1Mjc2MmFmZmIyZDQ5MWQzMzQ4NWIzMmY4ZWYxNmU3ZmI5N2Q4MjU5OTI1OTQyMGIwM2RjZGEiLCJ0YWtlciI6IjAzMThhOTZjMmFlYjIzYTgyYzZlN2M4ZGJkNmY4ODhmMzZjNzI4YThjYWE3ZjFjM2M5Y2M5OWYxYjdjYmZmMzg2ZSIsInRha2VyX291dGJvdW5kX2NoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSIsImFzc2V0X2lkIjpbMTQ3LDE4MSwyMTQsMjEsMTgzLDEyMSwxOTYsODQsMTQzLDkyLDE0NywxMjcsMTYxLDE0MiwyNDMsNzYsMTksMTk1LDE3OSwyMzgsMTcwLDEwOCwyNCwyMTgsMTMsMTU2LDgsNjMsNzUsMTg0LDkyLDE5NV0sImFzc2V0X2Ftb3VudCI6IjEiLCJidGNfYW1vdW50X21zYXQiOiIxMDAwMDAwIiwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOiIxMDAwMDAwIiwibWFrZXJfZ2l2ZXNfcmdiIjp0cnVlLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6IjE3OTQ4MTY3ODgiLCJleHBpcnlfc2VjcyI6MzYwMCwic2V0dGxlbWVudF9tb2RlIjoiQ2lyY3VsYXJIdGxjIiwibWFrZXJfdG9fdGFrZXJfZmVlX3BheWVyIjoiTWFrZXIiLCJ0YWtlcl90b19tYWtlcl9mZWVfcGF5ZXIiOiJNYWtlciIsImNvbnN0cmFpbnRzIjp7Im1ha2VyX3RvX3Rha2VyIjp7ImZpeGVkX3JvdXRlIjpbeyJub2RlX2lkIjoiMDMxOGE5NmMyYWViMjNhODJjNmU3YzhkYmQ2Zjg4OGYzNmM3MjhhOGNhYTdmMWMzYzljYzk5ZjFiN2NiZmYzODZlIiwiY2hhbm5lbF9zY2lkIjoiMTU2MTMwNjUxMjA5NzI5In1dLCJyZXF1aXJlZF9ub2RlcyI6WyIwMzE4YTk2YzJhZWIyM2E4MmM2ZTdjOGRiZDZmODg4ZjM2YzcyOGE4Y2FhN2YxYzNjOWNjOTlmMWI3Y2JmZjM4NmUiXSwicmVxdWlyZWRfY2hhbm5lbHMiOlsiMTU2MTMwNjUxMjA5NzI5Il0sInJlcXVpcmVkX2xzcHMiOltdLCJtYXhfZmVlX21zYXQiOiIwIn0sInRha2VyX3RvX21ha2VyIjp7ImZpeGVkX3JvdXRlIjpbeyJub2RlX2lkIjoiMDM4MzMwMGE4MmUwNTI3NjJhZmZiMmQ0OTFkMzM0ODViMzJmOGVmMTZlN2ZiOTdkODI1OTkyNTk0MjBiMDNkY2RhIiwiY2hhbm5lbF9zY2lkIjoiMTU2MTMwNjUxMjA5NzI5In1dLCJyZXF1aXJlZF9ub2RlcyI6WyIwMzgzMzAwYTgyZTA1Mjc2MmFmZmIyZDQ5MWQzMzQ4NWIzMmY4ZWYxNmU3ZmI5N2Q4MjU5OTI1OTQyMGIwM2RjZGEiXSwicmVxdWlyZWRfY2hhbm5lbHMiOlsiMTU2MTMwNjUxMjA5NzI5Il0sInJlcXVpcmVkX2xzcHMiOltdLCJtYXhfZmVlX21zYXQiOiIwIn0sIm1heF90b3RhbF9jbHR2X2RlbHRhIjoyMDE2fX0sInBheW1lbnRfaGFzaF9oZXgiOiJmOThjZWRlNGRlYmQwMjg0ODliNTU5N2Y5YjU4ODBiMGQyMmY4YmYzODRkYWQ4MmYzNDFiMWU2MjVhMmZkYmViIiwibWFrZXJfdG9fdGFrZXIiOlt7Im5vZGVfaWRfaGV4IjoiMDMxOGE5NmMyYWViMjNhODJjNmU3YzhkYmQ2Zjg4OGYzNmM3MjhhOGNhYTdmMWMzYzljYzk5ZjFiN2NiZmYzODZlIiwiY2hhbm5lbF9zY2lkIjoxNTYxMzA2NTEyMDk3Mjl9XSwidGFrZXJfdG9fbWFrZXIiOlt7Im5vZGVfaWRfaGV4IjoiMDM4MzMwMGE4MmUwNTI3NjJhZmZiMmQ0OTFkMzM0ODViMzJmOGVmMTZlN2ZiOTdkODI1OTkyNTk0MjBiMDNkY2RhIiwiY2hhbm5lbF9zY2lkIjoxNTYxMzA2NTEyMDk3Mjl9XSwiY29udHJhY3RfaWQiOiJjb250cmFjdDprN1hXRmJkNS14RlNQWEpOLX5vWTd6VEItUERzXzZxYi1CamFEWndJLVAwdTRYTU0iLCJhc3NldF9hbW91bnQiOjEsImJ0Y19hbW91bnRfbXNhdCI6MTAwMDAwMCwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOjEwMDAwMDAsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiZXhwaXJ5X3NlY3MiOjM2MDAsImNyZWF0ZWRfYXRfdW5peF9zZWNzIjoxNzk0ODE2Nzg4fQ",
>   "payment_hash": "f98cede4debd028489b5597f9b5880b0d22f8bf384dad82f341b1e625a2fdbeb",
>   "info": {
>     "payment_hash": "f98cede4debd028489b5597f9b5880b0d22f8bf384dad82f341b1e625a2fdbeb",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e",
>     "channel_scid": "156130651209729",
>     "contract_id": "contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1794816788",
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
>   "invoice": "lnbcrt100n1p404wc4dq8v3jk6mcnp4qvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxupp5fy7n6wr9wq88vqqqmm35523ysraau9220zkx0frhles8uvsfyymssp542nv6ynqx6nrhe34zm8ew59tasphya5xzgk70t70kx8a0w8ax4dq9qyysgqcqzp2xqrrssrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqnvgqqqqqqqlgqqqqqqqqfqpgyp2f4j2jr9cesu76mgdv650skmspug8f4gqsu304y8tqfctjvryssk8xnpjgxk22ygj3dtedxrcvkx79rjjr87fpk8mga0d9afqusqr6xlwm"
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
> | payment_hash | 493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e |
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
>   "payment_id": "493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137",
>   "preimage": "69e66f5bc95a18d43d5fc7aebf631d0a97e2dfed9e4185511cd67df4263ec143",
>   "amount_sats": "10",
>   "destination": "0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137
>
> ```

**Run:**

```bash
$ rgbldk pay get 493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137

```

**Result:**

> ```text
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                         |
> +========================================================================================================================================================================================================================================================================+
> | id                     | 493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                      |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                        |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816790                                                                                                                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                             |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"493d3d3865700e760000dee34a2a2480fbde154a78ac67a477fe607e32092137","preimage":"69e66f5bc95a18d43d5fc7aebf631d0a97e2dfed9e4185511cd67df4263ec143","secret":"aaa6cd126036a63be63516cf9750abec03727686122de7afcfb18fd7b8fd355a"} |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 10,000 msat                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                        |
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt130n1p404wckdq0v3jk6medwdjkueqnp4qvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxupp58sc5spv0asf88e6st7wjgxnymvxwv9nt755zj3clldr0rnlggr2qsp5yn8zaaehz0q26zkvywwkyv7wa4f38rp2sd52ps8t3dvhwlhsyl7s9qyysgqcqzp2xqrrssrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqnvgqqqqqqqlgqqqqqqqqfqw4anjmyzv4yg3epslxz5kcugr2c9lnnwkqm7nvextxhe259hslf3j94upm7urewq6esjghlx2rp0lykefaj9fcpvfdzrxmd5kfmhjmspfypamf"
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
>   "payment_id": "3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4
>
> ```

**Run:**

```bash
$ rgbldk pay get 3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4

```

**Result:**

> ```text
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                         |
> +========================================================================================================================================================================================================================================================================+
> | id                     | 3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                      |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                        |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816791                                                                                                                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                             |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"3c3148058fec1273e7505f9d241a64db0ce6166bf52829471ffb46f1cfe840d4","preimage":"462719da7293a5480879aacd08d5b9478534702198bdd269f70ee75844dd0257","secret":"24ce2ef73713c0ad0acc239d6233ceed53138c2a8368a0c0eb8b59777ef027fd"} |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 13,000 msat                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                        |
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt1p404wchdqdv3jk6medweshynp4qvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxupp5el72pk0235gz4et8tght3pmfzjrusxlwyt2epje2p9lzgk7037yssp5573ur5mp8d63javzs9z0fzjt94gnz5pk9jg2n5h5t7e840zdaahs9qyysgqcqzp2xqrrssrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqxusqqvqqqqlgqqqqqqqqfqhfxuwpdl45n2ztpwvhmz65wggs9rj54lhxnd93fxkz3eaa6cpw5jgfxxjnclcaxekw5ecvvuqa28hkk0yqta5z048k4qdwjs79ef8pgpaqvfwj"
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
>   "payment_id": "cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89
>
> ```

**Run:**

```bash
$ rgbldk pay get cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89

```

**Result:**

> ```text
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                         |
> +========================================================================================================================================================================================================================================================================+
> | id                     | cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                      |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                        |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816792                                                                                                                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                             |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"cffca0d9ea8d102ae5675a2eb887691487c81bee22d590cb2a097e245bcf8f89","preimage":"2d13af7b9ed83a9bc709dfd518b07b64e749a2848861dd400ce16feed5a2b633","secret":"a7a3c1d3613b751975828144f48a4b2d513150362c90a9d2f45fb27abc4def6f"} |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 11,000 msat                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                        |
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt140n1p404wcedqcv3jk6meddphkcepdvdkxz6tdnp4qvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxupp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp50vwku4lehrtakxyeuztc0gn7czaeftunmhjyglvlnv74qad93t3s9qyysgqcqzp2xqrrssrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqnvgqqqqqqqlgqqqqqqqqfq43pmyxlmlr49nqj60t4qps80mj38xl6csn4snkck4j63j0gwz8s90vmw2jejhs7w0xmrek9uzk8a29z6tf40lkkpr8esvr82250g3pspyv8l6m"
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
> | destination  | 0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e |
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
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                         |
> +========================================================================================================================================================================================================================================================================+
> | id                     | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                      |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                        |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816794                                                                                                                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                             |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":"4242424242424242424242424242424242424242424242424242424242424242","secret":"7b1d6e57f9b8d7db1899e09787a27ec0bb94af93dde4447d9f9b3d5075a58ae3"} |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 14,000 msat                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                        |
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt150n1p404wcmdqhv3jk6meddphkcepdveskjmqnp4qvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxupp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp5l5vzx04eqpm39ac4qz4td808tazwugp8s4m8gcv90fzy5w050tqq9qyysgqcqzp2xqrrssrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqnvgqqqqqqqlgqqqqqqqqfqm4p9w0ts866hwcvrv0f4cxun3gzxh52cvrz007km40mseem4pyexndqgrzn7xz6pz4lh43erjkpfqelm2vu7wkz67plvkvc285jjjcsqpsz33s"
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
> +------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                           |
> +==========================================================================================================================================================================================================+
> | id                     | 4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24                                                                                                                |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24                                                                                                                |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                        |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✘ Failed                                                                                                                                                                        |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                          |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                           |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816797                                                                                                                                                                      |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                               |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"fd18233eb9007712f71500aab69de75f44ee202785767461857a444a39f47ac0"} |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 15,000 msat                                                                                                                                                                     |
> |------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | -                                                                                                                                                                               |
> +------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "invoice": "lnbcrt120n1p404wc7dq0v3jk6medvfskx6cnp4qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5pp5nrhqm5xpp8ers30pp370fszrqnq4w74ymclfyc4fwg6ume52cmgssp5rcsq5gv58xc50gdv53w5w4vs97ehxxxagxmzu3g0t7lap8dq9zas9qyysgqcqzp2xqrrssrzjqvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxuqqqqyqqs0sqqcqqqqlgqqqqqqqqfqmqmapaunpceu00a3lh8vjgpurfgtpcmk7777rugtzjah365a2qxns59n6rjm0gz0dqw3s2ym5l4a7s2h6448h40rx6p00c8yklqvg2cq70qlmg"
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
>   "payment_id": "98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1",
>   "preimage": "e229be6baac4a5e9f1ae21a5938277957ceb8461c4762ab9b9d4b0294de80b27",
>   "amount_sats": "12",
>   "destination": "0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1
>
> ```

**Run:**

```bash
$ rgbldk pay get 98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1

```

**Result:**

> ```text
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                         |
> +========================================================================================================================================================================================================================================================================+
> | id                     | 98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1                                                                                                                                                                              |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                      |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                        |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816799                                                                                                                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                             |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"98ee0dd0c109f23845e10c7cf4c04304c1577aa4de3e9262a97235cde68ac6d1","preimage":"e229be6baac4a5e9f1ae21a5938277957ceb8461c4762ab9b9d4b0294de80b27","secret":"1e200a219439b147a1aca45d4755902fb37318dd41b62e450f5fbfd09da028bb"} |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 12,000 msat                                                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                        |
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx47kf9ugd5qcc49kz46er4qkxulydh4hc3rekcu523j4878punnye7xmuhlecdcpks4ljuxldts57g87c5956yvph76zyhmt3fppcemczgqvsfqe5n7cpqwwzur9fqezhfly3v892fakywcqn6r8he026wzdfdnxy9kqhqhe2wqr5cz73dmltk96k3td7e66afmwwvsandyvzxqfkrgsyxjf9u4mk49p862jnhawxscyqtawf4ll5277sj4lde9avhv57xkuc36hxxw5jnyq4wyus80dhx22ljms6yqlv8jrrqhfw02xgcq3xfvfgltpgvd5hz2aeuzpts495xglae5dvhw3usdrxfk559vtzzqc5c0yppmgf754q5zwckn93laqr9zpksqdw0na56xk3pffxnsgmrq"
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
> | offer_id                  | f9df46f367939d41585a752a1ace46b4d752b558412c6051f1da4bb90ace8f69   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 0314c3c810ed09f52a0a09d8b4cb1ff40328836801ae7cfb4d1ad10a5269c11b18 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1794820399                                                         |
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
>   "payment_id": "62306e655282806edad5c6db524a06fcd0eb588387b80727409cedfca36dde85"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 62306e655282806edad5c6db524a06fcd0eb588387b80727409cedfca36dde85 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 62306e655282806edad5c6db524a06fcd0eb588387b80727409cedfca36dde85
>
> ```

**Run:**

```bash
$ rgbldk pay get 62306e655282806edad5c6db524a06fcd0eb588387b80727409cedfca36dde85

```

**Result:**

> ```text
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                               |
> +==============================================================================================================================================================================================================================+
> | id                     | 62306e655282806edad5c6db524a06fcd0eb588387b80727409cedfca36dde85                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 402f570466309a9a8b8fd58318c329c91d96b2abe92ef06e8548448994f2be6c                                                                                                                                    |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                            |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt12Offer                                                                                                                                                                                         |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                               |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816800                                                                                                                                                                                          |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                   |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"offer_id":"f9df46f367939d41585a752a1ace46b4d752b558412c6051f1da4bb90ace8f69","payer_note":null,"payment_hash":"402f570466309a9a8b8fd58318c329c91d96b2abe92ef06e8548448994f2be6c","quantity":null} |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 5,555 msat                                                                                                                                                                                          |
> |------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                              |
> +------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "refund": "lnr1qqsz00wdgpsvv2gqvx8aat96xnwdcg8lh5y9w8yh2wgs0lnlrak6l3s2qq8qg6h6eyc9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssyuhmvshjp7qm0s5gnqtux8tzxxkqpgcyuhx5rgzpyfxkz8cdnmn9ty9hyetxw4hxgttyv4kk7kk6qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qlec9y9rsqgpjp6x58xzct2nhq2320g0je9a050xyat2g8zwf8hhcqsyvncdyarflm5xqdkhhk94nj6l4np0ukzc35xf3atvjwh269j000gqp6vdlmaf9rr69yqlnqrwevgzq4e78eskty7njfqjv2pzge8epp8hkpz2vwqra8umgxjk9nnzxf6cffhqn34fexxgfhc93szqfyrt5r8mqpda5rt4g0h5k3ddnqc098gafae3wjlzns0gn0aehmknetah2cnnw7lu0srswx7y60k9znmslax2r5y86ddx",
>   "payment_id": "67e2267378f0a5716256982eae93d5c1226fb20368ed4f00b607eb671f4dcecb"
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
> | absolute_expiry_unix_secs | 1794820400                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 0272fb642f20f81b7c2889817c31d6231ac00a304e5cd41a041224d611f0d9ee65 |
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
> lni1qqsz00wdgpsvv2gqvx8aat96xnwdcg8lh5y9w8yh2wgs0lnlrak6l3s2qq8qg6h6eyc9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssyuhmvshjp7qm0s5gnqtux8tzxxkqpgcyuhx5rgzpyfxkz8cdnmn9ty9hyetxw4hxgttyv4kk7kk6qwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qlec9y9rsqgpjp6x58xzct2nhq2320g0je9a050xyat2g8zwf8hhcqsyvncdyarflm5xqdkhhk94nj6l4np0ukzc35xf3atvjwh269j000gqp6vdlmaf9rr69yqlnqrwevgzq4e78eskty7njfqjv2pzge8epp8hkpz2vwqra8umgxjk9nnzxf6cffhqn34fexxgfhc93szqfyrt5r8mqpda5rt4g0h5k3ddnqc098gafae3wjlzns0gn0aehmknetah2cnnw7lu0srswx7y60k9znmslax2r5y86dd8g8wqvv2jmp2av36strw0jxm6mug3umvw29ge2nlrs7fejvlrd7tluuxuq32k53xca9upvehgssq2c5hl593hyv6vhq7682w6dprkmgnxt380yqsyxfj0nfs75ls2q5cxyqwn3era5dk2k9gu2efadly5j5f57f5j6v3qzyvfcrmcmkhsflhm79tjv596x5m80gcf624duezwlvxgg6e4jmnpyfwgrtcrgyt6jxugyxdxqehr2d6qn6hmxaywfd5yw6ljdcxw4802p42650jhfhxzqescujmqt4p45zqvja68fejaypmk7t2u6p6gne228v8y5jxrnllkgqa99upsqk8jawdy6e8nnfwn20swsqc3tmd9g65atwlcvxmn05v0gsuqqqqqqqqqqqqqqp2qqqqqqqqqqqqz8fykt06c5sqqqqqpfqydtatkgdgyrp9fpmzh40z43eaeqavlvgmmf2h37e3eqnrkgykpddhth2tdsm6p2szzrs6uqczqqqtqggzwm06cqudnqscxfvjafnckrwkgnafhyke5366uct9qsyj4lgafhqlqsznvlz8a3mcvg5qu9hzgy0szgdzngav8xv0hrdh4c7a8nulm5r3gpe9fmg8e3nykgwsfx9ma696jwherrqwf57d2dyuj4qls39aa4g5q
> payment_id: c2548762bd5e2ac73dc83acfb11bda5578fb31c8263b20960b5b75dd4b6c37a0
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
$ rgbldk pay wait 67e2267378f0a5716256982eae93d5c1226fb20368ed4f00b607eb671f4dcecb --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 67e2267378f0a5716256982eae93d5c1226fb20368ed4f00b607eb671f4dcecb
>
> ```

**Run:**

```bash
$ rgbldk pay get 67e2267378f0a5716256982eae93d5c1226fb20368ed4f00b607eb671f4dcecb

```

**Result:**

> ```text
> +------------------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                          |
> +=========================================================================================================================================================+
> | id                     | 67e2267378f0a5716256982eae93d5c1226fb20368ed4f00b607eb671f4dcecb                                                               |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | c2548762bd5e2ac73dc83acfb11bda5578fb31c8263b20960b5b75dd4b6c37a0                                                               |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                       |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                    |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt12Refund                                                                                                                   |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                          |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816802                                                                                                                     |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                              |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payer_note":"refund-demo","payment_hash":"c2548762bd5e2ac73dc83acfb11bda5578fb31c8263b20960b5b75dd4b6c37a0","quantity":null} |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 4,321 msat                                                                                                                     |
> |------------------------+--------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                         |
> +------------------------+--------------------------------------------------------------------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty pay refund initiate --amount-msat 1111 --payer-note refund-abandon-demo

```

**Result:**

> ```text
> {
>   "refund": "lnr1qqsqhm2ryfnyyhp7s8rxztzcs95t8tmfujkacpqwmgz85dp06m25r0g2qq8qg6h6eye9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqss9mfw7jg7ld45kmjge65952xw2emyuyahr2smcey09k07flalc8uhtyfhyetxw4hxgttpvfskuer0dckkgetddadd5qurxq9g9czjwc40lvk5j8fnfpdn9780zmnlh97cykvjt9pqkq7umgpn5emvafttygp3djy7u2cgem0wqgy3vucuce7rcluexdvcjh4rwrcpqf0fwqkswyvm6wm2q3w7d0nzaf8zfjscd8wtstgjqm50atrtpt3ekqr5zdrsahnt9wcawk7z9z03ugnrcc2d0t60semq4t3ulpfzl58vys8fcwr2szclp6htkedfwaf5x6n5en9et0sukf6uepdcx90a33mw6hzkpxqql3m7zp6fdy4vyklph2geggr2v8kjs0dgv0asgahet03q8nz54tsgh970v5qukwmehnz3d796xzww8v",
>   "payment_id": "f738b8dedad73246a349bb064cc0a9238180749a1db273c2b3155480ce39f40e"
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
> | 9e7daaea...1d99f4a2 | -                   | Succeeded | Onchain      | Outbound | 100,000,000     | 9,232,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | cffca0d9...5bcf8f89 | cffca0d9...5bcf8f89 | Succeeded | Bolt11       | Outbound | 11,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 67e22673...1f4dcecb | c2548762...4b6c37a0 | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 37cdc78d...285b1ae4 | -                   | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 3c314805...cfe840d4 | 3c314805...cfe840d4 | Succeeded | Bolt11       | Outbound | 13,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 561825ba...86b0b36f | 561825ba...86b0b36f | Pending   | Bolt11       | Inbound  | 10,084,000      | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | f738b8de...ce39f40e | -                   | Failed    | Bolt12Refund | Outbound | 1,111           | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 94372ffc...cbbf40f0 | -                   | Succeeded | Onchain      | Inbound  | 10,084,000      | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 62306e65...a36dde85 | 402f5704...94f2be6c | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 8c8e4dbd...f55d679d | 8c8e4dbd...f55d679d | Pending   | Bolt11       | Inbound  | 10,084,000      | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | a8cb1597...93650b8f | a8cb1597...93650b8f | Pending   | Bolt11       | Inbound  | 10,084,000      | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 98ee0dd0...e68ac6d1 | 98ee0dd0...e68ac6d1 | Succeeded | Bolt11       | Inbound  | 12,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 425ed4e4...6e55384c | 425ed4e4...6e55384c | Succeeded | Bolt11       | Outbound | 14,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 493d3d38...32092137 | 493d3d38...32092137 | Succeeded | Bolt11       | Outbound | 10,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4113d54b...cc0b3a24 | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | c0723ba7...ae4f0b23 | -                   | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  | false       |
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
> +---------------------------------+----------------+
> | Field                           | Value          |
> +==================================================+
> | Receive available               | yes            |
> |---------------------------------+----------------|
> | Can create RGB invoice          | yes            |
> |---------------------------------+----------------|
> | Blocking reason                 | -              |
> |---------------------------------+----------------|
> | Required carrier (msat)         | 355,000        |
> |---------------------------------+----------------|
> | Required carrier reason         | holder_reserve |
> |---------------------------------+----------------|
> | Available inbound (msat)        | 98,340,000     |
> |---------------------------------+----------------|
> | Suggested action                | -              |
> |---------------------------------+----------------|
> | Minimum viable carrier (msat)   | 355,000        |
> |---------------------------------+----------------|
> | Minimum viable reason           | holder_reserve |
> |---------------------------------+----------------|
> | Default create carrier (msat)   | 355,000        |
> |---------------------------------+----------------|
> | Default create reason           | holder_reserve |
> |---------------------------------+----------------|
> | Admission threshold (msat)      | 330,000        |
> |---------------------------------+----------------|
> | Minimum allowed carrier (msat)  | 1,000          |
> |---------------------------------+----------------|
> | Holder reserve threshold (msat) | 355,000        |
> |---------------------------------+----------------|
> | Estimate only                   | yes            |
> +---------------------------------+----------------+
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason | Suggested action | Min msat | Min reason     | Default msat | Default reason  |
> +==============================================================================================================================================================================================================================================================================================================+
> | 9e7daaea3de28c050f2fc62bfdc30481b03f70fb4c9643586e944d131d99f4a3 | baa8fe3c11c6e487c99164bb77ea18bd | yes    | 98,340,000   | 0          | no      | yes     | yes        | 355,000       | 98,340,000     | -               | -                | 355,000  | holder_reserve | 355,000      | holder_reserve  |
> |------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------|
> | ea6112f5969c35607dc3eeec53dd283cc9eb487c45c4ee4d59e3764c6f9369a3 | f29d7f2969db062e789bacef883fba12 | yes    | 88,294,124   | 10,045     | yes     | yes     | yes        | 330,000       | 88,294,124     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt30u1p404werdq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp5cu8wkp96cxkf5aq88e2vvxzreqtfyv3qznkd0zpwjckng6mak69q9qrsgqxqrrsscqpjlp5jw6av9dh08z9fr6ujdl6rrhnfsfu8vlw4fkp3ksdnsyr7jactnps7qpzrzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqnvgqqqqqqqlgqqqqqqqqfqy4n0jecukgcear3h2udzmarnzmur3m0aq8calfk69e9dcpn64erxel0n65d2z6rj432zdc3p6kcp2mrsf7wl0uxx70ea3j6spj7sxwqqnef906",
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
> | Destination           | 0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM          |
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
>   "invoice": "lnbcrt50u1p404werdqjwfnkygrvdcsxgetddupp5exakge5y4ar3q8sz6uk0mqh4q5tze0pcl2lyx85atfa9qw4p80lssp55zfny6m0y3rvrda7uswve2rf6a4rsfdn56ecssqgg937z4aw6kuq9qrsgqxqrrsscqpjlp5jw6av9dh08z9fr6ujdl6rrhnfsfu8vlw4fkp3ksdnsyr7jactnps7qp9rzjqwpnqz5zupf8v2hlkt2fr5e5skejlrh3delmjlvztxf9jsstq0wd5qqqqyqqnvgqqqqqqqlgqqqqqqqqfqhkl8vj3yqezx88x8a344lxajkywryy0hvjn3h9a9dp44gtl5z6esj3wn65ezxvtutuaw8nxpxjuvzv2qp3mgepruyckcqhlum3yxtagpmfsw4h",
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
> | Payment hash          | c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 0318a96c2aeb23a82c6e7c8dbd6f888f36c728a8caa7f1c3c9cc99f1b7cbff386e |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM          |
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
>   "payment_id": "c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff
>
> ```

**Run:**

```bash
$ rgbldk pay get c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff

```

**Result:**

> ```text
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +====================================================================================================================================================================================================================================================================================================================================================================================================================+
> | id                     | c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff                                                                                                                                                                                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff                                                                                                                                                                                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816804                                                                                                                                                                                                                                                                                                                                                                                |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                                                                                                                                                                         |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"c9bb646684af47101e02d72cfd82f505162cbc38fabe431e9d5a7a503aa13bff","preimage":"c689b574cef9e493286c74c9e31b0fe4e429b7fccd046c23d2bcc98a3def4f6c","rgb":{"asset_amount":"5","contract_id":"contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM","direction":"Outbound","is_swap":false},"secret":"a093326b6f2446c1b7bee41ccca869d76a3825b3a6b38840084163e157aed5b8"} |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 5,000,000 msat                                                                                                                                                                                                                                                                                                                                                                            |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                                                                                                                                                                    |
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
> | Available inbound (msat)        | 9,045,876       |
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
> | Holder reserve threshold (msat) | 355,000         |
> |---------------------------------+-----------------|
> | Estimate only                   | yes             |
> +---------------------------------+-----------------+
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason                  | Suggested action              | Min msat | Min reason     | Default msat | Default reason  |
> +============================================================================================================================================================================================================================================================================================================================================+
> | 9e7daaea3de28c050f2fc62bfdc30481b03f70fb4c9643586e944d131d99f4a3 | b281163c081842b6beaca4b3818ef5b7 | yes    | 0            | 100,000    | yes     | no      | no         | 330,000       | 0              | carrier_inbound_capacity_too_low | add_inbound_carrier_liquidity | -        | -              | -            | -               |
> |------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------|
> | ea6112f5969c35607dc3eeec53dd283cc9eb487c45c4ee4d59e3764c6f9369a3 | 5d9bbe711f1c32343bd6df05425c5188 | yes    | 9,045,876    | 89,954     | yes     | yes     | yes        | 330,000       | 9,045,876      | -                                | -                             | 1,000    | minimum_viable | 330,000      | admission_floor |
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------+
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
>   "invoice": "lnbcrt40u1p404weydq6wfnkygrvdcsxgetddusxyctrdvpp5zdsxaxur7azh9z3c5t3rev0rfucjr9klagr8477hhlgxj4eaglnqsp5ln879luxqlw9nftu7zhjqk9e6egclggau2ws4nt6e63m2d2lntfs9qrsgqxqrrsscqpjlp5jw6av9dh08z9fr6ujdl6rrhnfsfu8vlw4fkp3ksdnsyr7jactnps7qpr2mhj8slww25y642wg4nud2sqwp4h58lyamdcx0g6x8rgxp969dcxas9rscwrsxawu85jejcltkj9999ja0n9jfdc9ycus9sk5yfr0ssqanlhfn",
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
> | Payment hash          | 13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 0383300a82e052762affb2d491d33485b32f8ef16e7fb97d82599259420b03dcda |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM          |
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
>   "payment_id": "13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6
>
> ```

**Run:**

```bash
$ rgbldk pay get 13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6

```

**Result:**

> ```text
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +====================================================================================================================================================================================================================================================================================================================================================================================================================+
> | id                     | 13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6                                                                                                                                                                                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6                                                                                                                                                                                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816805                                                                                                                                                                                                                                                                                                                                                                                |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                                                                                                                                                                                                                                         |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"13606e9b83f745728a38a2e23cb1e34f312196dfea067afbd7bfd069573d47e6","preimage":"3920c39ff37dc47c26d3377ca4430ce9b8762a77e937e908a52db456938e7b3c","rgb":{"asset_amount":"3","contract_id":"contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM","direction":"Outbound","is_swap":false},"secret":"fccfe2ff8607dc59a57cf0af2058b9d6518fa11de29d0acd7acea3b5355f9ad3"} |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 4,000,000 msat                                                                                                                                                                                                                                                                                                                                                                            |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                                                                                                                                                                                                                                                    |
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
> | b281163c...818ef5b7 | 0318a96c...cbff386e | 140737488420865 | 1099569430531 | 1099667734532 | 100,000         | true  | true   | -                   | -         | -          |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------|
> | 5d9bbe71...425c5188 | 0318a96c...cbff386e | 156130651209729 | 1099674222592 | 1099649908742 | 100,000         | true  | true   | contract...-P0u4XMM | 6         | 4          |
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
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+-------+---------------------+
> | User Channel ID     | Counterparty        | SCID            | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable | RGB Contract        | RGB Local | RGB Remote | Lease | Lease Ends (height) |
> +=============================================================================================================================================================================================================+
> | baa8fe3c...77ea18bd | 0383300a...0b03dcda | 140737488420865 | 1099667734532 | 1099569430531 | 100,000         | true  | true   | -                   | -         | -          | lsps1 | 4506                |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+-------+---------------------|
> | f29d7f29...883fba12 | 0383300a...0b03dcda | 156130651209729 | 1099649908742 | 1099674222592 | 100,000         | true  | true   | contract...-P0u4XMM | 4         | 6          | lsps1 | 4515                |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+-------+---------------------+
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
>   "payment_id": "c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554
>
> ```

**Run:**

```bash
$ rgbldk pay get c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554

```

**Result:**

> ```text
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                             |
> +============================================================================================================================================================================================+
> | id                     | c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554                                                                                                  |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554                                                                                                  |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✔ Succeeded                                                                                                                                                       |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Spontaneous                                                                                                                                                       |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                             |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816807                                                                                                                                                        |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                 |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"c430e8ab5185d7e9954d87badc3d32ce3429830df4ab231dcf465e8c8d4d0554","preimage":"d7c4c4f27261311a6f0d90f7e88365979e9b3ac0ae8a288f58f6ff30c2c81643"} |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 1,234 msat                                                                                                                                                        |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | 0 msat                                                                                                                                                            |
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
>   "payment_id": "9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8 --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8

```

**Result:**

> ```text
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field                  | Value                                                                                                                                                             |
> +============================================================================================================================================================================================+
> | id                     | 9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8                                                                                                  |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash           | 9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8                                                                                                  |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction              | Outbound                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status                 | ✘ Failed                                                                                                                                                          |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind                   | Spontaneous                                                                                                                                                       |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked            | false                                                                                                                                                             |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | updated_at (unix secs) | 1794816810                                                                                                                                                        |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | expires_at (unix secs) | -                                                                                                                                                                 |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details           | {"payment_hash":"9878e8e19876a6a4efe377b6a5cbbfe06acb40a5b1dedaf799c40a2b744e54b8","preimage":"a1cc9a316ec92499dc8cac52d2cd8730aa8e0cb67fd192c3270c50d59fad34a7"} |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | amount (msat)          | 2,345 msat                                                                                                                                                        |
> |------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | fee paid (msat)        | -                                                                                                                                                                 |
> +------------------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
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
> ChannelPending funding_txo=a9484691e3d35869cf6d49f2e66c0eca88eb25ffa07a22576387e238ca13589a:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=a9484691e3d35869cf6d49f2e66c0eca88eb25ffa07a22576387e238ca13589a:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=06a670186fbdf718974e6b2ae446a3f4
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
> ChannelReady user_channel_id=95fb0e1599a9a9c7498264c41c3a622c
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
> | 5d9bbe71...425c5188 | ea6112f5...6f9369a3 | 0318a96c...cbff386e | negotiating | coop   | -            | 86,348     | contract...-P0u4XMM | 6         | -         |
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
> | f29d7f29...883fba12 | ea6112f5...6f9369a3 | 0383300a...0b03dcda | negotiating | coop   | -            | 11,047     | contract...-P0u4XMM | 4         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "2e07b9cd30cb8223e06c777b53aae0f9c3fd3c817da6c5e5572f49b7013ca0ca",
>   "4cdeac0962f4b61b098b1af75bffce7bd27a7d35fd85636c36c1b24505800757",
>   "4708ea93cf6d8e38c1bb72864f8fadebd9dec74be94b1a6aadbd4d47eb638848",
>   "4a31992bddfde1add46e88c29f12fc9f5caa977106c9309133ee702267258cbe",
>   "6d583059cf244eecc6e97d7d88e2b508668c51b8a0737be0090a37caac6d67eb",
>   "0d15ed5b10102d1cfa671d7561d9224ec30e1d9bb4d120b1580e48eabb550fe2"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "34f6f8a72117ad9e1086ee4e88b357288f4740d79eb5f9d9ba51685cd0d90b5d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "2e970f24ecd7713f9bf54635ac955c8ca261c310b8846678ca14677ce6b13c6c"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "2592e9359f66f61eb5523ccb5df39be70d25aaf9c91ff33ee9bb6f00ba29b770"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "059aa436b78d5066d918e5ece764ca2669563f6e04e660dee0c99298381b0baf"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "0859df5a02506dca49fcbdf8f7a737451b9e73fe669fa2199ee7e926a6db5df6"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "4b6c78f0ad5da6a8d933ad55b7eda6c4da20dc285c70944c9f85952f7d9b012d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "333a8f361c9eda9c44fd6d84054f15e5008a444cfc02a0300f36ce0a6574071b"
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
$ rgbldk --connect http://127.0.0.1:8502 channel closing

```

**Result:**

> ```text
> No channels currently closing.
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
> +--------------------------+-----------------+
> | Asset                    | Balance         |
> +============================================+
> | BTC On-chain (total)     |  1.00016658 BTC |
> |--------------------------+-----------------|
> | BTC On-chain (spendable) | 99,991,658 sats |
> |--------------------------+-----------------|
> | BTC Anchor reserve       |     25,000 sats |
> |--------------------------+-----------------|
> | BTC Lightning (total)    |     97,654 sats |
> +--------------------------+-----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |    13 |         0 |        0 |    13 |
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
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
> | BTC On-chain (total)     | 1.01406449 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.01381449 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |    25,000 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |         0 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM | contract:k7XWFbd5-xFSPXJN-~oY7zTB-PDs_6qb-BjaDZwI-P0u4XMM |    87 |         0 |        0 |    87 |
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
>   "user_channel_id": "2ae864b78291a540f2045fdb89992f59"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3wyxk5s8p9p75l7zu0mu9kh43kqldkxv04f2p

```

**Result:**

> ```text
> [
>   "685a8c45144ab2462d37fc88f8446838a8bacde7656e25dc5149cf9a9f74fde3",
>   "2d4574815d58aa4fafbfe3c57d9bfa33621027c5f8fed4073ed996a6a7f20a0e",
>   "1d00df0cd8e633dd271667e78333d8f22663b639689a516ce30b820a8cc50ba8",
>   "5f9b274f530437ecc9315ced48bf35b5d797408f8b114b196b8f94513bc1cead",
>   "1710d1d3bc84cbe8cfe90cb915a9f13160f6ca64089af1fd62f5e2764c0cbae0",
>   "75e07b6058db493de896fb710f7a62de81efb14800145e710cad6f9d5a970673"
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
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
> | User Channel ID | Channel ID          | Counterparty        | Status       | Source  | Closing Txid | BTC (sats) |
> +==================================================================================================================+
> | -               | decdf60f...195428f5 | 0318a96c...cbff386e | broadcasting | unknown | -            | 46,654     |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 channel closing

```

**Result:**

> ```text
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
> | User Channel ID | Channel ID          | Counterparty        | Status       | Source  | Closing Txid | BTC (sats) |
> +==================================================================================================================+
> | -               | decdf60f...195428f5 | 0383300a...0b03dcda | broadcasting | unknown | -            | 1,000      |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
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
