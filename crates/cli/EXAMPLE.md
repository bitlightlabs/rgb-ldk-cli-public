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
node_a=http://127.0.0.1:56763
node_b=http://127.0.0.1:8502
esplora=http://127.0.0.1:56759

```

## 1) Contexts (ctx)

Contexts let you name daemon endpoints (e.g. `node-a`, `node-b`) and switch the default target without repeatedly passing `--connect`.


### Add + show

**Run:**

```bash
$ rgbldk ctx add node-a --url http://127.0.0.1:56763 --use-now

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
> | *       | node-a | http://127.0.0.1:56763 |
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
> node-a -> http://127.0.0.1:56763
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
>   "node_id": "02521fe7f2e9dcb5ae713b0f70137a6185fe7337a784974b89504599419fb0274a"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:56766
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
>   "node_id": "03dbd45aa83eb290e535b9e1cee011cab100ba8566adc6fafc343971a3ae59cc5e"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:56768
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
> {"keystore_path":"/tmp/rgbldk-cli-example-ro187v0o/keystore-init-demo/keystore","mnemonic":"person fluid monster next artefact coast bean belt alpha body science silent brave cart ball age undo vote clinic law uncle syrup over cat","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-ro187v0o/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-ro187v0o/keystore-migrate-demo/keys_seed.bak.20260521-122526","ok":true}
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
> | 03dbd45a...ae59cc5e | 127.0.0.1:56768 | true      | true      |
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
>   "address": "bcrt1qjkkcwqwptf3zcr25kf8xqe0uj3jm08gfs02jdc"
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
>   "address": "bcrt1qjlynyxxz8cgxhkhyh24mxz6xk538hp5tmlfeh5"
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
> bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qjkkcwqwptf3zcr25kf8xqe0uj3jm08gfs02jdc 1

```

**Result:**

> ```text
> ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qjlynyxxz8cgxhkhyh24mxz6xk538hp5tmlfeh5 1

```

**Result:**

> ```text
> cd83d991638be69fbabc21c6d317eb7080721e8def3b4c10b5bf63323c977be0
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "50c99cbb6709388d082fea020ebd404cc23c6e32ced336090eb062e96c167c86",
>   "002b00672c7391dfb2e7302a8056c896d05b2ed3bdc4fc62c2baded05dea9dcd",
>   "213f4b061dbc3140d35d0010faeb7fdf8c8f6a68c491bd2b0fa4b7dc57692bea",
>   "27ac0771809afa237fe1d4dec7f60890f8efe6407989fd2ad8afdeb2169cf72b",
>   "58e3d7e75a6d76028ba9c1427ebd854cd4b5c0b138ccb1a2b1778383c1131820",
>   "3e462e5829bcab7f2d6420c8189b78e7300f347d8b8c047633383fe56bfba5f2"
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
>   "user_channel_id": "9ad6db67bc0b6b148666e8f682496ecd"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "0e35e4a7f30e98b37102875a5ed7dde10dfdde5580f65f3e4b368ddaef3ac371",
>   "4a2a2fb179ac1c51cbaeec4ae6d2168859f9febea80d016e3b0585a163d69de3",
>   "413eb29f352c6a35be349938a8e75cb8ce02d2aaebf2ff8e622b1d2a7f91ab86",
>   "1c6796b283e04a4d52ba56dd96274b9e93b0895e619e2d52fd10e3a509cadfb3",
>   "7b038f12637d863f34e005d5986b36c49c04053130749c4260576884e7ad2baf",
>   "53088ca59f86f9163c46a4ce5a8ff60b1de21e7553818d006dd6bade8154bef3"
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "0702d28515d39313540fb9d9f8730a024d793df447469a8ab8e311912db29a0f",
>   "06eb196ab22baccded01f7dd6cfc0af04d5cb7d64814ece23060eda68ee4d774",
>   "57de347eb7fdb6db9f257d0b0466fd32f15429a6714a99f26895e924e5edcf79",
>   "768a209b5e08dd738b9418a0a05020f39fdf5acc274dc22303e63cf212fc3727",
>   "784e56b43ee4c576cfbbbdb2510ac7acd6bbc4d96c0b817807fb12721cf42c87",
>   "596133e8b261513668e0abdb7da6d9b6b45339f8c9e2cfd551ecc1d8d1db9dae"
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
> BTC balance change: on-chain total +2,000 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (spendable) |     1.0 BTC |
> |--------------------------+-------------|
> | BTC Anchor reserve       |      0 sats |
> |--------------------------+-------------|
> | BTC Lightning (total)    |  2,000 sats |
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
> BTC balance change: on-chain total +55,989 sats, spendable 0 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (spendable) | 99,939,085 sats |
> |--------------------------+-----------------|
> | BTC Anchor reserve       |          0 sats |
> |--------------------------+-----------------|
> | BTC Lightning (total)    |     55,654 sats |
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
>   "address": "bcrt1qjkkcwqwptf3zcr25kf8xqe0uj3jm08gfs02jdc"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb issuers ls

