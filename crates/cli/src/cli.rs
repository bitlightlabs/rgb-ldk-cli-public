use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum OutputOpt {
	Auto,
	Text,
	Json,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ColorOpt {
	Auto,
	Always,
	Never,
}

#[derive(Parser, Debug)]
#[command(name = "rgbldk", about = "Human-friendly CLI for talking to rgbldkd over HTTP")]
#[command(
	subcommand_required = true,
	arg_required_else_help = true,
	disable_help_subcommand = true
)]
pub struct Cli {
	/// Target daemon base URL (overrides ctx/env/default).
	#[arg(long)]
	pub connect: Option<String>,

	/// Output mode (auto=text on TTY, json otherwise).
	#[arg(long, value_enum, default_value_t = OutputOpt::Auto)]
	pub output: OutputOpt,

	/// Color output mode.
	#[arg(long, value_enum, default_value_t = ColorOpt::Auto)]
	pub color: ColorOpt,

	/// Pretty-print JSON output (only applies to `--output json|auto` when json is selected).
	#[arg(long)]
	pub pretty: bool,

	/// Assume yes for prompts (for destructive operations).
	#[arg(long)]
	pub yes: bool,

	/// Do not truncate long IDs in tables.
	#[arg(long)]
	pub no_truncate: bool,

	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	/// Manage connection contexts.
	Ctx {
		#[command(subcommand)]
		command: CtxCommand,
	},
	/// Node-level operations (health/ready/status).
	Node {
		#[command(subcommand)]
		command: NodeCommand,
	},
	/// Wallet operations.
	Wallet {
		#[command(subcommand)]
		command: WalletCommand,
	},
	/// Peer operations.
	Peer {
		#[command(subcommand)]
		command: PeerCommand,
	},
	/// Channel operations.
	Channel {
		#[command(subcommand)]
		command: ChannelCommand,
	},
	/// Payments (BOLT11 invoices, BOLT12 offers/refunds, keysend).
	Pay {
		#[command(subcommand)]
		command: PayCommand,
	},
	/// Event queue operations.
	Events {
		#[command(subcommand)]
		command: EventsCommand,
	},
}

#[derive(Subcommand, Debug)]
pub enum CtxCommand {
	/// List contexts.
	Ls,
	/// Show the current context.
	Show,
	/// Add or update a context.
	Add(CtxAddArgs),
	/// Use a context as default.
	Use { name: String },
	/// Remove a context.
	Rm { name: String },
}

#[derive(Args, Debug)]
pub struct CtxAddArgs {
	pub name: String,
	#[arg(long)]
	pub url: String,
	/// Set as current after adding.
	#[arg(long)]
	pub use_now: bool,
}

#[derive(Subcommand, Debug)]
pub enum NodeCommand {
	Health,
	Ready,
	Status,
	Id,
	Listen,
	Version,
}

#[derive(Subcommand, Debug)]
pub enum WalletCommand {
	Balance(WalletBalanceArgs),
	Address,
	Sync,
}

#[derive(Args, Debug, Default)]
pub struct WalletBalanceArgs {
	/// Force displaying balances in sats (no BTC conversion, no separators).
	#[arg(long)]
	pub sats: bool,
}

#[derive(Subcommand, Debug)]
pub enum PeerCommand {
	Ls,
	Connect(PeerConnectArgs),
	Disconnect { node_id: String },
}

#[derive(Args, Debug)]
pub struct PeerConnectArgs {
	pub node_id: String,
	pub addr: String,
	#[arg(long)]
	pub persist: bool,
}

#[derive(Subcommand, Debug)]
pub enum ChannelCommand {
	Ls,
	Open(ChannelOpenArgs),
	Close(ChannelCloseArgs),
	ForceClose(ChannelCloseArgs),
}

#[derive(Args, Debug)]
pub struct ChannelOpenArgs {
	#[arg(long)]
	pub node_id: String,
	#[arg(long)]
	pub addr: String,
	#[arg(long)]
	pub amount_sats: u64,
	#[arg(long)]
	pub push_msat: Option<u64>,
	/// Create a private channel (unannounced).
	#[arg(long)]
	pub private: bool,
}

#[derive(Args, Debug)]
pub struct ChannelCloseArgs {
	#[arg(long)]
	pub user_channel_id: String,
	#[arg(long)]
	pub counterparty_node_id: String,
}

#[derive(Subcommand, Debug)]
pub enum PayCommand {
	/// BOLT11 invoice operations (create/pay).
	Invoice {
		#[command(subcommand)]
		command: InvoiceCommand,
	},

	/// BOLT12 offer operations (create/decode/pay).
	Offer {
		#[command(subcommand)]
		command: OfferCommand,
	},

