use owo_colors::OwoColorize;

use rgbldk_http_dto::{
	Bolt11ClaimForHashRequest, Bolt11DecodeRequest, Bolt11DecodeResponse, Bolt11FailForHashRequest,
	Bolt11PayRequest, Bolt11PayResponse, Bolt11ReceiveForHashRequest, Bolt11ReceiveRequest,
	Bolt11ReceiveResponse, Bolt11ReceiveVarRequest, Bolt11SendRequest,
	Bolt11SendUsingAmountRequest, Bolt12OfferDecodeRequest, Bolt12OfferDecodeResponse,
	Bolt12OfferReceiveRequest, Bolt12OfferReceiveVarRequest, Bolt12OfferResponse,
	Bolt12OfferSendRequest, Bolt12RefundDecodeRequest, Bolt12RefundDecodeResponse,
	Bolt12RefundInitiateRequest, Bolt12RefundInitiateResponse, Bolt12RefundRequestPaymentRequest,
	Bolt12RefundRequestPaymentResponse, CustomTlvDto, OkResponse, PaymentDetailsDto,
	PaymentWaitRequest, PaymentWaitResponse, SendResponse, SpontaneousSendRequest,
};

use serde::Serialize;

use crate::app::App;
use crate::cli::{InvoiceCommand, KeysendCommand, OfferCommand, PayCommand, RefundCommand};
use crate::client::{join_url, send_json, send_value};
use crate::ui;
use crate::utils::{die, print_json};

fn print_json_or_text<T: Serialize>(app: &App, v: &T, text: impl FnOnce()) {
	match app.output {
		ui::OutputMode::Json => print_json(v, app.pretty),
		ui::OutputMode::Text => text(),
	}
}

fn print_wait_error_text(app: &App, v: &serde_json::Value) {
	let err = v.get("error").and_then(|e| e.as_str()).unwrap_or("request failed");
	if app.theme.color {
		eprintln!("{}", err.red());
	} else {
		eprintln!("{err}");
	}
	if let Some(checks) = v.get("checks") {
		if let Ok(checks) =
			serde_json::from_value::<Vec<rgbldk_http_dto::HealthCheckDto>>(checks.clone())
		{
			ui::print_checks(app.theme, "Details", false, &checks);
		}
	}
	if let Some(payment) = v.get("payment") {
		if let Ok(p) = serde_json::from_value::<PaymentDetailsDto>(payment.clone()) {
			println!("{}", p.id);
		}
	}
}

