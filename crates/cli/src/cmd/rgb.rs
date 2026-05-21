use rgbldk_http_dto::{
	OkResponse, RgbContractBalanceResponse, RgbContractKnownResponse, RgbContractsExportRequest,
	RgbContractsExportResponse, RgbContractsImportResponse, RgbContractsIssueRequest,
	RgbContractsIssueResponse, RgbContractsResponse, RgbIssuersImportResponse, RgbIssuersResponse,
	RgbLnInvoiceCreateForHashRequest, RgbLnInvoiceCreateRequest, RgbLnInvoiceDecodeRequest,
	RgbLnInvoiceDecodeResponse, RgbLnInvoiceResponse, RgbLnPayRequest, RgbNewAddressResponse,
	RgbOnchainInvoiceCreateRequest, RgbOnchainInvoiceDecodeRequest,
	RgbOnchainInvoiceDecodeResponse, RgbOnchainInvoiceResponse, RgbOnchainPaymentsResponse,
	RgbOnchainReceiveRequest, RgbOnchainReceiveResponse, RgbOnchainSendRequest,
	RgbOnchainSendResponse, RgbUtxosReleaseRequest, RgbUtxosReleaseResponse,
	RgbUtxosReserveRequest, RgbUtxosReserveResponse, RgbUtxosSummaryResponse, SendResponse,
};

use crate::app::App;
use crate::cli::{
	RgbCommand, RgbConsignmentsCommand, RgbContractsCommand, RgbIssuersCommand, RgbLnCommand,
	RgbLnInvoiceCommand, RgbOnchainCommand, RgbUtxosCommand,
};
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

fn error_from_body(body: &[u8]) -> Option<String> {
	let v = serde_json::from_slice::<serde_json::Value>(body).ok()?;
	let error = v.get("error").and_then(|e| e.as_str()).unwrap_or("request failed");
	let message = v.get("message").and_then(|m| m.as_str());
	let hint = v.get("hint").and_then(|h| h.as_str());
	let detail = v.get("detail");
	let checks = v.get("checks");

	let mut parts = vec![error.to_string()];
	if let Some(message) = message.filter(|m| *m != error) {
		parts.push(format!("message={message}"));
	}
	if let Some(hint) = hint {
		parts.push(format!("hint={hint}"));
	}
	if let Some(detail) = detail {
		parts.push(format!("detail={detail}"));
	}
	if let Some(checks) = checks {
		parts.push(format!("checks={checks}"));
	}
	Some(parts.join("\n"))
}

fn ensure_success_or_die(status: reqwest::StatusCode, body: &[u8]) {
	if status.is_success() {
		return;
	}
	let msg =
		error_from_body(body).unwrap_or_else(|| format!("request failed with status {status}"));
	die(msg);
}

fn write_bytes_or_die(out: &str, body: &[u8]) {
	std::fs::write(out, body).unwrap_or_else(|e| die(format!("failed to write {out}: {e}")));
}

fn format_json_u64(value: Option<&serde_json::Value>) -> String {
	match value {
		Some(serde_json::Value::String(s)) => {
			s.parse::<u64>().map(ui::format_u64_with_commas).unwrap_or_else(|_| s.clone())
		},
		Some(serde_json::Value::Number(n)) => {
			n.as_u64().map(ui::format_u64_with_commas).unwrap_or_else(|| n.to_string())
		},
		_ => "-".into(),
	}
}

fn format_json_string(value: Option<&serde_json::Value>) -> String {
	value.and_then(|v| v.as_str()).unwrap_or("-").to_string()
}

fn format_json_bool(value: Option<&serde_json::Value>) -> String {
	value.and_then(|v| v.as_bool()).map(|b| b.to_string()).unwrap_or_else(|| "-".into())
}

fn format_rgb_allocations(value: Option<&serde_json::Value>) -> String {
	let Some(allocations) = value.and_then(|v| v.as_array()) else {
		return "-".into();
	};
	if allocations.is_empty() {
		return "-".into();
	}
	allocations
		.iter()
		.map(|a| {
			format!(
				"{}:{}:{}",
				format_json_string(a.get("contract_id")),
				format_json_u64(a.get("amount")),
				format_json_string(a.get("layer"))
			)
		})
		.collect::<Vec<_>>()
		.join(",")
}

