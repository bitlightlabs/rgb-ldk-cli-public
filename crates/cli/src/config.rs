use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CliConfig {
	#[serde(default)]
	pub current: Option<String>,
	#[serde(default)]
	pub contexts: BTreeMap<String, CtxEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxEntry {
	pub url: String,
	#[serde(default)]
	pub token_file: Option<String>,
}

fn home_dir() -> Option<PathBuf> {
	std::env::var_os("HOME").map(PathBuf::from)
}

fn xdg_config_home() -> Option<PathBuf> {
	std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from)
}

pub fn default_config_path() -> Result<PathBuf, String> {
	let base = if let Some(xdg) = xdg_config_home() {
		xdg
	} else if let Some(home) = home_dir() {
		home.join(".config")
	} else {
		return Err("cannot determine config dir (missing $HOME and $XDG_CONFIG_HOME)".to_string());
	};
	Ok(base.join("rgbldk").join("config.toml"))
}

pub fn load_config(path: &Path) -> Result<CliConfig, String> {
	if !path.exists() {
		return Ok(CliConfig::default());
	}
	let s = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
	toml::from_str(&s).map_err(|e| format!("parse {}: {e}", path.display()))
}

pub fn save_config(path: &Path, cfg: &CliConfig) -> Result<(), String> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
	}
	let s = toml::to_string_pretty(cfg).map_err(|e| e.to_string())?;
	std::fs::write(path, s).map_err(|e| format!("write {}: {e}", path.display()))
}

pub fn resolve_connect(cli_connect: Option<String>, cfg: &CliConfig) -> String {
	if let Some(c) = cli_connect {
		return c;
	}
	if let Ok(u) = std::env::var("RGBLDK_URL") {
		if !u.trim().is_empty() {
			return u;
		}
	}
	if let Ok(u) = std::env::var("RGBLDK_CONNECT") {
		if !u.trim().is_empty() {
			return u;
		}
	}
	if let Ok(name) = std::env::var("RGBLDK_CTX") {
		let name = name.trim();
		if !name.is_empty() {
			if let Some(ctx) = cfg.contexts.get(name) {
				return ctx.url.clone();
			}
		}
	}
	if let Some(cur) = cfg.current.as_ref() {
		if let Some(ctx) = cfg.contexts.get(cur) {
			return ctx.url.clone();
		}
	}
	"http://127.0.0.1:8500".to_string()
}

pub fn resolve_token_file(cli_token_file: Option<String>, cfg: &CliConfig) -> Option<String> {
	if cli_token_file.is_some() {
		return cli_token_file;
	}
	if let Ok(name) = std::env::var("RGBLDK_CTX") {
		let name = name.trim();
		if !name.is_empty() {
			if let Some(ctx) = cfg.contexts.get(name) {
				if ctx.token_file.is_some() {
					return ctx.token_file.clone();
				}
			}
		}
	}
	if let Some(cur) = cfg.current.as_ref() {
		if let Some(ctx) = cfg.contexts.get(cur) {
			if ctx.token_file.is_some() {
				return ctx.token_file.clone();
			}
		}
	}
	None
}