pub(crate) async fn handle(app: &App, command: &PayCommand) {
	match command {
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
								if app.no_truncate { p.id.clone() } else { ui::truncate_id(&p.id) };
							vec![
								id,
								p.status,
								p.kind,
								p.direction,
								p.amount_msat
									.map(ui::format_u64_with_commas)
									.unwrap_or_else(|| "-".into()),
								p.fee_paid_msat
									.map(ui::format_u64_with_commas)
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

			if status == HttpStatus::OK {
				let resp: PaymentWaitResponse =
					serde_json::from_value(v).unwrap_or_else(|e| die(e.to_string()));
				print_json_or_text(app, &resp, || {
					ui::print_checks(app.theme, "Payment wait", resp.ok, &resp.checks);
					println!("{}", &resp.payment.id);
				});
			} else {
				match app.output {
					ui::OutputMode::Json => print_json(&v, app.pretty),
					ui::OutputMode::Text => print_wait_error_text(app, &v),
				}
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
								.map(|v| format!("{} msat", ui::format_u64_with_commas(v)))
								.unwrap_or_else(|| "-".into()),
						],
						vec![
							"fee paid (msat)".into(),
							p.fee_paid_msat
								.map(|v| format!("{} msat", ui::format_u64_with_commas(v)))
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
						description: args.desc.clone(),
						expiry_secs: args.expiry_secs,
					})
					.unwrap();
				} else {
					url = join_url(&app.base, "/api/v1/bolt11/receive_var");
					body = serde_json::to_value(Bolt11ReceiveVarRequest {
						description: args.desc.clone(),
						expiry_secs: args.expiry_secs,
					})
					.unwrap();
				}
				let resp: Bolt11ReceiveResponse =
					send_json(app.client.post(url).json(&body)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.invoice));
			},
			InvoiceCommand::CreateForHash(args) => {
				let url = join_url(&app.base, "/api/v1/bolt11/receive_for_hash");
				let req = Bolt11ReceiveForHashRequest {
					amount_msat: args.amount_msat,
					description: args.desc.clone(),
					expiry_secs: args.expiry_secs,
					payment_hash: args.payment_hash.clone(),
				};
				let resp: Bolt11ReceiveResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.invoice));
			},
			InvoiceCommand::Decode { invoice } => {
				let url = join_url(&app.base, "/api/v1/bolt11/decode");
				let req = Bolt11DecodeRequest { invoice: invoice.clone() };
				let resp: Bolt11DecodeResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				match app.output {
					ui::OutputMode::Json => print_json(&resp, app.pretty),
					ui::OutputMode::Text => {
						let rows = vec![
							vec!["payment_hash".into(), resp.payment_hash],
							vec!["destination".into(), resp.destination],
							vec![
								"amount_msat".into(),
								resp.amount_msat
									.map(ui::format_u64_with_commas)
									.unwrap_or_else(|| "-".into()),
							],
							vec!["expiry_secs".into(), resp.expiry_secs.to_string()],
						];
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
			InvoiceCommand::FailForHash { payment_hash } => {
				let url = join_url(&app.base, "/api/v1/bolt11/fail_for_hash");
				let req = Bolt11FailForHashRequest { payment_hash: payment_hash.clone() };
				let resp: OkResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || {
					ui::print_checks(app.theme, "Fail", resp.ok, &resp.checks)
				});
				if !resp.ok {
					std::process::exit(1);
				}
			},
			InvoiceCommand::ClaimForHash(args) => {
				let url = join_url(&app.base, "/api/v1/bolt11/claim_for_hash");
				let req = Bolt11ClaimForHashRequest {
					payment_hash: args.payment_hash.clone(),
					preimage: args.preimage.clone(),
					claimable_amount_msat: args.claimable_amount_msat,
				};
				let resp: OkResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || {
					ui::print_checks(app.theme, "Claim", resp.ok, &resp.checks)
				});
				if !resp.ok {
					std::process::exit(1);
				}
			},
			InvoiceCommand::Pay(args) => {
				let req = Bolt11PayRequest {
					invoice: args.invoice.clone(),
					amount_msat: args.amount_msat,
				};
				let url = join_url(&app.base, "/api/v1/bolt11/pay");
				let resp: Bolt11PayResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.payment_id));
			},
			InvoiceCommand::Send { invoice } => {
				let url = join_url(&app.base, "/api/v1/bolt11/send");
				let req = Bolt11SendRequest { invoice: invoice.clone() };
				let resp: SendResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.payment_id));
			},
			InvoiceCommand::SendUsingAmount(args) => {
				let url = join_url(&app.base, "/api/v1/bolt11/send_using_amount");
				let req = Bolt11SendUsingAmountRequest {
					invoice: args.invoice.clone(),
					amount_msat: args.amount_msat,
				};
				let resp: SendResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.payment_id));
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
							description: args.desc.clone(),
							expiry_secs,
							quantity: args.quantity,
						})
						.unwrap(),
					)
				} else {
					(
						join_url(&app.base, "/api/v1/bolt12/offer/receive_var"),
						serde_json::to_value(Bolt12OfferReceiveVarRequest {
							description: args.desc.clone(),
							expiry_secs,
						})
						.unwrap(),
					)
				};
				let resp: Bolt12OfferResponse =
					send_json(app.client.post(url).json(&body)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.offer));
			},
			OfferCommand::Decode { offer } => {
				let url = join_url(&app.base, "/api/v1/bolt12/offer/decode");
				let req = Bolt12OfferDecodeRequest { offer: offer.clone() };
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
									.map(ui::format_u64_with_commas)
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
					offer: args.offer.clone(),
					amount_msat: args.amount_msat,
					quantity: args.quantity,
					payer_note: args.payer_note.clone(),
				};
				let resp: SendResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.payment_id));
			},
		},
		PayCommand::Refund { command } => match command {
			RefundCommand::Initiate(args) => {
				let url = join_url(&app.base, "/api/v1/bolt12/refund/initiate");
				let req = Bolt12RefundInitiateRequest {
					amount_msat: args.amount_msat,
					expiry_secs: args.expiry_secs,
					quantity: args.quantity,
					payer_note: args.payer_note.clone(),
				};
				let resp: Bolt12RefundInitiateResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || {
					println!("{}", &resp.refund);
					eprintln!("payment_id: {}", &resp.payment_id);
				});
			},
			RefundCommand::Decode { refund } => {
				let url = join_url(&app.base, "/api/v1/bolt12/refund/decode");
				let req = Bolt12RefundDecodeRequest { refund: refund.clone() };
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
								ui::format_u64_with_commas(resp.amount_msat),
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
								resp.quantity.map(|v| v.to_string()).unwrap_or_else(|| "-".into()),
							],
							vec!["paths_count".into(), resp.paths_count.to_string()],
						];
						ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			},
			RefundCommand::RequestPayment { refund } => {
				let url = join_url(&app.base, "/api/v1/bolt12/refund/request_payment");
				let req = Bolt12RefundRequestPaymentRequest { refund: refund.clone() };
				let resp: Bolt12RefundRequestPaymentResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || {
					println!("{}", &resp.invoice);
					eprintln!("payment_id: {}", &resp.payment_id);
				});
			},
		},
		PayCommand::Keysend { command } => match command {
			KeysendCommand::Send(args) => {
				let tlvs: Vec<CustomTlvDto> = args
					.tlv
					.iter()
					.cloned()
					.map(|t| CustomTlvDto { r#type: t.r#type, value_hex: t.value_hex })
					.collect();
				let req = SpontaneousSendRequest {
					counterparty_node_id: args.node_id.clone(),
					amount_msat: args.amount_msat,
					custom_tlvs: tlvs,
				};
				let url = join_url(&app.base, "/api/v1/spontaneous/send");
				let resp: SendResponse =
					send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
				print_json_or_text(app, &resp, || println!("{}", &resp.payment_id));
			},
		},
	}
}
