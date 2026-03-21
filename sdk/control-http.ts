import { HttpError, type FetchLike, type RequestOptions } from "./client.js";
import { parse, parseNumberAndBigInt } from "lossless-json";

export interface ControlHttpClientOptions {
  fetch?: FetchLike;
  headers?: Record<string, string>;
}

export interface ControlStatusResponse {
  ok: boolean;
  locked: boolean;
  running: boolean;
}

export interface ControlUnlockResponse {
  ok: boolean;
  locked: boolean;
  running: boolean;
}

export interface ControlLockResponse {
  ok: boolean;
  locked: boolean;
  running: boolean;
}

export interface ControlVersionResponse {
  ok: boolean;
  protocol: string;
  daemon: string;
  daemon_version: string;
}

export class ControlHttpClient {
  private readonly baseUrl: string;
  private readonly fetchFn: FetchLike;
  private readonly token: string;
  private readonly defaultHeaders: Record<string, string>;

  constructor(baseUrl: string, token: string, opts?: ControlHttpClientOptions) {
    if (!baseUrl) throw new Error("baseUrl is required");
    if (!token) throw new Error("token is required");
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.token = token;
    const globalFetch: any = (globalThis as any).fetch;
    this.fetchFn = opts?.fetch ?? (globalFetch?.bind(globalThis) as FetchLike);
    if (!this.fetchFn) {
      throw new Error("No fetch implementation found. Provide opts.fetch or a global fetch");
    }
    this.defaultHeaders = { "Content-Type": "application/json", ...(opts?.headers ?? {}) };
  }

  // GET /control/status
  status(options?: RequestOptions): Promise<ControlStatusResponse> {
    return this.request<ControlStatusResponse>("GET", "/control/status", undefined, options) as Promise<ControlStatusResponse>;
  }

  // POST /control/unlock
  unlock(passphrase: string, options?: RequestOptions): Promise<ControlUnlockResponse> {
    if (!passphrase) throw new Error("passphrase is required");
    return this.request<ControlUnlockResponse>("POST", "/control/unlock", { passphrase }, options) as Promise<ControlUnlockResponse>;
  }

  // POST /control/unlock (server-managed passphrase file)
  unlockUsingServerSecret(options?: RequestOptions): Promise<ControlUnlockResponse> {
    return this.request<ControlUnlockResponse>("POST", "/control/unlock", {}, options) as Promise<ControlUnlockResponse>;
  }

  // POST /control/lock
  lock(options?: RequestOptions): Promise<ControlLockResponse> {
    return this.request<ControlLockResponse>("POST", "/control/lock", { yes: true }, options) as Promise<ControlLockResponse>;
  }

  // GET /control/version
  version(options?: RequestOptions): Promise<ControlVersionResponse> {
    return this.request<ControlVersionResponse>("GET", "/control/version", undefined, options) as Promise<ControlVersionResponse>;
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const headers = {
      ...this.defaultHeaders,
      ...(options?.headers ?? {}),
      Authorization: `Bearer ${this.token}`,
    };
    const controller = new AbortController();
    const timeout = options?.timeoutMs && options.timeoutMs > 0 ? setTimeout(() => controller.abort(), options.timeoutMs) : undefined;
    if (options?.signal) {
      if (options.signal.aborted) controller.abort();
      else options.signal.addEventListener("abort", () => controller.abort());
    }

    try {
      const resp = await this.fetchFn(url, {
        method,
        headers,
        body: body !== undefined ? JSON.stringify(body) : undefined,
        signal: controller.signal,
      });
      const text = await resp.text();
      const json = text ? safeJsonParse(text) : undefined;
      if (!resp.ok) {
        const msg = (json as any)?.error || `HTTP ${resp.status}`;
        throw new HttpError(msg, resp.status, json ?? text);
      }
      return (json as T) ?? ({} as T);
    } catch (e: any) {
      if (e?.name === "AbortError") throw new Error("Request aborted");
      throw e;
    } finally {
      if (timeout) clearTimeout(timeout);
    }
  }
}

function safeJsonParse(text: string): unknown {
  try {
    return parse(text, normalizeBigIntReviver, parseNumberAndBigInt);
  } catch {
    return undefined;
  }
}

function normalizeBigIntReviver(_key: string, value: unknown): unknown {
  if (typeof value === "bigint") {
    const max = BigInt(Number.MAX_SAFE_INTEGER);
    const min = BigInt(Number.MIN_SAFE_INTEGER);
    if (value <= max && value >= min) return Number(value);
  }
  return value;
}
