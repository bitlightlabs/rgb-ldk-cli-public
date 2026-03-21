use std::io::IsTerminal;
use std::io::Read;
use std::io::Write;
use std::time::Duration;

pub fn print_json<T: serde::Serialize>(value: &T, pretty: bool) {
	if pretty {
		println!("{}", serde_json::to_string_pretty(value).unwrap());
	} else {
		println!("{}", serde_json::to_string(value).unwrap());
	}
}

pub fn die(msg: impl AsRef<str>) -> ! {
	eprintln!("{}", msg.as_ref());
	std::process::exit(1);
}

pub fn confirm_or_exit(yes: bool, prompt: &str) {
	if yes {
		return;
	}
	if !std::io::stdin().is_terminal() {
		die(format!("{prompt}\nRefusing to proceed non-interactively without --yes."));
	}
	eprintln!("{prompt} Type 'yes' to continue:");
	let mut line = String::new();
	let _ = std::io::stdin().read_line(&mut line);
	if line.trim() != "yes" {
		die("aborted");
	}
}

pub async fn with_spinner<T>(
	enabled: bool, msg: &str, fut: impl std::future::Future<Output = T>,
) -> T {
	if !enabled || !std::io::stderr().is_terminal() {
		return fut.await;
	}

	let msg = msg.to_string();
	let handle = tokio::spawn(async move {
		let frames = ["|", "/", "-", "\\"];
		let mut i = 0usize;
		let mut tick = tokio::time::interval(Duration::from_millis(120));
		loop {
			tick.tick().await;
			eprint!("\r{msg} {}", frames[i % frames.len()]);
			let _ = std::io::stderr().flush();
			i += 1;
		}
	});

	let out = fut.await;
	handle.abort();
	eprint!("\r\x1b[2K");
	let _ = std::io::stderr().flush();
	out
}

pub fn read_passphrase(passphrase_stdin: bool) -> Result<String, String> {
	if passphrase_stdin {
		let mut buf = String::new();
		std::io::stdin()
			.read_to_string(&mut buf)
			.map_err(|e| format!("failed to read stdin: {e}"))?;
		Ok(buf.trim_end().to_string())
	} else {
		rpassword::prompt_password("Passphrase: ")
			.map_err(|e| format!("failed to read passphrase: {e}"))
	}
}

pub fn read_stdin_trimmed(label: &str) -> Result<String, String> {
	let mut buf = String::new();
	std::io::stdin()
		.read_to_string(&mut buf)
		.map_err(|e| format!("failed to read stdin for {label}: {e}"))?;
	let s = buf.trim().to_string();
	if s.is_empty() {
		return Err(format!("{label} must not be empty"));
	}
	Ok(s)
}

pub fn read_file_trimmed(path: &str) -> Result<String, String> {
	let s = std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
	let v = s.trim().to_string();
	if v.is_empty() {
		return Err(format!("{path} is empty"));
	}
	Ok(v)
}
