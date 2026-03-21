use std::collections::BTreeSet;
use std::io::Read;

use amplify::confinement::{SmallBlob, SmallOrdMap, TinyVec};
use base64::Engine;
use bp::seals::{Noise, TxoSeal, TxoSealExt, WOutpoint, WTxoSeal};
use commit_verify::ReservedBytes;
use flate2::read::GzDecoder;
use rgb::invoice::{RgbBeneficiary, RgbInvoice};
use rgb::{Issue, OperationSeals, RgbSealDef, Semantics, SigBlob, MAX_CONSIGNMENT_OPS};
use serde_json::json;
use strict_encoding::{DecodeError, StreamReader, StrictDecode, StrictReader, TypedRead};
use zip::ZipArchive;

use crate::app::App;
use crate::cli::{DebugCommand, DebugConsignmentArgs, DebugConsignmentFormat, DebugInvoiceArgs};
use crate::ui;
use crate::utils::{die, print_json, read_file_trimmed, read_stdin_trimmed};

struct ConsignmentHeader {
	genesis_seals: SmallOrdMap<u16, WTxoSeal>,
	op_count: u32,
}

impl ConsignmentHeader {
	fn decode(reader: &mut impl TypedRead) -> Result<Self, DecodeError> {
		let _extensions = TinyVec::<SmallBlob>::strict_decode(reader)?;
		let _semantics = Semantics::strict_decode(reader)?;
		let _sig = Option::<SigBlob>::strict_decode(reader)?;
		let _issue = Issue::strict_decode(reader)?;
		let genesis_seals = SmallOrdMap::<u16, WTxoSeal>::strict_decode(reader)?;
		let _witness = ReservedBytes::<1>::strict_decode(reader)?;
		let op_count = u32::strict_decode(reader)?;
		Ok(Self { genesis_seals, op_count })
	}
}

#[derive(Default)]
struct SealIndex {
	seals_total: usize,
	extern_outpoints: BTreeSet<String>,
	wout_outpoints: BTreeSet<String>,
	fallback_outpoints: BTreeSet<String>,
	noises: BTreeSet<String>,
	/// Auth tokens derived from revealed seal definitions in the consignment.
	seal_auth_tokens: BTreeSet<String>,
	/// Auth tokens present in operation state outputs (may include tokens without revealed seals).
	op_auth_tokens: BTreeSet<String>,
}

fn read_bytes(path: &str) -> Vec<u8> {
	if path == "-" {
		let mut buf = Vec::new();
		std::io::stdin().read_to_end(&mut buf).unwrap_or_else(|e| die(e.to_string()));
		if buf.is_empty() {
			die("stdin is empty");
		}
		return buf;
	}
	std::fs::read(path).unwrap_or_else(|e| die(format!("failed to read {path}: {e}")))
}

fn decode_archive(format: DebugConsignmentFormat, bytes: &[u8]) -> Vec<u8> {
	let detect = || {
		if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
			return DebugConsignmentFormat::Gzip;
		}
		if bytes.len() >= 4 && bytes[0] == b'P' && bytes[1] == b'K' {
			return DebugConsignmentFormat::Zip;
		}
		DebugConsignmentFormat::Raw
	};

	let format = match format {
		DebugConsignmentFormat::Auto => detect(),
		other => other,
	};

	match format {
		DebugConsignmentFormat::Raw => bytes.to_vec(),
		DebugConsignmentFormat::Gzip => {
			let mut dec = GzDecoder::new(bytes);
			let mut out = Vec::new();
			dec.read_to_end(&mut out).unwrap_or_else(|e| die(format!("gzip decode: {e}")));
			out
		},
		DebugConsignmentFormat::Zip => {
			let cur = std::io::Cursor::new(bytes);
			let mut zip = ZipArchive::new(cur).unwrap_or_else(|e| die(format!("zip decode: {e}")));
			if zip.len() == 0 {
				die("zip archive is empty");
			}

			let mut best_idx = None::<usize>;
			let mut best_score = i32::MAX;
			for i in 0..zip.len() {
				let f = zip.by_index(i).unwrap_or_else(|e| die(format!("zip entry: {e}")));
				if f.is_dir() {
					continue;
				}
				let name = f.name().to_lowercase();
				let mut score = 100;
				if name.contains("consign") {
					score -= 50;
				}
				if name.ends_with(".rgbc") || name.ends_with(".consignment") {
					score -= 20;
				}
				if name.ends_with(".bin") || name.ends_with(".dat") {
					score -= 5;
				}
				if score < best_score {
					best_score = score;
					best_idx = Some(i);
				}
			}

			let Some(i) = best_idx else {
				die("zip archive contains no files");
			};
			let mut f = zip.by_index(i).unwrap_or_else(|e| die(format!("zip entry: {e}")));
			let mut out = Vec::new();
			f.read_to_end(&mut out).unwrap_or_else(|e| die(format!("zip read: {e}")));
			out
		},
		DebugConsignmentFormat::Auto => unreachable!(),
	}
}

