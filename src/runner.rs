//! Running a problem: compile the attempt with `rustc --test`, run the binary
//! (with a timeout, measuring its wall-clock time and peak memory) and parse
//! libtest's output. Lint with `clippy-driver`.
//!
//! The tests run one at a time (`--test-threads=1`), so the measurements don't
//! depend on the number of cores: the runtime is the sum of the tests', the
//! peak memory the largest test's. And only one test binary runs at a time in
//! the process, so a measurement of the reference solution
//! ([`crate::perf`]) never competes with one of your code.
//!
//! A working file is only your code. What's compiled is a copy in the build
//! dir with the problem's `tests.rs` attached at the end ([`attach_tests`]);
//! `--remap-path-prefix` makes diagnostics and panics name your file, and
//! since your code comes first its line numbers are unchanged.
//!
//! One build at a time per [`Env`] user, and every child process is waited
//! for before the call returns, killed first if it times out or is
//! [cancelled](Cancel): Windows can't replace a program that's still running.

use std::collections::VecDeque;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::toolchain;

/// How long a test run may take before it's killed.
pub const TEST_TIMEOUT: Duration = Duration::from_secs(30);

pub const EDITION: &str = "2024";

/// What the template's bodies contain until you replace them.
pub const SENTINEL: &str = "todo!()";

/// What a test that hits [`SENTINEL`] panics with.
const TODO_PANIC: &str = "not yet implemented";

/// How template and solution end, so they compile where they are; stripped
/// to make a working file.
const TESTS_MOD: [&str; 3] = ["#[cfg(test)]", "#[path = \"tests.rs\"]", "mod tests;"];

/// Where a run happens.
#[derive(Clone, Debug)]
pub struct Env {
    /// Scratch artifacts: `<cache>/build`.
    pub build_dir: PathBuf,
    pub timeout: Duration,
    /// Stops whatever this env is running.
    pub cancel: Cancel,
}

/// Shared by the runs it's given to: once cancelled, a running compiler or
/// test binary is killed and waited for, and the run returns an
/// [`io::ErrorKind::Interrupted`] error. So is any run started after.
#[derive(Clone, Debug, Default)]
pub struct Cancel(Arc<AtomicBool>);

impl Cancel {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// Code to compile: `source`, which is (or will be) the contents of `file`,
/// with the problem's `tests` attached.
pub struct Code<'a> {
    /// Named in diagnostics and panics.
    pub file: &'a Path,
    pub source: &'a str,
    pub tests: &'a Path,
}

pub enum Outcome {
    Passed(String),
    Failed(String),
    /// Every test hit `todo!()`.
    NotStarted,
    CompileError(String),
    Timeout(String),
}

impl Outcome {
    pub fn passed(&self) -> bool {
        matches!(self, Outcome::Passed(_))
    }

    pub fn output(&self) -> &str {
        match self {
            Outcome::Passed(s) | Outcome::Failed(s) | Outcome::CompileError(s) | Outcome::Timeout(s) => s,
            Outcome::NotStarted => "",
        }
    }
}

pub struct RunResult {
    pub outcome: Outcome,
    /// Wall-clock time of the test process (not compilation).
    pub runtime: Duration,
    /// Its peak memory (resident set, or working set on Windows) in bytes,
    /// where the platform reports it.
    pub peak_memory: Option<u64>,
    pub passed: u32,
    pub failed: u32,
}

impl RunResult {
    fn of(outcome: Outcome) -> RunResult {
        RunResult { outcome, runtime: Duration::ZERO, peak_memory: None, passed: 0, failed: 0 }
    }
}

pub struct Lint {
    pub clean: bool,
    pub output: String,
}

/// Problem ids contain `/`; build artifacts sit flat in the build dir.
fn artifact(name: &str) -> String {
    name.replace('/', "__")
}

/// The test binary for artifact `name`: `.exe` on Windows. Built, run and
/// replaced under this one name.
fn binary(build_dir: &Path, name: &str) -> PathBuf {
    build_dir.join(format!("{}{}", artifact(name), std::env::consts::EXE_SUFFIX))
}

fn color_flag(color: bool) -> &'static str {
    if color { "always" } else { "never" }
}

fn crate_name(name: &str) -> String {
    format!("g_{}", name.replace(|c: char| !c.is_ascii_alphanumeric(), "_"))
}

impl Env {
    /// Write `code`, tests attached, to the build dir as `<name>.rs`; and the
    /// `--remap-path-prefix` argument that makes the compiler call it `code.file`.
    fn generate(&self, name: &str, code: &Code) -> io::Result<(PathBuf, OsString)> {
        fs::create_dir_all(&self.build_dir)?;
        let generated = self.build_dir.join(format!("{}.rs", artifact(name)));
        fs::write(&generated, attach_tests(code.source, code.tests))?;
        let mut remap = generated.clone().into_os_string();
        remap.push("=");
        remap.push(code.file);
        Ok((generated, remap))
    }

    /// Compile `code` as a test crate (overflow checks on, like `cargo test`)
    /// and run it. `name` is a unique artifact name, e.g. `three_sum-solution`.
    pub fn run_tests(&self, name: &str, code: &Code, color: bool) -> io::Result<RunResult> {
        match self.compile(name, code, color)? {
            Ok(bin) => self.test(&bin, color),
            Err(failed) => Ok(failed),
        }
    }

    /// Compile `code` once and run its tests `times` times: the results of
    /// every run, or of the first that didn't pass, or the compile error.
    pub fn run_tests_repeatedly(&self, name: &str, code: &Code, times: usize) -> io::Result<Vec<RunResult>> {
        let bin = match self.compile(name, code, false)? {
            Ok(bin) => bin,
            Err(failed) => return Ok(vec![failed]),
        };
        let mut results = Vec::with_capacity(times);
        for _ in 0..times {
            let result = self.test(&bin, false)?;
            let passed = result.outcome.passed();
            results.push(result);
            if !passed {
                break;
            }
        }
        Ok(results)
    }

