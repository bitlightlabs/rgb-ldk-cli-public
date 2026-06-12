use rgbldk_http_client::dto::{
	ChannelUpdateInfoDto, NetworkGraphChannelInfoResponse, NetworkGraphChannelsResponse,
	NetworkGraphNodeInfoResponse, NetworkGraphNodesResponse,
};

use crate::app::App;
use crate::cli::GraphCommand;
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &GraphCommand) {
	match command {
		GraphCommand::Nodes => {
			let url = join_url(&app.base, "/api/v1/network_graph/nodes");
			let resp: NetworkGraphNodesResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let rows = resp.nodes.into_iter().map(|node| vec![node]).collect::<Vec<_>>();
					ui::print_table(app.theme, &["Node ID"], rows);
				},
			}
		},
		GraphCommand::Node { node_id_hex } => {
			let url = join_url(&app.base, &format!("/api/v1/network_graph/node/{node_id_hex}"));
			let resp: NetworkGraphNodeInfoResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let display_node = if app.no_truncate {
						node_id_hex.clone()
					} else {
						ui::truncate_id(node_id_hex)
					};
					println!("Node {display_node}.");
					println!("Known channels: {}.", resp.channels.len());

					if !resp.channels.is_empty() {
						let rows = resp
							.channels
							.iter()
							.map(|scid| vec![scid.to_string()])
							.collect::<Vec<_>>();
						ui::print_table(app.theme, &["SCID"], rows);
					}

					if let Some(info) = resp.announcement_info {
						println!();
						println!("Alias: {}.", info.alias);
						println!("Last gossip update: {}.", info.last_update);
						if info.addresses.is_empty() {
							println!("No announced addresses.");
						} else {
							println!("Addresses: {}.", info.addresses.join(", "));
						}
					}
				},
			}
		},
		GraphCommand::Channels => {
			let url = join_url(&app.base, "/api/v1/network_graph/channels");
			let resp: NetworkGraphChannelsResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let rows = resp
						.channels
						.into_iter()
						.map(|scid| vec![scid.to_string()])
						.collect::<Vec<_>>();
					ui::print_table(app.theme, &["SCID"], rows);
				},
			}
		},
		GraphCommand::Channel { scid } => {
			let url = join_url(&app.base, &format!("/api/v1/network_graph/channel/{scid}"));
			let resp: NetworkGraphChannelInfoResponse =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				ui::OutputMode::Json => print_json(&resp, app.pretty),
				ui::OutputMode::Text => {
					let node_one = if app.no_truncate {
						resp.node_one.clone()
					} else {
						ui::truncate_id(&resp.node_one)
					};
					let node_two = if app.no_truncate {
						resp.node_two.clone()
					} else {
						ui::truncate_id(&resp.node_two)
					};

					println!("Channel {scid} connects {node_one} and {node_two}.");
					match resp.capacity_sats {
						Some(capacity_sats) => {
							println!(
								"Capacity: {} sats.",
								ui::format_u64_with_commas(capacity_sats)
							);
						},
						None => println!("Capacity: unknown."),
					}
					println!();
					print_channel_update(
						app,
						&format!("{node_one} -> {node_two}"),
						resp.one_to_two.as_ref(),
					);
					println!();
					print_channel_update(
						app,
						&format!("{node_two} -> {node_one}"),
						resp.two_to_one.as_ref(),
					);
				},
			}
		},
	}
}

fn print_channel_update(app: &App, label: &str, update: Option<&ChannelUpdateInfoDto>) {
	match update {
		Some(update) => {
			let state = if update.enabled { "Enabled" } else { "Disabled" };
			println!("{label}: {state}.");
			let rows = vec![
				vec!["Last update".into(), update.last_update.to_string()],
				vec!["CLTV delta".into(), update.cltv_expiry_delta.to_string()],
				vec![
					"Min HTLC".into(),
					format!("{} msat", ui::format_u64_with_commas(update.htlc_minimum_msat)),
				],
				vec![
					"Max HTLC".into(),
					format!("{} msat", ui::format_u64_with_commas(update.htlc_maximum_msat)),
				],
				vec![
					"Base fee".into(),
					format!("{} msat", ui::format_u64_with_commas(update.fee_base_msat.into())),
				],
				vec![
					"Fee rate".into(),
					format!(
						"{} ppm",
						ui::format_u64_with_commas(update.fee_proportional_millionths.into())
					),
				],
			];
			ui::print_table(app.theme, &["Field", "Value"], rows);
		},
		None => println!("{label}: no gossip update available."),
	}
}
