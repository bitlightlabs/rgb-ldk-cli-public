// TypeScript DTOs mirroring src/http/dto.rs

export { U64 } from "./u64.js";
import type { U64 } from "./u64.js";

export interface UnlockedStatusDto {
  is_running: boolean;
  is_listening: boolean;
  best_block_height: number;
  locked?: false;
}

export interface LockedStatusDto {
  ok: boolean;
  locked: true;
  running: boolean;
  checks?: HealthCheckDto[];
}

export type StatusDto = UnlockedStatusDto | LockedStatusDto;

export interface BalancesDto {
  btc: BtcBalancesDto;
  rgb: RgbBalancesDto;
}

export interface BtcBalancesDto {
  onchain_total_sats: U64;
  onchain_spendable_sats: U64;
  anchor_channels_reserve_sats: U64;
  lightning_total_sats: U64;
}

export interface RgbBalancesDto {
  l1: RgbL1BalanceDto[];
  l2: RgbL2BalanceDto[];
}

export interface RgbL1BalanceDto {
  contract_id: string;
  mined: U64;
  tentative: U64;
  offchain: U64;
  archived: U64;
  total: U64;
}

export interface RgbL2BalanceDto {
  channel_id: string;
  contract_id: string;
  local_amount: U64;
  remote_amount: U64;
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
  amount_msat: U64 | null;
  kind: PaymentKind;
  fee_paid_msat: U64 | null;
  payment_hash?: string | null;
  htlc_locked: boolean;
  kind_details?: any;
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
  channel_value_sats: U64;
  outbound_capacity_msat: U64;
  inbound_capacity_msat: U64;
  is_channel_ready: boolean;
  is_usable: boolean;
  is_announced: boolean;
  short_channel_id?: U64 | null;
  outbound_scid_alias?: U64 | null;
  inbound_scid_alias?: U64 | null;
}

export interface OpenChannelRequest {
  node_id: string;
  address: string;
  channel_amount_sats: U64;
  push_to_counterparty_msat?: U64 | null;
  announce?: boolean;
  rgb?: RgbOpenChannelRequest | null;
}

export interface OpenChannelResponse {
  user_channel_id: string;
}

export interface CloseChannelRequest {
  user_channel_id: string;
  counterparty_node_id: string;
}

export interface Bolt11ReceiveRequest {
  amount_msat: U64;
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
  amount_msat: U64;
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
  amount_msat: U64 | null;
  expiry_secs: number;
}

export interface Bolt11PayRequest {
  invoice: string;
  amount_msat?: U64 | null;
}

export interface Bolt11PayResponse {
  payment_id: string;
  preimage: string;
  amount_sats: U64;
  destination: string;
  fee_paid_msat: U64 | null;
}

export interface CustomTlvDto {
  type: U64; // r#type in Rust
  value_hex: string;
}

export interface SpontaneousSendRequest {
  counterparty_node_id: string;
  amount_msat: U64;
  custom_tlvs?: CustomTlvDto[];
}

export interface OutPointDto {
  txid: string;
  vout: number;
}

export interface RgbPaymentContextDto {
  contract_id: string;
  asset_amount: U64;
  direction: string; // Inbound | Outbound
  is_swap: boolean;
}

export type EventDto =
  | { type: "PaymentSuccessful"; data: { payment_id: string | null; fee_paid_msat: U64 | null } }
  | { type: "PaymentFailed"; data: { payment_id: string | null } }
  | {
    type: "PaymentReceived";
    data: {
      payment_id: string | null;
      payment_hash: string;
      amount_msat: U64;
      custom_records?: CustomTlvDto[];
      rgb?: RgbPaymentContextDto | null;
    };
  }
  | { type: "ChannelPending"; data: { funding_txo: OutPointDto } }
  | { type: "ChannelReady"; data: { user_channel_id: string } }
  | {
    type: "ChannelClosed";
    data: {
      channel_id: string;
      user_channel_id: string;
      counterparty_node_id?: string | null;
      reason?: string | null;
    };
  }
  | { type: "Other"; data: { kind: string } };

export interface HealthCheckDto {
  name: string;
  ok: boolean;
  detail?: string | null;
  hint?: string | null;
}

