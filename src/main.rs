//! gauntlet — a terminal trainer: the standard library you reach for every
//! day, then data structures and algorithms, built and applied. Solve them
//! all, then race the clock.

mod core;
mod db;
mod dirs;
mod fmt;
mod perf;
mod problem;
mod run;
mod runner;
mod store;
mod toolchain;
mod tui;
mod ui;

use std::fs;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;
use std::sync::Mutex;

use color_eyre::eyre::{Result, WrapErr};

use crate::core::Gauntlet;
use crate::dirs::{Dirs, show};
use crate::problem::{REQUIRED_FILES, SOLUTION, TEMPLATE, Workspace};
use crate::runner::{Code, Outcome, SENTINEL};
use crate::store::Rating;
use crate::ui::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Where the data directory is, in the platform's terms.
const DATA_HOME: &str = if cfg!(windows) { r"%APPDATA%\gauntlet" } else { "$XDG_DATA_HOME/gauntlet" };
const LOG_HOME: &str =
    if cfg!(windows) { r"%LOCALAPPDATA%\gauntlet\gauntlet.log" } else { "$XDG_STATE_HOME/gauntlet/gauntlet.log" };

/// The help text. `dirs` fills in where things are, if it could be worked out.
fn usage(dirs: Option<&Dirs>) -> String {
    let at = |f: fn(&Dirs) -> String, unknown: &str| dirs.map_or_else(|| unknown.to_string(), f);
    let data = at(|d| show(&d.data), DATA_HOME);
    fn builtin(d: &Dirs) -> String {
        format!("built in, extracted to {}", show(&d.cache.join("content")))
    }
    let content = at(|d| d.content_override.as_ref().map_or_else(|| builtin(d), |c| show(c)), "built in");
    let log = at(|d| show(&d.log_file()), LOG_HOME);
    format!(
        "gauntlet {VERSION} — 128 practice problems that take you from knowing Rust to being fluent in
it. Solve them all, then race the clock.

USAGE:
  gauntlet [scope]               completionist mode: work through every problem, at your
                                 own pace. scope narrows it to a section (gauntlet trees)
                                 or a problem (gauntlet three_sum); a part of one name
                                 works too (gauntlet pointers)
  gauntlet review [scope]        completionist mode over the problems up for review: those
                                 you last rated 1 or 2 (not trivial), hardest and least
                                 recently solved first, then those whose best solution is
                                 well off the reference solution's runtime or memory
  gauntlet run [scope]           start a timed run over scope (default: everything), e.g.
                                 gauntlet run two_pointers, or resume the run in progress
  gauntlet run --delete          abandon the run in progress and delete its files
  gauntlet run --clean [n]       delete the saved code of finished run n, or of every
                                 finished run; their times and PBs are kept
  gauntlet stats [scope]         straight to the stats
  gauntlet reset                 delete your working files, runs and every event
  gauntlet --version · --help

  Everything else happens inside: press ? for keys. --yes (-y) skips confirmations.
  Open your data directory in your editor: its rust-project.json gives rust-analyzer
  every working file, and clippy on save. Your code is compiled with the rustc (Rust {MIN_RUST} or later) and,
  for a review, the clippy-driver on your PATH.

PATHS AND ENVIRONMENT:
  your data    {data}
               working files, runs and progress ({DATA_HOME})
  problems     {content}
               GAUNTLET_CONTENT_DIR reads them from a directory instead, e.g. a checkout's content/
  log          {log}
               GAUNTLET_LOG=debug (or trace) logs more

MAINTAINERS (in a checkout; cargo run reads content/ and keeps its data under target/dev):
  gauntlet check [scope]    reference solution passes, template untouched, clippy-clean
  gauntlet readme           regenerate the problem index in ./README.md",
        MIN_RUST = toolchain::MIN_RUST,
    )
}

const INDEX_START: &str = "<!-- problems:start -->";
const INDEX_END: &str = "<!-- problems:end -->";

/// Every flag any command takes.
const FLAGS: [&str; 8] = ["--yes", "-y", "--delete", "--clean", "--help", "-h", "--version", "-V"];

