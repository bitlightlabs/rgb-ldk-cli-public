//! HTTP API request/response types.

use serde::{Deserialize, Serialize};

/// Node status response.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusDto {
	/// Whether the node runtime is running.
	pub is_running: bool,
	/// Whether the node is currently listening for inbound peers.
	pub is_listening: bool,
	/// Current best block height as seen by the node.
	pub best_block_height: u32,
}

/// Wallet and channel balance overview.
#[derive(Debug, Serialize, Deserialize)]
pub struct BalancesDto {
	/// Total confirmed on-chain balance in satoshis.
	pub total_onchain_balance_sats: u64,
	/// Spendable on-chain balance in satoshis.
	pub spendable_onchain_balance_sats: u64,
	/// Sum of sats reserved for anchor channels.
	pub total_anchor_channels_reserve_sats: u64,
	/// Total claimable Lightning balance in satoshis.
	pub total_lightning_balance_sats: u64,
}

/// Peer details entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDetailsDto {
	/// Peer node id in hex.
	pub node_id: String,
	/// Selected address for the peer.
	pub address: String,
	/// Whether peer is persisted to peer store.
	pub is_persisted: bool,
	/// Whether peer is currently connected.
	pub is_connected: bool,
}

/// Request to connect to a peer.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerConnectRequest {
	/// Peer node id in hex.
	pub node_id: String,
	/// Socket address, e.g. `lnd1:9735`.
	pub address: String,
	/// Whether to persist the peer to disk.
	#[serde(default)]
	pub persist: bool,
}

/// Request to disconnect a peer.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDisconnectRequest {
	/// Peer node id in hex.
	pub node_id: String,
}

/// Payment details entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetailsDto {
	/// Payment id (hex-encoded 32 bytes).
	pub id: String,
	/// Direction, either Inbound or Outbound.
	pub direction: String,
	/// Status, one of Pending, Succeeded, Failed.
	pub status: String,
	/// Amount in millisatoshis, if known.
	pub amount_msat: Option<u64>,
	/// Payment kind label.
	pub kind: String,
	/// Fee paid in millisatoshis, if known.
	pub fee_paid_msat: Option<u64>,
}

/// Channel details entry (extended for control-plane integrations).
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelDetailsExtendedDto {
	/// Channel id (32-byte hex).
	pub channel_id: String,
	/// User channel id (hex-encoded 16 bytes BIG-ENDIAN).
	pub user_channel_id: String,
	/// Counterparty node id (hex pubkey).
	pub counterparty_node_id: String,
	/// Funding outpoint formatted as `txid:vout` if known.
	pub channel_point: Option<String>,
	/// Total channel capacity in satoshis.
	pub channel_value_sats: u64,
	/// Sendable capacity in millisatoshis.
	pub outbound_capacity_msat: u64,
	/// Receivable capacity in millisatoshis.
	pub inbound_capacity_msat: u64,
	/// Whether channel is ready.
	pub is_channel_ready: bool,
	/// Whether channel is usable.
	pub is_usable: bool,
	/// Whether channel is announced.
	pub is_announced: bool,
	/// RGB asset balance in this channel, if any.
	/// Only present for RGB-enabled channels.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rgb_balance: Option<RgbChannelBalanceDto>,
}

/// RGB asset balance in a channel.
#[derive(Debug, Serialize, Deserialize)]
pub struct RgbChannelBalanceDto {
	/// Asset ID (hex-encoded 32 bytes).
	pub asset_id: String,
	/// Local (our) RGB balance in this channel.
	pub local_amount: u64,
	/// Remote (counterparty) RGB balance in this channel.
	pub remote_amount: u64,
}

/// Channel details entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelDetailsDto {
	/// Channel id (32-byte hex).
	pub channel_id: String,
	/// Counterparty node id.
	pub counterparty_node_id: String,
	/// Whether channel is ready.
	pub is_channel_ready: bool,
	/// Whether channel is announced.
	pub is_announced: bool,
}

/// Request to decode a Bolt11 invoice.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bolt11DecodeRequest {
	/// The Bolt11 invoice string.
	pub invoice: String,
}

/// Decoded Bolt11 invoice summary.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bolt11DecodeResponse {
	/// Payment hash (hex).
	pub payment_hash: String,
	/// Destination node id (hex pubkey).
	pub destination: String,
	/// Amount in millisatoshis, if the invoice is fixed-amount.
	pub amount_msat: Option<u64>,
	/// Invoice expiry in seconds.
	pub expiry_secs: u64,
}

/// Request to pay a Bolt11 invoice and wait for completion.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bolt11PayRequest {
	/// The Bolt11 invoice string.
	pub invoice: String,
	/// Amount in millisatoshis (required for variable-amount invoices).
	pub amount_msat: Option<u64>,
}

/// Response of a completed Bolt11 payment.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bolt11PayResponse {
	/// Payment id (hex-encoded 32 bytes).
	pub payment_id: String,
	/// Payment preimage (hex-encoded 32 bytes).
	pub preimage: String,
	/// Amount paid in satoshis.
	pub amount_sats: u64,
	/// Destination node id (hex pubkey).
	pub destination: String,
	/// Fee paid in millisatoshis, if known.
	pub fee_paid_msat: Option<u64>,
}

