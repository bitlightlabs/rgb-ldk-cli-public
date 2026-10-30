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
esplora=http://127.0.0.1:51555

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
>   "node_id": "030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d"
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
>   "node_id": "02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8"
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
> {"keystore_path":"/tmp/rgbldk-cli-example-3oygcj1p/keystore-init-demo/keystore","mnemonic":"collect type velvet reopen mimic happy job canyon visa thumb ladder memory neutral only guitar uniform auto leg dutch ritual high target parade bring","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-3oygcj1p/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-3oygcj1p/keystore-migrate-demo/keys_seed.bak.20261030-041718","ok":true}
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
> | 02ab70db...e036e2a8 | 127.0.0.1:9736 | true      | true      |
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
>   "address": "bcrt1qfpnlprgwm2f4tvu5lpyw8nx70ysun6watsft7n"
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
>   "address": "bcrt1qsl2utxt3t03j93yfxpuysn7u2rm09sdqnuy9lu"
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
> bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qfpnlprgwm2f4tvu5lpyw8nx70ysun6watsft7n 1

```

**Result:**

> ```text
> 55d2e4d26e571e3b4bc7bfcc827a16d52c1494988b89cef4fc6fa6cb02d92e09
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qsl2utxt3t03j93yfxpuysn7u2rm09sdqnuy9lu 1

```

**Result:**

> ```text
> 754e004ec1e512cf6224384e273a9145be5b8dd73035aa42f8f034312617b305
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "6259cbb475cc1f6a66010f280ad293c071c86d1fb045d184495136b7e460183c",
>   "37ed06ab5f50244bca8d46cbc4c5233e61f5ca33d1ad9ee7f80c00b4bc48af61",
>   "01813323719f9ab312e80fb5f593318ce89db81874648992392a2f813fe10392",
>   "1e8915c8956dfdb3d286e8013bf0c15bc09b01eb43e64ae6761e1b18ad9c054e",
>   "27fd94d617d91bcbb0ea591b6a96122e89f18985d498e12b1ed76b737ade89b4",
>   "45970f4401e522d66216e58a4c4c996c3bd9075917de1e0bd19d7a9eb36481f1"
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
> | 55d2e4d26e571e3b4bc7bfcc827a16d52c1494988b89cef4fc6fa6cb02d92e09:0 |  100,000,000 | Confirmed |    102 | Available    | -        |            - |
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
>   "user_channel_id": "000df37919ba2870b307405f653ceadc"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "5c56ef2c40745227c8de22f8de712c742c1bf7738af9ed7bbe5886f96ca4f95e",
>   "6e3c3d2a19301587ab5f034f188da54416bb87e9620d99de6e7d32b39600c4a6",
>   "68f191c6850041c2c3f053101d1d21bab90fc2c0d3c75947c47adb64b398e351",
>   "459051b3e86a78bffa17f1dc3c3b48e16d6cd816398c37f34769a2dd70ab170b",
>   "3cf88756ea56c16270b6e791e49ebaba7aa4a2713672bbb1dca7315fe9d79790",
>   "3afac5a38bd48faba5d656d87b54b494cdb7ed6b53d92155dab8f22ab233ce74"
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
> | 000df379...653ceadc | 030c5f52...9b584f6d | 118747255865345 | 1099675336705 | 1099668783108 | 120,000         | true  | true   |
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
> | f1b43f0c...0c7f3d50 | 02ab70db...e036e2a8 | 118747255865345 | 1099668783108 | 1099675336705 | 120,000         | true  | true   |
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
> | 030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d |
> |--------------------------------------------------------------------|
> | 02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8 |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 02ab70db...e036e2a8.
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
> Channel 118747255865345 connects 02ab70db...e036e2a8 and 030c5f52...9b584f6d.
> Capacity: unknown.
>
> 02ab70db...e036e2a8 -> 030c5f52...9b584f6d: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1793333848       |
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
> 030c5f52...9b584f6d -> 02ab70db...e036e2a8: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1793333848       |
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
>   "address": "bcrt1qe34vqh60p7cl99ptp8fk6j98j80cd6s465s5ds"
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
> | f1b43f0c...0c7f3d50 | 02ab70db...e036e2a8 | 118747255865345 | 1099668783108 | 1099675336705 | 120,000         | true  | true   |
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
> | 000df379...653ceadc | 030c5f52...9b584f6d | 118747255865345 | 1099675336705 | 1099668783108 | 120,000         | true  | true   |
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
> | -               | a3bd2739...c4c1318e | 030c5f52...9b584f6d | broadcasting | unknown | -            | 87,654     |
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
> | -               | a3bd2739...c4c1318e | 02ab70db...e036e2a8 | broadcasting | unknown | -            | 30,000     |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "00120856e5db9985e468b430bedd6fc9206236e96e125d7940ff7affa501ddbb",
>   "62cbf266931f335000d0d5a60802bf5af08797290c8544c1208be71c4b5c1ab3",
>   "3a40ee3174f4a72d7cb64df478132cdc67dbf6da56d45ff588636c6f1a81b04a",
>   "474403642fa9fa3005ad9df5b682ed91486510aa835a2f201358a8aa4c012999",
>   "1f52a80595aba6e70181d8f6064c3b5fadb80ba337e4ce08b51c00cac1a06fcd",
>   "758dfad2141eb60d03f2f9c45fa3c2190c71dbd57402a1d25808ac3414674c7f"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "612e530d7850f6217d1d2b7001e53f2698cf6c0ed9cf49bf6d08b66237ba2ff1"
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
>     "onchain_open_timeout_blocks": 144
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
> {"supported_options":{"min_required_channel_confirmations":0,"min_funding_confirms_within_blocks":1,"supports_zero_channel_reserve":false,"max_channel_expiry_blocks":52560,"min_initial_client_balance_sat":"0","max_initial_client_balance_sat":"0","min_initial_lsp_balance_sat":"10000","max_initial_lsp_balance_sat":"100000000","min_channel_balance_sat":"10000","max_channel_balance_sat":"100000000"},"service":{"require_token":null,"bolt11_invoice_expiry_secs":3600,"min_onchain_payment_confirmations":1,"auto_close_expired_channels":true,"channel_expiry_grace_blocks":144,"late_deposit_refund_window_secs":"2592000","onchain_open_timeout_blocks":144}}
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
> LSP 030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d @ 127.0.0.1:9735 saved (LSPS1, LSPS2)
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
> | 030c5f52...9b584f6d | 127.0.0.1:9735 | LSPS1, LSPS2 | yes   | yes        | no    |
> +---------------------+----------------+--------------+-------+------------+-------+
>
> ```

**Run:**

```bash
$ rgbldk lsps1 info

```

**Result:**

> ```text
> {"lsps":[{"lsp_node_id":"030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d","info":{"supported_options":{"min_required_channel_confirmations":0,"min_funding_confirms_within_blocks":1,"supports_zero_channel_reserve":false,"max_channel_expiry_blocks":52560,"min_initial_client_balance_sat":"0","max_initial_client_balance_sat":"0","min_initial_lsp_balance_sat":"10000","max_initial_lsp_balance_sat":"100000000","min_channel_balance_sat":"10000","max_channel_balance_sat":"100000000"},"pricing":{"btc_capacity_ppm_per_year":10000,"onchain_cost_sat":"10000","min_fee_sat":"1000"},"rgb":{"rgb_assets":[]}},"error":null}]}
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
>   "order_id": "34f59a92-4e0d-4bb4-95ee-05c0c48b678d",
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
>       "expires_at_unix_secs": "1793337467",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "invoice": "lnbcrt100840n1p4wgxntdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6pp5rlmkdzy5v3w69uxfeyf4v7wmrelxdud56w8zg5a848zxymn9qr0ssp54s8d97wkjnt7sucq82xaag53h68jdv5dsnv969kv9h77sr2f6d0q9qyysgqcqzp2xqrrssu4y320h8r599drksq0n4ljfdm2vtaa89qnvmq2l93qwhsg8ewvnz8f0t54hvz2znuyat26evw3e0ks8ku9n9tvd37ypmqfmel2dskggq0tumyg"
>     },
>     "onchain": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1793337467",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "address": "bcrt1qstp5rjuj2rsxx0c2f9649jc2j4vfxvl5mkx962",
>       "min_onchain_payment_confirmations": 1,
>       "refund_onchain_address": "bcrt1qctk765395h6wx7knexar8nk6zuckmtgjsk2n6t"
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qstp5rjuj2rsxx0c2f9649jc2j4vfxvl5mkx962 0.00010084

```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Run:**

```bash
$ rgbldk lsps1 order get <order_id>