fn main() -> Result<ExitCode> {
    let hook = color_eyre::config::HookBuilder::default();
    if no_color() { hook.theme(color_eyre::config::Theme::new()) } else { hook }.install()?;
    // The plain-text output is coloured with escape codes, which the
    // Windows console shows as such unless asked not to.
    #[cfg(windows)]
    let _ = ratatui::crossterm::ansi_support::supports_ansi();
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(ok) => Ok(if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }),
        // Not a bug: one plain line.
        Err(e) if e.chain().any(|c| c.is::<UserError>()) => {
            eprintln!("gauntlet: {e:#}");
            Ok(ExitCode::FAILURE)
        }
        Err(e) => Err(e),
    }
}

fn run(args: &[String]) -> Result<bool> {
    let flag = |f: &str| args.iter().any(|a| a == f);
    if let Some(bad) = args.iter().find(|a| a.starts_with('-') && !FLAGS.contains(&a.as_str())) {
        return Err(user(format!("unknown option {bad:?} (see gauntlet --help)")));
    }
    if flag("--version") || flag("-V") {
        println!("gauntlet {VERSION}");
        return Ok(true);
    }
    let command = args.first().map(String::as_str);
    if flag("--help") || flag("-h") || command == Some("help") {
        println!("{}", usage(Dirs::from_env().ok().as_ref()));
        return Ok(true);
    }

    let dirs = Dirs::from_env()?;
    init_logging(&dirs);
    let arg = args.get(1).map(String::as_str).filter(|a| !a.starts_with('-'));
    let yes = flag("--yes") || flag("-y");
    // One gauntlet at a time on a data directory, for anything that may write to it.
    let mut lock = match command {
        Some("check" | "readme") => None,
        _ => Some(dirs::lock(&dirs.data)?),
    };

    let ok = match command {
        None => tui_session(Gauntlet::completionist(None)?, tui::Tab::Arena)?,
        Some("run") => run_command(&args[1..], yes)?,
        Some("review") => review(Gauntlet::review(arg)?, arg, |g| tui_session(g, tui::Tab::Arena))?,
        Some("stats") => tui_session(Gauntlet::completionist(arg)?, tui::Tab::Stats)?,
        Some("reset") => reset(&dirs, yes, lock.take().expect("taken above"))?,
        Some("check") => {
            toolchain::require_rustc()?;
            toolchain::require_clippy()?;
            let ws = Workspace::discover()?;
            let idxs = match arg {
                None => (0..ws.problems.len()).collect(),
                Some(q) => ws.select_many(q)?,
            };
            if idxs.is_empty() {
                return Err(user(format!("no problems to check in {}", show(&ws.roots.content))));
            }
            check(&ws, &idxs)?
        }
        Some("readme") => write_readme_index(&dirs)?,
        Some("__snapshot") => {
            // gauntlet __snapshot [run-]<arena|info|stats> [keys] [WxH]
            // `run-…` resumes the run in progress first (and pauses it after).
            let which = arg.unwrap_or_default();
            let (run, which) = match which.strip_prefix("run-") {
                Some(rest) => (true, rest),
                None => (false, which),
            };
            let tab = match which {
                "stats" => tui::Tab::Stats,
                "info" => tui::Tab::Info,
                _ => tui::Tab::Arena,
            };
            toolchain::require_rustc()?;
            let keys = args.get(2).map_or("", String::as_str);
            let (w, h) = args
                .get(3)
                .and_then(|s| s.split_once('x'))
                .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
                .unwrap_or((120, 40));
            print!("{}", tui::snapshot(tab, w, h, keys, run)?);
            true
        }
        // Anything else is a scope; the workspace says whether it matches.
        Some(scope) => tui_session(Gauntlet::completionist(Some(scope))?, tui::Tab::Arena)?,
    };
    Ok(ok)
}

/// Run the TUI, and stop the run clock when it exits, however it exits.
fn tui_session(mut g: Gauntlet, tab: tui::Tab) -> Result<bool> {
    // Otherwise every save would show the same error.
    toolchain::require_rustc()?;
    let result = tui::run(&mut g, tab);
    let quit = g.quit();
    result.and(quit).map(|()| true)
}