export interface OkResponse {
  ok: boolean;
  checks?: HealthCheckDto[];
}

export interface NodeIdResponse {
  node_id: string;
}

export interface ListeningAddressesResponse {
  addresses: string[];
}

// ---- RGB ----

export interface RgbOpenChannelRequest {
  contract_id: string;
  asset_amount: U64;
  color_context_data: string; // e.g. file://...
}

export interface RgbNewAddressResponse {
  address: string;
}

export interface RgbDerivedDescriptorDto {
  fingerprint: string;
  derivation_path: string;
  xpub: string;
  descriptor: string;
}

export interface RgbDescriptorResponse {
  descriptor: string;
  derived_descriptors?: RgbDerivedDescriptorDto[];
}

export type RgbSignMessageAlgorithmDto = "bitcoin_signed_message" | "ecdsa";
export type RgbSignMessageEncodingDto = "hex" | "base64";

export interface RgbSignMessageRequest {
  message: string;
  algorithm?: RgbSignMessageAlgorithmDto | null;
  compact?: boolean | null;
  encoding?: RgbSignMessageEncodingDto | null;
}

export interface RgbSignMessageResponse {
  message: string;
  algorithm: string;
  signature: string;
  encoding?: string | null;
  compact?: boolean | null;
  pubkey: string;
  derivation_path: string;
  digest_hex?: string | null;
}

export interface RgbContractDto {
  contract_id: string;
  name?: string | null;
  ticker?: string | null;
  precision?: number | null; // u8
  issued_supply?: U64 | null; // u64
  details?: string | null;
}

export interface RgbContractsResponse {
  contracts: RgbContractDto[];
}

export interface RgbInvalidIssuerDto {
  name: string;
  error: string;
}

export interface RgbIssuersResponse {
  issuers: string[];
  invalid_issuers?: RgbInvalidIssuerDto[];
}

export interface RgbIssuersImportResponse {
  ok: boolean;
  issuer_name: string;
  checks?: HealthCheckDto[];
}

export interface RgbContractsImportResponse {
  ok: boolean;
  contract_id: string;
  consignment_key: string;
  checks?: HealthCheckDto[];
}

export interface RgbContractsIssueRequest {
  issuer_name: string;
  contract_name: string;
  ticker?: string | null;
  precision?: number | null; // u8
  issued_supply: U64;
  utxo?: string | null; // "txid:vout"
}

export interface RgbContractsIssueResponse {
  ok: boolean;
  contract_id: string;
  issued_supply: U64;
  checks?: HealthCheckDto[];
}

export interface RgbContractsExportRequest {
  contract_id: string;
}

export interface RgbContractsExportResponse {
  ok: boolean;
  contract_id: string;
  consignment_key: string;
  checks?: HealthCheckDto[];
}

export interface RgbContractBalanceDto {
  mined: U64;
  tentative: U64;
  offchain: U64;
  archived: U64;
  total: U64;
}

export interface RgbContractBalanceResponse {
  contract_id: string;
  balance: RgbContractBalanceDto;
}

export interface RgbContractKnownResponse {
  contract_id: string;
  known: boolean;
}

export interface RgbLnCarrierEstimateChannelDto {
  channel_id: string;
  user_channel_id: string;
  is_usable: boolean;
  inbound_capacity_msat: U64;
  inbound_htlc_minimum_msat: U64;
  inbound_htlc_maximum_msat?: U64 | null;
  local_balance_output_sats: U64;
  has_holder_reserve: boolean;
  receive_available: boolean;
  can_receive_rgb_invoice: boolean;
  blocking_reason?: string | null;
  required_carrier_msat?: U64 | null;
  required_carrier_reason?: string | null;
  available_inbound_capacity_msat: U64;
  suggested_action?: string | null;
  minimum_viable_carrier_amount_msat?: U64 | null;
  minimum_viable_reason?: string | null;
  default_create_carrier_amount_msat?: U64 | null;
  default_create_reason?: string | null;
}

