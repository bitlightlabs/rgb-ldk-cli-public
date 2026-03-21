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
node_a=http://127.0.0.1:61448
node_b=http://127.0.0.1:61450
esplora=http://127.0.0.1:3003

```

## 1) Contexts (ctx)

Contexts let you name daemon endpoints (e.g. `node-a`, `node-b`) and switch the default target without repeatedly passing `--connect`.


### Add + show

**Run:**

```bash
$ rgbldk ctx add node-a --url http://127.0.0.1:61448 --use-now

```

**Result:**

> ```text
> Context "node-a" created and set as active.
> Next: run `rgbldk node status` to verify the connection.
>
> ```

**Run:**

```bash
$ rgbldk ctx add node-b --url http://127.0.0.1:61450

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
> | *       | node-a | http://127.0.0.1:61448 |
> |---------+--------+------------------------|
> |         | node-b | http://127.0.0.1:61450 |
> +---------+--------+------------------------+
>
> ```

**Run:**

```bash
$ rgbldk ctx show

```

**Result:**

> ```text
> node-a -> http://127.0.0.1:61448
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
> node-b -> http://127.0.0.1:61450
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
>   "node_id": "03431a20a8ffdc8b09763f7d4bf370c48433c82f6ca1878ccc889f97a4af6de0bd"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:61452
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
>   "node_id": "02883ba3b52895d8c36c52121d8b2e0d2471c7f7db090f91ccaa4a338148b8c93c"
> }
>
> ```

**Run:**

```bash
$ rgbldk node listen

```

**Result:**

> ```text
> 127.0.0.1:61454
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
> {"keystore_path":"/tmp/rgbldk-cli-example-yvy9dhig/keystore-init-demo/keystore","mnemonic":"uphold weather page story until viable click vehicle morning position immense peanut claim will cactus choice bitter scorpion quality oyster will brown clean champion","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-yvy9dhig/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-yvy9dhig/keystore-migrate-demo/keys_seed.bak.20260312-121853","ok":true}
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
> | 02883ba3...48b8c93c | 127.0.0.1:61454 | true      | true      |
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
>   "address": "bcrt1q6808955tzdncl32erh33zy2flhjfyp0cs8d7pq"
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
>   "address": "bcrt1q5ra67mln8fun0c2j8qwju3lhgk0764khmld473"
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
> bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1q6808955tzdncl32erh33zy2flhjfyp0cs8d7pq 1

```

**Result:**

> ```text
> f41e2a9f386128a552a904f0894464e469277065ca84dfbe4b60f9048ae9b489
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1q5ra67mln8fun0c2j8qwju3lhgk0764khmld473 1

