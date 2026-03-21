use crate::app::App;
use crate::cli::CtxCommand;
use crate::config;
use crate::utils::{die, print_json};

pub(crate) fn handle(
	app: &App, cfg_path: &std::path::Path, cfg: &mut config::CliConfig, command: &CtxCommand,
) {
	match command {
		CtxCommand::Ls => match app.output {
			crate::ui::OutputMode::Json => print_json(cfg, app.pretty),
			crate::ui::OutputMode::Text => {
				let rows = cfg
					.contexts
					.iter()
					.map(|(name, c)| {
						vec![
							if cfg.current.as_deref() == Some(name) {
								"*".into()
							} else {
								"".into()
							},
							name.clone(),
							c.url.clone(),
						]
					})
					.collect::<Vec<_>>();
				crate::ui::print_table(app.theme, &["Current", "Name", "URL"], rows);
			},
		},
		CtxCommand::Show => {
			if let Some(cur) = cfg.current.as_ref() {
				if let Some(ctx) = cfg.contexts.get(cur) {
					match app.output {
						crate::ui::OutputMode::Json => print_json(ctx, app.pretty),
						crate::ui::OutputMode::Text => {
							println!("{cur} -> {}", ctx.url);
							if let Some(p) = ctx.token_file.as_deref() {
								println!("token_file={p}");
							}
						},
					}
				} else {
					die(format!("current context {cur} not found in config"));
				}
			} else {
				die("no current context set");
			}
		},
		CtxCommand::Add(args) => {
			let name = args.name.clone();
			cfg.contexts.insert(
				args.name.clone(),
				config::CtxEntry { url: args.url.clone(), token_file: args.token_file.clone() },
			);
			if args.use_now || cfg.current.is_none() {
				cfg.current = Some(args.name.clone());
			}
			config::save_config(cfg_path, cfg).unwrap_or_else(|e| die(e));
			if app.output == crate::ui::OutputMode::Text {
				let active = cfg.current.as_deref() == Some(name.as_str());
				if active {
					println!("Context \"{name}\" created and set as active.");
				} else {
					println!("Context \"{name}\" created.");
				}
				println!("Next: run `rgbldk node status` to verify the connection.");
			} else {
				print_json(cfg, app.pretty);
			}
		},
		CtxCommand::Use { name } => {
			if !cfg.contexts.contains_key(name) {
				die(format!("context not found: {name}"));
			}
			cfg.current = Some(name.clone());
			config::save_config(cfg_path, cfg).unwrap_or_else(|e| die(e));
			if app.output == crate::ui::OutputMode::Text {
				let cur = cfg.current.as_deref().unwrap_or("");
				println!("Switched to context \"{cur}\".");
				println!("Next: run `rgbldk node status` to verify the connection.");
			} else {
				print_json(cfg, app.pretty);
			}
		},
		CtxCommand::Rm { name } => {
			cfg.contexts.remove(name);
			if cfg.current.as_deref() == Some(name.as_str()) {
				cfg.current = None;
			}
			config::save_config(cfg_path, cfg).unwrap_or_else(|e| die(e));
			if app.output == crate::ui::OutputMode::Text {
				println!("Context \"{name}\" removed.");
			} else {
				print_json(cfg, app.pretty);
			}
		},
	}
}