```

**Result:**

> ```text
> +-------------+
> | issuer_name |
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
> +------+--------+-----------+---------------+-------------+
> | name | ticker | precision | issued_supply | contract_id |
> +=========================================================+
> +------+--------+-----------+---------------+-------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos ls

```

**Result:**

> ```text
> +--------------------------------------------------------------------+-------------+------------------+-----------------+-------+---------------------------+
> | outpoint                                                           | value_sats  | confirmed_height | rgb_allocations | mixed | spend_roles               |
> +===========================================================================================================================================================+
> | ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a:0 | 100,000,000 |                - | -               | false | FeeSupport,BlindingTarget |
> |--------------------------------------------------------------------+-------------+------------------+-----------------+-------+---------------------------|
> | d1c31098c459beb6a1b4c5eb09bb20647c566905427eb725a305edd115e64202:0 |       2,000 |                - | -               | false | FeeSupport,BlindingTarget |
> +--------------------------------------------------------------------+-------------+------------------+-----------------+-------+---------------------------+
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
> | ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a:0 | 100,000,000 |              102 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | d1c31098c459beb6a1b4c5eb09bb20647c566905427eb725a305edd115e64202:0 |       2,000 |              114 | false    |              - |
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
> 4ddaaad3c81195b981b17e51e4db1a34b53d84d4e184d3e2ab955e9e11e5f1ce
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "116bd11e027bf60dd3f0053f25d95f9511b2572610e4851efc3ae07dfffd257a"
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
> | ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a:0 | 100,000,000 |              102 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | d1c31098c459beb6a1b4c5eb09bb20647c566905427eb725a305edd115e64202:0 |       2,000 |              114 | false    |              - |
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
>   "reservation_id": "manual-reservation-1779366383-sNni4Wd0cY41geG2JX9fmFIbWykUdwVK",
>   "outpoint": "ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a:0",
>   "reserved_until_unix_secs": "1779366443"
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
> +-------------+
> | issuer_name |
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
>   "contract_id": "contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg",
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
>       "detail": "ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a:0"
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
> +-----------+--------+-----------+---------------+-----------------------------------------------------------+
> | name      | ticker | precision | issued_supply | contract_id                                               |
> +============================================================================================================+
> | DemoAsset | DEMO   |         0 |           100 | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
> +-----------+--------+-----------+---------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
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
$ rgbldk rgb contracts export --contract-id contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg --out /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg.raw --format raw --direct

```

**Result:**

> ```text
> exported=contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
> consignment_key=contract_export_8c8146e50b15f73cab8a52c5c13e4b2377abfc3da7e16aef5454be6ff16cd736
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg.raw
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=ba6260d09abebd26bb2120dc1d38d529756c444d7d2f4b466ca8549807fd447a:0
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
>         "detail": "contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_58f73de7e829518b45c423ef40b6aa3eeda9fc226532c86aa553e06083566f2c",
>     "contract_id": "contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg",
>     "ok": true
>   },
>   "out": "/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> consignment_key=contract_export_58f73de7e829518b45c423ef40b6aa3eeda9fc226532c86aa553e06083566f2c
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg.download.zip
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
>   "address": "bcrt1qjlynyxxz8cgxhkhyh24mxz6xk538hp5tmlfeh5"
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
> cc62393d20d90f8383070b55a735832d2b12518880cb1a313f16caff6f3242a1
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "705ec6b14ba3808572a0db75c5bc03b678769da4ac279eb7d3d37e607399d33d"
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
$ rgbldk rgb contracts import --contract-id contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg --file /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_10e8c6242b2ae5a1d863a17f993f1fd490f237cfb6bdb8caff11689416c9dc35
> contract_id=contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
> consignment_key=contract_import_10e8c6242b2ae5a1d863a17f993f1fd490f237cfb6bdb8caff11689416c9dc35
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts ls

