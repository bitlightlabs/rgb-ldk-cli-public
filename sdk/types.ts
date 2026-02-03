// TypeScript DTOs mirroring src/http/dto.rs

export type Int = number | bigint;

export interface StatusDto {
  is_running: boolean;
  is_listening: boolean;
  best_block_height: number;
}

export interface BalancesDto {
  total_onchain_balance_sats: Int;
  spendable_onchain_balance_sats: Int;
  total_anchor_channels_reserve_sats: Int;
  total_lightning_balance_sats: Int;
}

export interface PeerDetailsDto {
  node_id: string;
  address: string;
  is_persisted: boolean;
  is_connected: boolean;
}

export interface PeerConnectRequest {
  node_id: string;
  address: string;
  persist?: boolean;
}

export interface PeerDisconnectRequest {
  node_id: string;
}

export type PaymentDirection = "Inbound" | "Outbound";
export type PaymentStatus = "Pending" | "Succeeded" | "Failed";
export type PaymentKind =
  | "Bolt11"
  | "Bolt11Jit"
  | "Bolt12Offer"
  | "Bolt12Refund"
  | "Spontaneous"
  | "Onchain";

export interface PaymentDetailsDto {
  id: string;
  direction: PaymentDirection;
  status: PaymentStatus;
  amount_msat: Int | null;
  kind: PaymentKind;
  fee_paid_msat: Int | null;
}

export interface ChannelDetailsDto {
  channel_id: string;
  counterparty_node_id: string;
  is_channel_ready: boolean;
  is_announced: boolean;
}

export interface ChannelDetailsExtendedDto {
  channel_id: string;
  user_channel_id: string;
  counterparty_node_id: string;
  channel_point: string | null;
  channel_value_sats: Int;
  outbound_capacity_msat: Int;
  inbound_capacity_msat: Int;
  is_channel_ready: boolean;
  is_usable: boolean;
  is_announced: boolean;
}

export interface OpenChannelRequest {
  node_id: string;
  address: string;
  channel_amount_sats: number;
  push_to_counterparty_msat?: number;
  announce?: boolean;
}

export interface OpenChannelResponse {
  user_channel_id: string;
}

export interface CloseChannelRequest {
  user_channel_id: string;
  counterparty_node_id: string;
}

export interface Bolt11ReceiveRequest {
  amount_msat: number;
  description: string;
  expiry_secs: number;
}

export interface Bolt11ReceiveVarRequest {
  description: string;
  expiry_secs: number;
}

export interface Bolt11ReceiveResponse {
  invoice: string;
}

export interface Bolt11SendRequest {
  invoice: string;
}

export interface Bolt11SendUsingAmountRequest {
  invoice: string;
  amount_msat: number;
}

export interface SendResponse {
  payment_id: string;
}

export interface Bolt11DecodeRequest {
  invoice: string;
}

export interface Bolt11DecodeResponse {
  payment_hash: string;
  destination: string;
  amount_msat: Int | null;
  expiry_secs: number;
}

export interface Bolt11PayRequest {
  invoice: string;
  amount_msat?: number | null;
}

export interface Bolt11PayResponse {
  payment_id: string;
  preimage: string;
  amount_sats: Int;
  destination: string;
  fee_paid_msat: Int | null;
}

export interface CustomTlvDto {
  type: number; // r#type in Rust
  value_hex: string;
}

export interface SpontaneousSendRequest {
  counterparty_node_id: string;
  amount_msat: number;
  custom_tlvs?: CustomTlvDto[];
}

export interface OutPointDto {
  txid: string;
  vout: number;
}

export type EventDto =
  | { type: "PaymentSuccessful"; data: { payment_id: string | null; fee_paid_msat: Int | null } }
  | { type: "PaymentFailed"; data: { payment_id: string | null } }
  | { type: "PaymentReceived"; data: { payment_id: string | null; amount_msat: Int } }
  | { type: "ChannelPending"; data: { funding_txo: OutPointDto } }
  | { type: "ChannelReady"; data: { user_channel_id: string } }
  | { type: "ChannelClosed"; data: {} }
  | { type: "Other"; data: { kind: string } };

export interface OkResponse {
  ok: boolean;
}

export interface NodeIdResponse {
  node_id: string;
}

export interface ListeningAddressesResponse {
  addresses: string[];
}
