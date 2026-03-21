import { spawn } from "node:child_process";

import type { JsonValue, NmhRequest, NmhResponse, NmhTransport } from "./nmh.js";

type Pending = {
  resolve: (v: NmhResponse<JsonValue>) => void;
  reject: (e: unknown) => void;
  timer?: ReturnType<typeof setTimeout>;
};

export function createNodeNativeMessagingTransport(
  command: string,
  args: string[] = [],
  options?: { cwd?: string; env?: Record<string, string>; maxMsgBytes?: number },
): NmhTransport {
  if (!command) throw new Error("command is required");
  const maxMsgBytes = options?.maxMsgBytes ?? 1024 * 1024;

  const child = spawn(command, args, {
    stdio: ["pipe", "pipe", "pipe"],
    cwd: options?.cwd,
    env: { ...process.env, ...(options?.env ?? {}) },
  });

  if (!child.stdin || !child.stdout) {
    throw new Error("failed to spawn native messaging host");
  }

  let nextId = 1;
  const pending = new Map<string, Pending>();
  let buf = Buffer.alloc(0);
  let closed = false;

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

  child.on("exit", (code: number | null, signal: NodeJS.Signals | null) => {
    closed = true;
    failAll(
      new Error(
        `native messaging host exited (code=${code ?? "null"}, signal=${signal ?? "null"})`,
      ),
    );
  });
  child.on("error", (e: unknown) => {
    closed = true;
    failAll(new Error(`native messaging host error: ${String(e)}`));
  });

  child.stdout.on("data", (chunk: Buffer) => {
    buf = Buffer.concat([buf, chunk]);
    while (buf.length >= 4) {
      const len = buf.readUInt32LE(0);
      if (len === 0) {
        closed = true;
        failAll(new Error("invalid native message length=0"));
        return;
      }
      if (len > maxMsgBytes) {
        closed = true;
        failAll(new Error(`native message too large: ${len} > ${maxMsgBytes}`));
        return;
      }
      if (buf.length < 4 + len) return;
      const payload = buf.subarray(4, 4 + len);
      buf = buf.subarray(4 + len);
      let msg: unknown;
      try {
        msg = JSON.parse(payload.toString("utf8"));
      } catch (e) {
        closed = true;
        failAll(new Error(`invalid host json: ${String(e)}`));
        return;
      }
      const id = (msg as any)?.id;
      if (id === undefined) continue;
      const k = keyOf(id);
      const p = pending.get(k);
      if (!p) continue;
      pending.delete(k);
      if (p.timer) clearTimeout(p.timer);
      p.resolve(msg as NmhResponse<JsonValue>);
    }
  });

  child.stderr?.on("data", (_chunk: Buffer) => {});

  function writeMessage(obj: unknown) {
    const payload = Buffer.from(JSON.stringify(obj), "utf8");
    const lenBuf = Buffer.alloc(4);
    lenBuf.writeUInt32LE(payload.length, 0);
    child.stdin.write(lenBuf);
    child.stdin.write(payload);
  }

  return {
    request(req: NmhRequest, opts?: { timeoutMs?: number }): Promise<NmhResponse<JsonValue>> {
      if (closed) throw new Error("native messaging transport closed");
      const id = req.id ?? nextId++;
      const k = keyOf(id);
      if (pending.has(k)) throw new Error("duplicate request id");

      return new Promise((resolve, reject) => {
        const timeoutMs = opts?.timeoutMs;
        const timer = timeoutMs && timeoutMs > 0
          ? setTimeout(() => {
            pending.delete(k);
            reject(new Error("native messaging request timeout"));
          }, timeoutMs)
          : undefined;
        pending.set(k, { resolve, reject, timer });
        writeMessage({ id, method: req.method, params: req.params ?? {} });
      });
    },
    close(): void {
      if (closed) return;
      closed = true;
      try {
        child.kill();
      } finally {
        failAll(new Error("native messaging transport closed"));
      }
    },
  };
}
