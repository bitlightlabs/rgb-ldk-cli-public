import type {
  BalancesDto,
  Bolt11DecodeResponse,
  Bolt11PayResponse,
  Bolt12OfferDecodeResponse,
  Bolt12RefundDecodeResponse,
  ChannelDetailsExtendedDto,
  CustomTlvDto,
  EventDto,
  PaymentDetailsDto,
  PaymentWaitResponse,
  RgbContractBalanceResponse,
  RgbContractsResponse,
  RgbContractsIssueResponse,
  RgbLnInvoiceDecodeResponse,
  RgbOnchainReceiveResponse,
} from "./types.js";
import { U64 } from "./u64.js";

type AnyRecord = Record<string, any>;

function asRecord(value: unknown): AnyRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("Invalid JSON shape: expected object");
  }
  return value as AnyRecord;
}

function asArray(value: unknown): any[] {
  if (!Array.isArray(value)) {
    throw new Error("Invalid JSON shape: expected array");
  }
  return value;
}

function decodeU64(value: unknown): U64 {
  return U64.from(value as any);
}

function decodeU64Nullable(value: unknown): U64 | null {
  if (value === null || value === undefined) return null;
  return decodeU64(value);
}

export function decodeBalancesDto(value: unknown): BalancesDto {
  const v = asRecord(value);
  const btc = asRecord(v.btc);
  btc.onchain_total_sats = decodeU64(btc.onchain_total_sats);
  btc.onchain_spendable_sats = decodeU64(btc.onchain_spendable_sats);
  btc.anchor_channels_reserve_sats = decodeU64(btc.anchor_channels_reserve_sats);
  btc.lightning_total_sats = decodeU64(btc.lightning_total_sats);
  v.btc = btc;

  const rgb = asRecord(v.rgb);
  rgb.l1 = asArray(rgb.l1).map((row) => {
    const r = asRecord(row);
    r.mined = decodeU64(r.mined);
    r.tentative = decodeU64(r.tentative);
    r.offchain = decodeU64(r.offchain);
    r.archived = decodeU64(r.archived);
    r.total = decodeU64(r.total);
    return r;
  });
  rgb.l2 = asArray(rgb.l2).map((row) => {
    const r = asRecord(row);
    r.local_amount = decodeU64(r.local_amount);
    r.remote_amount = decodeU64(r.remote_amount);
    return r;
  });
  v.rgb = rgb;
  return v as BalancesDto;
}

export function decodeChannelDetailsExtendedDto(value: unknown): ChannelDetailsExtendedDto {
  const v = asRecord(value);
  v.channel_value_sats = decodeU64(v.channel_value_sats);
  v.outbound_capacity_msat = decodeU64(v.outbound_capacity_msat);
  v.inbound_capacity_msat = decodeU64(v.inbound_capacity_msat);
  return v as ChannelDetailsExtendedDto;
}

export function decodeBolt11DecodeResponse(value: unknown): Bolt11DecodeResponse {
  const v = asRecord(value);
  v.amount_msat = decodeU64Nullable(v.amount_msat);
  return v as Bolt11DecodeResponse;
}

export function decodeBolt11PayResponse(value: unknown): Bolt11PayResponse {
  const v = asRecord(value);
  v.amount_sats = decodeU64(v.amount_sats);
  v.fee_paid_msat = decodeU64Nullable(v.fee_paid_msat);
  return v as Bolt11PayResponse;
}

export function decodeRgbContractsIssueResponse(value: unknown): RgbContractsIssueResponse {
  const v = asRecord(value);
  v.issued_supply = decodeU64(v.issued_supply);
  return v as RgbContractsIssueResponse;
}

export function decodeRgbContractsResponse(value: unknown): RgbContractsResponse {
  const v = asRecord(value);
  const contracts = asArray(v.contracts).map((c) => {
    const cc = asRecord(c);
    if (cc.issued_supply !== null && cc.issued_supply !== undefined) cc.issued_supply = decodeU64(cc.issued_supply);
    return cc;
  });
  v.contracts = contracts;
  return v as RgbContractsResponse;
}