fn read_invoice_arg(s: &str) -> String {
	if s == "-" {
		return read_stdin_trimmed("invoice").unwrap_or_else(|e| die(e));
	}
	if let Some(path) = s.strip_prefix('@') {
		return read_file_trimmed(path).unwrap_or_else(|e| die(e));
	}
	s.to_string()
}

fn index_seal(index: &mut SealIndex, seal: &WTxoSeal) {
	index.seals_total += 1;

	match seal.primary {
		WOutpoint::Extern(outpoint) => {
			index.extern_outpoints.insert(format!("{}:{}", outpoint.txid, outpoint.vout));
		},
		WOutpoint::Wout(vout) => {
			index.wout_outpoints.insert(format!("~:{vout}"));
		},
	}

	match seal.secondary {
		TxoSealExt::Noise(noise) => {
			index.noises.insert(noise.to_string());
		},
		TxoSealExt::Fallback(outpoint) => {
			index.fallback_outpoints.insert(format!("{}:{}", outpoint.txid, outpoint.vout));
		},
	}

	index.seal_auth_tokens.insert(seal.auth_token().to_string());
}

fn index_consignment_seals(bytes: &[u8]) -> (String, SealIndex) {
	let cur = std::io::Cursor::new(bytes);
	let raw = StreamReader::new::<{ usize::MAX }>(cur);
	let mut reader = StrictReader::with(raw);

	let contract_id =
		rgb::parse_consignment(&mut reader).unwrap_or_else(|e| die(format!("consignment: {e}")));

	let header = ConsignmentHeader::decode(&mut reader)
		.unwrap_or_else(|e| die(format!("consignment header decode: {e}")));

	if header.op_count > MAX_CONSIGNMENT_OPS {
		die(format!(
			"consignment op_count={} exceeds MAX_CONSIGNMENT_OPS={MAX_CONSIGNMENT_OPS}",
			header.op_count
		));
	}

	let mut index = SealIndex::default();
	for (_, seal) in header.genesis_seals.iter() {
		index_seal(&mut index, seal);
	}

	for _ in 0..header.op_count {
		let op = OperationSeals::<TxoSeal>::strict_decode(&mut reader)
			.unwrap_or_else(|e| die(format!("consignment op decode: {e}")));
		for (_, seal) in op.defined_seals.iter() {
			index_seal(&mut index, seal);
		}
		for cell in op.operation.destructible_out.iter() {
			index.op_auth_tokens.insert(cell.auth.to_string());
		}
	}

	(contract_id.to_string(), index)
}

fn parse_outpoint(s: &str) -> Result<String, String> {
	let (txid, vout) = s.rsplit_once(':').ok_or_else(|| "expected format txid:vout".to_string())?;
	let txid = txid.trim().to_lowercase();
	if txid.len() != 64 || !txid.chars().all(|c| c.is_ascii_hexdigit()) {
		return Err("txid must be 64 hex chars".to_string());
	}
	let vout = vout.trim();
	let vout_n: u32 = vout.parse().map_err(|_| "vout must be a u32".to_string())?;
	Ok(format!("{txid}:{vout_n}"))
}

fn extract_outpoints_from_kv(kv: &std::collections::BTreeMap<String, String>) -> Vec<String> {
	let mut out = BTreeSet::<String>::new();

	let mut try_add = |s: &str| {
		let s = s.trim();
		if s.is_empty() {
			return;
		}
		if let Ok(op) = parse_outpoint(s) {
			out.insert(op);
		}
	};

	for (k, v) in kv {
		try_add(k);
		for part in v.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
			try_add(part);
		}
	}

	out.into_iter().collect()
}