/// Request to open a channel to a peer.
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenChannelRequest {
	/// Counterparty node id (hex secp256k1 pubkey).
	pub node_id: String,
	/// Socket address of the counterparty as advertised.
	pub address: String,
	/// Channel capacity in satoshis.
	pub channel_amount_sats: u64,
	/// Optional initial push to counterparty in msat.
	pub push_to_counterparty_msat: Option<u64>,
	/// Whether to create an announced channel (default true).
	pub announce: Option<bool>,
}

/// Response of a successful channel open request.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenChannelResponse {
	/// User channel id (hex-encoded 16 bytes BIG-ENDIAN).
	pub user_channel_id: String,
}

/// Request to close or force-close a channel.
#[derive(Debug, Deserialize, Serialize)]
pub struct CloseChannelRequest {
	/// User channel id (hex-encoded 16 bytes BIG-ENDIAN).
	pub user_channel_id: String,
	/// Counterparty node id.
	pub counterparty_node_id: String,
}

/// Request to create a Bolt11 invoice.
#[derive(Debug, Deserialize, Serialize)]
pub struct Bolt11ReceiveRequest {
	/// Amount in millisatoshis.
	pub amount_msat: u64,
	/// Invoice description.
	pub description: String,
	/// Invoice expiry in seconds.
	pub expiry_secs: u32,
}

/// Response containing a newly created Bolt11 invoice.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bolt11ReceiveResponse {
	/// The invoice string.
	pub invoice: String,
}

/// Request to create a variable-amount Bolt11 invoice.
#[derive(Debug, Deserialize, Serialize)]
pub struct Bolt11ReceiveVarRequest {
	/// Invoice description.
	pub description: String,
	/// Invoice expiry in seconds.
	pub expiry_secs: u32,
}

/// Request to pay a Bolt11 invoice.
#[derive(Debug, Deserialize, Serialize)]
pub struct Bolt11SendRequest {
	/// The Bolt11 invoice string.
	pub invoice: String,
}

/// Request to pay a Bolt11 invoice using a specified amount.
#[derive(Debug, Deserialize, Serialize)]
pub struct Bolt11SendUsingAmountRequest {
	/// The Bolt11 invoice string.
	pub invoice: String,
	/// Amount in millisatoshis to pay.
	pub amount_msat: u64,
}

/// Response containing the id of a sent payment.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendResponse {
	/// Payment id (hex-encoded 32 bytes).
	pub payment_id: String,
}

/// Request to send a spontaneous (keysend) payment.
#[derive(Debug, Deserialize, Serialize)]
pub struct SpontaneousSendRequest {
	/// Counterparty node id (hex pubkey).
	pub counterparty_node_id: String,
	/// Amount in millisatoshis.
	pub amount_msat: u64,
	/// Optional custom TLV list.
	#[serde(default)]
	pub custom_tlvs: Vec<CustomTlvDto>,
}

/// Custom TLV record as passed to keysend.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTlvDto {
	/// TLV type number.
	pub r#type: u64,
	/// Hex-encoded value bytes.
	pub value_hex: String,
}

/// Generic `{ ok: true }` response.
#[derive(Debug, Serialize, Deserialize)]
pub struct OkResponse {
	/// Whether the operation succeeded.
	pub ok: bool,
	/// Optional list of sub-checks providing more details about how `ok` was determined.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub checks: Vec<HealthCheckDto>,
}

/// A single health/readiness sub-check.
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckDto {
	/// Machine-readable check name.
	pub name: String,
	/// Whether the check passed.
	pub ok: bool,
	/// Optional short detail.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub detail: Option<String>,
	/// Optional user-actionable hint.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub hint: Option<String>,
}

/// Generic `{ error: "..." }` error response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
	/// Human-readable error message.
	pub error: String,
}

/// Outpoint representation.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutPointDto {
	/// Transaction id (hex string).
	pub txid: String,
	/// Output index.
	pub vout: u32,
}

/// Event returned by `/events/wait_next`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventDto {
	/// A sent payment succeeded.
	PaymentSuccessful {
		/// Optional payment id.
		payment_id: Option<String>,
		/// Total fee paid in millisatoshis, if provided.
		fee_paid_msat: Option<u64>,
	},
	/// A sent payment failed.
	PaymentFailed {
		/// Optional payment id.
		payment_id: Option<String>,
	},
	/// A payment has been received.
	PaymentReceived {
		/// Optional payment id.
		payment_id: Option<String>,
		/// Amount received in millisatoshis.
		amount_msat: u64,
	},
	/// A channel has been created and is pending on-chain confirmation.
	ChannelPending {
		/// Funding outpoint of the channel transaction.
		funding_txo: OutPointDto,
	},
	/// A channel is ready to be used.
	ChannelReady {
		/// User channel id (hex-encoded 16 bytes BIG-ENDIAN).
		user_channel_id: String,
	},
	/// A channel has been closed.
	ChannelClosed {
		/// Channel id (hex-encoded 32 bytes).
		channel_id: String,
		/// User channel id (hex-encoded 16 bytes BIG-ENDIAN).
		user_channel_id: String,
		/// Counterparty node id (hex pubkey), if known.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		counterparty_node_id: Option<String>,
		/// Closure reason (Debug string), if provided by the node.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		reason: Option<String>,
	},
	/// Other event kinds (not explicitly modeled).
	Other {
		/// Name of the event kind.
		kind: String,
	},
}