```

**Result:**

> ```text
> +-----------+--------+-----------+---------------+-----------------------------------------------------------+
> | name      | ticker | precision | issued_supply | contract_id                                               |
> +============================================================================================================+
> | DemoAsset | DEMO   |         0 |           100 | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
> +-----------+--------+-----------+---------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
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
>   "invoice": "contract:tb@wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg/90@at:FmzMbRPK-XI9pRrae-W0_NMlT9-ykVbKLK1-dIxYOqqS-nNjYhw/?expiry=2026-05-21T13:26:39.390156+00:00",
>   "blinding_utxo_used": "0c60ff8c480f965b52dd6ffde8129a89bf7ef698988ad09d37cb7d5de435941e:1"
> }
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg/90@at:FmzMbRPK-XI9pRrae-W0_NMlT9-ykVbKLK1-dIxYOqqS-nNjYhw/?expiry=2026-05-21T13:26:39.390156+00:00
> scope=contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:FmzMbRPK-XI9pRrae-W0_NMlT9-ykVbKLK1-dIxYOqqS-nNjYhw
> data=90
> expiry=2026-05-21T13:26:39.390156+00:00
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
>   "txid": "cafe2066973edcf011ee64d9a6605db2075c6fe43fcbb1b0ab3d9ff4bfb79d5e",
>   "consignment_key": "rgb_consignment_contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg_cafe2066973edcf011ee64d9a6605db2075c6fe43fcbb1b0ab3d9ff4bfb79d5e"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> consignment_key=rgb_consignment_contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg_cafe2066973edcf011ee64d9a6605db2075c6fe43fcbb1b0ab3d9ff4bfb79d5e
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
> contract_id=contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
> amount=90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "31999e328ff4721ad62fd983880cb340e346926a2227d165315939dd8da379eb",
>   "61f4ada8aa56cad0dba11153eef693152dbdc1fd449ba8d25355d82111a499f9",
>   "175d375a135d23c6c3bf0225051d2095c8d9d68357741be8999baee7a04a501b",
>   "732cf60d26e33fe6ed21700b72cf088a8d67d606d03f8bf54685164204249ccf",
>   "39c0f9274bb23f787cbac36db83ade3ef3c52cf859b9ec77af7a355563720266",
>   "420ce79369ffe330098b5743fef11768cc5bec45f72549b5a44f401a333dc47f"
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
$ rgbldk rgb contracts balance contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
> |-------------+-----------------------------------------------------------|
> | mined       | 0                                                         |
> |-------------+-----------------------------------------------------------|
> | tentative   | 90                                                        |
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
$ rgbldk rgb contracts balance contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
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
$ rgbldk channel open --node-id <node_id_b> --addr <node_b_p2p> --amount-sats 100000 --push-msat 20000000 --private --rgb-contract-id <contract_id> --rgb-asset-amount 10 --rgb-context 'http://<A_HOST>:8501/api/v1/rgb/consignments/{txid}?format=zip'