```

**Result:**

> ```text
> order 34f59a92-4e0d-4bb4-95ee-05c0c48b678d — channel opened
> {"order_id":"34f59a92-4e0d-4bb4-95ee-05c0c48b678d","order":{"lsp_balance_sat":"100000","client_balance_sat":"0","required_channel_confirmations":0,"funding_confirms_within_blocks":1,"channel_expiry_blocks":4380,"announce_channel":false},"payment":{"bolt11":{"state":"expect_payment","expires_at_unix_secs":"1793337467","fee_total_sat":"10084","order_total_sat":"10084","invoice":"lnbcrt100840n1p4wgxntdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6pp5rlmkdzy5v3w69uxfeyf4v7wmrelxdud56w8zg5a848zxymn9qr0ssp54s8d97wkjnt7sucq82xaag53h68jdv5dsnv969kv9h77sr2f6d0q9qyysgqcqzp2xqrrssu4y320h8r599drksq0n4ljfdm2vtaa89qnvmq2l93qwhsg8ewvnz8f0t54hvz2znuyat26evw3e0ks8ku9n9tvd37ypmqfmel2dskggq0tumyg"},"onchain":{"state":"paid","expires_at_unix_secs":"1793337467","fee_total_sat":"10084","order_total_sat":"10084","address":"bcrt1qstp5rjuj2rsxx0c2f9649jc2j4vfxvl5mkx962","min_onchain_payment_confirmations":1,"refund_onchain_address":"bcrt1qctk765395h6wx7knexar8nk6zuckmtgjsk2n6t"}},"channel":{"funded_at_unix_secs":"1793333876","funding_outpoint":"65fb08b900accc995fcd5464f6579b79a67f0eafde761f2296ee977080a43d07:1","expires_at_unix_secs":"1795961876"},"rgb":null}
>
> ```

**Run:**

```bash
$ rgbldk channel ls

```

**Result:**

> ```text
> +---------------------+---------------------+------+---------------+---------------+-----------------+-------+--------+
> | User Channel ID     | Counterparty        | SCID | Out Alias     | In Alias      | Capacity (sats) | Ready | Usable |
> +=====================================================================================================================+
> | 030f295e...f63a8ab7 | 030c5f52...9b584f6d | -    | 1099546558466 | 1099657052161 | 100,000         | true  | true   |
> +---------------------+---------------------+------+---------------+---------------+-----------------+-------+--------+
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
> +---------------------+---------------------+-----------+---------+----------+
> | Order ID            | Counterparty        | Order     | Payment | Paid Via |
> +============================================================================+
> | 34f59a92...c48b678d | 02ab70db...e036e2a8 | completed | paid    | onchain  |
> +---------------------+---------------------+-----------+---------+----------+
>
> ```

**Run:**

```bash
$ rgbldk lsps1 orders get <order_id>

```

**Result:**

> ```text
> {"order_id":"34f59a92-4e0d-4bb4-95ee-05c0c48b678d","counterparty_node_id":"02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8","order_state":"completed","payment_state":"paid","paid_via":"onchain","lsp_balance_sat":"100000","client_balance_sat":"0","channel_expiry_blocks":4380,"announce_channel":false,"fee_total_sat":"10084","order_total_sat":"10084","onchain_address":"bcrt1qstp5rjuj2rsxx0c2f9649jc2j4vfxvl5mkx962","onchain_paid_sat":"10084","refund_onchain_address":"bcrt1qctk765395h6wx7knexar8nk6zuckmtgjsk2n6t","refund_txid":null,"fulfill_retry_count":0,"created_at_unix_secs":"1793333867","payment_expires_at_unix_secs":"1793337467","funding_outpoint":"65fb08b900accc995fcd5464f6579b79a67f0eafde761f2296ee977080a43d07:1","funded_at_height":126,"channel_closed_at_unix_secs":null,"rgb":null}
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
> {"advertise_service":true,"require_token":null,"channel_opening_fee_ppm":10000,"channel_over_provisioning_ppm":100000,"min_channel_opening_fee_msat":"2000000","min_channel_lifetime":4032,"max_client_to_self_delay":2016,"min_payment_size_msat":"10000000","max_payment_size_msat":"1000000000","max_unpaid_channels_per_peer":1,"max_unpaid_channels_total":10,"unpaid_channel_close_secs":"86400"}
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
> {"offers":[{"min_fee_msat":"2000000","proportional_ppm":10000,"valid_until":"2026-10-31T04:17:58.979+00:00","min_lifetime":4032,"max_client_to_self_delay":2016,"min_payment_size_msat":"10000000","max_payment_size_msat":"1000000000"}],"fee_msat":"2000000"}
>
> ```

**Run:**

```bash
$ rgbldk lsps2 invoice --lsp <node_id_a> --amount-msat 50000000 --max-fee 2500000

```

**Result:**

> ```text
> lnbcrt500u1p4wgxnhdqqpp5n5pvtx00y4e2qcyhap5c9vqw6g45a2vqvtd6h75gglfsh2v6l3cssp58sup55fqppu9fq7dj04ax6r7wft4v0qckqqnfyf3qz9r2vj8s0xs9qyysgqcqzpvxqrrssrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqwrqqqvqqqqqqqqqqqqqqfqjgdmr2gztprl886yq0z0r0frueelu5ppe8856ldeuhsvwff3mq9jevtc0ssz7cpqrt9jq26s2p5eus8ywg5n3mn36kk69w62m4rltkcpsgcdds
>
> ```

The invoice is paid by any node with a route to node-a; node-b then sees a `PaymentReceived` for the amount minus the fee, and `pay payments` lists it with kind `Bolt11Jit`. This walkthrough has no third node, so the invoice is not paid here.


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
> LSP 030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d removed
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
>   "address": "bcrt1qr9hue56m3uzs3s5nxlxdq6y6x378sscfyw2vhp"
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
> rgb(wpkh([4071bfcc/86h/827167h/0h]tpubDCWvmdtVURoGtqoEUf8PhX8tX89rsJS6mc5xHiNWU3EwHh6fXDq9nE3vZw7GSYPo6oKBviREwobmrp2HH2LCzY76xt2Go9yr4fxkeM5CCBX/<0;1>/*),seals(),noise(da8082945a7ba772312c4425e8c4a5f86e9457248f2263561b16a2a26b973c69))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | 4071bfcc    | m/86'/827167'/0' | tpubDCWvmdtVURoGtqoEUf8PhX8tX89rsJS6mc5xHiNWU3EwHh6fXDq9nE3vZw7GSYPo6oKBviREwobmrp2HH2LCzY76xt2Go9yr4fxkeM5CCBX | wpkh([4071bfcc/86'/827167'/0']tpubDCWvmdtVURoGtqoEUf8PhX8tX89rsJS6mc5xHiNWU3EwHh6fXDq9nE3vZw7GSYPo6oKBviREwobmrp2HH2LCzY76xt2Go9yr4fxkeM5CCBX/<0;1>/*)#8lrruzp7 |
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
> | Public key      | 03b7ce4b89191f733bd73fb21dfb99a774aa8b2b53176954a429a5e689cdedbcac |
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
> 30450221009f90e13efd79884b4e108afeb8ee25ead80a7760824fb5c6c4e2233ba28eae9d022076c836459cb1973c071de1f12b53072032c5b5d2bacf08d4303c13cfc803f585
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
>   "address": "bcrt1q5d66c8mzl06culvnesw0lc7cfyzkrhkgx7syn7"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> d0c80402393e5cdac8311f248768468337f909db055bfe1f96aaabbe667dfac0
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> 94f8a97201ce0ccaedbf8ac57685c72a3729657faaf268dc10c669c02373d4f4
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "57f28dc902b34a602f443dee0c93205ce0ffce616f747ebddc588429ca3e7ad3"
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
> | 94f8a97201ce0ccaedbf8ac57685c72a3729657faaf268dc10c669c02373d4f4:1 |      110,000 | Confirmed |    128 | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------|
> | d0c80402393e5cdac8311f248768468337f909db055bfe1f96aaabbe667dfac0:1 |   10,000,000 | Confirmed |    128 | Available    | -           | FeeSupport, BlindingTarget |
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
> | 94f8a97201ce0ccaedbf8ac57685c72a3729657faaf268dc10c669c02373d4f4:1 |      110,000 |    128 | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | d0c80402393e5cdac8311f248768468337f909db055bfe1f96aaabbe667dfac0:1 |   10,000,000 |    128 | Available |        |
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
>   "reservation_id": "manual-reservation-1793333885-JuapO5PMvxcTxkWzsbIjbnjzdsfiRb8o",
>   "outpoint": "94f8a97201ce0ccaedbf8ac57685c72a3729657faaf268dc10c669c02373d4f4:1",
>   "reserved_until_unix_secs": "1793333945"
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
>   "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
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
>       "detail": "94f8a97201ce0ccaedbf8ac57685c72a3729657faaf268dc10c669c02373d4f4:1"
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
> | DemoAsset | DEMO   |         0 |    100 | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
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
$ rgbldk rgb contracts export --contract-id contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 --out /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.
> Consignment key: contract_export_6532f5e55f9a6fc0d43859adc6287fb684c4bd9c6611015f9af921f5cdfdd779
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=94f8a97201ce0ccaedbf8ac57685c72a3729657faaf268dc10c669c02373d4f4:1
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip

```

**Result:**

