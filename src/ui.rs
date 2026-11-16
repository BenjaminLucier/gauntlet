//! Plain-text output for the few commands that run outside the TUI.

use std::fmt;
use std::path::Path;
use std::time::Duration;

use color_eyre::eyre::Report;

use crate::problem::{Workspace, relative_path};
use crate::runner::{Outcome, SENTINEL};

pub const BOLD: &str = "\x1b[1m";
pub const RED: Colour = Colour("\x1b[31m");
pub const GREEN: Colour = Colour("\x1b[32m");
pub const RESET: &str = "\x1b[0m";

/// A colour's escape code, which writes nothing under [`no_color`].
pub struct Colour(&'static str);

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if no_color() { Ok(()) } else { f.write_str(self.0) }
    }
}

/// <https://no-color.org>: `NO_COLOR` set and not empty means no colour.
pub fn no_color() -> bool {
    std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty())
}

/// An error that's the user's to fix, not a bug: `main` prints it as one
/// plain line instead of a report with source locations.
#[derive(Debug)]
pub struct UserError(pub String);

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for UserError {}

pub fn user(msg: impl Into<String>) -> Report {
    Report::new(UserError(msg.into()))
}

pub fn render_outcome(outcome: &Outcome, timeout: Duration) -> String {
    match outcome {
        Outcome::Passed(output) => format!("{output}\n{GREEN}{BOLD}✓ All tests passed!{RESET}\n"),
        Outcome::NotStarted => format!("Not started: the tests hit a {SENTINEL}.\n"),
        Outcome::Failed(out) => format!("{out}\n{RED}{BOLD}✗ Some tests failed.{RESET}\n"),
        Outcome::CompileError(err) => format!("{err}\n{RED}{BOLD}✗ Doesn't compile.{RESET}\n"),
        Outcome::Timeout(out) => format!("{out}\n{RED}{BOLD}✗ Timed out after {}s.{RESET}\n", timeout.as_secs()),
    }
}

/// Markdown index for the project README: a heading and a table per section,
/// linking each statement relative to `readme_dir`.
pub fn render_index(ws: &Workspace, readme_dir: &Path) -> String {
    let mut out = String::new();
    let mut last = None;
    for p in &ws.problems {
        if last != Some(p.section) {
            if last.is_some() {
                out.push('\n');
            }
            let s = ws.section_of(p);
            out.push_str(&format!("### {}\n\n", s.title));
            if !s.description.is_empty() {
                out.push_str(&format!("{}\n\n", s.description));
            }
            out.push_str("| Problem | Difficulty |\n|---|---|\n");
            last = Some(p.section);
        }
        let readme = relative_path(readme_dir, &p.readme()).to_string_lossy().replace('\\', "/");
        out.push_str(&format!("| [{}]({readme}) | {} |\n", p.title, p.difficulty,));
    }
    out
}
