use rgbldk_http_client::dto::{
	CloseChannelRequest, ClosingChannelDto, OkResponse, OpenChannelRequest, OpenChannelResponse,
	RgbOpenChannelRequest, SpliceInRequest, SpliceOutRequest,
};
use serde::{Deserialize, Serialize};

use crate::app::App;
use crate::cli::ChannelCommand;
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{confirm_or_exit, die, print_json};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChannelDetailsView {
	channel_id: String,
	user_channel_id: String,
	counterparty_node_id: String,
	channel_point: Option<String>,
	#[serde(default)]
	short_channel_id: Option<String>,
	#[serde(default)]
	outbound_scid_alias: Option<String>,
	#[serde(default)]
	inbound_scid_alias: Option<String>,
	channel_value_sats: String,
	outbound_capacity_msat: String,
	inbound_capacity_msat: String,
	#[serde(default)]
	local_balance_msat: Option<String>,
	#[serde(default)]
	remote_balance_msat: Option<String>,
	#[serde(default)]
	local_unspendable_punishment_reserve_sats: Option<String>,
	#[serde(default)]
	remote_unspendable_punishment_reserve_sats: Option<String>,
	is_channel_ready: bool,
	is_usable: bool,
	is_announced: bool,
	rgb_balance: Option<RgbChannelBalanceView>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RgbChannelBalanceView {
	contract_id: String,
	local_amount: String,
	remote_amount: String,
}

pub(crate) async fn handle(app: &App, command: &ChannelCommand) {
	match command {
		ChannelCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/channels");
			let chans: Vec<ChannelDetailsView> =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&chans, app.pretty),
				crate::ui::OutputMode::Text => {
					let show_rgb = chans.iter().any(|c| c.rgb_balance.is_some());
					let rows = chans
						.into_iter()
						.map(|c| {
							let mut row = vec![
								c.user_channel_id,
								c.counterparty_node_id,
								c.short_channel_id.unwrap_or_else(|| "-".into()),
								c.outbound_scid_alias.unwrap_or_else(|| "-".into()),
								c.inbound_scid_alias.unwrap_or_else(|| "-".into()),
								format_decimal(&c.channel_value_sats),
								c.is_channel_ready.to_string(),
								c.is_usable.to_string(),
							];
							if show_rgb {
								if let Some(rgb) = c.rgb_balance {
									let contract = if app.no_truncate {
										rgb.contract_id
									} else {
										crate::ui::truncate_id(&rgb.contract_id)
									};
									row.push(contract);
									row.push(format_decimal(&rgb.local_amount));
									row.push(format_decimal(&rgb.remote_amount));
								} else {
									row.extend(["-".into(), "-".into(), "-".into()]);
								}
							}
							row
						})
						.map(|mut r| {
							if !app.no_truncate {
								r[0] = crate::ui::truncate_id(&r[0]);
								r[1] = crate::ui::truncate_id(&r[1]);
							}
							r
						})
						.collect::<Vec<_>>();
					let mut headers = vec![
						"User Channel ID",
						"Counterparty",
						"SCID",
						"Out Alias",
						"In Alias",
						"Capacity (sats)",
						"Ready",
						"Usable",
					];
					if show_rgb {
						headers.extend(["RGB Contract", "RGB Local", "RGB Remote"]);
					}
					crate::ui::print_table(app.theme, &headers, rows);
				},
			}
		},
		ChannelCommand::Closing => {
			let url = join_url(&app.base, "/api/v1/channels/closing");
			let closing: Vec<ClosingChannelDto> =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&closing, app.pretty),
				crate::ui::OutputMode::Text => {
					if closing.is_empty() {
						println!("No channels currently closing.");
						return;
					}
					let show_rgb = closing.iter().any(|c| c.rgb.is_some());
					let rows = closing
						.into_iter()
						.map(|c| {
							let btc_sats: u64 =
								c.btc_balances.iter().map(|b| b.amount_sats).sum::<u64>()
									+ c.sweeping_balances
										.iter()
										.map(|b| b.amount_sats)
										.sum::<u64>();
							let mut row = vec![
								c.user_channel_id.clone().unwrap_or_else(|| "-".into()),
								c.channel_id.clone(),
								c.counterparty_node_id.clone(),
								dto_enum_label(&c.status),
								dto_enum_label(&c.close_source),
								c.closing_txid.clone().unwrap_or_else(|| "-".into()),
								crate::ui::format_u64_with_commas(btc_sats),
							];
							if show_rgb {
								if let Some(rgb) = c.rgb {
									let contract = if app.no_truncate {
										rgb.contract_id
									} else {
										crate::ui::truncate_id(&rgb.contract_id)
									};
									let sweep = rgb
										.sweep_status
										.map(|s| dto_enum_label(&s))
										.unwrap_or_else(|| "-".into());
									row.push(contract);
									row.push(crate::ui::format_u64_with_commas(rgb.local_amount));
									row.push(sweep);
								} else {
									row.extend(["-".into(), "-".into(), "-".into()]);
								}
							}
							row
						})
						.map(|mut r| {
							if !app.no_truncate {
								// user_channel_id, channel_id, counterparty
								r[0] = crate::ui::truncate_id(&r[0]);
								r[1] = crate::ui::truncate_id(&r[1]);
								r[2] = crate::ui::truncate_id(&r[2]);
								// closing_txid
								r[5] = crate::ui::truncate_id(&r[5]);
							}
							r
						})
						.collect::<Vec<_>>();
					let mut headers = vec![
						"User Channel ID",
						"Channel ID",
						"Counterparty",
						"Status",
						"Source",
						"Closing Txid",
						"BTC (sats)",
					];
					if show_rgb {
						headers.extend(["RGB Contract", "RGB Local", "RGB Sweep"]);
					}
					crate::ui::print_table(app.theme, &headers, rows);
				},
			}
		},
		ChannelCommand::Open(args) => {
			let rgb = match (&args.rgb_contract_id, args.rgb_asset_amount, &args.rgb_context) {
				(None, None, None) => None,
				(Some(contract_id), Some(asset_amount), Some(color_context_data)) => Some(
					RgbOpenChannelRequest {
						contract_id: contract_id.clone(),
						asset_amount,
						color_context_data: color_context_data.clone(),
						funding_utxo_policy: None,
						funding_utxos: None,
					},
				),
				_ => die(
					"invalid rgb channel args: require --rgb-contract-id, --rgb-asset-amount, and --rgb-context together",
				),
			};
			let req = OpenChannelRequest {
				node_id: args.node_id.clone(),
				address: args.addr.clone(),
				channel_amount_sats: args.amount_sats,
				push_to_counterparty_msat: args.push_msat,
				announce: if args.private { Some(false) } else { None },
				rgb,
			};
			let url = join_url(&app.base, "/api/v1/channel/open");
			let resp: OpenChannelResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&resp, app.pretty),
				crate::ui::OutputMode::Text => println!("{}", resp.user_channel_id),
			}
		},
		ChannelCommand::Close(args) => {
			let req = CloseChannelRequest {
				user_channel_id: args.user_channel_id.clone(),
				counterparty_node_id: args.counterparty_node_id.clone(),
			};
			let url = join_url(&app.base, "/api/v1/channel/close");
			let v: serde_json::Value =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => println!("Channel close initiated."),
			}
		},
		ChannelCommand::ForceClose(args) => {
			confirm_or_exit(
				app.yes,
				"About to force-close a channel (potentially costly/time-locked).",
			);
			let req = CloseChannelRequest {
				user_channel_id: args.user_channel_id.clone(),
				counterparty_node_id: args.counterparty_node_id.clone(),
			};
			let url = join_url(&app.base, "/api/v1/channel/force_close");
			let v: serde_json::Value =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => println!("Channel force-close initiated."),
			}
		},
		ChannelCommand::SpliceIn(args) => {
			let req = SpliceInRequest {
				user_channel_id: args.user_channel_id.clone(),
				counterparty_node_id: args.counterparty_node_id.clone(),
				splice_amount_sats: args.splice_amount_sats,
			};
			let url = join_url(&app.base, "/api/v1/channel/splice_in");
			let resp: OkResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&resp, app.pretty),
				crate::ui::OutputMode::Text => {
					ui::print_checks(app.theme, "Splice in", resp.ok, &resp.checks);
					if resp.ok {
						println!(
							"Adding {} sats into channel {}.",
							crate::ui::format_u64_with_commas(args.splice_amount_sats),
							if app.no_truncate {
								args.user_channel_id.clone()
							} else {
								crate::ui::truncate_id(&args.user_channel_id)
							}
						);
					}
				},
			}
		},
		ChannelCommand::SpliceOut(args) => {
			let req = SpliceOutRequest {
				user_channel_id: args.user_channel_id.clone(),
				counterparty_node_id: args.counterparty_node_id.clone(),
				address: args.address.clone(),
				splice_amount_sats: args.splice_amount_sats,
			};
			let url = join_url(&app.base, "/api/v1/channel/splice_out");
			let resp: OkResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&resp, app.pretty),
				crate::ui::OutputMode::Text => {
					ui::print_checks(app.theme, "Splice out", resp.ok, &resp.checks);
					if resp.ok {
						println!(
							"Withdrawing {} sats from channel {} to {}.",
							crate::ui::format_u64_with_commas(args.splice_amount_sats),
							if app.no_truncate {
								args.user_channel_id.clone()
							} else {
								crate::ui::truncate_id(&args.user_channel_id)
							},
							args.address
						);
					}
				},
			}
		},
	}
}

fn format_decimal(value: &str) -> String {
	value
		.parse::<u64>()
		.map(crate::ui::format_u64_with_commas)
		.unwrap_or_else(|_| value.to_string())
}

fn dto_enum_label<T: Serialize + std::fmt::Debug>(value: &T) -> String {
	match serde_json::to_value(value) {
		Ok(serde_json::Value::String(s)) => s,
		_ => format!("{value:?}"),
	}
}
