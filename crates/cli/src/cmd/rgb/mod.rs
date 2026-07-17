mod utxos;

use rgbldk_http_client::dto::{
	OkResponse, RgbContractBalanceResponse, RgbContractKnownResponse, RgbContractsExportRequest,
	RgbContractsExportResponse, RgbContractsImportResponse, RgbContractsIssueRequest,
	RgbContractsIssueResponse, RgbContractsResponse, RgbDescriptorResponse,
	RgbIssuersImportResponse, RgbIssuersResponse, RgbLnInvoiceCreateForHashRequest,
	RgbLnInvoiceCreateRequest, RgbLnInvoiceDecodeRequest, RgbLnInvoiceDecodeResponse,
	RgbLnInvoiceResponse, RgbLnPayRequest, RgbNewAddressResponse, RgbOnchainInvoiceCreateRequest,
	RgbOnchainInvoiceDecodeRequest, RgbOnchainInvoiceDecodeResponse, RgbOnchainInvoiceResponse,
	RgbOnchainPaymentsResponse, RgbOnchainReceiveRequest, RgbOnchainReceiveResponse,
	RgbOnchainSendRequest, RgbOnchainSendResponse, RgbSignMessageAlgorithmDto,
	RgbSignMessageEncodingDto, RgbSignMessageRequest, RgbSignMessageResponse, SendResponse,
};
use serde::{Deserialize, Serialize};

use crate::app::App;
use crate::cli::{
	RgbCommand, RgbConsignmentsCommand, RgbContractsCommand, RgbIssuersCommand, RgbLnCommand,
	RgbLnInvoiceCommand, RgbOnchainCommand, RgbSignMessageAlgorithmOpt, RgbSignMessageArgs,
	RgbSignMessageEncodingOpt,
};
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RgbLnCarrierEstimateChannelView {
	channel_id: String,
	user_channel_id: String,
	is_usable: bool,
	inbound_capacity_msat: String,
	inbound_htlc_minimum_msat: String,
	#[serde(default)]
	inbound_htlc_maximum_msat: Option<String>,
	local_balance_output_sats: String,
	has_holder_reserve: bool,
	receive_available: bool,
	#[serde(default)]
	can_receive_rgb_invoice: bool,
	#[serde(default)]
	blocking_reason: Option<String>,
	#[serde(default)]
	required_carrier_msat: Option<String>,
	#[serde(default)]
	required_carrier_reason: Option<String>,
	#[serde(default)]
	available_inbound_capacity_msat: Option<String>,
	#[serde(default)]
	suggested_action: Option<String>,
	minimum_viable_carrier_amount_msat: Option<String>,
	minimum_viable_reason: Option<String>,
	default_create_carrier_amount_msat: Option<String>,
	default_create_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RgbLnCarrierEstimateView {
	receive_available: bool,
	#[serde(default)]
	can_create_rgb_invoice: bool,
	#[serde(default)]
	blocking_reason: Option<String>,
	#[serde(default)]
	required_carrier_msat: Option<String>,
	#[serde(default)]
	required_carrier_reason: Option<String>,
	#[serde(default)]
	available_inbound_capacity_msat: Option<String>,
	#[serde(default)]
	suggested_action: Option<String>,
	minimum_viable_carrier_amount_msat: String,
	minimum_viable_reason: String,
	default_create_carrier_amount_msat: String,
	default_create_reason: String,
	carrier_admission_threshold_msat: String,
	minimum_allowed_carrier_amount_msat: String,
	holder_reserve_threshold_msat: String,
	channels: Vec<RgbLnCarrierEstimateChannelView>,
	estimate_only: bool,
	warning: String,
}

pub(crate) async fn handle(app: &App, command: &RgbCommand) {
	match command {
		RgbCommand::Sync => {
			let url = join_url(&app.base, "/api/v1/rgb/sync");
			let resp: OkResponse = send_json(app.client.post(url).json(&serde_json::json!({})))
				.await
				.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					ui::print_checks(app.theme, "RGB sync", resp.ok, &resp.checks)
				},
			}
		},
		RgbCommand::Address => {
			let url = join_url(&app.base, "/api/v1/rgb/new_address");
			let resp: RgbNewAddressResponse =
				send_json(app.client.post(url).json(&serde_json::json!({})))
					.await
					.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => println!("{}", resp.address),
			}
		},
		RgbCommand::Utxos { command } => utxos::handle(app, command).await,
		RgbCommand::Descriptor => handle_descriptor(app).await,
		RgbCommand::SignMessage(args) => handle_sign_message(app, args).await,
		RgbCommand::Consignments { command } => match command {
			RgbConsignmentsCommand::Download(args) => {
				let url = join_url(
					&app.base,
					&format!("/api/v1/rgb/consignments/{}?format={}", args.key, args.format),
				);
				let resp = app.client.get(url).send().await.unwrap_or_else(|e| die(e.to_string()));
				let status = resp.status();
				let body = resp.bytes().await.unwrap_or_else(|e| die(e.to_string()));
				ensure_success_or_die(status, &body);
				write_bytes_or_die(&args.out, &body);

				match app.output {
					ui::OutputMode::Json => {
						let v = serde_json::json!({
							"ok": true,
							"key": args.key,
							"out": args.out,
							"bytes": body.len(),
						});
						print_json(&v, app.pretty)
					},
					ui::OutputMode::Text => {
						println!("Downloaded consignment {}.", args.key);
						println!("Saved to {}.", args.out);
					},
				}
			},
		},
		RgbCommand::Contracts { command } => handle_contracts(app, command).await,
		RgbCommand::Issuers { command } => handle_issuers(app, command).await,
		RgbCommand::Ln { command } => handle_ln(app, command).await,
		RgbCommand::Onchain { command } => handle_onchain(app, command).await,
	}
}

