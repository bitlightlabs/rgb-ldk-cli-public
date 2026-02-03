use std::io::IsTerminal;

use comfy_table::presets;
use comfy_table::{Cell, CellAlignment, Table};
use owo_colors::OwoColorize;
use supports_color::Stream;

use rgbldk_api::http::HealthCheckDto;

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