> ```text
> {
>   "bytes": 2988,
>   "export": {
>     "checks": [
>       {
>         "name": "rgb_enabled",
>         "ok": true
>       },
>       {
>         "detail": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_7b7d9cc497038222e8ef0578906c4e4a1e8e4977ba549e54416d2e6b2db1ff64",
>     "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>     "ok": true
>   },
>   "out": "/Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_7b7d9cc497038222e8ef0578906c4e4a1e8e4977ba549e54416d2e6b2db1ff64.
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.download.zip.
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
>   "address": "bcrt1qxevmmj7g4ugehqcm5e57eygeyjry37gyh4cv8n"
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
> 23f16cfef6eb3c5dbb8435d042d74a73994fa99f71f4d1ada2c78b3f83cb9903
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "62eb07dce3081a36f96c4f20c3745857c4269b834b40ec766afcb0f86ed3cd4e"
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
$ rgbldk rgb contracts import --contract-id contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 --file /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_2a5ece42ffd3cd2d959675cceafa8e9becf80372090d5970df06cbf8e517b9e7
> Imported contract contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.
> Consignment key: contract_import_2a5ece42ffd3cd2d959675cceafa8e9becf80372090d5970df06cbf8e517b9e7
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
> | DemoAsset | DEMO   |         0 |    100 | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
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
> The node knows contract contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.
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
>   "invoice": "contract:tb@iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30/90@at:I_~69Z2u-3vfrd7uq-NRVqS5pJ-ixvV0lvj-5tD7N_uj-k4KCRg/?expiry=2026-10-30T05:18:09.996+00:00",
>   "blinding_utxo_used": "23f16cfef6eb3c5dbb8435d042d74a73994fa99f71f4d1ada2c78b3f83cb9903:0"
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
> | Contract         | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:I_~69Z2u-3vfrd7uq-NRVqS5pJ-ixvV0lvj-5tD7N_uj-k4KCRg    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1793337489                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30/90@at:I_~69Z2u-3vfrd7uq-NRVqS5pJ-ixvV0lvj-5tD7N_uj-k4KCRg/?expiry=2026-10-30T05:18:09.996+00:00
> scope=contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:I_~69Z2u-3vfrd7uq-NRVqS5pJ-ixvV0lvj-5tD7N_uj-k4KCRg
> data=90
> expiry=2026-10-30T05:18:09.996+00:00
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
>   "txid": "fc1652cde85526bcd0a50145aa98eda300798f9c3b6d7154c3eee1f7a2250029",
>   "consignment_key": "rgb_consignment_contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30_fc1652cde85526bcd0a50145aa98eda300798f9c3b6d7154c3eee1f7a2250029"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30_fc1652cde85526bcd0a50145aa98eda300798f9c3b6d7154c3eee1f7a2250029.
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
> Accepted contract contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "1cb59b8a37cb362d3afe08c3992f8868b05ea22eb5850055806555c4e92a956f",
>   "15f9ec3ffa0e1cd8cd9e7dbdfa95ac0507a91dac8e16910c2c2268b454da266d",
>   "1a194021d6f93447db8e81b32023837a269b8f57ef50318446b9c730270d5f06",
>   "15bac147622f85249881a0a84beb7d3ad9743dd19a0249d998ed6565d852d7df",
>   "71d05a7510da4b14c5a7926b3fd2dee73f53d5e5286e3fe7b803fe9769d67fea",
>   "72644d0a4773d7c63d72e995e26d1151e728b9600ed6ae8d6124f3fd173a3ccf"
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
$ rgbldk rgb contracts balance contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
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
> | 28c7a8842bd4a324c80b77a05127dd1cbb5e36a6dea24278e19a89976b0a12eb | succeeded | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |     90 | fc1652cde85526bcd0a50145aa98eda300798f9c3b6d7154c3eee1f7a2250029 | transfer_import_eefaf2e327f35940d8b0d151cd2c2a78b2b4e17286ab38d9d2779c182908e3d0 |
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
$ rgbldk rgb contracts balance contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
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
>   "user_channel_id": "9125abe4de6a79eee1552ee687e6fc60"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "693311e6e0ae5cf33bd158e513023802406dc00eec3f4e9c96bf518cafb640e6",
>   "320b5bf8dcafc469c769f2b2b0fe60423d1bcb11d310085a8591618563d97d86",
>   "210bdfef0c048898c88a260910fda53ac5f78006e7a4dd765d0640dec1f81ad5",
>   "39c103fd84b8b8bdd197b7645c9ef6a51d5581b8ce256f7e000491ef6060e80f",
>   "7af61f2552ec495b5ae2031126fa157ab77180dfbd40e03b272299d2279794fe",
>   "1ff96666da82034695d1ee09fe2abee62ddbc3f2832f241d270b7b1bb277d8d7"
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
> | 34f59a92...c48b678d | 02ab70db...e036e2a8 | 140737488420865 | 1099657052161 | 1099546558466 | 100,000         | true  | true   |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------|
> | 9125abe4...87e6fc60 | 02ab70db...e036e2a8 | -               | -             | -             | 100,000         | false | false  |
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
>   "invoice": "contract:tb@iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30/7@wout:AQAAAAAA-AAAQiscu-t4btSaoz-NNIO2L_y-wZ68EfmC-JSWK/?expiry=2026-10-30T05:18:35.555+00:00",
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
>   "txid": "0b0f8f9e350be609532bc0046f6cac8768ee154ce02c73c24dfee2c425445dd0",
>   "consignment_key": "rgb_consignment_contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30_0b0f8f9e350be609532bc0046f6cac8768ee154ce02c73c24dfee2c425445dd0"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30_0b0f8f9e350be609532bc0046f6cac8768ee154ce02c73c24dfee2c425445dd0.
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
> Accepted contract contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "1bf2c0e089a00bc0ef7b77fa7157e1d78f63b2a148b367a33f938fef18ba9ed8",
>   "60fed3602ae917d6226e7647ba6be305623a350985a84fcbab3525b65bde15e0",
>   "43a8a632c6f0cc3c9ead0e03c911787b49218d5ee1472291573f8770723541e7",
>   "436b8027a1769bd69b56e1aa4a2a3f205a10a84dfdb78f380b5951e18c9e04dd",
>   "6523061bdc162c6c4299caa42b755fb03c8cd33ffd2652339ab65efaeebdc94f",
>   "1b6fb7d8efa6ba0979d5b2482e77ade20525bbe23ff8dd00628d65cd002545cf"
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
$ rgbldk rgb contracts balance contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |
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
>   "address": "bcrt1qgy3f99xty6uhu4e5e2uje3wcwnzgupmv7t9uvn"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> ce1611e44689719dcb546d1948cec1a7ee165234c51b20f8e087620b8ad321c6
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "2c844bd4f8ad598a879089934bd2804d4fcbb28be68626ec58c121e75b642129"
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
>   "address": "bcrt1qmychy5pt5rkm2rdnlqxq3dmlv6lx88u7c2378s"
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
>   "address": "bcrt1qpygc5y7k3jmmkag7krlrja4etcevragqjppxtl"
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
>   "address": "bcrt1qqmmme9gy42ag0smgwyfcqjn0druuq0juezw0yn"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"dd39e5d7ea567a5a04f4ba08792f8e376ffb482821e906b142a75fb1b0c52f87","status":"broadcast","outputs":[{"address":"bcrt1qmychy5pt5rkm2rdnlqxq3dmlv6lx88u7c2378s","value_sats":"30000","vout":0},{"address":"bcrt1qpygc5y7k3jmmkag7krlrja4etcevragqjppxtl","value_sats":"28000","vout":2}],"change":{"address":"bcrt1qqmmme9gy42ag0smgwyfcqjn0druuq0juezw0yn","value_sats":"941828","vout":1},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "52c456af686ab96acdb3b36cc2b60702821f7ae2a78d8568dd508682d74fb6bc"
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
$ rgbldk --color never --output json --pretty wallet address

```

**Result:**

