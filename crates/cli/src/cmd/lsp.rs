use rgbldk_http_client::dto::{LspAddRequest, LspDto, LspListResponse};

use crate::app::App;
use crate::cli::{LspAddArgs, LspCommand};
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &LspCommand) {
	match command {
		LspCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/lsps");
			let resp: LspListResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => print_lsps_table(app, &resp.lsps),
			}
		},
		LspCommand::Add(args) => {
			let LspAddArgs { pubkey, address, token, trust_0conf, trust_no_reserve } = args;
			let req = LspAddRequest {
				pubkey: pubkey.clone(),
				address: address.clone(),
				token: token.clone(),
				trust_peer_0conf: *trust_0conf,
				trust_peer_no_reserve: *trust_no_reserve,
			};
			let url = join_url(&app.base, "/api/v1/lsps");
			let lsp: LspDto =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&lsp, app.pretty),
				ui::OutputMode::Text => {
					println!("LSP {} @ {} saved ({})", lsp.pubkey, lsp.address, protocols(&lsp))
				},
			}
		},
		LspCommand::Rm { pubkey } => {
			let url = join_url(&app.base, &format!("/api/v1/lsps/{pubkey}"));
			let resp: serde_json::Value =
				send_json(app.client.delete(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => println!("LSP {pubkey} removed"),
			}
		},
	}
}

fn print_lsps_table(app: &App, lsps: &[LspDto]) {
	if lsps.is_empty() {
		println!("No LSPs configured.");
		return;
	}
	let rows = lsps
		.iter()
		.map(|l| {
			let pubkey =
				if app.no_truncate { l.pubkey.clone() } else { ui::truncate_id(&l.pubkey) };
			vec![
				pubkey,
				l.address.clone(),
				protocols(l),
				yes_no(l.trust_peer_0conf),
				yes_no(l.trust_peer_no_reserve),
				yes_no(l.token.is_some()),
			]
		})
		.collect::<Vec<_>>();
	let headers = vec!["Pubkey", "Address", "Protocols", "0conf", "No Reserve", "Token"];
	ui::print_table(app.theme, &headers, rows);
}

/// `LSPS1, LSPS2`, or `unknown` until the LSP has answered an LSPS0 probe.
fn protocols(lsp: &LspDto) -> String {
	match &lsp.protocols {
		Some(p) if !p.is_empty() => {
			let mut p = p.clone();
			p.sort_unstable();
			p.iter().map(|n| format!("LSPS{n}")).collect::<Vec<_>>().join(", ")
		},
		Some(_) => "none".into(),
		None => "unknown".into(),
	}
}

fn yes_no(value: bool) -> String {
	if value { "yes" } else { "no" }.into()
}