```

**Result:**

> ```text
> {
>   "user_channel_id": "132dcf26822af62ebc80eff6f70b93f3"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "0b5b5d371c8cb8c71ffad671559ccd32d84f188c69f916a3feb04b8e277b5263",
>   "2aa6bb2fb3244391f8f3cb3a369eb668e2906561548421a1176e72649547df45",
>   "15226b0076e72cb678ac69c382dcec7ff9e3bc655852566d154d98dd6743504b",
>   "28279cc6f9eee3795c28195733976c5aeb4c96daa2cd74e43b94e3ce5a8b814c",
>   "0f578c8cbcf725537566208c383cc4ef7898a3afa4722f7f7495e0d3c60fdeb4",
>   "20579fd1b425d2e3ac79f359da887cac169e464422fd4d11bca67b4547fe57b1"
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
> | 132dcf26...f70b93f3 | 03dbd45a...ae59cc5e | 100000          | false | false  | contract...-kgzhxeg | 10        | 0          |
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
> BTC balance change: on-chain total -101,500 sats, spendable -101,500 sats, anchor reserve 0 sats, lightning 0 sats.
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
>   "invoice": "contract:tb@wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg/7@at:IRcXzDrj-PE5CFsYl-b3GqFvqj-VA0s3Mg2-9Yx7ALsO-DxkZ2g/?expiry=2026-05-21T13:27:01.505105+00:00",
>   "blinding_utxo_used": "4ddaaad3c81195b981b17e51e4db1a34b53d84d4e184d3e2ab955e9e11e5f1ce:1"
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
>   "txid": "d101ab339540d646d1a177396518832d1c887518d2993b429cbed61518c3faca",
>   "consignment_key": "rgb_consignment_contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg_d101ab339540d646d1a177396518832d1c887518d2993b429cbed61518c3faca"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> consignment_key=rgb_consignment_contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg_d101ab339540d646d1a177396518832d1c887518d2993b429cbed61518c3faca
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
> contract_id=contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg
> amount=7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "61014d04668de21dee4eb5698063582a3479cec5872c28aeb9629139c617507d",
>   "3f2b8f0892020ab4de68830943d1ca99d38e3874d948840bc6bd19948bdcdaa8",
>   "40284c019b5d1ddd97013f0322c53e6a3271874eebb544ad46e74e82e6a4d5e6",
>   "4aaa3818a5606822af37643929542040a5f43c70afea318ff4f9206b0401738a",
>   "4e4eabb27f225eb768d49ba729ca96380b9a8e09715edbff1f4b86e509a9f044",
>   "6423c03702a16e4b215623148a0adfddddd5717b9ecc757aee07f480fc22ec26"
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
$ rgbldk rgb contracts balance contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |
> |-------------+-----------------------------------------------------------|
> | mined       | 7                                                         |
> |-------------+-----------------------------------------------------------|
> | tentative   | 0                                                         |
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
>   "invoice": "lnbcrt100n1p4qa73qdq8v3jk6mcnp4q0dagk4g86efpef4h8suacq3e2cspw59v6kud7huxsuhrgawt8x9upp5jsqdyshe2uz80prnnrzkse2w30gqdlxncurwqwqfaur848rs9c9ssp5gmln2jp893eh4vjuwjjpc4uyk5504738c5hey7uey3j49ndxkmus9qyysgqcqpcxqrrssrzjqffplelja8wtttn38v8hqym6vxzluueh57zfwjuf2pzejsvlkqn55qqqqyqqpwcqqyqqqqlgqqqqqqqqfqk253rty0sy0h27w6ep3p6kchesme2cedz6cgmsktlrwprty3n64rlj7v7rp0k4pcrpkharystmgtladvh7pdug0hvmwy40pa5sndvlgpfm2wu8"
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
>   "payment_id": "9400d242f9570477847398c568654e8bd006fcd3c706e03809ef067a9c702e0b",
>   "preimage": "3874c07ca85dd8b3492b7ef5fbe65cf18c2ae46d45a1422e29df0cebe133bb54",
>   "amount_sats": "10",
>   "destination": "03dbd45aa83eb290e535b9e1cee011cab100ba8566adc6fafc343971a3ae59cc5e",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 9400d242f9570477847398c568654e8bd006fcd3c706e03809ef067a9c702e0b --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 9400d242f9570477847398c568654e8bd006fcd3c706e03809ef067a9c702e0b
>
> ```

**Run:**

```bash
$ rgbldk pay get 9400d242f9570477847398c568654e8bd006fcd3c706e03809ef067a9c702e0b

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 9400d242f9570477847398c568654e8bd006fcd3c706e03809ef067a9c702e0b                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"9400d242f9570477847398c568654e8bd006fcd3c706e03809ef067a9c702e0b","preimage":"3874c07ca85dd8b3492b7ef5fbe65cf18c2ae46d45a1422e29df0cebe133bb54","secret":"46ff3548272c737ab25c74a41c5784b528fafa27c52f927b99246552cda6b6f9"} |
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
>   "invoice": "lnbcrt1p4qa738dqdv3jk6medweshynp4q0dagk4g86efpef4h8suacq3e2cspw59v6kud7huxsuhrgawt8x9upp58uzslhstp9yyefvr9d7c3ugj23r4a49xketp3x8qjxn2l23g8uwqsp54wrkt6zpyxuth660v0zr0patj2umrznqmmnfqcqck5mc7ex53uds9qyysgqcqpcxqrrssrzjqffplelja8wtttn38v8hqym6vxzluueh57zfwjuf2pzejsvlkqn55qqqqyqqpwcqqyqqqqlgqqqqqqqqfq4gxvwy3h6h488scqujmqnkecprwa3ntvmnxmk40uvdwem2vlylwzycjse6ela9gajnv4tth5p2rlperzl0zwmdpshyd07d7x09yum2cp3p0h0d"
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
>   "payment_id": "3f050fde0b09484ca5832b7d88f11254475ed4a6b6561898e091a6afaa283f1c",
>   "preimage": "66225b7b401f7bb039904bd857826846ac92fc8cc952d5bc2193661de4068c84",
>   "amount_sats": "11",
>   "destination": "03dbd45aa83eb290e535b9e1cee011cab100ba8566adc6fafc343971a3ae59cc5e",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 3f050fde0b09484ca5832b7d88f11254475ed4a6b6561898e091a6afaa283f1c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 3f050fde0b09484ca5832b7d88f11254475ed4a6b6561898e091a6afaa283f1c
>
> ```

**Run:**

```bash
$ rgbldk pay get 3f050fde0b09484ca5832b7d88f11254475ed4a6b6561898e091a6afaa283f1c

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 3f050fde0b09484ca5832b7d88f11254475ed4a6b6561898e091a6afaa283f1c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"3f050fde0b09484ca5832b7d88f11254475ed4a6b6561898e091a6afaa283f1c","preimage":"66225b7b401f7bb039904bd857826846ac92fc8cc952d5bc2193661de4068c84","secret":"ab8765e84121b8bbeb4f63c43787ab92b9b18a60dee6906018b5378f64d48f1b"} |
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
>   "invoice": "lnbcrt120n1p4qa73ddq0v3jk6medvfskx6cnp4qffplelja8wtttn38v8hqym6vxzluueh57zfwjuf2pzejsvlkqn55pp5x2ct7etc7qpmctz73wdqajc30sjv87kyg7hhxsqxapsqm295ca3ssp59gluf6jdttwekgzdr5uj73rpcdfu8my38aevsmmpj536tj506pzq9qyysgqcqpcxqrrssrzjq0dagk4g86efpef4h8suacq3e2cspw59v6kud7huxsuhrgawt8x9uqqqqyqqdacqq5qqqqlgqqqqqqqqfq5hx99hhgqcylzfry7vwgtpm5wzrr2kfhvxycm5vpwsq4aq4ggyu39sze7dt96aaz96hu92936v4unmgrq4v6uyg4msdscgsygegeutgqrvn4tx"
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
>   "payment_id": "32b0bf6578f003bc2c5e8b9a0ecb117c24c3fac447af734006e8600da8b4c763",
>   "preimage": "c1d8729c92c7e4d033e1c0b626a8baa00628ccbaa9bdf8b8b9ad56953ef4a90e",
>   "amount_sats": "12",
>   "destination": "02521fe7f2e9dcb5ae713b0f70137a6185fe7337a784974b89504599419fb0274a",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 32b0bf6578f003bc2c5e8b9a0ecb117c24c3fac447af734006e8600da8b4c763 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 32b0bf6578f003bc2c5e8b9a0ecb117c24c3fac447af734006e8600da8b4c763
>
> ```

**Run:**

```bash
$ rgbldk pay get 32b0bf6578f003bc2c5e8b9a0ecb117c24c3fac447af734006e8600da8b4c763

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 32b0bf6578f003bc2c5e8b9a0ecb117c24c3fac447af734006e8600da8b4c763                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"32b0bf6578f003bc2c5e8b9a0ecb117c24c3fac447af734006e8600da8b4c763","preimage":"c1d8729c92c7e4d033e1c0b626a8baa00628ccbaa9bdf8b8b9ad56953ef4a90e","secret":"2a3fc4ea4d5add9b204d1d392f4461c353c3ec913f72c86f619523a5ca8fd044"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx5rcggsgv7qjjrlnl96wukkh8zwc0wqfh5cv9leen0fuyja9cj5z9n9qelvp8fgp9ue3xnufn9wp0kty6yfys9w9gtl8tc3xuqjt8sssa9qsczjc54wgzqtcn234r0qtp0zckq4f0j6nxwak66l3kwmxhayczh6caqh0gxgpykqq63tl29qev48svpe7zsyr4vkuk0svp8lkrcdl5mwpq3cpvkpkgum02ayts7tg9y36alhmhr6l3d06gkvaauw3kgy0j38r8ayqq9jnczevjqkf3a0qphvxd7wh08g7zhsvn2m6502js3c0qqzl9nztg3ereuu0w3wcu9qpcp2efzcss8xncf5wrdavyjye8mwn9z3y9avp0m0umwn97cwwxdw0gu5zc0y5p"
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
> | offer_id                  | c195788f93fdfeba388bbe50457887b4692d6fbcfd2aa87c3b00cc5f104e757d   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 039a784d1c36f58491327dba6514485eb02fdbf9b74cbec39c66b9e8e505879281 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1779370052                                                         |
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
>   "payment_id": "0393ae3de02912cb1d60b5573d5b5351b5722b55870a015f1bc394ba997de095"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 0393ae3de02912cb1d60b5573d5b5351b5722b55870a015f1bc394ba997de095 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 0393ae3de02912cb1d60b5573d5b5351b5722b55870a015f1bc394ba997de095
>
> ```

**Run:**

```bash
$ rgbldk pay get 0393ae3de02912cb1d60b5573d5b5351b5722b55870a015f1bc394ba997de095

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | 0393ae3de02912cb1d60b5573d5b5351b5722b55870a015f1bc394ba997de095                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"c195788f93fdfeba388bbe50457887b4692d6fbcfd2aa87c3b00cc5f104e757d","payer_note":null,"payment_hash":"71443a234f31b03f18d6daf0c61f45e293653569dc132c81feebbb78f60d1cfe","quantity":null} |
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
>   "refund": "lnr1qqs02hqr7ac5gt7ur389xy88x6wrzuz78yjls78cvujgugzc77ypeyc2qq8qg6s0pp99qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxzdtrqc3qxhauw8g9hpt9p5en247p5nc5jej2eal4f40sptewpthty9hyetxw4hxgttyv4kk7kh3q0dagk4g86efpef4h8suacq3e2cspw59v6kud7huxsuhrgawt8x9uq76nr4lhkwkzk4sd408c300dfr3u02puqgjpw0es74actd9gx4mkupqygpufvqm4cft53sqf38rlx8udctjujqgtzz29932uwcvjru99w20qqdqqs8pl7qmhym3aqzhhftahap0wukup2rfr0z5rzc4jqnfe07j7yeue3wqa8cs55c3x86uj9aw7e0g76r62ew63yjj9mj52gqyu7pv66p88uc24kfa8jfh9da3jyv7fdnsqrreswt9s6ej96aqwlu6qx6yzkm0wxu369h8g7c6w3pvazfgxr9g5vusr0jwn8hg6x87wcnwuskf7we0at8ckz7se78zug",
>   "payment_id": "8486d1b30b6027c91ed345b7fa2298119a4d0d657c446b649399ea4fedfefd02"
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
> | absolute_expiry_unix_secs | 1779370058                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 0309ab1831101afde38e82dc2b286999aabe0d278a4b32567bfaa6af8057970577 |
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
> lni1qqs02hqr7ac5gt7ur389xy88x6wrzuz78yjls78cvujgugzc77ypeyc2qq8qg6s0pp99qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxzdtrqc3qxhauw8g9hpt9p5en247p5nc5jej2eal4f40sptewpthty9hyetxw4hxgttyv4kk7kh3q0dagk4g86efpef4h8suacq3e2cspw59v6kud7huxsuhrgawt8x9uq76nr4lhkwkzk4sd408c300dfr3u02puqgjpw0es74actd9gx4mkupqygpufvqm4cft53sqf38rlx8udctjujqgtzz29932uwcvjru99w20qqdqqs8pl7qmhym3aqzhhftahap0wukup2rfr0z5rzc4jqnfe07j7yeue3wqa8cs55c3x86uj9aw7e0g76r62ew63yjj9mj52gqyu7pv66p88uc24kfa8jfh9da3jyv7fdnsqrreswt9s6ej96aqwlu6qx6yzkm0wxu369h8g7c6w3pvazfgxr9g5vusr0jwn8hg6x87wcnwuskf7we0at8ckz7se78zu2s06qf0qffplelja8wtttn38v8hqym6vxzluueh57zfwjuf2pzejsvlkqn55qk09dvmj5sykuskahz9qu8arsha55h7c2cjqm6dncpgqr5n0ps5lspq8gylphh0am9t9krqzj4ytgs9hr3ad64yz723pgz3j7qzv9g4zamjqq4lh8vjzsy6p2v7xwxc30npuw74pn7hfewsc6dfg7mkdqgw8qwhr08unxmvd62gmgyj9c84qtt7fw8jrfh45gnxckmr7ck2ptfphngc6hmaa6fdat9j92r680luuqrm7kefe0vxl8lkrm5sssm883llr2clvfhaaqal264axdpwnglwklldshjtzq3n9h8rhdgcye3h0nzwp0v6v3k3yanfvz4extsf5xmeml00ndlk6kyvdd9t6j8k9gtvwl7na5a6wejdetv79eh27ejlz8j0e9f4chxmzqey9zypatpzpentuqhlegs4fy5dx4rq2ckfmgsuqqqq86qqqqqqqqrgqqqqqqqqqqqqzqqqqqqqt46cnqqqpfqydg80509gyqqxwnga9q6an8lcs3ytjr7aempk7dkhswxdjnsavjwz8ze77t3nt2szzrs6uqczqqqtqggr9urpcmkvf2he4pc6eejxsrk82njln87rdmljp59f70dtmcsdrth0qsxv29qlwqt9e2tyh5my9hpegym706dnfwww036vwqfpjawgah4phg9ear5gw37ncmunnns9e5f5fcql3gjn5jlkendr5exkznfa6009s
> payment_id: 00674d1d2835d99ff88448b90fddcec36f36d7838cd94e1d649c238b3ef2e335
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
$ rgbldk pay wait 8486d1b30b6027c91ed345b7fa2298119a4d0d657c446b649399ea4fedfefd02 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 8486d1b30b6027c91ed345b7fa2298119a4d0d657c446b649399ea4fedfefd02
>
> ```

**Run:**

```bash
$ rgbldk pay get 8486d1b30b6027c91ed345b7fa2298119a4d0d657c446b649399ea4fedfefd02

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | 8486d1b30b6027c91ed345b7fa2298119a4d0d657c446b649399ea4fedfefd02                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"00674d1d2835d99ff88448b90fddcec36f36d7838cd94e1d649c238b3ef2e335","quantity":null} |
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
>   "refund": "lnr1qqsq39lsqds8gv5e7ledwa5qug35st8flahmaxmu6nj6psjce5vewfq2qq8qg6s0ppg9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqssyd4f9xlkcamncp8s2ncym7s8ntt9a38ag5pfzgvn9qkhj75epq0dtyfhyetxw4hxgttpvfskuer0dckkgetddad0zq7m63d2s04jjrjntw0pemsprj43qzag2e4dcma0cdpewx36ukwvtcpj8clxdsfuezx6f7z57l3ue0kxf9zpc0xrzw92gatuvg4zhewc56czqfc50jjmj5d8y4z62qys8d7ljh3nutx59ap8c4gmmznh8lmw20ajjqq6r4zwkasuf0zfu9m6maps5dtw8qsp0dz94snlg9984spakngcqgddqd33cduhc7jg8frspwccjwkxzhtaq9gt34jgrxvl8qqqfmc4vuqtu609szjy990j3wemd3euqgv7wdlasd3sy4qrxs03qdxm2sytnjnnhtmc72e2jc3vgaaa50hlytkzcs6krxpqdjnzuca5pzu6yqrzm9vg7k47ekhps8qglkg",
>   "payment_id": "276e4d892f730725eab549758122b31712fb146a63282bf701affb38e538e352"
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
> | 86d9a877...8a0bc268 | Succeeded | Onchain      | Outbound | 100,000,000     | 1,500,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 32b0bf65...a8b4c763 | Succeeded | Bolt11       | Inbound  | 12,000          | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | a0a2f4c6...e0cae9a0 | Succeeded | Onchain      | Outbound | 0               | 655,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 0393ae3d...997de095 | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 8486d1b3...edfefd02 | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 7a44fd07...d06062ba | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 5e9db7bf...6620feca | Succeeded | Onchain      | Outbound | 0               | 250,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 276e4d89...e538e352 | Failed    | Bolt12Refund | Outbound | 1,111           | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | cef1e511...d3aada4d | Succeeded | Onchain      | Inbound  | 10,000,000,000  | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 3f050fde...aa283f1c | Succeeded | Bolt11       | Outbound | 11,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 0242e615...9810c3d1 | Succeeded | Onchain      | Inbound  | 2,000,000       | 2,011,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 9400d242...9c702e0b | Succeeded | Bolt11       | Outbound | 10,000          | 0          |
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
$ rgbldk --color never --output json --pretty rgb ln invoice create --contract-id <contract_id> --asset-amount 5 --desc "rgb ln demo" --btc-carrier-amount-msat 5000000

