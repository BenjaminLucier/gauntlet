//! The Rust toolchain gauntlet compiles with: the `rustc` and `clippy-driver`
//! on your `PATH` (cargo isn't needed). Each is checked when it's needed
//! rather than up front: rustc when a command is about to compile, clippy
//! when a review or `check` uses it, and the linker (which rustup doesn't
//! install) when a compile fails for want of one.

use std::cmp::Ordering;
use std::io;
use std::process::Command;
use std::sync::OnceLock;

use color_eyre::eyre::Result;

use crate::ui::user;

/// The oldest Rust the problems are checked on (in CI), and so the oldest
/// gauntlet accepts: the crate's own `rust-version`.
pub const MIN_RUST: &str = env!("CARGO_PKG_RUST_VERSION");

const RUSTUP: &str = "https://rustup.rs";

/// What a missing clippy is told, in the TUI and by `check`.
pub const CLIPPY_MISSING: &str = "clippy isn't installed: run `rustup component add clippy`";

/// Fail with one line unless `rustc` runs and is at least [`MIN_RUST`]. One
/// `rustc --version`, asked once per process.
pub fn require_rustc() -> Result<()> {
    static VERDICT: OnceLock<Result<(), String>> = OnceLock::new();
    VERDICT.get_or_init(|| judge_rustc(version_output("rustc"))).clone().map_err(user)
}

/// Fail with one line unless `clippy-driver` runs and is new enough. For
/// `check`; the TUI finds out when you ask for a review.
pub fn require_clippy() -> Result<()> {
    judge_clippy(version_output("clippy-driver")).map_err(user)
}

/// `rustc --version`, e.g. `rustc 1.91.0 (f8297e351 2025-09-14)`, asked once
/// per process; `None` if it fails.
pub fn rustc_version() -> Option<&'static str> {
    static VERSION: OnceLock<Option<String>> = OnceLock::new();
    VERSION
        .get_or_init(|| match version_output("rustc") {
            Ok((true, text)) => Some(first_line(&text).to_string()),
            _ => None,
        })
        .as_deref()
}

/// Whether it ran, and everything it printed.
fn version_output(bin: &str) -> io::Result<(bool, String)> {
    let out = Command::new(bin).arg("--version").output()?;
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    Ok((out.status.success(), text))
}

/// The first line of `text` that says something, for quoting.
fn first_line(text: &str) -> &str {
    text.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("no output")
}

fn judge_rustc(ran: io::Result<(bool, String)>) -> Result<(), String> {
    let need = format!("gauntlet needs Rust {MIN_RUST} or later to compile your code");
    let (ok, text) = match ran {
        Ok(ran) => ran,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(format!("rustc isn't on your PATH, and {need}: install it from {RUSTUP}"));
        }
        Err(e) => return Err(format!("couldn't run rustc ({e}), and {need}: see {RUSTUP}")),
    };
    if !ok {
        return Err(format!("`rustc --version` failed ({}), and {need}: see {RUSTUP}", first_line(&text)));
    }
    match extract_version(&text) {
        Some(v) if compare_versions(&v, MIN_RUST).is_lt() => {
            Err(format!("rustc {v} is too old: {need}; run `rustup update` (or see {RUSTUP})"))
        }
        // Too odd to judge: let the compiler speak for itself.
        _ => Ok(()),
    }
}

fn judge_clippy(ran: io::Result<(bool, String)>) -> Result<(), String> {
    let (ok, text) = match ran {
        Ok(ran) => ran,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Err(CLIPPY_MISSING.into()),
        Err(e) => return Err(format!("couldn't run clippy-driver ({e}): {CLIPPY_MISSING}")),
    };
    if !ok {
        return Err(if clippy_not_installed(&text) {
            CLIPPY_MISSING.to_string()
        } else {
            format!("`clippy-driver --version` failed: {}", first_line(&text))
        });
    }
    // clippy 0.1.N ships with Rust 1.N.
    match extract_version(&text).and_then(|v| v.strip_prefix("0.").map(str::to_string)) {
        Some(v) if compare_versions(&v, MIN_RUST).is_lt() => Err(format!(
            "{} is too old: gauntlet needs the clippy of Rust {MIN_RUST} or later; run `rustup update`",
            first_line(&text)
        )),
        _ => Ok(()),
    }
}

/// rustup's proxy for a component that isn't installed: "error: 'clippy-driver'
/// is not installed for the toolchain 'stable-…'".
pub fn clippy_not_installed(output: &str) -> bool {
    output.contains("is not installed for the toolchain") || output.contains("rustup component add clippy")
}

