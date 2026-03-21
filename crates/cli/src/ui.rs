use std::io::IsTerminal;

use comfy_table::presets;
use comfy_table::{Cell, CellAlignment, Table};
use owo_colors::OwoColorize;
use supports_color::Stream;

use rgbldk_http_dto::{EventDto, HealthCheckDto};

#[derive(Clone, Copy, Debug)]
pub enum ColorMode {
	Auto,
	Always,
	Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputMode {
	Text,
	Json,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
	pub color: bool,
	pub unicode: bool,
	pub ok: &'static str,
	pub bad: &'static str,
}

pub fn truncate_id(s: &str) -> String {
	const HEAD: usize = 8;
	const TAIL: usize = 8;
	if s.len() <= HEAD + TAIL + 3 {
		return s.to_string();
	}
	format!("{}...{}", &s[..HEAD], &s[s.len() - TAIL..])
}

pub fn format_u64_with_commas(v: u64) -> String {
	format_sats_with_commas(v)
}

fn format_sats_with_commas(v: u64) -> String {
	let s = v.to_string();
	let mut out = String::with_capacity(s.len() + s.len() / 3);
	for (n, ch) in s.chars().rev().enumerate() {
		if n > 0 && n.is_multiple_of(3) {
			out.push(',');
		}
		out.push(ch);
	}
	out.chars().rev().collect()
}

fn format_btc_from_sats(sats: u64) -> String {
	let btc = (sats as f64) / 100_000_000.0;
	let mut s = format!("{btc:.8}");
	while s.contains('.') && s.ends_with('0') {
		s.pop();
	}
	if s.ends_with('.') {
		s.push('0');
	}
	format!("{s} BTC")
}

pub fn format_balance_sats(sats: u64, force_sats: bool) -> String {
	if force_sats {
		format!("{sats} sats")
	} else if sats >= 100_000_000 {
		format_btc_from_sats(sats)
	} else {
		format!("{} sats", format_sats_with_commas(sats))
	}
}

pub fn print_event_text(ev: &EventDto, no_truncate: bool) {
	match ev {
		EventDto::PaymentSuccessful { payment_id, fee_paid_msat } => {
			let pid = payment_id.as_deref().unwrap_or("-");
			let fee = fee_paid_msat
				.map(|v| format!("{} msat", format_u64_with_commas(v)))
				.unwrap_or_else(|| "-".into());
			println!("PaymentSuccessful payment_id={pid} fee_paid={fee}");
		},
		EventDto::PaymentFailed { payment_id } => {
			let pid = payment_id.as_deref().unwrap_or("-");
			println!("PaymentFailed payment_id={pid}");
		},
		EventDto::PaymentReceived { payment_id, payment_hash, amount_msat, rgb, .. } => {
			let pid = payment_id.as_deref().unwrap_or("-");
			let rgb_summary = rgb.as_ref().map(|r| {
				let asset_id =
					if no_truncate { r.asset_id.clone() } else { truncate_id(&r.asset_id) };
				format!(
					" rgb={{asset_id={} asset_amount={} dir={} swap={}}}",
					asset_id,
					format_u64_with_commas(r.asset_amount),
					r.direction,
					r.is_swap
				)
			});
			println!(
				"PaymentReceived payment_id={pid} payment_hash={} amount={} msat{}",
				payment_hash,
				format_u64_with_commas(*amount_msat),
				rgb_summary.unwrap_or_default()
			);
		},
		EventDto::ChannelPending { funding_txo } => {
			println!("ChannelPending funding_txo={}:{}", funding_txo.txid, funding_txo.vout);
		},
		EventDto::ChannelReady { user_channel_id } => {
			println!("ChannelReady user_channel_id={user_channel_id}");
		},
		EventDto::ChannelClosed { channel_id, user_channel_id, counterparty_node_id, reason } => {
			let cp = counterparty_node_id.as_deref().unwrap_or("-");
			let reason = reason.as_deref().unwrap_or("-");
			println!(
				"ChannelClosed user_channel_id={user_channel_id} channel_id={channel_id} counterparty_node_id={cp} reason={reason}"
			);
		},
		EventDto::Other { kind } => {
			println!("Other kind={kind}");
		},
	}
}

pub fn resolve_output_mode(mode: crate::OutputOpt) -> OutputMode {
	match mode {
		crate::OutputOpt::Text => OutputMode::Text,
		crate::OutputOpt::Json => OutputMode::Json,
		crate::OutputOpt::Auto => {
			if std::io::stdout().is_terminal() {
				OutputMode::Text
			} else {
				OutputMode::Json
			}
		},
	}
}

pub fn resolve_theme(color_mode: ColorMode) -> Theme {
	let is_tty = std::io::stdout().is_terminal();
	let supports = supports_color::on(Stream::Stdout).is_some();
	let use_color = match color_mode {
		ColorMode::Always => true,
		ColorMode::Never => false,
		ColorMode::Auto => is_tty && supports && std::env::var_os("NO_COLOR").is_none(),
	};

	let unicode = is_tty;
	let (ok, bad) = if unicode { ("✔", "✘") } else { ("[OK]", "[X]") };
	Theme { color: use_color, unicode, ok, bad }
}

pub fn print_checks(theme: Theme, title: &str, ok: bool, checks: &[HealthCheckDto]) {
	if theme.color {
		if ok {
			println!("{} {title}", theme.ok.green());
		} else {
			println!("{} {title}", theme.bad.red());
		}
	} else if ok {
		println!("{} {title}", theme.ok);
	} else {
		println!("{} {title}", theme.bad);
	}

	for c in checks {
		let sym = if c.ok { theme.ok } else { theme.bad };
		let name = display_check_name(&c.name);
		let detail = c.detail.as_deref().unwrap_or("");

		if theme.color {
			if c.ok {
				if detail.is_empty() {
					println!("  {} {name}", sym.green());
				} else {
					println!("  {} {name}: {detail}", sym.green());
				}
			} else if detail.is_empty() {
				println!("  {} {name}", sym.red());
			} else {
				println!("  {} {name}: {detail}", sym.red());
			}
		} else if detail.is_empty() {
			println!("  {sym} {name}");
		} else {
			println!("  {sym} {name}: {detail}");
		}

		if let Some(hint) = &c.hint {
			println!("      hint: {hint}");
		}
	}
}

fn display_check_name(name: &str) -> String {
	match name {
		"http_server" => "HTTP Server".to_string(),
		"node_is_running" => "Lightning Node".to_string(),
		"p2p_is_listening" => "P2P Listener".to_string(),
		"best_block_height_known" => "Best Block Height".to_string(),
		_ => humanize_check_name(name),
	}
}

fn humanize_check_name(name: &str) -> String {
	let mut out = String::new();
	for (i, part) in name.split('_').enumerate() {
		if part.is_empty() {
			continue;
		}
		if i > 0 && !out.is_empty() {
			out.push(' ');
		}
		let upper = match part {
			"http" => Some("HTTP"),
			"api" => Some("API"),
			"p2p" => Some("P2P"),
			"rgb" => Some("RGB"),
			"ldk" => Some("LDK"),
			_ => None,
		};
		if let Some(s) = upper {
			out.push_str(s);
		} else {
			let mut chars = part.chars();
			if let Some(first) = chars.next() {
				out.extend(first.to_uppercase());
				out.push_str(chars.as_str());
			}
		}
	}
	if out.is_empty() {
		name.to_string()
	} else {
		out
	}
}

pub fn print_table(theme: Theme, headers: &[&str], rows: Vec<Vec<String>>) {
	let mut table = Table::new();
	table
		.load_preset(if theme.unicode { presets::UTF8_FULL } else { presets::ASCII_FULL })
		.set_header(headers.iter().map(|h| Cell::new(*h)));

	for r in rows {
		table.add_row(r.into_iter().map(Cell::new));
	}
	println!("{table}");
}

pub fn print_table_with_right_align(
	theme: Theme, headers: &[&str], rows: Vec<Vec<String>>, right_align_cols: &[usize],
) {
	let mut table = Table::new();
	table
		.load_preset(if theme.unicode { presets::UTF8_FULL } else { presets::ASCII_FULL })
		.set_header(headers.iter().map(|h| Cell::new(*h)));

	for r in rows {
		let mut cells = Vec::with_capacity(r.len());
		for (i, v) in r.into_iter().enumerate() {
			let mut cell = Cell::new(v);
			if right_align_cols.contains(&i) {
				cell = cell.set_alignment(CellAlignment::Right);
			}
			cells.push(cell);
		}
		table.add_row(cells);
	}
	println!("{table}");
}
