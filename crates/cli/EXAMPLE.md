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
esplora=http://127.0.0.1:51104

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
>   "node_id": "022c41148b6c80f47b184e22bf2d47f20c27048094a427d74d961c27720b03bd34"
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
>   "node_id": "02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204"
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
> {"keystore_path":"/tmp/rgbldk-cli-example-pdou6llq/keystore-init-demo/keystore","mnemonic":"gown floor buzz door profit achieve hawk human chapter begin dutch pride submit popular keep have assault clock usual group ill payment fork umbrella","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-pdou6llq/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-pdou6llq/keystore-migrate-demo/keys_seed.bak.20260702-081607","ok":true}
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
> | 02b12717...b1dfd204 | 127.0.0.1:9736 | true      | true      |
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
>   "address": "bcrt1qzn5a6mh9fg4jqqkvtxr62xd7gm4aa8h9shssgh"
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
>   "address": "bcrt1qpgxc99m0075j7kyl8er8nmvqw0qs69c5j567k0"
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
> bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qzn5a6mh9fg4jqqkvtxr62xd7gm4aa8h9shssgh 1

```

**Result:**

> ```text
> e21b57a35c28308ddbdb3464f78c259f44b22b0e79adf1e3de23463e5fa15fcb
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qpgxc99m0075j7kyl8er8nmvqw0qs69c5j567k0 1

