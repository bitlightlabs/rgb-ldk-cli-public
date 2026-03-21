use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::path::Path;

use clap::Parser;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::runtime::Runtime;
use zeroize::Zeroize;

mod control;

const DEFAULT_MAX_MSG_BYTES: usize = 1024 * 1024;
const NMH_PROTOCOL: &str = "native-messaging/v1";

#[derive(Debug, Parser)]
#[command(
	name = "rgbldk-nmh",
	version,
	about = "Native Messaging Host for rgbldk (stdin/stdout JSON)"
)]
struct Args {
	/// Override control socket path (default: <data_dir>/control.sock).
	#[arg(long)]
	control_socket: Option<String>,

	/// Node data dir (default: /tmp/ldk_node).
	#[arg(long)]
	data_dir: Option<String>,

	/// Max inbound message size.
	#[arg(long, default_value_t = DEFAULT_MAX_MSG_BYTES)]
	max_msg_bytes: usize,

	/// Only allow these methods (repeatable). If unset, all supported methods are allowed.
	#[arg(long = "allow-method")]
	allow_methods: Vec<String>,

	/// Require a local tty confirmation for sensitive methods (unlock, lock).
	#[arg(long)]
	require_user_presence: bool,

	/// Log each request to stderr (off, text, json).
	#[arg(long, default_value = "off")]
	log: LogMode,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum LogMode {
	Off,
	Text,
	Json,
}

#[derive(Debug, Deserialize)]
struct HostRequest {
	#[serde(default)]
	id: Option<Value>,
	method: String,
	#[serde(default)]
	params: Value,
}

fn main() -> Result<(), String> {
	let args = Args::parse();
	let socket_path = control::default_control_socket_path(
		args.control_socket.as_deref(),
		args.data_dir.as_deref(),
	);
	let allow_methods = build_allowlist(&args.allow_methods);
	let rt = Runtime::new().map_err(|e| format!("tokio runtime init failed: {e}"))?;

	let stdin = io::stdin();
	let mut input = stdin.lock();
	let stdout = io::stdout();
	let mut output = stdout.lock();

	while let Some(bytes) = read_native_message(&mut input, args.max_msg_bytes)
		.map_err(|e| format!("read message failed: {e}"))?
	{
		let parsed: Result<HostRequest, _> = serde_json::from_slice(&bytes);
		let (method, id, resp) = match parsed {
			Ok(req) => {
				let method = req.method.clone();
				let id = req.id.clone();
				let resp = handle_request(
					&rt,
					socket_path.as_path(),
					req,
					&allow_methods,
					args.require_user_presence,
				);
				(method, id, resp)
			},
			Err(e) => ("<invalid>".to_string(), None, err(format!("invalid request: {e}"), None)),
		};

		log_request(
			args.log,
			&method,
			resp.get("ok").and_then(|v| v.as_bool()),
			resp.get("error"),
			&resp,
		);

		let mut out = resp;
		if let Some(id) = id {
			out["id"] = id;
		}
		let out_bytes = serde_json::to_vec(&out).map_err(|e| format!("serialize response: {e}"))?;
		write_native_message(&mut output, &out_bytes)
			.map_err(|e| format!("write response failed: {e}"))?;
	}

	Ok(())
}

fn handle_request(
	rt: &Runtime, socket_path: &Path, req: HostRequest, allow_methods: &AllowMethods,
	require_user_presence: bool,
) -> Value {
	let method = req.method.as_str();
	if !allow_methods.is_allowed(method) {
		return err(
			format!("method not allowed: {method}"),
			Some("This host was started with an allowlist."),
		);
	}

	match method {
		"status" | "control.status" => rt
			.block_on(control::call_control(socket_path, "status", json!({})))
			.map(ok)
			.unwrap_or_else(|e| err(e, None)),
		"unlock" | "control.unlock" => {
			let pass = req.params.get("passphrase").and_then(|v| v.as_str());
			let Some(passphrase) = pass else {
				return err(
					"missing params.passphrase".to_string(),
					Some("Provide params.passphrase"),
				);
			};
			if require_user_presence {
				if let Err(e) = confirm_on_tty("Allow unlock? Type yes to continue: ") {
					return err(e, Some("Disable --require-user-presence or run with a tty"));
				}
			};
			let mut passphrase_owned = passphrase.to_string();
			let params = json!({ "passphrase": passphrase_owned.clone() });
			let resp = rt.block_on(control::call_control(socket_path, "unlock", params));
			passphrase_owned.zeroize();
			match resp {
				Ok(v) => map_control_result(v),
				Err(e) => err(e, None),
			}
		},
		"lock" | "control.lock" => {
			if require_user_presence {
				if let Err(e) = confirm_on_tty("Allow lock? Type yes to continue: ") {
					return err(e, Some("Disable --require-user-presence or run with a tty"));
				}
			}
			rt.block_on(control::call_control(socket_path, "lock", json!({ "yes": true })))
				.map(map_control_result)
				.unwrap_or_else(|e| err(e, None))
		},
		"version" => ok(json!({
			"host": "rgbldk-nmh",
			"protocol": NMH_PROTOCOL,
			"host_version": env!("CARGO_PKG_VERSION"),
		})),
		_ => err("unknown method".to_string(), Some("Try: status, unlock, lock, version")),
	}
}

fn ok(result: Value) -> Value {
	json!({ "ok": true, "result": result })
}

fn err(error: String, hint: Option<&str>) -> Value {
	let mut v = json!({ "ok": false, "error": error });
	if let Some(h) = hint {
		v["hint"] = json!(h);
	}
	v
}

fn map_control_result(v: Value) -> Value {
	let ok_control = v.get("ok").and_then(|b| b.as_bool()).unwrap_or(true);
	if ok_control {
		ok(v)
	} else {
		let error =
			v.get("error").and_then(|e| e.as_str()).unwrap_or("control method failed").to_string();
		let hint = v.get("hint").and_then(|h| h.as_str());
		err(error, hint)
	}
}

struct AllowMethods {
	enabled: Option<HashSet<String>>,
}

impl AllowMethods {
	fn is_allowed(&self, method: &str) -> bool {
		let Some(set) = &self.enabled else {
			return true;
		};
		set.contains(method)
	}
}

fn build_allowlist(allow_methods: &[String]) -> AllowMethods {
	if allow_methods.is_empty() {
		return AllowMethods { enabled: None };
	}
	let mut set = HashSet::new();
	for m in allow_methods {
		set.insert(m.to_string());
	}
	AllowMethods { enabled: Some(set) }
}

fn confirm_on_tty(prompt: &str) -> Result<(), String> {
	#[cfg(unix)]
	{
		use std::fs::OpenOptions;
		use std::io::{BufRead, BufReader};
		let tty = OpenOptions::new()
			.read(true)
			.write(true)
			.open("/dev/tty")
			.map_err(|e| format!("open /dev/tty failed: {e}"))?;
		let mut reader = BufReader::new(tty);
		let mut line = String::new();
		eprint!("{prompt}");
		let _ = io::stderr().flush();
		reader.read_line(&mut line).map_err(|e| format!("read confirmation failed: {e}"))?;
		if line.trim() == "yes" {
			Ok(())
		} else {
			Err("user presence not confirmed".to_string())
		}
	}
	#[cfg(not(unix))]
	{
		let _ = prompt;
		Err("user presence confirmation not supported on this platform".to_string())
	}
}

fn log_request(mode: LogMode, method: &str, ok: Option<bool>, error: Option<&Value>, resp: &Value) {
	match mode {
		LogMode::Off => {},
		LogMode::Text => {
			if ok.unwrap_or(false) {
				eprintln!("method={method} ok=true");
			} else {
				let e = error.and_then(|v| v.as_str()).unwrap_or("error");
				eprintln!("method={method} ok=false error={e}");
			}
		},
		LogMode::Json => {
			let e = error.and_then(|v| v.as_str());
			let line = json!({
				"method": method,
				"ok": ok,
				"error": e,
				"has_result": resp.get("result").is_some(),
			});
			eprintln!("{line}");
		},
	}
}

fn read_native_message<R: Read>(r: &mut R, max_bytes: usize) -> io::Result<Option<Vec<u8>>> {
	let mut len_buf = [0u8; 4];
	match r.read_exact(&mut len_buf) {
		Ok(()) => {},
		Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
		Err(e) => return Err(e),
	}
	let len = u32::from_le_bytes(len_buf) as usize;
	if len == 0 {
		return Err(io::Error::new(io::ErrorKind::InvalidData, "empty message"));
	}
	if len > max_bytes {
		return Err(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("message too large: {len} > {max_bytes}"),
		));
	}
	let mut buf = vec![0u8; len];
	r.read_exact(&mut buf)?;
	Ok(Some(buf))
}

