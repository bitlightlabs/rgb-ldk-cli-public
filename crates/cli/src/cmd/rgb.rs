use rgbldk_http_dto::{
	OkResponse, RgbContractBalanceResponse, RgbContractKnownResponse, RgbContractsExportRequest,
	RgbContractsExportResponse, RgbContractsImportResponse, RgbContractsIssueRequest,
	RgbContractsIssueResponse, RgbContractsResponse, RgbIssuersImportResponse, RgbIssuersResponse,
	RgbLnInvoiceCreateForHashRequest, RgbLnInvoiceCreateRequest, RgbLnInvoiceDecodeRequest,
	RgbLnInvoiceDecodeResponse, RgbLnInvoiceResponse, RgbLnPayRequest, RgbNewAddressResponse,
	RgbOnchainInvoiceCreateRequest, RgbOnchainInvoiceResponse, RgbOnchainReceiveRequest,
	RgbOnchainReceiveResponse, RgbOnchainSendRequest, RgbOnchainSendResponse,
	RgbUtxosReleaseRequest, RgbUtxosReleaseResponse, RgbUtxosReserveRequest,
	RgbUtxosReserveResponse, RgbUtxosResponse, RgbUtxosSummaryResponse, SendResponse,
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
	serde_json::from_slice::<serde_json::Value>(body)
		.ok()
		.and_then(|v| v.get("error").and_then(|e| e.as_str()).map(|s| s.to_string()))
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
				let resp: RgbUtxosResponse =
					send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						for u in resp.utxos {
							println!("{u}");
						}
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
									c.asset_id,
									c.contract_id,
								]
							})
							.collect::<Vec<_>>();
						ui::print_table_with_right_align(
							app.theme,
							&[
								"name",
								"ticker",
								"precision",
								"issued_supply",
								"asset_id",
								"contract_id",
							],
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
						println!("asset_id={}", resp.asset_id);
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
						asset_id: args.asset_id.clone(),
						asset_amount: args.asset_amount,
						description: args.desc.clone(),
						expiry_secs: args.expiry_secs,
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
						asset_id: args.asset_id.clone(),
						asset_amount: args.asset_amount,
						payment_hash: args.payment_hash.clone(),
						description: args.desc.clone(),
						expiry_secs: args.expiry_secs,
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
									"asset_id".into(),
									resp.asset_id.unwrap_or_else(|| "-".into()),
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
					asset_id: args.asset_id.clone(),
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
				let url = join_url(&app.base, "/api/v1/rgb/onchain/receive");
				let resp: RgbOnchainReceiveResponse =
					if let Some(consignment_key) = args.consignment_key.clone() {
						let req = RgbOnchainReceiveRequest { consignment_key };
						send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e))
					} else if let Some(file) = args.file.clone() {
						let bytes = std::fs::read(&file)
							.unwrap_or_else(|e| die(format!("failed to read {file}: {e}")));
						let url = join_url(
							&app.base,
							&format!("/api/v1/rgb/onchain/receive?format={}", args.format),
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
						println!("asset_id={}", resp.asset_id);
						println!("amount={}", ui::format_u64_with_commas(resp.amount));
					},
				}
			},
		},
	}
}