fn invoice_checks(invoice: &RgbInvoice, index: &SealIndex) -> Vec<(String, bool, String)> {
	match &invoice.auth {
		RgbBeneficiary::Token(token) => {
			let t = token.to_string();
			let ok = index.seal_auth_tokens.contains(&t) || index.op_auth_tokens.contains(&t);
			vec![("invoice.beneficiary.token_in_consignment".to_string(), ok, t)]
		},
		RgbBeneficiary::WitnessOut(wout) => {
			let noise: Noise = wout.noise();
			let n = noise.to_string();
			let ok = index.noises.contains(&n);
			vec![("invoice.beneficiary.witness_out_noise_in_consignment".to_string(), ok, n)]
		},
	}
}

pub(crate) async fn handle(app: &App, command: &DebugCommand) {
	match command {
		DebugCommand::Invoice(args) => debug_invoice(app, args),
		DebugCommand::Consignment(args) => debug_consignment(app, args),
	}
}

fn debug_invoice(app: &App, args: &DebugInvoiceArgs) {
	let invoice_str = read_invoice_arg(&args.invoice);
	let invoice: RgbInvoice = invoice_str.parse().unwrap_or_else(|e| die(format!("{e}")));

	let unknown_query = invoice
		.unknown_query
		.iter()
		.map(|(k, v)| (k.to_string(), v.to_string()))
		.collect::<std::collections::BTreeMap<String, String>>();
	let utxos = extract_outpoints_from_kv(&unknown_query);

	let beneficiary = match &invoice.auth {
		RgbBeneficiary::Token(t) => json!({
			"type": "token",
			"value": t.to_string(),
		}),
		RgbBeneficiary::WitnessOut(wout) => {
			let noise: Noise = wout.noise();
			json!({
				"type": "wout",
				"value": wout.to_string(),
				"noise": noise.to_string(),
			})
		},
	};

	let out = json!({
		"invoice": invoice_str,
		"layer1": {
			"consensus": invoice.layer1.consensus.to_string(),
			"testnet": invoice.layer1.testnet,
		},
		"scope": invoice.scope.to_string(),
		"api": invoice.api.as_ref().map(|t| t.to_string()),
		"call": invoice.call.as_ref().map(|c| json!({
			"method": c.method.to_string(),
			"owned": c.owned.as_ref().map(|s| s.to_string()),
		})),
		"beneficiary": beneficiary,
		"data": invoice.data.as_ref().map(|d| d.to_string()),
		"lock": invoice.lock.as_ref().map(|b| base64::engine::general_purpose::STANDARD.encode(b)),
		"expiry": invoice.expiry.as_ref().map(|e| e.to_rfc3339()),
		"endpoints": invoice.endpoints.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
		"unknown_query": unknown_query,
		"utxos": utxos,
	});

	match app.output {
		ui::OutputMode::Json => print_json(&out, app.pretty),
		ui::OutputMode::Text => {
			println!("invoice={}", out["invoice"].as_str().unwrap_or("-"));
			println!("scope={}", out["scope"].as_str().unwrap_or("-"));
			println!(
				"layer1={} testnet={}",
				out["layer1"]["consensus"].as_str().unwrap_or("-"),
				out["layer1"]["testnet"].as_bool().unwrap_or(false)
			);
			if let Some(api) = out["api"].as_str() {
				println!("api={api}");
			}
			if let Some(method) = out["call"]["method"].as_str() {
				println!("call_method={method}");
			}
			if let Some(owned) = out["call"]["owned"].as_str() {
				println!("call_owned={owned}");
			}
			println!("beneficiary_type={}", out["beneficiary"]["type"].as_str().unwrap_or("-"));
			println!("beneficiary_value={}", out["beneficiary"]["value"].as_str().unwrap_or("-"));
			if let Some(noise) = out["beneficiary"]["noise"].as_str() {
				println!("beneficiary_noise={noise}");
			}
			if let Some(data) = out["data"].as_str() {
				println!("data={data}");
			}
			if let Some(expiry) = out["expiry"].as_str() {
				println!("expiry={expiry}");
			}
			if let Some(arr) = out["utxos"].as_array() {
				for v in arr {
					if let Some(s) = v.as_str() {
						println!("utxo={s}");
					}
				}
			}
			if let Some(obj) = out["unknown_query"].as_object() {
				for (k, v) in obj {
					if let Some(s) = v.as_str() {
						println!("query.{k}={s}");
					}
				}
			}
		},
	}
}

