import type {
  BalancesDto,
  Bolt11DecodeResponse,
  Bolt11PayResponse,
  Bolt12OfferDecodeResponse,
  Bolt12RefundDecodeResponse,
  ChannelDetailsExtendedDto,
  ClosingBtcBalanceDto,
  ClosingChannelDto,
  ClosingRgbDto,
  CustomTlvDto,
  EventDto,
  Lsps1InfoResponse,
  Lsps1LspConfigDto,
  Lsps1OptionsDto,
  Lsps1OrderResponse,
  Lsps1PricingDto,
  Lsps1ServiceOrderDto,
  Lsps1ServiceOrdersResponse,
  PaymentDetailsDto,
  PaymentWaitResponse,
  RgbContractBalanceResponse,
  RgbContractsResponse,
  RgbContractsIssueResponse,
  RgbLnCarrierEstimateResponse,
  RgbLnInvoiceDecodeResponse,
  RgbLnInvoiceResponse,
  RgbOnchainInvoiceDecodeResponse,
  RgbOnchainPaymentsResponse,
  RgbOnchainReceiveResponse,
  RgbUtxosMergeResponse,
  RgbUtxosMergeStatusEntryDto,
  RgbUtxosMergeStatusResponse,
  RgbUtxosReserveResponse,
  RgbUtxosResponse,
  RgbUtxosSummaryResponse,
  SwapInfoDto,
  SwapOfferResponse,
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
  v.short_channel_id = decodeU64Nullable(v.short_channel_id);
  v.outbound_scid_alias = decodeU64Nullable(v.outbound_scid_alias);
  v.inbound_scid_alias = decodeU64Nullable(v.inbound_scid_alias);
  return v as ChannelDetailsExtendedDto;
}

function decodeClosingBtcBalanceDto(value: unknown): ClosingBtcBalanceDto {
  const v = asRecord(value);
  v.amount_sats = decodeU64(v.amount_sats);
  return v as ClosingBtcBalanceDto;
}

function decodeClosingRgbDto(value: unknown): ClosingRgbDto {
  const v = asRecord(value);
  v.local_amount = decodeU64(v.local_amount);
  v.remote_amount = decodeU64(v.remote_amount);
  return v as ClosingRgbDto;
}

