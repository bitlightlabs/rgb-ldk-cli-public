#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import http.client
import http.server
import json
import os
import socket
import socketserver
import subprocess
import sys
import tempfile
import threading
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Optional


REPO_ROOT = Path(__file__).resolve().parents[3]
DEFAULT_OUT = REPO_ROOT / "crates/cli/EXAMPLE.md"
DEFAULT_COMPOSE_FILE = REPO_ROOT / "crates/cli/docker-compose.yaml"
DEFAULT_RGB_LDK_NODE_DIR = (REPO_ROOT / "../rgb-ldk-node").resolve()

DEFAULT_STEP_TIMEOUT_S = 60.0
DEFAULT_MAX_RUNTIME_SECS = 15 * 60.0


def _progress(msg: str) -> None:
    print(f"[gen_example_md] {msg}", file=sys.stderr)


def _terminate_process(proc: subprocess.Popen[str], *, timeout_s: float) -> None:
    if proc.poll() is not None:
        return
    proc.terminate()
    start = time.monotonic()
    while proc.poll() is None and (time.monotonic() - start) < timeout_s:
        time.sleep(0.05)
    if proc.poll() is None:
        proc.kill()
        while proc.poll() is None and (time.monotonic() - start) < (timeout_s + 5.0):
            time.sleep(0.05)


def sh(cmd: str) -> list[str]:
    return ["bash", "-lc", cmd]


def first_nonempty_line(s: str) -> str:
    for line in s.splitlines():
        line = line.strip()
        if line:
            return line
    return ""


def normalize_rgbldk_cmd(cmd: str) -> str:
    cmd = cmd.strip()
    if not cmd.startswith("rgbldk "):
        return cmd
    # Default output is auto=text on TTY, json otherwise. Since this generator captures output,
    # force stable text output and disable colors.
    if "--output" not in cmd:
        cmd = cmd.replace("rgbldk ", "rgbldk --output text ", 1)
    if "--color" not in cmd:
        cmd = cmd.replace("rgbldk ", "rgbldk --color never ", 1)
    return cmd


def normalize_rgbldk_json_cmd(cmd: str) -> str:
    cmd = cmd.strip()
    if not cmd.startswith("rgbldk "):
        return cmd
    # Stable JSON output for scripting and disable colors (we parse stdout).
    if "--output" not in cmd:
        cmd = cmd.replace("rgbldk ", "rgbldk --output json --pretty ", 1)
    if "--color" not in cmd:
        cmd = cmd.replace("rgbldk ", "rgbldk --color never ", 1)
    return cmd


class _ThreadingHTTPServer(socketserver.ThreadingMixIn, http.server.HTTPServer):
    daemon_threads = True


def start_http_mux_proxy(*, bitcoind: str, esplora: str) -> tuple[_ThreadingHTTPServer, threading.Thread, int]:
    """
    Start a local HTTP proxy that routes:
    - `/rest/*` to `bitcoind`
    - everything else to `esplora`

    This is used as a workaround when the daemon requires `--bitcoind-rest` (for RGB),
    while the RGB wallet expects an Esplora-compatible indexer URL.
    """

    def parse_hostport(s: str) -> tuple[str, int]:
        host, port_str = s.rsplit(":", 1)
        return host, int(port_str)

    bitcoind_host, bitcoind_port = parse_hostport(bitcoind)
    esplora_host, esplora_port = parse_hostport(esplora)

    class Handler(http.server.BaseHTTPRequestHandler):
        def log_message(self, _fmt: str, *_args: object) -> None:
            # Keep generator output stable (and quiet); proxy is an internal implementation detail.
            return

        def _proxy(self) -> None:
            use_bitcoind = self.path.startswith("/rest/")
            up_host, up_port = (bitcoind_host, bitcoind_port) if use_bitcoind else (esplora_host, esplora_port)

            content_len = int(self.headers.get("Content-Length", "0") or "0")
            body = self.rfile.read(content_len) if content_len > 0 else None

            headers: dict[str, str] = {}
            for k, v in self.headers.items():
                lk = k.lower()
                if lk in ("host", "content-length", "connection", "proxy-connection"):
                    continue
                headers[k] = v

            conn = http.client.HTTPConnection(up_host, up_port, timeout=30)
            try:
                conn.request(self.command, self.path, body=body, headers=headers)
                resp = conn.getresponse()
                data = resp.read()
            finally:
                conn.close()

            self.send_response(resp.status)
            for k, v in resp.getheaders():
                lk = k.lower()
                if lk in ("transfer-encoding", "connection", "content-length"):
                    continue
                self.send_header(k, v)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            if self.command != "HEAD":
                self.wfile.write(data)

        do_GET = _proxy
        do_POST = _proxy
        do_PUT = _proxy
        do_DELETE = _proxy
        do_HEAD = _proxy
        do_OPTIONS = _proxy

    httpd: _ThreadingHTTPServer = _ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()
    return httpd, thread, int(httpd.server_address[1])


def indent_quote_block(lines: list[str]) -> list[str]:
    out: list[str] = []
    for line in lines:
        out.append("> " + line if line else ">")
    return out


@dataclass
class RunResult:
    stdout: str
    stderr: str
    returncode: int


class Markdown:
    def __init__(self) -> None:
        self.lines: list[str] = []

    def write(self, s: str = "") -> None:
        self.lines.append(s)

    def heading(self, level: int, title: str) -> None:
        self.write("#" * level + " " + title)
        self.write()

    def paragraph(self, text: str) -> None:
        self.write(text)
        self.write()

    def fence(self, lang: str, content: str) -> None:
        self.write(f"```{lang}")
        # Keep a blank line before closing fence (matches existing EXAMPLE.md style)
        if not content.endswith("\n"):
            content += "\n"
        self.write(content.rstrip("\n"))
        self.write()
        self.write("```")
        self.write()

    def quoted_text_fence(self, content: str) -> None:
        # content is plain text. Wrap it in a quoted ```text``` block.
        self.write("**Result:**")
        self.write()
        block_lines: list[str] = []
        block_lines.append("```text")
        for line in content.rstrip("\n").splitlines():
            block_lines.append(line)
        block_lines.append("")  # blank line inside the fence
        block_lines.append("```")
        self.lines.extend(indent_quote_block(block_lines))
        self.write()

    def run_block(self, display_cmd: str) -> None:
        self.write("**Run:**")
        self.write()
        self.fence("bash", f"$ {display_cmd}\n")

    def command_only(self, display_cmd: str) -> None:
        self.fence("bash", f"$ {display_cmd}\n")

    def render(self) -> str:
        return "\n".join(self.lines).rstrip() + "\n"


class Runner:
    def __init__(self, cwd: Path, env: dict[str, str], verbose: bool) -> None:
        self.cwd = cwd
        self.env = env
        self.verbose = verbose

    def run(
        self,
        cmd: str,
        *,
        retries: int = 0,
        retry_sleep_s: float = 1.0,
        check: bool = True,
        timeout_s: Optional[float] = None,
    ) -> RunResult:
        for attempt in range(retries + 1):
            if self.verbose:
                print(f"+ {cmd}", file=sys.stderr)

            proc = subprocess.Popen(
                sh(cmd),
                cwd=self.cwd,
                env=self.env,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )

            start = time.monotonic()
            while True:
                rc = proc.poll()
                if rc is not None:
                    break
                if timeout_s is not None and (time.monotonic() - start) >= timeout_s:
                    proc.kill()
                    out, err = proc.communicate()
                    rr = RunResult(stdout=out or "", stderr=(err or "").rstrip("\n") + f"\nTimed out after {timeout_s}s\n", returncode=124)
                    if attempt < retries:
                        time.sleep(retry_sleep_s)
                        break
                    if check:
                        raise TimeoutError(f"Command timed out after {timeout_s}s: {cmd}")
                    return rr
                time.sleep(0.05)

            if rc is None:
                # timed out + retry case: continue outer retry loop
                continue

            out, err = proc.communicate()
            rr = RunResult(stdout=out or "", stderr=err or "", returncode=int(rc))
            if rr.returncode == 0:
                return rr

            if attempt < retries:
                time.sleep(retry_sleep_s)
                continue

            if check:
                if self.verbose:
                    if rr.stdout.strip():
                        print("--- stdout ---", file=sys.stderr)
                        print(rr.stdout.rstrip("\n"), file=sys.stderr)
                    if rr.stderr.strip():
                        print("--- stderr ---", file=sys.stderr)
                        print(rr.stderr.rstrip("\n"), file=sys.stderr)
                raise subprocess.CalledProcessError(
                    rr.returncode,
                    cmd,
                    output=rr.stdout,
                    stderr=rr.stderr,
                )
            return rr
        raise RuntimeError("unreachable")


def start_dummy_control_http_server(*, token: str) -> tuple[_ThreadingHTTPServer, threading.Thread, int]:
    """
    Start a tiny local HTTP server that implements:
    - POST /control/unlock  (requires Authorization: Bearer <token>)

    Used to demo `rgbldk node unlock-hosted` without depending on a real hosted control server.
    """

    class Handler(http.server.BaseHTTPRequestHandler):
        def log_message(self, _fmt: str, *_args: object) -> None:
            return

        def do_POST(self) -> None:  # noqa: N802
            if self.path != "/control/unlock":
                self.send_response(404)
                self.send_header("Content-Length", "0")
                self.end_headers()
                return

            auth = self.headers.get("Authorization", "")
            if auth != f"Bearer {token}":
                body = json.dumps({"ok": False, "error": "unauthorized"}).encode("utf-8")
                self.send_response(401)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return

            body = json.dumps({"ok": True}).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    httpd: _ThreadingHTTPServer = _ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()
    return httpd, thread, int(httpd.server_address[1])