> ```text
> {
>   "address": "bcrt1qtwxus6sxglen8p30zd07r9hjk6lwuj2lysa702"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> db7a5ffd053927f56c690931fc44bfc569f92576929c9c551662ece4700df82d
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "72c7d21b2b96cd8a1b3b0426aed870ca5ac63a5ba3f881802f95377dbaa8837c"
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
>   "address": "bcrt1q8h9fj62lq2kj0ukqxrkee25t8lrslxemqs4uw5"
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
>   "address": "bcrt1qgtf4cug8sxlddwxd48738gg2zkt0cffa43p8wh"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO 0b0f8f9e350be609532bc0046f6cac8768ee154ce02c73c24dfee2c425445dd0:2.
> Broadcast transaction: c63db454f33f5a793427ae7c2b24dfab58194ff0f7763e5f31c351093f1c6bac
> Replacement output: bcrt1q8h9fj62lq2kj0ukqxrkee25t8lrslxemqs4uw5 with 10,029,800 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30_c63db454f33f5a793427ae7c2b24dfab58194ff0f7763e5f31c351093f1c6bac
> Change output: bcrt1qgtf4cug8sxlddwxd48738gg2zkt0cffa43p8wh with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "2ef84a02ab99fc8aa3fa1c04b0164025a5f4ee7c4e269bd0c5f7d5b3000b5883",
>   "1ae8acea334c4bdd40279277897a44bee4eb0065b68e9cd9c1c50bbd6b09c16e",
>   "24f0a3711aea43e96704a13309539099cdca206c603d98af20ade1ddb25b34b3",
>   "56952bda48aa7a698e2ad62cfe2bacf0266cee175826ea68f43f484679737b97",
>   "6996449eb95f1089b6fa50e5ddb9c7c8af0a212598d1a8b6752ecfd896c6b20a",
>   "20124e5972b37ddb875cc5f92e87cc75ed0f3efc1593e3036182457041686511"
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
>   "address": "bcrt1qag72lgkgumnt57z5dg67apqvw5v35xr5537a6q"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO dd39e5d7ea567a5a04f4ba08792f8e376ffb482821e906b142a75fb1b0c52f87:2.
> Broadcast transaction: 5d716d77a0ed9d55a5591f5529ad47ab224149b8a6dbb4ad82e18509f2a4f103
> Sent 27,889 sats to bcrt1qag72lgkgumnt57z5dg67apqvw5v35xr5537a6q.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "6fc181dee393e64a3c7f0cb2e98415733cb28e3d59c52b1e9f87e30cdc8a9d79",
>   "3bb9e094d6d8b8f9b9d409f1f84967b41083024ed63da82b4a80f4abbbbdabfe",
>   "3d0400f5e5a673ee2371250bc75e6c73072668dd9d5973da4ddf575981c4e938",
>   "2bdf30270d166241d5e2c3e9a85fe02038d2ebd4dec4b52bf5cb0b29af4fca4e",
>   "38136b1e4169b213a51352c9286da2ff10665c19f6c1b8d6b88e7670b5cbce32",
>   "2a1ac914ab0c2b213699d1cd55a26b8e1f710978d9dad8b6b1482f7c4691c143"
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
>   "swap_string": "rgb-swap:v1:eyJxdW90ZSI6eyJleGVjdXRpb25faWQiOiIyMjkyYzZiMzdlZjIzMzljODEwMjQ2YTg2NzUxNWJiZGVkNzg5MmYzOWY1YzZjMDY0NzA5OWZiMDYxYmFmNTFiIiwibWFrZXIiOiIwMzBjNWY1MmQ4ZWU2YzYwODcxZTUzZjQ1ODMyOWMwYzdiNTJlMzliMmNhNjdhYjdkYWE5MTVkZTE1OWI1ODRmNmQiLCJ0YWtlciI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsInRha2VyX291dGJvdW5kX2NoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSIsImFzc2V0X2lkIjpbMTM2LDkzLDEyNCwxOCw4NSw2OCwyLDEyMywyMjgsMjUyLDEwMyw0NSw0OCwyOSw3NCwyMTUsMjU1LDIyNSwyMTYsMjIxLDE4MCwyNDEsMjQ1LDExMSwxNjksMjM1LDE1NywxNDgsMTA0LDY2LDE2MywxMjVdLCJhc3NldF9hbW91bnQiOiIyIiwiYnRjX2Ftb3VudF9tc2F0IjoiMTAwMDAwMDAiLCJidGNfY2Fycmllcl9hbW91bnRfbXNhdCI6IjUwMDAwMDAiLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImNyZWF0ZWRfYXRfdW5peF9zZWNzIjoiMTc5MzMzMzkyOCIsImV4cGlyeV9zZWNzIjozNjAwLCJzZXR0bGVtZW50X21vZGUiOiJDaXJjdWxhckh0bGMiLCJtYWtlcl90b190YWtlcl9mZWVfcGF5ZXIiOiJNYWtlciIsInRha2VyX3RvX21ha2VyX2ZlZV9wYXllciI6Ik1ha2VyIiwiY29uc3RyYWludHMiOnsibWFrZXJfdG9fdGFrZXIiOnsiZml4ZWRfcm91dGUiOlt7Im5vZGVfaWQiOiIwMmFiNzBkYjYwZGI5OTk4Njg2YzBjZWQyZjg0NTA0NTlkOGVmZDYyZmVmMjhhY2NlYTI4NTM5MjI4ZTAzNmUyYTgiLCJjaGFubmVsX3NjaWQiOiIxNTYxMzA2NTEyMDk3MjkifV0sInJlcXVpcmVkX25vZGVzIjpbIjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCJdLCJyZXF1aXJlZF9jaGFubmVscyI6WyIxNTYxMzA2NTEyMDk3MjkiXSwicmVxdWlyZWRfbHNwcyI6W10sIm1heF9mZWVfbXNhdCI6IjAifSwidGFrZXJfdG9fbWFrZXIiOnsiZml4ZWRfcm91dGUiOlt7Im5vZGVfaWQiOiIwMzBjNWY1MmQ4ZWU2YzYwODcxZTUzZjQ1ODMyOWMwYzdiNTJlMzliMmNhNjdhYjdkYWE5MTVkZTE1OWI1ODRmNmQiLCJjaGFubmVsX3NjaWQiOiIxNTYxMzA2NTEyMDk3MjkifV0sInJlcXVpcmVkX25vZGVzIjpbIjAzMGM1ZjUyZDhlZTZjNjA4NzFlNTNmNDU4MzI5YzBjN2I1MmUzOWIyY2E2N2FiN2RhYTkxNWRlMTU5YjU4NGY2ZCJdLCJyZXF1aXJlZF9jaGFubmVscyI6WyIxNTYxMzA2NTEyMDk3MjkiXSwicmVxdWlyZWRfbHNwcyI6W10sIm1heF9mZWVfbXNhdCI6IjAifSwibWF4X3RvdGFsX2NsdHZfZGVsdGEiOjIwMTZ9fSwicGF5bWVudF9oYXNoX2hleCI6IjIyOTJjNmIzN2VmMjMzOWM4MTAyNDZhODY3NTE1YmJkZWQ3ODkyZjM5ZjVjNmMwNjQ3MDk5ZmIwNjFiYWY1MWIiLCJjb3VudGVycGFydHlfbm9kZV9pZF9oZXgiOiIwMmFiNzBkYjYwZGI5OTk4Njg2YzBjZWQyZjg0NTA0NTlkOGVmZDYyZmVmMjhhY2NlYTI4NTM5MjI4ZTAzNmUyYTgiLCJjaGFubmVsX3NjaWQiOjE1NjEzMDY1MTIwOTcyOSwiY29udHJhY3RfaWQiOiJjb250cmFjdDppRjE4RWxWRS1BbnZrfkdjLXRNQjFLMX4tfmgyTjIwOC1mVnZxZXVkLWxHaENvMzAiLCJhc3NldF9hbW91bnQiOjIsImJ0Y19hbW91bnRfbXNhdCI6MTAwMDAwMDAsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0Ijo1MDAwMDAwLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImV4cGlyeV9zZWNzIjozNjAwLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6MTc5MzMzMzkyOH0",
>   "payment_hash": "2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b",
>   "info": {
>     "payment_hash": "2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8",
>     "channel_scid": "156130651209729",
>     "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>     "asset_amount": "2",
>     "btc_amount_msat": "10000000",
>     "btc_carrier_amount_msat": "5000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1793333928",
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
>   "payment_hash": "2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b",
>   "role": "Taker",
>   "status": "Offered",
>   "counterparty_node_id": "030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d",
>   "channel_scid": "156130651209729",
>   "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1793333928",
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
>   "payment_hash": "2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b",
>   "role": "Taker",
>   "status": "Accepted",
>   "counterparty_node_id": "030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d",
>   "channel_scid": "156130651209729",
>   "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1793333928",
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
$ rgbldk swap get 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Accepted                                                         |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 02ab70db...e036e2a8                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 156130651209729                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-lGhCo30                                              |
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
> | Created (unix secs) | 1793333928                                                       |
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
>   "payment_hash": "2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b",
>   "status": "InFlight"
> }
>
> ```

**Run:**

