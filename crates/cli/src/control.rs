use std::path::{Path, PathBuf};

use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub fn default_control_socket_path(
	control_socket: Option<&str>, data_dir: Option<&str>,
) -> PathBuf {
	if let Some(p) = control_socket {
		return PathBuf::from(p);
	}
	let base = data_dir.unwrap_or("/tmp/ldk_node");
	PathBuf::from(base).join("control.sock")
}

pub async fn call_control(
	socket_path: &Path, method: &str, params: serde_json::Value,
) -> Result<serde_json::Value, String> {
	let stream = UnixStream::connect(socket_path).await.map_err(|e| {
		format!("failed to connect to control socket {}: {e}", socket_path.display())
	})?;
	let (read_half, mut write_half) = stream.into_split();
	let mut reader = BufReader::new(read_half);

	let req = json!({
		"id": "1",
		"method": method,
		"params": params,
	});
	write_half
		.write_all(format!("{req}\n").as_bytes())
		.await
		.map_err(|e| format!("failed to write control request: {e}"))?;

	let mut line = String::new();
	reader
		.read_line(&mut line)
		.await
		.map_err(|e| format!("failed to read control response: {e}"))?;
	let resp: serde_json::Value = serde_json::from_str(line.trim_end())
		.map_err(|e| format!("invalid control response: {e}"))?;
	Ok(resp)
}