```

**Result:**

> ```text
> 65eef4108255a6514355bf8310f9863ec182aa647a52baa9ce5db1decd6f4284
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "5dd177f1a9ca245d360b43d579c3a4f310361259ce0c852654d262fb42831438",
>   "48d0fdb91f4596aa3f13684fc7e13c1427ece1a2f1a68955fd5cdeb1f712aff3",
>   "2d66afa98f8218e5c4589ad6e26e83611169f8afbc73e21201545ed89712c44a",
>   "21946b52c97627dce0fb0ce9cfe9295229ad3fd8ef3ee62ae1abc4db3a4acf12",
>   "142dee5e9753e7222a92e2ca99a733b5f52fd4dcf7f8b5a06bee7f73b27f3daf",
>   "641aaee5106d76887a96c7ab4f73ef91f4b4098a085b1354c9de3aff2bd9acb3"
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
$ rgbldk --connect http://127.0.0.1:61450 wallet sync

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
$ rgbldk --connect http://127.0.0.1:61450 wallet balance

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
$ rgbldk --connect http://127.0.0.1:61450 wallet balance --sats

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
>   "user_channel_id": "a2f103359c5bf4987caa028878118345"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "0cb283999a6d634c0f84d5280fc9d2daf3635e37346a0005206b28b07c8b0e80",
>   "5131b755e13e33ed18a38174d7c6da88789d1c1d5e35ac38145171444f0f303b",
>   "594d970b3d4bb1b3fef5abc8d73a25e89e0462723b1760da9c43dcd460335b2e",
>   "35e178d6e0eb97aa37f1f6d9e79bb44f53c94efdaa9433c7923ac82c81a721bd",
>   "412aa42ab8836b55332a0cd0b880cc93fbe791d070a6b7ad149bd46c2f7ce619",
>   "4b93d61097655ed7ac25e67342aea2470c1965591e71b25e7017c6fd8abc9e41"
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "354e57fac9522a60e46100afc0b98adcd3c99af4f45f58f642ecb98a3ce533f0",
>   "70b41960d8caca76fa8966588e4a58c93440ba39ff12adfe26a712a4aac9554a",
>   "7ac1a851c413c31ee61bc93ae3f248c8d7ebe73d0a98ac168dcb77f844d5dc0b",
>   "453c87cd3967759e80019c1724b85f078ef92eb33241ad5cdd3f58a1172920a8",
>   "1c78c5733e5adebc041244513beb258214f7b780205b384e886d47f7d572c7ef",
>   "24c8d04227b799ad89c891a173694cb4e5358aa7e85d4a69ab4a1a2a2b306f5e"
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
> | BTC Lightning (total)    |     55,396 sats |
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
>   "address": "bcrt1q6808955tzdncl32erh33zy2flhjfyp0cs8d7pq"
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
> f41e2a9f386128a552a904f0894464e469277065ca84dfbe4b60f9048ae9b489:0
> ba0f5a191912e30e9cc7ae6af8558fcd0c6fa14f6b041a0e1b5038bc8c53d994:0
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
> | ba0f5a191912e30e9cc7ae6af8558fcd0c6fa14f6b041a0e1b5038bc8c53d994:0 |       2,000 |              114 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | f41e2a9f386128a552a904f0894464e469277065ca84dfbe4b60f9048ae9b489:0 | 100,000,000 |              102 | false    |              - |
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
> 5c3c7a817536377646f792b8863c3c3f001d984b9cddb6cd00402b49ec465c68
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "0610e0598e360e1873d621642bc94e8a903509391e77d9fa1c22335becfc3940"
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
> | 5c3c7a817536377646f792b8863c3c3f001d984b9cddb6cd00402b49ec465c68:0 |  10,000,000 |              120 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | ba0f5a191912e30e9cc7ae6af8558fcd0c6fa14f6b041a0e1b5038bc8c53d994:0 |       2,000 |              114 | false    |              - |
> |--------------------------------------------------------------------+-------------+------------------+----------+----------------|
> | f41e2a9f386128a552a904f0894464e469277065ca84dfbe4b60f9048ae9b489:0 | 100,000,000 |              102 | false    |              - |
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
>   "reservation_id": "0waOzIubrEePRxvyAAZjEnE9VAlzrXms",
>   "outpoint": "5c3c7a817536377646f792b8863c3c3f001d984b9cddb6cd00402b49ec465c68:0",
>   "reserved_until_unix_secs": "1773318076"
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
>   "contract_id": "contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4",
>   "asset_id": "44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e",
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
>       "detail": "5c3c7a817536377646f792b8863c3c3f001d984b9cddb6cd00402b49ec465c68:0"
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
> | DemoAsset | DEMO   |         - |           100 | 44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
> +-----------+--------+-----------+---------------+------------------------------------------------------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
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
$ rgbldk rgb contracts export --contract-id contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 --out /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4.raw --format raw --direct

```

**Result:**

> ```text
> exported=contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4
> consignment_key=contract_export_afb861a31e3c1d8a7fb8cf2048947d8adfbb487283a1266ecc7d6387a2ea7bb9
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4.raw
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=5c3c7a817536377646f792b8863c3c3f001d984b9cddb6cd00402b49ec465c68:0
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip

```

**Result:**

> ```text
> {
>   "bytes": 2989,
>   "export": {
>     "checks": [
>       {
>         "name": "rgb_enabled",
>         "ok": true
>       },
>       {
>         "detail": "contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_35518a1efae59c6ef2b164e3ecec40bfd957830565af39b64038f82265a0f430",
>     "contract_id": "contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4",
>     "ok": true
>   },
>   "out": "/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> consignment_key=contract_export_35518a1efae59c6ef2b164e3ecec40bfd957830565af39b64038f82265a0f430
> wrote=/Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4.download.zip
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
>   "address": "bcrt1q5ra67mln8fun0c2j8qwju3lhgk0764khmld473"
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
> b5b7a4b5be8b0414adbc9aeb7a18023299f78ed4697b2da48f85d8563299ae26
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "1f94456a71f74716550effb3d9ec3d122ad41bebac46ab6c9137f8b4590e3d9e"
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
$ rgbldk rgb contracts import --contract-id contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 --file /Users/zijingzhang/work/rgb-ldk-cli/target/rgbldk-example/contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_1a21eb02bdc53d0ffa35ea1a246eccbb5a870818a8415d33703602f3eba2578f
> contract_id=contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4
> consignment_key=contract_import_1a21eb02bdc53d0ffa35ea1a246eccbb5a870818a8415d33703602f3eba2578f
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
> | DemoAsset | DEMO   |         - |           100 | 44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
> +-----------+--------+-----------+---------------+------------------------------------------------------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
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
>   "invoice": "contract:tb@RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4/90@at:V5pawn24-OY0Nmkxr-ZE8WMsVn-b2L8qG~w-pN5toniS-Nd3dnw/",
>   "blinding_utxo_used": "ba0f5a191912e30e9cc7ae6af8558fcd0c6fa14f6b041a0e1b5038bc8c53d994:1"
> }
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4/90@at:V5pawn24-OY0Nmkxr-ZE8WMsVn-b2L8qG~w-pN5toniS-Nd3dnw/
> scope=contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:V5pawn24-OY0Nmkxr-ZE8WMsVn-b2L8qG~w-pN5toniS-Nd3dnw
> data=90
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
>   "txid": "3606cc3eed3530fdd9215fc82c1edda114c3754d040469b65268ca8d9a9bb5f4",
>   "consignment_key": "rgb_consignment_contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4_3606cc3eed3530fdd9215fc82c1edda114c3754d040469b65268ca8d9a9bb5f4"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> consignment_key=rgb_consignment_contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4_3606cc3eed3530fdd9215fc82c1edda114c3754d040469b65268ca8d9a9bb5f4
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
$ rgbldk rgb onchain receive --file <a-to-b.zip> --format zip

```

**Result:**

> ```text
> asset_id=44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e
> amount=90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "4894ff1180e88feee4068747e0dbc625b84dbd195e2c2e3e21ae02288da992df",
>   "4d951c4a6f2a996817be7bc6013bf18c81df4b931f9c74e7c98f4ac364af732f",
>   "2d5115a4517a2cc5d66170ab18342d44046190f865efd13d8627e9f714d5f29e",
>   "3e47c860cda60a0a4d2b038fc99c85cda2179cd0afd49e51f2812d41c061b649",
>   "3c1b53fe0ed28e215b3b9ef7a294eac807e67726c4205e16662ad29a67186e93",
>   "33703b9f1d6ad88c24e931b4331a19c46b8ca6ebf3461b656add904dc82540c4"
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
> BTC balance change: on-chain total +10,000,000 sats, spendable +10,055,989 sats, anchor reserve 0 sats, lightning 0 sats.
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
$ rgbldk rgb contracts balance contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
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
$ rgbldk rgb contracts balance contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
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
>   "user_channel_id": "976a26cb3538018590843b167293a0db"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "0d326eff149372d2162b9b02d5a908ff12fddd026ff38ac4d51f1191c31cafb4",
>   "54d08a83abaedf3a1f40ba6516cf7ed673bacc9bfc7516d5ee1b457376e94524",
>   "3473e86f2b6af80be2115d0abc485351073d430879ac3d15f88aab9e64e5a268",
>   "5de152f3aec19e449b0d7c4f34e3a77aaca7ae980c64b9c924072d30a8fa85db",
>   "3e5022f31bcdf751072435ebffb06ae04eec6640cba328bb88b61517dfe4eb2b",
>   "6b2a6191469e766afd013a0f53ef86d6c0b86f8799dabb180324a9e9dcacccd9"
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
> | 976a26cb...7293a0db | 02883ba3...48b8c93c | 100000          | false | false  | 44742950...1165725e | 10        | 0          |
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
$ rgbldk --connect http://127.0.0.1:61450 wallet sync

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
>   "invoice": "contract:tb@RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4/7@at:ItieUP2u-8LJkuHRh-F_a48CPd-7IV74b8v-ADmOv8YH-A6CgDA/",
>   "blinding_utxo_used": "3ffa43ba4c045f3567fb23d1a641c8b73863e423b3aa8f2c1410db2c930e5970:2"
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
>   "txid": "3a2e64a5e2eb6f901305f649c28a1a3568255d7b97a49642993884fc4711b07e",
>   "consignment_key": "rgb_consignment_contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4_3a2e64a5e2eb6f901305f649c28a1a3568255d7b97a49642993884fc4711b07e"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> consignment_key=rgb_consignment_contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4_3a2e64a5e2eb6f901305f649c28a1a3568255d7b97a49642993884fc4711b07e
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
$ rgbldk rgb onchain receive --file <b-to-a.zip> --format zip

```

**Result:**

> ```text
> asset_id=44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e
> amount=7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "627c796c4c55c2c64f0793dec40f586f139cf9d87b9bb2a0f21a0936229d4dd0",
>   "766a3ce7edbfddacce63b8965f415de9054e0f2412de2b099236929e4636be30",
>   "2b83d4614146ea6cb34c8b8e2f19b359c906a7e0212bf5acecf53f61e0c5c7b6",
>   "19a2981c21d90785ed7cccb4b994d4462435bc655e24d8675816ba0ed0e6dbac",
>   "28ac7a7fe2a90ed17cb342d4f7974bcde52a2f4fd5a73f7e9bd0f2f1f66dbfe1",
>   "308b972c549ef28ddec843ef800e30231b8715ade3a19aa7920f7deecca719b4"
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
$ rgbldk rgb contracts balance contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4

```

**Result:**

> ```text
> +-------------+-----------------------------------------------------------+
> | Field       | Value                                                     |
> +=========================================================================+
> | contract_id | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |
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
>   "invoice": "lnbcrt100n1p5m9taudq8v3jk6mcnp4q2yrhga49z2a3smv2gfpmzewp5j8r3lhmvyslywv4f9r8q2ghryncpp5mjcuuuepcwxm6h4j7au9uhra8y66653xcs5vpul9xcfd3882t5gssp5ke4ngdn45qw3aasa90s6ahqtcy96e87l3kxenwe2n0vr7z9jma2q9qyysgqcqpcxqrrssrzjqdp35g9gllwgkztk8a75humscjzr8jp0djsc0rxv3z0e0f90dhst6qqqqyqqrvqqquqqqqlgqqqqqqqqfqw9024nt5pyqw7v23l77ff6tngsv8vrfnrye48tw05jjylrc723m3wfsv869j5mtqqu4et7yv23f887vmelmlzza9487stw3h97klcpcpgzdq66"
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
>   "payment_id": "dcb1ce7321c38dbd5eb2f7785e5c7d3935ad5226c428c0f3e53612d89cea5d11",
>   "preimage": "d2633979778f04b078e0894a4f76227b37dbded00082d83f853eb02ce4c5e2fb",
>   "amount_sats": "10",
>   "destination": "02883ba3b52895d8c36c52121d8b2e0d2471c7f7db090f91ccaa4a338148b8c93c",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait dcb1ce7321c38dbd5eb2f7785e5c7d3935ad5226c428c0f3e53612d89cea5d11 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> dcb1ce7321c38dbd5eb2f7785e5c7d3935ad5226c428c0f3e53612d89cea5d11
>
> ```

**Run:**

```bash
$ rgbldk pay get dcb1ce7321c38dbd5eb2f7785e5c7d3935ad5226c428c0f3e53612d89cea5d11

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | dcb1ce7321c38dbd5eb2f7785e5c7d3935ad5226c428c0f3e53612d89cea5d11                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"dcb1ce7321c38dbd5eb2f7785e5c7d3935ad5226c428c0f3e53612d89cea5d11","preimage":"d2633979778f04b078e0894a4f76227b37dbded00082d83f853eb02ce4c5e2fb","secret":"b66b343675a01d1ef61d2be1aedc0bc10bac9fdf8d8d99bb2a9bd83f08b2df54"} |
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
>   "invoice": "lnbcrt1p5m9t79dqdv3jk6medweshynp4q2yrhga49z2a3smv2gfpmzewp5j8r3lhmvyslywv4f9r8q2ghryncpp5cqahxff600vsxj5p5l0dk3ufz6y4qmf7djgvgq0yj3utjg3dy5yssp54hyw987q455rn0fmk7usgd983yynd6ewjajsq5w7tkgzmg6u0rsq9qyysgqcqpcxqrrssrzjqdp35g9gllwgkztk8a75humscjzr8jp0djsc0rxv3z0e0f90dhst6qqqqyqqrvqqquqqqqlgqqqqqqqqfq3xk7ll07vpqlcv4zxkz034mtqrqh20ylthxnwvmpdz3gpr69u47sa75fhdsu84aspqhmvn489drhety0zej09783jw8fp35w2kjtlhgq2n9fjr"
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
>   "payment_id": "c03b73253a7bd9034a81a7dedb47891689506d3e6c90c401e49478b9222d2509",
>   "preimage": "9e3c9ffe33590bde6bd1dc778fa9969438c5047f75b756faddcbc168bca25cb0",
>   "amount_sats": "11",
>   "destination": "02883ba3b52895d8c36c52121d8b2e0d2471c7f7db090f91ccaa4a338148b8c93c",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait c03b73253a7bd9034a81a7dedb47891689506d3e6c90c401e49478b9222d2509 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> c03b73253a7bd9034a81a7dedb47891689506d3e6c90c401e49478b9222d2509
>
> ```

**Run:**

```bash
$ rgbldk pay get c03b73253a7bd9034a81a7dedb47891689506d3e6c90c401e49478b9222d2509

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | c03b73253a7bd9034a81a7dedb47891689506d3e6c90c401e49478b9222d2509                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"c03b73253a7bd9034a81a7dedb47891689506d3e6c90c401e49478b9222d2509","preimage":"9e3c9ffe33590bde6bd1dc778fa9969438c5047f75b756faddcbc168bca25cb0","secret":"adc8e29fc0ad2839bd3bb7b90434a7890936eb2e97650051de5d902da35c78e0"} |
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
>   "invoice": "lnbcrt120n1p5m9t7vdq0v3jk6medvfskx6cnp4qdp35g9gllwgkztk8a75humscjzr8jp0djsc0rxv3z0e0f90dhst6pp54atky4kywc3t9scq4f28ey9j755hl3v86sgsc83gcsfzp4qpzlcssp58ru0ltgh2jjnxjz84eh55h55kuhnjct8augv8h7ftsc427h9xszq9qyysgqcqpcxqrrssrzjq2yrhga49z2a3smv2gfpmzewp5j8r3lhmvyslywv4f9r8q2ghryncqqqqyqqgfsqqyqqqqlgqqqqqqqqfqklg52a7njvpwewfrqykr9885ppq659llusm6p95kewazhsvpf39xhpg9tqpx65qjd9wprsmq2szt273rcd995pd3dg9cftwve0reedspv32kf6"
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
>   "payment_id": "af576256c47622b2c300aa547c90b2f5297fc587d4110c1e28c41220d40117f1",
>   "preimage": "1b64ef5c885959ce798e3b776f4b0c732f24bb4d88799c923dc7bc845f8c213a",
>   "amount_sats": "12",
>   "destination": "03431a20a8ffdc8b09763f7d4bf370c48433c82f6ca1878ccc889f97a4af6de0bd",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait af576256c47622b2c300aa547c90b2f5297fc587d4110c1e28c41220d40117f1 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> af576256c47622b2c300aa547c90b2f5297fc587d4110c1e28c41220d40117f1
>
> ```

**Run:**

```bash
$ rgbldk pay get af576256c47622b2c300aa547c90b2f5297fc587d4110c1e28c41220d40117f1

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | af576256c47622b2c300aa547c90b2f5297fc587d4110c1e28c41220d40117f1                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"af576256c47622b2c300aa547c90b2f5297fc587d4110c1e28c41220d40117f1","preimage":"1b64ef5c885959ce798e3b776f4b0c732f24bb4d88799c923dc7bc845f8c213a","secret":"38f8ffad1754a5334847ae6f4a5e94b72f396167ef10c3dfc95c31557ae53404"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczxnv4ausgv7q6rrgs23l7u3vyhv0maf0ehp3yyx0yz7m9ps7xvezylj7j27m0qh5p30turftse574rs6lyqyk2s4c0ehmn35q0vmm7us9t5vzpefw0f4qzqwhptewdlmmhmarc525a3xhypxs88qp8prvwrnqzza6ns8pp5xfrxqq6jwph2v9msrwcvw9clv963gspszvk9h276nxjyzxepgp5ak2hk73jf6c8lrdrx397qdc6f6f5m60hgf95r560qey45tlcajcq9n4js8geyszwhpfuxnne29qkx5de5wtvwt3nr97202shuwhvxy0l9q5a85tsdd6xj9nug8u5zcss9qq5l2a3tp99t4e7lcn897c2r46z79rfyl7ceh26htn7jdst8nqe"
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
> | offer_id                  | 21f6252120b7c6a07a4ca0e6c3eb852320d8b1f2e01733d49f87b6895795e98e   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 028014fabb1584a55d73efe2672fb0a1d742f146927fd8cdd5abae7e9360b3cc19 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1773321700                                                         |
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
>   "payment_id": "7662b57591cf3b74eac18c9f1527cc9e8a529546d27d1a98682bfbba607b98fe"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 7662b57591cf3b74eac18c9f1527cc9e8a529546d27d1a98682bfbba607b98fe --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 7662b57591cf3b74eac18c9f1527cc9e8a529546d27d1a98682bfbba607b98fe
>
> ```

**Run:**

```bash
$ rgbldk pay get 7662b57591cf3b74eac18c9f1527cc9e8a529546d27d1a98682bfbba607b98fe

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | 7662b57591cf3b74eac18c9f1527cc9e8a529546d27d1a98682bfbba607b98fe                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"21f6252120b7c6a07a4ca0e6c3eb852320d8b1f2e01733d49f87b6895795e98e","payer_note":null,"payment_hash":"baba6285e5a99de3298c9dffdfc42108753ec134c50be8c1b21ef420697aa657","quantity":null} |
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
>   "refund": "lnr1qqs00f2zqszdc38vrpupyprwv64q3ryj5n2sguwdtf4n8el6km86wng2qq8qg6djhhk9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxwvfy2x8zwf9rvft0lhh24w3q5tdxxp6adag496v282clugd86nrty9hyetxw4hxgttyv4kk7kh3q2yrhga49z2a3smv2gfpmzewp5j8r3lhmvyslywv4f9r8q2ghryncqsjc4v982f7wxlu63c8jfpvgrv8xq3ujeylwg2lf64a88juhnew8ypq9x0w0ghqmxwnzfvex9885tk6dq6gthu78dcfmm24rhkmdeuq7rqpqqdwfzph34mdheu0g9ck8g86fy0hgtleua0rcqkf5rxnzqczc0qa6p7wkran3tjs6j2swhv79u0k2w4uqh4nht5d6z88afe265qyurk5zz2yz704s27ewse00hws7py4z9hp0ffk6fz5znh80xn425y86t82gt7ngxrt90mrq3qvxqrl4ag0gw6lhyvez4qsq7hucnj9zzf6ry04gy9nn3ya2txupppm9g",
>   "payment_id": "846946c21ea721d016bf522e97699cbfcabd0c48ca07a3820eab6979453d856e"
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
> | absolute_expiry_unix_secs | 1773321708                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 033989228c7139251b12b7fef7555d10516d3183aeb7a8a974c51d58ff10d3ea63 |
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
> lni1qqs00f2zqszdc38vrpupyprwv64q3ryj5n2sguwdtf4n8el6km86wng2qq8qg6djhhk9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxwvfy2x8zwf9rvft0lhh24w3q5tdxxp6adag496v282clugd86nrty9hyetxw4hxgttyv4kk7kh3q2yrhga49z2a3smv2gfpmzewp5j8r3lhmvyslywv4f9r8q2ghryncqsjc4v982f7wxlu63c8jfpvgrv8xq3ujeylwg2lf64a88juhnew8ypq9x0w0ghqmxwnzfvex9885tk6dq6gthu78dcfmm24rhkmdeuq7rqpqqdwfzph34mdheu0g9ck8g86fy0hgtleua0rcqkf5rxnzqczc0qa6p7wkran3tjs6j2swhv79u0k2w4uqh4nht5d6z88afe265qyurk5zz2yz704s27ewse00hws7py4z9hp0ffk6fz5znh80xn425y86t82gt7ngxrt90mrq3qvxqrl4ag0gw6lhyvez4qsq7hucnj9zzf6ry04gy9nn3ya2txupppm92s06qf0qdp35g9gllwgkztk8a75humscjzr8jp0djsc0rxv3z0e0f90dhst6qm9dpj2ppc3xnhn7qwly2qe8mytedthpwfcp8ervc79ye7dfgkc6gpq90y83znt486sgeutaajkdw5au5elyd22p0dzxqrulc2cu8uzl5hgqq4jdn5zje9zy4acet2g0dwrac8cwzjzhv807z2nfwynfhaa839xpksxkystpqrrn5ywygz9q2pgt8yga69vntqzu808zxevdd4v2hedcg620ssyp0t3lvx46047zqrmyxtaysxuhpfxhwfdtedkkw79xnwmrfh9trvfc5kgl3ce8tkhfcjq6j9uxljw72rlkqrz7ejpjlvyg3tmgl5wk6teu6g0cxqk2se5jezvkyf5sxjjalp5qx5l4s4vayf796vwz0t03gqyepz5yzq36mfwtlr063xkt5wh0f9epvq9dt6xatedx2dalcfvpjl4gtff4gsuqqqq86qqqqqqqqrgqqqqqqqqqqqqzqqqqqqqt46cnqqqpfqydxe2lh4gyzvre9qaflt2hfzzsygu6tfz826w8tgsh8swpq85dmg7yndjl52f82szzrs6uqczqqqtqggrjjup4ydt4xerp7xtph7edy8lt0ays6w2yv2de5zptd0uswjtrnhlqs9pe4dmf4p04k9740mr4mz8m6hh3amhv9rt4zeaj4m386m7qqvy7234g9a3mgdh6laxpf35x5ewnp3kkv2vs6whx7kv9fnk5uvvgve97
> payment_id: 983c941d4fd6aba4428111cd2d223ab4e3ad10b9e0e080f46ed1e24db2fd1493
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
$ rgbldk pay wait 846946c21ea721d016bf522e97699cbfcabd0c48ca07a3820eab6979453d856e --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 846946c21ea721d016bf522e97699cbfcabd0c48ca07a3820eab6979453d856e
>
> ```

**Run:**

```bash
$ rgbldk pay get 846946c21ea721d016bf522e97699cbfcabd0c48ca07a3820eab6979453d856e

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | 846946c21ea721d016bf522e97699cbfcabd0c48ca07a3820eab6979453d856e                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"983c941d4fd6aba4428111cd2d223ab4e3ad10b9e0e080f46ed1e24db2fd1493","quantity":null} |
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
>   "refund": "lnr1qqsxc96yaumlnaj7wp35dsxelcmd8pnwa2vv0xzvcyqhtze6zwa8u6g2qq8qg6djhhe9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqssxpnzymxc2jp2jzhvjy9zx7ymlpe6z3znz6y6mljj3kwxjrzhx8rxtyfhyetxw4hxgttpvfskuer0dckkgetddad0zq5g8w3m22y4mrpkc5sjrk9jurfyw8rl0kcfp7gue2j2xwq53wxf8spnkcg9hya63xzdh4y7872azhdxucxurwfxw5ldrqs8y3au5lv8zjszqvm0dsnghrc9x6cwccnrd4ktyv87t8ptnwfd82me4vrc9s0grep4vqq6cgguw87fzzavt5824pvfnu496mj7dh7l7e9sg7z46gp6ekdmxsdccqxks89z29fvgagpxc3dge7fpztf2czmr9r44t2e3nsqfezml6ud7vya6n6nss07nvwdzcmnuuncwf5re6djk5l75jdw8excky6nlt9ufpl4t6qlma6hwcmspymv7axzmma73af2249g2wava0qkdry4hr8av6l5e934x6ul9ys",
>   "payment_id": "7d3dcece5144d176fe6ac6bc956e2efe3c7d2743cff6a0b9651021b317890bf8"
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
> | c03b7325...222d2509 | Succeeded | Bolt11       | Outbound | 11,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 94d9538c...195a0fba | Succeeded | Onchain      | Inbound  | 2,000,000       | 2,011,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 89b4e98a...9f2a1ef4 | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | dcb1ce73...9cea5d11 | Succeeded | Bolt11       | Outbound | 10,000          | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | af576256...d40117f1 | Succeeded | Bolt11       | Inbound  | 12,000          | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | f4b59b9a...3ecc0636 | Succeeded | Onchain      | Outbound | 0               | 250,000    |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 7d3dcece...17890bf8 | Failed    | Bolt12Refund | Outbound | 1,111           | -          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 685c46ec...817a3c5c | Succeeded | Onchain      | Inbound  | 10,000,000,000  | 2,820,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 846946c2...453d856e | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 70590e93...ba43fa3f | Succeeded | Onchain      | Outbound | 100,000,000     | 1,500,000  |
> |---------------------+-----------+--------------+----------+-----------------+------------|
> | 7662b575...607b98fe | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          |
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
>   "invoice": "lnbcrt50u1p5m9tlxdqjwfnkygrvdcsxgetddupp575x7tfhjly5n2fcraufq34hqv3ldmrekcqmvtrw4zhpumshtmp7ssp5k9ze2tm605ky2edmnq2ec9uatngc4k0ejwrwzf4m4clx6c5hzy4q9qrsgqxqrrsscqpjlp5g36zj59sr4x5gfymjka78lqzpcgyg6aqa2j9gexv4j50kyt9wf0q7qp9dsknvpyjjgcn8x839jt75dhaktpaykft9akagvvvgz9v7r58jd7jtm3q2wxj434cjlyfynn8gy7kggxmnd59a2c3392pjdw54ya5hssql9yayq"
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
> | payment_hash        | f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d   |
> |---------------------+--------------------------------------------------------------------|
> | destination         | 02883ba3b52895d8c36c52121d8b2e0d2471c7f7db090f91ccaa4a338148b8c93c |
> |---------------------+--------------------------------------------------------------------|
> | carrier_amount_msat | 5,000,000                                                          |
> |---------------------+--------------------------------------------------------------------|
> | expiry_secs         | 3600                                                               |
> |---------------------+--------------------------------------------------------------------|
> | asset_id            | 44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e   |
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
>   "payment_id": "f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d
>
> ```

**Run:**

```bash
$ rgbldk pay get f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d                                                                                                                                                                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"f50de5a6f2f929352703ef1208d6e0647edd8f36c036c58dd515c3cdc2ebd87d","preimage":"32e08b3e3724291b5f835621b354fa9d41801e296bbddaaa8d8ab233a1fcd14f","rgb":{"asset_amount":"5","asset_id":"44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e","direction":"Outbound","is_swap":false},"secret":"b145952f7a7d2c4565bb98159c179d5cd18ad9f99386e126bbae3e6d6297112a"} |
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
>   "invoice": "lnbcrt40u1p5m9tlwdq6wfnkygrvdcsxgetddusxyctrdvpp53nsunxqkranau9tztk4awwujhprl77a4kuwst5rq2e626dx3emdqsp5yd7pepqpwsvly4z6cxz6x8sms044ygswy89074k0mqtgxjenvv0q9qrsgqxqrrsscqpjlp5g36zj59sr4x5gfymjka78lqzpcgyg6aqa2j9gexv4j50kyt9wf0q7qprlg3htvsax745qe20jn7mpg3hjq8kulwwl7jh6l02ue26ejj05npn0wmjm4ssn7pejnpsd89jsq24l7f8jxgujz67vrxvuevu54v64cspl34yka"
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
> | payment_hash        | 8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda   |
> |---------------------+--------------------------------------------------------------------|
> | destination         | 03431a20a8ffdc8b09763f7d4bf370c48433c82f6ca1878ccc889f97a4af6de0bd |
> |---------------------+--------------------------------------------------------------------|
> | carrier_amount_msat | 4,000,000                                                          |
> |---------------------+--------------------------------------------------------------------|
> | expiry_secs         | 3600                                                               |
> |---------------------+--------------------------------------------------------------------|
> | asset_id            | 44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e   |
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
>   "payment_id": "8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda
>
> ```

**Run:**

```bash
$ rgbldk pay get 8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda                                                                                                                                                                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"8ce1c998161f67de15625dabd73b92b847ff7bb5b71d05d0605674ad34d1ceda","preimage":"719e231d18010b52fc8d49dae3293fba3693c7fb3d4a37ef3c18f1e9a0cd4e2e","rgb":{"asset_amount":"3","asset_id":"44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e","direction":"Outbound","is_swap":false},"secret":"237c1c84017419f2545ac185a31e1b83eb52220e21caff56cfd816834b33631e"} |
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
> | 976a26cb...7293a0db | 02883ba3...48b8c93c | 100000          | true  | true   | 44742950...1165725e | 8         | 2          |
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
> | 73091f78...23609bf9 | 03431a20...af6de0bd | 100000          | true  | true   | 44742950...1165725e | 2         | 8          |
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
>   "payment_id": "ad13277110f5e14c9676129417a30795d168a2941fa0ae9f32cc457bbb3de33c"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ad13277110f5e14c9676129417a30795d168a2941fa0ae9f32cc457bbb3de33c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ad13277110f5e14c9676129417a30795d168a2941fa0ae9f32cc457bbb3de33c
>
> ```

**Run:**

```bash
$ rgbldk pay get ad13277110f5e14c9676129417a30795d168a2941fa0ae9f32cc457bbb3de33c

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | ad13277110f5e14c9676129417a30795d168a2941fa0ae9f32cc457bbb3de33c                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"ad13277110f5e14c9676129417a30795d168a2941fa0ae9f32cc457bbb3de33c","preimage":"66915e32d3ffdc9fc314c28b82c7df2165065e937b30e658e3a9e6c2bea8e270"} |
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
>   "payment_id": "333a88f67d0f29cc3a0e07d6afcf0f88c6220139d94f294189d044487d395b58"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 333a88f67d0f29cc3a0e07d6afcf0f88c6220139d94f294189d044487d395b58 --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> 333a88f67d0f29cc3a0e07d6afcf0f88c6220139d94f294189d044487d395b58
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get 333a88f67d0f29cc3a0e07d6afcf0f88c6220139d94f294189d044487d395b58

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 333a88f67d0f29cc3a0e07d6afcf0f88c6220139d94f294189d044487d395b58                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"333a88f67d0f29cc3a0e07d6afcf0f88c6220139d94f294189d044487d395b58","preimage":"30ea5571f92841a87c5c8ae2a975b25f8ee42719e9cada38baa38b8553c3a02a"} |
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
$ rgbldk --connect http://127.0.0.1:61448 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=bb1f2e99991e156281ee9fcaaccc3d3131a136fbad0a920ff26b4bdaa17e9bd7:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:61450 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=bb1f2e99991e156281ee9fcaaccc3d3131a136fbad0a920ff26b4bdaa17e9bd7:1
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:61448 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=e71a34a34e13ace55677d1ba36dfc777
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:61448 events handled