```bash
$ rgbldk swap get 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 02ab70db...e036e2a8                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 156130651209729                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-lGhCo30                                              |
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
> | Created (unix secs) | 1793333928                                                       |
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
$ rgbldk swap get 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b |
> |---------------------+------------------------------------------------------------------|
> | Role                | Taker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 030c5f52...9b584f6d                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 156130651209729                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-lGhCo30                                              |
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
> | Created (unix secs) | 1793333928                                                       |
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
> | 2292c6b37ef2339c810246a867515bbded7892f39f5c6c0647099fb061baf51b | Taker | Settled | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 | 2     | RGB         |
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
>   "swap_string": "rgb-swap:v1:eyJxdW90ZSI6eyJleGVjdXRpb25faWQiOiIxNjUyNDAxYzk4M2E1ZDg3ZWIxNzk5M2JkZjdjMGRlN2IyMzA5ZmU3MjMxOGY5Nzg3Nzk2M2NlYzQ0N2JkMjk5IiwibWFrZXIiOiIwMzBjNWY1MmQ4ZWU2YzYwODcxZTUzZjQ1ODMyOWMwYzdiNTJlMzliMmNhNjdhYjdkYWE5MTVkZTE1OWI1ODRmNmQiLCJ0YWtlciI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsInRha2VyX291dGJvdW5kX2NoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSIsImFzc2V0X2lkIjpbMTM2LDkzLDEyNCwxOCw4NSw2OCwyLDEyMywyMjgsMjUyLDEwMyw0NSw0OCwyOSw3NCwyMTUsMjU1LDIyNSwyMTYsMjIxLDE4MCwyNDEsMjQ1LDExMSwxNjksMjM1LDE1NywxNDgsMTA0LDY2LDE2MywxMjVdLCJhc3NldF9hbW91bnQiOiIxIiwiYnRjX2Ftb3VudF9tc2F0IjoiMTAwMDAwMCIsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoiMTAwMDAwMCIsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiY3JlYXRlZF9hdF91bml4X3NlY3MiOiIxNzkzMzMzOTMwIiwiZXhwaXJ5X3NlY3MiOjM2MDAsInNldHRsZW1lbnRfbW9kZSI6IkNpcmN1bGFySHRsYyIsIm1ha2VyX3RvX3Rha2VyX2ZlZV9wYXllciI6Ik1ha2VyIiwidGFrZXJfdG9fbWFrZXJfZmVlX3BheWVyIjoiTWFrZXIiLCJjb25zdHJhaW50cyI6eyJtYWtlcl90b190YWtlciI6eyJmaXhlZF9yb3V0ZSI6W3sibm9kZV9pZCI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsImNoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSJ9XSwicmVxdWlyZWRfbm9kZXMiOlsiMDJhYjcwZGI2MGRiOTk5ODY4NmMwY2VkMmY4NDUwNDU5ZDhlZmQ2MmZlZjI4YWNjZWEyODUzOTIyOGUwMzZlMmE4Il0sInJlcXVpcmVkX2NoYW5uZWxzIjpbIjE1NjEzMDY1MTIwOTcyOSJdLCJyZXF1aXJlZF9sc3BzIjpbXSwibWF4X2ZlZV9tc2F0IjoiMCJ9LCJ0YWtlcl90b19tYWtlciI6eyJmaXhlZF9yb3V0ZSI6W3sibm9kZV9pZCI6IjAzMGM1ZjUyZDhlZTZjNjA4NzFlNTNmNDU4MzI5YzBjN2I1MmUzOWIyY2E2N2FiN2RhYTkxNWRlMTU5YjU4NGY2ZCIsImNoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSJ9XSwicmVxdWlyZWRfbm9kZXMiOlsiMDMwYzVmNTJkOGVlNmM2MDg3MWU1M2Y0NTgzMjljMGM3YjUyZTM5YjJjYTY3YWI3ZGFhOTE1ZGUxNTliNTg0ZjZkIl0sInJlcXVpcmVkX2NoYW5uZWxzIjpbIjE1NjEzMDY1MTIwOTcyOSJdLCJyZXF1aXJlZF9sc3BzIjpbXSwibWF4X2ZlZV9tc2F0IjoiMCJ9LCJtYXhfdG90YWxfY2x0dl9kZWx0YSI6MjAxNn19LCJwYXltZW50X2hhc2hfaGV4IjoiMTY1MjQwMWM5ODNhNWQ4N2ViMTc5OTNiZGY3YzBkZTdiMjMwOWZlNzIzMThmOTc4Nzc5NjNjZWM0NDdiZDI5OSIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsImNoYW5uZWxfc2NpZCI6MTU2MTMwNjUxMjA5NzI5LCJjb250cmFjdF9pZCI6ImNvbnRyYWN0OmlGMThFbFZFLUFudmt-R2MtdE1CMUsxfi1-aDJOMjA4LWZWdnFldWQtbEdoQ28zMCIsImFzc2V0X2Ftb3VudCI6MSwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJidGNfY2Fycmllcl9hbW91bnRfbXNhdCI6MTAwMDAwMCwibWFrZXJfZ2l2ZXNfcmdiIjp0cnVlLCJleHBpcnlfc2VjcyI6MzYwMCwiY3JlYXRlZF9hdF91bml4X3NlY3MiOjE3OTMzMzM5MzB9",
>   "payment_hash": "1652401c983a5d87eb17993bdf7c0de7b2309fe72318f97877963cec447bd299",
>   "info": {
>     "payment_hash": "1652401c983a5d87eb17993bdf7c0de7b2309fe72318f97877963cec447bd299",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8",
>     "channel_scid": "156130651209729",
>     "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1793333930",
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
>   "swap_string": "rgb-swap:v3:eyJxdW90ZSI6eyJleGVjdXRpb25faWQiOiI0NjRlYTgwMjhiNDMzMTAyZDczNjJhZjE4MWJlMTIxNzk4MjQ2ODE4ZTFlMGNjMmViNmVlMjUwN2FlN2ZjNDZmIiwibWFrZXIiOiIwMzBjNWY1MmQ4ZWU2YzYwODcxZTUzZjQ1ODMyOWMwYzdiNTJlMzliMmNhNjdhYjdkYWE5MTVkZTE1OWI1ODRmNmQiLCJ0YWtlciI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsInRha2VyX291dGJvdW5kX2NoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSIsImFzc2V0X2lkIjpbMTM2LDkzLDEyNCwxOCw4NSw2OCwyLDEyMywyMjgsMjUyLDEwMyw0NSw0OCwyOSw3NCwyMTUsMjU1LDIyNSwyMTYsMjIxLDE4MCwyNDEsMjQ1LDExMSwxNjksMjM1LDE1NywxNDgsMTA0LDY2LDE2MywxMjVdLCJhc3NldF9hbW91bnQiOiIxIiwiYnRjX2Ftb3VudF9tc2F0IjoiMTAwMDAwMCIsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoiMTAwMDAwMCIsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiY3JlYXRlZF9hdF91bml4X3NlY3MiOiIxNzkzMzMzOTMwIiwiZXhwaXJ5X3NlY3MiOjM2MDAsInNldHRsZW1lbnRfbW9kZSI6IkNpcmN1bGFySHRsYyIsIm1ha2VyX3RvX3Rha2VyX2ZlZV9wYXllciI6Ik1ha2VyIiwidGFrZXJfdG9fbWFrZXJfZmVlX3BheWVyIjoiTWFrZXIiLCJjb25zdHJhaW50cyI6eyJtYWtlcl90b190YWtlciI6eyJmaXhlZF9yb3V0ZSI6W3sibm9kZV9pZCI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsImNoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSJ9XSwicmVxdWlyZWRfbm9kZXMiOlsiMDJhYjcwZGI2MGRiOTk5ODY4NmMwY2VkMmY4NDUwNDU5ZDhlZmQ2MmZlZjI4YWNjZWEyODUzOTIyOGUwMzZlMmE4Il0sInJlcXVpcmVkX2NoYW5uZWxzIjpbIjE1NjEzMDY1MTIwOTcyOSJdLCJyZXF1aXJlZF9sc3BzIjpbXSwibWF4X2ZlZV9tc2F0IjoiMCJ9LCJ0YWtlcl90b19tYWtlciI6eyJmaXhlZF9yb3V0ZSI6W3sibm9kZV9pZCI6IjAzMGM1ZjUyZDhlZTZjNjA4NzFlNTNmNDU4MzI5YzBjN2I1MmUzOWIyY2E2N2FiN2RhYTkxNWRlMTU5YjU4NGY2ZCIsImNoYW5uZWxfc2NpZCI6IjE1NjEzMDY1MTIwOTcyOSJ9XSwicmVxdWlyZWRfbm9kZXMiOlsiMDMwYzVmNTJkOGVlNmM2MDg3MWU1M2Y0NTgzMjljMGM3YjUyZTM5YjJjYTY3YWI3ZGFhOTE1ZGUxNTliNTg0ZjZkIl0sInJlcXVpcmVkX2NoYW5uZWxzIjpbIjE1NjEzMDY1MTIwOTcyOSJdLCJyZXF1aXJlZF9sc3BzIjpbXSwibWF4X2ZlZV9tc2F0IjoiMCJ9LCJtYXhfdG90YWxfY2x0dl9kZWx0YSI6MjAxNn19LCJwYXltZW50X2hhc2hfaGV4IjoiNDY0ZWE4MDI4YjQzMzEwMmQ3MzYyYWYxODFiZTEyMTc5ODI0NjgxOGUxZTBjYzJlYjZlZTI1MDdhZTdmYzQ2ZiIsIm1ha2VyX3RvX3Rha2VyIjpbeyJub2RlX2lkX2hleCI6IjAyYWI3MGRiNjBkYjk5OTg2ODZjMGNlZDJmODQ1MDQ1OWQ4ZWZkNjJmZWYyOGFjY2VhMjg1MzkyMjhlMDM2ZTJhOCIsImNoYW5uZWxfc2NpZCI6MTU2MTMwNjUxMjA5NzI5fV0sInRha2VyX3RvX21ha2VyIjpbeyJub2RlX2lkX2hleCI6IjAzMGM1ZjUyZDhlZTZjNjA4NzFlNTNmNDU4MzI5YzBjN2I1MmUzOWIyY2E2N2FiN2RhYTkxNWRlMTU5YjU4NGY2ZCIsImNoYW5uZWxfc2NpZCI6MTU2MTMwNjUxMjA5NzI5fV0sImNvbnRyYWN0X2lkIjoiY29udHJhY3Q6aUYxOEVsVkUtQW52a35HYy10TUIxSzF-LX5oMk4yMDgtZlZ2cWV1ZC1sR2hDbzMwIiwiYXNzZXRfYW1vdW50IjoxLCJidGNfYW1vdW50X21zYXQiOjEwMDAwMDAsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImV4cGlyeV9zZWNzIjozNjAwLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6MTc5MzMzMzkzMH0",
>   "payment_hash": "464ea8028b433102d7362af181be121798246818e1e0cc2eb6ee2507ae7fc46f",
>   "info": {
>     "payment_hash": "464ea8028b433102d7362af181be121798246818e1e0cc2eb6ee2507ae7fc46f",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8",
>     "channel_scid": "156130651209729",
>     "contract_id": "contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1793333930",
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
>   "invoice": "lnbcrt100n1p4wgx42dq8v3jk6mcnp4q24hpkmqmwves6rvpnkjlpzsgkwcaltzlmeg4n829pfey28qxm32spp52sw6ldut77ntm7q9mw63qw2w0dffpzjnwxmr6u4e97spkvu7ajxqsp5dptfsvkg3sx6ahulte4zu4yf8r67e9hxte8nxsh2k5knxyf726nq9qyysgqcqzp2xqrrssrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqq7sqqsqqqqlgqqqqqqqqfqs68q94e3vrscztjxx8946trz4d0qefu9p8hmlpakyhcdlqn6n2updmsszj5v52wnt9htnntg76hxggne6e4n75e7u45pdxq9hchrdcspt84xf4"
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
> | payment_hash | 541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8 |
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
>   "payment_id": "541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c",
>   "preimage": "dbba56d97a242062ce98105b86387916482abe280653dcb1b7bbd5b9e5ff1c9b",
>   "amount_sats": "10",
>   "destination": "02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c
>
> ```