fn write_native_message<W: Write>(w: &mut W, payload: &[u8]) -> io::Result<()> {
	let len: u32 = payload
		.len()
		.try_into()
		.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "payload too large"))?;
	w.write_all(&len.to_le_bytes())?;
	w.write_all(payload)?;
	w.flush()?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Cursor;

	#[test]
	fn framing_roundtrip() {
		let msg = br#"{"id":1,"method":"version","params":{}}"#;
		let mut out = Vec::new();
		write_native_message(&mut out, msg).unwrap();

		let mut cur = Cursor::new(out);
		let read = read_native_message(&mut cur, 1024).unwrap().unwrap();
		assert_eq!(read, msg);
	}

	#[test]
	fn framing_rejects_too_large() {
		let payload = vec![b'a'; 10];
		let mut out = Vec::new();
		write_native_message(&mut out, &payload).unwrap();

		let mut cur = Cursor::new(out);
		let err = read_native_message(&mut cur, 5).unwrap_err();
		assert_eq!(err.kind(), io::ErrorKind::InvalidData);
	}

	#[test]
	fn allowlist_blocks_unknown_method() {
		let allow = build_allowlist(&["version".to_string()]);
		assert!(allow.is_allowed("version"));
		assert!(!allow.is_allowed("unlock"));
	}
}
