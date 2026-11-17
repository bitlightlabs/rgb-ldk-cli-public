use rgbldk_http_client::dto::{
	Lsps2InvoiceRequest, Lsps2InvoiceResponse, Lsps2JitChannelDto, Lsps2JitChannelsResponse,
	Lsps2OptionsDto, Lsps2OptionsResponse, Lsps2QuoteRequest, Lsps2QuoteResponse,
	Lsps2RgbInvoiceRequest, Lsps2RgbInvoiceResponse, Lsps2RgbQuoteRequest, Lsps2RgbQuoteResponse,
};

use crate::app::App;
use crate::cli::{Lsps1GetSetCommand, Lsps2Command, Lsps2RgbQuoteArgs};
use crate::client::{join_url, send_json};
use crate::cmd::lsps1::read_json_arg;
use crate::ui;
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &Lsps2Command) {
	match command {
		Lsps2Command::Quote(args) => {
			let req =
				Lsps2QuoteRequest { lsp_node_id: args.lsp.clone(), amount_msat: args.amount_msat };
			let url = join_url(&app.base, "/api/v1/lsps2/quote");
			let resp: Lsps2QuoteResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			print_json(&resp, app.pretty);
		},
		Lsps2Command::Invoice(args) => {
			let req = Lsps2InvoiceRequest {
				lsp_node_id: args.lsp.clone(),
				amount_msat: args.amount_msat,
				max_fee: args.max_fee,
				description: args.description.clone(),
				expiry_secs: args.expiry_secs,
			};
			let url = join_url(&app.base, "/api/v1/lsps2/invoice");
			let resp: Lsps2InvoiceResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => println!("{}", resp.invoice),
			}
		},
		Lsps2Command::RgbQuote(args) => {
			let url = join_url(&app.base, "/api/v1/lsps2/rgb_quote");
			let resp: Lsps2RgbQuoteResponse =
				send_json(app.client.post(url).json(&rgb_quote(args)))
					.await
					.unwrap_or_else(|e| die(e));
			print_json(&resp, app.pretty);
		},
		Lsps2Command::RgbInvoice(args) => {
			let req = Lsps2RgbInvoiceRequest {
				quote: rgb_quote(&args.quote),
				max_fee_sat: args.max_fee_sat,
				description: args.description.clone(),
				expiry_secs: args.expiry_secs,
			};
			let url = join_url(&app.base, "/api/v1/lsps2/rgb_invoice");
			let resp: Lsps2RgbInvoiceResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!(
						"payer pays {} msat (fee {} sat)\n{}",
						resp.amount_msat, resp.fee.fee_sat, resp.invoice
					)
				},
			}
		},
		Lsps2Command::Options { command } => match command {
			Lsps1GetSetCommand::Get => {
				let url = join_url(&app.base, "/api/v1/lsps2/options");
				let opts: Lsps2OptionsResponse =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				print_json(&opts, app.pretty);
			},
			Lsps1GetSetCommand::Set(args) => {
				// The node rejects `advertise_service` in the body. It is dropped here so that the
				// output of `options get` can be edited and fed back.
				let mut doc: serde_json::Value = read_json_arg(args);
				if let Some(fields) = doc.as_object_mut() {
					fields.remove("advertise_service");
				}
				let body: Lsps2OptionsDto = serde_json::from_value(doc)
					.unwrap_or_else(|e| die(format!("invalid JSON: {e}")));
				let url = join_url(&app.base, "/api/v1/lsps2/options");
				let updated: Lsps2OptionsResponse =
					send_json(app.client.put(url).json(&body)).await.unwrap_or_else(|e| die(e));
				print_json(&updated, app.pretty);
			},
		},
		Lsps2Command::JitChannels => {
			let url = join_url(&app.base, "/api/v1/lsps2/jit_channels");
			let resp: Lsps2JitChannelsResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => print_jit_channels_table(app, &resp.channels),
			}
		},
	}
}

fn print_jit_channels_table(app: &App, channels: &[Lsps2JitChannelDto]) {
	if channels.is_empty() {
		println!("No JIT channels sold.");
		return;
	}
	let short = |id: &str| if app.no_truncate { id.to_string() } else { ui::truncate_id(id) };
	let rows = channels
		.iter()
		.map(|c| {
			vec![
				short(&c.counterparty_node_id),
				c.state.clone(),
				c.channel_id.as_deref().map(short).unwrap_or_else(|| "-".into()),
				if c.rgb.is_some() { "RGB".into() } else { "BTC".into() },
				c.opening_fee_msat.map(|fee| fee.to_string()).unwrap_or_else(|| "-".into()),
				c.counts_toward_unpaid_quota.to_string(),
				c.close_kind.clone().unwrap_or_else(|| "-".into()),
				c.close_attempts.to_string(),
			]
		})
		.collect::<Vec<_>>();
	let headers = vec![
		"Counterparty",
		"State",
		"Channel ID",
		"Kind",
		"Fee (msat)",
		"Unpaid Quota",
		"Closed By",
		"Close Attempts",
	];
	ui::print_table(app.theme, &headers, rows);
}

fn rgb_quote(args: &Lsps2RgbQuoteArgs) -> Lsps2RgbQuoteRequest {
	Lsps2RgbQuoteRequest {
		lsp_node_id: args.lsp.clone(),
		contract_id: args.contract_id.clone(),
		asset_amount: args.asset_amount,
		lsp_asset_balance: args.lsp_asset_balance,
		lsp_balance_sat: args.lsp_balance_sat,
		btc_carrier_amount_msat: args.btc_carrier_amount_msat,
	}
}