pub(super) fn ensure_success_or_die(status: reqwest::StatusCode, body: &[u8]) {
	if status.is_success() {
		return;
	}
	let msg =
		error_from_body(body).unwrap_or_else(|| format!("request failed with status {status}"));
	die(msg);
}

pub(super) fn write_bytes_or_die(out: &str, body: &[u8]) {
	std::fs::write(out, body).unwrap_or_else(|e| die(format!("failed to write {out}: {e}")));
}

fn error_from_body(body: &[u8]) -> Option<String> {
	let v = serde_json::from_slice::<serde_json::Value>(body).ok()?;
	let error = v.get("error").and_then(|e| e.as_str()).unwrap_or("request failed");
	let message = v.get("message").and_then(|m| m.as_str());
	let hint = v.get("hint").and_then(|h| h.as_str());
	let detail = v.get("detail");
	let checks = v.get("checks");

	let mut parts = vec![error.to_string()];
	if let Some(message) = message.filter(|m| *m != error) {
		parts.push(message.to_string());
	}
	if let Some(detail) = detail {
		parts.push(format!("Details: {detail}"));
	}
	if let Some(hint) = hint {
		parts.push(format!("Hint: {hint}"));
	}
	if let Some(checks) = checks {
		parts.push(format!("Checks: {checks}"));
	}
	Some(parts.join("\n"))
}

async fn handle_descriptor(app: &App) {
	let url = join_url(&app.base, "/api/v1/rgb/descriptor");
	let resp: RgbDescriptorResponse =
		send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => {
			println!("RGB wallet descriptor:");
			println!("{}", resp.descriptor);

			if !resp.derived_descriptors.is_empty() {
				println!();
				let rows = resp
					.derived_descriptors
					.into_iter()
					.map(|descriptor| {
						vec![
							descriptor.fingerprint,
							descriptor.derivation_path,
							descriptor.xpub,
							descriptor.descriptor,
						]
					})
					.collect::<Vec<_>>();
				ui::print_table(
					app.theme,
					&["Fingerprint", "Derivation Path", "Xpub", "Descriptor"],
					rows,
				);
			}
		},
	}
}

