import {
  BalancesDto,
  Bolt11DecodeRequest,
  Bolt11DecodeResponse,
  Bolt11PayRequest,
  Bolt11PayResponse,
  Bolt11ReceiveRequest,
  Bolt11ReceiveResponse,
  Bolt11ReceiveVarRequest,
  Bolt11SendRequest,
  Bolt11SendUsingAmountRequest,
  Bolt12OfferDecodeRequest,
  Bolt12OfferDecodeResponse,
  Bolt12OfferReceiveRequest,
  Bolt12OfferReceiveVarRequest,
  Bolt12OfferResponse,
  Bolt12OfferSendRequest,
  Bolt12RefundDecodeRequest,
  Bolt12RefundDecodeResponse,
  Bolt12RefundInitiateRequest,
  Bolt12RefundInitiateResponse,
  Bolt12RefundRequestPaymentRequest,
  Bolt12RefundRequestPaymentResponse,
  ChannelDetailsExtendedDto,
  CloseChannelRequest,
  EventDto,
  ListeningAddressesResponse,
  NodeIdResponse,
  OkResponse,
  OpenChannelRequest,
  OpenChannelResponse,
  PaymentDetailsDto,
  PaymentWaitRequest,
  PaymentWaitResponse,
  PeerConnectRequest,
  PeerDetailsDto,
  PeerDisconnectRequest,
  RgbContractBalanceResponse,
  RgbContractsExportRequest,
  RgbContractsExportResponse,
  RgbContractsIssueRequest,
  RgbContractsIssueResponse,
  RgbContractsResponse,
  RgbContractsImportResponse,
  RgbIssuersImportResponse,
  RgbIssuersResponse,
  RgbLnInvoiceCreateRequest,
  RgbLnInvoiceDecodeRequest,
  RgbLnInvoiceDecodeResponse,
  RgbLnInvoiceResponse,
  RgbLnPayRequest,
  RgbNewAddressResponse,
  RgbOnchainInvoiceCreateRequest,
  RgbOnchainInvoiceResponse,
  RgbOnchainReceiveRequest,
  RgbOnchainReceiveResponse,
  RgbOnchainSendRequest,
  RgbOnchainSendResponse,
  SendResponse,
  SpontaneousSendRequest,
  LockedStatusDto,
  StatusDto,
  UnlockedStatusDto,
} from "./types.js";
import { parse, parseNumberAndBigInt } from "lossless-json";
import {
  decodeArray,
  decodeBalancesDto,
  decodeBolt11DecodeResponse,
  decodeBolt11PayResponse,
  decodeBolt12OfferDecodeResponse,
  decodeBolt12RefundDecodeResponse,
  decodeChannelDetailsExtendedDto,
  decodeEventDto,
  decodePaymentDetailsDto,
  decodePaymentWaitResponse,
  decodeRgbContractBalanceResponse,
  decodeRgbContractsResponse,
  decodeRgbContractsIssueResponse,
  decodeRgbLnInvoiceDecodeResponse,
  decodeRgbOnchainReceiveResponse,
} from "./codec.js";

export interface RequestOptions {
  timeoutMs?: number;
  headers?: Record<string, string>;
  signal?: AbortSignal;
}

export class HttpError extends Error {
  public readonly status: number;
  public readonly body: unknown;
  constructor(message: string, status: number, body: unknown) {
    super(message);
    this.status = status;
    this.body = body;
  }
}

export const HTTP_STATUS_LOCKED = 423;

export function isHttpError(e: unknown): e is HttpError {
  return e instanceof HttpError;
}

export function isLockedHttpError(e: unknown): e is HttpError {
  return isHttpError(e) && e.status === HTTP_STATUS_LOCKED;
}