```

**Result:**

> ```text
> bbb16d42fd1a53d4df3a324a550dd6b05ebda6fae37b0f210635872ffafa36ec
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "7b0eee16af3d2499dea71646a68c6e2b0a3b08a711b10e0035343de760126799",
>   "4938a88bdc654e4662d1fbc65944ea14a3da3b73282b8e7f39f257842c3feabb",
>   "3e9738caec14394522525f534eab1037aec51899a8626b94c4f654b0cc7de9e1",
>   "258f2f1e0a2df0519f5273136dc2a6f06117dd38cb768d28d78b8779ffef8f8d",
>   "51ceec8d02691650f3c513cea487ca6c1e52986df8c7c3a6ba18a670c457ea70",
>   "24ca520f665f7f24aa7040ac22e2d2c32b65f7688efa15c3c7540b035da0518d"
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
> | e21b57a35c28308ddbdb3464f78c259f44b22b0e79adf1e3de23463e5fa15fcb:1 |  100,000,000 | Confirmed |    102 | Available    | -        |            - |
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
>   "user_channel_id": "e31ab7ff562b14dad77b10f4e3e05dfd"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "7c71e9305ebca10a04a3c4bf5508283efcb33371c49a603d8385d2ce8dcb4517",
>   "063f85a345a22dc8c1feb4d53a04d32e43e131b683f0b61d0c9d466d96f3d533",
>   "6c59028b8e0d9c8b9cc369c929df291ea2571633d8ad1a8e921afcf53fe2e46c",
>   "687249de96015fb59121562133b1c7bac562e65867248fe7c038f89c3937f70f",
>   "3f3dbab9b9da8848af4dff2fb2419b03269ad2328469e408b393c8b598119c18",
>   "250ae3d17940ad06e4a6c1aa8e4419404dd4ca84546bdb305f9c083cf44a5330"
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
> | e31ab7ff...e3e05dfd | 022c4114...0b03bd34 | 120000          | true  | true   |
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
> | cb68c2bf...5034cd9a | 02b12717...b1dfd204 | 120000          | true  | true   |
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
> | 022c41148b6c80f47b184e22bf2d47f20c27048094a427d74d961c27720b03bd34 |
> |--------------------------------------------------------------------|
> | 02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204 |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 02b12717...b1dfd204.
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
> Channel 118747255865345 connects 022c4114...0b03bd34 and 02b12717...b1dfd204.
> Capacity: unknown.
>
> 022c4114...0b03bd34 -> 02b12717...b1dfd204: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1782980203       |
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
> 02b12717...b1dfd204 -> 022c4114...0b03bd34: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1782980203       |
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
>   "address": "bcrt1qxtr5x478dd5yg2ypvcq8j6meuflnz3p48h8pnk"
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
> | cb68c2bf...5034cd9a | 02b12717...b1dfd204 | 120000          | true  | true   |
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
> | e31ab7ff...e3e05dfd | 022c4114...0b03bd34 | 120000          | true  | true   |
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "2aa8a7c82d70b92fcabfe4050d8904af93a79051412ed56ad0b169006c11e694",
>   "6727128d4763db3523ba6fd440735a98f2b751c43bb9f4c576cefd3f6c95aeab",
>   "12b8ffb0ae8ede3d89bc283820babc56f3d89efad5fa19bf3ad8aa02cbfee9c2",
>   "2efa5268522cc177e581f1bfcc99d0bf0bc1ce119884254fbf68951700633c0a",
>   "78d24948af4fc03a3a12f4b6d51adbed01ff769361aec38966a4a6e788c4f952",
>   "15e7ca7085275c29b047185821af40d8665f5558a4d057c1221a0f07cdfb7e2f"
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
> BTC balance change: on-chain total +30,000 sats, spendable +30,000 sats, anchor reserve 0 sats, lightning -30,000 sats.
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
>   "address": "bcrt1qrws7y9gaswxmx825y88hcwal43zgxrwzt2vlzy"
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
> rgb(wpkh([696893cb/86h/827167h/0h]tpubDC792Khz6Lk2YBR2k9oN56neR2biPRQJ7CunoBXzJixGpBWxNNL9uG11Ta6nVxDkyVGUxa3oGtmfEYWxwBKsJF21Go7NQwkerJ7ojNFb9Gy/<0;1>/*),seals(),noise(60c73e0242f08792a0980fa99c88e74de5386f8693b8be517a30e4c1f31d4143))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | 696893cb    | m/86'/827167'/0' | tpubDC792Khz6Lk2YBR2k9oN56neR2biPRQJ7CunoBXzJixGpBWxNNL9uG11Ta6nVxDkyVGUxa3oGtmfEYWxwBKsJF21Go7NQwkerJ7ojNFb9Gy | wpkh([696893cb/86'/827167'/0']tpubDC792Khz6Lk2YBR2k9oN56neR2biPRQJ7CunoBXzJixGpBWxNNL9uG11Ta6nVxDkyVGUxa3oGtmfEYWxwBKsJF21Go7NQwkerJ7ojNFb9Gy/<0;1>/*)#u6ac2e4p |
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
> | Public key      | 03eb63747462a08ba95512785e02ebfa85e2f1caeda639ba96a12920784d6b71ed |
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
> 30440220437bf719f6d465f4a2eca2a9b59f6ddc7bceafbbd32515cfb2158451b49d536d02205eaf269b4defa17dcf4e39d1accad1278adef984ca0960771161f7ec56eda477
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
>   "address": "bcrt1qfe7yvnkuye2py89twtswzxh096evant7gw88sr"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> 28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> c22f4b03a8b1a6b72ec243b02a9024eec40300e4e10d22b37974f04643937243
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "7aef02b7c63b91971706608a93fc891ba93a412bff74526684cac88d56891d7a"
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
> | 28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99:0 |   10,000,000 | In mempool |      - | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+------------+--------+--------------+-------------+----------------------------|
> | c22f4b03a8b1a6b72ec243b02a9024eec40300e4e10d22b37974f04643937243:0 |      110,000 | In mempool |      - | Available    | -           | FeeSupport, BlindingTarget |
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
> | 28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99:0 |   10,000,000 |      - | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | c22f4b03a8b1a6b72ec243b02a9024eec40300e4e10d22b37974f04643937243:0 |      110,000 |      - | Available |        |
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
>   "reservation_id": "manual-reservation-1782980280-ok5hYnV62av1V62TghWHjfemWVNDGSLN",
>   "outpoint": "28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99:0",
>   "reserved_until_unix_secs": "1782980340"
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
>   "contract_id": "contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs",
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
>       "detail": "28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99:0"
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
> | DemoAsset | DEMO   |         0 |    100 | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
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
$ rgbldk rgb contracts export --contract-id contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs --out /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.
> Consignment key: contract_export_d1b7aa080d22a42c7cd7601c387446022da90138ae52e6a685d50f8ada7b87a8
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99:0
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
>         "detail": "contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_0efc33e690d23ef042a67bb36d3b9b55b396e1756d776cb7ac6feff2f3ea957a",
>     "contract_id": "contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs",
>     "ok": true
>   },
>   "out": "/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_0efc33e690d23ef042a67bb36d3b9b55b396e1756d776cb7ac6feff2f3ea957a.
> Saved to /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.download.zip.
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
>   "address": "bcrt1qe7w6dgwpvqrdktypllaqu7ks40x7326lx7cx2x"
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
> e5bc2434ff2f68128a66c02550027a6b414bbc9db844e549de453f551da200b4
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "5e92fe5a18d5b588e03c4cc47cb9be8eabaa62e5018aaef81f3fd56d0d869b23"
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
$ rgbldk rgb contracts import --contract-id contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs --file /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_b1b57d91c1737168029a6488cb36f9606a979b989de15a45886256ecdee92923
> Imported contract contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.
> Consignment key: contract_import_b1b57d91c1737168029a6488cb36f9606a979b989de15a45886256ecdee92923
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
> | DemoAsset | DEMO   |         0 |    100 | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
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
> The node knows contract contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.
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
>   "address": "bcrt1qmfjjp7k9wjk8lzkupqtgm348d954ur6rhmhw7z"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> 321e44b20d7b57772ef8f5346d00f1bc311b06a36a83cefc8250193cf0e809ff
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "72b49f71476638372445b2bc79b43675ac70ed8eec1b090bb1d2b966a92812a8"
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
>   "address": "bcrt1q4c6d22njmp8au4myasty8tm50g8hhfwn3pqzr2"
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
>   "address": "bcrt1q9jl0fzfe8vug28edjxcne8qwltdfh22926etm0"
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
>   "address": "bcrt1q6sys9pduqmplxpdzpml92c9axtjspqqn322gj0"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"df887781d7ce2a2868ce6d10a7b5e7bb5054da1863e0d427e932e6e4ff1db2cb","status":"broadcast","outputs":[{"address":"bcrt1q4c6d22njmp8au4myasty8tm50g8hhfwn3pqzr2","value_sats":"30000","vout":2},{"address":"bcrt1q9jl0fzfe8vug28edjxcne8qwltdfh22926etm0","value_sats":"28000","vout":0}],"change":{"address":"bcrt1q6sys9pduqmplxpdzpml92c9axtjspqqn322gj0","value_sats":"941828","vout":1},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "40cf33a9dd4e3b45b9c0d59acc56b871cb7e018c5ed2a51ae6617be53a045889"
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
> BTC balance change: on-chain total -58,172 sats, spendable -58,172 sats, anchor reserve 0 sats, lightning 0 sats.
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
>   "address": "bcrt1qqegdzyxccfk2vy8mre5h3ufw2j54msxun93knv"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> e4ff91e32130c5b15f681c5c1041828ab1bb6385a6bfb9e9ba963f3c99a1babd
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "02ff3f689c68af6f74f289d42f3b71f8ef50a3b2e3277dc079f5d0927e676d19"
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
> BTC balance change: on-chain total +500,000 sats, spendable +500,000 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --color never --output json --pretty rgb address

```

**Result:**

> ```text
> {
>   "address": "bcrt1q8cg7vjpeprq55fj8c4n37wayldrhg07y0flc4p"
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
>   "address": "bcrt1qhv65rv2vw2y6ej02658r46r6xryawxgk9zkcdj"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO 28f38adb1040b4a987a937ab8ad6163d7b17ceaf76d491ccef128b6bcd7f8f99:0.
> Broadcast transaction: c650b013f861d29e44585e343f5eb8ded5e42099f10e481c6006fc38a33e90cf
> Replacement output: bcrt1q8cg7vjpeprq55fj8c4n37wayldrhg07y0flc4p with 10,040,000 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs_c650b013f861d29e44585e343f5eb8ded5e42099f10e481c6006fc38a33e90cf
> Change output: bcrt1qhv65rv2vw2y6ej02658r46r6xryawxgk9zkcdj with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "235af7c5f105eccc9a46b1128bead01b113a933f23bbf562d50af6592c194172"
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
>   "address": "bcrt1qphah3wmny2l2yulm3f63phz6d6khjathe30x8j"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO df887781d7ce2a2868ce6d10a7b5e7bb5054da1863e0d427e932e6e4ff1db2cb:0.
> Broadcast transaction: 63d783e4d0869fa2d1a718965b55f5a1c2ebacabe782bf1bec71b4dbe2d64694
> Sent 27,889 sats to bcrt1qphah3wmny2l2yulm3f63phz6d6khjathe30x8j.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "09e61b0bd53fde4fe51e18953c247b285fd6ab02d303b8a2bc648cbf1405c6a9"
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
>   "invoice": "contract:tb@Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs/90@at:T~9KVw8p-LmDNG_NC-jTHl~sg7-2jyAxc0V-GtjuBP1b-M3p68Q/?expiry=2026-07-02T09:18:55.489839+00:00",
>   "blinding_utxo_used": "e5bc2434ff2f68128a66c02550027a6b414bbc9db844e549de453f551da200b4:0"
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
> | Contract         | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:T~9KVw8p-LmDNG_NC-jTHl~sg7-2jyAxc0V-GtjuBP1b-M3p68Q    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1782983935                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs/90@at:T~9KVw8p-LmDNG_NC-jTHl~sg7-2jyAxc0V-GtjuBP1b-M3p68Q/?expiry=2026-07-02T09:18:55.489839+00:00
> scope=contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:T~9KVw8p-LmDNG_NC-jTHl~sg7-2jyAxc0V-GtjuBP1b-M3p68Q
> data=90
> expiry=2026-07-02T09:18:55.489839+00:00
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
>   "txid": "462eccb407e8ca13d9664eca1de576d6ab4b82bf84f74ffe89965dc44613fde9",
>   "consignment_key": "rgb_consignment_contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs_462eccb407e8ca13d9664eca1de576d6ab4b82bf84f74ffe89965dc44613fde9"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs_462eccb407e8ca13d9664eca1de576d6ab4b82bf84f74ffe89965dc44613fde9.
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
> Accepted contract contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "2374565ae64da825efcbdbe3da4850835da6c86f9b38a61a7a222d917addabad",
>   "1bb30eb6a5927ef674383446dafecf823edb749eac8f23364682836c7b49aa0b",
>   "389b8623474d1a8473999fe175d7c1ec8a0f908c72ec4134c37d46471bbe786f",
>   "62448cfb33c7f534443ca6f4a76e8bca7ab35c2a1173cdc0bc71f40281030c63",
>   "6fcc950a969ea0c2460fe97486af5c21fb539b5a428de34a158992e93cb718c0",
>   "198abd08409dd47f91e37229c9381968afe9e9aeae8616d500ff91dbcec93216"
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
$ rgbldk rgb contracts balance contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
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
> | 3e05fd5e72efd8959652b2bc317c7119163a5ab3c30fbf7e6ba69cd19ee7d7b9 | succeeded | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |     90 | -    | transfer_import_af064239ef6748d44e6a33d5bfec9b1f3be96b85f751196521db4a2b0630319f |
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
$ rgbldk rgb contracts balance contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
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
>   "user_channel_id": "3e2068aa7a8cdf120a0db3a1dd060a0f"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "170f8c37d52a1a9380274888bf901f55f7de4e27b9757d8ee0cae686cf83486f",
>   "19a9a4811dab99319ec9232200164a2a5dc8247b8793a82ffa5260f823a309b0",
>   "77e85a794276a2db75ade2b11abba126ddb5dbe7900f8fd137fa600057526567",
>   "1ccd6c9645211c623f8612098385e203b7457634014666de36f06ef1299c06f2",
>   "1ae0a9d1cea1a02087968e07ac67e107f812ed91eeb23e5120d3bd873b84fa55",
>   "152a9af1bb46d0327019257e6bc784f67ced6158eae6168ef53f9865eb12abee"
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
> | 3e2068aa...dd060a0f | 02b12717...b1dfd204 | 100000          | false | false  | contract...-nNouDAs | 10        | 0          |
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
>   "invoice": "contract:tb@Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs/7@at:LEMvBliJ-R8WJC6~y-Z4zZ3P~E-V8xZ7pv2-TunPuU2y-ngUFCw/?expiry=2026-07-02T09:19:26.248329+00:00",
>   "blinding_utxo_used": "4e119b467a7b78325fe9eb8da04efff2bab438ec5b30c059fa1beaa28f3ac352:2"
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
>   "txid": "7f23ed84b4049c0c8908b67d4583d6e3868dc4b1f778ce46de1a5ef6736185ee",
>   "consignment_key": "rgb_consignment_contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs_7f23ed84b4049c0c8908b67d4583d6e3868dc4b1f778ce46de1a5ef6736185ee"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs_7f23ed84b4049c0c8908b67d4583d6e3868dc4b1f778ce46de1a5ef6736185ee.
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
> Accepted contract contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "22590fa90f5ffb89f31f6457481cb277dd6a2ef3683e513d225b6eae84fb534f",
>   "6120637047a091bb529c56d2c96da32b21789d78f3dd69b99727c3e80b5d5235",
>   "229700c9e4c1eb89d3fb20e55868415116148a387b73ad63ef26bf66bfc474da",
>   "2c79ccea272c074b931abf1afb5d615fc55b064ca03ed90a94f349552f90fdcf",
>   "733f698a9371c9ad0b30970e9b4113ba9f5c0d777d949978e348188c5f1005f9",
>   "1d916fc2a20bebd3227e11e1e0ca35f09ede1d6f5926c77f86a3bfe4100b5a1b"
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
$ rgbldk rgb contracts balance contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |
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
>   "invoice": "lnbcrt100n1p4yv8cudq8v3jk6mcnp4q2cjw9ctqs38wrry9hyg3cy4pf93xzfhg52gy8cd8ld5q4d3mlfqgpp56j2ceads9pg4607lrqv7jlzteclnt4d0esny04d9rz3h7pclnhaqsp5dq8rkjg0g7rwxd974ysl003jvyeezk0w2cy5e020lcdc53vd7jvq9qyysgqcqzp2xqrrssrzjqgkyz9ytdjq0g7ccfc3t7t287gxzwpyqjjjz046djcwzwustqw7ngqqqqyqqxuqqquqqqqlgqqqqqqqqfqu6q8sq8lvjus9v2w8f3epkn6xxddjv3csrpvl4dnzlwy339vmn2szgv4kpj87hm3wh8vpkp08x8q858vy2zlcykxecfj9wvrekanxjgpdkx0p7"
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
> | payment_hash | d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204 |
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
>   "payment_id": "d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa",
>   "preimage": "b016a25ef9d7ff2c5888d5424c4c2ef7fc45f027250cf60d613d25689bdcd9bb",
>   "amount_sats": "10",
>   "destination": "02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa
>
> ```

**Run:**

```bash
$ rgbldk pay get d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"d4958cf5b028515d3fdf1819e97c4bce3f35d5afcc2647d5a518a37f071f9dfa","preimage":"b016a25ef9d7ff2c5888d5424c4c2ef7fc45f027250cf60d613d25689bdcd9bb","secret":"680e3b490f4786e334bea921f7be3261339159ee56094cbd4ffe1b8a458df498"} |
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
>   "invoice": "lnbcrt130n1p4yv8e9dq0v3jk6medwdjkueqnp4q2cjw9ctqs38wrry9hyg3cy4pf93xzfhg52gy8cd8ld5q4d3mlfqgpp53janc3p5dzc8mnlcu5mn0k2yk5pjletja2f7z7nhcdtycf3dmqrqsp5nmxjl00hcqym5w5yld4edxk7gq3q4rycnh2gs73j3tfrudhuv4gq9qyysgqcqzp2xqrrssrzjqgkyz9ytdjq0g7ccfc3t7t287gxzwpyqjjjz046djcwzwustqw7ngqqqqyqqxuqqquqqqqlgqqqqqqqqfqw7d298jjknuxsypwh4hjcvacl8rypennfulugt0kn5xfu47c9a04gsu48un05dj2cefhrl6e5m27twazc4e39hd3cnej6th9wvjryucp62ypua"
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
>   "payment_id": "8cbb3c443468b07dcff8e53737d944b5032fe572ea93e17a77c3564c262dd806"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 8cbb3c443468b07dcff8e53737d944b5032fe572ea93e17a77c3564c262dd806 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 8cbb3c443468b07dcff8e53737d944b5032fe572ea93e17a77c3564c262dd806
>
> ```

**Run:**

```bash
$ rgbldk pay get 8cbb3c443468b07dcff8e53737d944b5032fe572ea93e17a77c3564c262dd806

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 8cbb3c443468b07dcff8e53737d944b5032fe572ea93e17a77c3564c262dd806                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"8cbb3c443468b07dcff8e53737d944b5032fe572ea93e17a77c3564c262dd806","preimage":"074f43e583e31a1f34fab2002c40f9b1a34aa5ce874a7b2fe009fd306da0e6fd","secret":"9ecd2fbdf7c009ba3a84fb6b969ade40220a8c989dd4887a328ad23e36fc6550"} |
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
>   "invoice": "lnbcrt1p4yv8etdqdv3jk6medweshynp4q2cjw9ctqs38wrry9hyg3cy4pf93xzfhg52gy8cd8ld5q4d3mlfqgpp5a49r6cn2w9dp8k4k07u30w88hrmekgrdmpp02sfe88aegjxuxmzssp5p9np2w04ewj0yhqxdy3v6esxj8sgc9ff6fstf8896l77wy0x9swq9qyysgqcqzp2xqrrssrzjqgkyz9ytdjq0g7ccfc3t7t287gxzwpyqjjjz046djcwzwustqw7ngqqqqyqqxuqqquqqqqlgqqqqqqqqfqpyxptkj65nr05ctwtu4qk5pxvrs0chq8x8vkedq5w6rr7fsgclq8gtd583z6uvxqhqvuxwwj5n79n2rvtjc29ej6v25v4f8gevmqg8gpems8mc"
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
>   "payment_id": "ed4a3d626a715a13dab67fb917b8e7b8f79b206dd842f5413939fb9448dc36c5"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ed4a3d626a715a13dab67fb917b8e7b8f79b206dd842f5413939fb9448dc36c5 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ed4a3d626a715a13dab67fb917b8e7b8f79b206dd842f5413939fb9448dc36c5
>
> ```

**Run:**

```bash
$ rgbldk pay get ed4a3d626a715a13dab67fb917b8e7b8f79b206dd842f5413939fb9448dc36c5

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | ed4a3d626a715a13dab67fb917b8e7b8f79b206dd842f5413939fb9448dc36c5                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"ed4a3d626a715a13dab67fb917b8e7b8f79b206dd842f5413939fb9448dc36c5","preimage":"ec9a1d2538db28c9a1aa6e1f9e3f83db824dd3c41c2122e33c059803c9f38a4e","secret":"09661539f5cba4f25c066922cd660691e08c1529d260b49ce5d7fde711e62c1c"} |
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
>   "invoice": "lnbcrt140n1p4yv8e3dqcv3jk6meddphkcepdvdkxz6tdnp4q2cjw9ctqs38wrry9hyg3cy4pf93xzfhg52gy8cd8ld5q4d3mlfqgpp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp54mngwurh352qnvqyvw69dna4f4jjrpyhpvh4q6y4gj03mqdq5k3q9qyysgqcqzp2xqrrssrzjqgkyz9ytdjq0g7ccfc3t7t287gxzwpyqjjjz046djcwzwustqw7ngqqqqyqqxuqqquqqqqlgqqqqqqqqfqa07aqvvw4st2nrt45te97g0nhp8ctgqxjwudwxr7nrsjrlpm4kh5ppd37hx08f43wp5gw35wsknye4fw0gcynv2z58uur42czux37tgpnn348j"
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
> | destination  | 02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204 |
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
> | kind_details    | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":"4242424242424242424242424242424242424242424242424242424242424242","secret":"aee68770778d1409b00463b456cfb54d652184970b2f506895449f1d81a0a5a2"} |
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
>   "invoice": "lnbcrt150n1p4yv8emdqhv3jk6meddphkcepdveskjmqnp4q2cjw9ctqs38wrry9hyg3cy4pf93xzfhg52gy8cd8ld5q4d3mlfqgpp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp5hfk58nuxnl2c9ach7r7grdgs62mj7txh456vp8h3wksd2h950laq9qyysgqcqzp2xqrrssrzjqgkyz9ytdjq0g7ccfc3t7t287gxzwpyqjjjz046djcwzwustqw7ngqqqqyqqxuqqquqqqqlgqqqqqqqqfqnhwgu352n298hygy7hqnhfqzsyyqhy5kfmx2p0l5cny00ksawx5hnzrqtut2agd9y2svn2jwjjzl779n5umpgk428deet5m3s6jfagqq03w7ks"
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
> | kind_details    | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"ba6d43cf869fd582f717f0fc81b510d2b72f2cd7ad34c09ef175a0d55cb47ffa"} |
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
>   "invoice": "lnbcrt120n1p4yv869dq0v3jk6medvfskx6cnp4qgkyz9ytdjq0g7ccfc3t7t287gxzwpyqjjjz046djcwzwustqw7ngpp5k22eng2cexmnnufcr7dat4huqw2f45wx4d9dstg3vfntg8jrffmssp5cag6qtdeu5ed5k5ya9tgqak4ayl6sdcj9j0dq46q7x55jud2ylhq9qyysgqcqzp2xqrrssrzjq2cjw9ctqs38wrry9hyg3cy4pf93xzfhg52gy8cd8ld5q4d3mlfqgqqqqyqqt7gqquqqqqlgqqqqqqqqfqv8pn9hx3yhnz5fadykey4vftzvh6nwzsw956xmevyl4ulgepewqsr4xlx2he8v5jqqpjccjv53xspphk8pauymm3dce3kh93t7wllncqtlf6jz"
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
>   "payment_id": "b29599a158c9b739f1381f9bd5d6fc03949ad1c6ab4ad82d116266b41e434a77",
>   "preimage": "00f397d285a58234424fc99cadd67b9114ef4084ef7f9e9066621f3b54258208",
>   "amount_sats": "12",
>   "destination": "022c41148b6c80f47b184e22bf2d47f20c27048094a427d74d961c27720b03bd34",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait b29599a158c9b739f1381f9bd5d6fc03949ad1c6ab4ad82d116266b41e434a77 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> b29599a158c9b739f1381f9bd5d6fc03949ad1c6ab4ad82d116266b41e434a77
>
> ```

**Run:**

```bash
$ rgbldk pay get b29599a158c9b739f1381f9bd5d6fc03949ad1c6ab4ad82d116266b41e434a77

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | b29599a158c9b739f1381f9bd5d6fc03949ad1c6ab4ad82d116266b41e434a77                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"b29599a158c9b739f1381f9bd5d6fc03949ad1c6ab4ad82d116266b41e434a77","preimage":"00f397d285a58234424fc99cadd67b9114ef4084ef7f9e9066621f3b54258208","secret":"c751a02db9e532da5a84e9568076d5e93fa837122c9ed05740f1a94971aa27ee"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx533dtsgvyqgqqpkqqqqpqqqs95ektvarylmty6mjwgje7525m937h82dkudk4ekfc9k6lqylx392qyp5dfg4au7zw2ryp23lfhyumwu7j6nqrtt75pr986ugq8lq6edfzmgqwnpgrnv62mapsrqmdgws3xpxr8xwawd8vhh7tu0hdpqckj5hjqaf3nlkrmu3t8v7krg8ge50edx39kh6sm9hhuvpgaz2at48xtheffmw29ecsenj7vtkcnz8jqv0dcklp2j5fcjq42gr06xh8klg6wvp4pyfg77ps2r3ryw5llndhdz232fywn9anug3vggzalhk2fr8lkk4qlsucdy2rl0zx9250fvtfcjhkqafcy9h84xxqchs"
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
> | offer_id                  | f03cbedc9be270474cba77ec4861d1bbbfe305f26aad4389f5713b4b03156860   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 02efef652467fdad507e1cc348a1fde2315547a58b4e257b03a9c10b73d4c6062f |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1782984028                                                         |
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
>   "payment_id": "d20c75edec1f44c5a2d8b758920d3c14f99e852c45973fe7971f4d643612f46e"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait d20c75edec1f44c5a2d8b758920d3c14f99e852c45973fe7971f4d643612f46e --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> d20c75edec1f44c5a2d8b758920d3c14f99e852c45973fe7971f4d643612f46e
>
> ```

**Run:**

```bash
$ rgbldk pay get d20c75edec1f44c5a2d8b758920d3c14f99e852c45973fe7971f4d643612f46e

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | d20c75edec1f44c5a2d8b758920d3c14f99e852c45973fe7971f4d643612f46e                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"f03cbedc9be270474cba77ec4861d1bbbfe305f26aad4389f5713b4b03156860","payer_note":null,"payment_hash":"42cd3fc4309320f8ca1b81001a66c724fb95f77ac3b8563e0f42093aa59c6b25","quantity":null} |
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
>   "refund": "lnr1qqs9l865dfdj7cfdfgkthf7ypgaveahlkjyd2g2utnxkgl9vuzzx2dq2qq8qg6jx9434qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxge00yg8gmcg8u5cxxx625epgz5azd4utq3hsgvrjn7dtljf2c4gty9hyetxw4hxgttyv4kk7kkzqqqqqmqqqqqsqqgrurp9zfzvfg7992699lcmevlvjfnqqa3qknyxckqf8mp00t7sm72qzq4ct66nlfdlapv6mhmm0axgtxv9l86wx2gctqffppyzp66yypt0k5q8gffegkv3cwrdggafldhgketwn4m9hg0m3k6hueh4lcq5xsydans4kqetzctrcaw9tv2er4j0y9m7vzfatkx7z7yndvg4rfx8yatjt3leurymn7kve5ce25j5fwtt3yprvfsmws9lyev4yf65mvkrqs09z39zjgrykre3s5fd76aaz0a57njs2gqxsxs",
>   "payment_id": "d474d27007df62b189c8acddefb0a94105456d589c53190c2719689042e0a8e5"
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
> | absolute_expiry_unix_secs | 1782984035                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 03232f7910746f083f298318da5532140a9d136bc582378218394fcd5fe49562a8 |
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
> lni1qqs9l865dfdj7cfdfgkthf7ypgaveahlkjyd2g2utnxkgl9vuzzx2dq2qq8qg6jx9434qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxge00yg8gmcg8u5cxxx625epgz5azd4utq3hsgvrjn7dtljf2c4gty9hyetxw4hxgttyv4kk7kkzqqqqqmqqqqqsqqgrurp9zfzvfg7992699lcmevlvjfnqqa3qknyxckqf8mp00t7sm72qzq4ct66nlfdlapv6mhmm0axgtxv9l86wx2gctqffppyzp66yypt0k5q8gffegkv3cwrdggafldhgketwn4m9hg0m3k6hueh4lcq5xsydans4kqetzctrcaw9tv2er4j0y9m7vzfatkx7z7yndvg4rfx8yatjt3leurymn7kve5ce25j5fwtt3yprvfsmws9lyev4yf65mvkrqs09z39zjgrykre3s5fd76aaz0a57njs2gqxsx4qacptzfchpvzzyacvvsku3z8qj59ykycfxaz3fqslp5laksz4k80aypqzvg8rctjylmupnt2llzzaxaknq558ed867rz3plkazq9yd52w3szqzq4v7z5z3wnpqvcjh06nkc3w8lr0y8ttfn0jhd07t0m2t20z4ak6tyqgs38r7jujaakx525cv3pm8uue3d4m6mw5un4tknj8jgzntzhxanvhxdq3w2wx4538679kt9gy5hgsu4k4y2gmsel035vz5ch5558w7eamg0qnxu2ktr3xh5lmm29tr38324rtard96wr2hp383u3yrzzuzu54kfk4vd0plxwmyx00859q0ux09aj63gugumw4q8xg840xh6d5qth85c264ahv549zrsqqqqqqqqqqqqqq9gqqqqqqqqqqqqgayjedltzjqqqqqq9yq34yv86k4qszzn2pj44q8g7pxp573ujwnneh7nq4l44txyvs5kc0dnkqudete8d2qggwrtsrqgqqpvppqwxkwu20pz9u4cyfm2p0z436l5365su90etm8lectmrrhevxxhvw4uzq76k9tkru9p60q0kmv0kvcufk4jfj94sm99hmtaeydw6qtcuhqh6szv4rpv04w8zccmssayrpn3t3l6s3fx0yeprejr8tvek0rqz42ns
> payment_id: 214d41956a03a3c13069e8f24e9cf37f4c15fd6ab31190a5b0f6cec0e372bc9d
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
$ rgbldk pay wait d474d27007df62b189c8acddefb0a94105456d589c53190c2719689042e0a8e5 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> d474d27007df62b189c8acddefb0a94105456d589c53190c2719689042e0a8e5
>
> ```

**Run:**

```bash
$ rgbldk pay get d474d27007df62b189c8acddefb0a94105456d589c53190c2719689042e0a8e5

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | d474d27007df62b189c8acddefb0a94105456d589c53190c2719689042e0a8e5                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"214d41956a03a3c13069e8f24e9cf37f4c15fd6ab31190a5b0f6cec0e372bc9d","quantity":null} |
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
>   "refund": "lnr1qqs2vyx259gfl8qh93dku7weajktstd8330w5j7mn9rfyf4dkmv9w4s2qq8qg6jx9449qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqss8ph5cs5tsxv49dgt7ahx6w59qhsm9e5drnlhqvu483ep0pu0t03dtyfhyetxw4hxgttpvfskuer0dckkgetddadvyqqqqpkqqqqpqqqsx5d3lsjyhf7evvhlrdeea8qfdfezh6jq5wds99a9pz3wpppr23q6qyp8sjrwhh53p6wsy6lqam5xmq97rhsu7rev2lck9twkn49eq9dk8jsqw35uawauddl094eagjvx4y8qr9l9p0ewdaa4ssklfadl2vc8ww4wcwv44zesmeqgyjnly3yd7ehl4ac8j788xsm8ezk3unn26uh03a44ndu9hg80wc8espvf49kkjj3604m3stvvs3nd7ncszrpey5zcaxjetfe3a0cd8p7mv5xjxrktqdam4n4w423q",
>   "payment_id": "fafd2655c2c1d4f687fd7c53bc4407e2b145667343fb3abfb640f26a2c9782fc"
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
> | fafd2655...2c9782fc | Failed    | Bolt12Refund | Outbound | 1,111           | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | cb5fa15f...a3571be2 | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 5e66afd8...b110f365 | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | d20c75ed...3612f46e | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | bdbaa199...e391ffe4 | Succeeded | Onchain      | Inbound  | 500,000,000     | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | cf903ea3...13b050c6 | Succeeded | Onchain      | Outbound | 40,000,000      | 254,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 9446d6e2...e483d763 | Succeeded | Onchain      | Inbound  | 27,889,000      | 111,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | b29599a1...1e434a77 | Succeeded | Bolt11       | Inbound  | 12,000          | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | cbb21dff...817788df | Succeeded | Onchain      | Outbound | 58,000,000      | 172,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | d474d270...42e0a8e5 | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 425ed4e4...6e55384c | Succeeded | Bolt11       | Outbound | 14,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | d4958cf5...071f9dfa | Succeeded | Bolt11       | Outbound | 10,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | ff09e8f0...b2441e32 | Succeeded | Onchain      | Inbound  | 1,000,000,000   | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | ed4a3d62...48dc36c5 | Succeeded | Bolt11       | Outbound | 11,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 8cbb3c44...262dd806 | Succeeded | Bolt11       | Outbound | 13,000          | 0          |
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
$ rgbldk rgb ln invoice estimate-carrier

```

**Result:**

> ```text
> +---------------------------------+-----------------+
> | Field                           | Value           |
> +===================================================+
> | Receive available               | yes             |
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
> +------------------------------------------------------------------+--------+--------------+------------+---------+---------+----------+----------------+--------------+-----------------+
> | Channel                                                          | Usable | Inbound msat | Local sats | Reserve | Receive | Min msat | Min reason     | Default msat | Default reason  |
> +========================================================================================================================================================================================+
> | 52c33a8fa2ea1bfa59c0305bec38b4baf2ff4ea08debe95f32787b7a469b114f | yes    | 78,294,124   | 20,045     | yes     | yes     | 1,000    | minimum_viable | 330,000      | admission_floor |
> +------------------------------------------------------------------+--------+--------------+------------+---------+---------+----------+----------------+--------------+-----------------+
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
>   "invoice": "lnbcrt30u1p4yv8mzdq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp5j9rxc2h4fc0z2g36z4ak6nkh89mkug0tglvuvrkh4vspyehlkfxs9qrsgqxqrrsscqpjlp59m8n8a9vt2elr7hk37mn6q6g5mmv5rz82yyzrv5kvu9eek3wps9s7qpztxjk33v8lhmdjffk6csdjp3n0sg3em0rx3fay2kru4327673v7hhsratxjugf3rq9tvrfp5qm3wjz720mhf5reh4v9yshm7nk76au7cpvesyh2",
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
> | Destination           | 02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs          |
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
>   "invoice": "lnbcrt50u1p4yv8mydqjwfnkygrvdcsxgetddupp5yhdszytlpyft0ca6egaeupucrxzacec85ufysm9rkdyzecqgac6ssp5yfqq6rmuhzn7tpwqkx6470cfgqkhjtw2dd8wmw4h03anztw3pygq9qrsgqxqrrsscqpjlp59m8n8a9vt2elr7hk37mn6q6g5mmv5rz82yyzrv5kvu9eek3wps9s7qp9xjvxjfv040j22u6w7kd8cjmzfylr3fx9cw8emqru9g5tvv5lmrsytd8dczmv6vsnwp578gqhgy6uvulyg8vuga56qgj0ec6p7kd53wspqr9m6k",
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
> | Payment hash          | 25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 02b127170b0422770c642dc888e0950a4b1309374514821f0d3fdb4055b1dfd204 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs          |
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
>   "payment_id": "25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35
>
> ```

**Run:**

```bash
$ rgbldk pay get 25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"25db01117f0912b7e3baca3b9e07981985dc6707a712486ca3b3482ce008ee35","preimage":"a8358b3e0073b34abbf47060994ad9e33f2b0e624c574c2b3a4a689668080f93","rgb":{"asset_amount":"5","contract_id":"contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs","direction":"Outbound","is_swap":false},"secret":"22400d0f7cb8a7e585c0b1b55f3f09402d792dca6b4eedbab77c7b312dd10910"} |
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
> +------------------------------------------------------------------+--------+--------------+------------+---------+---------+----------+----------------+--------------+-----------------+
> | Channel                                                          | Usable | Inbound msat | Local sats | Reserve | Receive | Min msat | Min reason     | Default msat | Default reason  |
> +========================================================================================================================================================================================+
> | 52c33a8fa2ea1bfa59c0305bec38b4baf2ff4ea08debe95f32787b7a469b114f | yes    | 24,045,876   | 74,954     | yes     | yes     | 1,000    | minimum_viable | 330,000      | admission_floor |
> +------------------------------------------------------------------+--------+--------------+------------+---------+---------+----------+----------------+--------------+-----------------+
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
>   "invoice": "lnbcrt40u1p4yv8mvdq6wfnkygrvdcsxgetddusxyctrdvpp5dw5q22phsky45hdtm863pda5fr2tr5guqtw0svywfs9f7pknjz8qsp5hv5530g67qagjkypq0ysfhpguxz2emtygc5mx747y2ewv5q0q2ss9qrsgqxqrrsscqpjlp59m8n8a9vt2elr7hk37mn6q6g5mmv5rz82yyzrv5kvu9eek3wps9s7qprwm73wz00t9y6x39fgt9d2sdecplynkcgsw82zfauac3hr3fvw749x7e3prugemk2k84zcyk3vpm6tnxhwm04nn4kd9hdtrmrrzpe3nqpednvww",
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
> | Payment hash          | 6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 022c41148b6c80f47b184e22bf2d47f20c27048094a427d74d961c27720b03bd34 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs          |
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
>   "payment_id": "6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e
>
> ```

**Run:**

```bash
$ rgbldk pay get 6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"6ba805283785895a5dabd9f510b7b448d4b1d11c02dcf8308e4c0a9f06d3908e","preimage":"5be5aacd92db2423efcd7c356578b9cd8de440c7517043aef67c87f307e49607","rgb":{"asset_amount":"3","contract_id":"contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs","direction":"Outbound","is_swap":false},"secret":"bb2948bd1af03a89588103c904dc28e184aced644629b37abe22b2e6500f02a1"} |
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
> | 3e2068aa...dd060a0f | 02b12717...b1dfd204 | 100000          | true  | true   | contract...-nNouDAs | 8         | 2          |
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
> | b1bdacd5...65bd73cd | 022c4114...0b03bd34 | 100000          | true  | true   | contract...-nNouDAs | 2         | 8          |
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
>   "payment_id": "49aeb380c605028f98dbc712c5b793630ba6a3ef37a2e57ea166e665008a2f6c"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 49aeb380c605028f98dbc712c5b793630ba6a3ef37a2e57ea166e665008a2f6c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 49aeb380c605028f98dbc712c5b793630ba6a3ef37a2e57ea166e665008a2f6c
>
> ```

**Run:**

```bash
$ rgbldk pay get 49aeb380c605028f98dbc712c5b793630ba6a3ef37a2e57ea166e665008a2f6c

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 49aeb380c605028f98dbc712c5b793630ba6a3ef37a2e57ea166e665008a2f6c                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"49aeb380c605028f98dbc712c5b793630ba6a3ef37a2e57ea166e665008a2f6c","preimage":"62f8bec5bc23d421841c7a7ff3f459628dec62d13141d7df261493ecc4aba9bf"} |
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
>   "payment_id": "0ef2dfa683e09a9b19fd0c9ac1b8b21cc340d758936f23fe5bc594aed474294b"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 0ef2dfa683e09a9b19fd0c9ac1b8b21cc340d758936f23fe5bc594aed474294b --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 0ef2dfa683e09a9b19fd0c9ac1b8b21cc340d758936f23fe5bc594aed474294b
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 0ef2dfa683e09a9b19fd0c9ac1b8b21cc340d758936f23fe5bc594aed474294b

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 0ef2dfa683e09a9b19fd0c9ac1b8b21cc340d758936f23fe5bc594aed474294b                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"0ef2dfa683e09a9b19fd0c9ac1b8b21cc340d758936f23fe5bc594aed474294b","preimage":"0d4c5b22b0b15d2b9db58399ac117d1ceae2a1befc5e461a9a3bbf7cbda91954"} |
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
> ChannelPending funding_txo=3ca1a52c911def17a28e8b4bfe7cdbacb1395046432ac3977bf44ec0309829a8:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=3ca1a52c911def17a28e8b4bfe7cdbacb1395046432ac3977bf44ec0309829a8:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=cb68c2bf90071077eabc9aee5034cd9a
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
> ChannelReady user_channel_id=e31ab7ff562b14dad77b10f4e3e05dfd
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "299dda9e495505c8c27c9cb2f53f8e649640f97529065f488385eeb362f6dbae",
>   "35021dd4d3f6e72c99806217578d2a81b0cf947a349ae2fd8e7039ed79febc67",
>   "72c596a4773607fa590a92737c922895d19c0f5a64155dee6439547858f9181a",
>   "2c2cf722e02ef54e7cdd0e113db52df74e36063167180a4631acf2c56b41b0ed",
>   "2fe803c7e29db0aee2205f9ff0ed8050e2bbd23bd9666ac4f5da58cb3b2ce7cd",
>   "1d6db9d1f572f5060f2029886fd9b2f1fac13c241c89d25b963e04b02262ec27"
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
> BTC balance change: on-chain total +76,941 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (total)     | 1.01536404 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.01459463 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    76,348 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |     7 |         0 |        0 |     7 |
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
> | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs | contract:Ls8z9Kxa-s~H69o_-3PQNIpv-bKDEdRC-CGylmcL-nNouDAs |    83 |         0 |        0 |    83 |
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
>   "user_channel_id": "efdd318fab46661fff142f76753cb0f7"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1ql2ldkxnh3y06ph73pjdn2kdy9lc2dwaychc5hd

```

**Result:**

> ```text
> [
>   "6f4b7f59d97b6971b618d5a66c4c93ff7a4dacd30e218b56aed8ccddd08f549e",
>   "027caf515685c97e59f63d8619886346b05840f5357b43478d80374c54dd6415",
>   "740400ebc4cbed8534ffc9db85a66649c9c18e9498d4e6a1ea100db63c501bbe",
>   "14dfb1309e2bb21cab7e8a7a423df41fb11fc65508b41d3d3b6ded8625a8d14d",
>   "69445d1037d982914bf852478803e0ed6ecf0932af3570cafcb2ee0945249922",
>   "122e2f734b7ffad874754d7d174a5975abcdde946b78c4ff5dcb8f87d5719f0f"
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
