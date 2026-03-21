use rgbldk_http_dto::EventDto;

use crate::app::App;
use crate::cli::EventsCommand;
use crate::client::{join_url, send_json};
use crate::ui;
use crate::utils::{die, print_json, with_spinner};

pub(crate) async fn handle(app: &App, command: &EventsCommand) {
	match command {
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
				ui::OutputMode::Text => ui::print_event_text(&ev, app.no_truncate),
			}
		},
		EventsCommand::Handled => {
			let url = join_url(&app.base, "/api/v1/events/handled");
			let v: serde_json::Value = send_json(app.client.post(url).json(&serde_json::json!({})))
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
					ui::OutputMode::Text => ui::print_event_text(&ev, app.no_truncate),
				}
				seen += 1;
			}
		},
	}
}