// ---- The linker -------------------------------------------------------------

/// What to do about compiler output that says the linker is missing, or
/// `None` if it doesn't. `os` is [`std::env::consts::OS`]: rustc reports a
/// missing `cc` the same way on macOS and Linux, but the fix differs.
pub fn linker_hint(stderr: &str, os: &str) -> Option<&'static str> {
    const MACOS: &str = "rustc needs a linker, which rustup doesn't install: install the Xcode Command Line \
                         Tools with `xcode-select --install`, then try again.";
    const LINUX: &str = "rustc needs a linker, which rustup doesn't install: install a C toolchain, e.g. \
                         `sudo apt install build-essential` or your distribution's gcc package, then try again.";
    const WINDOWS: &str = "rustc needs the MSVC linker, which rustup doesn't install: install the Visual Studio \
                           Build Tools (https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the \
                           \"Desktop development with C++\" workload, or use the GNU toolchain \
                           (`rustup default stable-gnu`), then try again.";
    if stderr.contains("xcrun: error") || stderr.contains("invalid active developer path") {
        Some(MACOS)
    } else if stderr.contains("linker `link.exe` not found")
        || stderr.contains("msvc targets depend on the msvc linker")
    {
        Some(WINDOWS)
    } else if stderr.contains("linker `cc` not found") {
        Some(if os == "macos" { MACOS } else { LINUX })
    } else {
        None
    }
}

// ---- Versions ---------------------------------------------------------------

/// The first version-looking token: digits with at least one dot, an optional
/// leading `v`, ignoring surrounding punctuation and any `-pre`/`+build` suffix.
fn extract_version(output: &str) -> Option<String> {
    output.split(|c: char| c.is_whitespace() || "(),;:\"'[]".contains(c)).find_map(|tok| {
        let tok = tok.strip_prefix(['v', 'V']).unwrap_or(tok);
        let core: String = tok.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
        let core = core.trim_end_matches('.');
        let valid = core.contains('.') && core.split('.').all(|p| !p.is_empty());
        let rest = &tok[core.len()..];
        let clean_end = rest.is_empty() || rest.starts_with(['-', '+', '.']);
        (valid && clean_end && core.starts_with(|c: char| c.is_ascii_digit())).then(|| core.to_string())
    })
}