```

**Result:**

> ```text
> {
>   "invoice": "lnbcrt50u1p4qa7jrdqjwfnkygrvdcsxgetddupp5fcffec4n6p7smkt4emzqgmyp707prwpr7pg7kyvm73mfgj5y825ssp5glw6h7zyrdtkp3m7zqvu98wx9tl5c3qzecwymr7x2j35jcyp7x2q9qrsgqxqrrsscqpjlp5czqdxdj3537fq8f4zhnlpx6dlrk7yx8cu9tjjxgw2s6fyr8pch5q7qp9d50wcjnsv4kjwfv8eayzaz7g3rsj409u86jdytz7gm6yjj6ugt34536egq4pdee6rx725t0qptws84wtqqxk7c5dymkupdz0hdzq60qq05g9l6"
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
> | payment_hash        | 4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9   |
> |---------------------+--------------------------------------------------------------------|
> | destination         | 03dbd45aa83eb290e535b9e1cee011cab100ba8566adc6fafc343971a3ae59cc5e |
> |---------------------+--------------------------------------------------------------------|
> | carrier_amount_msat | 5,000,000                                                          |
> |---------------------+--------------------------------------------------------------------|
> | expiry_secs         | 3600                                                               |
> |---------------------+--------------------------------------------------------------------|
> | contract_id         | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg          |
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
>   "payment_id": "4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9
>
> ```

**Run:**

```bash
$ rgbldk pay get 4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4e129ce2b3d07d0dd975cec4046c81f3fc11b823f051eb119bf476944a843aa9","preimage":"d8f1f10cd3e375b6ebe6feea358c942bc340876358df04df99e4140521aabb30","rgb":{"asset_amount":"5","contract_id":"contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg","direction":"Outbound","is_swap":false},"secret":"47ddabf8441b5760c77e1019c29dc62aff4c4402ce1c4d8fc654a3496081f194"} |
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
>   "invoice": "lnbcrt40u1p4qa7jfdq6wfnkygrvdcsxgetddusxyctrdvpp50c8d5099hgqxmz3x70wz5d27h8uqfczvsmln7f2ww0ep7ku3xnpqsp5thz356j2gxu7j4g4293yyphykra73wwwanq9mgtzu2y70zgxmhvq9qrsgqxqrrsscqpjlp5czqdxdj3537fq8f4zhnlpx6dlrk7yx8cu9tjjxgw2s6fyr8pch5q7qprnaqnnxycmju0vkg7dzh3prawu89ns37l8v3a7wn99fwsxf6hf8wxrx4rr2yp8ueadfxm956hycr80s9zwjr9v3hp63ca3vp3ta56pygq932vqy"
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
> | payment_hash        | 7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2   |
> |---------------------+--------------------------------------------------------------------|
> | destination         | 02521fe7f2e9dcb5ae713b0f70137a6185fe7337a784974b89504599419fb0274a |
> |---------------------+--------------------------------------------------------------------|
> | carrier_amount_msat | 4,000,000                                                          |
> |---------------------+--------------------------------------------------------------------|
> | expiry_secs         | 3600                                                               |
> |---------------------+--------------------------------------------------------------------|
> | contract_id         | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg          |
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
>   "payment_id": "7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2
>
> ```

**Run:**

```bash
$ rgbldk pay get 7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"7e0eda3ca5ba006d8a26f3dc2a355eb9f804e04c86ff3f254e73f21f5b9134c2","preimage":"8b71a7bfb75ce19833f8234fe0a5bd0fb338323ab47b6077929a447ef36ef851","rgb":{"asset_amount":"3","contract_id":"contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg","direction":"Outbound","is_swap":false},"secret":"5dc51a6a4a41b9e9551551624206e4b0fbe8b9ceecc05da162e289e78906ddd8"} |
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
> | 132dcf26...f70b93f3 | 03dbd45a...ae59cc5e | 100000          | true  | true   | contract...-kgzhxeg | 8         | 2          |
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
> | e976f575...d6e3fe32 | 02521fe7...9fb0274a | 100000          | true  | true   | contract...-kgzhxeg | 2         | 8          |
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
>   "payment_id": "4c10692de6f75bdaa11fc05a6e116c38eaca0e7fb78bf8f23bbf62ed03d3401f"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4c10692de6f75bdaa11fc05a6e116c38eaca0e7fb78bf8f23bbf62ed03d3401f --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 4c10692de6f75bdaa11fc05a6e116c38eaca0e7fb78bf8f23bbf62ed03d3401f
>
> ```

**Run:**

```bash
$ rgbldk pay get 4c10692de6f75bdaa11fc05a6e116c38eaca0e7fb78bf8f23bbf62ed03d3401f

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 4c10692de6f75bdaa11fc05a6e116c38eaca0e7fb78bf8f23bbf62ed03d3401f                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4c10692de6f75bdaa11fc05a6e116c38eaca0e7fb78bf8f23bbf62ed03d3401f","preimage":"26ae9680def0740d951ee6703e77acf55698cccac647fd05dfdeaf5f77a78e7a"} |
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
>   "payment_id": "4c9b04c0c01d23f17b0bfa81b13845206b95b1946bf7422baa1bb699c1585018"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4c9b04c0c01d23f17b0bfa81b13845206b95b1946bf7422baa1bb699c1585018 --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 4c9b04c0c01d23f17b0bfa81b13845206b95b1946bf7422baa1bb699c1585018
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 4c9b04c0c01d23f17b0bfa81b13845206b95b1946bf7422baa1bb699c1585018

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 4c9b04c0c01d23f17b0bfa81b13845206b95b1946bf7422baa1bb699c1585018                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4c9b04c0c01d23f17b0bfa81b13845206b95b1946bf7422baa1bb699c1585018","preimage":"51159df78041e1ce5c077b46771298c50b093e838ef0cc5dcef37a4aec2eafb0"} |
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
$ rgbldk --connect http://127.0.0.1:56763 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=0c60ff8c480f965b52dd6ffde8129a89bf7ef698988ad09d37cb7d5de435941e:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=0c60ff8c480f965b52dd6ffde8129a89bf7ef698988ad09d37cb7d5de435941e:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:56763 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=c12cb346400f459846252db04d6e720d
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:56763 events handled

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
> ChannelReady user_channel_id=9ad6db67bc0b6b148666e8f682496ecd
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "7874924cc4d03bcd3eaa141041948106bef62573498718c6efd779dbcd145cd7",
>   "68e79438ca9bee8538eb38781ef497fb294919366631cca9bb06a6604d4e7d9f",
>   "4379ad2686015d75dd10bc29985793c5257e1291c82c5da45bfe288377cf04e2",
>   "1a6b86ab0bafeefad3cb6472217383a3f2e58aee1b02b4c1d522d5bd9f534359",
>   "6c05ca374c455ea28c84969a68c2b5a90a48ca29463ae9afed2e2ecf4695f2fe",
>   "21b36b13db106177f5262c60338075c14e72bd80efff6406f1b5a9cf5343292f"
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
> BTC balance change: on-chain total +76,968 sats, spendable +76,968 sats, anchor reserve 0 sats, lightning 0 sats.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```

**Result:**

> ```text
> Wallet synced.
> BTC balance change: on-chain total +21,020 sats, spendable +21,020 sats, anchor reserve 0 sats, lightning 0 sats.
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
> | BTC On-chain (total)     | 1.09976563 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.09976563 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    76,375 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |    15 |         0 |        0 |    15 |
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
> | BTC On-chain (total)     | 1.10015189 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.10015189 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    21,020 sats |
> +--------------------------+----------------+
> +-----------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Contract                                           | Contract                                                  | Mined | Tentative | Offchain | Total |
> +==============================================================================================================================================================+
> | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg | contract:wIDTNlGk-fJAdNRX-n8JtN_O-3iGPjhV-ykZDlQ0-kgzhxeg |    85 |         0 |        0 |    85 |
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
>   "user_channel_id": "01998f674192550faec697a67f6966d1"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1qh3g4rze0qru86makzkcdxfdyl3xze0594vr5vf

```

**Result:**

> ```text
> [
>   "3f76e6f9e7866c35c65071c6c596fb377ff9ef98fed59327850ba11fb3b2e071",
>   "5b01b83e5731d129ab762069001b4e8357307e394657d48c563899a0a651d038",
>   "0599d5c53733bdb397626668169ad38c5fb634f40ddc78959be4c71f099de116",
>   "3abe443d95a793aa768223e95e8e4e7d5a6862ffc0db67a5324f4c98e44a65ff",
>   "1ea4e13efb740f3c39c1fa5b13786a2df7d30d861b34cebad6b2f5e47f816736",
>   "74d07200419a9b37f5725bf8e64ee5dd43d50188db661042ee4b423611ec576a"
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