	/// BOLT12 refund operations (initiate/decode/request-payment).
	Refund {
		#[command(subcommand)]
		command: RefundCommand,
	},

	/// Spontaneous (keysend) payment operations.
	Keysend {
		#[command(subcommand)]
		command: KeysendCommand,
	},

	/// List known payments.
	Ls,

	/// Wait for a payment to reach a terminal state.
	Wait(PaymentWaitArgs),

	/// Abandon an in-flight outbound payment (useful for BOLT12 awaiting an invoice).
	Abandon { payment_id: String },

	/// Get details for a payment id.
	Get { payment_id: String },
}

#[derive(Subcommand, Debug)]
pub enum InvoiceCommand {
	/// Create an invoice (fixed amount if `--amount-msat` is set, variable otherwise).
	Create(InvoiceCreateArgs),
	/// Pay an invoice (uses `--amount-msat` for variable-amount invoices).
	Pay(InvoicePayArgs),
}

#[derive(Args, Debug)]
pub struct InvoiceCreateArgs {
	#[arg(long)]
	pub desc: String,
	#[arg(long)]
	pub amount_msat: Option<u64>,
	#[arg(long, default_value_t = 3600)]
	pub expiry_secs: u32,
}

#[derive(Args, Debug)]
pub struct InvoicePayArgs {
	#[arg(long)]
	pub invoice: String,
	#[arg(long)]
	pub amount_msat: Option<u64>,
}

#[derive(Subcommand, Debug)]
pub enum OfferCommand {
	/// Create an offer (fixed amount if `--amount-msat` is set, variable otherwise).
	Create(OfferCreateArgs),
	/// Decode an offer.
	Decode { offer: String },
	/// Pay an offer.
	Pay(OfferPayArgs),
}

#[derive(Args, Debug)]
pub struct OfferCreateArgs {
	#[arg(long)]
	pub desc: String,
	#[arg(long)]
	pub amount_msat: Option<u64>,
	/// Offer expiry in seconds from now. Use `--no-expiry` for no expiry.
	#[arg(long, default_value_t = 3600)]
	pub expiry_secs: u32,
	/// Do not set an expiry on the offer.
	#[arg(long)]
	pub no_expiry: bool,
	/// Optional item quantity.
	#[arg(long)]
	pub quantity: Option<u64>,
}

#[derive(Args, Debug)]
pub struct OfferSendArgs {
	#[arg(long)]
	pub offer: String,
	/// Required for zero-amount offers; may be used to overpay fixed-amount offers.
	#[arg(long)]
	pub amount_msat: Option<u64>,
	#[arg(long)]
	pub quantity: Option<u64>,
	#[arg(long)]
	pub payer_note: Option<String>,
}

pub type OfferPayArgs = OfferSendArgs;

#[derive(Subcommand, Debug)]
pub enum RefundCommand {
	/// Initiate a refund (payer-side), returning an encoded refund string.
	Initiate(RefundInitiateArgs),
	/// Decode a refund.
	Decode { refund: String },
	/// Request the refund payment (payee-side), sending an invoice via onion messages.
	RequestPayment { refund: String },
}

#[derive(Args, Debug)]
pub struct RefundInitiateArgs {
	#[arg(long)]
	pub amount_msat: u64,
	#[arg(long, default_value_t = 3600)]
	pub expiry_secs: u32,
	#[arg(long)]
	pub quantity: Option<u64>,
	#[arg(long)]
	pub payer_note: Option<String>,
}

#[derive(Args, Debug)]
pub struct PaymentWaitArgs {
	pub payment_id: String,
	#[arg(long)]
	pub timeout_secs: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct TlvArg {
	pub r#type: u64,
	pub value_hex: String,
}

impl std::str::FromStr for TlvArg {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (t, hex) =
			s.split_once(':').ok_or_else(|| "invalid --tlv (expected <type>:<hex>)".to_string())?;
		let type_num: u64 = t.parse().map_err(|_| "invalid --tlv type".to_string())?;
		Ok(Self { r#type: type_num, value_hex: hex.to_string() })
	}
}

#[derive(Args, Debug)]
pub struct KeysendArgs {
	#[arg(long)]
	pub node_id: String,
	#[arg(long)]
	pub amount_msat: u64,
	#[arg(long, value_name = "<type>:<hex>")]
	pub tlv: Vec<TlvArg>,
}

#[derive(Subcommand, Debug)]
pub enum KeysendCommand {
	/// Send a spontaneous (keysend) payment.
	Send(KeysendArgs),
}

#[derive(Subcommand, Debug)]
pub enum EventsCommand {
	Next,
	Handled,
	Watch(EventsWatchArgs),
}

#[derive(Args, Debug)]
pub struct EventsWatchArgs {
	/// Stop after N events (default: run forever).
	#[arg(long)]
	pub count: Option<u64>,
}