export interface RgbLnCarrierEstimateResponse {
  receive_available: boolean;
  can_create_rgb_invoice: boolean;
  blocking_reason?: string | null;
  required_carrier_msat?: U64 | null;
  required_carrier_reason?: string | null;
  available_inbound_capacity_msat?: U64 | null;
  suggested_action?: string | null;
  minimum_viable_carrier_amount_msat: U64;
  minimum_viable_reason: string;
  default_create_carrier_amount_msat: U64;
  default_create_reason: string;
  carrier_admission_threshold_msat: U64;
  minimum_allowed_carrier_amount_msat: U64;
  holder_reserve_threshold_msat: U64;
  channels: RgbLnCarrierEstimateChannelDto[];
  estimate_only: boolean;
  warning: string;
}

export interface SwapCreateOfferRequest {
  counterparty_node_id: string;
  channel_scid: U64;
  contract_id: string;
  asset_amount: U64;
  btc_amount_msat: U64;
  btc_carrier_amount_msat: U64;
  maker_gives_rgb: boolean;
  expiry_secs: number;
}

export interface SwapHopDto {
  node_id: string;
  channel_scid: U64;
}

export interface SwapCreateMultihopOfferRequest {
  rgb_path: SwapHopDto[];
  btc_path: SwapHopDto[];
  contract_id: string;
  asset_amount: U64;
  btc_amount_msat: U64;
  btc_carrier_amount_msat: U64;
  maker_gives_rgb: boolean;
  expiry_secs: number;
}

export interface SwapStringRequest {
  swap_string: string;
}

export interface SwapExecuteRequest {
  swap_string?: string | null;
  payment_hash?: string | null;
  force?: boolean | null;
}

export interface SwapExecuteResponse {
  ok: boolean;
  payment_hash: string;
  status: string;
}

export interface SwapInfoDto {
  payment_hash: string;
  role: string;
  status: string;
  counterparty_node_id: string;
  channel_scid: U64;
  contract_id: string;
  asset_amount: U64;
  btc_amount_msat: U64;
  btc_carrier_amount_msat: U64;
  maker_gives_rgb: boolean;
  expiry_secs: number;
  created_at_unix_secs: U64;
  is_multihop: boolean;
  last_error?: string | null;
}

export interface SwapOfferResponse {
  swap_string: string;
  payment_hash: string;
  info: SwapInfoDto;
}

export interface RgbLnInvoiceCreateRequest {
  contract_id: string;
  asset_amount: U64;
  description: string;
  expiry_secs?: number | null;
  btc_carrier_amount_msat?: U64 | null;
}

export interface RgbLnInvoiceCreateForHashRequest {
  contract_id: string;
  asset_amount: U64;
  payment_hash: string;
  description: string;
  expiry_secs?: number | null;
  btc_carrier_amount_msat?: U64 | null;
}

export interface RgbLnInvoiceResponse {
  invoice: string;
  btc_carrier_amount_msat: U64;
}

export interface RgbLnInvoiceDecodeRequest {
  invoice: string;
}

export interface RgbLnInvoiceDecodeResponse {
  payment_hash: string;
  destination: string;
  carrier_amount_msat?: U64 | null;
  expiry_secs: U64;
  contract_id?: string | null;
  asset_amount?: U64 | null;
}

export interface RgbLnPayRequest {
  invoice: string;
  contract_id?: string | null;
  asset_amount?: U64 | null;
}

export interface RgbOnchainInvoiceCreateRequest {
  contract_id: string;
  amount: U64;
  expiry_secs?: U64 | null;
  use_witness_utxo?: boolean;
  nonce?: U64 | null;
  blinding_utxo?: string | null;
}

export interface RgbOnchainInvoiceResponse {
  invoice: string;
  blinding_utxo_used?: string | null;
}

export interface RgbOnchainInvoiceDecodeRequest {
  invoice: string;
}

export interface RgbOnchainInvoiceDecodeResponse {
  contract_id: string;
  amount: U64;
  beneficiary: string;
  use_witness_utxo: boolean;
  expiry_unix_secs?: U64 | null;
}

export interface RgbOnchainPaymentDto {
  id: string;
  kind: string;
  status: string;
  created_at_unix_secs: U64;
  latest_update_timestamp: U64;
  expires_at_unix_secs?: U64 | null;
  invoice?: string | null;
  contract_id?: string | null;
  amount?: U64 | null;
  txid?: string | null;
  consignment_key?: string | null;
  consignment_download_path?: string | null;
}

