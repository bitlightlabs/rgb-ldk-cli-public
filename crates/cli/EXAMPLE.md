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
esplora=http://127.0.0.1:62444

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
>   "node_id": "037dccf24dbf902d23ca729656b0b1c2df86f7c97f032fb5e4aca03370e75ec775"
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
>   "node_id": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03"
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
> {"keystore_path":"/tmp/rgbldk-cli-example-4hk_iwdq/keystore-init-demo/keystore","mnemonic":"vacant kite nephew employ size way jacket control century click angry vapor length echo daughter side impose laundry cat reflect describe possible cereal scan","ok":true}
>
> ```

#### Keystore migrate (legacy keys_seed -> encrypted keystore)

**Run:**

```bash
$ printf '%s\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin

```

**Result:**

> ```text
> {"keystore_path":"/tmp/rgbldk-cli-example-4hk_iwdq/keystore-migrate-demo/keystore","legacy_backup_path":"/tmp/rgbldk-cli-example-4hk_iwdq/keystore-migrate-demo/keys_seed.bak.20260922-163605","ok":true}
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
> | 037b45df...4596af03 | 127.0.0.1:9736 | true      | true      |
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
>   "address": "bcrt1q6mp9np3wsmmqjk28e0ry8y4d53g8t0r0epj6m7"
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
>   "address": "bcrt1qk73muwvs5mx60t8q8psvqjg5swyc2u9evml6l9"
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
> bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1q6mp9np3wsmmqjk28e0ry8y4d53g8t0r0epj6m7 1

```

**Result:**

> ```text
> 9612e65549e02712e0b56afbe3d6141777d6f3d81ab6c0d52884adb7815454c7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qk73muwvs5mx60t8q8psvqjg5swyc2u9evml6l9 1

```

**Result:**

> ```text
> 0e9a99b7da1fc1010472fba2badcdfa4ba64f39d40ac567d78949b5326685a97
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "0b8916f0dabd208c369e88e89eed3ffffb97a9b38fa5d873b71c82fa2710b8c7",
>   "6e0dcbfa6d411a2026611ff384b43be9ed78c18b2f8faab55e1afa0dd4f42022",
>   "1e7068cf3b5f899d9afe3a65bb15a1861ac2c2c04c8863b10dd16241022adbd2",
>   "50c88835b758d9c7172d4ea2bec8aac880e8e1bad9f935be58717386b2c5702d",
>   "3556db99fbe80e40e5984b5ef263a073cfdd079b8a0a57ea205e1f9987045e45",
>   "39c2cb2fbd74c3cb9840a484903f0563b1b3db7ce46b23ec2f76c83b453c5f97"
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
> | 9612e65549e02712e0b56afbe3d6141777d6f3d81ab6c0d52884adb7815454c7:1 |  100,000,000 | Confirmed |    102 | Available    | -        |            - |
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
>   "user_channel_id": "9543cad7e29977fb8bf71b13d3dd7754"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "305aa1f76c5cb8c35be11f42eefaf03b965e20a5904cba6be581ab76ff873781",
>   "2ad9221ef70697c3e7950eda9e36f4a460f7468b79f06bfd28a03ff88aad4340",
>   "65832af96b8ab669307cdca17acd7f473c29ca851e52c89f4c3751133d6b28f0",
>   "555cd6593fed5252e77d1ae6f65d481c84b0326d2c64647eab25a1e85d765d98",
>   "3788767c00c01a26ad9f3043d0960f369bde0a8985da51dae06c3914253f1489",
>   "28db071c239fd88800bbf5f35ac334b45860bd49f873efc5df2c5fb987ffe806"
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
> | 9543cad7...d3dd7754 | 037dccf2...e75ec775 | 118747255865344 | 1099614322695 | 1099520868357 | 120,000         | true  | true   |
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
> | b1fc9928...47c14291 | 037b45df...4596af03 | 118747255865344 | 1099520868357 | 1099614322695 | 120,000         | true  | true   |
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
> | 037dccf24dbf902d23ca729656b0b1c2df86f7c97f032fb5e4aca03370e75ec775 |
> |--------------------------------------------------------------------|
> | 037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03 |
> +--------------------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk graph node <node_id_b>

```

**Result:**

> ```text
> Node 037b45df...4596af03.
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
> Channel 118747255865344 connects 037b45df...4596af03 and 037dccf2...e75ec775.
> Capacity: unknown.
>
> 037b45df...4596af03 -> 037dccf2...e75ec775: no gossip update available.
>
> 037dccf2...e75ec775 -> 037b45df...4596af03: Enabled.
> +-------------+------------------+
> | Field       | Value            |
> +================================+
> | Last update | 1790094978       |
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
>   "address": "bcrt1qa9pms55t44gutjrjyrp2w9zqwv0g7c0atngtk0"
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
> | b1fc9928...47c14291 | 037b45df...4596af03 | 118747255865344 | 1099520868357 | 1099614322695 | 120,000         | true  | true   |
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
> | 9543cad7...d3dd7754 | 037dccf2...e75ec775 | 118747255865344 | 1099614322695 | 1099520868357 | 120,000         | true  | true   |
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
> | -               | 9a73e319...5377fbef | 037dccf2...e75ec775 | broadcasting | unknown | -            | 87,654     |
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
> | -               | 9a73e319...5377fbef | 037b45df...4596af03 | broadcasting | unknown | -            | 30,000     |
> +-----------------+---------------------+---------------------+--------------+---------+--------------+------------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "199583a581454919fd86942199abe85e599d037162c556262fcc3d07a3f55bb9",
>   "46420bf30f1b6132b62d762801792532297cf77324f9b21b8721c633e127cff3",
>   "2487f6b89b437c9b36bf068c9562f1c535e7f64c593290f4964f51a1922a93d8",
>   "124ce628618b722a2a68b65b4e2130473dbd7545684e6e4290abb12c6a84f1b1",
>   "0ca35048a1b33f56db59024d0dd46fd41ad351d4002bde644f666bb76af22fe4",
>   "4572be3b3e0416a640b6640c40df998abc1f1c262b2f2e2bd9fbe7dd9f2c9792"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "43fcae8a541d29338aedc262d7da0a20487c527be38f6d6eceba0972f082065c"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "0ba9d808351ec0d74dd928abca4dcda82cc767442078e875084996721c0b13bd"
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

node-a runs the LSPS1 (bLIP-51) channel-selling service (started with `--lsps1-service-config`). node-b, the client, configures node-a as its LSP, places a BTC channel order, pays the order on-chain to the deposit address, and node-a opens the ordered channel back to node-b. The operator side (`pricing`, `options`, `orders`) and the client side (`lsp`, `info`, `order`) are both shown. Amounts are decimal-string sats; an order is complete once its `channel` field is populated (and the operator ledger shows `order_state: completed`).


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
>     "max_fulfill_retries": 3,
>     "auto_close_expired_channels": true,
>     "channel_expiry_grace_blocks": 144,
>     "late_deposit_refund_window_secs": "2592000"
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
> {"supported_options":{"min_required_channel_confirmations":0,"min_funding_confirms_within_blocks":1,"supports_zero_channel_reserve":false,"max_channel_expiry_blocks":52560,"min_initial_client_balance_sat":"0","max_initial_client_balance_sat":"0","min_initial_lsp_balance_sat":"10000","max_initial_lsp_balance_sat":"100000000","min_channel_balance_sat":"10000","max_channel_balance_sat":"100000000"},"service":{"require_token":null,"bolt11_invoice_expiry_secs":3600,"min_onchain_payment_confirmations":1,"max_fulfill_retries":3,"auto_close_expired_channels":true,"channel_expiry_grace_blocks":144,"late_deposit_refund_window_secs":"2592000"}}
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

### Client: configure LSP, inspect the offering, place an order (node-b)

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
$ rgbldk lsps1 lsp set --pubkey <node_id_a> --address <node_a_p2p>

```

**Result:**

> ```text
> LSP set to 037dccf24dbf902d23ca729656b0b1c2df86f7c97f032fb5e4aca03370e75ec775 @ 127.0.0.1:9735
>
> ```

**Run:**

```bash
$ rgbldk lsps1 lsp get

```

**Result:**

> ```text
> {"pubkey":"037dccf24dbf902d23ca729656b0b1c2df86f7c97f032fb5e4aca03370e75ec775","address":"127.0.0.1:9735","token":null}
>
> ```

**Run:**

```bash
$ rgbldk lsps1 info

```

**Result:**

> ```text
> {"supported_options":{"min_required_channel_confirmations":0,"min_funding_confirms_within_blocks":1,"supports_zero_channel_reserve":false,"max_channel_expiry_blocks":52560,"min_initial_client_balance_sat":"0","max_initial_client_balance_sat":"0","min_initial_lsp_balance_sat":"10000","max_initial_lsp_balance_sat":"100000000","min_channel_balance_sat":"10000","max_channel_balance_sat":"100000000"},"pricing":{"btc_capacity_ppm_per_year":10000,"onchain_cost_sat":"10000","min_fee_sat":"1000"},"rgb":{"rgb_assets":[]}}
>
> ```

The order is paid on-chain: the client sends the order total to the deposit address the LSP returns (`payment.onchain.address`), with no pre-existing channel required. The LSP's watcher opens the ordered channel once the deposit confirms.


**Run:**

```bash
$ rgbldk --color never --output json --pretty lsps1 order create --lsp-balance-sat 100000 --channel-expiry-blocks 4380