**Run:**

```bash
$ rgbldk pay get 541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"541dafb78bf7a6bdf805dbb510394e7b52908a5371b63d72b92fa01b339eec8c","preimage":"dbba56d97a242062ce98105b86387916482abe280653dcb1b7bbd5b9e5ff1c9b","secret":"68569832c88c0daedf9f5e6a2e548938f5ec96e65e4f3342eab52d33113e56a6"} |
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
>   "invoice": "lnbcrt130n1p4wgx4vdq0v3jk6medwdjkueqnp4q24hpkmqmwves6rvpnkjlpzsgkwcaltzlmeg4n829pfey28qxm32spp58cgs4wer4ysh70rlzssq72d69d3lv4rsm90md2nyxaam23f4hwzssp5mlqlpgfnvupuzqk854zl9t0nq640jyhl5g0sqlvsna9s6wwr0kxq9qyysgqcqzp2xqrrssrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqq7sqqsqqqqlgqqqqqqqqfq8qczy2xq2hl7u8gcjfudzzwvfhc9tn4d2y302kk4swu8446havdhuht37ae5fgz49mf8d5xzph3waqydvm00xtpdqu4afyhkm4mp2hspnjws3v"
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
>   "payment_id": "3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85
>
> ```

**Run:**

```bash
$ rgbldk pay get 3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"3e110abb23a9217f3c7f14200f29ba2b63f65470d95fb6aa64377bb54535bb85","preimage":"66511c88c248e6916822b0c8b7c59827af143f28e7c5d8b3bdf8d837dd66aa85","secret":"dfc1f0a1336703c102c7a545f2adf306aaf912ffa21f007d909f4b0d39c37d8c"} |
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
>   "invoice": "lnbcrt1p4wgx4ddqdv3jk6medweshynp4q24hpkmqmwves6rvpnkjlpzsgkwcaltzlmeg4n829pfey28qxm32spp5fzp6edwhkytl7tjxmfapsnpa7tf9lergz9dh37k88wnekl0m242qsp5sxg7xudjcx9zqjuf8msv9dyuk5ge9gsflxwaszh7axns0qg6znhq9qyysgqcqzp2xqrrssrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqq32cqqyqqqqlgqqqqqqqqfqsfhtr5xu6ucdkt3wpg9mwa8ua95x9hx5at4aaudl50wgenlvws532yj8p73fhws4a48n5l9l8q403mgpjwcydz333heudc8tlvdmmpcphpaljd"
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
>   "payment_id": "4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554
>
> ```

**Run:**

```bash
$ rgbldk pay get 4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4883acb5d7b117ff2e46da7a184c3df2d25fe468115b78fac73ba79b7dfb5554","preimage":"720a13f3e4d6eabd34179e40991a26cde432077ac6efd98527f1f5fa46f98a69","secret":"8191e371b2c18a204b893ee0c2b49cb51192a209f99dd80afee9a707811a14ee"} |
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
>   "invoice": "lnbcrt140n1p4wgx4wdqcv3jk6meddphkcepdvdkxz6tdnp4q24hpkmqmwves6rvpnkjlpzsgkwcaltzlmeg4n829pfey28qxm32spp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp5rhq4rv8jher5g23z2vqjmkp4crhq96858vvyx5g5s7268vucp95s9qyysgqcqzp2xqrrssrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqq7sqqsqqqqlgqqqqqqqqfqjl7a8gfnmf73flv40zpgta5achma3newmde8zdmc6ecpmgmuqmyzrrhwy4qp5wg5awts8csavyqgtn59dxn983pe6evual3dyzpghygq8k0dlt"
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
> | destination  | 02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8 |
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
> | kind_details    | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":"4242424242424242424242424242424242424242424242424242424242424242","secret":"1dc151b0f2be47442a2253012dd835c0ee02e8f43b184351148795a3b3980969"} |
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
>   "invoice": "lnbcrt150n1p4wgx4sdqhv3jk6meddphkcepdveskjmqnp4q24hpkmqmwves6rvpnkjlpzsgkwcaltzlmeg4n829pfey28qxm32spp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp5vvex5whs97waxnfhtpcgc799g797jjj65jhsytlggm362rts77ls9qyysgqcqzp2xqrrssrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqq7sqqsqqqqlgqqqqqqqqfqrhgqmsvcllztancjcfhfqqcggchxk7ftxqd0q5ujhvjjdrqqg7dre6z3aezplealmesuetgq3uq2rtlhteyw9f7gpdms7yjmnfvtcpqpxera3z"
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
> | kind_details    | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"63326a3af02f9dd34d3758708c78a5478be94a5aa4af022fe846e3a50d70f7bf"} |
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
>   "invoice": "lnbcrt120n1p4wgx4ndq0v3jk6medvfskx6cnp4qvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6pp5fsxtry53rhfznttglgphsqw2n3wp6aetxu5ppw3krmcs6ul083jqsp53lp95xtg0x0c92hgwd2p5cf2hhedegg4ek39sw78za7nlthewqts9qyysgqcqzp2xqrrssrzjq24hpkmqmwves6rvpnkjlpzsgkwcaltzlmeg4n829pfey28qxm32sqqqqyqqsagqqvqqqqlgqqqqqqqqfqjy3yr925za5h8c2cp0ncshl82lccyq3l5zutfrw77r0ghu3qs78spfnac339eklvd33ram9yt6hl4pen96p2fmp0edars0qndcevl8qpl33r2t"
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
>   "payment_id": "4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64",
>   "preimage": "08b183b0cedddbeee49341eb92583a5c7734645338bbebb6a7107b5f9b42135c",
>   "amount_sats": "12",
>   "destination": "030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64
>
> ```

**Run:**

```bash
$ rgbldk pay get 4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4c0cb192911dd229ad68fa037801ca9c5c1d772b372810ba361ef10d73ef3c64","preimage":"08b183b0cedddbeee49341eb92583a5c7734645338bbebb6a7107b5f9b42135c","secret":"8fc25a1968799f82aae873541a612abdf2dca115cda2583bc7177d3faef97017"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx4epgcsgvyqqqqpkqqqqpqqqsyn3868sv7hu8a4enhkaasw2s3ep39dj9wnntkr9rhqxrv6ekvttqqyplpnwrs2dfy6n0m443h5f738mzde0usmnzuqapnzksp9xreyuz93cqwsa4facta5xsaaan79znhugvsjh9cqln93sx2vvztqrvyrdzhr8wzxnjzs6u5tp4nqrydjh7c5xd9w74uglnallzdeze8z4emkum4kcu677k50vylsjqx2jlhscj7r0pwqv3qxgpne7q7nkpemsqdqwqqd553qsuffvd6k46wm6rzur2dc53nrl90hf3vggrahf2qw9wgs8aafvfwzut03s0nhwk4crt3d2d03yn69x7ln0rw36s"
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
> | offer_id                  | efa7f5eb29a67ae5be73c6f18cf7a2dc81246a9e634ea8c27f545f16ced0e03c   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 03edd2a038ae440fdea58970b8b7c60f9ddd6ae06b8b54d7c493d14defcde37475 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1793337540                                                         |
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
>   "payment_id": "d4680b21b901431961bba4289de7637b4aac647bfd3382b1a91d1e828860097e"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait d4680b21b901431961bba4289de7637b4aac647bfd3382b1a91d1e828860097e --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> d4680b21b901431961bba4289de7637b4aac647bfd3382b1a91d1e828860097e
>
> ```

**Run:**