async fn handle_sign_message(app: &App, args: &RgbSignMessageArgs) {
	if matches!(args.algorithm, Some(RgbSignMessageAlgorithmOpt::Ecdsa)) && args.encoding.is_none()
	{
		die("ECDSA signing requires --encoding hex or --encoding base64");
	}

	let url = join_url(&app.base, "/api/v1/rgb/sign_message");
	let req = RgbSignMessageRequest {
		message: args.message.clone(),
		algorithm: args.algorithm.map(map_sign_message_algorithm),
		compact: args.compact.then_some(true),
		encoding: args.encoding.map(map_sign_message_encoding),
	};
	let resp: RgbSignMessageResponse =
		send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
	match app.output {
		ui::OutputMode::Json => print_json(&resp, app.pretty),
		ui::OutputMode::Text => {
			println!("Message signed with {}.", resp.algorithm);

			let mut rows = vec![
				vec!["Public key".into(), resp.pubkey],
				vec!["Derivation path".into(), resp.derivation_path],
			];
			if let Some(encoding) = resp.encoding {
				rows.push(vec!["Encoding".into(), encoding]);
			}
			if let Some(compact) = resp.compact {
				rows.push(vec![
					"Format".into(),
					if compact { "Compact".into() } else { "Standard".into() },
				]);
			}
			if let Some(digest_hex) = resp.digest_hex {
				rows.push(vec!["Digest".into(), digest_hex]);
			}
			ui::print_table(app.theme, &["Field", "Value"], rows);
			println!();
			println!("Signature:");
			println!("{}", resp.signature);
		},
	}
}

fn map_sign_message_algorithm(algorithm: RgbSignMessageAlgorithmOpt) -> RgbSignMessageAlgorithmDto {
	match algorithm {
		RgbSignMessageAlgorithmOpt::BitcoinSignedMessage => {
			RgbSignMessageAlgorithmDto::BitcoinSignedMessage
		},
		RgbSignMessageAlgorithmOpt::Ecdsa => RgbSignMessageAlgorithmDto::Ecdsa,
	}
}

fn map_sign_message_encoding(encoding: RgbSignMessageEncodingOpt) -> RgbSignMessageEncodingDto {
	match encoding {
		RgbSignMessageEncodingOpt::Hex => RgbSignMessageEncodingDto::Hex,
		RgbSignMessageEncodingOpt::Base64 => RgbSignMessageEncodingDto::Base64,
	}
}

