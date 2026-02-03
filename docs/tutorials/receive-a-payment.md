# Tutorial: Receive a Payment (invoice)

## 1) Create invoice

On the receiver node:

- `POST /api/v1/bolt11/receive` (fixed amount) or
- `POST /api/v1/bolt11/receive_var` (payer chooses amount)

## 2) Deliver invoice to payer

Return the invoice string to the payer (out-of-band).

## 3) Watch for `PaymentReceived`

Consume events:

- `POST /api/v1/events/wait_next`
- Expect `PaymentReceived`
- Process it (e.g., update your DB)
- `POST /api/v1/events/handled`

