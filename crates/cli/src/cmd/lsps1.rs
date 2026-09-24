use rgbldk_http_client::dto::{
	Lsps1InfoResponse, Lsps1LspConfigDto, Lsps1OptionsDto, Lsps1OrderCreateRequest,
	Lsps1OrderResponse, Lsps1PricingDto, Lsps1RgbOrderCreateRequest, Lsps1ServiceOrderDto,
	Lsps1ServiceOrdersResponse,
};

use crate::app::App;
use crate::cli::{
	Lsps1Command, Lsps1GetSetCommand, Lsps1JsonArgs, Lsps1LspCommand, Lsps1LspSetArgs,
	Lsps1OrderCommand, Lsps1OrderCreateArgs, Lsps1OrdersCommand,
};
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &Lsps1Command) {
	match command {
		Lsps1Command::Info => {
			let url = join_url(&app.base, "/api/v1/lsps1/info");
			let info: Lsps1InfoResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			print_json(&info, app.pretty);
		},
		Lsps1Command::Lsp { command } => handle_lsp(app, command).await,
		Lsps1Command::Options { command } => match command {
			Lsps1GetSetCommand::Get => {
				let url = join_url(&app.base, "/api/v1/lsps1/options");
				let opts: Lsps1OptionsDto =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				print_json(&opts, app.pretty);
			},
			Lsps1GetSetCommand::Set(args) => {
				let body: Lsps1OptionsDto = read_json_arg(args);
				let url = join_url(&app.base, "/api/v1/lsps1/options");
				let updated: Lsps1OptionsDto =
					send_json(app.client.put(url).json(&body)).await.unwrap_or_else(|e| die(e));
				print_json(&updated, app.pretty);
			},
		},
		Lsps1Command::Pricing { command } => match command {
			Lsps1GetSetCommand::Get => {
				let url = join_url(&app.base, "/api/v1/lsps1/pricing");
				let pricing: Lsps1PricingDto =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				print_json(&pricing, app.pretty);
			},
			Lsps1GetSetCommand::Set(args) => {
				let body: Lsps1PricingDto = read_json_arg(args);
				let url = join_url(&app.base, "/api/v1/lsps1/pricing");
				let updated: Lsps1PricingDto =
					send_json(app.client.put(url).json(&body)).await.unwrap_or_else(|e| die(e));
				print_json(&updated, app.pretty);
			},
		},
		Lsps1Command::Order { command } => handle_order(app, command).await,
		Lsps1Command::RgbOrder(args) => {
			let req = Lsps1RgbOrderCreateRequest {
				asset_id: args.asset_id.clone(),
				lsp_asset_balance: args.lsp_asset_balance,
				client_asset_balance: args.client_asset_balance,
				lsp_balance_sat: args.lsp_balance_sat,
				client_balance_sat: args.client_balance_sat,
				channel_expiry_blocks: args.channel_expiry_blocks,
				announce_channel: args.announce_channel,
			};
			let url = join_url(&app.base, "/api/v1/lsps1/rgb_order");
			let resp: Lsps1OrderResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			print_order(app, &resp);
		},
		Lsps1Command::Orders { command } => handle_orders(app, command).await,
	}
}

async fn handle_lsp(app: &App, command: &Lsps1LspCommand) {
	match command {
		Lsps1LspCommand::Get => {
			let url = join_url(&app.base, "/api/v1/lsps1/lsp");
			let lsp: Lsps1LspConfigDto =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			print_json(&lsp, app.pretty);
		},
		Lsps1LspCommand::Set(args) => {
			let Lsps1LspSetArgs { pubkey, address, token } = args;
			let req = Lsps1LspConfigDto {
				pubkey: pubkey.clone(),
				address: address.clone(),
				token: token.clone(),
			};
			let url = join_url(&app.base, "/api/v1/lsps1/lsp");
			let updated: Lsps1LspConfigDto =
				send_json(app.client.put(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&updated, app.pretty),
				ui::OutputMode::Text => {
					println!("LSP set to {} @ {}", updated.pubkey, updated.address)
				},
			}
		},
	}
}

async fn handle_order(app: &App, command: &Lsps1OrderCommand) {
	match command {
		Lsps1OrderCommand::Create(args) => {
			let Lsps1OrderCreateArgs {
				lsp_balance_sat,
				client_balance_sat,
				channel_expiry_blocks,
				announce_channel,
			} = args;
			let req = Lsps1OrderCreateRequest {
				lsp_balance_sat: *lsp_balance_sat,
				client_balance_sat: *client_balance_sat,
				channel_expiry_blocks: *channel_expiry_blocks,
				announce_channel: *announce_channel,
			};
			let url = join_url(&app.base, "/api/v1/lsps1/order");
			let resp: Lsps1OrderResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			print_order(app, &resp);
		},
		Lsps1OrderCommand::Get { order_id } => {
			let url = join_url(&app.base, &format!("/api/v1/lsps1/order/{order_id}"));
			let resp: Lsps1OrderResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			print_order(app, &resp);
		},
	}
}

async fn handle_orders(app: &App, command: &Lsps1OrdersCommand) {
	match command {
		Lsps1OrdersCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/lsps1/orders");
			let resp: Lsps1ServiceOrdersResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => print_orders_table(app, &resp.orders),
			}
		},
		Lsps1OrdersCommand::Get { order_id } => {
			let url = join_url(&app.base, &format!("/api/v1/lsps1/orders/{order_id}"));
			let order: Lsps1ServiceOrderDto =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			print_json(&order, app.pretty);
		},
	}
}

/// A placed order shows its id and channel state; the full body carries payment
/// options and RGB terms, so JSON stays the honest default in text mode too.
fn print_order(app: &App, resp: &Lsps1OrderResponse) {
	match app.output {
		ui::OutputMode::Json => print_json(resp, app.pretty),
		ui::OutputMode::Text => {
			let opened = resp.channel.is_some();
			println!(
				"order {} — {}",
				resp.order_id,
				if opened { "channel opened" } else { "awaiting fulfillment" }
			);
			print_json(resp, app.pretty);
		},
	}
}

fn print_orders_table(app: &App, orders: &[rgbldk_http_client::dto::Lsps1ServiceOrderDto]) {
	if orders.is_empty() {
		println!("No LSPS1 orders served.");
		return;
	}
	let rows = orders
		.iter()
		.map(|o| {
			let counterparty = if app.no_truncate {
				o.counterparty_node_id.clone()
			} else {
				ui::truncate_id(&o.counterparty_node_id)
			};
			let order_id =
				if app.no_truncate { o.order_id.clone() } else { ui::truncate_id(&o.order_id) };
			vec![
				order_id,
				counterparty,
				o.order_state.clone(),
				o.payment_state.clone(),
				o.paid_via.clone().unwrap_or_else(|| "-".into()),
			]
		})
		.collect::<Vec<_>>();
	let headers = vec!["Order ID", "Counterparty", "Order", "Payment", "Paid Via"];
	ui::print_table(app.theme, &headers, rows);
}

/// Read a `--json` argument: a file path, or `-` for stdin.
fn read_json_arg<T: serde::de::DeserializeOwned>(args: &Lsps1JsonArgs) -> T {
	use std::io::Read;
	let raw = if args.json == "-" {
		let mut buf = String::new();
		std::io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| die(e.to_string()));
		buf
	} else {
		std::fs::read_to_string(&args.json).unwrap_or_else(|e| die(format!("{}: {}", args.json, e)))
	};
	serde_json::from_str(&raw).unwrap_or_else(|e| die(format!("invalid JSON: {e}")))
}
