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

	/// Bearer token for the main node HTTP API.
	#[arg(long, env = "RGBLDK_HTTP_TOKEN", hide_env_values = true)]
	pub token: Option<String>,

	/// Read the bearer token for the main node HTTP API from stdin.
	#[arg(long, conflicts_with_all = ["token", "token_file"])]
	pub token_stdin: bool,

	/// Read the bearer token for the main node HTTP API from a file.
	#[arg(long, env = "RGBLDK_HTTP_TOKEN_FILE")]
	pub token_file: Option<String>,

	/// Local node data directory (used for keystore + control socket defaults).
	#[arg(long)]
	pub data_dir: Option<String>,

	/// Local control socket path for lock/unlock (Unix domain socket).
	///
	/// Default: <data_dir>/control.sock, or /tmp/ldk_node/control.sock if `--data-dir` is omitted.
	#[arg(long)]
	pub control_socket: Option<String>,

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
	/// Local keystore operations (encrypted root secret).
	Keystore {
		#[command(subcommand)]
		command: KeystoreCommand,
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
	/// RGB asset operations (Lightning + on-chain).
	Rgb {
		#[command(subcommand)]
		command: RgbCommand,
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
	/// Offline debugging helpers (invoice parsing, consignment inspection).
	Debug {
		#[command(subcommand)]
		command: DebugCommand,
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
	/// Optional token file for the main node HTTP API (added to the context config).
	#[arg(long)]
	pub token_file: Option<String>,
	/// Set as current after adding.
	#[arg(long)]
	pub use_now: bool,
}

#[derive(Subcommand, Debug)]
pub enum DebugCommand {
	/// Parse an RGB invoice URI and print its fields.
	Invoice(DebugInvoiceArgs),
	/// Inspect a consignment archive and optionally check for specific outpoints.
	Consignment(DebugConsignmentArgs),
}

#[derive(Args, Debug)]
pub struct DebugInvoiceArgs {
	/// RGB invoice URI string (scheme `contract:`).
	pub invoice: String,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum DebugConsignmentFormat {
	Auto,
	Raw,
	Gzip,
	Zip,
}

#[derive(Args, Debug)]
pub struct DebugConsignmentArgs {
	/// Path to a consignment archive (raw/gzip/zip). Use `-` for stdin.
	pub path: String,

	/// Archive format (default: auto-detect).
	#[arg(long, value_enum, default_value_t = DebugConsignmentFormat::Auto)]
	pub format: DebugConsignmentFormat,

	/// Optional outpoint(s) to check for presence (format: `txid:vout`).
	#[arg(long)]
	pub outpoint: Vec<String>,

	/// Optional invoice URI; if it contains an explicit seal/noise, it will also be checked.
	#[arg(long)]
	pub invoice: Option<String>,

	/// Limit how many extracted outpoints are printed in text mode (default: 50).
	#[arg(long, default_value_t = 50)]
	pub max_outpoints: usize,
}

#[derive(Subcommand, Debug)]
pub enum NodeCommand {
	Health,
	Ready,
	Status,
	Id,
	Listen,
	Version,
	/// Lock the daemon (graceful stop + wipe secrets).
	Lock,
	/// Unlock the daemon (requires passphrase).
	Unlock(UnlockArgs),
	/// Unlock a hosted daemon via the authenticated control HTTP server (no passphrase sent).
	UnlockHosted(UnlockHostedArgs),
}

#[derive(Args, Debug)]
pub struct UnlockArgs {
	/// Read the passphrase from stdin (recommended for automation/native hosts).
	#[arg(long)]
	pub passphrase_stdin: bool,
}

#[derive(Args, Debug)]
pub struct UnlockHostedArgs {
	/// Control HTTP base URL (for example http://127.0.0.1:8550).
	#[arg(long, env = "RGBLDK_CONTROL_CONNECT", default_value = "http://127.0.0.1:8550")]
	pub control_connect: String,

	/// Control HTTP bearer token (prefer --control-token-stdin or --control-token-file).
	#[arg(long, env = "RGBLDK_CONTROL_HTTP_TOKEN", hide_env_values = true)]
	pub control_token: Option<String>,

	/// Read control HTTP bearer token from stdin.
	#[arg(long, conflicts_with_all = ["control_token", "control_token_file"])]
	pub control_token_stdin: bool,

	/// Read control HTTP bearer token from a file.
	#[arg(long, env = "RGBLDK_CONTROL_HTTP_TOKEN_FILE")]
	pub control_token_file: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum KeystoreCommand {
	/// Initialize a new encrypted keystore.
	Init(KeystoreInitArgs),
	/// Migrate a legacy plaintext keys_seed file into an encrypted keystore.
	Migrate(KeystoreMigrateArgs),
}

#[derive(Args, Debug)]
pub struct KeystoreInitArgs {
	/// Initialization mode.
	#[arg(long, value_enum, default_value_t = KeystoreInitMode::GenerateMnemonic)]
	pub mode: KeystoreInitMode,
	/// Keystore passphrase (read from stdin if set).
	#[arg(long)]
	pub passphrase_stdin: bool,
	/// Mnemonic to import (required for `--mode import-mnemonic`).
	#[arg(long)]
	pub mnemonic: Option<String>,
	/// Seed bytes to import as hex (required for `--mode import-seed`).
	#[arg(long)]
	pub seed_hex: Option<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum KeystoreInitMode {
	GenerateMnemonic,
	ImportMnemonic,
	ImportSeed,
}

#[derive(Args, Debug)]
pub struct KeystoreMigrateArgs {
	/// Keystore passphrase (read from stdin if set).
	#[arg(long)]
	pub passphrase_stdin: bool,
	/// Legacy plaintext seed filename (defaults to <data_dir>/keys_seed).
	#[arg(long)]
	pub legacy_seed_path: Option<String>,
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
	/// Open an RGB-enabled channel (requires all `--rgb-*` args).
	#[arg(long)]
	pub rgb_asset_id: Option<String>,
	#[arg(long)]
	pub rgb_asset_amount: Option<u64>,
	#[arg(long)]
	pub rgb_context: Option<String>,
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
pub enum RgbCommand {
	/// Sync RGB wallet state with the indexer.
	Sync,
	/// Get a new RGB wallet address.
	Address,
	/// RGB wallet UTXO operations (list/reserve/release).
	Utxos {
		#[command(subcommand)]
		command: RgbUtxosCommand,
	},
	/// RGB issuer registry (for asset issuance).
	Issuers {
		#[command(subcommand)]
		command: RgbIssuersCommand,
	},
	/// RGB contracts inventory.
	Contracts {
		#[command(subcommand)]
		command: RgbContractsCommand,
	},
	/// RGB consignment cache operations.
	Consignments {
		#[command(subcommand)]
		command: RgbConsignmentsCommand,
	},
	/// RGB Lightning (BOLT11 with RGB fields).
	Ln {
		#[command(subcommand)]
		command: RgbLnCommand,
	},
	/// RGB on-chain payments.
	Onchain {
		#[command(subcommand)]
		command: RgbOnchainCommand,
	},
}

#[derive(Subcommand, Debug)]
pub enum RgbUtxosCommand {
	/// List wallet UTXOs known to the RGB wallet (outpoints `txid:vout`).
	Ls,
	/// Show UTXO details (value/confirmation/reservation).
	Summary,
	/// Reserve a UTXO for upcoming RGB operations.
	Reserve(RgbUtxosReserveArgs),
	/// Release a reservation by id or outpoint.
	Release(RgbUtxosReleaseArgs),
}

#[derive(Args, Debug)]
pub struct RgbUtxosReserveArgs {
	/// Optional explicit outpoint to reserve (`txid:vout`). If omitted, the node auto-selects one.
	#[arg(long)]
	pub outpoint: Option<String>,
	/// Optional reservation TTL in seconds (default: 300).
	#[arg(long)]
	pub ttl_secs: Option<u64>,
}

#[derive(Args, Debug)]
pub struct RgbUtxosReleaseArgs {
	/// Release by reservation id (preferred).
	#[arg(long, conflicts_with = "outpoint")]
	pub reservation_id: Option<String>,
	/// Release by outpoint (`txid:vout`).
	#[arg(long, conflicts_with = "reservation_id")]
	pub outpoint: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum RgbContractsCommand {
	/// List known RGB contracts.
	Ls,
	/// Show contract balance.
	Balance { contract_id: String },
	/// Check whether a contract is known to the node.
	Known { contract_id: String },
	/// Import a contract consignment archive (raw/gzip/zip).
	Import(RgbContractsImportArgs),
	/// Issue an RGB contract (asset).
	Issue(RgbContractsIssueArgs),
	/// Export a contract into a consignment file (raw/gzip/zip).
	Export(RgbContractsExportArgs),
}

#[derive(Subcommand, Debug)]
pub enum RgbConsignmentsCommand {
	/// Download a consignment (raw/gzip/zip) from the daemon by consignment key.
	Download(RgbConsignmentsDownloadArgs),
}

#[derive(Args, Debug)]
pub struct RgbConsignmentsDownloadArgs {
	/// Consignment key in the daemon's consignment cache.
	#[arg(long)]
	pub key: String,
	/// Output path for the downloaded bytes.
	#[arg(long)]
	pub out: String,
	/// Response encoding: `raw` (default), `gzip`, `zip`.
	#[arg(long, default_value = "zip")]
	pub format: String,
}

#[derive(Args, Debug)]
pub struct RgbContractsImportArgs {
	#[arg(long)]
	pub contract_id: String,
	/// Path to the consignment archive (raw consignment file, `.gz`, or `.zip`).
	#[arg(long)]
	pub file: String,
	/// Archive format: `auto` (default), `raw`, `gzip`, `zip`.
	#[arg(long, default_value = "auto")]
	pub format: String,
}

#[derive(Args, Debug)]
pub struct RgbContractsIssueArgs {
	/// Issuer name (must exist in the local issuer registry).
	#[arg(long)]
	pub issuer_name: String,
	/// Contract name.
	#[arg(long)]
	pub contract_name: String,
	/// Optional ticker override.
	#[arg(long)]
	pub ticker: Option<String>,
	/// Optional precision override.
	#[arg(long)]
	pub precision: Option<u8>,
	/// Total issued supply.
	#[arg(long)]
	pub issued_supply: u64,
	/// Optional issuance outpoint (`txid:vout`). If omitted, a wallet UTXO is auto-selected.
	#[arg(long)]
	pub utxo: Option<String>,
}

#[derive(Args, Debug)]
pub struct RgbContractsExportArgs {
	#[arg(long)]
	pub contract_id: String,
	/// Output path for the exported consignment bytes.
	#[arg(long)]
	pub out: String,
	/// Response encoding: `raw` (default), `gzip`, `zip`.
	#[arg(long, default_value = "raw")]
	pub format: String,
	/// Use one-step export+download (`/rgb/contracts/export?download=true`) instead of 2-step download.
	#[arg(long)]
	pub direct: bool,
}

#[derive(Subcommand, Debug)]
pub enum RgbIssuersCommand {
	/// List known issuer names in the local issuer registry.
	Ls,
	/// Import an issuer archive (raw/gzip/zip) into the local issuer registry.
	Import(RgbIssuersImportArgs),
}

#[derive(Args, Debug)]
pub struct RgbIssuersImportArgs {
	#[arg(long)]
	pub name: String,
	/// Path to the issuer archive (raw `.issuer` file, `.gz`, or `.zip`).
	#[arg(long)]
	pub file: String,
	/// Archive format: `auto` (default), `raw`, `gzip`, `zip`.
	#[arg(long, default_value = "auto")]
	pub format: String,
}

#[derive(Subcommand, Debug)]
pub enum RgbLnCommand {
	/// Create or decode RGB LN invoices.
	Invoice {
		#[command(subcommand)]
		command: RgbLnInvoiceCommand,
	},
	/// Pay an RGB LN invoice.
	Pay(RgbLnPayArgs),
}

#[derive(Subcommand, Debug)]
pub enum RgbLnInvoiceCommand {
	/// Create a BOLT11 invoice embedding RGB fields.
	Create(RgbLnInvoiceCreateArgs),
	/// Create a BOLT11 invoice embedding RGB fields for a specific payment hash.
	CreateForHash(RgbLnInvoiceCreateForHashArgs),
	/// Decode a BOLT11 invoice and show RGB fields if present.
	Decode { invoice: String },
}

#[derive(Args, Debug)]
pub struct RgbLnInvoiceCreateArgs {
	#[arg(long)]
	pub asset_id: String,
	#[arg(long)]
	pub asset_amount: u64,
	#[arg(long)]
	pub desc: String,
	#[arg(long, default_value_t = 3600)]
	pub expiry_secs: u32,
	#[arg(long)]
	pub btc_carrier_amount_msat: u64,
}

#[derive(Args, Debug)]
pub struct RgbLnInvoiceCreateForHashArgs {
	#[arg(long)]
	pub asset_id: String,
	#[arg(long)]
	pub asset_amount: u64,
	/// Payment hash (64 hex chars) provided by caller.
	#[arg(long)]
	pub payment_hash: String,
	#[arg(long)]
	pub desc: String,
	#[arg(long, default_value_t = 3600)]
	pub expiry_secs: u32,
	#[arg(long)]
	pub btc_carrier_amount_msat: u64,
}

#[derive(Args, Debug)]
pub struct RgbLnPayArgs {
	#[arg(long)]
	pub invoice: String,
	#[arg(long)]
	pub asset_id: Option<String>,
	#[arg(long)]
	pub asset_amount: Option<u64>,
}

#[derive(Subcommand, Debug)]
pub enum RgbOnchainCommand {
	/// Create an RGB on-chain invoice.
	InvoiceCreate(RgbOnchainInvoiceCreateArgs),
	/// Send an RGB on-chain payment.
	Send(RgbOnchainSendArgs),
	/// Receive/finalize an RGB on-chain payment.
	Receive(RgbOnchainReceiveArgs),
}

#[derive(Args, Debug)]
pub struct RgbOnchainReceiveArgs {
	/// Consignment key in the daemon cache (works when both parties share the same cache).
	#[arg(required_unless_present = "file", conflicts_with = "file")]
	pub consignment_key: Option<String>,
	/// Upload a consignment archive (raw/gzip/zip) in the request body (cross-host setups).
	#[arg(long, required_unless_present = "consignment_key")]
	pub file: Option<String>,
	/// Archive format for `--file`: `auto` (default), `raw`, `gzip`, `zip`.
	#[arg(long, default_value = "auto")]
	pub format: String,
}

#[derive(Args, Debug)]
pub struct RgbOnchainInvoiceCreateArgs {
	#[arg(long)]
	pub contract_id: String,
	#[arg(long)]
	pub amount: u64,
	#[arg(long)]
	pub use_witness_utxo: bool,
	#[arg(long)]
	pub nonce: Option<u64>,
	/// Optional outpoint used for blinding when `--use-witness-utxo=false` (`txid:vout`).
	///
	/// If omitted, the node will auto-select a wallet UTXO and return it in the response.
	#[arg(long)]
	pub blinding_utxo: Option<String>,
}

#[derive(Args, Debug)]
pub struct RgbOnchainSendArgs {
	#[arg(long)]
	pub invoice: String,
	#[arg(long)]
	pub sats_for_fee_and_outputs: Option<u64>,
	#[arg(long)]
	pub fee_rate_sats_per_vb: f32,
}

#[derive(Subcommand, Debug)]
pub enum InvoiceCommand {
	/// Create an invoice (fixed amount if `--amount-msat` is set, variable otherwise).
	Create(InvoiceCreateArgs),
	/// Create a fixed-amount invoice for a specific payment hash (hold invoice).
	CreateForHash(InvoiceCreateForHashArgs),
	/// Decode a BOLT11 invoice.
	Decode { invoice: String },
	/// Fail a held payment created via `create-for-hash`.
	FailForHash { payment_hash: String },
	/// Claim a held payment created via `create-for-hash`.
	ClaimForHash(InvoiceClaimForHashArgs),
	/// Pay an invoice (uses `--amount-msat` for variable-amount invoices).
	Pay(InvoicePayArgs),
	/// Send (pay) a BOLT11 invoice.
	Send { invoice: String },
	/// Send (pay) a variable-amount BOLT11 invoice with an explicit amount.
	SendUsingAmount(InvoiceSendUsingAmountArgs),
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
pub struct InvoiceCreateForHashArgs {
	#[arg(long)]
	pub desc: String,
	#[arg(long)]
	pub amount_msat: u64,
	/// Payment hash (hex-encoded 32 bytes).
	#[arg(long)]
	pub payment_hash: String,
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

#[derive(Args, Debug)]
pub struct InvoiceSendUsingAmountArgs {
	#[arg(long)]
	pub invoice: String,
	#[arg(long)]
	pub amount_msat: u64,
}

#[derive(Args, Debug)]
pub struct InvoiceClaimForHashArgs {
	/// Payment hash (hex-encoded 32 bytes).
	#[arg(long)]
	pub payment_hash: String,
	/// Payment preimage (hex-encoded 32 bytes).
	#[arg(long)]
	pub preimage: String,
	/// Claimable amount in millisatoshis.
	#[arg(long)]
	pub claimable_amount_msat: u64,
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