/// Compare dotted versions numerically, part by part; missing parts are 0.
fn compare_versions(a: &str, b: &str) -> Ordering {
    let parts = |s: &str| -> Vec<u64> { s.split('.').map(|p| p.parse().unwrap_or(0)).collect() };
    let (a, b) = (parts(a), parts(b));
    let n = a.len().max(b.len());
    (0..n).map(|i| a.get(i).unwrap_or(&0).cmp(b.get(i).unwrap_or(&0))).find(|o| o.is_ne()).unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_versions_from_real_outputs() {
        let cases = [
            ("Version 7.0.2", "7.0.2"),
            ("v24.18.0\n", "24.18.0"),
            ("cargo 1.99.1 (Homebrew 2026-10-01)", "1.99.1"),
            ("v2.0.0 (Cyanoptera) abc123", "2.0.0"),
            ("rustc 1.99.1 (deadbeef 2026-09-01)", "1.99.1"),
            ("clippy 0.1.99 (deadbeef 2026-09-01)", "0.1.99"),
            ("rust-analyzer 0.9.0-beta.1", "0.9.0"),
            ("tool, version 1.2.", "1.2"),
            ("rustc 1.92.0-nightly (abc 2026-01-01)\r\n", "1.92.0"),
        ];
        for (out, want) in cases {
            assert_eq!(extract_version(out).as_deref(), Some(want), "output {out:?}");
        }
    }

    #[test]
    fn ignores_tokens_that_are_not_versions() {
        assert_eq!(extract_version("no version here"), None);
        assert_eq!(extract_version("built 2026 by x"), None);
        assert_eq!(extract_version("x86_64 tool 3.1"), Some("3.1".into()));
        assert_eq!(extract_version("hash a1.2b then 4.5.6"), Some("4.5.6".into()));
        assert_eq!(extract_version(""), None);
    }

    #[test]
    fn compares_numerically_part_by_part() {
        assert_eq!(compare_versions("1.10.0", "1.9.0"), Ordering::Greater);
        assert_eq!(compare_versions("0.13.7", "0.5.0"), Ordering::Greater);
        assert_eq!(compare_versions("24.18.0", "24.18.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.2", "1.2.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.2", "1.2.1"), Ordering::Less);
        assert_eq!(compare_versions("2.0.0", "10.0.0"), Ordering::Less);
        assert_eq!(compare_versions("3.13.2", "3.13"), Ordering::Greater);
    }

    fn ran(ok: bool, text: &str) -> io::Result<(bool, String)> {
        Ok((ok, text.to_string()))
    }

    #[test]
    fn rustc_must_run_and_be_new_enough() {
        assert_eq!(MIN_RUST, env!("CARGO_PKG_RUST_VERSION"));
        let missing = judge_rustc(Err(io::ErrorKind::NotFound.into())).unwrap_err();
        assert!(missing.starts_with("rustc isn't on your PATH") && missing.contains(RUSTUP), "{missing}");
        assert!(missing.contains(&format!("Rust {MIN_RUST} or later")), "{missing}");
        assert!(!missing.contains('\n'), "one line: {missing}");

        let old = judge_rustc(ran(true, "rustc 1.80.1 (3f5fd8dd4 2024-08-06)\n")).unwrap_err();
        assert!(old.starts_with("rustc 1.80.1 is too old") && old.contains("rustup update"), "{old}");

        let broken = "error: rustup could not choose a version of rustc to run\nhelp: run `rustup default stable`";
        let err = judge_rustc(ran(false, broken)).unwrap_err();
        assert!(err.contains("(error: rustup could not choose a version of rustc to run)"), "{err}");

        assert_eq!(judge_rustc(ran(true, &format!("rustc {MIN_RUST}.0 (abc 2025-10-30)"))), Ok(()));
        assert_eq!(judge_rustc(ran(true, "rustc 9.0.0 (abc 2099-01-01)")), Ok(()));
        assert_eq!(judge_rustc(ran(true, "a rustc of some kind")), Ok(()), "unknown: let it try");
    }

    #[test]
    fn clippy_must_be_installed_and_new_enough() {
        assert_eq!(judge_clippy(Err(io::ErrorKind::NotFound.into())).unwrap_err(), CLIPPY_MISSING);
        let proxy = "error: 'clippy-driver' is not installed for the toolchain 'stable-aarch64-apple-darwin'.\n\
                     To install, run `rustup component add clippy`\n";
        assert_eq!(judge_clippy(ran(false, proxy)).unwrap_err(), CLIPPY_MISSING);
        let old = judge_clippy(ran(true, "clippy 0.1.80 (3f5fd8dd41 2024-08-06)")).unwrap_err();
        assert!(old.contains("too old") && old.contains("rustup update"), "{old}");
        assert_eq!(judge_clippy(ran(true, &format!("clippy 0.{MIN_RUST} (abc 2025-10-30)"))), Ok(()));
        assert!(judge_clippy(ran(false, "Segmentation fault")).unwrap_err().contains("Segmentation fault"));
    }

    #[test]
    fn missing_linkers_are_recognised_on_every_platform() {
        let macos = "error: linking with `cc` failed: exit status: 1\n  |\n  = note: \"cc\" \"-arch\" \"arm64\" ...\n  \
                     = note: xcrun: error: invalid active developer path (/Library/Developer/CommandLineTools), \
                     missing xcrun at: /Library/Developer/CommandLineTools/usr/bin/xcrun\n";
        assert!(linker_hint(macos, "macos").unwrap().contains("xcode-select --install"));

        let linux = "error: linker `cc` not found\n  |\n  = note: No such file or directory (os error 2)\n\n\
                     error: aborting due to 1 previous error\n";
        assert!(linker_hint(linux, "linux").unwrap().contains("build-essential"));
        assert!(linker_hint(linux, "macos").unwrap().contains("xcode-select"), "cc missing on a Mac");

        let windows = "error: linker `link.exe` not found\n  |\n  = note: program not found\n\n\
                       note: the msvc targets depend on the msvc linker but `link.exe` was not found\n\n\
                       note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio \
                       were installed with the Visual C++ option.\n";
        let hint = linker_hint(windows, "windows").unwrap();
        assert!(hint.contains("Visual Studio Build Tools") && hint.contains("Desktop development with C++"));
        assert!(hint.contains("GNU toolchain"));
        let just_the_note = "note: the msvc targets depend on the msvc linker but `link.exe` was not found\n";
        assert_eq!(linker_hint(just_the_note, "windows"), Some(hint));

        let ordinary = "error[E0308]: mismatched types\n --> /home/u/two_sum.rs:3:5\n";
        assert_eq!(linker_hint(ordinary, "linux"), None);
        assert_eq!(linker_hint("error: linking with `cc` failed: undefined symbol: _main", "linux"), None);
    }
}
