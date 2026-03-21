use rgbldk_http_dto::{PeerConnectRequest, PeerDetailsDto, PeerDisconnectRequest};

use crate::app::App;
use crate::cli::PeerCommand;
use crate::client::{join_url, send_json};
use crate::utils::{die, print_json};

pub(crate) async fn handle(app: &App, command: &PeerCommand) {
	match command {
		PeerCommand::Ls => {
			let url = join_url(&app.base, "/api/v1/peers");
			let peers: Vec<PeerDetailsDto> =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&peers, app.pretty),
				crate::ui::OutputMode::Text => {
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
								r[0] = crate::ui::truncate_id(&r[0]);
							}
							r
						})
						.collect::<Vec<_>>();
					crate::ui::print_table(
						app.theme,
						&["Node ID", "Address", "Connected", "Persisted"],
						rows,
					);
				},
			}
		},
		PeerCommand::Connect(args) => {
			let req = PeerConnectRequest {
				node_id: args.node_id.clone(),
				address: args.addr.clone(),
				persist: args.persist,
			};
			let url = join_url(&app.base, "/api/v1/peers/connect");
			let v: serde_json::Value =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => println!("Peer connected."),
			}
		},
		PeerCommand::Disconnect { node_id } => {
			let req = PeerDisconnectRequest { node_id: node_id.clone() };
			let url = join_url(&app.base, "/api/v1/peers/disconnect");
			let v: serde_json::Value =
				send_json(app.client.post(url).json(&req)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => println!("Peer disconnected."),
			}
		},
	}
}