```

**Result:**

> ```text
> Marked handled.
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:61450 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=a2f103359c5bf4987caa028878118345
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:61450 events handled

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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "1a41a0d4aad7ca3b1f49b6f715ac4851996bab0c439350811eec16948d5e93fa",
>   "71babc0e7554ad49506cc94370834f0b33dd4a770aeebedb8c834cf5788a3c65",
>   "208caf0fd646bd1497dbcb60632281895d0c4894aebb7583ad34a7ef7a73e4ac",
>   "330587b1d02ba587b7255399e6aa5dd0c3b9e017c446d1b03f080d31d930a3f1",
>   "5b9bf9ff88cbd5de66d7c2697c675fd7a6a29f215f74bd869ab9d0d4d5c3e4dc",
>   "5d95e96995bb749cd28c0624c273e8396909e6b7b69f637cfdcc980c775bbbea"
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
$ rgbldk --connect http://127.0.0.1:61450 wallet sync

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
> | BTC On-chain (total)     | 1.09977218 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) |  1.0990025 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    76,375 sats |
> +--------------------------+----------------+
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Asset                                                     | Contract                                                  | Mined | Tentative | Offchain | Total |
> +=====================================================================================================================================================================+
> | 44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |     7 |         8 |        0 |    15 |
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:61450 wallet balance

