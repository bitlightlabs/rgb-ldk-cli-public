mod app;
mod cli;
mod client;
mod cmd;
mod config;
mod control;
mod keystore_local;
mod ui;
mod utils;

pub use crate::cli::OutputOpt;

use crate::cli::{Cli, ColorOpt, Command};
use crate::utils::die;
use clap::Parser;

use crate::app::App;

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	let cfg_path = config::default_config_path().unwrap_or_else(|e| die(e));
	let mut cfg = config::load_config(&cfg_path).unwrap_or_else(|e| die(e));

	let output = ui::resolve_output_mode(cli.output);
	let theme = ui::resolve_theme(match cli.color {
		ColorOpt::Auto => ui::ColorMode::Auto,
		ColorOpt::Always => ui::ColorMode::Always,
		ColorOpt::Never => ui::ColorMode::Never,
	});
	let needs_http = matches!(
		cli.command,
		Command::Node { .. }
			| Command::Wallet { .. }
			| Command::Rgb { .. }
			| Command::Peer { .. }
			| Command::Channel { .. }
			| Command::Graph { .. }
			| Command::Pay { .. }
			| Command::Swap { .. }
			| Command::Events { .. }
	);

	let base = config::resolve_connect(cli.connect.clone(), &cfg);
	let client = if needs_http {
		let token = if cli.token_stdin {
			Some(utils::read_stdin_trimmed("token").unwrap_or_else(|e| die(e)))
		} else if let Some(t) = cli.token.clone() {
			Some(t.trim().to_string())
		} else {
			config::resolve_token_file(cli.token_file.clone(), &cfg)
				.map(|p| utils::read_file_trimmed(&p).unwrap_or_else(|e| die(e)))
		};
		if let Some(t) = token.as_deref() {
			if t.len() < 16 {
				die("token must be at least 16 characters");
			}
		}
		if let Some(token) = token {
			let mut headers = reqwest::header::HeaderMap::new();
			let v = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
				.unwrap_or_else(|e| die(e.to_string()));
			headers.insert(reqwest::header::AUTHORIZATION, v);
			reqwest::Client::builder()
				.default_headers(headers)
				.build()
				.unwrap_or_else(|e| die(e.to_string()))
		} else {
			reqwest::Client::new()
		}
	} else {
		reqwest::Client::new()
	};
	let app = App {
		base,
		output,
		pretty: cli.pretty,
		theme,
		yes: cli.yes,
		no_truncate: cli.no_truncate,
		client,
	};

	match cli.command {
		Command::Ctx { ref command } => {
			cmd::ctx::handle(&app, &cfg_path, &mut cfg, command);
		},

		Command::Keystore { ref command } => {
			cmd::keystore::handle(&app, &cli, command);
		},

		Command::Node { ref command } => {
			cmd::node::handle(&app, &cli, command).await;
		},

		Command::Wallet { ref command } => {
			cmd::wallet::handle(&app, command).await;
		},

		Command::Rgb { ref command } => {
			cmd::rgb::handle(&app, command).await;
		},

		Command::Peer { ref command } => {
			cmd::peer::handle(&app, command).await;
		},

		Command::Channel { ref command } => {
			cmd::channel::handle(&app, command).await;
		},

		Command::Graph { ref command } => {
			cmd::graph::handle(&app, command).await;
		},

		Command::Pay { ref command } => {
			cmd::pay::handle(&app, command).await;
		},

		Command::Swap { ref command } => {
			cmd::swap::handle(&app, command).await;
		},

		Command::Events { ref command } => {
			cmd::events::handle(&app, command).await;
		},

		Command::Debug { ref command } => {
			cmd::debug::handle(&app, command).await;
		},
	}
}
