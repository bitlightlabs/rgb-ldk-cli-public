use rgbldk_http_client::dto::OkResponse;
use serde::{Deserialize, Serialize};

use crate::app::App;
use crate::cli::{
	SwapCommand, SwapCreateArgs, SwapCreateMultihopArgs, SwapExecuteArgs, SwapStringArgs,
};
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

#[derive(Debug, Clone, Serialize)]
struct SwapCreateOfferRequest {
	counterparty_node_id: String,
	channel_scid: String,
	contract_id: String,
	asset_amount: String,
	btc_amount_msat: String,
	btc_carrier_amount_msat: String,
	maker_gives_rgb: bool,
	expiry_secs: u32,
}

#[derive(Debug, Clone, Serialize)]
struct SwapHopDto {
	node_id: String,
	channel_scid: String,
}

#[derive(Debug, Clone, Serialize)]
struct SwapCreateMultihopOfferRequest {
	rgb_path: Vec<SwapHopDto>,
	btc_path: Vec<SwapHopDto>,
	contract_id: String,
	asset_amount: String,
	btc_amount_msat: String,
	btc_carrier_amount_msat: String,
	maker_gives_rgb: bool,
	expiry_secs: u32,
}

#[derive(Debug, Clone, Serialize)]
struct SwapStringRequest {
	swap_string: String,
}

#[derive(Debug, Clone, Serialize)]
struct SwapExecuteRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	swap_string: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	payment_hash: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	force: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SwapInfoDto {
	payment_hash: String,
	role: String,
	status: String,
	counterparty_node_id: String,
	channel_scid: String,
	contract_id: String,
	asset_amount: String,
	btc_amount_msat: String,
	btc_carrier_amount_msat: String,
	maker_gives_rgb: bool,
	expiry_secs: u32,
	created_at_unix_secs: String,
	is_multihop: bool,
	#[serde(default)]
	last_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SwapOfferResponse {
	swap_string: String,
	payment_hash: String,
	info: SwapInfoDto,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SwapExecuteResponse {
	ok: bool,
	#[serde(default)]
	payment_hash: Option<String>,
	#[serde(default)]
	status: Option<String>,
}

pub(crate) async fn handle(app: &App, command: &SwapCommand) {
	match command {
		SwapCommand::Create(args) => create_offer(app, args).await,
		SwapCommand::CreateMultihop(args) => create_multihop_offer(app, args).await,
		SwapCommand::Decode(args) => decode_or_accept(app, args, false).await,
		SwapCommand::Accept(args) => decode_or_accept(app, args, true).await,
		SwapCommand::Execute(args) => execute(app, args).await,
		SwapCommand::Ls => list(app).await,
		SwapCommand::Get { payment_hash } => get(app, payment_hash).await,
		SwapCommand::Cancel { payment_hash } => cancel(app, payment_hash).await,
	}
}

async fn create_offer(app: &App, args: &SwapCreateArgs) {
	let req = SwapCreateOfferRequest {
		counterparty_node_id: args.counterparty_node_id.clone(),
		channel_scid: args.channel_scid.to_string(),
		contract_id: args.contract_id.clone(),
		asset_amount: args.asset_amount.to_string(),
		btc_amount_msat: args.btc_amount_msat.to_string(),
		btc_carrier_amount_msat: args.btc_carrier_amount_msat.to_string(),
		maker_gives_rgb: args.maker_gives_rgb,
		expiry_secs: args.expiry_secs,
	};
	let url = join_url(&app.base, "/api/v1/swap/offers");
	let resp: SwapOfferResponse =
		send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
	print_offer(app, &resp);
}

async fn create_multihop_offer(app: &App, args: &SwapCreateMultihopArgs) {
	let rgb_path = args
		.rgb_hop
		.iter()
		.map(|hop| parse_hop(hop).unwrap_or_else(|e| die(e)))
		.collect::<Vec<_>>();
	let btc_path = args
		.btc_hop
		.iter()
		.map(|hop| parse_hop(hop).unwrap_or_else(|e| die(e)))
		.collect::<Vec<_>>();
	let req = SwapCreateMultihopOfferRequest {
		rgb_path,
		btc_path,
		contract_id: args.contract_id.clone(),
		asset_amount: args.asset_amount.to_string(),
		btc_amount_msat: args.btc_amount_msat.to_string(),
		btc_carrier_amount_msat: args.btc_carrier_amount_msat.to_string(),
		maker_gives_rgb: args.maker_gives_rgb,
		expiry_secs: args.expiry_secs,
	};
	let url = join_url(&app.base, "/api/v1/swap/offers/multihop");
	let resp: SwapOfferResponse =
		send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
	print_offer(app, &resp);
}

async fn decode_or_accept(app: &App, args: &SwapStringArgs, accept: bool) {
	let path = if accept { "/api/v1/swap/accept" } else { "/api/v1/swap/decode" };
	let url = join_url(&app.base, path);
	let req = SwapStringRequest { swap_string: args.swap_string.clone() };
	let resp: SwapInfoDto =
		send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => {
			if accept {
				println!("Swap offer accepted.");
			}
			print_info(app, &resp);
		},
	}
}

async fn execute(app: &App, args: &SwapExecuteArgs) {
	let req = SwapExecuteRequest {
		swap_string: args.swap_string.clone(),
		payment_hash: args.payment_hash.clone(),
		force: args.force.then_some(true),
	};
	let url = join_url(&app.base, "/api/v1/swap/execute");
	let resp: SwapExecuteResponse =
		send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => {
			println!("Swap execution {}.", if resp.ok { "initiated" } else { "rejected" });
			if let Some(payment_hash) = resp.payment_hash {
				println!("Payment hash: {payment_hash}");
			}
			if let Some(status) = resp.status {
				println!("Status: {status}");
				if status != "Settled" {
					println!("Poll `swap get` until the status is Settled or Failed.");
				}
			}
		},
	}
}