def start_dummy_control_socket_server(*, socket_path: Path) -> tuple[socketserver.UnixStreamServer, threading.Thread]:
    """
    Start a tiny local Unix-domain control socket that replies `{ "ok": true }` to any request.

    Used to demo `rgbldk node unlock` / `rgbldk node lock` without touching a real daemon.
    """

    class Handler(socketserver.StreamRequestHandler):
        def handle(self) -> None:
            _line = self.rfile.readline()
            self.wfile.write(b'{"ok":true}\n')

    if socket_path.exists():
        socket_path.unlink()
    server: socketserver.UnixStreamServer = socketserver.UnixStreamServer(str(socket_path), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    return server, thread


def is_tcp_port_available(port: int) -> bool:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.settimeout(0.2)
        return s.connect_ex(("127.0.0.1", port)) != 0


def pick_free_tcp_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        return int(s.getsockname()[1])


def main() -> int:
    p = argparse.ArgumentParser(description="Generate crates/cli/EXAMPLE.md from a local regtest run.")
    p.add_argument("--output", type=Path, default=DEFAULT_OUT)
    p.add_argument("--compose-file", type=Path, default=DEFAULT_COMPOSE_FILE)
    p.add_argument("--rgb-ldk-node-dir", type=Path, default=DEFAULT_RGB_LDK_NODE_DIR)
    p.add_argument(
        "--max-runtime-secs",
        type=float,
        default=DEFAULT_MAX_RUNTIME_SECS,
        help="Fail the generator if it runs longer than this (default: 900s).",
    )
    p.add_argument("--no-cleanup", action="store_true", help="Do not run docker compose down -v at the end.")
    p.add_argument("--skip-docker-up", action="store_true", help="Assume docker compose is already up.")
    p.add_argument("--skip-build", action="store_true", help="Skip cargo build; assumes rgbldk is already built.")
    p.add_argument(
        "--skip-daemon-build",
        action="store_true",
        help="Skip cargo build of rgbldkd; assumes ../rgb-ldk-node/target/debug/rgbldkd is already built.",
    )
    p.add_argument(
        "--yes-pull",
        action="store_true",
        help="If ../rgb-ldk-node is not present, pull remote images without prompting.",
    )
    p.add_argument("--verbose", action="store_true")
    args = p.parse_args()

    output_path: Path = args.output
    compose_file: Path = args.compose_file
    rgb_ldk_node_dir: Path = args.rgb_ldk_node_dir

    compose_file_display = (
        str(compose_file.relative_to(REPO_ROOT)) if compose_file.is_relative_to(REPO_ROOT) else str(compose_file)
    )
    rgb_ldk_node_dir_display = os.path.relpath(rgb_ldk_node_dir, REPO_ROOT)

    # Keep CLI config isolated so a local user config doesn't affect the generated output.
    # On macOS, the default temp directory path can be too long for Unix domain socket paths (SUN_LEN).
    # Using /tmp keeps control socket paths short and avoids startup failures.
    with tempfile.TemporaryDirectory(prefix="rgbldk-cli-example-", dir="/tmp") as tmpdir:
        start_ts = time.monotonic()

        def ensure_within_runtime(where: str) -> None:
            max_s = float(args.max_runtime_secs)
            if max_s <= 0:
                return
            elapsed = time.monotonic() - start_ts
            if elapsed > max_s:
                raise RuntimeError(
                    f"Generator exceeded max runtime ({max_s:.0f}s) at {where} (elapsed {elapsed:.1f}s)."
                )

        def clamp_timeout(timeout_s: Optional[float], where: str) -> Optional[float]:
            """
            Clamp a per-command timeout so the generator respects --max-runtime-secs.

            If --max-runtime-secs is <= 0, returns the input timeout.
            """
            max_s = float(args.max_runtime_secs)
            if max_s <= 0:
                return timeout_s
            elapsed = time.monotonic() - start_ts
            remaining = max_s - elapsed
            if remaining <= 0:
                raise RuntimeError(
                    f"Generator exceeded max runtime ({max_s:.0f}s) at {where} (elapsed {elapsed:.1f}s)."
                )
            if timeout_s is None:
                return remaining
            return min(timeout_s, remaining)

        env = dict(os.environ)
        env["XDG_CONFIG_HOME"] = tmpdir
        if not args.skip_docker_up:
            # Avoid local port collisions (bitcoind may already be running on common regtest ports).
            if not is_tcp_port_available(18443):
                env["BITCOIND_RPC_PORT"] = str(pick_free_tcp_port())
            if not is_tcp_port_available(18444):
                env["BITCOIND_REST_PORT"] = str(pick_free_tcp_port())
            # Esplora host ports (optional, but helpful for local tooling and this generator).
            if not is_tcp_port_available(3003):
                env["API_PORT"] = str(pick_free_tcp_port())
            if not is_tcp_port_available(50005):
                env["ELECTRUM_PORT"] = str(pick_free_tcp_port())

        runner = Runner(cwd=REPO_ROOT, env=env, verbose=args.verbose)
        md = Markdown()

        used_ports: set[int] = set()

        def pick_port(preferred: int) -> int:
            if preferred not in used_ports and is_tcp_port_available(preferred):
                used_ports.add(preferred)
                return preferred
            while True:
                p = pick_free_tcp_port()
                if p not in used_ports:
                    used_ports.add(p)
                    return p

        node_a_http_port = pick_port(8501)
        node_b_http_port = pick_port(8502)
        node_a_p2p_port = pick_port(9735)
        node_b_p2p_port = pick_port(9736)

        node_a = f"http://127.0.0.1:{node_a_http_port}"
        node_b = f"http://127.0.0.1:{node_b_http_port}"
        esplora = f"http://127.0.0.1:{env.get('API_PORT', '3003')}"
        node_a_p2p = f"127.0.0.1:{node_a_p2p_port}"
        node_b_p2p = f"127.0.0.1:{node_b_p2p_port}"
        issuer_fixture = rgb_ldk_node_dir / "tests" / "issuers" / "RGB20-Simplest-v0-rLosfg.issuer"
        demo_export_dir = REPO_ROOT / "target" / "rgbldk-example"
        contract_id: Optional[str] = None
        asset_id: Optional[str] = None
        node_id_a: Optional[str] = None
        node_id_b: Optional[str] = None

        md.heading(1, "`rgbldk` CLI examples (regtest, docker-compose)")
        md.paragraph(
            "This file is generated from a real local test run on regtest using `crates/cli/docker-compose.yaml`.\n\n"
            "Daemon mode:\n"
            "- If `../rgb-ldk-node` exists: build and start two local `rgbldkd` daemons from source.\n"
            "- Otherwise: use the docker image(s) defined in `crates/cli/docker-compose.yaml`.\n\n"
            "Notes:\n"
            "- Outputs contain IDs (txids, invoices, payment IDs, node IDs, channel IDs) that differ per run.\n"
            "- `rgbldk` is a client CLI talking to `rgbldkd` over HTTP.\n"
        )
        md.paragraph(
            "Mutual transfers covered in this example (BTC/RGB × L1/L2):\n"
            "- BTC L1: `BTC on-chain settlement (L1, node-b → node-a ...)` and `BTC on-chain settlement (L1, node-a → node-b ...)`\n"
            "- BTC L2: `BTC Lightning transfer (L2, node-a → node-b)` and `BTC Lightning transfer (L2, node-b → node-a)`\n"
            "- RGB L1: `RGB on-chain transfer (L1, node-a → node-b)` and `RGB on-chain transfer (L1, node-b → node-a)`\n"
            "- RGB L2: `RGB Lightning transfer (L2, node-a → node-b)` and `RGB Lightning transfer (L2, node-b → node-a)`\n"
        )

        md.heading(2, "0) Start environment")
        md.paragraph(
            "This section starts a self-contained regtest environment (bitcoind + esplora + two `rgbldkd` nodes) and "
            "shows the endpoints you will use in the rest of the examples.\n\n"
            "In source mode, docker-compose starts bitcoind + esplora, and the script starts the two `rgbldkd` daemons "
            "locally. In docker-image mode, docker-compose starts all services (including `node-a` and `node-b`).\n\n"
            "Note: Starting from Phase 0, `rgbldkd` starts locked. This generator initializes a local keystore and "
            "unlocks the daemons (source mode only) before running the rest of the commands.\n"
        )
        md.heading(3, "Commands")

        use_src = rgb_ldk_node_dir.exists()
        if not use_src:
            print(
                f'ERROR: ../rgb-ldk-node not found at "{rgb_ldk_node_dir}". EXAMPLE generation currently requires '
                "a local rgb-ldk-node checkout so the script can unlock the daemons via the local control socket.",
                file=sys.stderr,
            )
            return 2

        if use_src:
            docker_up_cmd = f"DOCKER_BUILDKIT=1 docker compose -f {compose_file} up -d bitcoind chain-init esplora wait-esplora"
            docker_up_cmd_display = (
                f"DOCKER_BUILDKIT=1 docker compose -f {compose_file_display} up -d bitcoind chain-init esplora wait-esplora"
            )
        else:
            docker_up_cmd = f"DOCKER_BUILDKIT=1 docker compose -f {compose_file} up -d"
            docker_up_cmd_display = f"DOCKER_BUILDKIT=1 docker compose -f {compose_file_display} up -d"
        md.command_only(docker_up_cmd_display)
        rgbldkd_proc_a: Optional[subprocess.Popen[str]] = None
        rgbldkd_proc_b: Optional[subprocess.Popen[str]] = None
        proxy_server: Optional[_ThreadingHTTPServer] = None
        proxy_thread: Optional[threading.Thread] = None
        dummy_control_http_server: Optional[_ThreadingHTTPServer] = None
        dummy_control_http_thread: Optional[threading.Thread] = None
        dummy_control_socket_server: Optional[socketserver.UnixStreamServer] = None
        dummy_control_socket_thread: Optional[threading.Thread] = None
        dummy_control_socket_path: Optional[Path] = None
        try:
            if not args.skip_docker_up:
                _progress(docker_up_cmd_display)
                runner.run(docker_up_cmd, timeout_s=clamp_timeout(1800.0, "docker compose up"))

            if use_src:
                build_daemon_cmd = f"(cd {rgb_ldk_node_dir} && cargo build -p ldk-node --bin rgbldkd)"
                build_daemon_cmd_display = (
                    f"(cd {rgb_ldk_node_dir_display} && cargo build -p ldk-node --bin rgbldkd)"
                )
                md.command_only(build_daemon_cmd_display)
                if not args.skip_daemon_build:
                    _progress(build_daemon_cmd_display)
                    runner.run(build_daemon_cmd, timeout_s=clamp_timeout(1800.0, "cargo build rgbldkd"))

                data_dir_a = Path(tmpdir) / "rgbldkd-node-a"
                data_dir_b = Path(tmpdir) / "rgbldkd-node-b"
                data_dir_a.mkdir(parents=True, exist_ok=True)
                data_dir_b.mkdir(parents=True, exist_ok=True)

                rgbldkd_bin = rgb_ldk_node_dir / "target" / "debug" / "rgbldkd"
                if not rgbldkd_bin.exists():
                    raise RuntimeError(f"rgbldkd binary not found at {rgbldkd_bin}")

                _progress("starting local rgbldkd node-a/node-b")
                cmd_a = (
                    f"{rgbldkd_bin} server "
                    f"--listen 127.0.0.1:{node_a_http_port} "
                    f"--ldk-listen {node_a_p2p} "
                    f"--network regtest "
                    f"--rgb-enabled "
                    f"--esplora-url {esplora} "
                    f"--data-dir {data_dir_a} "
                    f"--node-alias node-a "
                    f"--log-to-stdout --log-level info"
                )
                cmd_b = (
                    f"{rgbldkd_bin} server "
                    f"--listen 127.0.0.1:{node_b_http_port} "
                    f"--ldk-listen {node_b_p2p} "
                    f"--network regtest "
                    f"--rgb-enabled "
                    f"--esplora-url {esplora} "
                    f"--data-dir {data_dir_b} "
                    f"--node-alias node-b "
                    f"--log-to-stdout --log-level info"
                )

                # Start daemons (not included in markdown; markdown is focused on the client-side interactions).
                stdout_target = None if args.verbose else subprocess.DEVNULL
                stderr_target = None if args.verbose else subprocess.DEVNULL
                rgbldkd_proc_a = subprocess.Popen(
                    sh(cmd_a),
                    cwd=REPO_ROOT,
                    env=env,
                    text=True,
                    stdout=stdout_target,
                    stderr=stderr_target,
                    start_new_session=True,
                )
                rgbldkd_proc_b = subprocess.Popen(
                    sh(cmd_b),
                    cwd=REPO_ROOT,
                    env=env,
                    text=True,
                    stdout=stdout_target,
                    stderr=stderr_target,
                    start_new_session=True,
                )

            build_cmd = "cargo build -p rgbldk-cli --bin rgbldk"
            md.command_only(build_cmd)
            if not args.skip_build:
                _progress("cargo build -p rgbldk-cli --bin rgbldk")
                runner.run(build_cmd, timeout_s=clamp_timeout(1800.0, "cargo build rgbldk"))

            export_path_cmd = 'export PATH="$PWD/target/debug:$PATH"'
            md.command_only(export_path_cmd)
            env["PATH"] = f"{REPO_ROOT}/target/debug:{env.get('PATH','')}"

            md.heading(3, "Endpoints")
            md.fence(
                "text",
                f"node_a={node_a}\nnode_b={node_b}\nesplora={esplora}\n",
            )

            def run_step(
                cmd: str,
                *,
                display_cmd: Optional[str] = None,
                retries: int = 0,
                retry_sleep_s: float = 1.0,
                include_result: bool = True,
                check: bool = True,
                timeout_s: Optional[float] = None,
            ) -> RunResult:
                ensure_within_runtime(f"run_step: {cmd.splitlines()[0][:120]}")
                md.run_block(display_cmd or cmd)
                if timeout_s is None:
                    timeout_s = DEFAULT_STEP_TIMEOUT_S
                timeout_s = clamp_timeout(timeout_s, f"run_step: {cmd.splitlines()[0][:120]}")
                rr = runner.run(
                    normalize_rgbldk_cmd(cmd),
                    retries=retries,
                    retry_sleep_s=retry_sleep_s,
                    check=check,
                    timeout_s=timeout_s,
                )
                if include_result:
                    out = rr.stdout
                    if rr.stderr.strip():
                        out = (out.rstrip("\n") + "\n" + rr.stderr).lstrip("\n")
                    if rr.returncode != 0:
                        out = (out.rstrip("\n") + f"\n(exit {rr.returncode})\n").lstrip("\n")
                    md.quoted_text_fence(out)
                return rr

            def run_step_json(
                cmd: str,
                *,
                display_cmd: Optional[str] = None,
                retries: int = 0,
                retry_sleep_s: float = 1.0,
                check: bool = True,
                timeout_s: Optional[float] = None,
            ) -> dict[str, Any]:
                ensure_within_runtime(f"run_step_json: {cmd.splitlines()[0][:120]}")
                cmd_json = normalize_rgbldk_json_cmd(cmd)
                rr = run_step(
                    cmd_json,
                    display_cmd=display_cmd or cmd_json,
                    retries=retries,
                    retry_sleep_s=retry_sleep_s,
                    check=check,
                    timeout_s=timeout_s,
                )
                obj = json.loads(rr.stdout)
                if not isinstance(obj, dict):
                    raise RuntimeError(
                        f"Invalid JSON shape (expected object) for: {cmd_json}\n{rr.stdout}"
                    )
                return obj

            def bitcoind_cli(cmd: str, *, include_result: bool = True) -> RunResult:
                return run_step(
                    f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin {cmd}",
                    display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin {cmd}",
                    include_result=include_result,
                )

            def bitcoind_cli_json(cmd: str) -> Any:
                rr = runner.run(
                    f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin {cmd}",
                    timeout_s=DEFAULT_STEP_TIMEOUT_S,
                )
                return json.loads(rr.stdout)

            def tx_outpoint_for_address(txid: str, address: str) -> str:
                tx = bitcoind_cli_json(f"getrawtransaction {txid} true")
                if not isinstance(tx, dict) or not isinstance(tx.get("vout"), list):
                    raise RuntimeError(f"Unexpected getrawtransaction shape: {tx}")
                for vout in tx["vout"]:
                    if not isinstance(vout, dict):
                        continue
                    n = vout.get("n")
                    spk = vout.get("scriptPubKey")
                    if not isinstance(n, int) or not isinstance(spk, dict):
                        continue
                    addr = spk.get("address")
                    addrs = spk.get("addresses")
                    if addr == address or (isinstance(addrs, list) and address in addrs):
                        return f"{txid}:{n}"
                raise RuntimeError(f"Could not find vout for address {address} in tx {txid}")

            def wait_for_spendable_onchain_sats(connect: str, min_sats: int, *, timeout_s: float = 60.0) -> None:
                """
                Poll node wallet until `btc.onchain_spendable_sats` is at least `min_sats`.

                Not included in markdown (used to make the example generation robust against esplora indexing delays).
                """
                deadline = time.time() + timeout_s
                last_sync_err: Optional[str] = None
                while time.time() < deadline:
                    ensure_within_runtime(f"wait_for_spendable_onchain_sats({connect})")
                    # Keep the wallet in sync while esplora catches up.
                    sync_rr = runner.run(
                        normalize_rgbldk_cmd(f"rgbldk --connect {connect} wallet sync"),
                        retries=3,
                        retry_sleep_s=1.0,
                        check=False,
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, f"wallet sync ({connect})"),
                    )
                    if sync_rr.returncode != 0:
                        last_sync_err = (sync_rr.stdout + "\n" + sync_rr.stderr).strip()
                        time.sleep(1.0)
                        continue

                    rr = runner.run(
                        f"rgbldk --color never --output json --connect {connect} wallet balance --sats",
                        retries=1,
                        retry_sleep_s=1.0,
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, f"wallet balance ({connect})"),
                    )
                    obj = json.loads(rr.stdout)
                    if not isinstance(obj, dict):
                        raise RuntimeError(f"Invalid wallet balance JSON shape (expected object): {obj}")

                    v = (obj.get("btc") or {}).get("onchain_spendable_sats")
                    if isinstance(v, int):
                        spendable = v
                    elif isinstance(v, str) and v.strip().isdigit():
                        spendable = int(v.strip())
                    else:
                        raise RuntimeError(f"Invalid btc.onchain_spendable_sats in wallet balance: {v!r}")

                    if spendable >= min_sats:
                        return

                    time.sleep(1.0)

                raise RuntimeError(
                    f"Timed out waiting for spendable on-chain balance >= {min_sats} sats on {connect}. "
                    f"Last wallet sync error: {last_sync_err!r}. "
                    f"Try re-running after `docker compose -f {compose_file_display} down -v`."
                )

            def wait_for_usable_channel(connect: str, *, timeout_s: float = 60.0) -> None:
                """
                Wait until at least one channel is usable on `connect`.

                Not included in markdown (prevents flaky payment steps right after channel open).
                """
                deadline = time.time() + timeout_s
                last_err: Optional[str] = None
                while time.time() < deadline:
                    ensure_within_runtime(f"wait_for_usable_channel({connect})")
                    # Keep the wallet in sync while esplora catches up to freshly mined blocks.
                    sync_rr = runner.run(
                        normalize_rgbldk_cmd(f"rgbldk --connect {connect} wallet sync"),
                        retries=3,
                        retry_sleep_s=1.0,
                        check=False,
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, f"wallet sync ({connect})"),
                    )
                    if sync_rr.returncode != 0:
                        last_err = (sync_rr.stdout + "\n" + sync_rr.stderr).strip()
                        time.sleep(1.0)
                        continue

                    rr = runner.run(
                        f"rgbldk --color never --output json --connect {connect} channel ls",
                        retries=1,
                        retry_sleep_s=1.0,
                        check=False,
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, f"channel ls ({connect})"),
                    )
                    if rr.returncode != 0:
                        last_err = (rr.stdout + "\n" + rr.stderr).strip()
                        time.sleep(1.0)
                        continue
                    obj = json.loads(rr.stdout)

                    if isinstance(obj, list):
                        for ch in obj:
                            if isinstance(ch, dict) and ch.get("is_usable") is True:
                                return

                    time.sleep(1.0)

                raise RuntimeError(f"Timed out waiting for a usable channel on {connect}. Last error: {last_err!r}")

            # Give daemons a moment after docker up (do not include in the markdown).
            if not args.skip_docker_up:
                ensure_within_runtime("docker compose up")
                runner.run(
                    normalize_rgbldk_cmd(f"rgbldk --connect {node_a} node health"),
                    retries=60,
                    retry_sleep_s=1.0,
                    timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "node health (node-a)"),
                )

            # Phase 0: initialize keystores and unlock daemons (do not include in the markdown).
            passphrase = "example-passphrase"
            for dd in (data_dir_a, data_dir_b):
                ensure_within_runtime("keystore init/unlock")
                runner.run(
                    f"printf '%s\\n' '{passphrase}' | rgbldk --output text --color never --yes --data-dir {dd} keystore init --mode generate-mnemonic --passphrase-stdin",
                    timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "keystore init"),
                )
                runner.run(
                    f"printf '%s\\n' '{passphrase}' | rgbldk --output json --pretty --color never --data-dir {dd} node unlock --passphrase-stdin",
                    retries=30,
                    retry_sleep_s=1.0,
                    timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "node unlock"),
                )

            # Wait until the real HTTP API is up after unlock.
            runner.run(
                normalize_rgbldk_cmd(f"rgbldk --connect {node_a} node ready"),
                retries=60,
                retry_sleep_s=1.0,
                timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "node ready (node-a)"),
            )

            md.heading(2, "1) Contexts (ctx)")
            md.paragraph(
                "Contexts let you name daemon endpoints (e.g. `node-a`, `node-b`) and switch the default target without "
                "repeatedly passing `--connect`.\n"
            )
            md.heading(3, "Add + show")

            run_step(f"rgbldk ctx add node-a --url {node_a} --use-now")
            run_step(f"rgbldk ctx add node-b --url {node_b}")
            run_step("rgbldk ctx ls")
            run_step("rgbldk ctx show")
            run_step("rgbldk ctx use node-b")
            run_step("rgbldk ctx show")
            run_step("rgbldk ctx use node-a")

            md.heading(3, "Remove")
            run_step("rgbldk ctx add demo --url http://127.0.0.1:1")
            run_step("rgbldk ctx rm demo")

            md.heading(2, "2) Node basics (version/health/ready/status)")
            md.paragraph(
                "Use these commands to verify the CLI can reach the daemon, and to understand *why* a node is (not) ready "
                "via sub-checks.\n"
            )
            md.heading(3, "Health + status + identity")

            run_step("rgbldk node version")
            run_step("rgbldk node health")
            run_step("rgbldk node ready")
            run_step("rgbldk node status")
            run_step("rgbldk wallet balance")

            node_id_a = run_step_json("rgbldk node id").get("node_id")
            if not isinstance(node_id_a, str) or not node_id_a:
                raise RuntimeError(f"Invalid node_id from node-a: {node_id_a}")
            run_step("rgbldk node listen")

            run_step("rgbldk ctx use node-b")
            run_step("rgbldk node ready")
            node_id_b = run_step_json("rgbldk node id").get("node_id")
            if not isinstance(node_id_b, str) or not node_id_b:
                raise RuntimeError(f"Invalid node_id from node-b: {node_id_b}")
            run_step("rgbldk node listen")
            run_step("rgbldk ctx use node-a")

            md.heading(3, "Local-only keystore + lock/unlock")
            md.paragraph(
                "These commands do not require the daemon HTTP API. They operate on local files (keystore) and a local "
                "control socket / control HTTP server.\n"
            )
            md.heading(4, "Keystore init (generate mnemonic)")
            keystore_demo_dir = Path(tmpdir) / "keystore-init-demo"
            keystore_demo_dir.mkdir(parents=True, exist_ok=True)
            run_step(
                f"printf '%s\\n' '{passphrase}' | rgbldk --yes --data-dir {keystore_demo_dir} keystore init --mode generate-mnemonic --passphrase-stdin",
                display_cmd="printf '%s\\n' '<passphrase>' | rgbldk --yes --data-dir <data_dir> keystore init --mode generate-mnemonic --passphrase-stdin",
            )

            md.heading(4, "Keystore migrate (legacy keys_seed -> encrypted keystore)")
            keystore_migrate_dir = Path(tmpdir) / "keystore-migrate-demo"
            keystore_migrate_dir.mkdir(parents=True, exist_ok=True)
            (keystore_migrate_dir / "keys_seed").write_bytes(b"\x00" * 64)
            run_step(
                f"printf '%s\\n' '{passphrase}' | rgbldk --data-dir {keystore_migrate_dir} keystore migrate --passphrase-stdin",
                display_cmd="printf '%s\\n' '<passphrase>' | rgbldk --data-dir <data_dir> keystore migrate --passphrase-stdin",
            )

            md.heading(4, "Node unlock/lock (dummy local control socket)")
            control_demo_dir = Path(tmpdir) / "control-socket-demo"
            control_demo_dir.mkdir(parents=True, exist_ok=True)
            dummy_control_socket_path = control_demo_dir / "control.sock"
            dummy_control_socket_server, dummy_control_socket_thread = start_dummy_control_socket_server(
                socket_path=dummy_control_socket_path
            )
            run_step(
                f"printf '%s\\n' '{passphrase}' | rgbldk --yes --data-dir {control_demo_dir} node unlock --passphrase-stdin",
                display_cmd="printf '%s\\n' '<passphrase>' | rgbldk --yes --data-dir <data_dir> node unlock --passphrase-stdin",
            )
            run_step(
                f"rgbldk --yes --data-dir {control_demo_dir} node lock",
                display_cmd="rgbldk --yes --data-dir <data_dir> node lock",
            )

            md.heading(4, "Node unlock-hosted (dummy control HTTP server)")
            hosted_token = "demo-control-token"
            dummy_control_http_server, dummy_control_http_thread, hosted_port = start_dummy_control_http_server(
                token=hosted_token
            )
            run_step(
                f"printf '%s\\n' '{hosted_token}' | rgbldk node unlock-hosted --control-connect http://127.0.0.1:{hosted_port} --control-token-stdin",
                display_cmd="printf '%s\\n' '<control_token>' | rgbldk node unlock-hosted --control-connect http://127.0.0.1:<port> --control-token-stdin",
            )

            md.heading(2, "3) Peers (ls)")
            md.paragraph(
                "List currently known peers. This is usually empty before you open channels or connect peers explicitly.\n"
            )
            md.heading(3, "List peers")
            run_step("rgbldk peer ls")
            md.paragraph(
                "Tip: tables truncate long IDs by default to keep the output readable on an 80-column terminal. "
                "Use `--no-truncate` when you need to copy the full value.\n\n"
                "**Show full IDs (no truncation):**\n"
            )
            run_step("rgbldk --no-truncate peer ls")

            md.heading(3, "Connect + disconnect")
            if not node_id_b:
                raise RuntimeError("missing node_id_b (expected it to be set in the node basics section)")
            run_step(
                f"rgbldk peer connect {node_id_b} {node_b_p2p} --persist",
                display_cmd="rgbldk peer connect <node_id_b> <node_b_p2p> --persist",
            )
            if not node_id_a:
                raise RuntimeError("missing node_id_a (expected it to be set in the node basics section)")
            run_step("rgbldk ctx use node-b")
            run_step(
                f"rgbldk peer connect {node_id_a} {node_a_p2p} --persist",
                display_cmd="rgbldk peer connect <node_id_a> <node_a_p2p> --persist",
            )
            run_step("rgbldk ctx use node-a")
            run_step("rgbldk peer ls")
            md.paragraph(
                "Keep the peer connected for the rest of this example so channel opening and payments are reliable. "
                "A `peer disconnect` example is included in the cleanup section.\n"
            )

            md.heading(2, "4) Wallet (address/sync) + fund on regtest")
            md.paragraph(
                "Generate on-chain addresses for both nodes, fund them from the regtest miner wallet, and then "
                "`wallet sync` to make balances visible to the node wallet.\n"
            )
            md.heading(3, "Generate addresses")

            addr_a = run_step_json("rgbldk wallet address").get("address")
            if not isinstance(addr_a, str) or not addr_a:
                raise RuntimeError(f"Invalid wallet address for node-a: {addr_a}")
            run_step("rgbldk ctx use node-b")
            addr_b = run_step_json("rgbldk wallet address").get("address")
            if not isinstance(addr_b, str) or not addr_b:
                raise RuntimeError(f"Invalid wallet address for node-b: {addr_b}")
            run_step("rgbldk ctx use node-a")

            md.heading(3, "Fund + sync")
            miner_addr = first_nonempty_line(
                run_step(
                    f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin getnewaddress",
                    display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin getnewaddress",
                ).stdout
            )
            run_step(
                f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress {addr_a} 1",
                display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress {addr_a} 1",
            )
            run_step(
                f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress {addr_b} 1",
                display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress {addr_b} 1",
            )
            run_step(
                f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 {miner_addr}"
                ,
                display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 {miner_addr}",
            )

            # Wait until both nodes see spendable funds before continuing (do not include in markdown).
            wait_for_spendable_onchain_sats(node_a, 100_000, timeout_s=90.0)
            wait_for_spendable_onchain_sats(node_b, 100_000, timeout_s=90.0)

            run_step("rgbldk wallet sync")
            run_step(f"rgbldk --connect {node_b} wallet sync")
            run_step("rgbldk wallet balance")
            run_step("rgbldk wallet balance --sats")
            run_step(f"rgbldk --connect {node_b} wallet balance")
            run_step(f"rgbldk --connect {node_b} wallet balance --sats")

            md.heading(2, "5) BTC on-chain settlement (L1, node-b → node-a via channel push+close)")
            md.paragraph(
                "This demonstrates a BTC L1 transfer between the two nodes by using a channel open with `--push-msat` "
                "(gives the receiver an initial balance) and then a cooperative `channel close` to settle on-chain.\n"
            )
            md.heading(3, "Commands")
            if not node_id_a:
                raise RuntimeError("missing node_id_a (expected it to be set in the node basics section)")
            run_step("rgbldk ctx use node-b")
            chan_b_to_a = run_step_json(
                f"rgbldk channel open --node-id {node_id_a} --addr {node_a_p2p} --amount-sats 60000 --push-msat 2000000 --private",
                display_cmd="rgbldk channel open --node-id <node_id_a> --addr <node_a_p2p> --amount-sats 60000 --push-msat 2000000 --private",
                retries=30,
                retry_sleep_s=1.0,
            ).get("user_channel_id")
            if not isinstance(chan_b_to_a, str) or not chan_b_to_a:
                raise RuntimeError(f"Invalid user_channel_id from channel open: {chan_b_to_a}")
            bitcoind_cli(f"generatetoaddress 6 {miner_addr}")
            wait_for_usable_channel(node_b, timeout_s=90.0)
            run_step(
                f"rgbldk channel close --user-channel-id {chan_b_to_a} --counterparty-node-id {node_id_a}",
                display_cmd="rgbldk channel close --user-channel-id <user_channel_id> --counterparty-node-id <node_id_a>",
            )
            bitcoind_cli(f"generatetoaddress 6 {miner_addr}")
            run_step("rgbldk ctx use node-a")
            run_step("rgbldk wallet sync")
            run_step("rgbldk wallet balance")
            run_step("rgbldk ctx use node-b")
            run_step("rgbldk wallet sync")
            run_step("rgbldk wallet balance")
            run_step("rgbldk ctx use node-a")

            md.heading(2, "6) RGB (sync/issuers/contracts)")
            md.paragraph(
                "Demonstrate basic RGB wallet sync and a minimal issuer/contract workflow: import an issuer archive, "
                "issue a contract, export a consignment, and import it into the other node.\n"
            )

            if not use_src:
                md.paragraph(
                    "Note: RGB demo is skipped in docker-image mode. It requires starting `rgbldkd` with `--rgb-enabled` "
                    "(and a reachable Bitcoin Core REST endpoint), which is currently only configured in source mode.\n"
                )
            else:
                md.heading(3, "Sync + inventory")
                run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)
                rgb_addr_a = run_step_json("rgbldk rgb address").get("address")
                if not isinstance(rgb_addr_a, str) or not rgb_addr_a:
                    raise RuntimeError(f"Invalid RGB wallet address for node-a: {rgb_addr_a}")
                run_step("rgbldk rgb issuers ls")
                run_step("rgbldk rgb contracts ls")
                run_step("rgbldk rgb utxos ls")
                run_step("rgbldk rgb utxos summary")

                if issuer_fixture.exists():
                    md.heading(3, "Issue + share")
                    demo_export_dir.mkdir(parents=True, exist_ok=True)
                    issuer_name = "demo-issuer"
                    contract_name = "DemoAsset"

                    md.paragraph(
                        "Issuing a contract requires the RGB wallet to have a spendable UTXO. Fund the RGB wallet "
                        "address and sync before issuing.\n"
                    )
                    run_step(
                        f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress {rgb_addr_a} 0.1",
                        display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address> 0.1",
                    )
                    run_step(
                        f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 {miner_addr}",
                        display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 {miner_addr}",
                    )
                    run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)

                    md.heading(4, "UTXOs (reserve/release)")
                    run_step("rgbldk rgb utxos summary")
                    resv = run_step_json("rgbldk rgb utxos reserve --ttl-secs 60").get("reservation_id")
                    if not isinstance(resv, str) or not resv:
                        raise RuntimeError(f"Invalid reservation_id from rgb utxos reserve: {resv}")
                    run_step(
                        f"rgbldk rgb utxos release --reservation-id {resv}",
                        display_cmd="rgbldk rgb utxos release --reservation-id <reservation_id>",
                    )

                    run_step(
                        f"rgbldk rgb issuers import --name {issuer_name} --file {issuer_fixture}",
                        display_cmd=f"rgbldk rgb issuers import --name {issuer_name} --file ../rgb-ldk-node/tests/issuers/RGB20-Simplest-v0-rLosfg.issuer",
                    )
                    run_step("rgbldk rgb issuers ls")

                    issued = run_step_json(
                        f"rgbldk rgb contracts issue --issuer-name {issuer_name} --contract-name \"{contract_name}\" --ticker DEMO --precision 0 --issued-supply 100",
                        retries=5,
                        retry_sleep_s=1.0,
                    )
                    if issued.get("ok") is not True:
                        raise RuntimeError(f"RGB contract issue failed: {issued}")
                    contract_id = issued.get("contract_id")
                    asset_id = issued.get("asset_id")
                    if not isinstance(contract_id, str) or not contract_id:
                        raise RuntimeError(f"Invalid contract_id in issue response: {issued}")
                    if not isinstance(asset_id, str) or not asset_id:
                        raise RuntimeError(f"Invalid asset_id in issue response: {issued}")

                    run_step("rgbldk rgb contracts ls")
                    run_step(f"rgbldk rgb contracts balance {contract_id}")

                    consignment_file = demo_export_dir / f"{contract_id}.raw"
                    run_step(
                        f"rgbldk rgb contracts export --contract-id {contract_id} --out {consignment_file} --format raw --direct"
                    )
                    run_step(
                        f"rgbldk debug consignment {consignment_file} --format raw",
                        display_cmd="rgbldk debug consignment <contract.raw> --format raw",
                    )
                    consignment_zip = demo_export_dir / f"{contract_id}.zip"
                    exported = run_step_json(
                        f"rgbldk rgb contracts export --contract-id {contract_id} --out {consignment_zip} --format zip",
                        display_cmd="rgbldk rgb contracts export --contract-id <contract_id> --out <out.zip> --format zip",
                    )
                    export_obj = exported.get("export")
                    if not isinstance(export_obj, dict):
                        raise RuntimeError(f"Invalid export response JSON shape: {exported}")
                    export_key = export_obj.get("consignment_key")
                    if not isinstance(export_key, str) or not export_key:
                        raise RuntimeError(f"Invalid consignment_key in export response: {exported}")
                    consignment_zip_dl = demo_export_dir / f"{contract_id}.download.zip"
                    run_step(
                        f"rgbldk rgb consignments download --key {export_key} --out {consignment_zip_dl} --format zip",
                        display_cmd="rgbldk rgb consignments download --key <consignment_key> --out <out.zip> --format zip",
                    )

                    run_step("rgbldk ctx use node-b")
                    run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)
                    rgb_addr_b = run_step_json("rgbldk rgb address").get("address")
                    if not isinstance(rgb_addr_b, str) or not rgb_addr_b:
                        raise RuntimeError(f"Invalid RGB wallet address for node-b: {rgb_addr_b}")
                    md.paragraph(
                        "Fund the RGB wallet on node-b as well. This ensures node-b has spendable RGB wallet UTXOs for "
                        "later on-chain sends.\n"
                    )
                    run_step(
                        f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress {rgb_addr_b} 0.1",
                        display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin sendtoaddress <rgb_address_b> 0.1",
                    )
                    run_step(
                        f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 {miner_addr}",
                        display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 1 {miner_addr}",
                    )
                    run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)
                    run_step(
                        f"rgbldk rgb contracts import --contract-id {contract_id} --file {consignment_file}"
                    )
                    run_step("rgbldk rgb contracts ls")
                    run_step(f"rgbldk rgb contracts balance {contract_id}")

                    md.heading(4, "RGB on-chain transfer (L1, node-a → node-b)")
                    md.paragraph(
                        "Send most of the issued supply from node-a to node-b before opening an RGB-enabled channel. "
                        "This leaves node-a with exactly 10 units, so the channel funding transaction can spend that "
                        "UTXO without creating additional RGB change outputs.\n"
                    )

                    # node-a -> node-b (send 90, leave 10 for channel funding)
                    inv_split = run_step_json(
                        f"rgbldk rgb onchain invoice-create --contract-id {contract_id} --amount 90",
                        display_cmd="rgbldk rgb onchain invoice-create --contract-id <contract_id> --amount 90",
                    ).get("invoice")
                    if not isinstance(inv_split, str) or not inv_split:
                        raise RuntimeError(f"Invalid RGB on-chain invoice from node-b: {inv_split}")
                    inv_split_payment_id = hashlib.sha256(inv_split.encode("utf-8")).hexdigest()
                    run_step(
                        f"rgbldk debug invoice '{inv_split}'",
                        display_cmd="rgbldk debug invoice '<invoice>'",
                    )
                    run_step("rgbldk ctx use node-a")
                    ensure_within_runtime("pre rgb onchain send (node-a) wallet/rgb sync")
                    runner.run(
                        normalize_rgbldk_cmd("rgbldk wallet sync"),
                        retries=3,
                        retry_sleep_s=1.0,
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "pre-send wallet sync (node-a)"),
                    )
                    runner.run(
                        normalize_rgbldk_cmd("rgbldk rgb sync"),
                        retries=10,
                        retry_sleep_s=1.0,
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "pre-send rgb sync (node-a)"),
                    )
                    send_split = run_step_json(
                        f"rgbldk rgb onchain send --invoice '{inv_split}' --sats-for-fee-and-outputs 10000 --fee-rate-sats-per-vb 1.0",
                        display_cmd="rgbldk rgb onchain send --invoice '<invoice>' --sats-for-fee-and-outputs 10000 --fee-rate-sats-per-vb 1.0",
                        retries=5,
                        retry_sleep_s=2.0,
                        timeout_s=180.0,
                    )
                    consignment_key_split = send_split.get("consignment_key")
                    if not isinstance(consignment_key_split, str) or not consignment_key_split:
                        raise RuntimeError(f"Invalid consignment_key from rgb onchain send: {send_split}")
                    cons_split = demo_export_dir / "rgb-onchain-a-to-b-prechannel.zip"
                    run_step(
                        f"rgbldk rgb consignments download --key {consignment_key_split} --out {cons_split} --format zip",
                        display_cmd="rgbldk rgb consignments download --key <consignment_key> --out <a-to-b.zip> --format zip",
                    )
                    run_step("rgbldk ctx use node-b")
                    run_step(
                        f"rgbldk rgb onchain receive --file {cons_split} --format zip --payment-id {inv_split_payment_id}",
                        display_cmd="rgbldk rgb onchain receive --file <a-to-b.zip> --format zip --payment-id <payment_id>",
                    )
                    bitcoind_cli(f"generatetoaddress 6 {miner_addr}")
                    run_step("rgbldk wallet sync")
                    run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)
                    run_step(f"rgbldk rgb contracts balance {contract_id}")
                    run_step("rgbldk ctx use node-a")
                    run_step("rgbldk wallet sync")
                    run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)
                    run_step(f"rgbldk rgb contracts balance {contract_id}")
                    run_step("rgbldk ctx use node-b")

                else:
                    md.paragraph(
                        "Note: issuer/contract issuance demo is skipped because no issuer fixture was found at "
                        f"`{issuer_fixture}`.\n"
                    )

            # Ensure subsequent sections run against node-a by default.
            run_step("rgbldk ctx use node-a")

            md.heading(2, "7) Channel (ls/open)")
            md.paragraph(
                "Open a private Lightning channel from node-a to node-b and wait for confirmations so the channel becomes usable.\n"
            )
            if asset_id:
                md.paragraph(
                    "If you provide `--rgb-context` when opening the channel, the daemon will share it with the "
                    "counterparty as `color_context_data`. In a cross-host setup, this is typically an `http(s)` URL "
                    "template that lets the receiver fetch the *funding consignment* out-of-band.\n"
                )
                md.heading(3, "Open + confirm (RGB-enabled, consignment over HTTP)")
            else:
                md.heading(3, "Open + confirm")

            if not node_id_b:
                raise RuntimeError("missing node_id_b (expected it to be set in the node basics section)")

            # Capture current channels (best-effort) so we can identify the newly opened one even if the user_channel_id
            # isn't immediately reflected in `channel ls` (or if closed channels linger in the list).
            existing_channel_ids: set[str] = set()
            rr_existing = runner.run(
                normalize_rgbldk_json_cmd("rgbldk channel ls"),
                check=False,
                timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "channel ls (pre-open snapshot)"),
            )
            if rr_existing.returncode == 0:
                existing_obj = json.loads(rr_existing.stdout)
                if isinstance(existing_obj, list):
                    for c in existing_obj:
                        if isinstance(c, dict):
                            cid = c.get("channel_id")
                            if isinstance(cid, str) and cid:
                                existing_channel_ids.add(cid)

            if asset_id:
                rgb_context = f"{node_a}/api/v1/rgb/consignments/{{txid}}?format=zip"
                chan_100k = run_step_json(
                    f"rgbldk channel open --node-id {node_id_b} --addr {node_b_p2p} --amount-sats 100000 --push-msat 20000000 --private --rgb-asset-id {asset_id} --rgb-asset-amount 10 --rgb-context '{rgb_context}'",
                    display_cmd="rgbldk channel open --node-id <node_id_b> --addr <node_b_p2p> --amount-sats 100000 --push-msat 20000000 --private --rgb-asset-id <asset_id_hex> --rgb-asset-amount 10 --rgb-context 'http://<A_HOST>:8501/api/v1/rgb/consignments/{txid}?format=zip'",
                    retries=30,
                    retry_sleep_s=1.0,
                ).get("user_channel_id")
            else:
                chan_100k = run_step_json(
                    f"rgbldk channel open --node-id {node_id_b} --addr {node_b_p2p} --amount-sats 100000 --push-msat 20000000 --private",
                    retries=30,
                    retry_sleep_s=1.0,
                ).get("user_channel_id")
            if not isinstance(chan_100k, str) or not chan_100k:
                raise RuntimeError(f"Invalid user_channel_id from channel open: {chan_100k}")
            run_step(
                f"docker compose -f {compose_file} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 {miner_addr}"
                ,
                display_cmd=f"docker compose -f {compose_file_display} exec -T bitcoind bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=bitcoin generatetoaddress 6 {miner_addr}",
            )
            run_step("rgbldk channel ls")
            run_step("rgbldk wallet sync")
            run_step(f"rgbldk --connect {node_b} wallet sync")

            if asset_id:
                # Prove the "consignment over HTTP via context data" flow without requiring the user to manually
                # download/transfer a file. Node-b should have fetched the funding consignment automatically.
                deadline = time.time() + 60.0
                chan: Optional[dict[str, Any]] = None
                last_chans: Optional[object] = None
                while time.time() < deadline:
                    ensure_within_runtime("wait for channel ls entry (funding txid)")
                    rr = runner.run(
                        normalize_rgbldk_json_cmd("rgbldk channel ls"),
                        timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "channel ls (funding txid lookup)"),
                    )
                    chans = json.loads(rr.stdout)
                    last_chans = chans
                    if not isinstance(chans, list):
                        raise RuntimeError(f"Invalid channel ls JSON shape (expected array): {chans}")
                    found = next(
                        (c for c in chans if isinstance(c, dict) and c.get("user_channel_id") == chan_100k), None
                    )
                    if isinstance(found, dict) and isinstance(found.get("channel_point"), str) and ":" in found["channel_point"]:
                        chan = found
                        break
                    # Fallback: find the newly created channel by set difference.
                    diff = next(
                        (
                            c
                            for c in chans
                            if isinstance(c, dict)
                            and isinstance(c.get("channel_id"), str)
                            and c.get("channel_id") not in existing_channel_ids
                            and isinstance(c.get("channel_point"), str)
                            and ":" in c["channel_point"]
                        ),
                        None,
                    )
                    if isinstance(diff, dict):
                        chan = diff
                        break
                    if found is None and len(chans) == 1 and isinstance(chans[0], dict):
                        only = chans[0]
                        if isinstance(only.get("channel_point"), str) and ":" in only["channel_point"]:
                            chan = only
                            break
                    time.sleep(1.0)

                if not isinstance(chan, dict):
                    raise RuntimeError(
                        f"Failed to find channel by user_channel_id in channel ls: {chan_100k}. "
                        f"Last channel ls JSON: {last_chans!r}"
                    )
                channel_point = chan.get("channel_point")
                if not isinstance(channel_point, str) or ":" not in channel_point:
                    raise RuntimeError(f"Invalid channel_point in channel ls: {channel_point}")
                funding_txid = channel_point.split(":", 1)[0]

                md.paragraph(
                    "Because we passed `--rgb-context` during `channel open`, node-b can fetch the funding consignment "
                    "directly from node-a over HTTP (no manual download/upload step needed). You can verify that node-b "
                    "has cached the funding consignment by querying its consignment endpoint by the funding txid:\n"
                )
                run_step(
                    f"curl -sSf -o /dev/null -w '%{{http_code}}\\n' '{node_b}/api/v1/rgb/consignments/{funding_txid}?format=zip'",
                    display_cmd="curl -sSf -o /dev/null -w '%{http_code}\\n' 'http://<B_HOST>:8502/api/v1/rgb/consignments/<funding_txid>?format=zip'",
                )

            if contract_id:
                md.heading(3, "RGB on-chain transfer (L1, node-b → node-a)")
                md.paragraph(
                    "This demonstrates an RGB L1 transfer using the `rgb onchain` commands. Because the sender's consignment "
                    "cache is not shared with the receiver in a multi-node setup, we download the consignment from the sender "
                    "and upload it to the receiver via `--file`.\n\n"
                    "Note: a node-a → node-b RGB L1 transfer is shown earlier (before opening the RGB-enabled channel) to "
                    "make the channel funding transaction deterministic.\n"
                )

                # B -> A
                run_step("rgbldk ctx use node-a")
                inv_ba = run_step_json(
                    f"rgbldk rgb onchain invoice-create --contract-id {contract_id} --amount 7",
                    display_cmd="rgbldk rgb onchain invoice-create --contract-id <contract_id> --amount 7",
                ).get("invoice")
                if not isinstance(inv_ba, str) or not inv_ba:
                    raise RuntimeError(f"Invalid RGB on-chain invoice from node-a: {inv_ba}")
                inv_ba_payment_id = hashlib.sha256(inv_ba.encode("utf-8")).hexdigest()
                run_step("rgbldk ctx use node-b")
                run_step("rgbldk wallet sync")
                ensure_within_runtime("pre rgb onchain send (node-b) rgb sync")
                runner.run(
                    normalize_rgbldk_cmd("rgbldk rgb sync"),
                    retries=10,
                    retry_sleep_s=1.0,
                    timeout_s=clamp_timeout(DEFAULT_STEP_TIMEOUT_S, "pre-send rgb sync (node-b)"),
                )
                send_ba = run_step_json(
                    f"rgbldk rgb onchain send --invoice '{inv_ba}' --sats-for-fee-and-outputs 10000 --fee-rate-sats-per-vb 1.0",
                    display_cmd="rgbldk rgb onchain send --invoice '<invoice>' --sats-for-fee-and-outputs 10000 --fee-rate-sats-per-vb 1.0",
                    retries=5,
                    retry_sleep_s=2.0,
                    timeout_s=180.0,
                )
                consignment_key_ba = send_ba.get("consignment_key")
                if not isinstance(consignment_key_ba, str) or not consignment_key_ba:
                    raise RuntimeError(f"Invalid consignment_key from rgb onchain send: {send_ba}")
                cons_ba = demo_export_dir / "rgb-onchain-b-to-a.zip"
                run_step(
                    f"rgbldk rgb consignments download --key {consignment_key_ba} --out {cons_ba} --format zip",
                    display_cmd="rgbldk rgb consignments download --key <consignment_key> --out <b-to-a.zip> --format zip",
                )
                run_step("rgbldk ctx use node-a")
                run_step(
                    f"rgbldk rgb onchain receive --file {cons_ba} --format zip --payment-id {inv_ba_payment_id}",
                    display_cmd="rgbldk rgb onchain receive --file <b-to-a.zip> --format zip --payment-id <payment_id>",
                )
                bitcoind_cli(f"generatetoaddress 6 {miner_addr}")
                run_step("rgbldk wallet sync")
                run_step("rgbldk rgb sync", retries=10, retry_sleep_s=1.0)
                run_step(f"rgbldk rgb contracts balance {contract_id}")
                run_step("rgbldk ctx use node-a")

            md.heading(2, "8) Payments (BTC Lightning L2 + keysend + BOLT12 + RGB Lightning)")
            md.paragraph(
                "Create Bolt11 invoices on node-b and pay them from node-a. Use `pay wait` to block until the payment reaches "
                "a terminal state, and `pay get` to inspect details. Also demonstrate a spontaneous (keysend) payment.\n"
            )
            md.heading(3, "BTC Lightning transfer (L2, node-a → node-b)")

            run_step("rgbldk ctx use node-b")
            invoice1 = run_step_json("rgbldk pay invoice create --desc demo --amount-msat 10000").get(
                "invoice"
            )
            if not isinstance(invoice1, str) or not invoice1:
                raise RuntimeError(f"Invalid invoice from invoice create: {invoice1}")
            run_step("rgbldk ctx use node-a")

            # Ensure the channel is usable before attempting payments (do not include in markdown).
            wait_for_usable_channel(node_a, timeout_s=90.0)

            pay1 = run_step_json(
                f"rgbldk pay invoice pay --invoice {invoice1}",
                display_cmd="rgbldk --color never --output json --pretty pay invoice pay --invoice <invoice>",
                retries=30,
                retry_sleep_s=1.0,
            ).get("payment_id")
            if not isinstance(pay1, str) or not pay1:
                raise RuntimeError(f"Invalid payment_id from invoice pay: {pay1}")
            run_step(f"rgbldk pay wait {pay1} --timeout-secs 60")
            run_step(f"rgbldk pay get {pay1}")

            run_step("rgbldk ctx use node-b")
            invoice2 = run_step_json("rgbldk pay invoice create --desc demo-var").get("invoice")
            if not isinstance(invoice2, str) or not invoice2:
                raise RuntimeError(f"Invalid invoice from invoice create: {invoice2}")
            run_step("rgbldk ctx use node-a")
            pay2 = run_step_json(
                f"rgbldk pay invoice pay --invoice {invoice2} --amount-msat 11000",
                display_cmd="rgbldk --color never --output json --pretty pay invoice pay --invoice <invoice> --amount-msat 11000",
                retries=30,
                retry_sleep_s=1.0,
            ).get("payment_id")
            if not isinstance(pay2, str) or not pay2:
                raise RuntimeError(f"Invalid payment_id from invoice pay: {pay2}")
            run_step(f"rgbldk pay wait {pay2} --timeout-secs 60")
            run_step(f"rgbldk pay get {pay2}")

            md.heading(3, "BTC Lightning transfer (L2, node-b → node-a)")
            run_step("rgbldk ctx use node-a")
            invoice3 = run_step_json("rgbldk pay invoice create --desc demo-back --amount-msat 12000").get(
                "invoice"
            )
            if not isinstance(invoice3, str) or not invoice3:
                raise RuntimeError(f"Invalid invoice from invoice create: {invoice3}")
            run_step("rgbldk ctx use node-b")
            pay3_b_to_a = run_step_json(
                f"rgbldk pay invoice pay --invoice {invoice3}",
                display_cmd="rgbldk --color never --output json --pretty pay invoice pay --invoice <invoice>",
                retries=30,
                retry_sleep_s=1.0,
            ).get("payment_id")
            if not isinstance(pay3_b_to_a, str) or not pay3_b_to_a:
                raise RuntimeError(f"Invalid payment_id from invoice pay: {pay3_b_to_a}")
            run_step(f"rgbldk pay wait {pay3_b_to_a} --timeout-secs 60")
            run_step(f"rgbldk pay get {pay3_b_to_a}")
            run_step("rgbldk ctx use node-a")

            md.heading(3, "BOLT12 offer")
            run_step("rgbldk ctx use node-b")
            offer1 = run_step_json("rgbldk pay offer create --desc offer-demo --amount-msat 5555").get("offer")
            if not isinstance(offer1, str) or not offer1:
                raise RuntimeError(f"Invalid offer from offer create: {offer1}")
            run_step(
                f"rgbldk pay offer decode {offer1}",
                display_cmd="rgbldk pay offer decode <offer>",
            )
            run_step("rgbldk ctx use node-a")
            offer_pay = run_step_json(
                f"rgbldk pay offer pay --offer {offer1}",
                display_cmd="rgbldk --color never --output json --pretty pay offer pay --offer <offer>",
                retries=30,
                retry_sleep_s=1.0,
            ).get("payment_id")
            if not isinstance(offer_pay, str) or not offer_pay:
                raise RuntimeError(f"Invalid payment_id from offer pay: {offer_pay}")
            run_step(f"rgbldk pay wait {offer_pay} --timeout-secs 60")
            run_step(f"rgbldk pay get {offer_pay}")

            md.heading(3, "BOLT12 refund (initiate/request-payment) + abandon")
            run_step("rgbldk ctx use node-a")
            refund1 = run_step_json("rgbldk pay refund initiate --amount-msat 4321 --payer-note refund-demo")
            refund1_str = refund1.get("refund")
            refund1_pid = refund1.get("payment_id")
            if not isinstance(refund1_str, str) or not refund1_str:
                raise RuntimeError(f"Invalid refund from refund initiate: {refund1}")
            if not isinstance(refund1_pid, str) or not refund1_pid:
                raise RuntimeError(f"Invalid payment_id from refund initiate: {refund1}")
            run_step(
                f"rgbldk pay refund decode {refund1_str}",
                display_cmd="rgbldk pay refund decode <refund>",
            )
            run_step("rgbldk ctx use node-b")
            run_step(
                f"rgbldk pay refund request-payment {refund1_str}",
                display_cmd="rgbldk pay refund request-payment <refund>",
            )
            run_step("rgbldk ctx use node-a")
            run_step(f"rgbldk pay wait {refund1_pid} --timeout-secs 60")
            run_step(f"rgbldk pay get {refund1_pid}")

            refund2 = run_step_json("rgbldk pay refund initiate --amount-msat 1111 --payer-note refund-abandon-demo")
            refund2_pid = refund2.get("payment_id")
            if not isinstance(refund2_pid, str) or not refund2_pid:
                raise RuntimeError(f"Invalid payment_id from refund initiate: {refund2}")
            run_step(
                f"rgbldk pay abandon {refund2_pid}",
                display_cmd="rgbldk pay abandon <payment_id>",
            )

            md.heading(3, "Payments list")
            run_step("rgbldk pay ls")

            if asset_id:
                md.heading(3, "RGB Lightning transfer (L2, node-a → node-b)")
                md.paragraph(
                    "Create an RGB LN invoice on node-b and pay it from node-a over the RGB-enabled channel.\n"
                )
                run_step("rgbldk ctx use node-b")
                rgb_inv = run_step_json(
                    f"rgbldk rgb ln invoice create --asset-id {asset_id} --asset-amount 5 --desc \"rgb ln demo\" --btc-carrier-amount-msat 5000000",
                    display_cmd="rgbldk --color never --output json --pretty rgb ln invoice create --asset-id <asset_id_hex> --asset-amount 5 --desc \"rgb ln demo\" --btc-carrier-amount-msat 5000000",
                    retries=10,
                    retry_sleep_s=1.0,
                ).get("invoice")
                if not isinstance(rgb_inv, str) or not rgb_inv:
                    raise RuntimeError(f"Invalid RGB LN invoice from invoice create: {rgb_inv}")
                run_step(
                    f"rgbldk rgb ln invoice decode {rgb_inv}",
                    display_cmd="rgbldk rgb ln invoice decode <invoice>",
                )
                run_step("rgbldk ctx use node-a")
                rgb_pay = run_step_json(
                    f"rgbldk rgb ln pay --invoice {rgb_inv}",
                    display_cmd="rgbldk --color never --output json --pretty rgb ln pay --invoice <invoice>",
                    retries=30,
                    retry_sleep_s=1.0,
                ).get("payment_id")
                if not isinstance(rgb_pay, str) or not rgb_pay:
                    raise RuntimeError(f"Invalid payment_id from RGB LN pay: {rgb_pay}")
                run_step(f"rgbldk pay wait {rgb_pay} --timeout-secs 60")
                run_step(f"rgbldk pay get {rgb_pay}")

                md.heading(4, "RGB Lightning transfer (L2, node-b → node-a)")
                run_step("rgbldk ctx use node-a")
                rgb_inv2 = run_step_json(
                    f"rgbldk rgb ln invoice create --asset-id {asset_id} --asset-amount 3 --desc \"rgb ln demo back\" --btc-carrier-amount-msat 4000000",
                    display_cmd="rgbldk --color never --output json --pretty rgb ln invoice create --asset-id <asset_id_hex> --asset-amount 3 --desc \"rgb ln demo back\" --btc-carrier-amount-msat 4000000",
                    retries=10,
                    retry_sleep_s=1.0,
                ).get("invoice")
                if not isinstance(rgb_inv2, str) or not rgb_inv2:
                    raise RuntimeError(f"Invalid RGB LN invoice from invoice create: {rgb_inv2}")
                run_step(
                    f"rgbldk rgb ln invoice decode {rgb_inv2}",
                    display_cmd="rgbldk rgb ln invoice decode <invoice>",
                )
                run_step("rgbldk ctx use node-b")
                rgb_pay2 = run_step_json(
                    f"rgbldk rgb ln pay --invoice {rgb_inv2}",
                    display_cmd="rgbldk --color never --output json --pretty rgb ln pay --invoice <invoice>",
                    retries=30,
                    retry_sleep_s=1.0,
                ).get("payment_id")
                if not isinstance(rgb_pay2, str) or not rgb_pay2:
                    raise RuntimeError(f"Invalid payment_id from RGB LN pay: {rgb_pay2}")
                run_step(f"rgbldk pay wait {rgb_pay2} --timeout-secs 60")
                run_step(f"rgbldk pay get {rgb_pay2}")
                run_step("rgbldk ctx use node-a")
                md.paragraph(
                    "To confirm the asset movement, list channels on both nodes again. For RGB-enabled channels, "
                    "`channel ls` includes extra columns (`RGB Asset`, `RGB Local`, `RGB Remote`) showing the current "
                    "in-channel balances.\n"
                )
                run_step("rgbldk channel ls")
                run_step("rgbldk ctx use node-b")
                run_step("rgbldk channel ls")
                run_step("rgbldk ctx use node-a")

            md.heading(3, "Keysend")
            pay_keysend_a_to_b = run_step_json(
                f"rgbldk pay keysend send --node-id {node_id_b} --amount-msat 1234 --tlv 70001:02",
                display_cmd="rgbldk --color never --output json --pretty pay keysend send --node-id <node_id_b> --amount-msat 1234 --tlv 70001:02",
                retries=30,
                retry_sleep_s=1.0,
            ).get("payment_id")
            if not isinstance(pay_keysend_a_to_b, str) or not pay_keysend_a_to_b:
                raise RuntimeError(f"Invalid payment_id from keysend: {pay_keysend_a_to_b}")
            run_step(f"rgbldk pay wait {pay_keysend_a_to_b} --timeout-secs 60")
            run_step(f"rgbldk pay get {pay_keysend_a_to_b}")

            md.heading(4, "Keysend (reverse direction)")
            if not node_id_a:
                raise RuntimeError("missing node_id_a (expected it to be set in the node basics section)")
            run_step("rgbldk ctx use node-b")
            pay_keysend_b_to_a = run_step_json(
                f"rgbldk pay keysend send --node-id {node_id_a} --amount-msat 2345 --tlv 70002:03",
                display_cmd="rgbldk --color never --output json --pretty pay keysend send --node-id <node_id_a> --amount-msat 2345 --tlv 70002:03",
                retries=30,
                retry_sleep_s=1.0,
            ).get("payment_id")
            if not isinstance(pay_keysend_b_to_a, str) or not pay_keysend_b_to_a:
                raise RuntimeError(f"Invalid payment_id from keysend: {pay_keysend_b_to_a}")
            run_step(f"rgbldk pay wait {pay_keysend_b_to_a} --timeout-secs 60")
            run_step(f"rgbldk pay get {pay_keysend_b_to_a}")
            run_step("rgbldk ctx use node-a")

            md.heading(2, "9) Events (next/handled)")
            md.paragraph(
                "Demonstrate the event queue API: fetch the next event (`events next`) and acknowledge it (`events handled`) "
                "so the daemon can advance the queue.\n"
            )
            md.heading(3, "Queue")

            md.paragraph(
                "Tip: `events watch` blocks until it receives events. This example uses a short timeout so the generator "
                "does not hang if the queue is empty.\n"
            )
            run_step(
                f"rgbldk --connect {node_a} events watch --count 1",
                timeout_s=5,
                check=False,
            )
            run_step(
                f"rgbldk --connect {node_b} events watch --count 1",
                timeout_s=5,
                check=False,
            )
            run_step(f"rgbldk --connect {node_a} events next")
            run_step(f"rgbldk --connect {node_a} events handled")
            run_step(f"rgbldk --connect {node_b} events next")
            run_step(f"rgbldk --connect {node_b} events handled")

            md.heading(2, "10) BTC on-chain settlement (L1, node-a → node-b via channel push+close) + Channel force-close")
            md.paragraph(
                "Demonstrate graceful close vs force-close. Force-close is destructive and requires `--yes` for non-interactive safety.\n"
            )
            md.heading(3, "BTC on-chain settlement (L1, node-a → node-b via channel push+close)")
            run_step(
                f"rgbldk channel close --user-channel-id {chan_100k} --counterparty-node-id {node_id_b}",
                display_cmd="rgbldk channel close --user-channel-id <user_channel_id> --counterparty-node-id <node_id_b>",
            )
            bitcoind_cli(f"generatetoaddress 6 {miner_addr}")
            run_step("rgbldk wallet sync")
            run_step(f"rgbldk --connect {node_b} wallet sync")
            run_step("rgbldk wallet balance")
            run_step(f"rgbldk --connect {node_b} wallet balance")

            md.heading(3, "Force-close (example)")
            chan_force = run_step_json(
                f"rgbldk channel open --node-id {node_id_b} --addr {node_b_p2p} --amount-sats 50000 --push-msat 1000000 --private",
                display_cmd="rgbldk channel open --node-id <node_id_b> --addr <node_b_p2p> --amount-sats 50000 --push-msat 1000000 --private",
                retries=30,
                retry_sleep_s=1.0,
            ).get("user_channel_id")
            if not isinstance(chan_force, str) or not chan_force:
                raise RuntimeError(f"Invalid user_channel_id from channel open: {chan_force}")
            bitcoind_cli(f"generatetoaddress 6 {miner_addr}")
            wait_for_usable_channel(node_a, timeout_s=90.0)
            run_step(
                f"rgbldk --yes channel force-close --user-channel-id {chan_force} --counterparty-node-id {node_id_b}",
                display_cmd="rgbldk --yes channel force-close --user-channel-id <user_channel_id> --counterparty-node-id <node_id_b>",
            )

            md.heading(2, "11) Cleanup")
            md.paragraph("Tear down the docker-compose stack and remove volumes.\n")
            md.heading(3, "Disconnect peers")
            if not node_id_b:
                raise RuntimeError("missing node_id_b (expected it to be set in the node basics section)")
            run_step(
                f"rgbldk peer disconnect {node_id_b}",
                display_cmd="rgbldk peer disconnect <node_id_b>",
            )
            run_step("rgbldk peer ls")
            if node_id_a:
                run_step("rgbldk ctx use node-b")
                run_step(
                    f"rgbldk peer disconnect {node_id_a}",
                    display_cmd="rgbldk peer disconnect <node_id_a>",
                )
                run_step("rgbldk peer ls")
                run_step("rgbldk ctx use node-a")

            docker_down_cmd = f"docker compose -f {compose_file} down -v"
            docker_down_cmd_display = f"docker compose -f {compose_file_display} down -v"
            md.fence("bash", f"$ {docker_down_cmd_display}\n")
            if not args.no_cleanup:
                _progress(docker_down_cmd_display)
                runner.run(docker_down_cmd, timeout_s=clamp_timeout(300.0, "docker compose down"))

            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text(md.render(), encoding="utf-8")
        finally:
            # Stop local daemons if we started them.
            for proc in (rgbldkd_proc_a, rgbldkd_proc_b):
                if not proc:
                    continue
                _terminate_process(proc, timeout_s=15.0)

            if proxy_server is not None:
                proxy_server.shutdown()
                proxy_server.server_close()
                if proxy_thread is not None:
                    proxy_thread.join(timeout=5)

            if dummy_control_http_server is not None:
                dummy_control_http_server.shutdown()
                dummy_control_http_server.server_close()
                if dummy_control_http_thread is not None:
                    dummy_control_http_thread.join(timeout=5)

            if dummy_control_socket_server is not None:
                dummy_control_socket_server.shutdown()
                dummy_control_socket_server.server_close()
                if dummy_control_socket_thread is not None:
                    dummy_control_socket_thread.join(timeout=5)
            if dummy_control_socket_path is not None and dummy_control_socket_path.exists():
                dummy_control_socket_path.unlink()

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
