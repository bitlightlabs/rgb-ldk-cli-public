import { stdin, stdout } from "node:process";

let buf = Buffer.alloc(0);

function writeMsg(obj) {
  const payload = Buffer.from(JSON.stringify(obj), "utf8");
  const lenBuf = Buffer.alloc(4);
  lenBuf.writeUInt32LE(payload.length, 0);
  stdout.write(lenBuf);
  stdout.write(payload);
}

stdin.on("data", (chunk) => {
  buf = Buffer.concat([buf, chunk]);
  if (buf.length < 4) return;
  const len = buf.readUInt32LE(0);
  if (buf.length < 4 + len) return;
  const payload = buf.subarray(4, 4 + len);
  const req = JSON.parse(payload.toString("utf8"));
  const { id, method, params } = req;

  if (method === "version") {
    writeMsg({ ok: true, id, result: { host: "fake-nmh", protocol: "native-messaging/v1" } });
    process.exit(0);
  }
  if (method === "unlock") {
    if ((params?.passphrase || "") === "correct") {
      writeMsg({ ok: true, id, result: { ok: true, locked: false, running: true } });
    } else {
      writeMsg({ ok: false, id, error: "bad passphrase" });
    }
    process.exit(0);
  }
  if (method === "status") {
    writeMsg({ ok: true, id, result: { ok: true, locked: true, running: false } });
    process.exit(0);
  }

  writeMsg({ ok: false, id, error: "unknown method" });
  process.exit(0);
});