```

**Result:**

> ```text
> {
>   "order_id": "c6733cae-a628-4fa9-83a8-65129e28346e",
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
>       "expires_at_unix_secs": "1790098582",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "invoice": "lnbcrt100840n1p4t9t5xdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2pp5w0th8w0erdahfqyeayjy2v0v2kcsg2tls0cdgljh9lnukg2htzyqsp5zczg4v49tm67vzx7xh9s65qrnrpt5dy4jdzjhakjlyaccp9juxvq9qyysgqcqzp2xqrrssdayv4806w8xfe4sq8hnkplay4qz0l0hecvuc0hvcjhq6m9luptpr9n7cyw7yn3k3q5ue60q9qw47n9t5vpcacyzz4lra08qfe3hjk0gp5wrvg6"
>     },
>     "onchain": {
>       "state": "expect_payment",
>       "expires_at_unix_secs": "1790098582",
>       "fee_total_sat": "10084",
>       "order_total_sat": "10084",
>       "address": "bcrt1qs7jupkhhqt66cerqk5zkfwdfs7x9d284h50rka",
>       "min_onchain_payment_confirmations": 1,
>       "refund_onchain_address": "bcrt1qw0ny42r556l6qpfd5e7ht00pz25ghe9p27gr6k"
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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qs7jupkhhqt66cerqk5zkfwdfs7x9d284h50rka 0.00010084

```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Run:**

```bash
$ rgbldk lsps1 order get <order_id>

```

**Result:**

> ```text
> order c6733cae-a628-4fa9-83a8-65129e28346e — channel opened
> {"order_id":"c6733cae-a628-4fa9-83a8-65129e28346e","order":{"lsp_balance_sat":"100000","client_balance_sat":"0","required_channel_confirmations":0,"funding_confirms_within_blocks":1,"channel_expiry_blocks":4380,"announce_channel":false},"payment":{"bolt11":{"state":"expect_payment","expires_at_unix_secs":"1790098582","fee_total_sat":"10084","order_total_sat":"10084","invoice":"lnbcrt100840n1p4t9t5xdqlf3f4q5e3yp3ksctwdejkcgr0wfjx2usnp4qd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2pp5w0th8w0erdahfqyeayjy2v0v2kcsg2tls0cdgljh9lnukg2htzyqsp5zczg4v49tm67vzx7xh9s65qrnrpt5dy4jdzjhakjlyaccp9juxvq9qyysgqcqzp2xqrrssdayv4806w8xfe4sq8hnkplay4qz0l0hecvuc0hvcjhq6m9luptpr9n7cyw7yn3k3q5ue60q9qw47n9t5vpcacyzz4lra08qfe3hjk0gp5wrvg6"},"onchain":{"state":"paid","expires_at_unix_secs":"1790098582","fee_total_sat":"10084","order_total_sat":"10084","address":"bcrt1qs7jupkhhqt66cerqk5zkfwdfs7x9d284h50rka","min_onchain_payment_confirmations":1,"refund_onchain_address":"bcrt1qw0ny42r556l6qpfd5e7ht00pz25ghe9p27gr6k"}},"channel":{"funded_at_unix_secs":"1790094992","funding_outpoint":"ff4c2cb5b268f3453111cfdb6a07b46edaad2afce8cd8afa00f2fcfb81912a0a:1","expires_at_unix_secs":"1792722992"},"rgb":null}
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
> | 6ad3a764...58c70ab9 | 037dccf2...e75ec775 | -    | 1099594989569 | 1099658428423 | 100,000         | true  | true   |
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
> | c6733cae...9e28346e | 037b45df...4596af03 | completed | paid    | onchain  |
> +---------------------+---------------------+-----------+---------+----------+
>
> ```

**Run:**

```bash
$ rgbldk lsps1 orders get <order_id>

```

**Result:**

> ```text
> {"order_id":"c6733cae-a628-4fa9-83a8-65129e28346e","counterparty_node_id":"037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03","order_state":"completed","payment_state":"paid","paid_via":"onchain","lsp_balance_sat":"100000","client_balance_sat":"0","channel_expiry_blocks":4380,"announce_channel":false,"fee_total_sat":"10084","order_total_sat":"10084","onchain_address":"bcrt1qs7jupkhhqt66cerqk5zkfwdfs7x9d284h50rka","onchain_paid_sat":"10084","refund_onchain_address":"bcrt1qw0ny42r556l6qpfd5e7ht00pz25ghe9p27gr6k","refund_txid":null,"fulfill_retry_count":0,"created_at_unix_secs":"1790094982","payment_expires_at_unix_secs":"1790098582","funding_outpoint":"ff4c2cb5b268f3453111cfdb6a07b46edaad2afce8cd8afa00f2fcfb81912a0a:1","funded_at_height":136,"channel_closed_at_unix_secs":null,"rgb":null}
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
$ rgbldk lsps1 rgb-order --asset-id <contract_id> --lsp-asset-balance 100 --lsp-balance-sat 100000 --channel-expiry-blocks 4380

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
>   "address": "bcrt1q7j7n9yvz8dyuxj2mn068qv4ad9gs6j77tjf36g"
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
> rgb(wpkh([8cd4b038/86h/827167h/0h]tpubDCddGYUWSv9Lnne6VaosDwDbphbPkHKjzB2qpdqJLYBSbfZWCuTiv4FVKZuHNDs1zZFoSHx2KiGathdwsLaFDRnor4nJZ95sZgf15YTkxgS/<0;1>/*),seals(),noise(e9f71854b7f68ba8abdce1ba536cc58f20bc880c24885a122dfeb951a4adf54f))
>
> +-------------+------------------+-----------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Fingerprint | Derivation Path  | Xpub                                                                                                            | Descriptor                                                                                                                                                      |
> +====================================================================================================================================================================================================================================================================================================================+
> | 8cd4b038    | m/86'/827167'/0' | tpubDCddGYUWSv9Lnne6VaosDwDbphbPkHKjzB2qpdqJLYBSbfZWCuTiv4FVKZuHNDs1zZFoSHx2KiGathdwsLaFDRnor4nJZ95sZgf15YTkxgS | wpkh([8cd4b038/86'/827167'/0']tpubDCddGYUWSv9Lnne6VaosDwDbphbPkHKjzB2qpdqJLYBSbfZWCuTiv4FVKZuHNDs1zZFoSHx2KiGathdwsLaFDRnor4nJZ95sZgf15YTkxgS/<0;1>/*)#qf3hr57h |
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
> | Public key      | 035d224d11fc74d211303f57e82724342457e9d4c6ab1866e1c3fa2eae647b8bd7 |
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
> 3045022100e2ce3a8c95c23ba8c460bd61a62a7433c18086b890c19cb311c92a67fdb06d3102206c8072cae3f1708c5fb775e514e7cd04c2420bc2e6f950cf5d4385f4d6b635ec
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
>   "address": "bcrt1q2fdvpn3d0nfves889u8ezz7wpr6vqx2pejwrsp"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_1> 0.1

```

**Result:**

> ```text
> a298a20f16b6fd84f78f4475eb2a38d44f4f8844b1a8546ca40d630d12813759
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_2> 0.0011

```

**Result:**

> ```text
> aa36d5f59b236d9bb2c1d33716443e97030612178a9e426f8b8a3d536afdf744
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "2210401be30a8f0b0324237bc2b1bbe654b34a7798ffd052611f32af553a352c"
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
> | a298a20f16b6fd84f78f4475eb2a38d44f4f8844b1a8546ca40d630d12813759:0 |   10,000,000 | Confirmed |    137 | Available    | -           | FeeSupport, BlindingTarget |
> |--------------------------------------------------------------------+--------------+-----------+--------+--------------+-------------+----------------------------|
> | aa36d5f59b236d9bb2c1d33716443e97030612178a9e426f8b8a3d536afdf744:0 |      110,000 | Confirmed |    137 | Available    | -           | FeeSupport, BlindingTarget |
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
> | a298a20f16b6fd84f78f4475eb2a38d44f4f8844b1a8546ca40d630d12813759:0 |   10,000,000 |    137 | Available |        |
> |--------------------------------------------------------------------+--------------+--------+-----------+--------|
> | aa36d5f59b236d9bb2c1d33716443e97030612178a9e426f8b8a3d536afdf744:0 |      110,000 |    137 | Available |        |
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
>   "reservation_id": "manual-reservation-1790095000-nBAsDpJ5ZTDfizUy9Ng9idlIJOGQgEyY",
>   "outpoint": "a298a20f16b6fd84f78f4475eb2a38d44f4f8844b1a8546ca40d630d12813759:0",
>   "reserved_until_unix_secs": "1790095060"
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
>   "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
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
>       "detail": "a298a20f16b6fd84f78f4475eb2a38d44f4f8844b1a8546ca40d630d12813759:0"
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
> | DemoAsset | DEMO   |         0 |    100 | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
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
$ rgbldk rgb contracts export --contract-id contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI --out /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.raw --format raw --direct

```

**Result:**

> ```text
> Exported contract contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.
> Consignment key: contract_export_4fcd87c5b174ca56f8b6bb26d41ce1fd1a5f337ef1d443a1054040fb249ebe3d
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.raw.
>
> ```

**Run:**

```bash
$ rgbldk debug consignment <contract.raw> --format raw

```

**Result:**

> ```text
> ok=true
> contract_id=contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI
> seals_total=1
> extern_outpoints=1
> wout_outpoints=0
> fallback_outpoints=0
> noises=1
> outpoint=a298a20f16b6fd84f78f4475eb2a38d44f4f8844b1a8546ca40d630d12813759:0
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip

```

**Result:**

> ```text
> {
>   "bytes": 2992,
>   "export": {
>     "checks": [
>       {
>         "name": "rgb_enabled",
>         "ok": true
>       },
>       {
>         "detail": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>         "name": "contract_id_valid",
>         "ok": true
>       }
>     ],
>     "consignment_key": "contract_export_c477ee9e2faf6aea943c8ea1b02bd8a6d05e187c1c2759d800c55e3687b3fd21",
>     "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>     "ok": true
>   },
>   "out": "/Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.zip"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment contract_export_c477ee9e2faf6aea943c8ea1b02bd8a6d05e187c1c2759d800c55e3687b3fd21.
> Saved to /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.download.zip.
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
>   "address": "bcrt1qrvqenwph0mscfjne8qwp2khm4ydknrcsxuylrh"
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
> 5677fd985848044cb58fd8363ef45b0c5cbe4376b42f7756dc43ae3d06b66358
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "0c7639e5995e3f0257d25145acd2f15d43fdd1a2add5a14bceb66e9eedfb2ba9"
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
$ rgbldk rgb contracts import --contract-id contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI --file /Users/bincheng_paopao/project/repo/rust/myself/well/rgb-ldk-cli/target/rgbldk-example/contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.raw

```

**Result:**

> ```text
> [OK] RGB contract import
>   [OK] RGB Enabled
>   [OK] Contract Id Valid: contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI
>   [OK] Upload Size Ok: 6744 bytes
>   [OK] Archive Decoded: 6744 bytes
>   [OK] Consignment Stored: contract_import_be132340d3cd8f060d8d2b002db00333c3890a8190ebc9fe2c21d61c4a32e4a7
> Imported contract contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.
> Consignment key: contract_import_be132340d3cd8f060d8d2b002db00333c3890a8190ebc9fe2c21d61c4a32e4a7
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
> | DemoAsset | DEMO   |         0 |    100 | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
> +-----------+--------+-----------+--------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk rgb contracts balance contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
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
> The node knows contract contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.
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
>   "invoice": "contract:tb@oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI/90@at:oHDcZcxc-LE~ZFnK3-eK_shibr-15FrP~6O-HEJBySHt-q6OjoA/?expiry=2026-09-22T17:36:46.535+00:00",
>   "blinding_utxo_used": "5677fd985848044cb58fd8363ef45b0c5cbe4376b42f7756dc43ae3d06b66358:0"
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
> | Contract         | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
> |------------------+-----------------------------------------------------------|
> | Amount           | 90                                                        |
> |------------------+-----------------------------------------------------------|
> | Beneficiary      | at:oHDcZcxc-LE~ZFnK3-eK_shibr-15FrP~6O-HEJBySHt-q6OjoA    |
> |------------------+-----------------------------------------------------------|
> | Beneficiary type | Blinded                                                   |
> |------------------+-----------------------------------------------------------|
> | Expiry           | 1790098606                                                |
> +------------------+-----------------------------------------------------------+
>
> ```

**Run:**

```bash
$ rgbldk debug invoice '<invoice>'

```

**Result:**

> ```text
> invoice=contract:tb@oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI/90@at:oHDcZcxc-LE~ZFnK3-eK_shibr-15FrP~6O-HEJBySHt-q6OjoA/?expiry=2026-09-22T17:36:46.535+00:00
> scope=contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI
> layer1=bitcoin testnet=true
> beneficiary_type=token
> beneficiary_value=at:oHDcZcxc-LE~ZFnK3-eK_shibr-15FrP~6O-HEJBySHt-q6OjoA
> data=90
> expiry=2026-09-22T17:36:46.535+00:00
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
>   "txid": "123509669e4f961a93e202a8fa80572370096217ff4e9d40dcc0e0de0896c634",
>   "consignment_key": "rgb_consignment_contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI_123509669e4f961a93e202a8fa80572370096217ff4e9d40dcc0e0de0896c634"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI_123509669e4f961a93e202a8fa80572370096217ff4e9d40dcc0e0de0896c634.
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
> Accepted contract contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.
> Amount: 90
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "1e71614b1a1ea3717c81e3fa832561e656e7304d149d48d66361fa7dbff9fbcd",
>   "6758bc73fd8a329e00fefb905956d3efa7b307072ac22953e320995ebfa6fb50",
>   "4b704ac15523af206cc32bd5007b766be6472a15da6c28180cb9dc4538e9a47c",
>   "3bb4eb80e32a214b21486fb77035832bc1d563cd9afe5c973292d17184c46ec1",
>   "2b4cc1be38a403a91c5c360c910177087ed6caf19bcbfe90e5897f06b7ab7338",
>   "00c646b71635c49a3b4214c60cafbfd8af94dc9462b133133ee2333ec775de2f"
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
$ rgbldk rgb contracts balance contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
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
> | f4fff911bfcbef4a24d648237a1ce1f4485353c3621c1be54470a9e59f2dfd97 | succeeded | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |     90 | 123509669e4f961a93e202a8fa80572370096217ff4e9d40dcc0e0de0896c634 | transfer_import_44408f566fb94da6233461d504c0f8b5efb7f7688bd558b57b9ca1d5ee50201a |
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
$ rgbldk rgb contracts balance contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
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
>   "user_channel_id": "92cc2fb8029e7b4dbeb66cd6499d37d5"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "0e80d192d99d1eb8ee665f36d9ba8d81f2c36856bda4f87acb087e62b49e25fb",
>   "6412eb8833495a92f59b9d49e4c5720e67ec228bdea199f38e575289bc59fed3",
>   "547b40db1cadfb1310a1c071aef7cf8c1de202bf5c9d3c15ab2f2dac17b27efa",
>   "5a3da2b0b8fdf28aa0c6e47d163e13727031bbd379ebc13fe5e39555c815e63a",
>   "00505ff57f0bdf82dc0a3bd97a8c4eba29fd772271e21f8a6d4fd5ed0f7d1813",
>   "08bfd33c2ca5ae2d2784ccbb993ec682263183905f159ebc5674cedb77c99950"
> ]
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
> | c6733cae...9e28346e | 037b45df...4596af03 | 150633093070849 | 1099658428423 | 1099594989569 | 100,000         | true  | true   | -                   | -         | -          |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------|
> | 92cc2fb8...499d37d5 | 037b45df...4596af03 | -               | 1099543085060 | 1099597283330 | 100,000         | true  | true   | contract...-MUPxuNI | 10        | 0          |
> +---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------+
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
>   "invoice": "contract:tb@oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI/7@wout:AQAAAAAA-AAAQZsVl-EPSMaRgK-R5_5hUVe-MxFSdyDF-LS20/?expiry=2026-09-22T17:37:12.289+00:00",
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
>   "txid": "975a9c4b3e6f1cbf9119228572b3a3cc397e748122174aa632db433811641063",
>   "consignment_key": "rgb_consignment_contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI_975a9c4b3e6f1cbf9119228572b3a3cc397e748122174aa632db433811641063"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip

```

**Result:**

> ```text
> Downloaded consignment rgb_consignment_contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI_975a9c4b3e6f1cbf9119228572b3a3cc397e748122174aa632db433811641063.
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
> Accepted contract contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI.
> Amount: 7
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "025b494ca15ce0fe98af4ea17badf200a1970807a1f0882820674931936a8efe",
>   "60f5bc89a7a6e11c7c0956a70fb1d0225749e5efb08a3547f2b79f6ea30476ea",
>   "74d964c28282b42962962da4ebea725560236f413948fc3bc4f4c568c31b2efa",
>   "5c855b01c5b62b75fd51055452bf40bdfabdfbdadb0f8ba8235202e8c941da5d",
>   "0fbe2865335fc78f56a1935e4efa692bc53ce195b4fe8df6663b0778b868facb",
>   "68021ed94496f8f57da58f8b56bc80ecd99137a79a89cf3ee0fb18a36e21d2f3"
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
$ rgbldk rgb contracts balance contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI

```

**Result:**

> ```text
> +-----------+-----------------------------------------------------------+
> | Field     | Value                                                     |
> +=======================================================================+
> | Contract  | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |
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
>   "address": "bcrt1qlraqfejnwm5rea7fa7qzsv3c7lhzrxsessr8j2"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.01

```

**Result:**

> ```text
> 261c19d1ac9c06e481599597b3af6b8a27a11668c9d6e02f31718ddb142bf44c
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "069c6f37432135eb45066799fe1a8fe86bd26cc7fc1397938e611ea157aae538"
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
>   "address": "bcrt1q3pml5ydp5yy8w36fln7srfywd9gxshdlg2j6qa"
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
>   "address": "bcrt1q3fnzfrcnvt6xfqsmqnd20xgk3vnlltutnal70p"
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
>   "address": "bcrt1qpdn46ar2v95dt3ktyf8tkwy5tuew4tzq9vqrjv"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos fund --input <wallet_outpoint> --output <rgb_address_1>:30000 --output <rgb_address_2>:28000 --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> {"txid":"32d48ba2da6f378cbd2188b6f02f71b52b8a98027b88a6249ffd6b21e8ee6697","status":"broadcast","outputs":[{"address":"bcrt1q3pml5ydp5yy8w36fln7srfywd9gxshdlg2j6qa","value_sats":"30000","vout":2},{"address":"bcrt1q3fnzfrcnvt6xfqsmqnd20xgk3vnlltutnal70p","value_sats":"28000","vout":1}],"change":{"address":"bcrt1qpdn46ar2v95dt3ktyf8tkwy5tuew4tzq9vqrjv","value_sats":"941828","vout":0},"fee_sats":"172"}
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "6e25e217af1130bf8e8abdc49a9de11f56bb48ce28f4e7445a6d0a03b308131c"
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
>   "address": "bcrt1q2u2z6t20vsu9hslv2vyc3v9pwmdp3m444a2pmh"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <wallet_address> 0.005

```

**Result:**

> ```text
> a39d4733668eeaa5e237545aaff5a3275973b05ced8126313e4cc503fe42752e
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "46b0f542084b938186bb52881148eeb77fffb1aa9d812c9274412a1a84035e56"
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
>   "address": "bcrt1qcyrlavcrsrxvypvm5kye30adz3h5jrqjnu8uq4"
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
>   "address": "bcrt1qtlm52ge29fu9d4rzyedcxpy62plq6cpyujrjyv"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos top-up --rgb-outpoint <allocated_rgb_outpoint> --l1-input <wallet_outpoint> --rgb-address <rgb_address> --target-value-sats <larger_value_sats> --change-address <wallet_change_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Topped up RGB UTXO 975a9c4b3e6f1cbf9119228572b3a3cc397e748122174aa632db433811641063:2.
> Broadcast transaction: 0acdc1d8873263aa9282da0c782ac558a3f7172d32fad5909feeaad34914c5f7
> Replacement output: bcrt1qcyrlavcrsrxvypvm5kye30adz3h5jrqjnu8uq4 with 10,029,800 sats.
> Network fee: 254 sats.
> Consignment key: rgb_consignment_contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI_0acdc1d8873263aa9282da0c782ac558a3f7172d32fad5909feeaad34914c5f7
> Change output: bcrt1qtlm52ge29fu9d4rzyedcxpy62plq6cpyujrjyv with 459,746 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "4648be1ccf76bbe0aa32dc6678af738a6405a962e08d275f226dd59ecd3a6a06",
>   "38e14b6bb5dd53040339f60a6e72b0210592130b279843107e9da65251fa108e",
>   "222e814154d09ef6458867ab61d58b3c91c78e020d77277d3b8db9a28dc98d0c",
>   "52d33b0adff1e63a1b6760301bf2245fdcf8dee3cf50b019990bece4f310568a",
>   "6f4217b0d6b406e9f47654b56464e2a1914505decf41d61cf3cb488e713230c5",
>   "6efd338ac234f33aabeeadcfc32d51e242889d6d4a37feaa8f8fa3471757f0eb"
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
>   "address": "bcrt1qfprcjs7khqz99jnaydspjfwlrks3qq93k032tx"
> }
>
> ```

**Run:**

```bash
$ rgbldk rgb utxos sweep --outpoint <rgb_outpoint> --destination-address <wallet_address> --fee-rate-sats-per-vb 1.0

```

**Result:**

> ```text
> Swept RGB UTXO 32d48ba2da6f378cbd2188b6f02f71b52b8a98027b88a6249ffd6b21e8ee6697:1.
> Broadcast transaction: 97e27a592892a5ac1b06703c194b9940c952753aed9bb302ae6af489d13a24b0
> Sent 27,889 sats to bcrt1qfprcjs7khqz99jnaydspjfwlrks3qq93k032tx.
> Network fee: 111 sats.
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "40526b1059d67bf5619a945c7cc101bb5e6cf275496eace851514a0844d59ab0",
>   "4d79ecab110ffebc6a545cd2161eb4d2bc6d86df2ccc0a8e451e4cf32b1dce79",
>   "475e86e54a6b3f564c5f651b48cba0a14c60d22943f1c3010f95ab60c393818b",
>   "7dbc2b08ddaaa67c30c3469b08ac012dacdb103cf88f7209000bd4b566c3922d",
>   "3044f7a575f37eba7464f6b4d159714cc2a9019ffb8762c4bbc82ef6d34a9b71",
>   "4c933a9794d559a3f9bfa169fac8641c56587ef1041a0353a5d330540f524d89"
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
>   "swap_string": "rgb-swap:v1:eyJwYXltZW50X2hhc2hfaGV4IjoiNzM3M2Y3ZmU2OTllNzY5ZDYwNDA3MWUxOGE0N2U1NTNiMDc2YzYwMmQ5OTRlMmM1ZDkwZGI0YjE3OWMyMzhmOSIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAzN2I0NWRmNjg3OTZlNWVlMmFlOWJkYWY5MmQ1ZjUyNDE1NWEzM2RjNjMzNTE5NjA4NWQxODY5Mjk0NTk2YWYwMyIsImNoYW5uZWxfc2NpZCI6MTY2MDI2MjU1ODU5NzEzLCJjb250cmFjdF9pZCI6ImNvbnRyYWN0Om9TaGtETn5-LUt0SjVaTGwtdGV0UWdGVC1rRmo2cE5tLUdpbHRLM0wtTVVQeHVOSSIsImFzc2V0X2Ftb3VudCI6MiwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwMCwiYnRjX2NhcnJpZXJfYW1vdW50X21zYXQiOjUwMDAwMDAsIm1ha2VyX2dpdmVzX3JnYiI6dHJ1ZSwiZXhwaXJ5X3NlY3MiOjM2MDAsImNyZWF0ZWRfYXRfdW5peF9zZWNzIjoxNzkwMDk1MDQzfQ",
>   "payment_hash": "7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9",
>   "info": {
>     "payment_hash": "7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03",
>     "channel_scid": "166026255859713",
>     "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>     "asset_amount": "2",
>     "btc_amount_msat": "10000000",
>     "btc_carrier_amount_msat": "5000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1790095043",
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
>   "payment_hash": "7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9",
>   "role": "Taker",
>   "status": "Offered",
>   "counterparty_node_id": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03",
>   "channel_scid": "166026255859713",
>   "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1790095043",
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
>   "payment_hash": "7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9",
>   "role": "Taker",
>   "status": "Accepted",
>   "counterparty_node_id": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03",
>   "channel_scid": "166026255859713",
>   "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>   "asset_amount": "2",
>   "btc_amount_msat": "10000000",
>   "btc_carrier_amount_msat": "5000000",
>   "maker_gives_rgb": true,
>   "expiry_secs": 3600,
>   "created_at_unix_secs": "1790095043",
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
$ rgbldk swap get 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Accepted                                                         |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 037b45df...4596af03                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 166026255859713                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-MUPxuNI                                              |
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
> | Created (unix secs) | 1790095043                                                       |
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
>   "payment_hash": "7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9",
>   "status": "InFlight"
> }
>
> ```

**Run:**

```bash
$ rgbldk swap get 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Maker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 037b45df...4596af03                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 166026255859713                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-MUPxuNI                                              |
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
> | Created (unix secs) | 1790095043                                                       |
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
$ rgbldk swap get 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9

```

**Result:**

> ```text
> +---------------------+------------------------------------------------------------------+
> | Field               | Value                                                            |
> +========================================================================================+
> | Payment hash        | 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9 |
> |---------------------+------------------------------------------------------------------|
> | Role                | Taker                                                            |
> |---------------------+------------------------------------------------------------------|
> | Status              | Settled                                                          |
> |---------------------+------------------------------------------------------------------|
> | Counterparty        | 037b45df...4596af03                                              |
> |---------------------+------------------------------------------------------------------|
> | Channel SCID        | 166026255859713                                                  |
> |---------------------+------------------------------------------------------------------|
> | Contract            | contract...-MUPxuNI                                              |
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
> | Created (unix secs) | 1790095043                                                       |
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
> | 7373f7fe699e769d604071e18a47e553b076c602d994e2c5d90db4b179c238f9 | Taker | Settled | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI | 2     | RGB         |
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
>   "swap_string": "rgb-swap:v1:eyJwYXltZW50X2hhc2hfaGV4IjoiNDhhMmI3OWRhZmE4YzEyNTg4YWJjZTkxNmJmZmU4ODcyNmQ5ZjZiODQzZThlOWViMjM4NDRkMGJkODNjOTM4YiIsImNvdW50ZXJwYXJ0eV9ub2RlX2lkX2hleCI6IjAzN2I0NWRmNjg3OTZlNWVlMmFlOWJkYWY5MmQ1ZjUyNDE1NWEzM2RjNjMzNTE5NjA4NWQxODY5Mjk0NTk2YWYwMyIsImNoYW5uZWxfc2NpZCI6MTY2MDI2MjU1ODU5NzEzLCJjb250cmFjdF9pZCI6ImNvbnRyYWN0Om9TaGtETn5-LUt0SjVaTGwtdGV0UWdGVC1rRmo2cE5tLUdpbHRLM0wtTVVQeHVOSSIsImFzc2V0X2Ftb3VudCI6MSwiYnRjX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJidGNfY2Fycmllcl9hbW91bnRfbXNhdCI6MTAwMDAwMCwibWFrZXJfZ2l2ZXNfcmdiIjp0cnVlLCJleHBpcnlfc2VjcyI6MzYwMCwiY3JlYXRlZF9hdF91bml4X3NlY3MiOjE3OTAwOTUwNDV9",
>   "payment_hash": "48a2b79dafa8c12588abce916bffe88726d9f6b843e8e9eb23844d0bd83c938b",
>   "info": {
>     "payment_hash": "48a2b79dafa8c12588abce916bffe88726d9f6b843e8e9eb23844d0bd83c938b",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03",
>     "channel_scid": "166026255859713",
>     "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1790095045",
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
>   "swap_string": "rgb-swap:v2:eyJwYXltZW50X2hhc2hfaGV4IjoiM2FlYjUxMjQ1ZWY1YmMxZTZkYmVlMDE0MjE2ZWE1MWI1M2Q3OGUzOTVjODI3NjNlOWI5MGI0Mzg2YjAyNWFiNiIsInJnYl9wYXRoIjpbeyJub2RlX2lkX2hleCI6IjAzN2I0NWRmNjg3OTZlNWVlMmFlOWJkYWY5MmQ1ZjUyNDE1NWEzM2RjNjMzNTE5NjA4NWQxODY5Mjk0NTk2YWYwMyIsImNoYW5uZWxfc2NpZCI6MTY2MDI2MjU1ODU5NzEzfV0sImJ0Y19wYXRoIjpbeyJub2RlX2lkX2hleCI6IjAzN2RjY2YyNGRiZjkwMmQyM2NhNzI5NjU2YjBiMWMyZGY4NmY3Yzk3ZjAzMmZiNWU0YWNhMDMzNzBlNzVlYzc3NSIsImNoYW5uZWxfc2NpZCI6MTY2MDI2MjU1ODU5NzEzfV0sImNvbnRyYWN0X2lkIjoiY29udHJhY3Q6b1Noa0ROfn4tS3RKNVpMbC10ZXRRZ0ZULWtGajZwTm0tR2lsdEszTC1NVVB4dU5JIiwiYXNzZXRfYW1vdW50IjoxLCJidGNfYW1vdW50X21zYXQiOjEwMDAwMDAsImJ0Y19jYXJyaWVyX2Ftb3VudF9tc2F0IjoxMDAwMDAwLCJtYWtlcl9naXZlc19yZ2IiOnRydWUsImV4cGlyeV9zZWNzIjozNjAwLCJjcmVhdGVkX2F0X3VuaXhfc2VjcyI6MTc5MDA5NTA0Nn0",
>   "payment_hash": "3aeb51245ef5bc1e6dbee014216ea51b53d78e395c82763e9b90b4386b025ab6",
>   "info": {
>     "payment_hash": "3aeb51245ef5bc1e6dbee014216ea51b53d78e395c82763e9b90b4386b025ab6",
>     "role": "Maker",
>     "status": "Offered",
>     "counterparty_node_id": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03",
>     "channel_scid": "166026255859713",
>     "contract_id": "contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI",
>     "asset_amount": "1",
>     "btc_amount_msat": "1000000",
>     "btc_carrier_amount_msat": "1000000",
>     "maker_gives_rgb": true,
>     "expiry_secs": 3600,
>     "created_at_unix_secs": "1790095046",
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
>   "invoice": "lnbcrt100n1p4t9tkxdq8v3jk6mcnp4qda5thmg09h9ac4wn0d0jt2l2fq4tgeacce4r9sgt5vxj229j6hsxpp54y9ht4w2htg7pgr6rlpmc6kus7uf9m0yq4lr39mpj46d02erjpnqsp57lh8347yxadmytsyanp27zxmrudepl5rq0gq7kxzvdsxznpkstes9qyysgqcqzp2xqrrssrzjqd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2qqqqyqqrcqqqsqqqqlgqqqqqqqqfqymqjakutfnag6j5wd32vcl09jmngd797zy45qyrpzss9lckx3eyxj7h6vjcf58xc6mgnt3075yzpc4e3gyqymv63vpnyadahg9tey2qq47ulzx"
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
> | payment_hash | a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066   |
> |--------------+--------------------------------------------------------------------|
> | destination  | 037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03 |
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
>   "payment_id": "a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066",
>   "preimage": "e7125f2879cb0804188882892d8b205969360d419855a364bf6af6b9afa14211",
>   "amount_sats": "10",
>   "destination": "037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066
>
> ```

**Run:**

```bash
$ rgbldk pay get a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"a90b75d5cabad1e0a07a1fc3bc6adc87b892ede4057e3897619574d7ab239066","preimage":"e7125f2879cb0804188882892d8b205969360d419855a364bf6af6b9afa14211","secret":"f7ee78d7c4375bb22e04ecc2af08db1f1b90fe8303d00f58c26360614c3682f3"} |
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
>   "invoice": "lnbcrt130n1p4t9tk8dq0v3jk6medwdjkueqnp4qda5thmg09h9ac4wn0d0jt2l2fq4tgeacce4r9sgt5vxj229j6hsxpp57v8x3xm5ffvadr2vccvcssv8ky4s9rkk73f549n30jdx27lq6g7qsp54ecxk7fz0d9mn5xl2dqmkat9t8jzkcz50h3nlhlake0xp5cqdeqs9qyysgqcqzp2xqrrssrzjqd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2qqqqyqqrcqqqsqqqqlgqqqqqqqqfqaz94pckx7cufhyrhr97e7tnvjwwdzly390ehwdyhd53kk4p2gwnx2rfqn5kqjwk3tx28ndt5a5rkjgsq7sxalgw9l8j24zszpu53jlqqr0qlyc"
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
>   "payment_id": "f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c
>
> ```

**Run:**

```bash
$ rgbldk pay get f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"f30e689b744a59d68d4cc619884187b12b028ed6f4534a96717c9a657be0d23c","preimage":"8b7419e08cf8950e90c80552229d74266487e448cdb0f62545df72e8347d63a2","secret":"ae706b79227b4bb9d0df5341bb756559e42b60547de33fdffdb65e60d3006e41"} |
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
>   "invoice": "lnbcrt1p4t9tkgdqdv3jk6medweshynp4qda5thmg09h9ac4wn0d0jt2l2fq4tgeacce4r9sgt5vxj229j6hsxpp5fu3l4a079pa5xqwg76dx4q9zfzu4wzufpu885g5w5rdzpna5r8cssp5mmwp926nn8qach2jl0z4gavtxdnwuh75vcpvjxrshvnl48r43csq9qyysgqcqzp2xqrrssrzjqd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2qqqqyqq3sqqquqqqqlgqqqqqqqqfqwgnn49vqx9vhsmrum6ra24vcgk25hnel2hss3zlhxs3j5ajehuwyhc5nmspez9jta9733q60v8cgmkt37plernsjtzyyc63s9xkqddqpam3cdl"
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
>   "payment_id": "4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1
>
> ```

**Run:**

```bash
$ rgbldk pay get 4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4f23faf5fe287b4301c8f69a6a80a248b9570b890f0e7a228ea0da20cfb419f1","preimage":"193133b7dcc23d13e5b84a5eec0be34b6adaeb10089c607bba6f7a50d4b43d18","secret":"dedc12ab5399c1dc5d52fbc554758b3366ee5fd46602c91870bb27fa9c758e20"} |
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
>   "invoice": "lnbcrt140n1p4t9tk2dqcv3jk6meddphkcepdvdkxz6tdnp4qda5thmg09h9ac4wn0d0jt2l2fq4tgeacce4r9sgt5vxj229j6hsxpp5gf0dfe9rdvcw5gdepcsuwykxf85zznpfkl40dqyf6ypecmj48pxqsp5u0r92qce44eum345ftj5p2vshq4d8yertk5zchlzzmv4du8x2p9s9qyysgqcqzp2xqrrssrzjqd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2qqqqyqqrcqqqsqqqqlgqqqqqqqqfq5mj07reeheelyrh9s7zqqavp4n089kmxuqy4zep4plhl5mkhgwn402mtr006c74dfcvp2sn53qlxqtx0wupwtv7c0ysm0mcr7aglpscqrdzcp3"
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
> | destination  | 037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03 |
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
> | kind_details    | {"payment_hash":"425ed4e4a36b30ea21b90e21c712c649e8214c29b7eaf68089d1039c6e55384c","preimage":"4242424242424242424242424242424242424242424242424242424242424242","secret":"e3c6550319ad73cdc6b44ae540a990b82ad393235da82c5fe216d956f0e6504b"} |
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
>   "invoice": "lnbcrt150n1p4t9tkvdqhv3jk6meddphkcepdveskjmqnp4qda5thmg09h9ac4wn0d0jt2l2fq4tgeacce4r9sgt5vxj229j6hsxpp5gyfa2jctvyfffdl4jkmfrjwm2s0uplr3npydd3wrg53w4nqt8gjqsp5uap7frasmn2m35cmg6yl7u83q4h37trn4v36p23uw8vttckn8g6s9qyysgqcqzp2xqrrssrzjqd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2qqqqyqqrcqqqsqqqqlgqqqqqqqqfqv6ht9zchtvh7sd7hzq3g5ndlu4y8dj9jlnntk8v36yy0hd2xccpzru6cd0ru5264dcztxgdll4pl2ng5uwzy0rdqsgucapydd47nttqqf0qefx"
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
> | kind_details    | {"payment_hash":"4113d54b0b611294b7f595b691c9db541fc0fc719848d6c5c34522eacc0b3a24","preimage":null,"secret":"e743e48fb0dcd5b8d31b4689ff70f1056f1f2c73ab23a0aa3c71d8b5e2d33a35"} |
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
>   "invoice": "lnbcrt120n1p4t9tk0dq0v3jk6medvfskx6cnp4qd7ueujdh7gz6g72w2t9dv93ct0cda7f0upjld0y4jsrxu88tmrh2pp5v8s09jd3ksvlg35pq7yekvqrv6xk9au7kwrquwxrhq2df7qsd2nssp5sv3whrs93chctzp3ygeujenmsex02xa4zwtyyyqw5439khwmwvfq9qyysgqcqzp2xqrrssrzjqda5thmg09h9ac4wn0d0jt2l2fq4tgeacce4r9sgt5vxj229j6hsxqqqqyqq2xcqqgqqqqlgqqqqqqqqfqpq6tylmtxhuw5egn2y3tutv7e9dhk9hsk9msytze60dye730p7myn25erdu2jxgcnssxge8njasukwapukazgy5kcxk2jaqmr8532qsq5gj0j6"
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
>   "payment_id": "61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7",
>   "preimage": "7f9c25d5abe5f4e1102209d00adad408407c101fc8b1e240f2b74a3b047b1741",
>   "amount_sats": "12",
>   "destination": "037dccf24dbf902d23ca729656b0b1c2df86f7c97f032fb5e4aca03370e75ec775",
>   "fee_paid_msat": "0"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7
>
> ```

**Run:**

```bash
$ rgbldk pay get 61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                         |
> +=================================================================================================================================================================================================================================================================+
> | id              | 61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7                                                                                                                                                                              |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                      |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                   |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                        |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"61e0f2c9b1b419f4468107899b3003668d62f79eb3860e38c3b814d4f8106aa7","preimage":"7f9c25d5abe5f4e1102209d00adad408407c101fc8b1e240f2b74a3b047b1741","secret":"8322eb8e058e2f8588312233c9667b864cf51bb5139642100ea5625b5ddb7312"} |
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
>   "offer": "lno1qgsqvgnwgcg35z6ee2h3yczraddm72xrfua9uve2rlrm9deu7xyfzrcgqg2mxzs2danxvetj94jx2mt0pczx4v4uuygvyqqqqpkqqqqpqqqq930ghrh9nav55x3g7d22a2gx8xxy3plc4tljaa0zyvk0ma9c4wc0qypdevxfr7upjusjent33m27a455tp3ev20vyvwv3t8dqgt58eknlfcqwsrx4nyytd07j50aaynxs7qmadfz7exsscv4zk69qklwwmxv6v6drsqermwhvqjvfrgs7m9cx2hnvrm6yxnjlvdzfg975h8hzlw39az569sa4sj0wglv6veerjefvsthwyw5quvajyjmetd74584qqxs63sr92x0fuqa6e3g2lk0s3rc8vfywc65xpcpvggzdy839yzgf5tdg3nzyeuxspfaucjj2s6ad3d35dgwcghed2sstcvs"
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
> | offer_id                  | f9e8f459be9244650d2bdb76adba0d03b103a28cbd3cad52b953fcf6deb5b9a4   |
> |---------------------------+--------------------------------------------------------------------|
> | signing_pubkey            | 02690f1290484d16d44662267868053de62525435d6c5b1a350ec22f96aa105e19 |
> |---------------------------+--------------------------------------------------------------------|
> | description               | offer-demo                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | issuer                    | -                                                                  |
> |---------------------------+--------------------------------------------------------------------|
> | amount_msat               | 5,555                                                              |
> |---------------------------+--------------------------------------------------------------------|
> | absolute_expiry_unix_secs | 1790098657                                                         |
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
>   "payment_id": "ff000f6311fddb4bee6ffa297cb668b1a9360b8e3d7a4f37c9c958f0b7273321"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait ff000f6311fddb4bee6ffa297cb668b1a9360b8e3d7a4f37c9c958f0b7273321 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> ff000f6311fddb4bee6ffa297cb668b1a9360b8e3d7a4f37c9c958f0b7273321
>
> ```

**Run:**

```bash
$ rgbldk pay get ff000f6311fddb4bee6ffa297cb668b1a9360b8e3d7a4f37c9c958f0b7273321

```

**Result:**

> ```text
> +-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                               |
> +=======================================================================================================================================================================================================================+
> | id              | ff000f6311fddb4bee6ffa297cb668b1a9360b8e3d7a4f37c9c958f0b7273321                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 800e8fdf5f992459ed5a1cde8f09c8de6a57ee3173af5141325f4694bebfbcce                                                                                                                                    |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                            |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Offer                                                                                                                                                                                         |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                               |
> |-----------------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"offer_id":"f9e8f459be9244650d2bdb76adba0d03b103a28cbd3cad52b953fcf6deb5b9a4","payer_note":null,"payment_hash":"800e8fdf5f992459ed5a1cde8f09c8de6a57ee3173af5141325f4694bebfbcce","quantity":null} |
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
>   "refund": "lnr1qqsrpa0h8jwm6jj5fsjvm53s7vz45f3qa8zakffyzzm3fy39yxenlxg2qq8qg64jhn39qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxqad3f567fezrhad37qs6f2qzxzgf9q9nqqwq6p47w9ys59sccnrty9hyetxw4hxgttyv4kk7kkzqyqqqmqqqqqsqqqrnh7hcxupnmmszzklnhhpep8rlwml5srh7g0vtklw5ezxtmq0al0szqnt0ydw74sh028yynr0upkjex972thmpvtn235896rwkcearda3w5q8gdn94c5kpm8wghld58qjujrkndene8jsfx4wyx0xrk07vayea64x4z3ed9m0msxftyqs7t5rvrplmnrxea2vwv3pt2fvuuq2etwmqhgnmn7qrn2v8j0s63luaj5geyhguz4kdj95q5xhtzrw7hemp5tr5t9jq9ghgu7k77rgmlq33y4lgznd9e84krc",
>   "payment_id": "edbbfdd7d416b8d38c1e3060f434ede46a19d7a6cef83eac12542fb7af81bcf6"
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
> | absolute_expiry_unix_secs | 1790098658                                                         |
> |---------------------------+--------------------------------------------------------------------|
> | chain_hash                | 06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f   |
> |---------------------------+--------------------------------------------------------------------|
> | payer_signing_pubkey      | 0303ad8a69af27221dfad8f810d254011848494059800e06835f38a4850b0c6263 |
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
> lni1qqsrpa0h8jwm6jj5fsjvm53s7vz45f3qa8zakffyzzm3fy39yxenlxg2qq8qg64jhn39qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqyy8ptqssxqad3f567fezrhad37qs6f2qzxzgf9q9nqqwq6p47w9ys59sccnrty9hyetxw4hxgttyv4kk7kkzqyqqqmqqqqqsqqqrnh7hcxupnmmszzklnhhpep8rlwml5srh7g0vtklw5ezxtmq0al0szqnt0ydw74sh028yynr0upkjex972thmpvtn235896rwkcearda3w5q8gdn94c5kpm8wghld58qjujrkndene8jsfx4wyx0xrk07vayea64x4z3ed9m0msxftyqs7t5rvrplmnrxea2vwv3pt2fvuuq2etwmqhgnmn7qrn2v8j0s63luaj5geyhguz4kdj95q5xhtzrw7hemp5tr5t9jq9ghgu7k77rgmlq33y4lgznd9e84kraqacphk3wldpukuhhz46da47fdtafyz4dr8hrrx5vkppw3s6ffgkt27qcrw8k3knu87wp5fnycv92em3j0jjnhnmxn8qsrdd8g7hxqj02jpqgqzqjgsfwj9sk8jy025ezd3w9y5503mufm6xywrptqehffey944efryyqg32v9l4txqnkresk89jzy8tg5jahg3ag2u2g8ak4rknctryhckecu3l7nmcw8282nlwym20f0ssaqenxp2dl6au3r3r60px0s2k6avny3wear5d2nlku0kz5prnyq0te8f2tv443l30uxxl92pp3aarmtcadn4a4qgw6qkr7azuaxre2zv6x3ryqnaknr6djrupeu3rutv3w885anwdqr2zu5axdzrsqqqqqqqqqqqqqq9gqqqqqqqqqqqqgayjedltzjqqqqqq9yq34t9tkj4qsdkxqvt6auanvre99ptr5aexdpwckj2e5l2jrffdss8ctcxue5r6a2qggwrtsrqgqqpvppq2h7e7yexw98crq352c0j52cnglmy2pdvma4yafk2ak8w0jmk6c93uzqnk7y3uvm5kqlr26qzl9x4y4psyzcdhz7m64dnz4qcl9njj8033yhmqnt9qp86hyg50xduexpndzhw5lc8m9dsyjtarhn6xaw8ughjvc
> payment_id: db180c5ebbcecd83c94a158e9dc99a1762d25669f548694b6103e178373341eb
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
$ rgbldk pay wait edbbfdd7d416b8d38c1e3060f434ede46a19d7a6cef83eac12542fb7af81bcf6 --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> edbbfdd7d416b8d38c1e3060f434ede46a19d7a6cef83eac12542fb7af81bcf6
>
> ```

**Run:**

```bash
$ rgbldk pay get edbbfdd7d416b8d38c1e3060f434ede46a19d7a6cef83eac12542fb7af81bcf6

```

**Result:**

> ```text
> +-----------------+--------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                          |
> +==================================================================================================================================================+
> | id              | edbbfdd7d416b8d38c1e3060f434ede46a19d7a6cef83eac12542fb7af81bcf6                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | db180c5ebbcecd83c94a158e9dc99a1762d25669f548694b6103e178373341eb                                                               |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                       |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                    |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt12Refund                                                                                                                   |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                          |
> |-----------------+--------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payer_note":"refund-demo","payment_hash":"db180c5ebbcecd83c94a158e9dc99a1762d25669f548694b6103e178373341eb","quantity":null} |
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
>   "refund": "lnr1qqs09d787zzstmm087k52axf9z0srpd9jpd0tghnf7defxqyw7ghnpq2qq8qg64jhnj9qgqxyfhyvyg6pdvu4tcjvpp7kkal9rp57wj7xv4pl3ajku70rzy3pafqypzhtqss997gjen5scpjyz6kfmrnskwh2xe6x45swgc5p2g7s4gjjh0s943rtyfhyetxw4hxgttpvfskuer0dckkgetddadvyqgqqpkqqqqpqqqqynup754latek5wf0833aujg4jw0rccrmuc0j2jl4de6nctflv7umqypl6puymns398n822pte0gz5vaddjh3k5nrqwuz3c4cfagpfdtggscqwsllhhsekqm3fnq5ngr5zuywtvj4kxm6rmatzt768qj59dt68kls5n8fyk3mluy67zaassz0zhtse088jj2ewta2flwjqu4lxphq4ptgv0zl37pyk7zxzpt277a0w9ut7n4rmnlmsllwqxcaajs3fdke0qz9tch4lu044zwlghakfyweynacgmaun3hq",
>   "payment_id": "5e192e568c91a0b74232b64f6f29e7987c0f1e2cfac6b8e1a95f30f0122aa044"
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
> | f30e689b...7be0d23c | f30e689b...7be0d23c | Succeeded | Bolt11       | Outbound | 13,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | ff000f63...b7273321 | 800e8fdf...bebfbcce | Succeeded | Bolt12Offer  | Outbound | 5,555           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 0a2a9181...b52c4cff | -                   | Succeeded | Onchain      | Outbound | 100,000,000     | 9,232,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | c7545481...55e61296 | -                   | Succeeded | Onchain      | Inbound  | 100,000,000,000 | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4f24c1a2...32bc3380 | -                   | Succeeded | Onchain      | Inbound  | 30,000,000      | 2,011,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | a90b75d5...ab239066 | a90b75d5...ab239066 | Succeeded | Bolt11       | Outbound | 10,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 425ed4e4...6e55384c | 425ed4e4...6e55384c | Succeeded | Bolt11       | Outbound | 14,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 7dca1c13...e5df6e03 | -                   | Succeeded | Onchain      | Inbound  | 10,084,000      | 2,820,000  | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | edbbfdd7...af81bcf6 | db180c5e...373341eb | Succeeded | Bolt12Refund | Outbound | 4,321           | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 5e192e56...122aa044 | -                   | Failed    | Bolt12Refund | Outbound | 1,111           | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4f23faf5...cfb419f1 | 4f23faf5...cfb419f1 | Succeeded | Bolt11       | Outbound | 11,000          | 0          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 73d773b9...21575888 | 73d773b9...21575888 | Pending   | Bolt11       | Inbound  | 10,084,000      | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 4113d54b...cc0b3a24 | 4113d54b...cc0b3a24 | Failed    | Bolt11       | Outbound | 15,000          | -          | false       |
> |---------------------+---------------------+-----------+--------------+----------+-----------------+------------+-------------|
> | 61e0f2c9...f8106aa7 | 61e0f2c9...f8106aa7 | Succeeded | Bolt11       | Inbound  | 12,000          | -          | false       |
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
> | 0a2a9181fbfcf200fa8acde8fc2aadda6eb4076adbcf113145f368b2b52c4cfe | 6ad3a764784e13fbf8c6962558c70ab9 | yes    | 98,340,000   | 0          | no      | yes     | yes        | 354,000       | 98,340,000     | -               | -                | 354,000  | holder_reserve | 354,000      | holder_reserve  |
> |------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+-----------------+------------------+----------+----------------+--------------+-----------------|
> | 23102798d56925c79afed533d5a6f156ce1b9850f5fd04fed37b4f119346dfca | 3adc3722d289ed937f8bb5c2a25f0590 | yes    | 88,294,124   | 10,045     | yes     | yes     | yes        | 330,000       | 88,294,124     | -               | -                | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt30u1p4t9tk4dq6wfnkygrvdcsxsmmvvssxgetddupp5hvu3g9wqtcuawl9pwwqa803l05xdtefn9ed90yc34k4q4f3pqm5ssp5ts7tjjqsuygxzwkp2596q8nken76pmxssyr0snul9h9mml0ughrs9qrsgqxqrrsscqpjlp55y5xgrxllu4dy7tyh9kh44pqz5ustra2fkvx3fd54h9nzsl3hrfq7qpzqjluq7vjzfkwzcxpgklp4urhnekt4ggazsm47nra34d4azkvf9ljscd8vls6039x59ryn8yhrlkgz5kkpycqh8dxu3cf02yye4kfz9qpz4pqdn",
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
> | Destination           | 037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 3,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI          |
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
>   "invoice": "lnbcrt50u1p4t9tk4dqjwfnkygrvdcsxgetddupp5j07u2j4hqcmf6yz5q82s39e7ccf07k76vuhnu43v7fk6z8vhr99ssp5jf6t82mxpdqatggzl2j8cx38cs4559s7yc9uv8czzrsj4z5m060s9qrsgqxqrrsscqpjlp55y5xgrxllu4dy7tyh9kh44pqz5ustra2fkvx3fd54h9nzsl3hrfq7qp9vvllykh8sf46ln8n6z7dvf8tzduafyf3rgta7gtcx4cjqj7msckrkwppaj2kntvf0yt5qu53j3x6xlfk6xke5dj8na7qywnluh24legqgwakre",
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
> | Payment hash          | 93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 037b45df68796e5ee2ae9bdaf92d5f524155a33dc6335196085d1869294596af03 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 5,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI          |
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
>   "payment_id": "93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b
>
> ```

**Run:**

```bash
$ rgbldk pay get 93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | 93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"93fdc54ab706369d105401d508973ec612ff5bda672f3e562cf26da11d97194b","preimage":"4d5c52253256ca7349e271612fb93628109f34e7d8ff7b08e63e81c1b8e42b2f","rgb":{"asset_amount":"5","contract_id":"contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI","direction":"Outbound","is_swap":false},"secret":"9274b3ab660b41d5a102faa47c1a27c42b4a161e260bc61f0210e12a8a9b7e9f"} |
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
> +------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------+
> | Channel                                                          | User Channel                     | Usable | Inbound msat | Local sats | Reserve | Receive | Can create | Required msat | Available msat | Blocking reason                  | Suggested action              | Min msat | Min reason     | Default msat | Default reason  |
> +============================================================================================================================================================================================================================================================================================================================================+
> | 0a2a9181fbfcf200fa8acde8fc2aadda6eb4076adbcf113145f368b2b52c4cfe | c6733caea6284fa983a865129e28346e | yes    | 0            | 100,000    | yes     | no      | no         | 330,000       | 0              | carrier_inbound_capacity_too_low | add_inbound_carrier_liquidity | -        | -              | -            | -               |
> |------------------------------------------------------------------+----------------------------------+--------+--------------+------------+---------+---------+------------+---------------+----------------+----------------------------------+-------------------------------+----------+----------------+--------------+-----------------|
> | 23102798d56925c79afed533d5a6f156ce1b9850f5fd04fed37b4f119346dfca | 92cc2fb8029e7b4dbeb66cd6499d37d5 | yes    | 14,045,876   | 84,954     | yes     | yes     | yes        | 330,000       | 14,045,876     | -                                | -                             | 1,000    | minimum_viable | 330,000      | admission_floor |
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
>   "invoice": "lnbcrt40u1p4t9tkcdq6wfnkygrvdcsxgetddusxyctrdvpp57d05qmmgp6nskwjwxgsasrlgr5n8kml5am5fwc2t44cytjtxsuwssp5c727dhz93arwjulrjcnr0guzpklnzkudfdfmlcdvhsv8vce5l5js9qrsgqxqrrsscqpjlp55y5xgrxllu4dy7tyh9kh44pqz5ustra2fkvx3fd54h9nzsl3hrfq7qpr6ujsw6h4znpygm7c4cut0szyyyke36qkwlwa06hq3vzmszwzx4qy0txkxmcd694jamr5kw29fnsuyftsr66qvk9lxk85xwv0plqsfjcplejx2n",
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
> | Payment hash          | f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d   |
> |-----------------------+--------------------------------------------------------------------|
> | Destination           | 037dccf24dbf902d23ca729656b0b1c2df86f7c97f032fb5e4aca03370e75ec775 |
> |-----------------------+--------------------------------------------------------------------|
> | Carrier amount (msat) | 4,000,000                                                          |
> |-----------------------+--------------------------------------------------------------------|
> | Expiry (secs)         | 3600                                                               |
> |-----------------------+--------------------------------------------------------------------|
> | Contract              | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI          |
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
>   "payment_id": "f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d
>
> ```

**Run:**

```bash
$ rgbldk pay get f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                                                                                                                                                                                                                                                     |
> +=============================================================================================================================================================================================================================================================================================================================================================================================================+
> | id              | f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d                                                                                                                                                                                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                                                                                                                                                                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                                                                                                                                                                                                                                               |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Bolt11                                                                                                                                                                                                                                                                                                                                                                                    |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                                                                                                                                                                                                                                                     |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"f35f406f680ea70b3a4e3221d80fe81d267b6ff4eee897614bad7045c966871d","preimage":"cc606b9f2045d0a5bf0ba2efd3e19c6cc0169e24a9f2899e390f8ce84733c319","rgb":{"asset_amount":"3","contract_id":"contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI","direction":"Outbound","is_swap":false},"secret":"c795e6dc458f46e973e3962637a3820dbf315b8d4b53bfe1acbc18766334fd25"} |
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
> | c6733cae...9e28346e | 037b45df...4596af03 | 150633093070849 | 1099658428423 | 1099594989569 | 100,000         | true  | true   | -                   | -         | -          |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------|
> | 92cc2fb8...499d37d5 | 037b45df...4596af03 | 166026255859713 | 1099543085060 | 1099597283330 | 100,000         | true  | true   | contract...-MUPxuNI | 6         | 4          |
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
> | 6ad3a764...58c70ab9 | 037dccf2...e75ec775 | 150633093070849 | 1099594989569 | 1099658428423 | 100,000         | true  | true   | -                   | -         | -          |
> |---------------------+---------------------+-----------------+---------------+---------------+-----------------+-------+--------+---------------------+-----------+------------|
> | 3adc3722...a25f0590 | 037dccf2...e75ec775 | 166026255859713 | 1099597283330 | 1099543085060 | 100,000         | true  | true   | contract...-MUPxuNI | 4         | 6          |
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
>   "payment_id": "4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait 4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c --timeout-secs 60

```

**Result:**

> ```text
> [OK] Payment wait
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Succeeded
> 4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c
>
> ```

**Run:**

```bash
$ rgbldk pay get 4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | 4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | 4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"4da9c3556e12e2d00d7d3f27513aef9f1ca1f2db4021a430a094a37d59dbce8c","preimage":"8b45b78cf9f4871135f7a5bc56f9dec40e65ff374dda2a7d0a418f569906a9e1"} |
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
>   "payment_id": "f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1"
> }
>
> ```

**Run:**

```bash
$ rgbldk pay wait f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1 --timeout-secs 60

```

**Result:**

> ```text
> [X] Details
>   [OK] Payment Id Valid
>   [OK] Payment Terminal: Failed
> f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1
> payment failed
>
> ```

**Run:**

```bash
$ rgbldk pay get f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1

```

**Result:**

> ```text
> +-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------+
> | Field           | Value                                                                                                                                                             |
> +=====================================================================================================================================================================================+
> | id              | f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | payment_hash    | f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1                                                                                                  |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | direction       | Outbound                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | status          | ✘ Failed                                                                                                                                                          |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind            | Spontaneous                                                                                                                                                       |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | htlc_locked     | false                                                                                                                                                             |
> |-----------------+-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
> | kind_details    | {"payment_hash":"f9f6878a8062b8af713e8947a18d39e03a8b9d1e9f4a0f42f7f253e5295e87e1","preimage":"123e68611c6423f744361ba0809580a0bc630ca675a1688b0e3e0986677b6b1d"} |
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
> ChannelPending funding_txo=effb77536cfb9132fa8339d2a99341acc570e1953dde5ae1ff45c3b719e3739a:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8502 events watch --count 1

```

**Result:**

> ```text
> ChannelPending funding_txo=effb77536cfb9132fa8339d2a99341acc570e1953dde5ae1ff45c3b719e3739a:0
>
> ```

**Run:**

```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```

**Result:**

> ```text
> ChannelReady user_channel_id=b1fc99281dd79df997ce90e047c14291
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
> ChannelReady user_channel_id=9543cad7e29977fb8bf71b13d3dd7754
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
> | 92cc2fb8...499d37d5 | 23102798...9346dfca | 037b45df...4596af03 | negotiating | coop   | -            | 86,348     | contract...-MUPxuNI | 6         | -         |
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
> | 3adc3722...a25f0590 | 23102798...9346dfca | 037dccf2...e75ec775 | negotiating | coop   | -            | 11,047     | contract...-MUPxuNI | 4         | -         |
> +---------------------+---------------------+---------------------+-------------+--------+--------------+------------+---------------------+-----------+-----------+
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "3eb2b7f2aea57e7b78555d7e52d5c27f7adac17a5c559e9d3b90b3d8c1cfc2a4",
>   "014e93d5d8bfdf02cba4193e336b4e43f7de75f4867cd02d6afa401487b7aa43",
>   "2c9a0d78c7a6369c887cb2ebdd5d08255b3b3aa49e54bb9ce5c5fa4b84b8b577",
>   "5d78c17eb94eb63ef208b53dbe6e8fa84c1d06641f10513d41a64fa8c23d0726",
>   "49b7b6eee28d5f8b36ee473d3338f13d8177fe1ece466f22279c097bdd345ff8",
>   "705dfd65e62691503e6607886d18c410f1b3564ea639a646a284c776fb79ba83"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "63561e5e137e7edea0843cdf5c6ae030c8e9ca850d191a793e0209439574fb6d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "48a286886acd1a0f520c86731d17bcbd7e0e0511739e7a46e94560ae4bd86544"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "6c85d9ece100fa909927617329cd7e4162dfd2f2d22f9091b8f4eb1d30b50518"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "7d2d0317d399e2ade230991e84ae915fc8813de57dadac141ec69fe49a10dcb0"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "01e3986e138ed15a658204ee88ac91e1fb6bff0c96fb92fd85d8ec0c5b432d14"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "377f608d5bc7b837c4bb58bbba5af4716ce30f44595591dfab76644f06264b0d"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "51178fec0d233a95521916fbfcaede6da7ecfb9f64dc55f9ff44060cc0854899"
> ]
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "44600ba0d36fe390872d0c9759b46048d72fd2f057b01f55b791529b213abe14"
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
> | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |    13 |         0 |        0 |    13 |
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
> | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI | contract:oShkDN~~-KtJ5ZLl-tetQgFT-kFj6pNm-GiltK3L-MUPxuNI |    87 |         0 |        0 |    87 |
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
>   "user_channel_id": "3bd4c0593e0c379b0978b5d4692867b2"
> }
>
> ```

**Run:**

```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1quz38mcsknpz5jwpdshh0y0wyae7340mqk2htgr

```

**Result:**

> ```text
> [
>   "00827bc978920f6ed531e5c9fbde8aef522f6acab4467a65690da4afbc69df8f",
>   "3a4117019d2876139492e294203fcbde700961f059efb2837ce533c34cd86cbb",
>   "2704a4651a16015351ae623f56226a98ef50e00094ee43d228679825ad8f8ab6",
>   "0050cd9d923b494ee88db34fedfc1a9c7dc3c5a492d91cc4be69ca20c25c35bf",
>   "1a138a20a0fbbf420d81746308fb0368d254c2f2c442a97dde606d191ad0c869",
>   "4abddc1010f4569c10c9067313edb9541ee53bfc6abe188af5f8f9c9f6aba1fc"
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
> | -               | d95b195c...f0d40203 | 037b45df...4596af03 | broadcasting | unknown | -            | 46,654     |
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
> | -               | d95b195c...f0d40203 | 037dccf2...e75ec775 | broadcasting | unknown | -            | 1,000      |
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
