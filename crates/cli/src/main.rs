mod cli;
mod client;
mod config;
mod ui;
mod utils;

pub use crate::cli::OutputOpt;

use clap::Parser;
use rgbldk_api::http::{
	BalancesDto, Bolt11PayRequest, Bolt11PayResponse, Bolt11ReceiveRequest, Bolt11ReceiveResponse,
	Bolt11ReceiveVarRequest, Bolt12OfferDecodeRequest, Bolt12OfferDecodeResponse,
	Bolt12OfferReceiveRequest, Bolt12OfferReceiveVarRequest, Bolt12OfferResponse,
	Bolt12OfferSendRequest, Bolt12RefundDecodeRequest, Bolt12RefundDecodeResponse,
	Bolt12RefundInitiateRequest, Bolt12RefundInitiateResponse, Bolt12RefundRequestPaymentRequest,
	Bolt12RefundRequestPaymentResponse, ChannelDetailsExtendedDto, CloseChannelRequest,
	CustomTlvDto, EventDto, OkResponse, OpenChannelRequest, OpenChannelResponse, PaymentDetailsDto,
	PaymentWaitRequest, PaymentWaitResponse, PeerConnectRequest, PeerDetailsDto,
	PeerDisconnectRequest, SendResponse, SpontaneousSendRequest, StatusDto,
};

use owo_colors::OwoColorize;

use crate::cli::{
	ChannelCommand, Cli, ColorOpt, Command, CtxCommand, EventsCommand, InvoiceCommand,
	KeysendCommand, NodeCommand, OfferCommand, PayCommand, PeerCommand, RefundCommand,
	WalletCommand,
};
use crate::client::{join_url, send_json, send_json_allow_status, send_value};
use crate::utils::{confirm_or_exit, die, print_json, with_spinner};

struct App {
	base: String,
	output: ui::OutputMode,
	pretty: bool,
	theme: ui::Theme,
	yes: bool,
	no_truncate: bool,
	client: reqwest::Client,
}