```bash
$ rgbldk pay get d4680b21b901431961bba4289de7637b4aac647bfd3382b1a91d1e828860097e

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | d4680b21b901431961bba4289de7637b4aac647bfd3382b1a91d1e828860097e                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 1e6b3107fa5bea379a990c8b91a727aa0118ba54dd3693b591e0ff2b51b2d82a                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                               |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"efa7f5eb29a67ae5be73c6f18cf7a2dc81246a9e634ea8c27f545f16ced0e03c","payer_note":null,"payment_hash":"1e6b3107fa5bea379a990c8b91a727aa0118ba54dd3693b591e0ff2b51b2d82a","quantity":null} |
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
>   "refund": "lnr1qqsr4gqjpqhmyh3d7meuew4crqpc6e207qjrtk7kat4x5k5l2mzwzsc2qq8qg6hy9rr9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqss8rghf0lxlpdaftja3u03whwrrgytuddlspp40tz2x8qjy66w55tpty9hyetxw4hxgttyv4kk7kkzqyqqqmqqqqqsqqgzmqy26u20kmjmzjfjcaera38pnq70q2q8rczwkdm8elq3zm802vmszqhdz9kswgxq67z9frf4hpttn3vslkm9jzwkpf8z85udprlyghfp9qq8fvw73fh5eh204ckn397pte5jpp9vfup0u4h7e6qccdjq4em9mhyczwqxq5rgydqdpyvk690jnxlqv56vlh0dc665e56y85k2rz3rqexnarxxd8npac2sxap49a5zzxt8npwl7car6vg0u2dy3rp4dqz3j2zp33jug2v5fnpsqkm36jqlserfktwklds",
>   "payment_id": "456265689dc64294d9f65fa121bd22d17f4ec2b69332f45411a1e669346b9cdb"
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
> | absolute_expiry_unix_secs | 1793337542                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 038d174bfe6f85bd4ae5d8f1f175dc31a08be35bf804357ac4a31c1226b4ea5161 |
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
> lni1qqsr4gqjpqhmyh3d7meuew4crqpc6e207qjrtk7kat4x5k5l2mzwzsc2qq8qg6hy9rr9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqss8rghf0lxlpdaftja3u03whwrrgytuddlspp40tz2x8qjy66w55tpty9hyetxw4hxgttyv4kk7kkzqyqqqmqqqqqsqqgzmqy26u20kmjmzjfjcaera38pnq70q2q8rczwkdm8elq3zm802vmszqhdz9kswgxq67z9frf4hpttn3vslkm9jzwkpf8z85udprlyghfp9qq8fvw73fh5eh204ckn397pte5jpp9vfup0u4h7e6qccdjq4em9mhyczwqxq5rgydqdpyvk690jnxlqv56vlh0dc665e56y85k2rz3rqexnarxxd8npac2sxap49a5zzxt8npwl7car6vg0u2dy3rp4dqz3j2zp33jug2v5fnpsqkm36jqlserfktwkld4qacp2kuxmvrdenxrgdsxw6tuy2pzemrhavtl09zkvag598y3guqmw92qr8fp4mrlu079kkhgphrclp9f7mysxtx0q86jethmd942scgxfzxrqzqjrpsdwh4axddq885mpxktzkcpwxux0p7qzd0nrg2juvu6wp6mmjgqgs0ynf9v6y2nplspnu50zxkyrn2ljllt2l8jr0w8mk2dhvug43mttvk64w4agjrrqtdnm6hcq8hxmkyhw4fh5eky4369rhhm90hzjlwkrk56qzmau935xujlrhse0rmn4mwysk25wz5xy0dlywmekltz2ru7jqflyrn34ahhe97ugfrms8wrwy0s3lpmun8da83fv3v4lgns84nxdqp4kcttp4pdzrsqqqqqqqqqqqqqq9gqqqqqqqqqqqqgayjedltzjqqqqqq9yq34wgx4k4qs9wrw363pks5jytsw8h45l6uapp4mj8y54k2ry7zhge56s6y6fcta2qggwrtsrqgqqpvppqf7zf4qmj3cqpg6nc8skhvhrjswa58rsw9nzas335ca5frjmj4lktuzqf9sr6ql2rm6vsex5cmnrv6q4z5esyxfu682p0em0p7luev7n97jnyqxvdnjueavx0mnahkut8zakeaurgezxqdwazvnmaenk5yher9c
> payment_id: 570dd1d4436852445c1c7bd69fd73a10d77239295b2864f0ae8cd350d1349c2f
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
$ rgbldk pay wait 456265689dc64294d9f65fa121bd22d17f4ec2b69332f45411a1e669346b9cdb --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 456265689dc64294d9f65fa121bd22d17f4ec2b69332f45411a1e669346b9cdb
>
> ```

**Run:**

```bash
$ rgbldk pay get 456265689dc64294d9f65fa121bd22d17f4ec2b69332f45411a1e669346b9cdb

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | 456265689dc64294d9f65fa121bd22d17f4ec2b69332f45411a1e669346b9cdb                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 570dd1d4436852445c1c7bd69fd73a10d77239295b2864f0ae8cd350d1349c2f                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                          |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"570dd1d4436852445c1c7bd69fd73a10d77239295b2864f0ae8cd350d1349c2f","quantity":null} |
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
>   "refund": "lnr1qqstqkntu2mhpk50qn0d6f97ym5x338mxdr79g8h0yksmzc4q8v2d8g2qq8qg6hy9rr4qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqss8ct8l2k5xl3klq702nfjqtxxtpf68zr26tkltg8q0987r0297d8ftyfhyetxw4hxgttpvfskuer0dckkgetddadvyqgqqpkqqqqpqqqs8z59s7pf5d2ryzrsxtkmm935zgr7mnkenhm2vn3rur74dfdak9h9qypwy9pacray6przdsljcld39aajse29chezajrhxxz0jsams5m9w5sqwsmvh35gt9035myj5hwqqj93grt9j8g7p6q4y9dh29vat6nm9te0egwp2nelnjtu20uek7h07w8whxgr3p86965rn0vuqmzvu92wzas0es766gscwg6r2zx3v79c80egjefyegkkww07qepjw9ykq86j5d5qmqe98ug7xaaqwr39cg3jwqqn0g2yw4cs",
>   "payment_id": "06693a0bf02662625b36ef48a7667534459995248e00cffcadc1be29cd448532"
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
> | 3e110abb...4535bb85 | 3e110abb...4535bb85 | Succeeded | Bolt11       | Outbound | 13,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4113d54b...cc0b3a24 | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | d4680b21...8860097e | 1e6b3107...51b2d82a | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 092ed902...d2e4d255 | -                   | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4883acb5...7dfb5554 | 4883acb5...7dfb5554 | Succeeded | Bolt11       | Outbound | 11,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4c0cb192...73ef3c64 | 4c0cb192...73ef3c64 | Succeeded | Bolt11       | Inbound  | 12,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 1ff76688...6e6500df | 1ff76688...6e6500df | Pending   | Bolt11       | Inbound  | 10,084,000      | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 5aed8bb8...b26b40dd | -                   | Succeeded | Onchain      | Inbound  | 10,084,000      | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 073da480...b908fb65 | -                   | Succeeded | Onchain      | Outbound | 100,000,000     | 9,232,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 541dafb7...339eec8c | 541dafb7...339eec8c | Succeeded | Bolt11       | Outbound | 10,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 425ed4e4...6e55384c | 425ed4e4...6e55384c | Succeeded | Bolt11       | Outbound | 14,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 45626568...346b9cdb | 570dd1d4...d1349c2f | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | b33e8f9a...1fe1e7b9 | -                   | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 06693a0b...cd448532 | -                   | Failed    | Bolt12Refund | Outbound | 1,111           | -          | false       |
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
> | Required carrier (msat)         | 354,000        |
> |---------------------------------+----------------|
> | Required carrier reason         | holder_reserve |
> |---------------------------------+----------------|
> | Available inbound (msat)        | 98,340,000     |
> |---------------------------------+----------------|
> | Suggested action                | -              |
> |---------------------------------+----------------|
> | Minimum viable carrier (msat)   | 354,000        |
> |---------------------------------+----------------|
> | Minimum viable reason           | holder_reserve |
> |---------------------------------+----------------|
> | Default create carrier (msat)   | 354,000        |
> |---------------------------------+----------------|
> | Default create reason           | holder_reserve |
> |---------------------------------+----------------|
> | Admission threshold (msat)      | 330,000        |
> |---------------------------------+----------------|
> | Minimum allowed carrier (msat)  | 1,000          |
> |---------------------------------+----------------|
> | Holder reserve threshold (msat) | 354,000        |
> |---------------------------------+----------------|
> | Estimate only                   | yes            |
> +---------------------------------+----------------+
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason | Suggested action | Min msat | Min reason     | Default msat | Default reason  |
> +==============================================================================================================================================================================================================================================================================================================+
> | 073da4807097ee96221f76deaf0e7fa6799b57f66454cd5f99ccac00b908fb64 | 030f295e31a5066068d3f6eaf63a8ab7 | yes    | 98,340,000   | 0          | no      | yes     | yes        | 354,000       | 98,340,000     | -               | -                | 354,000  | holder_reserve | 354,000      | holder_reserve  |
> |------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------|
> | 3f164b1f0dd1121db7c5860e3af2db12cbc6d55fc9a9950e61015a07a4892a62 | 766b0a4eba9c49d3e29e0e24679dc07f | yes    | 88,294,124   | 10,045     | yes     | yes     | yes        | 330,000       | 88,294,124     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt30u1p4wgx4cdq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp5u8f6k6clvs3nct4xut7culgum2lh7wsvm7z9vny2qdp3vynrhqps9qrsgqxqrrsscqpjlp53pwhcyj4gsp8he8uvuknq8226ll7rkxakncl2mafawweg6zz5d7s7qpzrzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqq7sqqsqqqqlgqqqqqqqqfq5g7gk88nsq078z3rt8matlcq0xmc8ky6shtnd66zfxhafkxwhdf37sh49gjx7vx4hfnvk7kjl4evd7crmhmtkpl6q0vtvmgrphufp3qqf7qvwt",
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
> | Destination           | 02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30          |
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
>   "invoice": "lnbcrt50u1p4wgx4cdqjwfnkygrvdcsxgetddupp5hk6gfpskalj080zh6dra66jtfwjupk42lghjtyf3w5qjhtvs9ywqsp5fjf5shgmwjrkv2pg033ukykpqzay85yu32m5f6mnjgh76p66gqws9qrsgqxqrrsscqpjlp53pwhcyj4gsp8he8uvuknq8226ll7rkxakncl2mafawweg6zz5d7s7qp9rzjqvx975kcaekxppc72069sv5up3a49cum9jn84d764y2au9vmtp8k6qqqqyqqq7sqqsqqqqlgqqqqqqqqfqe2ye6zzuj053waaqw39zwg0fe3wp7u7kuqjpwspn2x6v076nghkhjzyxalptdv873qq24lahd5pa096f53hqrf38ng84kwnsudtmjvcqlc7u0x",
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
> | Payment hash          | bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 02ab70db60db9998686c0ced2f8450459d8efd62fef28accea28539228e036e2a8 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30          |
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
>   "payment_id": "bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c
>
> ```

