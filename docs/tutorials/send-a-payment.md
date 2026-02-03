# Tutorial: Send a Payment (invoice + keysend)

## Pay a Bolt11 invoice

On the sender node:

- `POST /api/v1/bolt11/send` (fixed amount invoices)
- `POST /api/v1/bolt11/send_using_amount` (variable amount invoices)

Then wait for a `PaymentSuccessful` or `PaymentFailed` event and ACK.

## Keysend (spontaneous)

On the sender node:

- `POST /api/v1/spontaneous/send`

You must provide:

- `counterparty_node_id` (destination pubkey hex)
- `amount_msat`
- optional `custom_tlvs[]`

Then use events + `GET /api/v1/payment/:payment_id` to track status.