async fn handle_contracts(app: &App, command: &RgbContractsCommand) {
	match command {
		RgbContractsCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/rgb/contracts");
			let resp: RgbContractsResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let rows = resp
						.contracts
						.into_iter()
						.map(|c| {
							vec![
								c.name.unwrap_or_else(|| "-".into()),
								c.ticker.unwrap_or_else(|| "-".into()),
								c.precision.map(|p| p.to_string()).unwrap_or_else(|| "-".into()),
								c.issued_supply
									.map(ui::format_u64_with_commas)
									.unwrap_or_else(|| "-".into()),
								c.contract_id,
							]
						})
						.collect::<Vec<_>>();
					ui::print_table_with_right_align(
						app.theme,
						&["Name", "Ticker", "Precision", "Issued", "Contract ID"],
						rows,
						&[2, 3],
					);
				},
			}
		},
		RgbContractsCommand::Balance { contract_id } => {
			let url = join_url(&app.base, &format!("/api/v1/rgb/contract/{}/balance", contract_id));
			let resp: RgbContractBalanceResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let b = resp.balance;
					let rows = vec![
						vec!["Contract".into(), resp.contract_id],
						vec!["Mined".into(), ui::format_u64_with_commas(b.mined)],
						vec!["Tentative".into(), ui::format_u64_with_commas(b.tentative)],
						vec!["Offchain".into(), ui::format_u64_with_commas(b.offchain)],
						vec!["Archived".into(), ui::format_u64_with_commas(b.archived)],
						vec!["Total".into(), ui::format_u64_with_commas(b.total)],
					];
					ui::print_table(app.theme, &["Field", "Value"], rows);
				},
			}
		},
		RgbContractsCommand::Known { contract_id } => {
			let url = join_url(&app.base, &format!("/api/v1/rgb/contract/{}/known", contract_id));
			let resp: RgbContractKnownResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					if resp.known {
						println!("The node knows contract {}.", resp.contract_id);
					} else {
						println!("The node does not know contract {}.", resp.contract_id);
						std::process::exit(1);
					}
				},
			}
		},
		RgbContractsCommand::Import(args) => {
			let bytes = std::fs::read(&args.file)
				.unwrap_or_else(|e| die(format!("failed to read file {}: {e}", args.file)));
			let url = join_url(
				&app.base,
				&format!(
					"/api/v1/rgb/contracts/import?contract_id={}&format={}",
					args.contract_id, args.format
				),
			);
			let resp: RgbContractsImportResponse = send_json(
				app.client.post(url).header("content-type", "application/octet-stream").body(bytes),
			)
			.await
			.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					ui::print_checks(app.theme, "RGB contract import", resp.ok, &resp.checks);
					println!("Imported contract {}.", resp.contract_id);
					println!("Consignment key: {}", resp.consignment_key);
				},
			}
		},
		RgbContractsCommand::Issue(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/contracts/issue");
			let req = RgbContractsIssueRequest {
				issuer_name: args.issuer_name.clone(),
				contract_name: args.contract_name.clone(),
				ticker: args.ticker.clone(),
				precision: args.precision,
				issued_supply: args.issued_supply,
				utxo: args.utxo.clone(),
			};
			let resp: RgbContractsIssueResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					ui::print_checks(app.theme, "RGB contract issue", resp.ok, &resp.checks);
					println!("Issued contract {}.", resp.contract_id);
					println!("Supply: {}", ui::format_u64_with_commas(resp.issued_supply));
				},
			}
		},
		RgbContractsCommand::Export(args) => {
			let export_req = RgbContractsExportRequest { contract_id: args.contract_id.clone() };
			if args.direct {
				let export_url = join_url(
					&app.base,
					&format!("/api/v1/rgb/contracts/export?download=true&format={}", args.format),
				);
				let resp = app
					.client
					.post(export_url)
					.json(&export_req)
					.send()
					.await
					.unwrap_or_else(|e| die(e.to_string()));
				let status = resp.status();
				let headers = resp.headers().clone();
				let body = resp.bytes().await.unwrap_or_else(|e| die(e.to_string()));
				ensure_success_or_die(status, &body);
				write_bytes_or_die(&args.out, &body);

				let contract_id_hdr =
					headers.get("x-rgb-contract-id").and_then(|v| v.to_str().ok());
				let consignment_key_hdr =
					headers.get("x-rgb-consignment-key").and_then(|v| v.to_str().ok());

				match app.output {
					ui::OutputMode::Json => {
						let v = serde_json::json!({
							"ok": true,
							"contract_id": contract_id_hdr,
							"consignment_key": consignment_key_hdr,
							"out": args.out,
							"bytes": body.len(),
						});
						print_json(&v, app.pretty)
					},
					ui::OutputMode::Text => {
						println!("Exported contract {}.", contract_id_hdr.unwrap_or("<unknown>"));
						if let Some(k) = consignment_key_hdr {
							println!("Consignment key: {k}");
						}
						println!("Saved to {}.", args.out);
					},
				}
			} else {
				let export_url = join_url(&app.base, "/api/v1/rgb/contracts/export");
				let export_resp: RgbContractsExportResponse =
					send_json(app.client.post(export_url).json(&export_req))
						.await
						.unwrap_or_else(|e| die(e));

				let get_url = join_url(
					&app.base,
					&format!(
						"/api/v1/rgb/consignments/{}?format={}",
						export_resp.consignment_key, args.format
					),
				);
				let bytes =
					app.client.get(get_url).send().await.unwrap_or_else(|e| die(e.to_string()));
				let status = bytes.status();
				let body = bytes.bytes().await.unwrap_or_else(|e| die(e.to_string()));
				ensure_success_or_die(status, &body);
				write_bytes_or_die(&args.out, &body);

				match app.output {
					ui::OutputMode::Json => {
						let v = serde_json::json!({
							"export": export_resp,
							"out": args.out,
							"bytes": body.len(),
						});
						print_json(&v, app.pretty)
					},
					ui::OutputMode::Text => {
						ui::print_checks(
							app.theme,
							"RGB contract export",
							export_resp.ok,
							&export_resp.checks,
						);
						println!("Exported contract {}.", export_resp.contract_id);
						println!("Consignment key: {}", export_resp.consignment_key);
						println!("Saved to {}.", args.out);
					},
				}
			}
		},
	}
}