**Run:**

```bash
$ rgbldk pay get bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"bdb4848616efe4f3bc57d347dd6a4b4ba5c0daaafa2f25913175012bad90291c","preimage":"65a780cd187b3ab007d176704eac21becb8d473cb195549ecf0cd6936cb98171","rgb":{"asset_amount":"5","contract_id":"contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30","direction":"Outbound","is_swap":false},"secret":"4c93485d1b74876628287c63cb12c100ba43d09c8ab744eb73922fed075a401d"} |
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
> | Holder reserve threshold (msat) | 354,000         |
> |---------------------------------+-----------------|
> | Estimate only                   | yes             |
> +---------------------------------+-----------------+
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason                  | Suggested action              | Min msat | Min reason     | Default msat | Default reason  |
> +============================================================================================================================================================================================================================================================================================================================================+
> | 073da4807097ee96221f76deaf0e7fa6799b57f66454cd5f99ccac00b908fb64 | 34f59a924e0d4bb495ee05c0c48b678d | yes    | 0            | 100,000    | yes     | no      | no         | 330,000       | 0              | carrier_inbound_capacity_too_low | add_inbound_carrier_liquidity | -        | -              | -            | -               |
> |------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------|
> | 3f164b1f0dd1121db7c5860e3af2db12cbc6d55fc9a9950e61015a07a4892a62 | 9125abe4de6a79eee1552ee687e6fc60 | yes    | 9,045,876    | 89,954     | yes     | yes     | yes        | 330,000       | 9,045,876      | -                                | -                             | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt40u1p4wgx46dq6wfnkygrvdcsxgetddusxyctrdvpp5np2csr8vx62tltg8nn9vslx26jszhxf5g9p385wnj7cj30jfkz0ssp559ckprvmade867dxsez39z027chq4h7tj2vr0xk9eu8yua7dkwvq9qrsgqxqrrsscqpjlp53pwhcyj4gsp8he8uvuknq8226ll7rkxakncl2mafawweg6zz5d7s7qpr9dm9p783qfrphe3re4vh25j2g77jfkpfqa7zf04jl84yk5dkutcj2sd7fewlqg4mkzdw5ek5ryug0a25m2hl35lcdd7ql6w2947xq0gqdxy8me",
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
> | Payment hash          | 9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 030c5f52d8ee6c60871e53f458329c0c7b52e39b2ca67ab7daa915de159b584f6d |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30          |
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
>   "payment_id": "9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f
>
> ```

**Run:**

```bash
$ rgbldk pay get 9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"9855880cec3694bfad079ccac87ccad4a02b9934414313d1d397b128be49b09f","preimage":"86d6e0f8d1157b1983f13fb1e1ebd5d6a891adf33f995d742316c9b666a389a6","rgb":{"asset_amount":"3","contract_id":"contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30","direction":"Outbound","is_swap":false},"secret":"a171608d9beb727d79a686451289eaf62e0adfcb9298379ac5cf0e4e77cdb398"} |
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
> | 34f59a92...c48b678d | 02ab70db...e036e2a8 | 140737488420865 | 1099657052161 | 1099546558466 | 100,000         | true  | true   | -                   | -         | -          |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------|
> | 9125abe4...87e6fc60 | 02ab70db...e036e2a8 | 156130651209729 | 1099519623172 | 1099653513219 | 100,000         | true  | true   | contract...-lGhCo30 | 6         | 4          |
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
> | 030f295e...f63a8ab7 | 030c5f52...9b584f6d | 140737488420865 | 1099546558466 | 1099657052161 | 100,000         | true  | true   | -                   | -         | -          |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------|
> | 766b0a4e...679dc07f | 030c5f52...9b584f6d | 156130651209729 | 1099653513219 | 1099519623172 | 100,000         | true  | true   | contract...-lGhCo30 | 4         | 6          |
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
>   "payment_id": "b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d
>
> ```

**Run:**

```bash
$ rgbldk pay get b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"b73306527cd00c7546da2b71d0944fa1133f894617e11ba11432eec94a0a582d","preimage":"76aaed668053bfb83ecb7a941c5e42929a0ef262390030211dc9eee0ebb82656"} |
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
>   "payment_id": "3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"3f9a6347f212ec3cd51942a5b3eb3f0019583b300430b0021613244a09d667ad","preimage":"1500a6c3626a18037c84ed59d831b7b0b1e577e43a1f5d54f56e88e0ca63615f"} |
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
> ChannelPending funding_txo=8f31c1c45fb9a4da26f7cc27acf8f402b570dba0afeab15a1288e80b3927bda3:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=8f31c1c45fb9a4da26f7cc27acf8f402b570dba0afeab15a1288e80b3927bda3:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=f1b43f0c5dc69ac997aed80a0c7f3d50
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
> ChannelReady user_channel_id=000df37919ba2870b307405f653ceadc
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
> | 9125abe4...87e6fc60 | 3f164b1f...a4892a62 | 02ab70db...e036e2a8 | negotiating | coop   | -            | 86,348     | contract...-lGhCo30 | 6         | -         |
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
> | 766b0a4e...679dc07f | 3f164b1f...a4892a62 | 030c5f52...9b584f6d | negotiating | coop   | -            | 11,047     | contract...-lGhCo30 | 4         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "6178563bbf1c32fc048fb2786bdb66d613a4b54aac7eaa6b6ce73ce3027dc1ef",
>   "15b16c8609bdb0fe40b787dc2e8051bd44aeda4a00681409a962c6ec49a21885",
>   "5476fa29a0b61d600fbe761f505e71ba65d7da0d54a6cfc1bed35ba71dad48a0",
>   "4e3d82d9f6b6213521df8fb0a04a13244b8f798adb74cef3999f94a1234c93d2",
>   "688dc83e6c136b5c39b92b38d3ee6627b9b58b3b29d92d1d986efeedd3401c94",
>   "1700419780937ed1d2d15803ab78e41a26d9899e42d3e265d7300ec4edd11bf3"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "01096522def301f395f9d5d36087e5aa281b1d40cf8b6fe72ffb2b016a4f1a0e"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "6434e6b9297d8fe00bfcaea4074465b1f160d97e5935dcb854095324ddb661dd"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "065a92d3fc032f77057194c1e1d75b77ebd6ee2b3da5585b7b6f89b266e9e33d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "2402228bb2fc10ee6decd2a1db9874b3115ceb252ed6aa9515b2654932281e63"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "5b54eb09a20837ebc6a19783c85210bdf3a5fe340116fddf0617113a3b46e9f5"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "7b140ab3e294ba133ba3881b49fc3eb0eeed011d1764aaec19bad31477b8c14a"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "3672000998a50f8ba1b1087c1dc3cd3f6b582c4ec6ff561ab8e14976970556d1"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "7ee31f630dc8d1f53a272b922d9cd9c7b2475c7534a04778f6b4ddd8c58b2102"
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
> | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |    13 |         0 |        0 |    13 |
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
> | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 | contract:iF18ElVE-Anvk~Gc-tMB1K1~-~h2N208-fVvqeud-lGhCo30 |    87 |         0 |        0 |    87 |
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
>   "user_channel_id": "1ff007dab641bdf17c69380560cd9451"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qvme5rdsvqdf9m7344g0q93vm3kq4n7a0jw36w8

```

**Result:**

> ```text
> [
>   "144513d095592c8be4e592e36c74fe564ea3b884158c66a6f5f55a2c84745870",
>   "3dc896ec2ae7f635223eca3332138afbf1143123f4393a9db4a6708d1f159aea",
>   "187c09b4bb03c4f01496f7e5774dd929737d2006d475c4db5baa7f31bf98fa7e",
>   "42ea1c9e6980ced88521c9eec2000aaa4962b1bd27c605de4eab4fa3681981b4",
>   "77a645394552d8ed69fb0f22e74349431bf01732018637669d89280d63f1abc4",
>   "2b5d1443f298ce6285b224869bb96252cb7617e0b26bc8bad2116dd939bc578f"
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
> | -               | b92eeaeb...2e138e53 | 02ab70db...e036e2a8 | broadcasting | unknown | -            | 46,654     |
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
> | -               | b92eeaeb...2e138e53 | 030c5f52...9b584f6d | broadcasting | unknown | -            | 1,000      |
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
