use owo_colors::OwoColorize;

use rgbldk_http_dto::{OkResponse, StatusDto};

use crate::app::App;
use crate::cli::{Cli, NodeCommand};
use crate::client::{join_url, send_json, send_json_allow_status, send_value};
use crate::control;
use crate::utils::{
	confirm_or_exit, die, print_json, read_file_trimmed, read_passphrase, read_stdin_trimmed,
};

pub(crate) async fn handle(app: &App, cli: &Cli, command: &NodeCommand) {
	match command {
		NodeCommand::Version => {
			let url = join_url(&app.base, "/api/v1/version");
			let v: serde_json::Value =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					let mut rows = Vec::new();
					if let Some(obj) = v.as_object() {
						for (k, vv) in obj {
							rows.push(vec![
								k.clone(),
								vv.as_str().unwrap_or(&vv.to_string()).to_string(),
							]);
						}
					}
					crate::ui::print_table(app.theme, &["Field", "Value"], rows);
				},
			}
		},
		NodeCommand::Health => {
			let url = join_url(&app.base, "/api/v1/healthz");
			let v: OkResponse = send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					crate::ui::print_checks(app.theme, "node health", v.ok, &v.checks)
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
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					crate::ui::print_checks(app.theme, "node ready", v.ok, &v.checks)
				},
			}
			if !v.ok {
				std::process::exit(1);
			}
		},
		NodeCommand::Status => {
			let url = join_url(&app.base, "/api/v1/status");
			let (_st, v) = send_value(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			if v.get("locked").is_some() {
				match app.output {
					crate::ui::OutputMode::Json => print_json(&v, app.pretty),
					crate::ui::OutputMode::Text => {
						println!("locked=true");
						if let Some(checks) = v.get("checks") {
							println!("checks={}", checks);
						}
					},
				}
			} else {
				let s: StatusDto = serde_json::from_value(v).unwrap_or_else(|e| die(e.to_string()));
				match app.output {
					crate::ui::OutputMode::Json => print_json(&s, app.pretty),
					crate::ui::OutputMode::Text => {
						let rows = vec![
							vec!["is_running".into(), s.is_running.to_string()],
							vec!["p2p_is_listening".into(), s.is_listening.to_string()],
							vec!["best_block_height".into(), s.best_block_height.to_string()],
						];
						crate::ui::print_table(app.theme, &["Field", "Value"], rows);
					},
				}
			}
		},
		NodeCommand::Id => {
			let url = join_url(&app.base, "/api/v1/node_id");
			let v: serde_json::Value =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					println!("{}", v.get("node_id").and_then(|v| v.as_str()).unwrap_or(""))
				},
			}
		},
		NodeCommand::Listen => {
			let url = join_url(&app.base, "/api/v1/listening_addresses");
			let v: serde_json::Value =
				send_json(app.client.get(url)).await.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					let addrs =
						v.get("addresses").and_then(|v| v.as_array()).cloned().unwrap_or_default();
					for a in addrs {
						if let Some(s) = a.as_str() {
							println!("{s}");
						}
					}
				},
			}
		},
		NodeCommand::Unlock(args) => {
			let socket = control::default_control_socket_path(
				cli.control_socket.as_deref(),
				cli.data_dir.as_deref(),
			);
			let passphrase = read_passphrase(args.passphrase_stdin).unwrap_or_else(|e| die(e));
			let resp = control::call_control(
				&socket,
				"unlock",
				serde_json::json!({ "passphrase": passphrase }),
			)
			.await
			.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&resp, app.pretty),
				crate::ui::OutputMode::Text => {
					if resp.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
						if app.theme.color {
							println!("{} Unlocked", app.theme.ok.green());
						} else {
							println!("{} Unlocked", app.theme.ok);
						}
					} else {
						die(resp.get("error").and_then(|v| v.as_str()).unwrap_or("unlock failed"));
					}
				},
			}
		},
		NodeCommand::UnlockHosted(args) => {
			let token = if args.control_token_stdin {
				read_stdin_trimmed("control token").unwrap_or_else(|e| die(e))
			} else if let Some(p) = args.control_token_file.as_deref() {
				read_file_trimmed(p).unwrap_or_else(|e| die(e))
			} else if let Some(t) = args.control_token.as_deref() {
				t.trim().to_string()
			} else {
				die("missing control token (use --control-token-stdin, --control-token-file, or RGBLDK_CONTROL_HTTP_TOKEN)");
			};
			if token.is_empty() {
				die("control token must not be empty");
			}

			let url = join_url(&args.control_connect, "/control/unlock");
			let (status, v) = send_value(
				app.client
					.post(url)
					.header("Authorization", format!("Bearer {token}"))
					.json(&serde_json::json!({})),
			)
			.await
			.unwrap_or_else(|e| die(e));

			match app.output {
				crate::ui::OutputMode::Json => print_json(&v, app.pretty),
				crate::ui::OutputMode::Text => {
					if status.is_success() && v.get("ok").and_then(|b| b.as_bool()).unwrap_or(false)
					{
						if app.theme.color {
							println!("{} Unlocked", app.theme.ok.green());
						} else {
							println!("{} Unlocked", app.theme.ok);
						}
					} else {
						let err =
							v.get("error").and_then(|e| e.as_str()).unwrap_or("unlock failed");
						let hint = v.get("hint").and_then(|h| h.as_str());
						if let Some(h) = hint {
							die(format!("HTTP {}: {err} (hint: {h})", status.as_u16()));
						} else {
							die(format!("HTTP {}: {err}", status.as_u16()));
						}
					}
				},
			}
		},
		NodeCommand::Lock => {
			confirm_or_exit(
				app.yes,
				"This will stop the node and wipe secrets. Type 'yes' to continue.",
			);
			let socket = control::default_control_socket_path(
				cli.control_socket.as_deref(),
				cli.data_dir.as_deref(),
			);
			let resp = control::call_control(&socket, "lock", serde_json::json!({ "yes": true }))
				.await
				.unwrap_or_else(|e| die(e));
			match app.output {
				crate::ui::OutputMode::Json => print_json(&resp, app.pretty),
				crate::ui::OutputMode::Text => {
					if resp.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
						if app.theme.color {
							println!("{} Locked", app.theme.ok.green());
						} else {
							println!("{} Locked", app.theme.ok);
						}
					} else {
						die(resp.get("error").and_then(|v| v.as_str()).unwrap_or("lock failed"));
					}
				},
			}
		},
	}
}
