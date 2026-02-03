pub fn join_url(base: &str, path: &str) -> String {
	format!("{}/{}", base.trim_end_matches('/'), path.trim_start_matches('/'))
}

pub async fn send_json<T: serde::de::DeserializeOwned>(
	rb: reqwest::RequestBuilder,
) -> Result<T, String> {
	let resp = rb.send().await.map_err(|e| e.to_string())?;
	let status = resp.status();
	let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
	if status.is_success() {
		serde_json::from_slice(&bytes).map_err(|e| e.to_string())
	} else {
		let err = serde_json::from_slice::<serde_json::Value>(&bytes)
			.ok()
			.and_then(|v| v.get("error").and_then(|e| e.as_str()).map(|s| s.to_string()))
			.unwrap_or_else(|| String::from_utf8_lossy(&bytes).to_string());
		Err(format!("HTTP {}: {}", status.as_u16(), err))
	}
}

pub async fn send_json_allow_status<T: serde::de::DeserializeOwned>(
	rb: reqwest::RequestBuilder, allowed: &[reqwest::StatusCode],
) -> Result<T, String> {
	let resp = rb.send().await.map_err(|e| e.to_string())?;
	let status = resp.status();
	let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
	if status.is_success() || allowed.contains(&status) {
		serde_json::from_slice(&bytes).map_err(|e| e.to_string())
	} else {
		let err = serde_json::from_slice::<serde_json::Value>(&bytes)
			.ok()
			.and_then(|v| v.get("error").and_then(|e| e.as_str()).map(|s| s.to_string()))
			.unwrap_or_else(|| String::from_utf8_lossy(&bytes).to_string());
		Err(format!("HTTP {}: {}", status.as_u16(), err))
	}
}