/// `gauntlet review [scope]`: `open` the review session, unless there's
/// nothing to review: then say so, and what review is, and succeed.
fn review(g: Gauntlet, scope: Option<&str>, open: impl FnOnce(Gauntlet) -> Result<bool>) -> Result<bool> {
    if !g.view.is_empty() {
        return open(g);
    }
    let within = match scope {
        Some(q) => format!(" in {}", g.ws.scope_title(&g.ws.resolve_scope(q)?)),
        None => String::new(),
    };
    println!(
        "Nothing to review{within}: review brings back the problems whose latest rating is 1 or 2 on \"{}\", \
         and those whose best solution is well off the reference solution's runtime or memory",
        Rating::STATEMENT
    );
    Ok(true)
}

/// `gauntlet run [scope] | --delete | --clean [n]`
fn run_command(args: &[String], yes: bool) -> Result<bool> {
    let mut g = Gauntlet::load()?;
    let rest: Vec<&str> = args.iter().map(String::as_str).filter(|a| !matches!(*a, "--yes" | "-y")).collect();
    match rest.as_slice() {
        ["--delete"] => {
            let Some(state) = &g.run else {
                println!("No run in progress.");
                return Ok(false);
            };
            let (solved, total) = g.run_progress().unwrap_or_default();
            if !yes
                && !confirm(
                    &format!(
                        "Abandon run {} over {} ({solved}/{total} solved, {} on the clock)?\n\
                         Its files in {} will be deleted. It won't count for PBs; its solves\n\
                         still count as solved. [y/N] ",
                        state.run,
                        g.ws.scope_title(&state.scope),
                        fmt::clock(state.total()),
                        show(&g.ws.run_dir(state.run)),
                    ),
                    "y",
                )?
            {
                println!("Nothing changed.");
                return Ok(false);
            }
            let run = g.delete_run()?;
            println!("Run {run} deleted.");
            Ok(true)
        }
        ["--clean"] => clean_runs(&g, None, yes),
        ["--clean", n] => {
            let n = n.parse().map_err(|_| user("--clean takes a run number, e.g. `gauntlet run --clean 3`"))?;
            clean_runs(&g, Some(n), yes)
        }
        [] => {
            // Before the run starts: it couldn't be tested.
            toolchain::require_rustc()?;
            g.start_or_resume_run(None)?;
            tui_session(g, tui::Tab::Arena)
        }
        [scope] if !scope.starts_with('-') => {
            toolchain::require_rustc()?;
            g.start_or_resume_run(Some(scope))?;
            tui_session(g, tui::Tab::Arena)
        }
        _ => Err(user("usage: gauntlet run [scope] | --delete | --clean [n] (see gauntlet --help)")),
    }
}

/// `gauntlet run --clean [n]`: delete the files of finished run `n`, or of
/// every finished run. Never the run in progress; the log is untouched.
fn clean_runs(g: &Gauntlet, run: Option<u32>, yes: bool) -> Result<bool> {
    let runs = g.cleanable_runs(run)?;
    if runs.is_empty() {
        match run {
            Some(n) => println!("Run {n} has no saved code."),
            None => println!("No finished run has saved code."),
        }
        return Ok(run.is_none());
    }
    let dirs: Vec<String> = runs.iter().map(|&n| show(&g.ws.run_dir(n))).collect();
    if !yes
        && !confirm(
            &format!(
                "Delete the saved code of {} finished run(s): {}?\n\
                 Their times and PBs are kept. [y/N] ",
                runs.len(),
                dirs.join(", ")
            ),
            "y",
        )?
    {
        println!("Nothing changed.");
        return Ok(false);
    }
    g.clean_runs(&runs)?;
    for dir in &dirs {
        println!("Removed {dir}");
    }
    Ok(true)
}

