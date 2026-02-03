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
  ChannelDetailsExtendedDto,
  CloseChannelRequest,
  EventDto,
  ListeningAddressesResponse,
  NodeIdResponse,
  OkResponse,
  OpenChannelRequest,
  OpenChannelResponse,
  PaymentDetailsDto,
  PeerConnectRequest,
  PeerDetailsDto,
  PeerDisconnectRequest,
  SendResponse,
  SpontaneousSendRequest,
  StatusDto,
} from "./types.js";
import { parse, parseNumberAndBigInt } from "lossless-json";

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

  // GET /status
  status(options?: RequestOptions): Promise<StatusDto> {
    return this.request<StatusDto>("GET", "/status", undefined, options) as Promise<StatusDto>;
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

  // GET /balances
  balances(options?: RequestOptions): Promise<BalancesDto> {
    return this.request<BalancesDto>("GET", "/balances", undefined, options) as Promise<BalancesDto>;
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
    return this.request<ChannelDetailsExtendedDto[]>("GET", "/channels", undefined, options) as Promise<
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
    return this.request<Bolt11DecodeResponse>("POST", "/bolt11/decode", req, options) as Promise<Bolt11DecodeResponse>;
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
    return this.request<Bolt11PayResponse>("POST", "/bolt11/pay", req, options) as Promise<Bolt11PayResponse>;
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
    return this.request<PaymentDetailsDto>(
      "GET",
      `/payment/${encodeURIComponent(paymentIdHex)}`,
      undefined,
      { ...options, returnNullOn404: true },
    ) as Promise<PaymentDetailsDto | null>;
  }

  // POST /events/wait_next (long-poll)
  eventsWaitNext(options?: RequestOptions): Promise<EventDto> {
    return this.request<EventDto>("POST", "/events/wait_next", {}, options) as Promise<EventDto>;
  }

  // POST /events/handled
  eventsHandled(options?: RequestOptions): Promise<OkResponse> {
    return this.request<OkResponse>("POST", "/events/handled", {}, options) as Promise<OkResponse>;
  }
}

function safeJsonParse(text: string): unknown {
  try {
    const value = parse(text, null, parseNumberAndBigInt);
    return normalizeBigInts(value);
  } catch {
    return undefined;
  }
}

function jsonBigIntReplacer(_key: string, value: unknown): unknown {
  if (typeof value === "bigint") {
    const max = BigInt(Number.MAX_SAFE_INTEGER);
    if (value <= max) return Number(value);
    throw new Error("Request body contains a bigint larger than Number.MAX_SAFE_INTEGER");
  }
  return value;
}

function normalizeBigInts(value: unknown): unknown {
  if (typeof value === "bigint") {
    const max = BigInt(Number.MAX_SAFE_INTEGER);
    const min = BigInt(Number.MIN_SAFE_INTEGER);
    if (value <= max && value >= min) return Number(value);
    return value;
  }

  if (Array.isArray(value)) {
    return value.map(normalizeBigInts);
  }

  if (value && typeof value === "object") {
    const obj = value as Record<string, unknown>;
    for (const key of Object.keys(obj)) {
      obj[key] = normalizeBigInts(obj[key]);
    }
    return obj;
  }

  return value;
}