fn truncate_id(s: &str) -> String {
	const HEAD: usize = 8;
	const TAIL: usize = 8;
	if s.len() <= HEAD + TAIL + 3 {
		return s.to_string();
	}
	format!("{}...{}", &s[..HEAD], &s[s.len() - TAIL..])
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

fn format_u64_with_commas(v: u64) -> String {
	format_sats_with_commas(v)
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

fn format_balance_sats(sats: u64, force_sats: bool) -> String {
	if force_sats {
		format!("{sats} sats")
	} else if sats >= 100_000_000 {
		format_btc_from_sats(sats)
	} else {
		format!("{} sats", format_sats_with_commas(sats))
	}
}

fn print_event_text(ev: &EventDto) {
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
		EventDto::PaymentReceived { payment_id, amount_msat } => {
			let pid = payment_id.as_deref().unwrap_or("-");
			println!(
				"PaymentReceived payment_id={pid} amount={} msat",
				format_u64_with_commas(*amount_msat)
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

fn resolve_connect(cli_connect: Option<String>, cfg: &config::CliConfig) -> String {
	if let Some(c) = cli_connect {
		return c;
	}
	if let Ok(u) = std::env::var("RGBLDK_URL") {
		if !u.trim().is_empty() {
			return u;
		}
	}
	if let Ok(u) = std::env::var("RGBLDK_CONNECT") {
		if !u.trim().is_empty() {
			return u;
		}
	}
	if let Ok(name) = std::env::var("RGBLDK_CTX") {
		let name = name.trim();
		if !name.is_empty() {
			if let Some(ctx) = cfg.contexts.get(name) {
				return ctx.url.clone();
			}
		}
	}
	if let Some(cur) = cfg.current.as_ref() {
		if let Some(ctx) = cfg.contexts.get(cur) {
			return ctx.url.clone();
		}
	}
	"http://127.0.0.1:8500".to_string()
}

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	let cfg_path = config::default_config_path().unwrap_or_else(|e| die(e));
	let mut cfg = config::load_config(&cfg_path).unwrap_or_else(|e| die(e));

	let base = resolve_connect(cli.connect, &cfg);
	let output = ui::resolve_output_mode(cli.output);
	let theme = ui::resolve_theme(match cli.color {
		ColorOpt::Auto => ui::ColorMode::Auto,
		ColorOpt::Always => ui::ColorMode::Always,
		ColorOpt::Never => ui::ColorMode::Never,
	});
	let app = App {
		base,
		output,
		pretty: cli.pretty,
		theme,
		yes: cli.yes,
		no_truncate: cli.no_truncate,
		client: reqwest::Client::new(),
	};

	match cli.command {
		Command::Ctx { command } => match command {
			CtxCommand::Ls => match app.output {
				ui::OutputMode::Json => print_json(&cfg, app.pretty),
				ui::OutputMode::Text => {
					let rows = cfg
						.contexts
						.iter()
						.map(|(name, c)| {
							vec![
								if cfg.current.as_deref() == Some(name) {
									"*".into()
								} else {
									"".into()
								},
								name.clone(),
								c.url.clone(),
							]
						})
						.collect::<Vec<_>>();
					ui::print_table(app.theme, &["Current", "Name", "URL"], rows);
				},
			},
			CtxCommand::Show => {
				if let Some(cur) = cfg.current.as_ref() {
					if let Some(ctx) = cfg.contexts.get(cur) {
						match app.output {
							ui::OutputMode::Json => print_json(&ctx, app.pretty),
							ui::OutputMode::Text => {
								println!("{cur} -> {}", ctx.url);
							},
						}
					} else {
						die(format!("current context {cur} not found in config"));
					}
				} else {
					die("no current context set");
				}
			},
			CtxCommand::Add(args) => {
				let name = args.name.clone();
				cfg.contexts.insert(args.name.clone(), config::CtxEntry { url: args.url });
				if args.use_now || cfg.current.is_none() {
					cfg.current = Some(args.name);
				}
				config::save_config(&cfg_path, &cfg).unwrap_or_else(|e| die(e));
				if app.output == ui::OutputMode::Text {
					let active = cfg.current.as_deref() == Some(name.as_str());
					if active {
						println!("Context \"{name}\" created and set as active.");
					} else {
						println!("Context \"{name}\" created.");
					}
					println!("Next: run `rgbldk node status` to verify the connection.");
				} else {
					print_json(&cfg, app.pretty);
				}
			},
			CtxCommand::Use { name } => {
				if !cfg.contexts.contains_key(&name) {
					die(format!("context not found: {name}"));
				}
				cfg.current = Some(name);
				config::save_config(&cfg_path, &cfg).unwrap_or_else(|e| die(e));
				if app.output == ui::OutputMode::Text {
					let cur = cfg.current.as_deref().unwrap_or("");
					println!("Switched to context \"{cur}\".");
					println!("Next: run `rgbldk node status` to verify the connection.");
				} else {
					print_json(&cfg, app.pretty);
				}
			},
			CtxCommand::Rm { name } => {
				cfg.contexts.remove(&name);
				if cfg.current.as_deref() == Some(&name) {
					cfg.current = None;
				}
				config::save_config(&cfg_path, &cfg).unwrap_or_else(|e| die(e));
				if app.output == ui::OutputMode::Text {
					println!("Context \"{name}\" removed.");
				} else {
					print_json(&cfg, app.pretty);
				}
			},
		},

		Command::Node { command } => match command {
			NodeCommand::Version => {
				let url = join_url(&app.base, "/api/v1/version");
				let v: serde_json::Value =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => {
						let mut rows = Vec::new();
						if let Some(obj) = v.as_object() {
							for (k, vv) in obj {
								rows.push(vec![
									k.clone(),
									vv.as_str().unwrap_or(&vv.to_string()).to_string(),
								]);
							}
						}
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
			NodeCommand::Health => {
				let url = join_url(&app.base, "/api/v1/healthz");
				let v: OkResponse = send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => {
						ui::print_checks(app.theme, "node health", v.ok, &v.checks)
					},
				}
			},
			NodeCommand::Ready => {
				let url = join_url(&app.base, "/api/v1/readyz");
				let v: OkResponse = send_json_allow_status(
					app.client.get(url),
					&[reqwest::StatusCode::SERVICE_UNAVAILABLE],
				)
				.await
				.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => {
						ui::print_checks(app.theme, "node ready", v.ok, &v.checks)
					},
				}
				if !v.ok {
					std::process::exit(1);
				}
			},
			NodeCommand::Status => {
				let url = join_url(&app.base, "/api/v1/status");
				let s: StatusDto = send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&s, app.pretty),
					ui::OutputMode::Text => {
						let rows = vec![
							vec!["is_running".into(), s.is_running.to_string()],
							vec!["p2p_is_listening".into(), s.is_listening.to_string()],
							vec!["best_block_height".into(), s.best_block_height.to_string()],
						];
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
			NodeCommand::Id => {
				let url = join_url(&app.base, "/api/v1/node_id");
				let v: serde_json::Value =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => {
						println!("{}", v.get("node_id").and_then(|v| v.as_str()).unwrap_or(""))
					},
				}
			},
			NodeCommand::Listen => {
				let url = join_url(&app.base, "/api/v1/listening_addresses");
				let v: serde_json::Value =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => {
						let addrs = v
							.get("addresses")
							.and_then(|v| v.as_array())
							.cloned()
							.unwrap_or_default();
						for a in addrs {
							if let Some(s) = a.as_str() {
								println!("{s}");
							}
						}
					},
				}
			},
		},

		Command::Wallet { command } => match command {
			WalletCommand::Balance(args) => {
				let url = join_url(&app.base, "/api/v1/balances");
				let b: BalancesDto =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&b, app.pretty),
					ui::OutputMode::Text => {
						let sats = args.sats;
						let rows = vec![
							vec![
								"On-chain (total)".into(),
								format_balance_sats(b.total_onchain_balance_sats, sats),
							],
							vec![
								"On-chain (spendable)".into(),
								format_balance_sats(b.spendable_onchain_balance_sats, sats),
							],
							vec![
								"Anchor reserve".into(),
								format_balance_sats(b.total_anchor_channels_reserve_sats, sats),
							],
							vec![
								"Lightning (total)".into(),
								format_balance_sats(b.total_lightning_balance_sats, sats),
							],
						];
						ui::print_table_with_right_align(
							app.theme,
							&["Asset", "Balance"],
							rows,
							&[1],
						);
					},
				}
			},
			WalletCommand::Address => {
				let url = join_url(&app.base, "/api/v1/wallet/new_address");
				let v: serde_json::Value =
					send_json(app.client.post(url).json(&serde_json::json!({})))
						.await
						.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => {
						println!("{}", v.get("address").and_then(|v| v.as_str()).unwrap_or(""))
					},
				}
			},
			WalletCommand::Sync => {
				if app.output == ui::OutputMode::Json {
					let url = join_url(&app.base, "/api/v1/wallet/sync");
					let v: serde_json::Value =
						send_json(app.client.post(url).json(&serde_json::json!({})))
							.await
							.unwrap_or_else(|e| die(e));
					print_json(&v, app.pretty);
				} else {
					let b0: BalancesDto =
						send_json(app.client.get(join_url(&app.base, "/api/v1/balances")))
							.await
							.unwrap_or_else(|e| die(e));
					let _v: serde_json::Value = send_json(
						app.client
							.post(join_url(&app.base, "/api/v1/wallet/sync"))
							.json(&serde_json::json!({})),
					)
					.await
					.unwrap_or_else(|e| die(e));
					let b1: BalancesDto =
						send_json(app.client.get(join_url(&app.base, "/api/v1/balances")))
							.await
							.unwrap_or_else(|e| die(e));

					let dt = b1.total_onchain_balance_sats as i128
						- b0.total_onchain_balance_sats as i128;
					let ds = b1.spendable_onchain_balance_sats as i128
						- b0.spendable_onchain_balance_sats as i128;
					let dl = b1.total_lightning_balance_sats as i128
						- b0.total_lightning_balance_sats as i128;

					println!("Wallet synced.");
					if dt == 0 && ds == 0 && dl == 0 {
						println!("No balance change.");
					} else {
						let fmt_delta = |d: i128| -> String {
							if d == 0 {
								"0 sats".to_string()
							} else if d > 0 {
								format!("+{}", format_balance_sats(d as u64, false))
							} else {
								format!("-{}", format_balance_sats((-d) as u64, false))
							}
						};
						println!(
							"Balance change: on-chain total {}, spendable {}, lightning {}.",
							fmt_delta(dt),
							fmt_delta(ds),
							fmt_delta(dl)
						);
					}
				}
			},
		},

		Command::Peer { command } => match command {
			PeerCommand::Ls => {
				let url = join_url(&app.base, "/api/v1/peers");
				let peers: Vec<PeerDetailsDto> =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&peers, app.pretty),
					ui::OutputMode::Text => {
						let rows = peers
							.into_iter()
							.map(|p| {
								vec![
									p.node_id,
									p.address,
									p.is_connected.to_string(),
									p.is_persisted.to_string(),
								]
							})
							.map(|mut r| {
								if !app.no_truncate {
									r[0] = truncate_id(&r[0]);
								}
								r
							})
							.collect::<Vec<_>>();
						ui::print_table(
							app.theme,
							&["Node ID", "Address", "Connected", "Persisted"],
							rows,
						);
					},
				}
			},
			PeerCommand::Connect(args) => {
				let req = PeerConnectRequest {
					node_id: args.node_id,
					address: args.addr,
					persist: args.persist,
				};
				let url = join_url(&app.base, "/api/v1/peers/connect");
				let v: serde_json::Value =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => println!("Peer connected."),
				}
			},
			PeerCommand::Disconnect { node_id } => {
				let req = PeerDisconnectRequest { node_id };
				let url = join_url(&app.base, "/api/v1/peers/disconnect");
				let v: serde_json::Value =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => println!("Peer disconnected."),
				}
			},
		},

		Command::Channel { command } => match command {
			ChannelCommand::Ls => {
				let url = join_url(&app.base, "/api/v1/channels");
				let chans: Vec<ChannelDetailsExtendedDto> =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&chans, app.pretty),
					ui::OutputMode::Text => {
						let rows = chans
							.into_iter()
							.map(|c| {
								vec![
									c.user_channel_id,
									c.counterparty_node_id,
									c.channel_value_sats.to_string(),
									c.is_channel_ready.to_string(),
									c.is_usable.to_string(),
								]
							})
							.map(|mut r| {
								if !app.no_truncate {
									r[0] = truncate_id(&r[0]);
									r[1] = truncate_id(&r[1]);
								}
								r
							})
							.collect::<Vec<_>>();
						ui::print_table(
							app.theme,
							&[
								"User Channel ID",
								"Counterparty",
								"Capacity (sats)",
								"Ready",
								"Usable",
							],
							rows,
						);
					},
				}
			},
			ChannelCommand::Open(args) => {
				let req = OpenChannelRequest {
					node_id: args.node_id,
					address: args.addr,
					channel_amount_sats: args.amount_sats,
					push_to_counterparty_msat: args.push_msat,
					announce: if args.private { Some(false) } else { None },
				};
				let url = join_url(&app.base, "/api/v1/channel/open");
				let resp: OpenChannelResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => println!("{}", resp.user_channel_id),
				}
			},
			ChannelCommand::Close(args) => {
				let req = CloseChannelRequest {
					user_channel_id: args.user_channel_id,
					counterparty_node_id: args.counterparty_node_id,
				};
				let url = join_url(&app.base, "/api/v1/channel/close");
				let v: serde_json::Value =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => println!("Channel close initiated."),
				}
			},
			ChannelCommand::ForceClose(args) => {
				confirm_or_exit(
					app.yes,
					"About to force-close a channel (potentially costly/time-locked).",
				);
				let req = CloseChannelRequest {
					user_channel_id: args.user_channel_id,
					counterparty_node_id: args.counterparty_node_id,
				};
				let url = join_url(&app.base, "/api/v1/channel/force_close");
				let v: serde_json::Value =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => println!("Channel force-close initiated."),
				}
			},
		},

		Command::Pay { command } => match command {
			PayCommand::Ls => {
				let url = join_url(&app.base, "/api/v1/payments");
				let ps: Vec<PaymentDetailsDto> =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&ps, app.pretty),
					ui::OutputMode::Text => {
						let rows = ps
							.into_iter()
							.map(|p| {
								let id =
									if app.no_truncate { p.id.clone() } else { truncate_id(&p.id) };
								vec![
									id,
									p.status,
									p.kind,
									p.direction,
									p.amount_msat
										.map(|v| format!("{}", format_u64_with_commas(v)))
										.unwrap_or_else(|| "-".into()),
									p.fee_paid_msat
										.map(|v| format!("{}", format_u64_with_commas(v)))
										.unwrap_or_else(|| "-".into()),
								]
							})
							.collect::<Vec<_>>();
						ui::print_table(
							app.theme,
							&["ID", "Status", "Kind", "Dir", "Amount (msat)", "Fee (msat)"],
							rows,
						);
					},
				}
			},
			PayCommand::Wait(args) => {
				use reqwest::StatusCode as HttpStatus;

				let url = join_url(&app.base, &format!("/api/v1/payment/{}/wait", args.payment_id));
				let req = PaymentWaitRequest { timeout_secs: args.timeout_secs };
				let (status, v) =
					send_value(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));

				match (status, app.output) {
					(HttpStatus::OK, ui::OutputMode::Json) => {
						let resp: PaymentWaitResponse =
							serde_json::from_value(v).unwrap_or_else(|e| die(e.to_string()));
						print_json(&resp, app.pretty);
					},
					(HttpStatus::OK, ui::OutputMode::Text) => {
						let resp: PaymentWaitResponse =
							serde_json::from_value(v).unwrap_or_else(|e| die(e.to_string()));
						ui::print_checks(app.theme, "Payment wait", resp.ok, &resp.checks);
						println!("{}", resp.payment.id);
					},
					(_, ui::OutputMode::Json) => {
						print_json(&v, app.pretty);
					},
					(_, ui::OutputMode::Text) => {
						let err =
							v.get("error").and_then(|e| e.as_str()).unwrap_or("request failed");
						if app.theme.color {
							eprintln!("{}", format!("{err}").red());
						} else {
							eprintln!("{err}");
						}
						if let Some(checks) = v.get("checks") {
							if let Ok(checks) = serde_json::from_value::<
								Vec<rgbldk_api::http::HealthCheckDto>,
							>(checks.clone())
							{
								ui::print_checks(app.theme, "Details", false, &checks);
							}
						}
						if let Some(payment) = v.get("payment") {
							if let Ok(p) =
								serde_json::from_value::<PaymentDetailsDto>(payment.clone())
							{
								println!("{}", p.id);
							}
						}
					},
				}
			},
			PayCommand::Abandon { payment_id } => {
				let url = join_url(&app.base, &format!("/api/v1/payment/{payment_id}/abandon"));
				let resp: OkResponse = send_json(app.client.post(url).json(&serde_json::json!({})))
					.await
					.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						ui::print_checks(app.theme, "Abandon payment", resp.ok, &resp.checks);
					},
				}
			},
			PayCommand::Get { payment_id } => {
				let url = join_url(&app.base, &format!("/api/v1/payment/{}", payment_id));
				let p: PaymentDetailsDto =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&p, app.pretty),
					ui::OutputMode::Text => {
						let status_value = match p.status.as_str() {
							"Succeeded" => {
								if app.theme.color {
									format!("{}", "✔ Succeeded".green())
								} else {
									"✔ Succeeded".to_string()
								}
							},
							"Pending" => {
								if app.theme.color {
									format!("{}", "… Pending".yellow())
								} else {
									"… Pending".to_string()
								}
							},
							"Failed" => {
								if app.theme.color {
									format!("{}", "✘ Failed".red())
								} else {
									"✘ Failed".to_string()
								}
							},
							_ => p.status.clone(),
						};
						let rows = vec![
							vec!["id".into(), p.id],
							vec!["direction".into(), p.direction],
							vec!["status".into(), status_value],
							vec!["kind".into(), p.kind],
							vec![
								"kind_details".into(),
								p.kind_details
									.as_ref()
									.map(|v| v.to_string())
									.unwrap_or_else(|| "-".into()),
							],
							vec![
								"amount (msat)".into(),
								p.amount_msat
									.map(|v| format!("{} msat", format_u64_with_commas(v)))
									.unwrap_or_else(|| "-".into()),
							],
							vec![
								"fee paid (msat)".into(),
								p.fee_paid_msat
									.map(|v| format!("{} msat", format_u64_with_commas(v)))
									.unwrap_or_else(|| "-".into()),
							],
						];
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
			PayCommand::Invoice { command } => match command {
				InvoiceCommand::Create(args) => {
					let url;
					let body;
					if let Some(amount_msat) = args.amount_msat {
						url = join_url(&app.base, "/api/v1/bolt11/receive");
						body = serde_json::to_value(Bolt11ReceiveRequest {
							amount_msat,
							description: args.desc,
							expiry_secs: args.expiry_secs,
						})
						.unwrap();
					} else {
						url = join_url(&app.base, "/api/v1/bolt11/receive_var");
						body = serde_json::to_value(Bolt11ReceiveVarRequest {
							description: args.desc,
							expiry_secs: args.expiry_secs,
						})
						.unwrap();
					}
					let resp: Bolt11ReceiveResponse = send_json(app.client.post(url).json(&body))
						.await
						.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => println!("{}", resp.invoice),
					}
				},
				InvoiceCommand::Pay(args) => {
					let req =
						Bolt11PayRequest { invoice: args.invoice, amount_msat: args.amount_msat };
					let url = join_url(&app.base, "/api/v1/bolt11/pay");
					let resp: Bolt11PayResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => println!("{}", resp.payment_id),
					}
				},
			},
			PayCommand::Offer { command } => match command {
				OfferCommand::Create(args) => {
					let expiry_secs = if args.no_expiry { None } else { Some(args.expiry_secs) };
					let (url, body) = if let Some(amount_msat) = args.amount_msat {
						(
							join_url(&app.base, "/api/v1/bolt12/offer/receive"),
							serde_json::to_value(Bolt12OfferReceiveRequest {
								amount_msat,
								description: args.desc,
								expiry_secs,
								quantity: args.quantity,
							})
							.unwrap(),
						)
					} else {
						(
							join_url(&app.base, "/api/v1/bolt12/offer/receive_var"),
							serde_json::to_value(Bolt12OfferReceiveVarRequest {
								description: args.desc,
								expiry_secs,
							})
							.unwrap(),
						)
					};
					let resp: Bolt12OfferResponse = send_json(app.client.post(url).json(&body))
						.await
						.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => println!("{}", resp.offer),
					}
				},
				OfferCommand::Decode { offer } => {
					let url = join_url(&app.base, "/api/v1/bolt12/offer/decode");
					let req = Bolt12OfferDecodeRequest { offer };
					let resp: Bolt12OfferDecodeResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => {
							let rows = vec![
								vec!["offer_id".into(), resp.offer_id],
								vec![
									"signing_pubkey".into(),
									resp.signing_pubkey.unwrap_or_else(|| "-".into()),
								],
								vec![
									"description".into(),
									resp.description.unwrap_or_else(|| "-".into()),
								],
								vec!["issuer".into(), resp.issuer.unwrap_or_else(|| "-".into())],
								vec![
									"amount_msat".into(),
									resp.amount_msat
										.map(|v| format_u64_with_commas(v))
										.unwrap_or_else(|| "-".into()),
								],
								vec![
									"absolute_expiry_unix_secs".into(),
									resp.absolute_expiry_unix_secs
										.map(|v| v.to_string())
										.unwrap_or_else(|| "-".into()),
								],
								vec!["paths_count".into(), resp.paths_count.to_string()],
								vec!["expects_quantity".into(), resp.expects_quantity.to_string()],
								vec!["chain_hashes".into(), resp.chain_hashes.join(", ")],
							];
							ui::print_table(app.theme, &["Field", "Value"], rows);
						},
					}
				},
				OfferCommand::Pay(args) => {
					let url = join_url(&app.base, "/api/v1/bolt12/offer/send");
					let req = Bolt12OfferSendRequest {
						offer: args.offer,
						amount_msat: args.amount_msat,
						quantity: args.quantity,
						payer_note: args.payer_note,
					};
					let resp: SendResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => println!("{}", resp.payment_id),
					}
				},
			},
			PayCommand::Refund { command } => match command {
				RefundCommand::Initiate(args) => {
					let url = join_url(&app.base, "/api/v1/bolt12/refund/initiate");
					let req = Bolt12RefundInitiateRequest {
						amount_msat: args.amount_msat,
						expiry_secs: args.expiry_secs,
						quantity: args.quantity,
						payer_note: args.payer_note,
					};
					let resp: Bolt12RefundInitiateResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => {
							println!("{}", resp.refund);
							eprintln!("payment_id: {}", resp.payment_id);
						},
					}
				},
				RefundCommand::Decode { refund } => {
					let url = join_url(&app.base, "/api/v1/bolt12/refund/decode");
					let req = Bolt12RefundDecodeRequest { refund };
					let resp: Bolt12RefundDecodeResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => {
							let rows = vec![
								vec!["description".into(), resp.description],
								vec!["issuer".into(), resp.issuer.unwrap_or_else(|| "-".into())],
								vec![
									"amount_msat".into(),
									format_u64_with_commas(resp.amount_msat),
								],
								vec![
									"absolute_expiry_unix_secs".into(),
									resp.absolute_expiry_unix_secs
										.map(|v| v.to_string())
										.unwrap_or_else(|| "-".into()),
								],
								vec!["chain_hash".into(), resp.chain_hash],
								vec!["payer_signing_pubkey".into(), resp.payer_signing_pubkey],
								vec![
									"payer_note".into(),
									resp.payer_note.unwrap_or_else(|| "-".into()),
								],
								vec![
									"quantity".into(),
									resp.quantity
										.map(|v| v.to_string())
										.unwrap_or_else(|| "-".into()),
								],
								vec!["paths_count".into(), resp.paths_count.to_string()],
							];
							ui::print_table(app.theme, &["Field", "Value"], rows);
						},
					}
				},
				RefundCommand::RequestPayment { refund } => {
					let url = join_url(&app.base, "/api/v1/bolt12/refund/request_payment");
					let req = Bolt12RefundRequestPaymentRequest { refund };
					let resp: Bolt12RefundRequestPaymentResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => {
							println!("{}", resp.invoice);
							eprintln!("payment_id: {}", resp.payment_id);
						},
					}
				},
			},
			PayCommand::Keysend { command } => match command {
				KeysendCommand::Send(args) => {
					let tlvs: Vec<CustomTlvDto> = args
						.tlv
						.into_iter()
						.map(|t| CustomTlvDto { r#type: t.r#type, value_hex: t.value_hex })
						.collect();
					let req = SpontaneousSendRequest {
						counterparty_node_id: args.node_id,
						amount_msat: args.amount_msat,
						custom_tlvs: tlvs,
					};
					let url = join_url(&app.base, "/api/v1/spontaneous/send");
					let resp: SendResponse =
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
					match app.output {
						ui::OutputMode::Json => print_json(&resp, app.pretty),
						ui::OutputMode::Text => println!("{}", resp.payment_id),
					}
				},
			},
		},

		Command::Events { command } => match command {
			EventsCommand::Next => {
				let url = join_url(&app.base, "/api/v1/events/wait_next");
				let ev: EventDto = with_spinner(
					app.output == ui::OutputMode::Text,
					"Waiting for event...",
					send_json(app.client.post(url).json(&serde_json::json!({}))),
				)
				.await
				.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&ev, app.pretty),
					ui::OutputMode::Text => print_event_text(&ev),
				}
			},
			EventsCommand::Handled => {
				let url = join_url(&app.base, "/api/v1/events/handled");
				let v: serde_json::Value =
					send_json(app.client.post(url).json(&serde_json::json!({})))
						.await
						.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => println!("Marked handled."),
				}
			},
			EventsCommand::Watch(args) => {
				let mut seen: u64 = 0;
				loop {
					if let Some(max) = args.count {
						if seen >= max {
							break;
						}
					}
					let url = join_url(&app.base, "/api/v1/events/wait_next");
					let ev: EventDto = with_spinner(
						app.output == ui::OutputMode::Text,
						"Waiting for events...",
						send_json(app.client.post(url).json(&serde_json::json!({}))),
					)
					.await
					.unwrap_or_else(|e| die(e));
					let _ = send_json::<serde_json::Value>(
						app.client
							.post(join_url(&app.base, "/api/v1/events/handled"))
							.json(&serde_json::json!({})),
					)
					.await;

					match app.output {
						ui::OutputMode::Json => print_json(&ev, app.pretty),
						ui::OutputMode::Text => print_event_text(&ev),
					}
					seen += 1;
				}
			},
		},
	}
}