/// Log to a file: the terminal belongs to the UI.
fn init_logging(dirs: &Dirs) {
    if fs::create_dir_all(&dirs.state).is_err() {
        return;
    }
    let Ok(file) = fs::OpenOptions::new().create(true).append(true).open(dirs.log_file()) else {
        return;
    };
    let level = match std::env::var("GAUNTLET_LOG").as_deref() {
        Ok("trace") => tracing::Level::TRACE,
        Ok("debug") => tracing::Level::DEBUG,
        _ => tracing::Level::INFO,
    };
    tracing_subscriber::fmt().with_writer(Mutex::new(file)).with_ansi(false).with_max_level(level).init();
}

fn confirm(prompt: &str, expected: &str) -> io::Result<bool> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut answer = String::new();
    io::stdin().lock().read_line(&mut answer)?;
    Ok(answer.trim().eq_ignore_ascii_case(expected))
}

/// `gauntlet reset`: delete gauntlet's files in the data directory (see
/// [`dirs::reset`]) and the compiled tests. `lock` is this gauntlet's on it.
fn reset(dirs: &Dirs, yes: bool, lock: dirs::Lock) -> Result<bool> {
    let data = show(&dirs.data);
    if !yes
        && !confirm(
            &format!(
                "{RED}{BOLD}Reset{RESET} deletes gauntlet's files in {data}: your working files, the run in\n\
                 progress, and every solve, rating and run. Anything else there is left alone.\n\
                 This can't be undone. Type {BOLD}reset{RESET} to confirm: "
            ),
            "reset",
        )?
    {
        println!("Nothing changed.");
        return Ok(false);
    }
    let removed = dirs::reset(&dirs.data, &dirs.build_dir(), lock).wrap_err_with(|| format!("resetting {data}"))?;
    if removed.is_empty() {
        println!("Nothing to reset in {data}.");
    } else {
        println!("Reset {data}. Run `gauntlet` to start from the top.");
    }
    Ok(true)
}

/// Maintainer check: the reference solution passes, the untouched template reports
/// "not started", both are lint-clean, and nothing references outside sources.
fn check(ws: &Workspace, idxs: &[usize]) -> Result<bool> {
    let mut failures: Vec<(String, String)> = Vec::new();
    for p in idxs.iter().map(|&i| &ws.problems[i]) {
        let env = ws.env();
        let mut problems_here = Vec::new();

        for file in REQUIRED_FILES {
            if !p.dir.join(file).is_file() {
                problems_here.push(format!("{file} is missing"));
            }
        }
        let tests = p.tests();
        let (solution, template) = (p.solution(), p.template());
        let solution_code = p.solution_code().unwrap_or_default();
        let template_code = p.template_code().unwrap_or_default();
        let solution_code = Code { file: &solution, source: &solution_code, tests: &tests };
        let template_code = Code { file: &template, source: &template_code, tests: &tests };
        if problems_here.is_empty() {
            match env.run_tests(&format!("{}-solution", p.id), &solution_code, true) {
                Ok(sol) => match &sol.outcome {
                    Outcome::Passed(_) if sol.runtime > env.timeout / 5 => problems_here
                        .push(format!("{SOLUTION} tests are slow ({})", fmt::runtime(sol.runtime.as_millis() as u64))),
                    Outcome::Passed(_) => {}
                    other => problems_here.push(format!("{SOLUTION}:\n{}", render_outcome(other, env.timeout))),
                },
                Err(e) => problems_here.push(format!("{SOLUTION}: couldn't run the tests: {e}")),
            }
            match env.run_tests(&format!("{}-template", p.id), &template_code, true) {
                Ok(tpl) if matches!(tpl.outcome, Outcome::NotStarted) => {}
                Ok(tpl) => problems_here.push(format!(
                    "{TEMPLATE} should fail every test with {SENTINEL}:\n{}",
                    render_outcome(&tpl.outcome, env.timeout)
                )),
                Err(e) => problems_here.push(format!("{TEMPLATE}: couldn't run the tests: {e}")),
            }
            // Templates are kept warning-free with `#![allow]`, so both are linted.
            for (label, code) in [(SOLUTION, &solution_code), (TEMPLATE, &template_code)] {
                match env.lint(&format!("{}-{label}-lint", p.id), code, true, true) {
                    Ok(lint) if lint.clean => {}
                    Ok(lint) => problems_here.push(format!("{label} has clippy warnings:\n{}", lint.output)),
                    Err(e) => problems_here.push(format!("{label}: couldn't run clippy: {e}")),
                }
            }
        }
        for file in REQUIRED_FILES {
            let text = fs::read_to_string(p.dir.join(file)).unwrap_or_default().to_lowercase();
            if text.contains("leetcode") || text.contains("neetcode") {
                problems_here.push(format!("{file} mentions LeetCode/NeetCode"));
            }
        }
        if !ws.section_of(p).drill && p.complexity.is_empty() {
            problems_here.push("problem.toml needs a complexity (its section isn't a drill)".into());
        }
        let header = p.template_header();
        let first = fs::read_to_string(p.template()).unwrap_or_default().lines().next().map(str::to_string);
        if p.template().is_file() && first.as_deref() != Some(header.as_str()) {
            problems_here.push(format!("{TEMPLATE} line 1 should be exactly:\n{header}"));
        }
        if p.hints.is_empty() {
            problems_here.push("problem.toml has no hints".into());
        }

        if problems_here.is_empty() {
            println!("{GREEN}  ok{RESET}  {}", p.id);
        } else {
            println!("{RED}FAIL{RESET}  {}", p.id);
            failures.push((p.id.clone(), problems_here.join("\n")));
        }
    }
    for (id, msg) in &failures {
        println!("\n{BOLD}── {id} ──{RESET}\n{msg}");
    }
    if failures.is_empty() {
        println!("\n{GREEN}All checks passed.{RESET}");
    } else {
        println!("\n{RED}{} problem(s) need attention.{RESET}", failures.len());
    }
    Ok(failures.is_empty())
}