    /// The test binary for `code`, or the result of a compile that failed.
    fn compile(&self, name: &str, code: &Code, color: bool) -> io::Result<Result<PathBuf, RunResult>> {
        let (generated, remap) = self.generate(name, code)?;
        let bin = binary(&self.build_dir, name);
        remove_stale(&bin)?;
        let mut rustc = Command::new("rustc");
        rustc
            .args(["--edition", EDITION, "--test", "-A", "warnings"])
            .args(["-C", "overflow-checks=on", "--color", color_flag(color)])
            .args(["--crate-name", &crate_name(name)])
            .arg("--remap-path-prefix")
            .arg(remap)
            .arg("-o")
            .arg(&bin)
            .arg(&generated);
        let compile = run(&mut rustc, None, &self.cancel)?;
        match compile.ended {
            Ended::Exited(true) => Ok(Ok(bin)),
            Ended::Cancelled => Err(cancelled()),
            Ended::Exited(false) | Ended::TimedOut => {
                Ok(Err(RunResult::of(Outcome::CompileError(explain_compile_error(compile.stderr)))))
            }
        }
    }

    /// Run the test binary `bin`, one test at a time, once no other test
    /// binary is running.
    fn test(&self, bin: &Path, color: bool) -> io::Result<RunResult> {
        static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
        let mut cmd = Command::new(bin);
        cmd.args(["--color", color_flag(color), "--test-threads=1"]);
        let ran = {
            let _turn = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
            // A program's first run after it's linked can take 100 ms longer
            // while macOS (or a virus scanner) checks it: listing the tests
            // takes that, so it doesn't count toward the runtime.
            run(Command::new(bin).arg("--list"), Some(self.timeout), &self.cancel)?;
            run(&mut cmd, Some(self.timeout), &self.cancel)?
        };
        let output = ran.output();
        let (passed, failed) = parse_counts(&output);
        let outcome = match ran.ended {
            Ended::Cancelled => return Err(cancelled()),
            Ended::TimedOut => Outcome::Timeout(timeout_message(&output, self.timeout)),
            Ended::Exited(success) => verdict(success, output, ran.cut, passed, failed),
        };
        Ok(RunResult { outcome, runtime: ran.runtime, peak_memory: ran.peak_memory, passed, failed })
    }

    /// Run clippy over `code` (including its tests). With `deny`, any warning
    /// fails. A clippy that isn't installed is an [`io::ErrorKind::NotFound`]
    /// error saying how to install it ([`toolchain::CLIPPY_MISSING`]).
    pub fn lint(&self, name: &str, code: &Code, deny: bool, color: bool) -> io::Result<Lint> {
        let (generated, remap) = self.generate(name, code)?;
        let mut cmd = Command::new("clippy-driver");
        // Set by `cargo run`: clippy would hold your code to gauntlet's own MSRV.
        cmd.env_remove("CARGO_PKG_RUST_VERSION");
        cmd.args(["--edition", EDITION, "--test", "--emit=metadata"])
            .args(["-C", "overflow-checks=on", "--color", color_flag(color)])
            .args(["--crate-name", &crate_name(name)])
            .arg("--remap-path-prefix")
            .arg(remap)
            .arg("-o")
            .arg(self.build_dir.join(format!("{}.rmeta", artifact(name))))
            .arg(&generated);
        if deny {
            cmd.args(["-D", "warnings"]);
        }
        let missing = || io::Error::new(io::ErrorKind::NotFound, toolchain::CLIPPY_MISSING);
        let ran = match run(&mut cmd, None, &self.cancel) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Err(missing()),
            ran => ran?,
        };
        let success = match ran.ended {
            Ended::Exited(success) => success,
            Ended::Cancelled | Ended::TimedOut => return Err(cancelled()),
        };
        // rustup's stand-in for a component that isn't installed.
        if !success && toolchain::clippy_not_installed(&ran.stderr) {
            return Err(missing());
        }
        let output = ran.stderr;
        Ok(Lint { clean: success && !output.contains("warning"), output })
    }
}

/// What a finished test run amounts to. Passing takes at least one passed
/// test and none failed: a binary that exits early (`std::process::exit(0)`)
/// passes nothing. Not started means nothing passed and every failure was a
/// `todo!()`; otherwise the real failures are shown, under a count of the
/// tests still hitting `todo!()`. Output that was `cut` (see [`Capped`]) is
/// shown as it is: a failure's report may have lost its heading.
fn verdict(success: bool, output: String, cut: bool, passed: u32, failed: u32) -> Outcome {
    if success && passed > 0 && failed == 0 {
        return Outcome::Passed(output);
    }
    if success {
        let why = if output.contains("test result:") {
            "No tests ran."
        } else {
            "The tests stopped before reporting a result: does your code call std::process::exit?"
        };
        return Outcome::Failed(format!("{output}\n{why}\n"));
    }
    if cut {
        return Outcome::Failed(output);
    }
    let failures = failures(&output);
    let todo: Vec<&str> = failures.iter().filter(|f| f.todo).map(|f| f.name.as_str()).collect();
    if passed == 0 && !failures.is_empty() && todo.len() == failures.len() {
        return Outcome::NotStarted;
    }
    if todo.is_empty() {
        return Outcome::Failed(output);
    }
    // The real failures, without the todo!() noise around them.
    let names = match todo.len() {
        n if n > 4 => format!("{} and {} more", todo[..3].join(", "), n - 3),
        _ => todo.join(", "),
    };
    let s = if todo.len() == 1 { "" } else { "s" };
    let mut shown = format!("{} test{s} still hit {SENTINEL}: {names}\n\nThe other failures:\n\n", todo.len());
    for f in failures.iter().filter(|f| !f.todo) {
        shown.push_str(&f.report);
    }
    if let Some(summary) = output.lines().rev().find(|l| l.contains("test result:")) {
        shown.push_str(summary);
        shown.push('\n');
    }
    Outcome::Failed(shown)
}