export function decodeRgbContractBalanceResponse(value: unknown): RgbContractBalanceResponse {
  const v = asRecord(value);
  const balance = asRecord(v.balance);
  balance.mined = decodeU64(balance.mined);
  balance.tentative = decodeU64(balance.tentative);
  balance.offchain = decodeU64(balance.offchain);
  balance.archived = decodeU64(balance.archived);
  balance.total = decodeU64(balance.total);
  v.balance = balance;
  return v as RgbContractBalanceResponse;
}

export function decodeRgbLnInvoiceDecodeResponse(value: unknown): RgbLnInvoiceDecodeResponse {
  const v = asRecord(value);
  v.carrier_amount_msat = decodeU64Nullable(v.carrier_amount_msat);
  if (v.asset_amount !== null && v.asset_amount !== undefined) v.asset_amount = decodeU64(v.asset_amount);
  return v as RgbLnInvoiceDecodeResponse;
}

export function decodeRgbOnchainReceiveResponse(value: unknown): RgbOnchainReceiveResponse {
  const v = asRecord(value);
  v.amount = decodeU64(v.amount);
  return v as RgbOnchainReceiveResponse;
}

export function decodePaymentDetailsDto(value: unknown): PaymentDetailsDto {
  const v = asRecord(value);
  v.amount_msat = decodeU64Nullable(v.amount_msat);
  v.fee_paid_msat = decodeU64Nullable(v.fee_paid_msat);
  return v as PaymentDetailsDto;
}

export function decodePaymentWaitResponse(value: unknown): PaymentWaitResponse {
  const v = asRecord(value);
  v.payment = decodePaymentDetailsDto(v.payment);
  return v as PaymentWaitResponse;
}

export function decodeBolt12OfferDecodeResponse(value: unknown): Bolt12OfferDecodeResponse {
  const v = asRecord(value);
  v.amount_msat = decodeU64Nullable(v.amount_msat);
  v.absolute_expiry_unix_secs = decodeU64Nullable(v.absolute_expiry_unix_secs);
  return v as Bolt12OfferDecodeResponse;
}

export function decodeBolt12RefundDecodeResponse(value: unknown): Bolt12RefundDecodeResponse {
  const v = asRecord(value);
  v.amount_msat = decodeU64(v.amount_msat);
  v.absolute_expiry_unix_secs = decodeU64Nullable(v.absolute_expiry_unix_secs);
  v.quantity = decodeU64Nullable(v.quantity);
  return v as Bolt12RefundDecodeResponse;
}

function decodeCustomTlvDto(value: unknown): CustomTlvDto {
  const v = asRecord(value);
  v.type = decodeU64(v.type);
  return v as CustomTlvDto;
}

export function decodeEventDto(value: unknown): EventDto {
  const v = asRecord(value);
  const t = v.type;
  if (t === "PaymentSuccessful") {
    v.data = asRecord(v.data);
    v.data.fee_paid_msat = decodeU64Nullable(v.data.fee_paid_msat);
    return v as EventDto;
  }
  if (t === "PaymentReceived") {
    v.data = asRecord(v.data);
    v.data.amount_msat = decodeU64(v.data.amount_msat);
    if (Array.isArray(v.data.custom_records)) {
      v.data.custom_records = v.data.custom_records.map(decodeCustomTlvDto);
    }
    if (v.data.rgb) {
      const rgb = asRecord(v.data.rgb);
      rgb.asset_amount = decodeU64(rgb.asset_amount);
      v.data.rgb = rgb;
    }
    return v as EventDto;
  }
  return v as EventDto;
}

export function decodeArray<T>(decodeItem: (value: unknown) => T): (value: unknown) => T[] {
  return (value: unknown) => asArray(value).map(decodeItem);
}