async fn list(app: &App) {
	let url = join_url(&app.base, "/api/v1/swap/list");
	let resp: Vec<SwapInfoDto> = send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => {
			let rows = resp
				.into_iter()
				.map(|info| {
					vec![
						info.payment_hash,
						info.role,
						info.status,
						info.contract_id,
						format_decimal(&info.asset_amount),
						if info.maker_gives_rgb { "RGB".into() } else { "BTC".into() },
					]
				})
				.collect::<Vec<_>>();
			ui::print_table(
				app.theme,
				&["Payment Hash", "Role", "Status", "Contract", "Asset", "Maker Gives"],
				rows,
			);
		},
	}
}

async fn get(app: &App, payment_hash: &str) {
	let url = join_url(&app.base, &format!("/api/v1/swap/{payment_hash}"));
	let resp: SwapInfoDto = send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => print_info(app, &resp),
	}
}

async fn cancel(app: &App, payment_hash: &str) {
	let url = join_url(&app.base, &format!("/api/v1/swap/{payment_hash}"));
	let resp: OkResponse = send_json(app.client.delete(url)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => println!("Swap cancelled."),
	}
}

fn print_offer(app: &App, resp: &SwapOfferResponse) {
	match app.output {
		ui::OutputMode::Json => print_json(resp, app.pretty),
		ui::OutputMode::Text => {
			println!("Swap offer created.");
			println!();
			println!("Swap string:");
			println!("{}", resp.swap_string);
			println!();
			print_info(app, &resp.info);
		},
	}
}

fn print_info(app: &App, info: &SwapInfoDto) {
	let counterparty = if app.no_truncate {
		info.counterparty_node_id.clone()
	} else {
		ui::truncate_id(&info.counterparty_node_id)
	};
	let contract =
		if app.no_truncate { info.contract_id.clone() } else { ui::truncate_id(&info.contract_id) };
	let rows = vec![
		vec!["Payment hash".into(), info.payment_hash.clone()],
		vec!["Role".into(), info.role.clone()],
		vec!["Status".into(), info.status.clone()],
		vec!["Counterparty".into(), counterparty],
		vec!["Channel SCID".into(), info.channel_scid.clone()],
		vec!["Contract".into(), contract],
		vec!["Asset amount".into(), format_decimal(&info.asset_amount)],
		vec!["BTC amount (msat)".into(), format_decimal(&info.btc_amount_msat)],
		vec!["BTC carrier (msat)".into(), format_decimal(&info.btc_carrier_amount_msat)],
		vec!["Maker gives".into(), if info.maker_gives_rgb { "RGB".into() } else { "BTC".into() }],
		vec!["Expiry (secs)".into(), info.expiry_secs.to_string()],
		vec!["Created (unix secs)".into(), info.created_at_unix_secs.clone()],
		vec!["Multihop".into(), info.is_multihop.to_string()],
		vec!["Last error".into(), info.last_error.clone().unwrap_or_else(|| "-".into())],
	];
	ui::print_table(app.theme, &["Field", "Value"], rows);
}

fn parse_hop(raw: &str) -> Result<SwapHopDto, String> {
	let (node_id, channel_scid) = raw
		.rsplit_once(':')
		.ok_or_else(|| format!("invalid hop {raw:?}; expected NODE_ID:SCID"))?;
	if node_id.is_empty() {
		return Err(format!("invalid hop {raw:?}; node id must not be empty"));
	}
	let channel_scid = channel_scid
		.parse::<u64>()
		.map_err(|_| format!("invalid hop {raw:?}; SCID must be an unsigned integer"))?;
	Ok(SwapHopDto { node_id: node_id.to_string(), channel_scid: channel_scid.to_string() })
}

fn format_decimal(value: &str) -> String {
	value.parse::<u64>().map(ui::format_u64_with_commas).unwrap_or_else(|_| value.to_string())
}
