use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use argon2::{Algorithm, Argon2, Params, Version};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use bip39::Mnemonic;
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{KeyInit, XChaCha20Poly1305, XNonce};
use rand::RngCore;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::cli::{Cli, KeystoreInitArgs, KeystoreInitMode, KeystoreMigrateArgs};

const KEYSTORE_AAD: &[u8] = b"rgbldk-keystore:v1";
const SEED_LEN: usize = 64;

#[derive(Debug, Clone)]
pub struct KeystorePaths {
	pub keystore: PathBuf,
	pub legacy_seed: PathBuf,
}

pub fn default_paths(cli: &Cli) -> KeystorePaths {
	let base = cli.data_dir.as_deref().unwrap_or("/tmp/ldk_node");
	KeystorePaths {
		keystore: PathBuf::from(base).join("keystore"),
		legacy_seed: PathBuf::from(base).join("keys_seed"),
	}
}

pub struct PreparedKeystoreInit {
	pub keystore_path: PathBuf,
	pub seed: [u8; SEED_LEN],
	pub passphrase: SecretString,
	pub mnemonic: Option<String>,
}

#[derive(Debug, Clone)]
pub enum KeystoreError {
	Io(String),
	Invalid(String),
	DecryptFailed,
}

impl std::fmt::Display for KeystoreError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Io(e) => write!(f, "keystore io error: {e}"),
			Self::Invalid(e) => write!(f, "keystore invalid: {e}"),
			Self::DecryptFailed => write!(f, "keystore decrypt failed"),
		}
	}
}

impl std::error::Error for KeystoreError {}