export function decodeClosingChannelDto(value: unknown): ClosingChannelDto {
  const v = asRecord(value);
  v.btc_balances = asArray(v.btc_balances).map(decodeClosingBtcBalanceDto);
  v.sweeping_balances = asArray(v.sweeping_balances).map(decodeClosingBtcBalanceDto);
  if (v.rgb !== null && v.rgb !== undefined) {
    v.rgb = decodeClosingRgbDto(v.rgb);
  }
  return v as ClosingChannelDto;
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

export function decodeRgbLnCarrierEstimateResponse(value: unknown): RgbLnCarrierEstimateResponse {
  const v = asRecord(value);
  v.minimum_viable_carrier_amount_msat = decodeU64(v.minimum_viable_carrier_amount_msat);
  v.default_create_carrier_amount_msat = decodeU64(v.default_create_carrier_amount_msat);
  v.carrier_admission_threshold_msat = decodeU64(v.carrier_admission_threshold_msat);
  v.minimum_allowed_carrier_amount_msat = decodeU64(v.minimum_allowed_carrier_amount_msat);
  v.holder_reserve_threshold_msat = decodeU64(v.holder_reserve_threshold_msat);
  v.required_carrier_msat = decodeU64Nullable(v.required_carrier_msat);
  v.available_inbound_capacity_msat = decodeU64Nullable(v.available_inbound_capacity_msat);
  v.channels = asArray(v.channels).map((channel) => {
    const c = asRecord(channel);
    c.inbound_capacity_msat = decodeU64(c.inbound_capacity_msat);
    c.inbound_htlc_minimum_msat = decodeU64(c.inbound_htlc_minimum_msat);
    c.inbound_htlc_maximum_msat = decodeU64Nullable(c.inbound_htlc_maximum_msat);
    c.local_balance_output_sats = decodeU64(c.local_balance_output_sats);
    c.required_carrier_msat = decodeU64Nullable(c.required_carrier_msat);
    c.available_inbound_capacity_msat = decodeU64(c.available_inbound_capacity_msat);
    c.minimum_viable_carrier_amount_msat = decodeU64Nullable(c.minimum_viable_carrier_amount_msat);
    c.default_create_carrier_amount_msat = decodeU64Nullable(c.default_create_carrier_amount_msat);
    return c;
  });
  return v as RgbLnCarrierEstimateResponse;
}

export function decodeRgbLnInvoiceResponse(value: unknown): RgbLnInvoiceResponse {
  const v = asRecord(value);
  v.btc_carrier_amount_msat = decodeU64(v.btc_carrier_amount_msat);
  return v as RgbLnInvoiceResponse;
}

export function decodeRgbLnInvoiceDecodeResponse(value: unknown): RgbLnInvoiceDecodeResponse {
  const v = asRecord(value);
  v.carrier_amount_msat = decodeU64Nullable(v.carrier_amount_msat);
  v.expiry_secs = decodeU64(v.expiry_secs);
  if (v.asset_amount !== null && v.asset_amount !== undefined) v.asset_amount = decodeU64(v.asset_amount);
  return v as RgbLnInvoiceDecodeResponse;
}

export function decodeRgbOnchainInvoiceDecodeResponse(value: unknown): RgbOnchainInvoiceDecodeResponse {
  const v = asRecord(value);
  v.amount = decodeU64(v.amount);
  v.expiry_unix_secs = decodeU64Nullable(v.expiry_unix_secs);
  return v as RgbOnchainInvoiceDecodeResponse;
}

export function decodeRgbOnchainPaymentsResponse(value: unknown): RgbOnchainPaymentsResponse {
  const v = asRecord(value);
  v.payments = asArray(v.payments).map((payment) => {
    const p = asRecord(payment);
    p.created_at_unix_secs = decodeU64(p.created_at_unix_secs);
    p.latest_update_timestamp = decodeU64(p.latest_update_timestamp);
    p.expires_at_unix_secs = decodeU64Nullable(p.expires_at_unix_secs);
    p.amount = decodeU64Nullable(p.amount);
    return p;
  });
  return v as RgbOnchainPaymentsResponse;
}

export function decodeRgbOnchainReceiveResponse(value: unknown): RgbOnchainReceiveResponse {
  const v = asRecord(value);
  v.amount = decodeU64(v.amount);
  return v as RgbOnchainReceiveResponse;
}

export function decodeRgbUtxosResponse(value: unknown): RgbUtxosResponse {
  const v = asRecord(value);
  v.utxos = asArray(v.utxos).map((utxo) => {
    const u = asRecord(utxo);
    u.value_sats = decodeU64(u.value_sats);
    u.rgb_allocations = asArray(u.rgb_allocations).map((allocation) => {
      const a = asRecord(allocation);
      a.amount = decodeU64(a.amount);
      return a;
    });
    return u;
  });
  return v as RgbUtxosResponse;
}

export function decodeRgbUtxosSummaryResponse(value: unknown): RgbUtxosSummaryResponse {
  const v = asRecord(value);
  v.utxos = asArray(v.utxos).map((utxo) => {
    const u = asRecord(utxo);
    u.value_sats = decodeU64Nullable(u.value_sats);
    u.reserved_until_unix_secs = decodeU64Nullable(u.reserved_until_unix_secs);
    if (Array.isArray(u.assets)) {
      u.assets = u.assets.map((asset) => {
        const a = asRecord(asset);
        a.amount = decodeU64(a.amount);
        return a;
      });
    }
    return u;
  });
  return v as RgbUtxosSummaryResponse;
}

export function decodeRgbUtxosReserveResponse(value: unknown): RgbUtxosReserveResponse {
  const v = asRecord(value);
  v.reserved_until_unix_secs = decodeU64(v.reserved_until_unix_secs);
  return v as RgbUtxosReserveResponse;
}

export function decodeRgbUtxosMergeResponse(value: unknown): RgbUtxosMergeResponse {
  const v = asRecord(value);
  v.total_amount = decodeU64(v.total_amount);
  return v as RgbUtxosMergeResponse;
}

export function decodeRgbUtxosMergeStatusResponse(value: unknown): RgbUtxosMergeStatusResponse {
  const v = asRecord(value);
  v.merges = asArray(v.merges).map((merge) => {
    const m = asRecord(merge);
    return m as RgbUtxosMergeStatusEntryDto;
  });
  return v as RgbUtxosMergeStatusResponse;
}

export function decodePaymentDetailsDto(value: unknown): PaymentDetailsDto {
  const v = asRecord(value);
  v.amount_msat = decodeU64Nullable(v.amount_msat);
  v.fee_paid_msat = decodeU64Nullable(v.fee_paid_msat);
  return v as PaymentDetailsDto;
}

export function decodeSwapInfoDto(value: unknown): SwapInfoDto {
  const v = asRecord(value);
  v.channel_scid = decodeU64(v.channel_scid);
  v.asset_amount = decodeU64(v.asset_amount);
  v.btc_amount_msat = decodeU64(v.btc_amount_msat);
  v.btc_carrier_amount_msat = decodeU64(v.btc_carrier_amount_msat);
  v.created_at_unix_secs = decodeU64(v.created_at_unix_secs);
  return v as SwapInfoDto;
}

export function decodeSwapOfferResponse(value: unknown): SwapOfferResponse {
  const v = asRecord(value);
  v.info = decodeSwapInfoDto(v.info);
  return v as SwapOfferResponse;
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

// ---- LSPS1 decoders ----

function decodeLsps1SupportedOptions(value: unknown): AnyRecord {
  const v = asRecord(value);
  v.min_initial_client_balance_sat = decodeU64(v.min_initial_client_balance_sat);
  v.max_initial_client_balance_sat = decodeU64(v.max_initial_client_balance_sat);
  v.min_initial_lsp_balance_sat = decodeU64(v.min_initial_lsp_balance_sat);
  v.max_initial_lsp_balance_sat = decodeU64(v.max_initial_lsp_balance_sat);
  v.min_channel_balance_sat = decodeU64(v.min_channel_balance_sat);
  v.max_channel_balance_sat = decodeU64(v.max_channel_balance_sat);
  return v;
}

function decodeLsps1RgbFeeBreakdown(value: unknown): AnyRecord {
  const v = asRecord(value);
  v.onchain_cost_sat = decodeU64(v.onchain_cost_sat);
  v.btc_rent_sat = decodeU64(v.btc_rent_sat);
  v.asset_rent_sat = decodeU64(v.asset_rent_sat);
  v.asset_sale_sat = decodeU64(v.asset_sale_sat);
  return v;
}

export function decodeLsps1LspConfigDto(value: unknown): Lsps1LspConfigDto {
  return asRecord(value) as Lsps1LspConfigDto;
}

export function decodeLsps1InfoResponse(value: unknown): Lsps1InfoResponse {
  const v = asRecord(value);
  v.supported_options = decodeLsps1SupportedOptions(v.supported_options);
  if (v.pricing !== null && v.pricing !== undefined) {
    const p = asRecord(v.pricing);
    p.onchain_cost_sat = decodeU64Nullable(p.onchain_cost_sat);
    p.min_fee_sat = decodeU64Nullable(p.min_fee_sat);
    v.pricing = p;
  }
  if (v.rgb !== null && v.rgb !== undefined) {
    const rgb = asRecord(v.rgb);
    rgb.rgb_assets = asArray(rgb.rgb_assets).map((asset) => {
      const a = asRecord(asset);
      a.asset_unit_price_sat = decodeU64(a.asset_unit_price_sat);
      a.min_lsp_asset_balance = decodeU64(a.min_lsp_asset_balance);
      a.max_lsp_asset_balance = decodeU64(a.max_lsp_asset_balance);
      a.max_client_asset_balance = decodeU64(a.max_client_asset_balance);
      return a;
    });
    v.rgb = rgb;
  }
  return v as Lsps1InfoResponse;
}

export function decodeLsps1OptionsDto(value: unknown): Lsps1OptionsDto {
  const v = asRecord(value);
  v.supported_options = decodeLsps1SupportedOptions(v.supported_options);
  const service = asRecord(v.service);
  service.late_deposit_refund_window_secs = decodeU64(service.late_deposit_refund_window_secs);
  v.service = service;
  return v as Lsps1OptionsDto;
}

export function decodeLsps1PricingDto(value: unknown): Lsps1PricingDto {
  const v = asRecord(value);
  const pricing = asRecord(v.pricing);
  pricing.onchain_cost_sat = decodeU64(pricing.onchain_cost_sat);
  pricing.min_fee_sat = decodeU64(pricing.min_fee_sat);
  v.pricing = pricing;
  v.assets = asArray(v.assets).map((asset) => {
    const a = asRecord(asset);
    a.asset_unit_price_sat = decodeU64(a.asset_unit_price_sat);
    a.min_lsp_asset_balance = decodeU64(a.min_lsp_asset_balance);
    a.max_lsp_asset_balance = decodeU64(a.max_lsp_asset_balance);
    a.max_client_asset_balance = decodeU64(a.max_client_asset_balance);
    return a;
  });
  return v as Lsps1PricingDto;
}

export function decodeLsps1OrderResponse(value: unknown): Lsps1OrderResponse {
  const v = asRecord(value);

  const order = asRecord(v.order);
  order.lsp_balance_sat = decodeU64(order.lsp_balance_sat);
  order.client_balance_sat = decodeU64(order.client_balance_sat);
  v.order = order;

  const payment = asRecord(v.payment);
  if (payment.bolt11 !== null && payment.bolt11 !== undefined) {
    const b = asRecord(payment.bolt11);
    b.expires_at_unix_secs = decodeU64(b.expires_at_unix_secs);
    b.fee_total_sat = decodeU64(b.fee_total_sat);
    b.order_total_sat = decodeU64(b.order_total_sat);
    payment.bolt11 = b;
  }
  if (payment.onchain !== null && payment.onchain !== undefined) {
    const o = asRecord(payment.onchain);
    o.expires_at_unix_secs = decodeU64(o.expires_at_unix_secs);
    o.fee_total_sat = decodeU64(o.fee_total_sat);
    o.order_total_sat = decodeU64(o.order_total_sat);
    payment.onchain = o;
  }
  v.payment = payment;

  if (v.channel !== null && v.channel !== undefined) {
    const c = asRecord(v.channel);
    c.funded_at_unix_secs = decodeU64(c.funded_at_unix_secs);
    c.expires_at_unix_secs = decodeU64(c.expires_at_unix_secs);
    v.channel = c;
  }

  if (v.rgb !== null && v.rgb !== undefined) {
    const rgb = asRecord(v.rgb);
    rgb.lsp_asset_balance = decodeU64(rgb.lsp_asset_balance);
    rgb.client_asset_balance = decodeU64(rgb.client_asset_balance);
    rgb.fee_breakdown = decodeLsps1RgbFeeBreakdown(rgb.fee_breakdown);
    v.rgb = rgb;
  }

  return v as Lsps1OrderResponse;
}

export function decodeLsps1ServiceOrderDto(value: unknown): Lsps1ServiceOrderDto {
  const v = asRecord(value);
  v.lsp_balance_sat = decodeU64(v.lsp_balance_sat);
  v.client_balance_sat = decodeU64(v.client_balance_sat);
  v.fee_total_sat = decodeU64(v.fee_total_sat);
  v.order_total_sat = decodeU64(v.order_total_sat);
  v.onchain_paid_sat = decodeU64Nullable(v.onchain_paid_sat);
  v.created_at_unix_secs = decodeU64(v.created_at_unix_secs);
  v.payment_expires_at_unix_secs = decodeU64(v.payment_expires_at_unix_secs);
  v.channel_closed_at_unix_secs = decodeU64Nullable(v.channel_closed_at_unix_secs);
  if (v.rgb !== null && v.rgb !== undefined) {
    const rgb = asRecord(v.rgb);
    rgb.lsp_asset_balance = decodeU64(rgb.lsp_asset_balance);
    rgb.client_asset_balance = decodeU64(rgb.client_asset_balance);
    rgb.fee_breakdown = decodeLsps1RgbFeeBreakdown(rgb.fee_breakdown);
    v.rgb = rgb;
  }
  return v as Lsps1ServiceOrderDto;
}

export function decodeLsps1ServiceOrdersResponse(value: unknown): Lsps1ServiceOrdersResponse {
  const v = asRecord(value);
  v.orders = asArray(v.orders).map((o) => decodeLsps1ServiceOrderDto(o));
  return v as Lsps1ServiceOrdersResponse;
}