export type FetchLike = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export class NodeHttpClient {
  private readonly baseUrl: string;
  private readonly fetchFn: FetchLike;
  private readonly defaultHeaders: Record<string, string>;

  constructor(baseUrl: string, opts?: { fetch?: FetchLike; headers?: Record<string, string> }) {
    if (!baseUrl) throw new Error("baseUrl is required");
    this.baseUrl = baseUrl.replace(/\/$/, "");
    const globalFetch: any = (globalThis as any).fetch;
    this.fetchFn = opts?.fetch ?? (globalFetch?.bind(globalThis) as FetchLike);
    if (!this.fetchFn) {
      throw new Error("No fetch implementation found. Provide opts.fetch or a global fetch");
    }
    this.defaultHeaders = { "Content-Type": "application/json", ...(opts?.headers ?? {}) };
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
    options?: RequestOptions & { returnNullOn404?: boolean },
  ): Promise<T | null> {
    const url = `${this.baseUrl}${path}`;
    const headers = { ...this.defaultHeaders, ...(options?.headers ?? {}) };
    const controller = new AbortController();
    const timeout = options?.timeoutMs && options.timeoutMs > 0
      ? setTimeout(() => controller.abort(), options.timeoutMs)
      : undefined;
    if (options?.signal) {
      if (options.signal.aborted) controller.abort();
      else options.signal.addEventListener("abort", () => controller.abort());
    }

    try {
      const resp = await this.fetchFn(url, {
        method,
        headers,
        body: body !== undefined ? JSON.stringify(body, jsonBigIntReplacer) : undefined,
        signal: controller.signal,
      });
      const text = await resp.text();
      const json = text ? safeJsonParse(text) : undefined;
      if (!resp.ok) {
        if (resp.status === 404 && options?.returnNullOn404) return null;
        const msg = (json as any)?.error || `HTTP ${resp.status}`;
        throw new HttpError(msg, resp.status, json ?? text);
      }
      return (json as T) ?? ({} as T);
    } catch (e: any) {
      if (e?.name === "AbortError") {
        throw new Error("Request aborted");
      }
      throw e;
    } finally {
      if (timeout) clearTimeout(timeout);
    }
  }

  private async requestRaw<T>(
    method: string,
    path: string,
    body?: BodyInit,
    options?: RequestOptions & { returnNullOn404?: boolean; headers?: Record<string, string> },
  ): Promise<T | null> {
    const url = `${this.baseUrl}${path}`;
    const headers = { ...this.defaultHeaders, ...(options?.headers ?? {}) };
    const controller = new AbortController();
    const timeout = options?.timeoutMs && options.timeoutMs > 0
      ? setTimeout(() => controller.abort(), options.timeoutMs)
      : undefined;
    if (options?.signal) {
      if (options.signal.aborted) controller.abort();
      else options.signal.addEventListener("abort", () => controller.abort());
    }

    try {
      const resp = await this.fetchFn(url, { method, headers, body, signal: controller.signal });
      const text = await resp.text();
      const json = text ? safeJsonParse(text) : undefined;
      if (!resp.ok) {
        if (resp.status === 404 && options?.returnNullOn404) return null;
        const msg = (json as any)?.error || `HTTP ${resp.status}`;
        throw new HttpError(msg, resp.status, json ?? text);
      }
      return (json as T) ?? ({} as T);
    } catch (e: any) {
      if (e?.name === "AbortError") {
        throw new Error("Request aborted");
      }
      throw e;
    } finally {
      if (timeout) clearTimeout(timeout);
    }
  }

  private async requestBinary(
    method: string,
    path: string,
    options?: RequestOptions & { headers?: Record<string, string> },
  ): Promise<Uint8Array> {
    const url = `${this.baseUrl}${path}`;
    const headers = { ...this.defaultHeaders, ...(options?.headers ?? {}) };
    const controller = new AbortController();
    const timeout = options?.timeoutMs && options.timeoutMs > 0
      ? setTimeout(() => controller.abort(), options.timeoutMs)
      : undefined;
    if (options?.signal) {
      if (options.signal.aborted) controller.abort();
      else options.signal.addEventListener("abort", () => controller.abort());
    }

    try {
      const resp = await this.fetchFn(url, { method, headers, signal: controller.signal });
      if (!resp.ok) {
        const text = await resp.text().catch(() => "");
        const json = text ? safeJsonParse(text) : undefined;
        const msg = (json as any)?.error || `HTTP ${resp.status}`;
        throw new HttpError(msg, resp.status, json ?? text);
      }
      const ab = await resp.arrayBuffer();
      return new Uint8Array(ab);
    } catch (e: any) {
      if (e?.name === "AbortError") {
        throw new Error("Request aborted");
      }
      throw e;
    } finally {
      if (timeout) clearTimeout(timeout);
    }
  }

  // GET /status
  status(options?: RequestOptions): Promise<StatusDto> {
    return this.request<StatusDto>("GET", "/status", undefined, options) as Promise<StatusDto>;
  }

  // Convenience helper if you only want the unlocked status shape.
  async statusUnlocked(options?: RequestOptions): Promise<UnlockedStatusDto> {
    const s = await this.status(options);
    if ((s as LockedStatusDto).locked) {
      throw new Error("Daemon is locked. Run `rgbldk node unlock --passphrase-stdin` to start the node runtime.");
    }
    return s as UnlockedStatusDto;
  }

  // GET /node_id
  nodeId(options?: RequestOptions): Promise<NodeIdResponse> {
    return this.request<NodeIdResponse>("GET", "/node_id", undefined, options) as Promise<NodeIdResponse>;
  }

  // GET /listening_addresses
  listeningAddresses(options?: RequestOptions): Promise<ListeningAddressesResponse> {
    return this.request<ListeningAddressesResponse>("GET", "/listening_addresses", undefined, options) as Promise<ListeningAddressesResponse>;
  }

  // POST /wallet/new_address
  walletNewAddress(options?: RequestOptions): Promise<{ address: string }> {
    return this.request<{ address: string }>("POST", "/wallet/new_address", {}, options) as Promise<{ address: string }>;
  }

  // POST /wallet/sync
  walletSync(options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/wallet/sync", {}, options) as Promise<OkResponse>;
  }

  // ---- RGB wallet ----

  // POST /rgb/sync
  rgbSync(options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/rgb/sync", {}, options) as Promise<OkResponse>;
  }

  // POST /rgb/new_address
  rgbNewAddress(options?: RequestOptions): Promise<RgbNewAddressResponse> {
    return this.request<RgbNewAddressResponse>("POST", "/rgb/new_address", {}, options) as Promise<RgbNewAddressResponse>;
  }

  // GET /rgb/contracts
  rgbContracts(options?: RequestOptions): Promise<RgbContractsResponse> {
    return this.request<unknown>("GET", "/rgb/contracts", undefined, options).then((v) => decodeRgbContractsResponse(v));
  }

  // GET /rgb/issuers
  rgbIssuers(options?: RequestOptions): Promise<RgbIssuersResponse> {
    return this.request<RgbIssuersResponse>("GET", "/rgb/issuers", undefined, options) as Promise<RgbIssuersResponse>;
  }

  // POST /rgb/issuers/import (binary body)
  rgbIssuersImport(
    name: string,
    archive: Uint8Array | ArrayBuffer,
    format: "auto" | "raw" | "gzip" | "zip" = "auto",
    options?: RequestOptions,
  ): Promise<RgbIssuersImportResponse> {
    if (!name) throw new Error("name is required");
    const q = `name=${encodeURIComponent(name)}&format=${encodeURIComponent(format)}`;
    return this.requestRaw<RgbIssuersImportResponse>(
      "POST",
      `/rgb/issuers/import?${q}`,
      archive as any,
      { ...options, headers: { "Content-Type": "application/octet-stream", ...(options?.headers ?? {}) } },
    ) as Promise<RgbIssuersImportResponse>;
  }

  // POST /rgb/contracts/import (binary body)
  rgbContractsImport(
    contractId: string,
    archive: Uint8Array | ArrayBuffer,
    format: "auto" | "raw" | "gzip" | "zip" = "auto",
    options?: RequestOptions,
  ): Promise<RgbContractsImportResponse> {
    if (!contractId) throw new Error("contractId is required");
    const q = `contract_id=${encodeURIComponent(contractId)}&format=${encodeURIComponent(format)}`;
    return this.requestRaw<RgbContractsImportResponse>(
      "POST",
      `/rgb/contracts/import?${q}`,
      archive as any,
      { ...options, headers: { "Content-Type": "application/octet-stream", ...(options?.headers ?? {}) } },
    ) as Promise<RgbContractsImportResponse>;
  }

  // POST /rgb/contracts/issue
  rgbContractsIssue(req: RgbContractsIssueRequest, options?: RequestOptions): Promise<RgbContractsIssueResponse> {
    return this.request<unknown>("POST", "/rgb/contracts/issue", req, options).then((v) =>
      decodeRgbContractsIssueResponse(v)
    ) as Promise<RgbContractsIssueResponse>;
  }

  // POST /rgb/contracts/export
  rgbContractsExport(req: RgbContractsExportRequest, options?: RequestOptions): Promise<RgbContractsExportResponse> {
    return this.request<RgbContractsExportResponse>("POST", "/rgb/contracts/export", req, options) as Promise<RgbContractsExportResponse>;
  }

  // GET /rgb/consignments/{consignmentKey} (binary response)
  rgbConsignmentDownload(
    consignmentKey: string,
    format: "raw" | "gzip" | "zip" = "raw",
    options?: RequestOptions,
  ): Promise<Uint8Array> {
    if (!consignmentKey) throw new Error("consignmentKey is required");
    const q = `format=${encodeURIComponent(format)}`;
    return this.requestBinary(
      "GET",
      `/rgb/consignments/${encodeURIComponent(consignmentKey)}?${q}`,
      options,
    );
  }

  // GET /rgb/contract/{contractId}/balance
  rgbContractBalance(contractId: string, options?: RequestOptions): Promise<RgbContractBalanceResponse> {
    if (!contractId) throw new Error("contractId is required");
    return this.request<RgbContractBalanceResponse>(
      "GET",
      `/rgb/contract/${encodeURIComponent(contractId)}/balance`,
      undefined,
      options,
    ).then((v) => decodeRgbContractBalanceResponse(v)) as Promise<RgbContractBalanceResponse>;
  }

  // ---- RGB Lightning ----

  // POST /rgb/ln/invoice/create
  rgbLnInvoiceCreate(req: RgbLnInvoiceCreateRequest, options?: RequestOptions): Promise<RgbLnInvoiceResponse> {
    return this.request<RgbLnInvoiceResponse>("POST", "/rgb/ln/invoice/create", req, options) as Promise<RgbLnInvoiceResponse>;
  }

  // POST /rgb/ln/invoice/decode
  rgbLnInvoiceDecode(req: RgbLnInvoiceDecodeRequest, options?: RequestOptions): Promise<RgbLnInvoiceDecodeResponse> {
    return this.request<unknown>("POST", "/rgb/ln/invoice/decode", req, options).then((v) =>
      decodeRgbLnInvoiceDecodeResponse(v)
    ) as Promise<RgbLnInvoiceDecodeResponse>;
  }

  // POST /rgb/ln/pay
  rgbLnPay(req: RgbLnPayRequest, options?: RequestOptions): Promise<SendResponse> {
    return this.request<SendResponse>("POST", "/rgb/ln/pay", req, options) as Promise<SendResponse>;
  }

  // ---- RGB on-chain ----

  // POST /rgb/onchain/invoice/create
  rgbOnchainInvoiceCreate(req: RgbOnchainInvoiceCreateRequest, options?: RequestOptions): Promise<RgbOnchainInvoiceResponse> {
    return this.request<RgbOnchainInvoiceResponse>("POST", "/rgb/onchain/invoice/create", req, options) as Promise<RgbOnchainInvoiceResponse>;
  }

  // POST /rgb/onchain/send
  rgbOnchainSend(req: RgbOnchainSendRequest, options?: RequestOptions): Promise<RgbOnchainSendResponse> {
    return this.request<RgbOnchainSendResponse>("POST", "/rgb/onchain/send", req, options) as Promise<RgbOnchainSendResponse>;
  }

  // POST /rgb/onchain/receive
  rgbOnchainReceive(req: RgbOnchainReceiveRequest, options?: RequestOptions): Promise<RgbOnchainReceiveResponse> {
    return this.request<unknown>("POST", "/rgb/onchain/receive", req, options).then((v) =>
      decodeRgbOnchainReceiveResponse(v)
    ) as Promise<RgbOnchainReceiveResponse>;
  }

  // GET /balances
  balances(options?: RequestOptions): Promise<BalancesDto> {
    return this.request<unknown>("GET", "/balances", undefined, options).then((v) => decodeBalancesDto(v)) as Promise<
      BalancesDto
    >;
  }

  // GET /peers
  peers(options?: RequestOptions): Promise<PeerDetailsDto[]> {
    return this.request<PeerDetailsDto[]>("GET", "/peers", undefined, options) as Promise<PeerDetailsDto[]>;
  }

  // POST /peers/connect
  peersConnect(req: PeerConnectRequest, options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/peers/connect", req, options) as Promise<OkResponse>;
  }

  // POST /peers/disconnect
  peersDisconnect(req: PeerDisconnectRequest, options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/peers/disconnect", req, options) as Promise<OkResponse>;
  }

  // GET /channels
  channels(options?: RequestOptions): Promise<ChannelDetailsExtendedDto[]> {
    const decode = decodeArray(decodeChannelDetailsExtendedDto);
    return this.request<unknown>("GET", "/channels", undefined, options).then((v) => decode(v)) as Promise<
      ChannelDetailsExtendedDto[]
    >;
  }

  // POST /channel/open
  channelOpen(req: OpenChannelRequest, options?: RequestOptions): Promise<OpenChannelResponse> {
    return this.request<OpenChannelResponse>("POST", "/channel/open", req, options) as Promise<OpenChannelResponse>;
  }

  // POST /channel/close
  channelClose(req: CloseChannelRequest, options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/channel/close", req, options) as Promise<OkResponse>;
  }

  // POST /channel/force_close
  channelForceClose(req: CloseChannelRequest, options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/channel/force_close", req, options) as Promise<OkResponse>;
  }

  // POST /bolt11/receive
  bolt11Receive(req: Bolt11ReceiveRequest, options?: RequestOptions): Promise<Bolt11ReceiveResponse> {
    return this.request<Bolt11ReceiveResponse>("POST", "/bolt11/receive", req, options) as Promise<Bolt11ReceiveResponse>;
  }

  // POST /bolt11/receive_var
  bolt11ReceiveVar(req: Bolt11ReceiveVarRequest, options?: RequestOptions): Promise<Bolt11ReceiveResponse> {
    return this.request<Bolt11ReceiveResponse>("POST", "/bolt11/receive_var", req, options) as Promise<Bolt11ReceiveResponse>;
  }

  // POST /bolt11/decode
  bolt11Decode(req: Bolt11DecodeRequest, options?: RequestOptions): Promise<Bolt11DecodeResponse> {
    return this.request<unknown>("POST", "/bolt11/decode", req, options).then((v) => decodeBolt11DecodeResponse(v)) as Promise<
      Bolt11DecodeResponse
    >;
  }

  // POST /bolt11/send
  bolt11Send(req: Bolt11SendRequest, options?: RequestOptions): Promise<SendResponse> {
    return this.request<SendResponse>("POST", "/bolt11/send", req, options) as Promise<SendResponse>;
  }

  // POST /bolt11/send_using_amount
  bolt11SendUsingAmount(req: Bolt11SendUsingAmountRequest, options?: RequestOptions): Promise<SendResponse> {
    return this.request<SendResponse>("POST", "/bolt11/send_using_amount", req, options) as Promise<SendResponse>;
  }

  // POST /bolt11/pay (waits for completion)
  bolt11Pay(req: Bolt11PayRequest, options?: RequestOptions): Promise<Bolt11PayResponse> {
    return this.request<unknown>("POST", "/bolt11/pay", req, options).then((v) => decodeBolt11PayResponse(v)) as Promise<
      Bolt11PayResponse
    >;
  }

  // POST /spontaneous/send
  spontaneousSend(req: SpontaneousSendRequest, options?: RequestOptions): Promise<SendResponse> {
    // The Rust side uses r#type identifier but JSON key is "type".
    const body = { ...req, custom_tlvs: req.custom_tlvs ?? [] };
    return this.request<SendResponse>("POST", "/spontaneous/send", body, options) as Promise<SendResponse>;
  }

  // GET /payment/{paymentId}
  getPayment(paymentIdHex: string, options?: RequestOptions): Promise<PaymentDetailsDto | null> {
    if (!paymentIdHex) throw new Error("paymentIdHex is required");
    return this.request<unknown>(
      "GET",
      `/payment/${encodeURIComponent(paymentIdHex)}`,
      undefined,
      { ...options, returnNullOn404: true },
    ).then((v) => (v ? decodePaymentDetailsDto(v) : null)) as Promise<PaymentDetailsDto | null>;
  }

  // GET /payments
  payments(options?: RequestOptions): Promise<PaymentDetailsDto[]> {
    const decode = decodeArray(decodePaymentDetailsDto);
    return this.request<unknown>("GET", "/payments", undefined, options).then((v) => decode(v)) as Promise<
      PaymentDetailsDto[]
    >;
  }

  // POST /payment/{paymentId}/wait
  paymentWait(paymentIdHex: string, req: PaymentWaitRequest = {}, options?: RequestOptions): Promise<PaymentWaitResponse> {
    if (!paymentIdHex) throw new Error("paymentIdHex is required");
    return this.request<unknown>(
      "POST",
      `/payment/${encodeURIComponent(paymentIdHex)}/wait`,
      req,
      options,
    ).then((v) => decodePaymentWaitResponse(v)) as Promise<PaymentWaitResponse>;
  }

  // POST /payment/{paymentId}/abandon
  paymentAbandon(paymentIdHex: string, options?: RequestOptions): Promise<OkResponse> {
    if (!paymentIdHex) throw new Error("paymentIdHex is required");
    return this.request<OkResponse>(
      "POST",
      `/payment/${encodeURIComponent(paymentIdHex)}/abandon`,
      {},
      options,
    ) as Promise<OkResponse>;
  }

  // ---- BOLT12 offers ----

  // POST /bolt12/offer/receive
  bolt12OfferReceive(req: Bolt12OfferReceiveRequest, options?: RequestOptions): Promise<Bolt12OfferResponse> {
    return this.request<Bolt12OfferResponse>("POST", "/bolt12/offer/receive", req, options) as Promise<
      Bolt12OfferResponse
    >;
  }

  // POST /bolt12/offer/receive_var
  bolt12OfferReceiveVar(req: Bolt12OfferReceiveVarRequest, options?: RequestOptions): Promise<Bolt12OfferResponse> {
    return this.request<Bolt12OfferResponse>(
      "POST",
      "/bolt12/offer/receive_var",
      req,
      options,
    ) as Promise<Bolt12OfferResponse>;
  }

  // POST /bolt12/offer/decode
  bolt12OfferDecode(req: Bolt12OfferDecodeRequest, options?: RequestOptions): Promise<Bolt12OfferDecodeResponse> {
    return this.request<unknown>("POST", "/bolt12/offer/decode", req, options).then((v) =>
      decodeBolt12OfferDecodeResponse(v)
    ) as Promise<Bolt12OfferDecodeResponse>;
  }

  // POST /bolt12/offer/send
  bolt12OfferSend(req: Bolt12OfferSendRequest, options?: RequestOptions): Promise<SendResponse> {
    return this.request<SendResponse>("POST", "/bolt12/offer/send", req, options) as Promise<SendResponse>;
  }

  // ---- BOLT12 refunds ----

  // POST /bolt12/refund/initiate
  bolt12RefundInitiate(req: Bolt12RefundInitiateRequest, options?: RequestOptions): Promise<Bolt12RefundInitiateResponse> {
    return this.request<Bolt12RefundInitiateResponse>(
      "POST",
      "/bolt12/refund/initiate",
      req,
      options,
    ) as Promise<Bolt12RefundInitiateResponse>;
  }

  // POST /bolt12/refund/decode
  bolt12RefundDecode(req: Bolt12RefundDecodeRequest, options?: RequestOptions): Promise<Bolt12RefundDecodeResponse> {
    return this.request<unknown>("POST", "/bolt12/refund/decode", req, options).then((v) =>
      decodeBolt12RefundDecodeResponse(v)
    ) as Promise<Bolt12RefundDecodeResponse>;
  }

  // POST /bolt12/refund/request_payment
  bolt12RefundRequestPayment(
    req: Bolt12RefundRequestPaymentRequest,
    options?: RequestOptions,
  ): Promise<Bolt12RefundRequestPaymentResponse> {
    return this.request<Bolt12RefundRequestPaymentResponse>(
      "POST",
      "/bolt12/refund/request_payment",
      req,
      options,
    ) as Promise<Bolt12RefundRequestPaymentResponse>;
  }

  // POST /events/wait_next (long-poll)
  eventsWaitNext(options?: RequestOptions): Promise<EventDto> {
    return this.request<unknown>("POST", "/events/wait_next", {}, options).then((v) => decodeEventDto(v)) as Promise<
      EventDto
    >;
  }

  // POST /events/handled
  eventsHandled(options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/events/handled", {}, options) as Promise<OkResponse>;
  }
}

function safeJsonParse(text: string): unknown {
  try {
    return parse(text, normalizeBigIntReviver, parseNumberAndBigInt);
  } catch {
    return undefined;
  }
}

function jsonBigIntReplacer(_key: string, value: unknown): unknown {
  if (typeof value === "bigint") {
    return value.toString(10);
  }
  return value;
}

function normalizeBigIntReviver(_key: string, value: unknown): unknown {
  if (typeof value === "bigint") {
    const max = BigInt(Number.MAX_SAFE_INTEGER);
    const min = BigInt(Number.MIN_SAFE_INTEGER);
    if (value <= max && value >= min) return Number(value);
  }

  return value;
}