async fn handle_issuers(app: &App, command: &RgbIssuersCommand) {
	match command {
		RgbIssuersCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/rgb/issuers");
			let resp: RgbIssuersResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let rows = resp.issuers.into_iter().map(|i| vec![i]).collect::<Vec<_>>();
					ui::print_table(app.theme, &["Issuer"], rows);
				},
			}
		},
		RgbIssuersCommand::Import(args) => {
			let bytes = std::fs::read(&args.file)
				.unwrap_or_else(|e| die(format!("failed to read file {}: {e}", args.file)));
			let url = join_url(
				&app.base,
				&format!("/api/v1/rgb/issuers/import?name={}&format={}", args.name, args.format),
			);
			let resp: RgbIssuersImportResponse = send_json(
				app.client.post(url).header("content-type", "application/octet-stream").body(bytes),
			)
			.await
			.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					ui::print_checks(app.theme, "RGB issuer import", resp.ok, &resp.checks);
					println!("Imported issuer {}.", resp.issuer_name);
				},
			}
		},
	}
}

async fn handle_ln(app: &App, command: &RgbLnCommand) {
	match command {
		RgbLnCommand::Invoice { command } => match command {
			RgbLnInvoiceCommand::EstimateCarrier => {
				let url = join_url(&app.base, "/api/v1/rgb/ln/invoice/estimate_carrier");
				let resp: RgbLnCarrierEstimateView =
					send_json(app.client.post(url).json(&serde_json::json!({})))
						.await
						.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => print_rgb_ln_carrier_estimate(app, &resp),
				}
			},
			RgbLnInvoiceCommand::Create(args) => {
				let url = join_url(&app.base, "/api/v1/rgb/ln/invoice/create");
				let req = RgbLnInvoiceCreateRequest {
					contract_id: args.contract_id.clone(),
					asset_amount: args.asset_amount,
					description: args.desc.clone(),
					expiry_secs: Some(args.expiry_secs),
					btc_carrier_amount_msat: args.btc_carrier_amount_msat,
				};
				let resp: RgbLnInvoiceResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => println!("{}", resp.invoice),
				}
			},
			RgbLnInvoiceCommand::CreateForHash(args) => {
				let url = join_url(&app.base, "/api/v1/rgb/ln/invoice/create_for_hash");
				let req = RgbLnInvoiceCreateForHashRequest {
					contract_id: args.contract_id.clone(),
					asset_amount: args.asset_amount,
					payment_hash: args.payment_hash.clone(),
					description: args.desc.clone(),
					expiry_secs: Some(args.expiry_secs),
					btc_carrier_amount_msat: args.btc_carrier_amount_msat,
				};
				let resp: RgbLnInvoiceResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => println!("{}", resp.invoice),
				}
			},
			RgbLnInvoiceCommand::Decode { invoice } => {
				let url = join_url(&app.base, "/api/v1/rgb/ln/invoice/decode");
				let req = RgbLnInvoiceDecodeRequest { invoice: invoice.clone() };
				let resp: RgbLnInvoiceDecodeResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						let rows = vec![
							vec!["Payment hash".into(), resp.payment_hash],
							vec!["Destination".into(), resp.destination],
							vec![
								"Carrier amount (msat)".into(),
								resp.carrier_amount_msat
									.map(ui::format_u64_with_commas)
									.unwrap_or_else(|| "-".into()),
							],
							vec!["Expiry (secs)".into(), resp.expiry_secs.to_string()],
							vec!["Contract".into(), resp.contract_id.unwrap_or_else(|| "-".into())],
							vec![
								"Asset amount".into(),
								resp.asset_amount
									.map(ui::format_u64_with_commas)
									.unwrap_or_else(|| "-".into()),
							],
						];
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
		},
		RgbLnCommand::Pay(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/ln/pay");
			let req = RgbLnPayRequest {
				invoice: args.invoice.clone(),
				contract_id: args.contract_id.clone(),
				asset_amount: args.asset_amount,
			};
			let resp: SendResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => println!("{}", resp.payment_id),
			}
		},
	}
}

fn yes_no(value: bool) -> String {
	if value { "yes" } else { "no" }.to_string()
}

fn format_decimal(value: &str) -> String {
	value.parse::<u64>().map(ui::format_u64_with_commas).unwrap_or_else(|_| value.to_string())
}