fn debug_consignment(app: &App, args: &DebugConsignmentArgs) {
	let raw = read_bytes(&args.path);
	let bytes = decode_archive(args.format, &raw);
	let (contract_id, index) = index_consignment_seals(&bytes);

	let mut checks = Vec::<serde_json::Value>::new();
	let mut ok = true;

	let mut want_outpoints = Vec::<String>::new();
	for s in &args.outpoint {
		let o = parse_outpoint(s).unwrap_or_else(|e| die(format!("--outpoint {s}: {e}")));
		want_outpoints.push(o);
	}

	for op in &want_outpoints {
		let present = index.extern_outpoints.contains(op) || index.fallback_outpoints.contains(op);
		ok &= present;
		checks.push(json!({
			"name": "outpoint_in_consignment",
			"target": op,
			"ok": present,
		}));
	}

	let mut invoice_checks_out = Vec::<serde_json::Value>::new();
	let invoice_opt = args.invoice.as_deref().map(read_invoice_arg);
	if let Some(invoice_str) = &invoice_opt {
		let invoice: RgbInvoice = invoice_str.parse().unwrap_or_else(|e| die(format!("{e}")));
		for (name, present, value) in invoice_checks(&invoice, &index) {
			ok &= present;
			invoice_checks_out.push(json!({
				"name": name,
				"value": value,
				"ok": present,
			}));
		}
	}

	let auth_tokens =
		index.seal_auth_tokens.union(&index.op_auth_tokens).cloned().collect::<Vec<_>>();

	let out = json!({
		"ok": ok,
		"contract_id": contract_id,
		"seals_total": index.seals_total,
		"extern_outpoints": index.extern_outpoints.iter().cloned().collect::<Vec<_>>(),
		"wout_outpoints": index.wout_outpoints.iter().cloned().collect::<Vec<_>>(),
		"fallback_outpoints": index.fallback_outpoints.iter().cloned().collect::<Vec<_>>(),
		"noises": index.noises.iter().cloned().collect::<Vec<_>>(),
		"auth_tokens": auth_tokens,
		"seal_auth_tokens": index.seal_auth_tokens.iter().cloned().collect::<Vec<_>>(),
		"op_auth_tokens": index.op_auth_tokens.iter().cloned().collect::<Vec<_>>(),
		"checks": checks,
		"invoice_checks": invoice_checks_out,
	});

	match app.output {
		ui::OutputMode::Json => print_json(&out, app.pretty),
		ui::OutputMode::Text => {
			println!("ok={}", out["ok"].as_bool().unwrap_or(false));
			println!("contract_id={}", out["contract_id"].as_str().unwrap_or("-"));
			println!("seals_total={}", out["seals_total"].as_u64().unwrap_or(0));
			println!("extern_outpoints={}", index.extern_outpoints.len());
			println!("wout_outpoints={}", index.wout_outpoints.len());
			println!("fallback_outpoints={}", index.fallback_outpoints.len());
			println!("noises={}", index.noises.len());

			if !want_outpoints.is_empty() {
				for v in out["checks"].as_array().unwrap_or(&vec![]) {
					println!(
						"check outpoint={} ok={}",
						v["target"].as_str().unwrap_or("-"),
						v["ok"].as_bool().unwrap_or(false)
					);
				}
			}
			if !invoice_checks_out.is_empty() {
				for v in out["invoice_checks"].as_array().unwrap_or(&vec![]) {
					println!(
						"check {}={} ok={}",
						v["name"].as_str().unwrap_or("-"),
						v["value"].as_str().unwrap_or("-"),
						v["ok"].as_bool().unwrap_or(false)
					);
				}
			}

			let max = args.max_outpoints;
			let mut shown = 0usize;
			for op in index.extern_outpoints.iter() {
				if shown >= max {
					break;
				}
				println!("outpoint={op}");
				shown += 1;
			}
			if index.extern_outpoints.len() > shown {
				println!("outpoint_more={}", index.extern_outpoints.len() - shown);
			}
		},
	}

	if !ok {
		std::process::exit(1);
	}
}
