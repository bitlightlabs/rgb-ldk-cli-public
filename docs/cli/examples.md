# CLI Examples (copy/paste)

These are intentionally short. For full end-to-end flows, see the tutorials in `../tutorials/`.

## Basic node checks

```bash
rgbldk node version
rgbldk node health
rgbldk node ready
rgbldk node status
```

## Wallet funding (regtest)

```bash
# 1) Get an address
addr="$(rgbldk --output json wallet address | jq -r .address)"

# 2) Send coins (using the repo's regtest docker-compose stack)
docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress "$addr" 1
docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 "$(
  docker compose -f crates/cli/docker-compose.yaml exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin getnewaddress
)"

# 3) Sync wallet
rgbldk wallet sync

# 4) Check balances
rgbldk wallet balance
```

## Event loop skeleton

```bash
while true; do
  rgbldk events next
  rgbldk events handled
done
```

## BOLT12 offer: receive + pay

Terminal A (receiver):

```bash
offer="$(rgbldk pay offer create --desc "coffee" --amount-msat 1000000)"
echo "$offer"
```

Terminal B (payer):

```bash
pid="$(rgbldk pay offer pay --offer "$offer")"
rgbldk pay wait "$pid" --timeout-secs 60
```

If the payment is waiting on an invoice (recipient offline / onion message not delivered yet),
retry `pay wait` or cancel it:

```bash
rgbldk pay abandon "$pid"
```

## BOLT12 refund: initiate + request-payment

Terminal B (payer initiates a refund request):

```bash
refund="$(rgbldk pay refund initiate --amount-msat 5000000)"
echo "$refund"
```

Terminal A (payee requests the actual payment for the refund):

```bash
pid="$(rgbldk --output json pay refund request-payment --refund "$refund" | jq -r .payment_id)"
rgbldk pay wait "$pid" --timeout-secs 60
```