```

**Result:**

> ```text
> +--------------------------+----------------+
> | Asset                    | Balance        |
> +===========================================+
> | BTC On-chain (total)     | 1.10015844 BTC |
> |--------------------------+----------------|
> | BTC On-chain (spendable) | 1.09994824 BTC |
> |--------------------------+----------------|
> | BTC Anchor reserve       |         0 sats |
> |--------------------------+----------------|
> | BTC Lightning (total)    |    21,020 sats |
> +--------------------------+----------------+
> +------------------------------------------------------------------+-----------------------------------------------------------+-------+-----------+----------+-------+
> | RGB L1 Asset                                                     | Contract                                                  | Mined | Tentative | Offchain | Total |
> +=====================================================================================================================================================================+
> | 44742950b01d4d44249b95bbe3fc020e10446ba0eaa45464ccaca8fb1165725e | contract:RHQpULAd-TUQkm5W-74~wCDh-BEa6Dqp-FRkzKyo-_xFlcl4 |    83 |         2 |        0 |    85 |
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
>   "user_channel_id": "e15fa18516262b04f2acdaab49553253"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q7ug3jsapqc9xm22yyt9z0hvsgk0t97l6gptt9m

```

**Result:**

> ```text
> [
>   "34d4eccdd81acda587871a273d6922372c24ae667a5cec26bddaf667cbf76e97",
>   "72bde281bab6053268194168e96c88e0c0670f4aa07678d28d863ab3c58990a0",
>   "1275fa9ed7a05e109e91d26252e4d3a4b8ad6ca8fb1b242cba3c455ba29adb81",
>   "22ccb234b75d50dcd603f815a2aea4d3b724a8b6b5b6a0cfb4ff1ddbf10d71a5",
>   "4e997b30fa71da88e0b15053fd2e603a41bda1453e53b1958e83187c9edb92f9",
>   "178642e4c43b4bb7115694cbe6404efbec027fad7248159d458cb6ad805800b0"
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