fn format_string_list(value: Option<&serde_json::Value>) -> String {
	let Some(values) = value.and_then(|v| v.as_array()) else {
		return "-".into();
	};
	let values = values.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>();
	if values.is_empty() {
		"-".into()
	} else {
		values.join(",")
	}
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
					ui::print_checks(app.theme, "RGB sync", resp.ok, &resp.checks);
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
		RgbCommand::Utxos { command } => match command {
			RgbUtxosCommand::Ls => {
				let url = join_url(&app.base, "/api/v1/rgb/utxos");
				let resp: serde_json::Value =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						let utxos =
							resp.get("utxos").and_then(|v| v.as_array()).unwrap_or_else(|| {
								die("invalid /rgb/utxos response: expected utxos array")
							});
						let rows = resp
							.get("utxos")
							.and_then(|v| v.as_array())
							.unwrap_or(utxos)
							.iter()
							.map(|u| {
								if let Some(outpoint) = u.as_str() {
									return vec![
										outpoint.to_string(),
										"-".into(),
										"-".into(),
										"-".into(),
										"-".into(),
										"-".into(),
									];
								}
								vec![
									format_json_string(u.get("outpoint")),
									format_json_u64(u.get("value_sats")),
									format_json_u64(u.get("confirmed_height")),
									format_rgb_allocations(u.get("rgb_allocations")),
									format_json_bool(u.get("has_mixed_asset_allocations")),
									format_string_list(u.get("spend_roles")),
								]
							})
							.collect::<Vec<_>>();
						ui::print_table_with_right_align(
							app.theme,
							&[
								"outpoint",
								"value_sats",
								"confirmed_height",
								"rgb_allocations",
								"mixed",
								"spend_roles",
							],
							rows,
							&[1, 2],
						);
					},
				}
			},
			RgbUtxosCommand::Summary => {
				let url = join_url(&app.base, "/api/v1/rgb/utxos/summary");
				let resp: RgbUtxosSummaryResponse =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						let rows = resp
							.utxos
							.into_iter()
							.map(|u| {
								vec![
									u.outpoint,
									u.value_sats
										.map(ui::format_u64_with_commas)
										.unwrap_or_else(|| "-".into()),
									u.confirmed_height
										.map(|h| h.to_string())
										.unwrap_or_else(|| "-".into()),
									u.reserved.to_string(),
									u.reserved_until_unix_secs
										.map(|t| t.to_string())
										.unwrap_or_else(|| "-".into()),
								]
							})
							.collect::<Vec<_>>();
						ui::print_table_with_right_align(
							app.theme,
							&[
								"outpoint",
								"value_sats",
								"confirmed_height",
								"reserved",
								"reserved_until",
							],
							rows,
							&[1, 2, 4],
						);
					},
				}
			},
			RgbUtxosCommand::Reserve(args) => {
				let url = join_url(&app.base, "/api/v1/rgb/utxos/reserve");
				let req = RgbUtxosReserveRequest {
					outpoint: args.outpoint.clone(),
					ttl_secs: args.ttl_secs,
				};
				let resp: RgbUtxosReserveResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						println!("reservation_id={}", resp.reservation_id);
						println!("outpoint={}", resp.outpoint);
						println!("reserved_until_unix_secs={}", resp.reserved_until_unix_secs);
					},
				}
			},
			RgbUtxosCommand::Release(args) => {
				if args.reservation_id.is_none() && args.outpoint.is_none() {
					die("expected --reservation-id or --outpoint");
				}
				let url = join_url(&app.base, "/api/v1/rgb/utxos/release");
				let req = RgbUtxosReleaseRequest {
					reservation_id: args.reservation_id.clone(),
					outpoint: args.outpoint.clone(),
				};
				let resp: RgbUtxosReleaseResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						if resp.released {
							println!("released=true");
						} else {
							println!("released=false");
							std::process::exit(1);
						}
					},
				}
			},
		},
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
						println!("consignment_key={}", args.key);
						println!("wrote={}", args.out);
					},
				}
			},
		},
		RgbCommand::Contracts { command } => match command {
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
									c.precision
										.map(|p| p.to_string())
										.unwrap_or_else(|| "-".into()),
									c.issued_supply
										.map(ui::format_u64_with_commas)
										.unwrap_or_else(|| "-".into()),
									c.contract_id,
								]
							})
							.collect::<Vec<_>>();
						ui::print_table_with_right_align(
							app.theme,
							&["name", "ticker", "precision", "issued_supply", "contract_id"],
							rows,
							&[2, 3],
						);
					},
				}
			},
			RgbContractsCommand::Balance { contract_id } => {
				let url =
					join_url(&app.base, &format!("/api/v1/rgb/contract/{}/balance", contract_id));
				let resp: RgbContractBalanceResponse =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						let b = resp.balance;
						let rows = vec![
							vec!["contract_id".into(), resp.contract_id],
							vec!["mined".into(), ui::format_u64_with_commas(b.mined)],
							vec!["tentative".into(), ui::format_u64_with_commas(b.tentative)],
							vec!["offchain".into(), ui::format_u64_with_commas(b.offchain)],
							vec!["archived".into(), ui::format_u64_with_commas(b.archived)],
							vec!["total".into(), ui::format_u64_with_commas(b.total)],
						];
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
			RgbContractsCommand::Known { contract_id } => {
				let url =
					join_url(&app.base, &format!("/api/v1/rgb/contract/{}/known", contract_id));
				let resp: RgbContractKnownResponse =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						println!("contract_id={}", resp.contract_id);
						println!("known={}", resp.known);
						if !resp.known {
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
					app.client
						.post(url)
						.header("content-type", "application/octet-stream")
						.body(bytes),
				)
				.await
				.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						ui::print_checks(app.theme, "RGB contract import", resp.ok, &resp.checks);
						println!("contract_id={}", resp.contract_id);
						println!("consignment_key={}", resp.consignment_key);
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
						println!("contract_id={}", resp.contract_id);
						println!(
							"issued_supply={}",
							ui::format_u64_with_commas(resp.issued_supply)
						);
					},
				}
			},
			RgbContractsCommand::Export(args) => {
				let export_req =
					RgbContractsExportRequest { contract_id: args.contract_id.clone() };
				if args.direct {
					let export_url = join_url(
						&app.base,
						&format!(
							"/api/v1/rgb/contracts/export?download=true&format={}",
							args.format
						),
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
							println!("exported={}", contract_id_hdr.unwrap_or("<unknown>"));
							if let Some(k) = consignment_key_hdr {
								println!("consignment_key={k}");
							}
							println!("wrote={}", args.out);
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
							println!("contract_id={}", export_resp.contract_id);
							println!("consignment_key={}", export_resp.consignment_key);
							println!("wrote={}", args.out);
						},
					}
				}
			},
		},
		RgbCommand::Issuers { command } => match command {
			RgbIssuersCommand::Ls => {
				let url = join_url(&app.base, "/api/v1/rgb/issuers");
				let resp: RgbIssuersResponse =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						let rows = resp.issuers.into_iter().map(|i| vec![i]).collect::<Vec<_>>();
						ui::print_table(app.theme, &["issuer_name"], rows);
					},
				}
			},
			RgbIssuersCommand::Import(args) => {
				let bytes = std::fs::read(&args.file)
					.unwrap_or_else(|e| die(format!("failed to read file {}: {e}", args.file)));
				let url = join_url(
					&app.base,
					&format!(
						"/api/v1/rgb/issuers/import?name={}&format={}",
						args.name, args.format
					),
				);
				let resp: RgbIssuersImportResponse = send_json(
					app.client
						.post(url)
						.header("content-type", "application/octet-stream")
						.body(bytes),
				)
				.await
				.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						ui::print_checks(app.theme, "RGB issuer import", resp.ok, &resp.checks);
						println!("issuer_name={}", resp.issuer_name);
					},
				}
			},
		},
		RgbCommand::Ln { command } => match command {
			RgbLnCommand::Invoice { command } => match command {
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
								vec!["payment_hash".into(), resp.payment_hash],
								vec!["destination".into(), resp.destination],
								vec![
									"carrier_amount_msat".into(),
									resp.carrier_amount_msat
										.map(ui::format_u64_with_commas)
										.unwrap_or_else(|| "-".into()),
								],
								vec!["expiry_secs".into(), resp.expiry_secs.to_string()],
								vec![
									"contract_id".into(),
									resp.contract_id.unwrap_or_else(|| "-".into()),
								],
								vec![
									"asset_amount".into(),
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
		},
		RgbCommand::Onchain { command } => match command {
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
							println!("blinding_utxo_used={u}");
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
							vec!["contract_id".into(), resp.contract_id],
							vec!["amount".into(), ui::format_u64_with_commas(resp.amount)],
							vec!["beneficiary".into(), resp.beneficiary],
							vec!["use_witness_utxo".into(), resp.use_witness_utxo.to_string()],
							vec![
								"expiry_unix_secs".into(),
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
							&[
								"payment_id",
								"status",
								"contract_id",
								"amount",
								"txid",
								"consignment_key",
							],
							rows,
							&[4],
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
						println!("txid={}", resp.txid);
						println!("consignment_key={}", resp.consignment_key);
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
						println!("contract_id={}", resp.contract_id);
						println!("amount={}", ui::format_u64_with_commas(resp.amount));
					},
				}
			},
		},
	}
}
