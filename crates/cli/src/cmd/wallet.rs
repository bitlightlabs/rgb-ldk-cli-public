use rgbldk_http_dto::BalancesDto;

use crate::app::App;
use crate::cli::WalletCommand;
use crate::client::{join_url, send_json};
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &WalletCommand) {
	match command {
		WalletCommand::Balance(args) => {
			let url = join_url(&app.base, "/api/v1/balances");
			let b: BalancesDto = send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&b, app.pretty),
				crate::ui::OutputMode::Text => {
					let sats = args.sats;
					let rows = vec![
						vec![
							"BTC On-chain (total)".into(),
							crate::ui::format_balance_sats(b.btc.onchain_total_sats, sats),
						],
						vec![
							"BTC On-chain (spendable)".into(),
							crate::ui::format_balance_sats(b.btc.onchain_spendable_sats, sats),
						],
						vec![
							"BTC Anchor reserve".into(),
							crate::ui::format_balance_sats(
								b.btc.anchor_channels_reserve_sats,
								sats,
							),
						],
						vec![
							"BTC Lightning (total)".into(),
							crate::ui::format_balance_sats(b.btc.lightning_total_sats, sats),
						],
					];
					crate::ui::print_table_with_right_align(
						app.theme,
						&["Asset", "Balance"],
						rows,
						&[1],
					);

					if !b.rgb.l1.is_empty() {
						let mut rows = Vec::new();
						for row in &b.rgb.l1 {
							rows.push(vec![
								row.contract_id.clone(),
								row.contract_id.clone(),
								row.mined.to_string(),
								row.tentative.to_string(),
								row.offchain.to_string(),
								row.total.to_string(),
							]);
						}
						crate::ui::print_table_with_right_align(
							app.theme,
							&[
								"RGB L1 Contract",
								"Contract",
								"Mined",
								"Tentative",
								"Offchain",
								"Total",
							],
							rows,
							&[2, 3, 4, 5],
						);
					}

					if !b.rgb.l2.is_empty() {
						let mut rows = Vec::new();
						for row in &b.rgb.l2 {
							rows.push(vec![
								row.channel_id.clone(),
								row.contract_id.clone(),
								row.local_amount.to_string(),
								row.remote_amount.to_string(),
							]);
						}
						crate::ui::print_table_with_right_align(
							app.theme,
							&["RGB L2 Channel", "Contract", "Local", "Remote"],
							rows,
							&[2, 3],
						);
					}
				},
			}
		},
		WalletCommand::Address => {
			let url = join_url(&app.base, "/api/v1/wallet/new_address");
			let v: serde_json::Value = send_json(app.client.post(url).json(&serde_json::json!({})))
				.await
				.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					println!("{}", v.get("address").and_then(|v| v.as_str()).unwrap_or(""))
				},
			}
		},
		WalletCommand::Sync => {
			if app.output == crate::ui::OutputMode::Json {
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

				let dt = b1.btc.onchain_total_sats as i128 - b0.btc.onchain_total_sats as i128;
				let ds =
					b1.btc.onchain_spendable_sats as i128 - b0.btc.onchain_spendable_sats as i128;
				let dr = b1.btc.anchor_channels_reserve_sats as i128
					- b0.btc.anchor_channels_reserve_sats as i128;
				let dl = b1.btc.lightning_total_sats as i128 - b0.btc.lightning_total_sats as i128;

				println!("Wallet synced.");
				if dt == 0 && ds == 0 && dr == 0 && dl == 0 {
					println!("No balance change.");
				} else {
					let fmt_delta = |d: i128| -> String {
						if d == 0 {
							"0 sats".to_string()
						} else if d > 0 {
							format!("+{}", crate::ui::format_balance_sats(d as u64, false))
						} else {
							format!("-{}", crate::ui::format_balance_sats((-d) as u64, false))
						}
					};
					println!(
						"BTC balance change: on-chain total {}, spendable {}, anchor reserve {}, lightning {}.",
						fmt_delta(dt),
						fmt_delta(ds),
						fmt_delta(dr),
						fmt_delta(dl)
					);
				}
			}
		},
	}
}
