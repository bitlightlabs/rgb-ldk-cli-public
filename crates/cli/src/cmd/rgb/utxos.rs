use rgbldk_http_client::dto::{
	RgbUtxosFundInputDto, RgbUtxosFundOutputDto, RgbUtxosFundRequest, RgbUtxosFundResponse,
	RgbUtxosReleaseRequest, RgbUtxosReleaseResponse, RgbUtxosReserveRequest,
	RgbUtxosReserveResponse, RgbUtxosResponse, RgbUtxosSummaryResponse, RgbUtxosSweepInputDto,
	RgbUtxosSweepRequest, RgbUtxosSweepResponse, RgbUtxosTopUpL1InputDto, RgbUtxosTopUpRequest,
	RgbUtxosTopUpResponse, RgbUtxosTopUpRgbInputDto, RgbUtxosTopUpRgbOutputDto,
};

use crate::app::App;
use crate::cli::RgbUtxosCommand;
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &RgbUtxosCommand) {
	match command {
		RgbUtxosCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/rgb/utxos");
			let resp: RgbUtxosResponse =
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
								ui::format_u64_with_commas(u.value_sats),
								match u.confirmation.status {
									rgbldk_http_client::dto::RgbUtxoConfirmationStatusDto::Confirmed =>
										"Confirmed".to_string(),
									rgbldk_http_client::dto::RgbUtxoConfirmationStatusDto::Mempool =>
										"In mempool".to_string(),
								},
								u.confirmation
									.height
									.map(|h| h.to_string())
									.unwrap_or_else(|| "-".into()),
								format_lock_state(u.lock.locked, &u.lock.kind),
								format_allocations(app.no_truncate, &u.rgb.allocations),
								format_roles(&u.rgb.spend_roles),
							]
						})
						.collect::<Vec<_>>();
					ui::print_table_with_right_align(
						app.theme,
						&[
							"Outpoint",
							"Value (sats)",
							"Status",
							"Height",
							"Availability",
							"Allocations",
							"Roles",
						],
						rows,
						&[1, 3],
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
								if u.reserved { "Reserved".into() } else { "Available".into() },
								u.assets
									.into_iter()
									.map(|a| {
										let contract = if app.no_truncate {
											a.contract_id
										} else {
											ui::truncate_id(&a.contract_id)
										};
										format!(
											"{contract}: {}",
											ui::format_u64_with_commas(a.amount)
										)
									})
									.collect::<Vec<_>>()
									.join(", "),
							]
						})
						.collect::<Vec<_>>();
					ui::print_table_with_right_align(
						app.theme,
						&["Outpoint", "Value (sats)", "Height", "State", "Assets"],
						rows,
						&[1, 2],
					);
				},
			}
		},
		RgbUtxosCommand::Reserve(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/utxos/reserve");
			let req =
				RgbUtxosReserveRequest { outpoint: args.outpoint.clone(), ttl_secs: args.ttl_secs };
			let resp: RgbUtxosReserveResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("Reserved RGB UTXO {}.", resp.outpoint);
					println!("Reservation id: {}", resp.reservation_id);
					println!("Expires at: {}", resp.reserved_until_unix_secs);
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
						println!("Reservation released.");
					} else {
						println!("No matching reservation was found.");
						std::process::exit(1);
					}
				},
			}
		},
		RgbUtxosCommand::Fund(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/utxos/fund");
			let req = RgbUtxosFundRequest {
				inputs: args
					.input
					.iter()
					.cloned()
					.map(|outpoint| RgbUtxosFundInputDto { outpoint })
					.collect(),
				outputs: args
					.output
					.iter()
					.cloned()
					.map(|output| RgbUtxosFundOutputDto {
						address: output.address,
						value_sats: output.value_sats,
					})
					.collect(),
				change_address: args.change_address.clone(),
				fee_rate_sats_per_vb: args.fee_rate_sats_per_vb,
			};
			let resp: RgbUtxosFundResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("Funded RGB outputs in transaction {}.", resp.txid);
					println!("Network fee: {} sats.", ui::format_u64_with_commas(resp.fee_sats));
					let rows = resp
						.outputs
						.into_iter()
						.map(|o| {
							vec![
								o.address,
								ui::format_u64_with_commas(o.value_sats),
								o.vout.to_string(),
							]
						})
						.collect::<Vec<_>>();
					ui::print_table_with_right_align(
						app.theme,
						&["RGB Address", "Value (sats)", "Vout"],
						rows,
						&[1, 2],
					);
					if let Some(change) = resp.change {
						let rows = vec![vec![
							change.address,
							ui::format_u64_with_commas(change.value_sats),
							change.vout.to_string(),
						]];
						ui::print_table_with_right_align(
							app.theme,
							&["Change Address", "Value (sats)", "Vout"],
							rows,
							&[1, 2],
						);
					}
				},
			}
		},
		RgbUtxosCommand::Sweep(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/utxos/sweep");
			let req = RgbUtxosSweepRequest {
				input: RgbUtxosSweepInputDto { outpoint: args.outpoint.clone() },
				destination_address: args.destination_address.clone(),
				fee_rate_sats_per_vb: args.fee_rate_sats_per_vb,
			};
			let resp: RgbUtxosSweepResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("Swept RGB UTXO {}.", resp.input.outpoint);
					println!("Broadcast transaction: {}", resp.txid);
					println!(
						"Sent {} sats to {}.",
						ui::format_u64_with_commas(resp.destination.value_sats),
						resp.destination.address
					);
					println!("Network fee: {} sats.", ui::format_u64_with_commas(resp.fee_sats));
				},
			}
		},
		RgbUtxosCommand::TopUp(args) => {
			let url = join_url(&app.base, "/api/v1/rgb/utxos/top_up");
			let req = RgbUtxosTopUpRequest {
				rgb_input: RgbUtxosTopUpRgbInputDto { outpoint: args.rgb_outpoint.clone() },
				l1_inputs: args
					.l1_input
					.iter()
					.cloned()
					.map(|outpoint| RgbUtxosTopUpL1InputDto { outpoint })
					.collect(),
				rgb_output: RgbUtxosTopUpRgbOutputDto {
					address: args.rgb_address.clone(),
					target_value_sats: args.target_value_sats,
				},
				change_address: args.change_address.clone(),
				fee_rate_sats_per_vb: args.fee_rate_sats_per_vb,
			};
			let resp: RgbUtxosTopUpResponse =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					println!("Topped up RGB UTXO {}.", resp.old_outpoint);
					println!("Broadcast transaction: {}", resp.txid);
					println!(
						"Replacement output: {} with {} sats.",
						resp.new_rgb_output.address,
						ui::format_u64_with_commas(resp.new_rgb_output.value_sats)
					);
					println!("Network fee: {} sats.", ui::format_u64_with_commas(resp.fee_sats));
					println!("Consignment key: {}", resp.consignment_key);
					if let Some(change) = resp.change {
						println!(
							"Change output: {} with {} sats.",
							change.address,
							ui::format_u64_with_commas(change.value_sats)
						);
					}
				},
			}
		},
	}
}

fn format_lock_state(locked: bool, kind: &rgbldk_http_client::dto::RgbUtxoLockKindDto) -> String {
	if !locked {
		return "Available".into();
	}
	match kind {
		rgbldk_http_client::dto::RgbUtxoLockKindDto::ManualReservation => "Reserved".into(),
		rgbldk_http_client::dto::RgbUtxoLockKindDto::Operation => "In use".into(),
		rgbldk_http_client::dto::RgbUtxoLockKindDto::None => "Locked".into(),
	}
}

fn format_allocations(
	no_truncate: bool, allocations: &[rgbldk_http_client::dto::RgbAllocationDto],
) -> String {
	if allocations.is_empty() {
		return "-".into();
	}
	allocations
		.iter()
		.map(|allocation| {
			let contract = if no_truncate {
				allocation.contract_id.clone()
			} else {
				ui::truncate_id(&allocation.contract_id)
			};
			format!(
				"{contract}: {} ({})",
				ui::format_u64_with_commas(allocation.amount),
				allocation.layer
			)
		})
		.collect::<Vec<_>>()
		.join(", ")
}

fn format_roles(roles: &[String]) -> String {
	if roles.is_empty() {
		"-".into()
	} else {
		roles.join(", ")
	}
}