/// A failed test in libtest's output.
struct Failure {
    name: String,
    /// Its `---- <name> stdout ----` section: what it printed and how it panicked.
    report: String,
    /// It panicked with `todo!()`'s message.
    todo: bool,
}

fn failures(output: &str) -> Vec<Failure> {
    let mut found = Vec::new();
    let mut current: Option<Failure> = None;
    for line in output.lines() {
        if let Some(name) = line.strip_prefix("---- ").and_then(|l| l.strip_suffix(" stdout ----")) {
            let next = Failure { name: name.to_string(), report: String::new(), todo: false };
            found.extend(current.replace(next));
        } else if line == "failures:" || line.contains("test result:") {
            found.extend(current.take());
            continue;
        }
        if let Some(f) = current.as_mut() {
            f.todo |= line.contains(TODO_PANIC);
            f.report.push_str(line);
            f.report.push('\n');
        }
    }
    found.extend(current);
    found
}

/// Compiler output, with a note when the cause is something only gauntlet
/// can explain, or a missing linker.
fn explain_compile_error(mut stderr: String) -> String {
    if stderr.contains("E0428") && stderr.contains("`tests`") {
        stderr.push_str(
            "\nYour file declares its own `mod tests`, but gauntlet attaches the problem's tests as\n\
             `mod tests` when it compiles your file. Rename your module, or remove it.\n",
        );
    }
    if let Some(hint) = toolchain::linker_hint(&stderr, std::env::consts::OS) {
        stderr.push_str(&format!("\n{hint}\n"));
    }
    stderr
}

/// Pull `N passed; M failed` out of libtest's summary line (colour codes and all).
fn parse_counts(output: &str) -> (u32, u32) {
    let Some(line) = output.lines().rev().find(|l| l.contains("test result:")) else {
        return (0, 0);
    };
    let number_before = |word: &str| -> u32 {
        line.find(word).and_then(|i| line[..i].split_whitespace().last()).and_then(|n| n.parse().ok()).unwrap_or(0)
    };
    (number_before(" passed;"), number_before(" failed;"))
}

/// The reference solution without its test-module plumbing.
pub fn display_solution(src: &str) -> String {
    match src.find("#[cfg(test)]") {
        Some(i) => src[..i].trim_end().to_string(),
        None => src.trim_end().to_string(),
    }
}

/// A template or solution without the test module it ends with, if it
/// does: the code a working file holds.
pub fn without_tests_mod(src: &str) -> String {
    let lines: Vec<&str> = src.trim_end().lines().collect();
    let n = lines.len();
    if n < TESTS_MOD.len() || lines[n - TESTS_MOD.len()..].iter().map(|l| l.trim()).ne(TESTS_MOD) {
        return src.to_string();
    }
    let mut code = lines[..n - TESTS_MOD.len()].join("\n").trim_end().to_string();
    code.push('\n');
    code
}

/// `source` with `tests` attached as its test module: what's compiled.
/// Nothing is added before or within `source`, so its line numbers hold.
pub fn attach_tests(source: &str, tests: &Path) -> String {
    let mut text = source.to_string();
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    let tests = string_literal(&tests.to_string_lossy());
    text.push_str(&format!(
        "\n// Added by gauntlet: the problem's tests.\n#[cfg(test)]\n#[path = {tests}]\nmod tests;\n"
    ));
    text
}

/// `s` as a Rust string literal. Windows paths are full of backslashes,
/// which are escapes in a literal: `C:\Users\…` has to be written `"C:\\Users\\…"`.
fn string_literal(s: &str) -> String {
    let mut lit = String::with_capacity(s.len() + 2);
    lit.push('"');
    for c in s.chars() {
        match c {
            '\\' => lit.push_str("\\\\"),
            '"' => lit.push_str("\\\""),
            '\n' => lit.push_str("\\n"),
            '\r' => lit.push_str("\\r"),
            '\t' => lit.push_str("\\t"),
            c if c.is_control() => lit.push_str(&format!("\\u{{{:x}}}", u32::from(c))),
            c => lit.push(c),
        }
    }
    lit.push('"');
    lit
}

/// `rustc --print sysroot`, asked once per process. `None` (and logged) if
/// that fails.
fn sysroot() -> Option<&'static str> {
    static SYSROOT: OnceLock<Option<String>> = OnceLock::new();
    SYSROOT
        .get_or_init(|| {
            let out = Command::new("rustc").args(["--print", "sysroot"]).output();
            let sysroot = match &out {
                Ok(o) if o.status.success() => Some(String::from_utf8_lossy(&o.stdout).trim().to_string()),
                _ => None,
            };
            let sysroot = sysroot.filter(|s| !s.is_empty());
            if sysroot.is_none() {
                tracing::warn!(?out, "`rustc --print sysroot` failed; rust-project.json will have no sysroot");
            }
            sysroot
        })
        .as_deref()
}

