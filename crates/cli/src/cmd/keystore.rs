use crate::app::App;
use crate::cli::{Cli, KeystoreCommand};
use crate::keystore_local;
use crate::ui;
use crate::utils::{confirm_or_exit, die, print_json};

pub(crate) fn handle(app: &App, cli: &Cli, command: &KeystoreCommand) {
	match command {
		KeystoreCommand::Init(args) => {
			let prepared = keystore_local::prepare_keystore_init(cli, args)
				.unwrap_or_else(|e| die(e.to_string()));

			if let Some(m) = prepared.mnemonic.as_deref() {
				if app.output == ui::OutputMode::Text {
					println!("mnemonic={m}");
				}
				confirm_or_exit(
					app.yes,
					"Write down the mnemonic now. Losing it may make funds unrecoverable.",
				);
			}

			keystore_local::commit_keystore_init(&prepared).unwrap_or_else(|e| die(e.to_string()));

			match app.output {
				ui::OutputMode::Json => print_json(
					&serde_json::json!({
						"ok": true,
						"keystore_path": prepared.keystore_path,
						"mnemonic": prepared.mnemonic,
					}),
					app.pretty,
				),
				ui::OutputMode::Text => {
					println!("ok=true");
					println!("keystore_path={}", prepared.keystore_path.display());
					if prepared.mnemonic.is_some() {
						println!("next=store the mnemonic safely");
					}
				},
			}
		},
		KeystoreCommand::Migrate(args) => {
			let (keystore_path, backup_path) =
				keystore_local::keystore_migrate(cli, args).unwrap_or_else(|e| die(e.to_string()));
			match app.output {
				ui::OutputMode::Json => print_json(
					&serde_json::json!({
						"ok": true,
						"keystore_path": keystore_path,
						"legacy_backup_path": backup_path,
					}),
					app.pretty,
				),
				ui::OutputMode::Text => {
					println!("ok=true");
					println!("keystore_path={}", keystore_path.display());
					println!("legacy_backup_path={}", backup_path.display());
				},
			}
		},
	}
}