impl From<io::Error> for KeystoreError {
	fn from(e: io::Error) -> Self {
		Self::Io(e.to_string())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeystoreFile {
	version: u32,
	kdf: KeystoreKdf,
	aead: KeystoreAead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeystoreKdf {
	alg: String,
	salt_b64: String,
	params: KeystoreKdfParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeystoreKdfParams {
	m_cost_kib: u32,
	t_cost: u32,
	p_cost: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeystoreAead {
	alg: String,
	nonce_b64: String,
	ciphertext_b64: String,
	aad_b64: String,
}

fn argon2_default_params() -> KeystoreKdfParams {
	KeystoreKdfParams { m_cost_kib: 64 * 1024, t_cost: 2, p_cost: 1 }
}

fn read_passphrase(passphrase_stdin: bool) -> Result<SecretString, KeystoreError> {
	let s = if passphrase_stdin {
		let mut buf = String::new();
		io::stdin()
			.read_to_string(&mut buf)
			.map_err(|e| KeystoreError::Io(format!("read stdin: {e}")))?;
		buf.trim_end().to_string()
	} else {
		rpassword::prompt_password("Keystore passphrase: ")
			.map_err(|e| KeystoreError::Io(format!("read passphrase: {e}")))? // no echo
	};
	let pass = SecretString::new(s.into());
	if pass.expose_secret().trim().is_empty() {
		return Err(KeystoreError::Invalid("passphrase must not be empty".to_string()));
	}
	Ok(pass)
}

fn derive_key(
	passphrase: &SecretString, salt: &[u8], params: &KeystoreKdfParams,
) -> Result<[u8; 32], KeystoreError> {
	let p = Params::new(params.m_cost_kib, params.t_cost, params.p_cost, Some(32))
		.map_err(|e| KeystoreError::Invalid(format!("invalid argon2 params: {e}")))?;
	let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, p);
	let mut out = [0u8; 32];
	argon2
		.hash_password_into(passphrase.expose_secret().as_bytes(), salt, &mut out)
		.map_err(|e| KeystoreError::Invalid(format!("argon2 failed: {e}")))?;
	Ok(out)
}

fn write_keystore(
	path: &Path, seed: &[u8; SEED_LEN], passphrase: &SecretString,
) -> Result<(), KeystoreError> {
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?;
	}

	let mut salt = [0u8; 16];
	rand::thread_rng().fill_bytes(&mut salt);
	let kdf_params = argon2_default_params();
	let mut key = derive_key(passphrase, &salt, &kdf_params)?;

	let cipher = XChaCha20Poly1305::new_from_slice(&key)
		.map_err(|_| KeystoreError::Invalid("invalid aead key length".to_string()))?;
	let mut nonce_bytes = [0u8; 24];
	rand::thread_rng().fill_bytes(&mut nonce_bytes);
	let nonce = XNonce::from_slice(&nonce_bytes);

	let ciphertext = cipher
		.encrypt(nonce, Payload { msg: seed.as_slice(), aad: KEYSTORE_AAD })
		.map_err(|_| KeystoreError::DecryptFailed)?;

	key.zeroize();

	let file = KeystoreFile {
		version: 1,
		kdf: KeystoreKdf {
			alg: "argon2id".to_string(),
			salt_b64: B64.encode(salt),
			params: kdf_params,
		},
		aead: KeystoreAead {
			alg: "xchacha20poly1305".to_string(),
			nonce_b64: B64.encode(nonce_bytes),
			ciphertext_b64: B64.encode(ciphertext),
			aad_b64: B64.encode(KEYSTORE_AAD),
		},
	};

	let json = serde_json::to_vec_pretty(&file)
		.map_err(|e| KeystoreError::Invalid(format!("serialize keystore: {e}")))?;
	fs::write(path, json)?;

	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		let mut perms = fs::metadata(path)?.permissions();
		perms.set_mode(0o600);
		fs::set_permissions(path, perms)?;
	}

	Ok(())
}

fn read_legacy_seed_file(path: &Path) -> Result<[u8; SEED_LEN], KeystoreError> {
	let bytes = fs::read(path)?;
	if bytes.len() != SEED_LEN {
		return Err(KeystoreError::Invalid(format!(
			"legacy seed file invalid length: {}",
			bytes.len()
		)));
	}
	let mut seed = [0u8; SEED_LEN];
	seed.copy_from_slice(&bytes);
	Ok(seed)
}

fn backup_legacy_seed_file(path: &Path) -> Result<PathBuf, KeystoreError> {
	let ts = chrono::Utc::now().format("%Y%m%d-%H%M%S").to_string();
	let name = path
		.file_name()
		.and_then(|n| n.to_str())
		.ok_or_else(|| KeystoreError::Io("invalid legacy seed filename".to_string()))?;
	let backup = path.with_file_name(format!("{name}.bak.{ts}"));
	fs::rename(path, &backup)?;
	Ok(backup)
}

fn seed_from_hex(hex_str: &str) -> Result<[u8; SEED_LEN], KeystoreError> {
	let bytes = hex::decode(hex_str.trim())
		.map_err(|e| KeystoreError::Invalid(format!("seed_hex invalid hex: {e}")))?;
	if bytes.len() != SEED_LEN {
		return Err(KeystoreError::Invalid(format!(
			"seed_hex must be {SEED_LEN} bytes, got {}",
			bytes.len()
		)));
	}
	let mut seed = [0u8; SEED_LEN];
	seed.copy_from_slice(&bytes);
	Ok(seed)
}

pub fn commit_keystore_init(prepared: &PreparedKeystoreInit) -> Result<(), KeystoreError> {
	write_keystore(&prepared.keystore_path, &prepared.seed, &prepared.passphrase)
}

pub fn prepare_keystore_init(
	cli: &Cli, args: &KeystoreInitArgs,
) -> Result<PreparedKeystoreInit, KeystoreError> {
	let paths = default_paths(cli);
	let passphrase = read_passphrase(args.passphrase_stdin)?;

	if paths.keystore.exists() {
		return Err(KeystoreError::Invalid(format!(
			"keystore already exists at {}",
			paths.keystore.display()
		)));
	}

	let (seed, mnemonic_out) = match args.mode {
		KeystoreInitMode::GenerateMnemonic => {
			let mut entropy = [0u8; 32];
			rand::thread_rng().fill_bytes(&mut entropy);
			let mnemonic = Mnemonic::from_entropy(&entropy)
				.map_err(|e| KeystoreError::Invalid(format!("generate mnemonic failed: {e}")))?;
			let seed = mnemonic.to_seed("");
			(seed, Some(mnemonic.to_string()))
		},
		KeystoreInitMode::ImportMnemonic => {
			let m = args
				.mnemonic
				.as_deref()
				.ok_or_else(|| KeystoreError::Invalid("missing --mnemonic".to_string()))?;
			let mnemonic = Mnemonic::parse_normalized(m)
				.map_err(|e| KeystoreError::Invalid(format!("invalid mnemonic: {e}")))?;
			(mnemonic.to_seed(""), None)
		},
		KeystoreInitMode::ImportSeed => {
			let h = args
				.seed_hex
				.as_deref()
				.ok_or_else(|| KeystoreError::Invalid("missing --seed-hex".to_string()))?;
			(seed_from_hex(h)?, None)
		},
	};

	Ok(PreparedKeystoreInit {
		keystore_path: paths.keystore,
		seed,
		passphrase,
		mnemonic: mnemonic_out,
	})
}

pub fn keystore_migrate(
	cli: &Cli, args: &KeystoreMigrateArgs,
) -> Result<(PathBuf, PathBuf), KeystoreError> {
	let paths = default_paths(cli);
	let legacy_path = args
		.legacy_seed_path
		.as_deref()
		.map(PathBuf::from)
		.unwrap_or_else(|| paths.legacy_seed.clone());
	let passphrase = read_passphrase(args.passphrase_stdin)?;

	if paths.keystore.exists() {
		return Err(KeystoreError::Invalid(format!(
			"keystore already exists at {}",
			paths.keystore.display()
		)));
	}
	if !legacy_path.exists() {
		return Err(KeystoreError::Invalid(format!(
			"legacy seed file not found at {}",
			legacy_path.display()
		)));
	}

	let seed = read_legacy_seed_file(&legacy_path)?;
	write_keystore(&paths.keystore, &seed, &passphrase)?;
	let backup = backup_legacy_seed_file(&legacy_path)?;
	Ok((paths.keystore, backup))
}