/// `gauntlet readme`: rewrite the problem index in `./README.md`. Only in a
/// checkout: the links point into the content directory beside it.
fn write_readme_index(dirs: &Dirs) -> Result<bool> {
    if dirs.content_override.is_none() {
        return Err(user(
            "`gauntlet readme` updates a checkout's README.md from its content/: run it there with \
             GAUNTLET_CONTENT_DIR set (cargo run -- readme does that)",
        ));
    }
    let ws = Workspace::discover()?;
    let here = std::env::current_dir()?;
    let path = here.join("README.md");
    let text = fs::read_to_string(&path).wrap_err_with(|| format!("reading {}", path.display()))?;
    let (Some(a), Some(b)) = (text.find(INDEX_START), text.find(INDEX_END)) else {
        return Err(user(format!("{} needs {INDEX_START} and {INDEX_END} markers", path.display())));
    };
    let updated = format!("{}{INDEX_START}\n\n{}\n{}", &text[..a], render_index(&ws, &here), &text[b..]);
    fs::write(&path, updated)?;
    println!("Updated the problem index in README.md ({} problems).", ws.problems.len());
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::perf::Perf;
    use crate::problem::tests::sample;

    #[test]
    fn review_with_nothing_to_review_does_not_open_the_tui() {
        let s = sample("main-review");
        let load = |scope: Option<&str>| {
            let mut g = Gauntlet::with_workspace(Workspace::discover_in(s.0.clone()).unwrap()).unwrap();
            g.narrow_to_review(scope).unwrap();
            g
        };
        assert!(review(load(None), None, |_| panic!("opened the TUI")).unwrap());

        // Rated 1: up for review, but not in the stack section.
        let mut g = load(None);
        let dup = g.idx_of("dup").unwrap();
        g.open(dup).unwrap();
        g.record_pass(dup, Perf::default()).unwrap();
        g.rate(dup, Rating::new(1).unwrap()).unwrap();
        assert!(review(load(Some("stack")), Some("stack"), |_| panic!("opened the TUI")).unwrap());
        let opened = review(load(None), None, |g| {
            assert_eq!(g.view, [dup]);
            Ok(false)
        });
        assert!(!opened.unwrap(), "something to review: the TUI opens");
    }
}