fn format_opt_msat(value: Option<&str>) -> String {
	value.map(format_decimal).unwrap_or_else(|| "-".to_string())
}

fn print_rgb_ln_carrier_estimate(app: &App, resp: &RgbLnCarrierEstimateView) {
	let summary_rows = vec![
		vec!["Receive available".into(), yes_no(resp.receive_available)],
		vec!["Can create RGB invoice".into(), yes_no(resp.can_create_rgb_invoice)],
		vec!["Blocking reason".into(), resp.blocking_reason.clone().unwrap_or_else(|| "-".into())],
		vec![
			"Required carrier (msat)".into(),
			format_opt_msat(resp.required_carrier_msat.as_deref()),
		],
		vec![
			"Required carrier reason".into(),
			resp.required_carrier_reason.clone().unwrap_or_else(|| "-".into()),
		],
		vec![
			"Available inbound (msat)".into(),
			format_opt_msat(resp.available_inbound_capacity_msat.as_deref()),
		],
		vec![
			"Suggested action".into(),
			resp.suggested_action.clone().unwrap_or_else(|| "-".into()),
		],
		vec![
			"Minimum viable carrier (msat)".into(),
			format_decimal(&resp.minimum_viable_carrier_amount_msat),
		],
		vec!["Minimum viable reason".into(), resp.minimum_viable_reason.clone()],
		vec![
			"Default create carrier (msat)".into(),
			format_decimal(&resp.default_create_carrier_amount_msat),
		],
		vec!["Default create reason".into(), resp.default_create_reason.clone()],
		vec![
			"Admission threshold (msat)".into(),
			format_decimal(&resp.carrier_admission_threshold_msat),
		],
		vec![
			"Minimum allowed carrier (msat)".into(),
			format_decimal(&resp.minimum_allowed_carrier_amount_msat),
		],
		vec![
			"Holder reserve threshold (msat)".into(),
			format_decimal(&resp.holder_reserve_threshold_msat),
		],
		vec!["Estimate only".into(), yes_no(resp.estimate_only)],
	];
	ui::print_table(app.theme, &["Field", "Value"], summary_rows);

	if !resp.channels.is_empty() {
		let rows = resp
			.channels
			.iter()
			.map(|c| {
				vec![
					c.channel_id.clone(),
					c.user_channel_id.clone(),
					yes_no(c.is_usable),
					format_decimal(&c.inbound_capacity_msat),
					format_decimal(&c.local_balance_output_sats),
					yes_no(c.has_holder_reserve),
					yes_no(c.receive_available),
					yes_no(c.can_receive_rgb_invoice),
					format_opt_msat(c.required_carrier_msat.as_deref()),
					format_opt_msat(c.available_inbound_capacity_msat.as_deref()),
					c.blocking_reason.clone().unwrap_or_else(|| "-".to_string()),
					c.suggested_action.clone().unwrap_or_else(|| "-".to_string()),
					format_opt_msat(c.minimum_viable_carrier_amount_msat.as_deref()),
					c.minimum_viable_reason.clone().unwrap_or_else(|| "-".to_string()),
					format_opt_msat(c.default_create_carrier_amount_msat.as_deref()),
					c.default_create_reason.clone().unwrap_or_else(|| "-".to_string()),
				]
			})
			.collect::<Vec<_>>();
		ui::print_table(
			app.theme,
			&[
				"Channel",
				"User Channel",
				"Usable",
				"Inbound msat",
				"Local sats",
				"Reserve",
				"Receive",
				"Can create",
				"Required msat",
				"Available msat",
				"Blocking reason",
				"Suggested action",
				"Min msat",
				"Min reason",
				"Default msat",
				"Default reason",
			],
			rows,
		);
	}

	if !resp.warning.is_empty() {
		println!("{}", resp.warning);
	}
}