/// The contents of a `rust-project.json` that makes each of `files` (paths
/// relative to `data`, where the json goes) a standalone test crate. Written
/// with `/` on every platform: rust-analyzer reads it as well on Windows.
///
/// It also gives rust-analyzer a `flycheck` runnable: without one it runs no
/// check on save for a project that isn't Cargo's. That runs `clippy-driver`
/// as [`Env::lint`] does, on the saved file's crate (its `{label}`, which is
/// its path), so the editor shows the warnings a review would. Its output
/// goes in `build`.
pub fn rust_project_json(files: &[PathBuf], data: &Path, build: &Path) -> String {
    let crates: Vec<serde_json::Value> = files
        .iter()
        .map(|f| {
            let root: Vec<String> = f.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
            let root = root.join("/");
            serde_json::json!({
                "root_module": root,
                "edition": EDITION,
                "deps": [],
                "cfg": ["test"],
                "build": { "label": root, "build_file": root, "target_kind": "test" },
            })
        })
        .collect();
    let rmeta = build.join("editor_check.rmeta");
    let flycheck = serde_json::json!({
        "kind": "flycheck",
        "program": "clippy-driver",
        "args": [
            "--edition", EDITION, "--test", "--emit=metadata", "--error-format=json",
            "-C", "overflow-checks=on", "--crate-name", "editor_check",
            "-o", rmeta.to_string_lossy(), "{label}",
        ],
        "cwd": data.to_string_lossy(),
    });
    let mut json = serde_json::json!({ "crates": crates, "runnables": [flycheck] });
    if let Some(sysroot) = sysroot() {
        json["sysroot"] = sysroot.into();
        // rust-analyzer needs the std sources spelled out: given only
        // `sysroot` it loads no std, so `todo!`, `Option` and the rest go
        // unresolved and completion is empty. They come with rust-src.
        let src = Path::new(sysroot).join("lib").join("rustlib").join("src").join("rust").join("library");
        if src.is_dir() {
            json["sysroot_src"] = src.to_string_lossy().into_owned().into();
        } else {
            tracing::warn!(?src, "no std sources; `rustup component add rust-src` for rust-analyzer");
        }
    }
    let mut text = serde_json::to_string_pretty(&json).expect("JSON values serialize");
    text.push('\n');
    text
}

// ---- Processes --------------------------------------------------------------

/// How a child process ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ended {
    /// On its own: whether it succeeded.
    Exited(bool),
    /// Killed after the timeout.
    TimedOut,
    /// Killed because the run was cancelled.
    Cancelled,
}

struct Ran {
    ended: Ended,
    stdout: String,
    stderr: String,
    /// Output was dropped from the middle of either stream.
    cut: bool,
    /// Wall-clock time from spawn to exit.
    runtime: Duration,
    /// Peak memory in bytes, if it exited on its own and the platform says.
    peak_memory: Option<u64>,
}

impl Ran {
    /// stdout followed by stderr.
    fn output(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}

fn cancelled() -> io::Error {
    io::Error::new(io::ErrorKind::Interrupted, "cancelled")
}

/// How often a running child is checked on.
const POLL: Duration = Duration::from_millis(2);

/// How long to keep reading a child's output after it has exited. It's
/// normally all there at once; this is for a program that left a process of
/// its own running with the pipes open, which would otherwise hang the read.
const DRAIN_GRACE: Duration = Duration::from_millis(500);

/// Spawn `cmd` with stdout and stderr captured and wait for it, killing it
/// after `timeout` or once `cancel` is cancelled. Either way it has exited,
/// and been reaped, when this returns.
fn run(cmd: &mut Command, timeout: Option<Duration>, cancel: &Cancel) -> io::Result<Ran> {
    if cancel.is_cancelled() {
        return Err(cancelled());
    }
    let mut child = cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let stdout = Drain::start(child.stdout.take().expect("piped"));
    let stderr = Drain::start(child.stderr.take().expect("piped"));
    let start = Instant::now();
    let waited = wait(&mut child, timeout, cancel);
    let runtime = start.elapsed();
    let deadline = Instant::now() + DRAIN_GRACE;
    let ((stdout, cut_out), (stderr, cut_err)) = (stdout.finish(deadline), stderr.finish(deadline));
    let (ended, peak_memory) = waited?;
    Ok(Ran { ended, stdout, stderr, cut: cut_out || cut_err, runtime, peak_memory })
}

/// Wait for `child` to exit, killing it after `timeout` or once `cancel` is
/// cancelled. If it exits on its own, its peak memory too where known.
fn wait(child: &mut Child, timeout: Option<Duration>, cancel: &Cancel) -> io::Result<(Ended, Option<u64>)> {
    let start = Instant::now();
    loop {
        match try_reap(child) {
            Ok(Some((success, peak))) => return Ok((Ended::Exited(success), peak)),
            Ok(None) => {}
            Err(e) => {
                kill(child);
                return Err(e);
            }
        }
        let ended = if cancel.is_cancelled() {
            Ended::Cancelled
        } else if timeout.is_some_and(|t| start.elapsed() > t) {
            Ended::TimedOut
        } else {
            thread::sleep(POLL);
            continue;
        };
        kill(child);
        return Ok((ended, None));
    }
}

/// If `child` has exited: whether it succeeded, and its peak resident set
/// from `wait4`, which reaps it. `child` must not be waited for or killed
/// after that: std doesn't know it's gone, and its pid may be reused.
#[cfg(unix)]
fn try_reap(child: &mut Child) -> io::Result<Option<(bool, Option<u64>)>> {
    use std::os::unix::process::ExitStatusExt;
    let pid = libc::pid_t::try_from(child.id()).map_err(io::Error::other)?;
    let mut status = 0;
    // SAFETY: `rusage` is plain integers, for which zero is a valid value.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    // SAFETY: `pid` is our unreaped child; `status` and `usage` are valid for writes.
    match unsafe { libc::wait4(pid, &mut status, libc::WNOHANG, &mut usage) } {
        0 => Ok(None),
        -1 => match io::Error::last_os_error() {
            e if e.kind() == io::ErrorKind::Interrupted => Ok(None),
            e => Err(e),
        },
        _ => {
            // Bytes on Apple platforms, kilobytes everywhere else.
            let unit = if cfg!(target_vendor = "apple") { 1 } else { 1024 };
            let peak = u64::try_from(usage.ru_maxrss).ok().filter(|&m| m > 0).map(|m| m * unit);
            Ok(Some((std::process::ExitStatus::from_raw(status).success(), peak)))
        }
    }
}

/// If `child` has exited: whether it succeeded, and its peak working set,
/// read while its handle is still open.
#[cfg(windows)]
fn try_reap(child: &mut Child) -> io::Result<Option<(bool, Option<u64>)>> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::System::ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    let Some(status) = child.try_wait()? else { return Ok(None) };
    let size = u32::try_from(std::mem::size_of::<PROCESS_MEMORY_COUNTERS>()).expect("a small struct");
    // SAFETY: plain integers, for which zero is a valid value.
    let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { std::mem::zeroed() };
    counters.cb = size;
    // SAFETY: the handle is the child's, open until `child` is dropped; `counters` is `size` bytes.
    let ok = unsafe { K32GetProcessMemoryInfo(child.as_raw_handle(), &mut counters, size) } != 0;
    Ok(Some((status.success(), ok.then_some(counters.PeakWorkingSetSize as u64).filter(|&m| m > 0))))
}

