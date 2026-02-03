# `rgbldk` CLI examples (regtest, docker-compose)


This file is generated from a real local test run on regtest using `crates/cli/docker-compose.yaml`.

Notes:
- Outputs contain IDs (txids, invoices, payment IDs, node IDs, channel IDs) that differ per run.
- `rgbldk` is a client CLI talking to `rgbldkd` over HTTP.



## 0) Start environment

This section starts a self-contained regtest environment (bitcoind + esplora + two `rgbldkd` nodes) via docker-compose, and shows the endpoints you will use in the rest of the examples.

Commands:


```bash
$ DOCKER_BUILDKIT=1 docker compose -f crates/cli/docker-compose.yaml up -d

```


```bash
$ cargo build -p rgbldk-cli --bin rgbldk

```


```bash
$ export PATH="$PWD/target/debug:$PATH"

```


Endpoints:


```text
node_a=http://127.0.0.1:8501
node_b=http://127.0.0.1:8502
esplora=http://127.0.0.1:3003

```


## 1) Contexts (ctx)

Contexts let you name daemon endpoints (e.g. `node-a`, `node-b`) and switch the default target without repeatedly passing `--connect`.

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
$ rgbldk ctx ls

```


**Result:**

> ```text
> +---------+--------+-----------------------+
> | Current | Name   | URL                   |
> +==========================================+
> | *       | node-a | http://127.0.0.1:8501 |
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


## 2) Node basics (version/health/ready/status)

Use these commands to verify the CLI can reach the daemon, and to understand *why* a node is (not) ready via sub-checks.

**Run:**


```bash
$ rgbldk node version

```


**Result:**

> ```text
> +--------------------+-----------+
> | Field              | Value     |
> +================================+
> | api_crate_version  | 0.1.0     |
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
>   [OK] Best Block Height: Height: 671
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
> | best_block_height | 671   |
> +-------------------+-------+
>
> ```


**Run:**


```bash
$ rgbldk wallet balance

```


**Result:**

> ```text
> +----------------------+----------------+
> | Asset                | Balance        |
> +=======================================+
> | On-chain (total)     | 4.99691066 BTC |
> |----------------------+----------------|
> | On-chain (spendable) | 4.99641066 BTC |
> |----------------------+----------------|
> | Anchor reserve       |    50,000 sats |
> |----------------------+----------------|
> | Lightning (total)    |   296,006 sats |
> +----------------------+----------------+
>
> ```


## 3) Peers (ls)

List currently known peers. This is usually empty before you open channels or connect peers explicitly.

**Run:**


```bash
$ rgbldk peer ls

```


**Result:**

> ```text
> +---------------------+-----------------+-----------+-----------+
> | Node ID             | Address         | Connected | Persisted |
> +===============================================================+
> | 03539473...72edf3cc | 172.31.0.2:9735 | true      | true      |
> +---------------------+-----------------+-----------+-----------+
>
> ```


Tip: tables truncate long IDs by default to keep the output readable on an 80-column terminal. Use `--no-truncate` when you need to copy the full value.

**Show full IDs (no truncation):**


```bash
$ rgbldk --no-truncate peer ls

```


**Result:**

> ```text
> +--------------------------------------------------------------------+-----------------+-----------+-----------+
> | Node ID                                                            | Address         | Connected | Persisted |
> +==============================================================================================================+
> | 03539473578248d1f18b26633512e054d603298d14391276aee924df8d72edf3cc | 172.31.0.2:9735 | true      | true      |
> +--------------------------------------------------------------------+-----------------+-----------+-----------+
>
> ```


## 4) Wallet (address/sync) + fund on regtest

Generate on-chain addresses for both nodes, fund them from the regtest miner wallet, and then `wallet sync` to make balances visible to the node wallet.

**Run:**


```bash
$ rgbldk wallet address

```


**Result:**

> ```text
> bcrt1qpfxf753ggymess89qu4p4uvaajxql20gdwfykp
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
$ rgbldk ctx use node-b

```


**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```


**Run:**


```bash
$ rgbldk wallet address

```


**Result:**

> ```text
> bcrt1qenx3zgccatpcgtn6k7e4dlu538x04jw8nq64p4
>
> ```


**Run:**


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
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin getnewaddress

```


