# CLI Examples (copy/paste)

These are intentionally short. For full end-to-end flows, see the tutorials in `../tutorials/`.

## Basic node checks

```bash
rgbldk version
rgbldk healthz
rgbldk readyz
rgbldk status
```

## Wallet funding (regtest)

```bash
# 1) Get an address
addr="$(rgbldk wallet new-address | jq -r .address)"

# 2) Send coins (using the repo's regtest docker-compose stack)
docker compose exec -T bitcoin-core bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress "$addr" 1
docker compose exec -T bitcoin-core bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 "$(
  docker compose exec -T bitcoin-core bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin getnewaddress
)"

# 3) Sync wallet
rgbldk wallet sync

# 4) Check balances
rgbldk balances
```

## Event loop skeleton

```bash
while true; do
  rgbldk events next
  rgbldk events handled
done
```