#[cfg(not(any(unix, windows)))]
fn try_reap(child: &mut Child) -> io::Result<Option<(bool, Option<u64>)>> {
    Ok(child.try_wait()?.map(|status| (status.success(), None)))
}

/// Kill `child` (`TerminateProcess` on Windows, `SIGKILL` elsewhere) and wait
/// for it to exit. Killing fails only if it has exited already, and waiting
/// reaps it all the same.
fn kill(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

/// How much of each output stream is kept: its start, and its end, where
/// libtest puts the failures and the summary. The middle of anything longer
/// is read and dropped, so a program printing in a loop can't fill memory.
const KEEP_HEAD: usize = 64 * 1024;
const KEEP_TAIL: usize = 256 * 1024;

/// A stream's output so far, its middle dropped beyond the limits.
#[derive(Default)]
struct Capped {
    head: Vec<u8>,
    tail: VecDeque<u8>,
    omitted: u64,
}

impl Capped {
    fn push(&mut self, bytes: &[u8]) {
        let (head, rest) = bytes.split_at(bytes.len().min(KEEP_HEAD - self.head.len()));
        self.head.extend_from_slice(head);
        self.tail.extend(rest);
        let over = self.tail.len().saturating_sub(KEEP_TAIL);
        self.tail.drain(..over);
        self.omitted += over as u64;
    }

    /// The text, and whether anything was dropped: then a marker stands in
    /// for the middle, cut at line breaks where there are any.
    fn into_text(mut self) -> (String, bool) {
        let (mut head, mut tail) = (&self.head[..], &*self.tail.make_contiguous());
        if self.omitted == 0 {
            return (String::from_utf8_lossy(&[head, tail].concat()).into_owned(), false);
        }
        let mut omitted = self.omitted;
        if let Some(i) = head.iter().rposition(|&b| b == b'\n') {
            omitted += (head.len() - i - 1) as u64;
            head = &head[..=i];
        }
        if let Some(i) = tail.iter().position(|&b| b == b'\n') {
            omitted += i as u64 + 1;
            tail = &tail[i + 1..];
        }
        let head = String::from_utf8_lossy(head);
        let sep = if head.ends_with('\n') { "" } else { "\n" };
        (format!("{head}{sep}… {omitted} bytes omitted …\n{}", String::from_utf8_lossy(tail)), true)
    }
}

/// One of a child's output pipes, read to the end on a thread of its own.
struct Drain {
    buf: Arc<Mutex<Capped>>,
    done: mpsc::Receiver<()>,
}

impl Drain {
    fn start(mut pipe: impl Read + Send + 'static) -> Drain {
        let buf = Arc::new(Mutex::new(Capped::default()));
        let (tx, done) = mpsc::channel();
        let shared = Arc::clone(&buf);
        thread::spawn(move || {
            let mut chunk = [0; 8192];
            loop {
                match pipe.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(n) => shared.lock().unwrap_or_else(|e| e.into_inner()).push(&chunk[..n]),
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
            let _ = tx.send(());
        });
        Drain { buf, done }
    }

    /// What was read (see [`Capped::into_text`]): to the end, unless the pipe
    /// is still open at `deadline` (the thread is then left to finish on its own).
    fn finish(self, deadline: Instant) -> (String, bool) {
        let _ = self.done.recv_timeout(deadline.saturating_duration_since(Instant::now()));
        std::mem::take(&mut *self.buf.lock().unwrap_or_else(|e| e.into_inner())).into_text()
    }
}

/// Delete the previous build at `path`, if any, so the compiler can write
/// the new one. Windows refuses while that program is running or open, and
/// even one just killed and waited for can be held a moment longer (by the
/// system, or a virus scanner), so this retries for two seconds.
fn remove_stale(path: &Path) -> io::Result<()> {
    let mut tries = 0;
    loop {
        match fs::remove_file(path) {
            Ok(()) => return Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(_) if tries < 40 => {
                tries += 1;
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                return Err(io::Error::new(
                    e.kind(),
                    format!("couldn't replace {} ({e}): is another gauntlet testing this problem?", path.display()),
                ));
            }
        }
    }
}

/// Read a working file. An editor on Windows can have it open exclusively
/// for a moment while it saves (a sharing violation), so that is retried
/// briefly.
pub fn read_source(path: &Path) -> io::Result<String> {
    let mut tries = 0;
    loop {
        match fs::read_to_string(path) {
            Err(e) if tries < 10 && busy(&e) => {
                tries += 1;
                thread::sleep(Duration::from_millis(30));
            }
            read => return read,
        }
    }
}

/// `ERROR_SHARING_VIOLATION` or `ERROR_LOCK_VIOLATION`: another program has
/// the file open in a way that keeps us out, for now.
fn busy(e: &io::Error) -> bool {
    cfg!(windows) && matches!(e.raw_os_error(), Some(32 | 33))
}

/// The output of a run that was killed, with an explanation appended.
fn timeout_message(output: &str, timeout: Duration) -> String {
    // A stack overflow can hang the test process rather than abort it, so
    // runaway recursion shows up here too.
    format!(
        "{output}\nThe tests didn't finish in {}s: an infinite loop, a slower complexity than\n\
         the target, or deep recursion (a stack overflow can hang instead of crashing).\n",
        timeout.as_secs()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_from_summary_line() {
        let plain = "running 3 tests\ntest result: FAILED. 2 passed; 1 failed; 0 ignored;";
        assert_eq!(parse_counts(plain), (2, 1));
        let colored = "test result: \x1b[32mok\x1b[0m. 17 passed; 0 failed; 0 ignored";
        assert_eq!(parse_counts(colored), (17, 0));
        assert_eq!(parse_counts("error: could not compile"), (0, 0));
    }

    /// libtest's output for failures with these panic messages, after `passed` passes.
    fn libtest(passed: u32, failures: &[(&str, &str)]) -> String {
        let mut out = format!("running {} tests\n", passed as usize + failures.len());
        for (name, _) in failures {
            out.push_str(&format!("test tests::{name} ... FAILED\n"));
        }
        out.push_str("\nfailures:\n\n");
        for (name, msg) in failures {
            out.push_str(&format!("---- tests::{name} stdout ----\n\n"));
            out.push_str(&format!("thread 'tests::{name}' panicked at x.rs:3:5:\n{msg}\n\n"));
        }
        out.push_str("\nfailures:\n");
        for (name, _) in failures {
            out.push_str(&format!("    tests::{name}\n"));
        }
        out + &format!("\ntest result: FAILED. {passed} passed; {} failed; 0 ignored;\n", failures.len())
    }

    fn outcome(success: bool, output: &str) -> Outcome {
        let (passed, failed) = parse_counts(output);
        verdict(success, output.to_string(), false, passed, failed)
    }

    #[test]
    fn passing_takes_a_passed_test() {
        let ok = "running 1 test\ntest tests::a ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n";
        assert!(outcome(true, ok).passed());
        // std::process::exit(0) in the middle of the tests.
        let o = outcome(true, "running 3 tests\n");
        assert!(matches!(&o, Outcome::Failed(out) if out.contains("std::process::exit")), "{}", o.output());
        let o = outcome(true, "running 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored;\n");
        assert!(matches!(&o, Outcome::Failed(out) if out.contains("No tests ran")), "{}", o.output());
    }

    #[test]
    fn not_started_only_when_every_failure_is_a_todo() {
        let todo = "not yet implemented";
        assert!(matches!(outcome(false, &libtest(0, &[("a", todo), ("b", todo)])), Outcome::NotStarted));
        // One real failure among the todo!()s: it's shown, under a count.
        let o = outcome(false, &libtest(0, &[("a", todo), ("b", "assertion `left == right` failed"), ("c", todo)]));
        let Outcome::Failed(out) = o else { panic!("{}", o.output()) };
        assert!(out.starts_with("2 tests still hit todo!(): tests::a, tests::c\n"), "{out}");
        assert!(out.contains("---- tests::b stdout ----\n\nthread 'tests::b' panicked"), "{out}");
        assert!(out.contains("assertion `left == right` failed"), "{out}");
        assert!(!out.contains("tests::a stdout") && out.ends_with("3 failed; 0 ignored;\n"), "{out}");
        // Some passed, the rest todo!(): started.
        let o = outcome(false, &libtest(2, &[("a", todo)]));
        assert!(matches!(&o, Outcome::Failed(out) if out.starts_with("1 test still hit todo!(): tests::a\n")));
        // A crash with no failure report isn't a todo!(), even if one was hit.
        assert!(matches!(outcome(false, "running 2 tests\nnot yet implemented\nAbort trap"), Outcome::Failed(_)));
    }

    #[test]
    fn endless_output_keeps_its_start_and_end() {
        const SUMMARY: &[u8] = b"\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n";
        let flood = 5_000_000;
        let pipe = b"running 1 test\n".chain(io::repeat(b'x').take(flood)).chain(SUMMARY);
        let (text, cut) = Drain::start(pipe).finish(Instant::now() + Duration::from_secs(10));
        assert!(cut);
        // The partial lines either side of the gap go too.
        let omitted = flood + 1;
        assert_eq!(text, format!("running 1 test\n… {omitted} bytes omitted …\n{}", &str_of(SUMMARY)[1..]));
        assert_eq!(parse_counts(&text), (1, 0));
        assert!(verdict(true, text, true, 1, 0).passed());

        let (text, cut) = Drain::start(&b"short\n"[..]).finish(Instant::now() + Duration::from_secs(10));
        assert_eq!((text.as_str(), cut), ("short\n", false));
        // No line breaks to cut at: still bounded.
        let (text, cut) = Drain::start(io::repeat(b'y').take(1 << 20)).finish(Instant::now() + Duration::from_secs(10));
        assert!(cut && text.len() < KEEP_HEAD + KEEP_TAIL + 64);
        assert!(text.contains(&format!("… {} bytes omitted …", (1 << 20) - KEEP_HEAD - KEEP_TAIL)));
    }

    fn str_of(b: &[u8]) -> &str {
        std::str::from_utf8(b).unwrap()
    }

    #[test]
    fn cut_output_is_shown_as_it_is() {
        let todo = "not yet implemented";
        let out = libtest(0, &[("a", todo), ("b", todo)]);
        assert!(matches!(verdict(false, out.clone(), true, 0, 2), Outcome::Failed(o) if o == out));
    }

    #[test]
    fn working_code_is_the_template_without_its_test_module() {
        let template = "// T\nfn a() {}\n\n#[cfg(test)]\n#[path = \"tests.rs\"]\nmod tests;\n";
        assert_eq!(without_tests_mod(template), "// T\nfn a() {}\n");
        assert_eq!(without_tests_mod("fn a() {}\n"), "fn a() {}\n");
        assert_eq!(without_tests_mod("#[cfg(test)]\nmod tests;\n"), "#[cfg(test)]\nmod tests;\n");
        let attached = attach_tests("fn a() {}", Path::new("/c/x/tests.rs"));
        assert!(attached.starts_with("fn a() {}\n\n"), "{attached}");
        assert!(attached.ends_with("#[cfg(test)]\n#[path = \"/c/x/tests.rs\"]\nmod tests;\n"), "{attached}");
        // CRLF from an editor on Windows: still a line of its own.
        assert!(attach_tests("fn a() {}\r\n", Path::new("t.rs")).starts_with("fn a() {}\r\n\n// Added"));
    }

    #[test]
    fn windows_paths_are_written_as_valid_literals() {
        let path = r"C:\Users\A B\AppData\Local\gauntlet\content\0.1.0-ab\05_two\01_three_sum\tests.rs";
        let attached = attach_tests("fn a() {}\n", Path::new(path));
        let want = r#"#[path = "C:\\Users\\A B\\AppData\\Local\\gauntlet\\content\\0.1.0-ab\\05_two\\01_three_sum\\tests.rs"]"#;
        assert!(attached.contains(&format!("\n{want}\n")), "{attached}");
        // Escapes that would otherwise change the path: \t, \n, \u, \x, \".
        assert_eq!(string_literal(r"C:\tmp\new\u{1}\x41"), r#""C:\\tmp\\new\\u{1}\\x41""#);
        assert_eq!(string_literal("a \"b\"\tc\u{7}"), r#""a \"b\"\tc\u{7}""#);
        assert_eq!(string_literal("/home/u/ünïcode/tests.rs"), "\"/home/u/ünïcode/tests.rs\"");
    }

    /// A scratch problem: `tests.rs` and a working file elsewhere.
    fn scratch(name: &str, tests: &str) -> (PathBuf, Env) {
        let dir = std::env::temp_dir().join(format!("gauntlet-runner-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("content")).unwrap();
        fs::write(dir.join("content/tests.rs"), tests).unwrap();
        let env = Env { build_dir: dir.join("build"), timeout: Duration::from_secs(30), cancel: Cancel::default() };
        (dir, env)
    }

    #[test]
    fn diagnostics_name_the_working_file() {
        let tests =
            "use super::*;\n#[test]\nfn two() { assert_eq!(double(1), 2); }\n#[test]\nfn three() { triple(1); }\n";
        let (dir, env) = scratch("diag", tests);
        // Joined part by part, as the workspace does: `\` on Windows.
        let file = dir.join("data").join("runs").join("2").join("double.rs");
        let tests = dir.join("content").join("tests.rs");
        let code = |source: &'static str| Code { file: &file, source, tests: &tests };
        let generated = env.build_dir.join("double.rs");
        let shown = file.display().to_string();

        // A compile error: on line 3 of the working file.
        let broken =
            "// header\npub fn double(x: i32) -> i32 {\n    x * \"2\"\n}\npub fn triple(x: i32) -> i32 { todo!() }\n";
        let r = env.run_tests("double", &code(broken), false).unwrap();
        let Outcome::CompileError(err) = &r.outcome else { panic!("{}", r.outcome.output()) };
        assert!(err.contains(&format!("{shown}:3:")), "{err}");
        assert!(!err.contains(&generated.display().to_string()), "{err}");

        // A failing assertion in the tests, and a panic located in the working file.
        let wrong =
            "// header\npub fn double(x: i32) -> i32 {\n    x * 3\n}\npub fn triple(x: i32) -> i32 { panic!() }\n";
        let r = env.run_tests("double", &code(wrong), false).unwrap();
        let Outcome::Failed(out) = &r.outcome else { panic!("{}", r.outcome.output()) };
        assert!(out.contains("left: 3") && out.contains(&format!("{}:3", tests.display())), "{out}");
        assert!(out.contains(&format!("{shown}:5:")), "the panic is in the working file: {out}");
        assert_eq!((r.passed, r.failed), (0, 2));

        // Half done: the real failure, under a count of the todo!()s.
        let half = "pub fn double(x: i32) -> i32 { x * 3 }\npub fn triple(x: i32) -> i32 { todo!() }\n";
        let r = env.run_tests("double", &code(half), false).unwrap();
        let Outcome::Failed(out) = &r.outcome else { panic!("{}", r.outcome.output()) };
        assert!(out.starts_with("1 test still hit todo!(): tests::three\n"), "{out}");
        assert!(out.contains("left: 3") && !out.contains("tests::three stdout"), "{out}");

        // Their own `mod tests`: explained.
        let own = "pub fn double(x: i32) -> i32 { x * 2 }\npub fn triple(x: i32) -> i32 { x * 3 }\nmod tests {}\n";
        let r = env.run_tests("double", &code(own), false).unwrap();
        let Outcome::CompileError(err) = &r.outcome else { panic!("{}", r.outcome.output()) };
        assert!(err.contains("declares its own `mod tests`") && err.contains("3 | mod tests {}"), "{err}");

        let right = "pub fn double(x: i32) -> i32 { x * 2 }\npub fn triple(x: i32) -> i32 { x * 3 }\n";
        assert!(env.run_tests("double", &code(right), false).unwrap().outcome.passed());
        assert!(binary(&env.build_dir, "double").is_file(), "built as {}", binary(&env.build_dir, "double").display());
        // Saved with CRLF line endings: the same.
        let crlf = right.replace('\n', "\r\n");
        let r = env.run_tests("double", &Code { file: &file, source: &crlf, tests: &tests }, false).unwrap();
        assert!(r.outcome.passed(), "{}", r.outcome.output());

        // Cancelled: nothing more runs.
        env.cancel.cancel();
        let err = env.run_tests("double", &code(right), false).err().expect("cancelled");
        assert_eq!(err.kind(), io::ErrorKind::Interrupted);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_missing_linker_gets_a_hint() {
        let stderr =
            "error: linker `cc` not found\n  |\n  = note: No such file or directory (os error 2)\n".to_string();
        let explained = explain_compile_error(stderr.clone());
        assert!(explained.starts_with(&stderr) && explained.len() > stderr.len(), "{explained}");
        let ordinary = "error[E0425]: cannot find value `x` in this scope\n".to_string();
        assert_eq!(explain_compile_error(ordinary.clone()), ordinary);
    }

    #[test]
    fn test_binaries_have_the_platform_suffix() {
        let bin = binary(Path::new("build"), "trees/invert-solution");
        assert_eq!(bin, Path::new("build").join(format!("trees__invert-solution{}", std::env::consts::EXE_SUFFIX)));
        if cfg!(windows) {
            assert!(bin.to_string_lossy().ends_with(".exe"));
        }
    }

    #[test]
    fn stale_binaries_are_removed() {
        let dir = std::env::temp_dir().join(format!("gauntlet-runner-stale-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let bin = binary(&dir, "x");
        remove_stale(&bin).expect("nothing there is fine");
        fs::write(&bin, "old").unwrap();
        remove_stale(&bin).unwrap();
        assert!(!bin.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn solution_display_stops_at_the_tests() {
        assert_eq!(display_solution("fn a() {}\n\n#[cfg(test)]\nmod tests;\n"), "fn a() {}");
    }

    #[test]
    fn rust_project_json_is_valid_for_any_path() {
        // Joined with the platform's separator, written with `/`.
        let weird = Path::new("runs").join("3").join("we\"ird ' päth.rs");
        let files = [PathBuf::from("two_sum.rs"), weird];
        let json: serde_json::Value =
            serde_json::from_str(&rust_project_json(&files, Path::new("/data"), Path::new("/build"))).unwrap();
        let crates = json["crates"].as_array().unwrap();
        let roots: Vec<&str> = crates.iter().map(|c| c["root_module"].as_str().unwrap()).collect();
        assert_eq!(roots, ["two_sum.rs", "runs/3/we\"ird ' päth.rs"]);
        #[cfg(unix)]
        {
            // A backslash is just a character in a Unix file name.
            let json = rust_project_json(&[PathBuf::from("a\\b.rs")], Path::new("/data"), Path::new("/build"));
            let json: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(json["crates"][0]["root_module"], "a\\b.rs");
        }
        assert_eq!(json["crates"][0]["edition"], EDITION);
        // Each crate is labelled with its path, which the flycheck runnable
        // hands to clippy-driver from the data directory.
        assert_eq!(json["crates"][1]["build"]["label"], "runs/3/we\"ird ' päth.rs");
        let flycheck = &json["runnables"][0];
        assert_eq!((&flycheck["kind"], &flycheck["program"]), (&"flycheck".into(), &"clippy-driver".into()));
        assert_eq!(flycheck["cwd"], Path::new("/data").to_string_lossy().as_ref());
        assert_eq!(flycheck["args"].as_array().unwrap().last().unwrap(), "{label}");
        assert!(json.get("sysroot").is_none_or(|s| s.as_str().is_some_and(|s| !s.is_empty())));
        if let Some(src) = json.get("sysroot_src") {
            let src = Path::new(src.as_str().unwrap());
            assert!(src.starts_with(json["sysroot"].as_str().unwrap()) && src.ends_with("library"));
        }
    }

    /// A shell running `script`: `sh -c` here, `cmd /C` on Windows.
    fn shell(unix: &str, windows: &str) -> Command {
        if cfg!(windows) {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", windows]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", unix]);
            cmd
        }
    }

    /// A program that sleeps for `secs` seconds, run directly.
    fn sleeper(secs: u32) -> Command {
        if cfg!(windows) {
            // ping waits a second between its n echoes.
            let mut cmd = Command::new("ping");
            cmd.args(["-n", &(secs + 1).to_string(), "127.0.0.1"]);
            cmd
        } else {
            let mut cmd = Command::new("sleep");
            cmd.arg(secs.to_string());
            cmd
        }
    }

    fn trimmed_lines(s: &str) -> Vec<&str> {
        s.lines().map(str::trim_end).collect()
    }

    #[test]
    fn captures_output_and_status() {
        let mut cmd = shell("echo out; echo err >&2; exit 3", "echo out&(echo err)1>&2&exit /b 3");
        let f = run(&mut cmd, Some(Duration::from_secs(10)), &Cancel::default()).unwrap();
        assert_eq!(f.ended, Ended::Exited(false));
        assert_eq!(trimmed_lines(&f.stdout), ["out"]);
        assert_eq!(trimmed_lines(&f.stderr), ["err"]);
        assert_eq!(trimmed_lines(&f.output()), ["out", "err"]);
        let ok = run(&mut shell("exit 0", "exit /b 0"), None, &Cancel::default()).unwrap();
        assert_eq!(ok.ended, Ended::Exited(true));
    }

    #[test]
    fn kills_after_the_timeout() {
        let started = Instant::now();
        let f = run(&mut sleeper(5), Some(Duration::from_millis(100)), &Cancel::default()).unwrap();
        assert_eq!(f.ended, Ended::TimedOut);
        assert!(f.runtime < Duration::from_secs(4));
        assert!(started.elapsed() < Duration::from_secs(4), "killed and reaped, not waited out");
    }

    #[test]
    fn a_grandchild_holding_the_pipes_does_not_hang_the_run() {
        // The shell is killed; the sleep it started keeps its stdout open.
        let mut cmd = shell("sleep 5; echo done", "ping -n 6 127.0.0.1 & echo done");
        let started = Instant::now();
        let f = run(&mut cmd, Some(Duration::from_millis(300)), &Cancel::default()).unwrap();
        assert_eq!(f.ended, Ended::TimedOut);
        assert!(started.elapsed() < Duration::from_secs(3), "took {:?}", started.elapsed());
    }

    #[test]
    fn cancelling_kills_the_child() {
        let cancel = Cancel::default();
        let canceller = cancel.clone();
        let started = Instant::now();
        let stop = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            canceller.cancel();
        });
        let f = run(&mut sleeper(5), None, &cancel).unwrap();
        stop.join().unwrap();
        assert_eq!(f.ended, Ended::Cancelled);
        assert!(started.elapsed() < Duration::from_secs(4), "took {:?}", started.elapsed());
        let again = run(&mut sleeper(5), None, &cancel).err().expect("cancelled before it starts");
        assert_eq!(again.kind(), io::ErrorKind::Interrupted);
    }
}
