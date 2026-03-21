use crate::ui;

pub(crate) struct App {
	pub(crate) base: String,
	pub(crate) output: ui::OutputMode,
	pub(crate) pretty: bool,
	pub(crate) theme: ui::Theme,
	pub(crate) yes: bool,
	pub(crate) no_truncate: bool,
	pub(crate) client: reqwest::Client,
}