**Result:**

> ```text
> bcrt1q795lhhsdudcevs5myww54y55sk664c4pscnwuq
>
> ```


**Run:**


```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qpfxf753ggymess89qu4p4uvaajxql20gdwfykp 1

```


**Result:**

> ```text
> 32d005d8cc227a66ab90eae08acdf1eb6138e263581b379d10ac1e6e6907c37d
>
> ```


**Run:**


```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress bcrt1qenx3zgccatpcgtn6k7e4dlu538x04jw8nq64p4 1

```


**Result:**

> ```text
> 7fab8344c9875e02208c6f7a71a871e0e610cd6de5e800fbbdd3d5b908643a5a
>
> ```


**Run:**


```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 bcrt1q795lhhsdudcevs5myww54y55sk664c4pscnwuq

```


**Result:**

> ```text
> [
>   "227727c983b48646474d70a6ffaa19900cfabb37f3d89cb5f29b52a88c6d6540"
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
> Balance change: on-chain total +1.0009872 BTC, spendable +1.0009872 BTC, lightning -99,013 sats.
>
> ```


**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet sync

```


**Result:**

> ```text
> Wallet synced.
> Balance change: on-chain total +1.0 BTC, spendable +1.0 BTC, lightning 0 sats.
>
> ```


**Run:**


```bash
$ rgbldk wallet balance

```


**Result:**

> ```text
> +----------------------+----------------+
> | Asset                | Balance        |
> +=======================================+
> | On-chain (total)     | 5.99789786 BTC |
> |----------------------+----------------|
> | On-chain (spendable) | 5.99739786 BTC |
> |----------------------+----------------|
> | Anchor reserve       |    50,000 sats |
> |----------------------+----------------|
> | Lightning (total)    |   196,993 sats |
> +----------------------+----------------+
>
> ```


**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8502 wallet balance

```


**Result:**

> ```text
> +----------------------+-------------+
> | Asset                | Balance     |
> +====================================+
> | On-chain (total)     |     6.0 BTC |
> |----------------------+-------------|
> | On-chain (spendable) |  5.9995 BTC |
> |----------------------+-------------|
> | Anchor reserve       | 50,000 sats |
> |----------------------+-------------|
> | Lightning (total)    |      0 sats |
> +----------------------+-------------+
>
> ```


## 5) Channel (ls/open)

Open a private Lightning channel from node-a to node-b and wait for confirmations so the channel becomes usable.

**Run:**


```bash
$ rgbldk node id

```


**Result:**

> ```text
> 03005dbb3b12f5f76c459c71675c0aa3d8fe8e5fe430c88db8d54dd870e7eae3d7
>
> ```


**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8502 node id

```


**Result:**

> ```text
> 03539473578248d1f18b26633512e054d603298d14391276aee924df8d72edf3cc
>
> ```


**Run:**


```bash
$ rgbldk channel open --node-id 03539473578248d1f18b26633512e054d603298d14391276aee924df8d72edf3cc --addr node-b:9735 --amount-sats 100000 --private

```


**Result:**

> ```text
> 983ccfc04689ea3a181dce394262b0bf
>
> ```


**Run:**


```bash
$ docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 bcrt1q795lhhsdudcevs5myww54y55sk664c4pscnwuq