export interface RgbOnchainPaymentsResponse {
  payments: RgbOnchainPaymentDto[];
}

export interface RgbOnchainSendRequest {
  invoice: string;
  sats_for_fee_and_outputs?: U64 | null;
  fee_rate_sats_per_vb: number;
}

export interface RgbOnchainSendResponse {
  txid: string;
  consignment_key: string;
}

export interface RgbOnchainReceiveRequest {
  consignment_key: string;
  payment_id?: string | null;
  invoice?: string | null;
}

export interface RgbOnchainReceiveResponse {
  contract_id: string;
  amount: U64;
}

export interface RgbAllocationDto {
  contract_id: string;
  amount: U64;
  layer: string;
}

export interface RgbUtxoAssetAllocationDto {
  contract_id: string;
  amount: U64;
}

export interface RgbUtxoDto {
  outpoint: string;
  value_sats: U64;
  confirmed_height?: number | null;
  rgb_allocations: RgbAllocationDto[];
  has_mixed_asset_allocations: boolean;
  spend_roles: string[];
}

export interface RgbUtxoSummaryDto {
  outpoint: string;
  value_sats?: U64 | null;
  confirmed_height?: number | null;
  reserved: boolean;
  reserved_until_unix_secs?: U64 | null;
  assets?: RgbUtxoAssetAllocationDto[];
}

export interface RgbUtxosResponse {
  utxos: RgbUtxoDto[];
}

export interface RgbUtxosSummaryResponse {
  utxos: RgbUtxoSummaryDto[];
}

export interface RgbUtxosReserveRequest {
  outpoint?: string | null;
  ttl_secs?: U64 | null;
}

export interface RgbUtxosReserveResponse {
  reservation_id: string;
  outpoint: string;
  reserved_until_unix_secs: U64;
}

export interface RgbUtxosReleaseRequest {
  reservation_id?: string | null;
  outpoint?: string | null;
}

export interface RgbUtxosReleaseResponse {
  released: boolean;
}

// ---- BOLT12 (offers + refunds) ----

export interface Bolt12OfferReceiveRequest {
  amount_msat: U64;
  description: string;
  expiry_secs?: number | null;
  quantity?: U64 | null;
}

export interface Bolt12OfferReceiveVarRequest {
  description: string;
  expiry_secs?: number | null;
}

export interface Bolt12OfferResponse {
  offer: string; // bech32 lno...
}

export interface Bolt12OfferDecodeRequest {
  offer: string;
}

export interface Bolt12OfferDecodeResponse {
  offer_id: string;
  signing_pubkey?: string | null;
  description?: string | null;
  issuer?: string | null;
  amount_msat?: U64 | null;
  absolute_expiry_unix_secs?: U64 | null;
  chain_hashes: string[];
  paths_count: number;
  expects_quantity: boolean;
}

export interface Bolt12OfferSendRequest {
  offer: string;
  amount_msat?: U64 | null;
  quantity?: U64 | null;
  payer_note?: string | null;
}

export interface Bolt12RefundInitiateRequest {
  amount_msat: U64;
  expiry_secs: number;
  quantity?: U64 | null;
  payer_note?: string | null;
}

export interface Bolt12RefundInitiateResponse {
  refund: string; // bech32 lnr...
  payment_id: string;
}

export interface Bolt12RefundDecodeRequest {
  refund: string;
}

export interface Bolt12RefundDecodeResponse {
  description: string;
  issuer?: string | null;
  amount_msat: U64;
  absolute_expiry_unix_secs?: U64 | null;
  chain_hash: string;
  payer_signing_pubkey: string;
  payer_note?: string | null;
  quantity?: U64 | null;
  paths_count: number;
}

export interface Bolt12RefundRequestPaymentRequest {
  refund: string;
}

export interface Bolt12RefundRequestPaymentResponse {
  invoice: string;
  invoice_hex: string;
  payment_id: string;
}

// ---- Payments (unified) ----

export interface PaymentWaitRequest {
  timeout_secs?: number | null;
}

export interface PaymentWaitResponse {
  ok: boolean;
  payment: PaymentDetailsDto;
  checks?: HealthCheckDto[];
}
