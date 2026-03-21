fn summarize_error_bytes(status: reqwest::StatusCode, bytes: &[u8]) -> String {
	if let Ok(v) = serde_json::from_slice::<serde_json::Value>(bytes) {
		let error = v.get("error").and_then(|e| e.as_str()).unwrap_or("request failed");
		let message = v.get("message").and_then(|m| m.as_str());
		let hint = v.get("hint").and_then(|h| h.as_str());
		let detail = v.get("detail");
		let checks = v.get("checks");

		let mut parts = vec![format!("HTTP {}: {}", status.as_u16(), error)];
		if let Some(message) = message.filter(|m| *m != error) {
			parts.push(format!("message={message}"));
		}
		if let Some(hint) = hint {
			parts.push(format!("hint={hint}"));
		}
		if let Some(detail) = detail {
			parts.push(format!("detail={}", detail));
		}
		if let Some(checks) = checks {
			parts.push(format!("checks={}", checks));
		}
		return parts.join("\n");
	}

	format!(
		"HTTP {}: {}",
		status.as_u16(),
		String::from_utf8_lossy(bytes).trim()
	)
}

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
		Err(summarize_error_bytes(status, &bytes))
	}
}

pub async fn send_value(
	rb: reqwest::RequestBuilder,
) -> Result<(reqwest::StatusCode, serde_json::Value), String> {
	let resp = rb.send().await.map_err(|e| e.to_string())?;
	let status = resp.status();
	let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
	let value = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap_or_else(
		|_| serde_json::json!({ "raw": String::from_utf8_lossy(&bytes).to_string() }),
	);
	Ok((status, value))
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
		Err(summarize_error_bytes(status, &bytes))
	}
}
