export type JsonValue =
  | null
  | boolean
  | number
  | string
  | JsonValue[]
  | { [key: string]: JsonValue };

export interface NmhRequest {
  id?: JsonValue;
  method: string;
  params?: JsonValue;
}

export interface NmhResponse<T = JsonValue> {
  ok: boolean;
  id?: JsonValue;
  result?: T;
  error?: string;
  hint?: string;
}

export interface NmhTransport {
  request(req: NmhRequest, options?: { timeoutMs?: number }): Promise<NmhResponse<JsonValue>>;
  close(): void;
}

export class NmhError extends Error {
  public readonly hint?: string;
  constructor(message: string, hint?: string) {
    super(message);
    this.hint = hint;
  }
}

export class NmhClient {
  private readonly transport: NmhTransport;
  constructor(transport: NmhTransport) {
    this.transport = transport;
  }

  close(): void {
    this.transport.close();
  }

  async version(options?: { timeoutMs?: number }): Promise<{ host: string; protocol: string }> {
    return await this.call<{ host: string; protocol: string }>("version", {}, options);
  }

  async status(options?: { timeoutMs?: number }): Promise<any> {
    return await this.call<any>("status", {}, options);
  }

  async unlock(passphrase: string, options?: { timeoutMs?: number }): Promise<any> {
    if (!passphrase) throw new Error("passphrase is required");
    return await this.call<any>("unlock", { passphrase }, options);
  }

  async lock(options?: { timeoutMs?: number }): Promise<any> {
    return await this.call<any>("lock", {}, options);
  }

  private async call<T>(method: string, params: JsonValue, options?: { timeoutMs?: number }): Promise<T> {
    const resp = await this.transport.request({ method, params }, options);
    if (!resp.ok) {
      throw new NmhError(resp.error || "native messaging host error", resp.hint);
    }
    return resp.result as T;
  }
}

type Listener<T> = (msg: T) => void;

interface ChromePort {
  postMessage(msg: any): void;
  disconnect(): void;
  onMessage: { addListener(fn: Listener<any>): void; removeListener(fn: Listener<any>): void };
  onDisconnect: { addListener(fn: Listener<any>): void; removeListener(fn: Listener<any>): void };
  error?: any;
}

interface ChromeRuntimeLike {
  connectNative(hostName: string): ChromePort;
  lastError?: { message?: string };
}

export function createChromeNativeMessagingTransport(hostName: string, runtime?: ChromeRuntimeLike): NmhTransport {
  if (!hostName) throw new Error("hostName is required");
  const rt = runtime ?? (globalThis as any)?.chrome?.runtime;
  if (!rt?.connectNative) {
    throw new Error("chrome.runtime.connectNative is not available");
  }

  const port: ChromePort = rt.connectNative(hostName);
  let nextId = 1;
  const pending = new Map<string, { resolve: (v: any) => void; reject: (e: any) => void; timer?: any }>();

  function keyOf(id: JsonValue): string {
    return typeof id === "string" ? id : JSON.stringify(id);
  }

  function failAll(err: Error) {
    for (const [, p] of pending) {
      if (p.timer) clearTimeout(p.timer);
      p.reject(err);
    }
    pending.clear();
  }

  const onMessage = (msg: NmhResponse) => {
    const id = msg?.id;
    if (id === undefined) return;
    const k = keyOf(id);
    const p = pending.get(k);
    if (!p) return;
    pending.delete(k);
    if (p.timer) clearTimeout(p.timer);
    p.resolve(msg);
  };

  const onDisconnect = () => {
    const msg = rt.lastError?.message || "native messaging host disconnected";
    failAll(new Error(msg));
  };

  port.onMessage.addListener(onMessage);
  port.onDisconnect.addListener(onDisconnect);

  return {
    request(req: NmhRequest, options?: { timeoutMs?: number }): Promise<NmhResponse<JsonValue>> {
      const id = req.id ?? nextId++;
      const k = keyOf(id);
      if (pending.has(k)) throw new Error("duplicate request id");

      return new Promise((resolve, reject) => {
        const timeoutMs = options?.timeoutMs;
        const timer = timeoutMs && timeoutMs > 0
          ? setTimeout(() => {
            pending.delete(k);
            reject(new Error("native messaging request timeout"));
          }, timeoutMs)
          : undefined;
        pending.set(k, { resolve, reject, timer });
        port.postMessage({ id, method: req.method, params: req.params ?? {} });
      });
    },
    close(): void {
      port.onMessage.removeListener(onMessage);
      port.onDisconnect.removeListener(onDisconnect);
      try {
        port.disconnect();
      } finally {
        failAll(new Error("native messaging transport closed"));
      }
    },
  };
}