async fn handle_onchain(app: &App, command: &RgbOnchainCommand) {
	match command {
		RgbOnchainCommand::InvoiceCreate(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/onchain/invoice/create");
			let req = RgbOnchainInvoiceCreateRequest {
				contract_id: args.contract_id.clone(),
				amount: args.amount,
				expiry_secs: args.expiry_secs,
				use_witness_utxo: Some(args.use_witness_utxo),
				nonce: args.nonce,
				blinding_utxo: args.blinding_utxo.clone(),
			};
			let resp: RgbOnchainInvoiceResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("{}", resp.invoice);
					if let Some(u) = resp.blinding_utxo_used.as_deref() {
						println!("Blinding UTXO used: {u}");
					}
				},
			}
		},
		RgbOnchainCommand::InvoiceDecode { invoice } => {
			let url = join_url(&app.base, "/api/v1/rgb/onchain/invoice/decode");
			let req = RgbOnchainInvoiceDecodeRequest { invoice: invoice.clone() };
			let resp: RgbOnchainInvoiceDecodeResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let rows = vec![
						vec!["Contract".into(), resp.contract_id],
						vec!["Amount".into(), ui::format_u64_with_commas(resp.amount)],
						vec!["Beneficiary".into(), resp.beneficiary],
						vec![
							"Beneficiary type".into(),
							if resp.use_witness_utxo {
								"Witness output".into()
							} else {
								"Blinded".into()
							},
						],
						vec![
							"Expiry".into(),
							resp.expiry_unix_secs
								.map(|v| v.to_string())
								.unwrap_or_else(|| "-".into()),
						],
					];
					ui::print_table(app.theme, &["Field", "Value"], rows);
				},
			}
		},
		RgbOnchainCommand::Payments(args) => {
			let path = match args.contract_id.as_deref() {
				Some(contract_id) => {
					format!("/api/v1/rgb/onchain/payments?contract_id={contract_id}")
				},
				None => "/api/v1/rgb/onchain/payments".to_string(),
			};
			let url = join_url(&app.base, &path);
			let resp: RgbOnchainPaymentsResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let rows = resp
						.payments
						.into_iter()
						.map(|p| {
							vec![
								p.id,
								p.status,
								p.contract_id.unwrap_or_else(|| "-".into()),
								p.amount
									.map(ui::format_u64_with_commas)
									.unwrap_or_else(|| "-".into()),
								p.txid.unwrap_or_else(|| "-".into()),
								p.consignment_key.unwrap_or_else(|| "-".into()),
							]
						})
						.collect::<Vec<_>>();
					ui::print_table_with_right_align(
						app.theme,
						&["Payment ID", "Status", "Contract", "Amount", "Txid", "Consignment"],
						rows,
						&[3],
					);
				},
			}
		},
		RgbOnchainCommand::Send(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/onchain/send");
			let req = RgbOnchainSendRequest {
				invoice: args.invoice.clone(),
				sats_for_fee_and_outputs: args.sats_for_fee_and_outputs,
				fee_rate_sats_per_vb: args.fee_rate_sats_per_vb,
			};
			let resp: RgbOnchainSendResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("Broadcast transaction: {}", resp.txid);
					println!("Consignment key: {}", resp.consignment_key);
				},
			}
		},
		RgbOnchainCommand::Receive(args) => {
			if args.payment_id.is_none() && args.invoice.is_none() {
				die("expected --payment-id or --invoice");
			}
			let url = join_url(&app.base, "/api/v1/rgb/onchain/receive");
			let resp: RgbOnchainReceiveResponse =
				if let Some(consignment_key) = args.consignment_key.clone() {
					let req = RgbOnchainReceiveRequest {
						consignment_key,
						payment_id: args.payment_id.clone(),
						invoice: args.invoice.clone(),
					};
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e))
				} else if let Some(file) = args.file.clone() {
					let payment_id = args.payment_id.as_deref().unwrap_or_else(|| {
						die("binary consignment upload requires --payment-id");
					});
					let bytes = std::fs::read(&file)
						.unwrap_or_else(|e| die(format!("failed to read {file}: {e}")));
					let url = join_url(
						&app.base,
						&format!(
							"/api/v1/rgb/onchain/receive?format={}&payment_id={}",
							args.format, payment_id
						),
					);
					send_json(
						app.client
							.post(url)
							.header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
							.body(bytes),
					)
					.await
					.unwrap_or_else(|e| die(e))
				} else {
					die("expected <consignment_key> or --file");
				};
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("Accepted contract {}.", resp.contract_id);
					println!("Amount: {}", ui::format_u64_with_commas(resp.amount));
				},
			}
		},
	}
}
