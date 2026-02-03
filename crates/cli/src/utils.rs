use std::io::IsTerminal;
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