```


**Result:**

> ```text
> [
>   "314488faafb87e0db2b02d3e951e78a2247ef63b9fc2108031e9f95cba127acb",
>   "623e3a993e2127b31e6ed91a666a41f688413d0a889cb95338a400e2a17a29ad",
>   "78f853366b75c3d7206d3f00d44480fa352ef6c21a8fe5378e2ddd468afbb068",
>   "58fae5066fec2aa2f57ec50af41fc7883b339e08b33ac7424d2989b947921ad6",
>   "1370f0e2496e0615e0667b758ddf29c0a03ef9b6486d67c4116116dd6bb2143e",
>   "752a8021529544db7b1a1b090b84ddb34f9b198a3ae73542ad3b21b4ce0604c9"
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
> | 647fc7b9...c16bdf4a | 03539473...72edf3cc | 100000          | true  | true   |
> |---------------------+---------------------+-----------------+-------+--------|
> | 983ccfc0...4262b0bf | 03539473...72edf3cc | 100000          | true  | true   |
> |---------------------+---------------------+-----------------+-------+--------|
> | b7f62dd2...dfb3d260 | 03539473...72edf3cc | 50000           | true  | true   |
> +---------------------+---------------------+-----------------+-------+--------+
>
> ```


**Run:**


```bash
$ rgbldk wallet sync

```


**Result:**

> ```text
> Wallet synced.
> Balance change: on-chain total -100,225 sats, spendable -100,225 sats, lightning 0 sats.
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


## 6) Payments (invoice create/pay + keysend)

Create Bolt11 invoices on node-b and pay them from node-a, then query `pay status` to verify the final state. Also demonstrate a spontaneous (keysend) payment.

**Run:**


```bash
$ rgbldk ctx use node-b

```


**Result:**

> ```text
> Switched to context "node-b".
> Next: run `rgbldk node status` to verify the connection.
>
> ```


**Run:**


```bash
$ rgbldk pay invoice --desc demo --amount-msat 10000

```


**Result:**

> ```text
> lnbcrt100n1p5cq80wdq8v3jk6mcnp4qdfegu6hsfydruvtye3n2yhq2ntqx2vdzsu3ya4wayjdlrtjaheucpp52pq4wafrtj72adkk3e4q5xj3gffdlm4t8cuxhngpt3craxs0xewssp5zt823awlau2cakshtas0557y4kpt63mlh0rvxflzpnmrumnenaqs9qyysgqcqpcxqrrssrzjqvq9mwemzt6lwmz9n3ckwhq250v0arjluscv3rdc64xasu88at3awqqqqyqqy4cqqyqqqqlgqqqqqqqqfqx4l60v6dftmzet343632f59cljcnfg33k2399x2k0cexfh6hfm8r2m2q2sxqflq4mdespjn89ez923etsnee7rcmvz23tea37v23ueqp9yvccv
>
> ```


**Run:**


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
$ rgbldk pay send --invoice <invoice>

```


**Result:**

> ```text
> 50415775235cbcaeb6d68e6a0a1a514252dfeeab3e386bcd015c703e9a0f365d
>
> ```


**Run:**


```bash
$ rgbldk pay status 50415775235cbcaeb6d68e6a0a1a514252dfeeab3e386bcd015c703e9a0f365d

```


**Result:**

> ```text
> +-----------------+------------------------------------------------------------------+
> | Field           | Value                                                            |
> +====================================================================================+
> | id              | 50415775235cbcaeb6d68e6a0a1a514252dfeeab3e386bcd015c703e9a0f365d |
> |-----------------+------------------------------------------------------------------|
> | direction       | Outbound                                                         |
> |-----------------+------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                      |
> |-----------------+------------------------------------------------------------------|
> | kind            | Bolt11                                                           |
> |-----------------+------------------------------------------------------------------|
> | amount (msat)   | 10,000 msat                                                      |
> |-----------------+------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                           |
> +-----------------+------------------------------------------------------------------+
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
$ rgbldk pay invoice --desc demo-var

```


**Result:**

> ```text
> lnbcrt1p5cq80wdqdv3jk6medweshynp4qdfegu6hsfydruvtye3n2yhq2ntqx2vdzsu3ya4wayjdlrtjaheucpp5dw3d8sa3p60ex2ssvm9n862l8s9he7em2zlhh0km8c975f5w6g3qsp57u5xswlrqvgerja52h7zv5sel9g76kpswmdxatlj3cpdkyu24pus9qyysgqcqpcxqrrssrzjqvq9mwemzt6lwmz9n3ckwhq250v0arjluscv3rdc64xasu88at3awqqqqyqqr2cqqvqqqqlgqqqqqqqqfqetkd9ydtxj6ha7hlfm5gpxhv978l5330me7csdc57j6lkzfwxu358pmjjzterwq33ffnllytc58xtqazjdw546tv5ctkpr6wc2fqf7gqsrzkaf
>
> ```


**Run:**


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
$ rgbldk pay send --invoice <invoice> --amount-msat 11000

```


**Result:**

> ```text
> 6ba2d3c3b10e9f932a1066cb33e95f3c0b7cfb3b50bf7bbedb3e0bea268ed222
>
> ```


**Run:**


```bash
$ rgbldk pay status 6ba2d3c3b10e9f932a1066cb33e95f3c0b7cfb3b50bf7bbedb3e0bea268ed222

```


**Result:**

> ```text
> +-----------------+------------------------------------------------------------------+
> | Field           | Value                                                            |
> +====================================================================================+
> | id              | 6ba2d3c3b10e9f932a1066cb33e95f3c0b7cfb3b50bf7bbedb3e0bea268ed222 |
> |-----------------+------------------------------------------------------------------|
> | direction       | Outbound                                                         |
> |-----------------+------------------------------------------------------------------|
> | status          | ✔ Succeeded                                                      |
> |-----------------+------------------------------------------------------------------|
> | kind            | Bolt11                                                           |
> |-----------------+------------------------------------------------------------------|
> | amount (msat)   | 11,000 msat                                                      |
> |-----------------+------------------------------------------------------------------|
> | fee paid (msat) | 0 msat                                                           |
> +-----------------+------------------------------------------------------------------+
>
> ```


**Run:**


```bash
$ rgbldk pay keysend --node-id <node_id_b> --amount-msat 1234 --tlv 70001:02

```


**Result:**

> ```text
> 440aef7e24450731bdab05ae555eef116fcfc4767e262eaa34d4eb9b6c38f337
>
> ```


**Run:**


```bash
$ rgbldk pay status 440aef7e24450731bdab05ae555eef116fcfc4767e262eaa34d4eb9b6c38f337

```


**Result:**

> ```text
> +-----------------+------------------------------------------------------------------+
> | Field           | Value                                                            |
> +====================================================================================+
> | id              | 440aef7e24450731bdab05ae555eef116fcfc4767e262eaa34d4eb9b6c38f337 |
> |-----------------+------------------------------------------------------------------|
> | direction       | Outbound                                                         |
> |-----------------+------------------------------------------------------------------|
> | status          | … Pending                                                        |
> |-----------------+------------------------------------------------------------------|
> | kind            | Spontaneous                                                      |
> |-----------------+------------------------------------------------------------------|
> | amount (msat)   | 1,234 msat                                                       |
> |-----------------+------------------------------------------------------------------|
> | fee paid (msat) | -                                                                |
> +-----------------+------------------------------------------------------------------+
>
> ```


## 7) Events (next/handled)

Demonstrate the event queue API: fetch the next event (`events next`) and acknowledge it (`events handled`) so the daemon can advance the queue.

**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8501 events next

```


**Result:**

> ```text
> PaymentSuccessful payment_id=81aab5f0d7e318a74150d8adb8b758b80b8a52a284f424abf29ac154c9982bcf fee_paid=0 msat
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
> PaymentReceived payment_id=81aab5f0d7e318a74150d8adb8b758b80b8a52a284f424abf29ac154c9982bcf amount=10,000 msat
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


## 8) Channel (close/force-close)

Demonstrate graceful close vs force-close. Force-close is destructive and requires `--yes` for non-interactive safety.

**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8501 channel open --node-id 03539473578248d1f18b26633512e054d603298d14391276aee924df8d72edf3cc --addr node-b:9735 --amount-sats 50000 --private

```


**Result:**

> ```text
> 41dd55d812060b9c6f5da9a087e391e5
>
> ```


**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8501 --yes channel force-close --user-channel-id 41dd55d812060b9c6f5da9a087e391e5 --counterparty-node-id 03539473578248d1f18b26633512e054d603298d14391276aee924df8d72edf3cc

```


**Result:**

> ```text
> Channel force-close initiated.
>
> ```


**Run:**


```bash
$ rgbldk --connect http://127.0.0.1:8501 channel close --user-channel-id 983ccfc04689ea3a181dce394262b0bf --counterparty-node-id 03539473578248d1f18b26633512e054d603298d14391276aee924df8d72edf3cc

```


**Result:**

> ```text
> Channel close initiated.
>
> ```


## 9) Cleanup

Tear down the docker-compose stack and remove volumes.

```bash
$ docker compose -f crates/cli/docker-compose.yaml down -v

```
