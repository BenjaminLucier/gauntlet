//! The gauntlet's terminal UI.
//!
//! Three tabs: the **Arena** (the problem you're on and its test output),
//! **Info** (the statement, hints revealed one at a time, the target
//! complexity, the tests and, if you must, the solution) and **Stats**
//! (progress, ratings, what's up for review, per-problem history, runs and PBs).
//!
//! Two modes, set up by the caller ([`crate::core::Mode`]):
//!
//! * **Completionist**: browse the problems in view; each solve asks for a
//!   rating ("I found this problem to be trivial.", 1–5). A review session
//!   (`gauntlet review`) is this over the problems rated 1–2.
//! * **Run**: a timed pass over a scope. The run decides which problem is on
//!   screen; `p` pauses, `n` skips to the back of the queue. No hints, no
//!   solution, no ratings (the tests are fine: `t`). Each solve shows its
//!   time and any splits.
//!
//! The working file of the problem on screen is watched: save it in your
//! editor and its tests run (debounced), and a pass is recorded. Other files
//! are tested when you open their problem.
//!
//! A pass shows how long the tests took and their peak memory, against the
//! reference solution's on this machine. That is measured in the background
//! after a problem's first pass (see [`crate::perf`]), and shown once known.

mod arena;
mod info;
mod markdown;
mod stats;
mod theme;

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use ansi_to_tui::IntoText;
use color_eyre::eyre::{Result, bail};
use ratatui::crossterm::event::{self, Event as TermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, TableState, Tabs};
use ratatui::{DefaultTerminal, Frame};
use throbber_widgets_tui::ThrobberState;
use tui_tree_widget::TreeState;

use crate::core::{Gauntlet, Mode, Solved};
use crate::dirs::show;
use crate::fmt;
use crate::perf::{Perf, Reference};
use crate::runner::{self, Cancel, Code, Env, Lint, Outcome, RunResult};
use crate::store::Rating;
use crate::toolchain;
use theme::{Background, accent, muted, star};

/// Editors often write a file several times per save.
const DEBOUNCE: Duration = Duration::from_millis(150);
/// How long a note in the top bar stays.
const NOTICE_FOR: Duration = Duration::from_secs(10);
/// How long a run toast (problem time, splits) stays.
const TOAST_FOR: Duration = Duration::from_secs(12);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Arena,
    Info,
    Stats,
}

impl Tab {
    const ALL: [Tab; 3] = [Tab::Arena, Tab::Info, Tab::Stats];

    fn index(self) -> usize {
        Tab::ALL.iter().position(|&t| t == self).expect("listed")
    }

    /// Tab title, split after its key letter.
    fn label(self) -> (&'static str, &'static str) {
        match self {
            Tab::Arena => ("A", "rena"),
            Tab::Info => ("I", "nfo"),
            Tab::Stats => ("S", "tats"),
        }
    }
}

/// Which pane of the Info tab has focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfoPane {
    Statement,
    Hints,
}

/// What the Info tab's main pane shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfoView {
    Statement,
    /// Never in a run.
    Solution,
    Tests,
}

/// Which pane of the Stats tab has focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatsPane {
    Tree,
    /// The Overview's "Suggested review" table.
    Review,
    /// The table on the right ("Recent solves" on the Overview).
    Details,
}

/// The end of a run, for the screen that sums it up.
pub struct RunOver {
    pub run: u32,
    pub scope: String,
    /// The last problem and its time.
    pub last: (String, f64),
}

enum Popup {
    /// A completionist solve of the problem at this index (not necessarily
    /// the one on screen) waiting for its rating. Only a rating closes it.
    Rate(usize),
    ConfirmRedo,
    RunOver(Box<RunOver>),
    /// Scrolled this far.
    Help(u16),
}

/// How the last test run or lint went, for the output pane's title.
#[derive(Clone, Copy)]
pub enum Verdict {
    Passed(u32),
    Failed { passed: u32, failed: u32 },
    CompileError,
    Timeout,
    NotStarted,
    Lint { clean: bool },
}

impl Verdict {
    fn of(r: &RunResult) -> Verdict {
        match r.outcome {
            Outcome::Passed(_) => Verdict::Passed(r.passed),
            Outcome::Failed(_) => Verdict::Failed { passed: r.passed, failed: r.failed },
            Outcome::CompileError(_) => Verdict::CompileError,
            Outcome::Timeout(_) => Verdict::Timeout,
            Outcome::NotStarted => Verdict::NotStarted,
        }
    }
}

enum Msg {
    Term(TermEvent),
    FileChanged(PathBuf),
    /// A test run of the problem on screen (when it started).
    Tested {
        idx: usize,
        generation: u64,
        result: io::Result<RunResult>,
    },
    Linted {
        idx: usize,
        generation: u64,
        result: io::Result<Lint>,
    },
    /// A reference solution measured (`None`: it didn't pass).
    Measured {
        id: String,
        result: io::Result<Option<Perf>>,
    },
}

pub struct App<'g> {
    g: &'g mut Gauntlet,
    tab: Tab,
    idx: usize,
    tx: Sender<Msg>,
    quit: bool,
    /// Stops the test run or lint in flight when the TUI exits.
    cancel: Cancel,
    /// The threads running them.
    workers: Vec<JoinHandle<()>>,

    // Arena
    output: Text<'static>,
    output_title: &'static str,
    output_scroll: u16,
    verdict: Option<Verdict>,
    /// How the tests of the last run ran, if they passed.
    perf: Option<Perf>,
    busy: Option<&'static str>,
    generation: u64,
    rerun_at: Option<Instant>,
    popup: Option<Popup>,
    throbber: ThrobberState,

    /// A note in the top bar: (text, colour, when).
    notice: Option<(String, Color, Instant)>,
    /// Run solves and splits, shown for a while in the arena.
    toasts: Vec<(Line<'static>, Instant)>,
    /// Problems whose reference solution this session has set out to measure.
    references_asked: HashSet<String>,
    /// How many of those measurements are still running.
    measuring: usize,

    // Info: rendered once per problem.
    statement: Text<'static>,
    hint_texts: Vec<Text<'static>>,
    /// Hints shown so far on this problem (nothing is recorded).
    hints_shown: usize,
    complexity: Text<'static>,
    solution: Text<'static>,
    tests: Text<'static>,
    info_view: InfoView,
    info_focus: InfoPane,
    statement_scroll: u16,
    solution_scroll: u16,
    tests_scroll: u16,
    hints_scroll: u16,

    // Stats
    tree: TreeState<String>,
    stats_focus: StatsPane,
    /// Selection in the details pane's table.
    detail_table: TableState,
    /// The problem behind each row of that table, if rows are problems.
    detail_rows: Vec<usize>,
    /// Tree selection the details table belongs to.
    detail_for: Vec<String>,
    /// The Overview's "Suggested review" table, and the problem behind each row.
    review_table: TableState,
    review_rows: Vec<usize>,
    /// Section pages list problems least trivial first, not in content order.
    least_trivial_first: bool,
}

/// Run the TUI on a loaded gauntlet (completionist, or a run already
/// started/resumed by the caller), starting on `tab`. The caller calls
/// [`Gauntlet::quit`] after.
pub fn run(g: &mut Gauntlet, tab: Tab) -> Result<()> {
    if g.view.is_empty() {
        bail!("no problems found under {}", show(&g.ws.roots.content));
    }
    fs::create_dir_all(g.ws.data_dir())?;

    let (tx, rx) = mpsc::channel();
    let file_tx = tx.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(ev) = res
            && matches!(ev.kind, notify::EventKind::Modify(_) | notify::EventKind::Create(_))
        {
            for path in ev.paths {
                let _ = file_tx.send(Msg::FileChanged(path));
            }
        }
    })?;
    notify::Watcher::watch(&mut watcher, &g.ws.data_dir(), notify::RecursiveMode::Recursive)?;

    // Ask the terminal for its colours before the input thread owns stdin.
    theme::init(Background::detect());
    let input_tx = tx.clone();
    thread::spawn(move || {
        while let Ok(ev) = event::read() {
            if input_tx.send(Msg::Term(ev)).is_err() {
                break;
            }
        }
    });

    let mut app = App::new(g, tx, tab);
    app.start()?;
    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal, &rx);
    ratatui::restore();
    app.stop_workers();
    result
}

impl<'g> App<'g> {
    fn new(g: &'g mut Gauntlet, tx: Sender<Msg>, tab: Tab) -> App<'g> {
        let mut tree = TreeState::default();
        tree.select(vec![stats::OVERVIEW.to_string()]);
        App {
            g,
            tab,
            idx: 0,
            tx,
            quit: false,
            cancel: Cancel::default(),
            workers: Vec::new(),
            output: Text::default(),
            output_title: "Tests",
            output_scroll: 0,
            verdict: None,
            perf: None,
            busy: None,
            generation: 0,
            rerun_at: None,
            popup: None,
            throbber: ThrobberState::default(),
            notice: None,
            toasts: Vec::new(),
            references_asked: HashSet::new(),
            measuring: 0,
            statement: Text::default(),
            hint_texts: Vec::new(),
            hints_shown: 0,
            complexity: Text::default(),
            solution: Text::default(),
            tests: Text::default(),
            info_view: InfoView::Statement,
            info_focus: InfoPane::Statement,
            statement_scroll: 0,
            solution_scroll: 0,
            tests_scroll: 0,
            hints_scroll: 0,
            tree,
            stats_focus: StatsPane::Tree,
            detail_table: TableState::default(),
            detail_rows: Vec::new(),
            detail_for: Vec::new(),
            review_table: TableState::default(),
            review_rows: Vec::new(),
            least_trivial_first: false,
        }
    }

    fn in_run(&self) -> bool {
        self.g.in_run()
    }

    /// Open what's up next (the run's current problem in a run) and ask for
    /// any rating left over from last time.
    fn start(&mut self) -> Result<()> {
        let tab = self.tab;
        let start = self.g.next_up().or_else(|| self.g.view.first().copied()).unwrap_or(0);
        self.open(start)?;
        self.prompt_rating(None);
        self.tab = tab;
        self.tree.select(vec![stats::OVERVIEW.to_string()]);
        Ok(())
    }

    fn run(&mut self, terminal: &mut DefaultTerminal, rx: &Receiver<Msg>) -> Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(msg) => self.handle(msg)?,
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
            while let Ok(msg) = rx.try_recv() {
                self.handle(msg)?;
            }
            self.tick();
        }
        Ok(())
    }

    fn tick(&mut self) {
        let now = Instant::now();
        self.g.tick();
        if self.busy.is_some() {
            self.throbber.calc_next();
        }
        if let Some(at) = self.rerun_at
            && now >= at
            && self.busy.is_none()
        {
            self.rerun_at = None;
            self.start_test();
        }
        if self.notice.as_ref().is_some_and(|&(_, _, at)| now - at >= NOTICE_FOR) {
            self.notice = None;
        }
        self.toasts.retain(|&(_, at)| now - at < TOAST_FOR);
    }

    fn note(&mut self, text: impl Into<String>, colour: Color) {
        self.notice = Some((text.into(), colour, Instant::now()));
    }

    // ── problems ───────────────────────────────────────────────────────────

    /// Show a problem and run its tests: its file may have changed since.
    fn open(&mut self, idx: usize) -> Result<()> {
        self.idx = idx;
        self.g.open(idx)?;
        self.g.refresh(idx);
        let p = self.g.problem(idx);
        self.statement = markdown::render(&fs::read_to_string(p.readme()).unwrap_or_default());
        self.hint_texts = p.hints.iter().map(|h| markdown::render(h)).collect();
        self.complexity = markdown::render(&p.complexity);
        self.solution = markdown::code(&p.solution_source());
        self.tests = markdown::code(fs::read_to_string(p.tests()).unwrap_or_default().trim_end());
        self.hints_shown = 0;
        self.info_view = InfoView::Statement;
        self.statement_scroll = 0;
        self.solution_scroll = 0;
        self.tests_scroll = 0;
        self.hints_scroll = 0;
        self.output = Text::default();
        self.output_scroll = 0;
        self.verdict = None;
        self.perf = None;
        let path = stats::path_to(self.g, idx);
        for depth in 1..path.len() {
            self.tree.open(path[..depth].to_vec());
        }
        self.tree.select(path);
        self.start_test();
        Ok(())
    }

    /// Where this TUI's tests and lints run: stopped by [`App::stop_workers`].
    fn env(&self) -> Env {
        Env { cancel: self.cancel.clone(), ..self.g.ws.env() }
    }

    /// Run `job` on a worker thread. Tests and lints one at a time
    /// ([`App::busy`]), so a test binary is never rebuilt while it may still
    /// be running; reference solutions alongside, under names of their own.
    fn spawn_worker(&mut self, job: impl FnOnce() + Send + 'static) {
        self.workers.retain(|w| !w.is_finished());
        self.workers.push(thread::spawn(job));
    }

    /// At exit: kill the compiler or test binary in flight, if any, and
    /// wait for it to be gone, so an endless loop doesn't outlive gauntlet.
    fn stop_workers(&mut self) {
        self.cancel.cancel();
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }

    fn start_test(&mut self) {
        if self.busy.is_some() {
            self.rerun_at = Some(Instant::now());
            return;
        }
        self.generation += 1;
        let (idx, generation, tx) = (self.idx, self.generation, self.tx.clone());
        self.busy = Some("Compiling and testing");
        let (env, file, name) = (self.env(), self.g.working_file(idx), self.g.problem(idx).id.clone());
        let tests = self.g.problem(idx).tests();
        self.spawn_worker(move || {
            let result = runner::read_source(&file)
                .and_then(|source| env.run_tests(&name, &Code { file: &file, source: &source, tests: &tests }, true));
            let _ = tx.send(Msg::Tested { idx, generation, result });
        });
    }

    fn start_lint(&mut self) {
        if self.busy.is_some() {
            return;
        }
        self.generation += 1;
        let (idx, generation, tx) = (self.idx, self.generation, self.tx.clone());
        self.busy = Some("Running clippy");
        let (env, file, name) = (self.env(), self.g.working_file(idx), format!("{}-lint", self.g.problem(idx).id));
        let tests = self.g.problem(idx).tests();
        self.spawn_worker(move || {
            let result = runner::read_source(&file)
                .and_then(|source| env.lint(&name, &Code { file: &file, source: &source, tests: &tests }, false, true));
            let _ = tx.send(Msg::Linted { idx, generation, result });
        });
    }

    /// Measure problem `idx`'s reference solution, unless that's known or
    /// under way. Its test runs take turns with yours (see [`runner`]).
    fn measure_reference(&mut self, idx: usize) {
        let p = self.g.problem(idx);
        if self.g.db.reference(&p.id).is_some() || !self.references_asked.insert(p.id.clone()) {
            return;
        }
        self.measuring += 1;
        let (env, tx, reference) = (self.env(), self.tx.clone(), Reference::of(&self.g.ws, p));
        self.spawn_worker(move || {
            let result = reference.measure(&env);
            let _ = tx.send(Msg::Measured { id: reference.id, result });
        });
    }

    // ── events ─────────────────────────────────────────────────────────────

    fn handle(&mut self, msg: Msg) -> Result<()> {
        match msg {
            Msg::Term(TermEvent::Key(key)) if key.kind == KeyEventKind::Press => self.on_key(key)?,
            Msg::Term(_) => {}
            Msg::FileChanged(path) => {
                // Only the problem on screen; others are tested when opened.
                if self.g.problem_of_file(&path) == Some(self.idx) {
                    self.g.refresh(self.idx);
                    self.rerun_at = Some(Instant::now() + DEBOUNCE);
                }
            }
            Msg::Tested { idx, generation, result } => {
                self.busy = None;
                // A run of a problem you've moved on from: opening it again tests it again.
                if idx != self.idx || generation != self.generation {
                    return Ok(());
                }
                self.on_tested(result)?;
            }
            Msg::Linted { idx, generation, result } => {
                self.busy = None;
                if idx != self.idx || generation != self.generation {
                    return Ok(());
                }
                // Not installed: say how to install it, and leave the tests on screen.
                if let Err(e) = &result
                    && e.to_string() == toolchain::CLIPPY_MISSING
                {
                    self.note(toolchain::CLIPPY_MISSING, theme::warn());
                    return Ok(());
                }
                self.output_title = "Clippy";
                self.output_scroll = 0;
                match result {
                    Ok(lint) if lint.clean => {
                        self.output = Text::from(vec![
                            Line::default(),
                            Line::from("  ✓ Clippy has no notes on your code.".fg(theme::good()).bold()),
                        ]);
                        self.verdict = Some(Verdict::Lint { clean: true });
                    }
                    Ok(lint) => {
                        self.output = ansi(&lint.output);
                        self.verdict = Some(Verdict::Lint { clean: false });
                    }
                    Err(e) => self.output = Text::from(e.to_string().fg(theme::bad())),
                }
            }
            Msg::Measured { id, result } => {
                self.measuring -= 1;
                match result {
                    Ok(Some(perf)) => self.g.db.set_reference(&id, perf),
                    Ok(None) => {}
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => tracing::warn!(problem = id, "measuring the reference solution: {e}"),
                }
            }
        }
        Ok(())
    }

    fn on_tested(&mut self, result: io::Result<RunResult>) -> Result<()> {
        self.output_title = "Tests";
        let run = match result {
            Ok(run) => run,
            Err(e) => {
                self.output = Text::from(format!("Couldn't run the tests: {e}").fg(theme::bad()));
                return Ok(());
            }
        };
        self.verdict = Some(Verdict::of(&run));
        self.perf = run.outcome.passed().then(|| Perf::of(&run));
        let at_bottom = self.output_scroll as usize + 1 >= self.output.lines.len();
        self.output = match run.outcome {
            Outcome::NotStarted => arena::not_started_text(self.in_run(), &show(&self.g.working_file(self.idx))),
            _ => ansi(run.outcome.output()),
        };
        if at_bottom || run.outcome.passed() {
            self.output_scroll = 0;
        }
        if let Some(perf) = self.perf {
            self.measure_reference(self.idx);
            self.on_pass(self.idx, perf)?;
        }
        Ok(())
    }

    /// Problem `idx`'s working file passed, as `perf` says: record it, and follow up.
    fn on_pass(&mut self, idx: usize, perf: Perf) -> Result<()> {
        let run = self.g.run.as_ref().map(|s| (s.run, s.scope.clone()));
        match self.g.record_pass(idx, perf)? {
            None => {}
            Some(Solved::Completionist) => self.prompt_rating(Some(idx)),
            Some(Solved::Run { problem, secs, prev_best, splits, finished }) => {
                let title = self.g.idx_of(&problem).map_or(problem.clone(), |i| self.g.problem(i).title.clone());
                let mut line =
                    vec!["✓ ".fg(theme::good()).bold(), title.bold(), format!("  {}", fmt::secs(secs)).bold()];
                line.extend(arena::versus(prev_best.map(|pb| pb.secs - secs)));
                let now = Instant::now();
                self.toasts.push((Line::from(line), now));
                if let Some(nudge) = arena::nudge_toast(perf, self.g.db.reference(&problem)) {
                    self.toasts.push((nudge, now));
                }
                for split in &splits {
                    self.toasts.push((arena::split_line(&self.g.ws.scope_title(&split.scope), split), now));
                }
                if finished {
                    let (run, scope) = run.unwrap_or_default();
                    // The run is over: back to completionist mode over everything.
                    self.g.mode = Mode::Completionist;
                    self.g.view = (0..self.g.ws.problems.len()).collect();
                    self.toasts.clear();
                    self.popup = Some(Popup::RunOver(Box::new(RunOver { run, scope, last: (problem, secs) })));
                    let next = self.g.next_up().unwrap_or(idx);
                    self.open(next)?;
                } else if let Some(next) = self.g.current() {
                    self.open(next)?;
                    self.tab = Tab::Arena;
                }
            }
        }
        Ok(())
    }

    /// Ask for the rating of a solve that's waiting for one: `prefer`'s if it
    /// is, else the first. A rating already being asked for stays on screen.
    fn prompt_rating(&mut self, prefer: Option<usize>) {
        if self.in_run() || matches!(self.popup, Some(Popup::Rate(_) | Popup::RunOver(_))) {
            return;
        }
        let idx =
            prefer.filter(|&i| self.g.db.awaiting_rating(&self.g.problem(i).id)).or_else(|| self.g.awaiting_rating());
        if let Some(idx) = idx {
            self.popup = Some(Popup::Rate(idx));
        }
    }

    fn on_key(&mut self, key: KeyEvent) -> Result<()> {
        let code = key.code;
        if code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.quit = true;
            return Ok(());
        }

        // Popups capture input.
        if let Some(popup) = self.popup.take() {
            match popup {
                Popup::Rate(idx) => {
                    // Only a rating closes it: there's no default.
                    if let KeyCode::Char(c) = code
                        && let Some(rating) = Rating::from_key(c)
                    {
                        self.g.rate(idx, rating)?;
                        self.prompt_rating(None);
                    } else {
                        self.popup = Some(Popup::Rate(idx));
                    }
                }
                Popup::ConfirmRedo => {
                    if matches!(code, KeyCode::Char('x' | 'y')) {
                        self.redo()?;
                    }
                }
                Popup::RunOver(over) => {
                    if !matches!(code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ' | 'q')) {
                        self.popup = Some(Popup::RunOver(over));
                    } else {
                        self.prompt_rating(None);
                    }
                }
                // Scroll keys scroll; any other key closes it.
                Popup::Help(mut offset) => {
                    if is_scroll(code) {
                        scroll(&mut offset, code);
                        self.popup = Some(Popup::Help(offset));
                    }
                }
            }
            return Ok(());
        }

        match code {
            KeyCode::Char('q') => self.quit = true,
            KeyCode::Esc => match self.tab {
                Tab::Info if self.info_view != InfoView::Statement => self.info_view = InfoView::Statement,
                Tab::Arena => self.quit = true,
                _ => self.tab = Tab::Arena,
            },
            KeyCode::Char('?') => self.popup = Some(Popup::Help(0)),
            KeyCode::Char('a') => self.tab = Tab::Arena,
            KeyCode::Char('i') => {
                self.tab = Tab::Info;
                self.info_focus = InfoPane::Statement;
            }
            KeyCode::Char('s') => self.tab = Tab::Stats,
            KeyCode::Tab | KeyCode::BackTab => self.cycle_focus(),
            // Not a spoiler: in a run too.
            KeyCode::Char('t') => self.toggle_info_view(InfoView::Tests),
            // In the Stats tree, `h` is left: fold, or up to the section.
            KeyCode::Char('h') if self.tab == Tab::Stats => self.stats_key(code)?,
            _ if self.in_run() => self.run_key(code)?,
            KeyCode::Char('h') => {
                self.tab = Tab::Info;
                self.info_focus = InfoPane::Hints;
            }
            KeyCode::Char('v') => self.toggle_info_view(InfoView::Solution),
            _ => match self.tab {
                Tab::Arena => self.arena_key(code)?,
                Tab::Info => self.info_key(code)?,
                Tab::Stats => self.stats_key(code)?,
            },
        }
        Ok(())
    }

    /// Show `view` in the Info tab, or the statement if it's already showing.
    fn toggle_info_view(&mut self, view: InfoView) {
        if self.tab == Tab::Info && self.info_view == view {
            self.info_view = InfoView::Statement;
        } else {
            self.tab = Tab::Info;
            self.info_focus = InfoPane::Statement;
            self.info_view = view;
        }
    }

    /// Keys in a run, after the ones shared with completionist mode.
    fn run_key(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Char('p') => {
                self.g.toggle_pause()?;
                let paused = self.g.run.as_ref().is_some_and(|s| s.paused);
                self.note(if paused { "⏸ paused" } else { "▶ clock running" }, star());
            }
            KeyCode::Char('n') => {
                let before = self.idx;
                if let Some(next) = self.g.skip()? {
                    if next == before {
                        self.note("Nothing to skip to: this is the last problem", muted());
                    } else {
                        self.note(format!("Skipped {}: back of the queue", self.g.problem(before).title), muted());
                        self.open(next)?;
                    }
                }
            }
            KeyCode::Char('h' | 'v') => self.note("No hints or solutions in a run", theme::warn()),
            KeyCode::Char('x' | 'u') => {}
            _ => match self.tab {
                Tab::Arena => self.arena_key(code)?,
                Tab::Info => self.info_key(code)?,
                Tab::Stats => self.stats_key(code)?,
            },
        }
        Ok(())
    }

    fn cycle_focus(&mut self) {
        match self.tab {
            Tab::Arena => {}
            Tab::Info => {
                if !self.in_run() {
                    self.info_focus = match self.info_focus {
                        InfoPane::Statement => InfoPane::Hints,
                        InfoPane::Hints => InfoPane::Statement,
                    };
                }
            }
            Tab::Stats => {
                // Rows are filled in when drawn: none means the table isn't on screen.
                self.stats_focus = match self.stats_focus {
                    StatsPane::Tree if !self.review_rows.is_empty() => StatsPane::Review,
                    StatsPane::Tree | StatsPane::Review => StatsPane::Details,
                    StatsPane::Details => StatsPane::Tree,
                };
                let table = match self.stats_focus {
                    StatsPane::Review => &mut self.review_table,
                    _ => &mut self.detail_table,
                };
                if table.selected().is_none() {
                    table.select_first();
                }
            }
        }
    }

    /// Completionist: move between the problems in view.
    fn navigate_key(&mut self, code: KeyCode) -> Result<bool> {
        if self.in_run() {
            return Ok(false);
        }
        let view = &self.g.view;
        let pos = view.iter().position(|&i| i == self.idx).unwrap_or(0);
        let target = match code {
            KeyCode::Char('n') | KeyCode::Right => view.get(pos + 1).copied(),
            KeyCode::Char('p') | KeyCode::Left => pos.checked_sub(1).and_then(|p| view.get(p).copied()),
            KeyCode::Char('u') => match self.g.next_up() {
                Some(i) => Some(i),
                None => {
                    let done = if self.g.review { "Nothing left to review" } else { "Everything in view is solved" };
                    self.note(done, theme::good());
                    None
                }
            },
            _ => return Ok(false),
        };
        if let Some(i) = target.filter(|&i| i != self.idx) {
            self.open(i)?;
        }
        Ok(true)
    }

    fn arena_key(&mut self, code: KeyCode) -> Result<()> {
        if self.navigate_key(code)? {
            return Ok(());
        }
        match code {
            KeyCode::Char('r') => self.start_test(),
            KeyCode::Char('c') => self.start_lint(),
            KeyCode::Char('x') if !self.in_run() => self.popup = Some(Popup::ConfirmRedo),
            other => scroll(&mut self.output_scroll, other),
        }
        Ok(())
    }

    fn info_key(&mut self, code: KeyCode) -> Result<()> {
        if self.navigate_key(code)? {
            return Ok(());
        }
        match (self.info_focus, code) {
            (InfoPane::Hints, KeyCode::Enter) => {
                self.hints_shown = (self.hints_shown + 1).min(self.hint_texts.len());
            }
            (InfoPane::Hints, other) => scroll(&mut self.hints_scroll, other),
            (InfoPane::Statement, other) => match self.info_view {
                InfoView::Statement => scroll(&mut self.statement_scroll, other),
                InfoView::Solution => scroll(&mut self.solution_scroll, other),
                InfoView::Tests => scroll(&mut self.tests_scroll, other),
            },
        }
        Ok(())
    }

    /// Whether the Stats tab shows a section page.
    fn on_section_page(&self) -> bool {
        self.tab == Tab::Stats && self.tree.selected().last().is_some_and(|id| stats::is_section(id))
    }

    fn stats_key(&mut self, code: KeyCode) -> Result<()> {
        if code == KeyCode::Char('o') && self.on_section_page() {
            self.least_trivial_first = !self.least_trivial_first;
            return Ok(());
        }
        let table = match self.stats_focus {
            StatsPane::Tree => None,
            StatsPane::Review => Some((&mut self.review_table, &self.review_rows)),
            StatsPane::Details => Some((&mut self.detail_table, &self.detail_rows)),
        };
        if let Some((table, rows)) = table {
            match code {
                KeyCode::Up | KeyCode::Char('k') => table.select_previous(),
                KeyCode::Down | KeyCode::Char('j') => table.select_next(),
                KeyCode::Home | KeyCode::Char('g') => table.select_first(),
                KeyCode::End | KeyCode::Char('G') => table.select_last(),
                KeyCode::Left | KeyCode::Char('h') => self.stats_focus = StatsPane::Tree,
                KeyCode::Enter => {
                    if let Some(idx) = table.selected().and_then(|r| rows.get(r)).copied() {
                        self.open_in_arena(idx)?;
                    }
                }
                _ => {}
            }
            return Ok(());
        }
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.tree.key_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.tree.key_down();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.tree.key_left();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.tree.key_right();
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.tree.select_first();
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.tree.select_last();
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let selected = self.tree.selected().last().cloned();
                match selected.and_then(|id| self.g.idx_of(&id)) {
                    Some(idx) if code == KeyCode::Enter => self.open_in_arena(idx)?,
                    _ => {
                        self.tree.toggle_selected();
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Completionist: open a problem from the stats. In a run the run picks
    /// the problem, so this just goes back to it.
    fn open_in_arena(&mut self, idx: usize) -> Result<()> {
        if !self.in_run() && idx != self.idx && self.g.view.contains(&idx) {
            self.open(idx)?;
        } else if !self.in_run() && !self.g.view.contains(&idx) {
            self.note("That problem isn't in this session's scope", muted());
            return Ok(());
        }
        self.tab = Tab::Arena;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        self.g.redo(self.idx)?;
        self.open(self.idx)?;
        self.note("↻ Fresh template: your next pass counts as another solve", star());
        Ok(())
    }

    // ── drawing ────────────────────────────────────────────────────────────

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(Block::new().style(theme::canvas()), frame.area());
        let [top, body, footer] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

        self.draw_top(frame, top);
        match self.tab {
            Tab::Arena => arena::draw(self, frame, body),
            Tab::Info => info::draw(self, frame, body),
            Tab::Stats => stats::draw(self, frame, body),
        }
        self.draw_footer(frame, footer);

        let in_run = self.in_run();
        match &self.popup {
            Some(Popup::Rate(idx)) => arena::draw_rate(*idx, self, frame),
            Some(Popup::ConfirmRedo) => arena::draw_confirm_redo(self, frame),
            Some(Popup::RunOver(over)) => arena::draw_run_over(over, self, frame),
            Some(Popup::Help(_)) | None => {}
        }
        if let Some(Popup::Help(offset)) = &mut self.popup {
            draw_help(in_run, frame, offset);
        }
        if theme::no_color() {
            theme::monochrome(frame.buffer_mut());
        }
    }

    /// The right of the top bar, in order of priority: the first part is
    /// always shown, each later one, drawn to the left of the one before, only
    /// while there's room. Counts and clocks are padded to their widest, so
    /// nothing moves as they tick.
    fn status(&self) -> Vec<Line<'static>> {
        let g = &*self.g;
        let Some(state) = g.run.as_ref().filter(|_| self.in_run()) else {
            let (solved, total) = (g.db.solved_count(&g.scope_ids(crate::db::ALL)), g.ws.problems.len());
            let mut first = vec![fmt::count(solved, total).bold(), " solved ".fg(muted())];
            if g.review {
                let left = g.view.iter().filter(|&&i| g.db.up_for_review(&g.problem(i).id)).count();
                first.splice(0..0, [fmt::count(left, g.view.len()).bold(), " to review · ".fg(muted())]);
                first.push(Span::styled(" REVIEW ", theme::badge(accent())));
            } else {
                first.push(" GAUNTLET ".fg(accent()).bold());
            }
            let mut parts = vec![Line::from(first)];
            let streak = g.streak();
            if streak > 0 {
                parts.push(Line::from(format!("{streak}-day streak").fg(muted())));
            }
            return parts;
        };
        let (done, total) = g.run_progress().unwrap_or_default();
        let (icon, clock) =
            if state.paused { ("⏸", theme::badge(theme::warn())) } else { ("⏱", Style::new().bold()) };
        let first = Line::from(vec![
            Span::styled(format!("{icon} {:>8} ", fmt::hms(state.total())), clock),
            Span::styled(format!(" RUN {} ", state.run), theme::badge(star())),
        ]);
        let progress = Line::from(vec![fmt::count(done, total).bold(), " solved".fg(muted())]);
        let on = state.time_on(&state.current).as_secs_f64();
        let mut this = vec![format!("{:>8}", fmt::secs(on)).bold(), " on this".fg(muted())];
        match g.db.problem_best(&state.current, Some(state.run)) {
            Some(pb) => {
                let colour = if on > pb.secs { theme::bad() } else { muted() };
                this.push(format!(" / best {}", fmt::secs(pb.secs)).fg(colour));
            }
            None => this.push(" / no best yet".fg(muted())),
        }
        vec![first, progress, Line::from(this)]
    }

    /// Tabs on the left; the status on the right. A notice goes between them,
    /// displacing the status's lesser parts and cut short if it must be;
    /// the tabs and the status's first part always stay whole.
    fn draw_top(&self, frame: &mut Frame, area: Rect) {
        let titles = Tab::ALL.map(|t| {
            let (key, rest) = t.label();
            Line::from(vec![key.underlined(), rest.into()])
        });
        // Each title is padded with a space either side, with one between.
        let tabs_width = titles.iter().map(|t| t.width() + 3).sum::<usize>() - 1;
        let room = usize::from(area.width).saturating_sub(tabs_width);
        let notice = self.notice.as_ref().map(|(text, colour, _)| Line::from(text.clone().fg(*colour).bold()));
        // A notice wants its text and a gap either side.
        let wanted = notice.as_ref().map_or(0, |n| n.width() + 3);
        let mut parts = self.status().into_iter();
        let mut status = parts.next().unwrap_or_default();
        for part in parts {
            let mut wider = part;
            wider.spans.push(" · ".fg(muted()));
            wider.spans.extend(status.spans.iter().cloned());
            if wider.width() + wanted > room {
                break;
            }
            status = wider;
        }
        let status = theme::fit(status, room);
        let status_width = u16::try_from(status.width()).unwrap_or(u16::MAX);
        let tabs_width = u16::try_from(tabs_width).unwrap_or(u16::MAX);
        let [tabs, notice_area, info] =
            Layout::horizontal([Constraint::Length(tabs_width), Constraint::Fill(1), Constraint::Length(status_width)])
                .areas(area);
        frame.render_widget(
            Tabs::new(titles)
                .select(self.tab.index())
                .style(Style::new().fg(muted()))
                .highlight_style(theme::badge(accent()))
                .divider(" ")
                .padding(" ", " "),
            tabs,
        );
        if let Some(notice) = notice {
            let mut notice = theme::fit(notice, usize::from(notice_area.width).saturating_sub(3));
            notice.spans.push("  ".into());
            frame.render_widget(Paragraph::new(notice).right_aligned(), notice_area);
        }
        frame.render_widget(Paragraph::new(status), info);
    }

    /// Key hints, most useful first, as many as fit; `? help` is always at
    /// the right, since it leads to the rest.
    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let help = Line::from(vec!["?".fg(accent()).bold(), " help ".fg(muted())]);
        let room = usize::from(area.width).saturating_sub(help.width());
        let mut hints = Line::from(" ");
        for &(k, what) in self.footer_keys() {
            let hint = [k.fg(accent()).bold(), format!(" {what}   ").fg(muted())];
            if hints.width() + hint.iter().map(Span::width).sum::<usize>() > room {
                break;
            }
            hints.spans.extend(hint);
        }
        let help_width = u16::try_from(help.width()).unwrap_or(u16::MAX);
        let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(help_width)]).areas(area);
        frame.render_widget(Paragraph::new(hints), left);
        frame.render_widget(Paragraph::new(help), right);
    }

    fn footer_keys(&self) -> &'static [(&'static str, &'static str)] {
        let run = self.in_run();
        match self.tab {
            Tab::Arena if run => &[
                ("p", "pause/resume"),
                ("n", "skip"),
                ("r", "rerun"),
                ("c", "clippy"),
                ("i", "info"),
                ("s", "stats"),
                ("q", "quit (pauses)"),
            ],
            Tab::Arena => &[
                ("n/p", "next/prev"),
                ("u", "next up"),
                ("r", "rerun"),
                ("c", "clippy"),
                ("x", "redo"),
                ("i", "info"),
                ("h", "hints"),
                ("s", "stats"),
            ],
            Tab::Info if run && self.info_view == InfoView::Tests => {
                &[("↑↓", "scroll"), ("t/Esc", "back to the statement"), ("p", "pause"), ("n", "skip"), ("a", "arena")]
            }
            Tab::Info if run => &[("↑↓", "scroll"), ("t", "tests"), ("p", "pause"), ("n", "skip"), ("a", "arena")],
            Tab::Info if self.info_view == InfoView::Solution => {
                &[("↑↓", "scroll"), ("v/Esc", "back to the statement"), ("Tab", "hints"), ("a", "arena")]
            }
            Tab::Info if self.info_view == InfoView::Tests => {
                &[("↑↓", "scroll"), ("t/Esc", "back to the statement"), ("Tab", "hints"), ("a", "arena")]
            }
            Tab::Info => match self.info_focus {
                InfoPane::Statement => &[
                    ("↑↓", "scroll"),
                    ("Tab", "hints"),
                    ("v", "solution"),
                    ("t", "tests"),
                    ("n/p", "next/prev"),
                    ("a", "arena"),
                ],
                InfoPane::Hints => &[
                    ("Enter", "show next hint"),
                    ("↑↓", "scroll"),
                    ("Tab", "statement"),
                    ("v", "solution"),
                    ("t", "tests"),
                    ("a", "arena"),
                ],
            },
            Tab::Stats if self.on_section_page() => match self.stats_focus {
                StatsPane::Tree => {
                    &[("↑↓", "move"), ("←→", "fold"), ("Tab", "details"), ("o", "order"), ("a", "arena")]
                }
                _ => &[("↑↓", "select"), ("Enter", "open"), ("Tab", "tree"), ("o", "order"), ("a", "arena")],
            },
            Tab::Stats => match self.stats_focus {
                StatsPane::Tree => {
                    &[("↑↓", "move"), ("←→", "fold"), ("Enter", "open"), ("Tab", "details"), ("a", "arena")]
                }
                StatsPane::Review => &[("↑↓", "select"), ("Enter", "open"), ("Tab", "recent solves"), ("a", "arena")],
                StatsPane::Details => &[("↑↓", "select"), ("Enter", "open"), ("Tab", "tree"), ("a", "arena")],
            },
        }
    }
}

/// Scroll keys shared by every scrollable pane. Clamped when drawn.
fn scroll(offset: &mut u16, code: KeyCode) {
    *offset = match code {
        KeyCode::Up | KeyCode::Char('k') => offset.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') => offset.saturating_add(1),
        KeyCode::PageUp => offset.saturating_sub(10),
        KeyCode::PageDown | KeyCode::Char(' ') => offset.saturating_add(10),
        KeyCode::Home | KeyCode::Char('g') => 0,
        KeyCode::End | KeyCode::Char('G') => u16::MAX,
        _ => *offset,
    };
}

/// Whether `code` is one of [`scroll`]'s keys.
fn is_scroll(code: KeyCode) -> bool {
    let mut probe = 1;
    scroll(&mut probe, code);
    probe != 1
}

fn ansi(s: &str) -> Text<'static> {
    // libtest resets colours with terminfo's `sgr0`, which for xterm-256color
    // (Terminal.app, iTerm2, VS Code) starts with `ESC ( B`, "select the ASCII
    // character set". That's a no-op here, but ansi-to-tui would show `(B`.
    let s = s.replace("\x1b(B", "");
    theme::recolor(s.into_text().unwrap_or_else(|_| Text::from(s.clone())))
}

/// Every key, in a popup that scrolls when the terminal is too short for it.
fn draw_help(in_run: bool, frame: &mut Frame, offset: &mut u16) {
    let section = |s: &str| Line::from(format!(" {s}").bold().fg(accent()));
    let key = |k: &str, what: &str| {
        Line::from(vec![format!("   {k:<16}").fg(theme::text()).bold(), what.to_string().fg(theme::subtext())])
    };
    let note = |s: &str| Line::from(format!("   {s}").fg(muted()));
    let mut lines = vec![
        section("Everywhere"),
        key("a · i · s", "Arena · Info · Stats"),
        key("Tab", "move focus between panes"),
        key("?", "this help"),
        key("q / Ctrl-C", "quit (Esc: back to the arena, then quit)"),
        note("Saving the problem on screen in your editor runs its tests; a"),
        note("pass is recorded. Other problems are tested when you open them."),
        Line::default(),
    ];
    if in_run {
        lines.extend([
            section("In a run"),
            key("p", "pause / resume the clock (quitting pauses too)"),
            key("n", "skip: send this problem to the back of the queue"),
            key("r · c", "Arena: re-run the tests (saving does this too) · clippy"),
            key("t", "the problem's tests (Info tab)"),
            key("↑↓ PgUp PgDn", "scroll"),
            note("Every unpaused second counts toward the problem on screen;"),
            note("time on a skipped problem carries over when it comes back."),
            note("Passing the tests solves it and the next one comes up."),
            note("No hints, no solution and no ratings in a run."),
            note("The run ends when every problem is solved."),
        ]);
    } else {
        lines.extend([
            section("Arena and Info"),
            key("n / → · p / ←", "next / previous problem"),
            key("u", "next up: first unsolved (in a review: next to review)"),
            key("h · v · t", "hints · the reference solution · the tests"),
            key("Enter", "show the next hint (Hints pane)"),
            key("↑↓ PgUp PgDn", "scroll"),
            Line::default(),
            section("Arena"),
            key("r · c", "re-run the tests (saving does this too) · clippy"),
            key("x", "redo: reset the file to a fresh template"),
            note("A pass shows the tests' runtime and peak memory against the"),
            note("reference solution's, measured on this machine. Speed never"),
            note("fails a solution; well off its pace, you'll get a nudge (▲)."),
            Line::default(),
            section("After a solve"),
            key("1 – 5", "rate \"I found this problem to be trivial.\""),
            note("1 strongly disagree … 5 strongly agree. Required: nothing"),
            note("else closes it. Passing a solved problem again doesn't count"),
            note("until you redo it (x)."),
            note("`gauntlet review [scope]` brings back what you rated 1–2: redo"),
            note("it (x), pass, and rate it again. 3 or more and it's reviewed."),
            note("Timed runs: `gauntlet run [scope]`, e.g. `gauntlet run trees`."),
        ]);
    }
    lines.extend([
        Line::default(),
        section("Stats"),
        key("↑↓ · ←→", "move · fold / unfold; ← on a problem: its section"),
        key("j k · h l", "the same, vim-style"),
        key("Enter", "open the selected problem in the arena"),
        key("o", "section page: content order / least trivial first"),
        note("Overview (suggested review, recent solves), Runs (PBs, sum"),
        note("of best), and a page per section and problem. ▲ marks a problem"),
        note("whose best solution is well off the reference's runtime or"),
        note("memory; it's suggested for review too."),
    ]);
    let height = u16::try_from(lines.len()).unwrap_or(u16::MAX).saturating_add(2);
    let area = theme::popup(frame, 76, height);
    let paragraph = Paragraph::new(lines);
    let max = height.saturating_sub(area.height);
    *offset = (*offset).min(max);
    let hint = if max == 0 {
        " any key closes ".to_string()
    } else if *offset < max {
        format!(" ↓ {} more · j k PgUp PgDn scroll · any other key closes ", max - *offset)
    } else {
        " ↑ j k PgUp PgDn scroll · any other key closes ".to_string()
    };
    let block = Block::bordered()
        .title(" Help ".fg(accent()).bold())
        .title_bottom(Line::from(hint.fg(muted())).right_aligned())
        .border_style(Style::new().fg(accent()));
    frame.render_widget(paragraph.scroll((*offset, 0)).block(block), area);
    if max > 0 {
        let mut state = ScrollbarState::new(usize::from(max)).position(usize::from(*offset));
        frame.render_stateful_widget(Scrollbar::new(ScrollbarOrientation::VerticalRight), area, &mut state);
    }
}

/// Render one frame to text, headlessly: open what's up next, wait for its
/// tests, replay `keys`, and print the screen. With `run`, resume the run in
/// progress first (an error if there isn't one); it's paused again after.
/// Used for development and docs.
pub fn snapshot(tab: Tab, width: u16, height: u16, keys: &str, run: bool) -> Result<String> {
    let mut g = Gauntlet::load()?;
    if run {
        if g.run.is_none() {
            bail!("no run in progress to snapshot");
        }
        g.start_or_resume_run(None)?;
    }
    fs::create_dir_all(g.ws.data_dir())?;
    let (tx, rx) = mpsc::channel();
    theme::init(Background::detect());
    let mut app = App::new(&mut g, tx, tab);
    app.start()?;
    let settle = |app: &mut App, rx: &Receiver<Msg>| -> Result<()> {
        while app.busy.is_some() || app.measuring > 0 {
            match rx.recv_timeout(Duration::from_secs(60)) {
                Ok(msg) => app.handle(msg)?,
                Err(_) => break,
            }
        }
        Ok(())
    };
    settle(&mut app, &rx)?;
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height))?;
    for c in keys.chars() {
        // Widgets like the tree learn their layout from the previous frame.
        terminal.draw(|frame| app.draw(frame))?;
        let code = match c {
            '\n' | '⏎' => KeyCode::Enter,
            '\t' | '⇥' => KeyCode::Tab,
            '←' => KeyCode::Left,
            '→' => KeyCode::Right,
            '↑' => KeyCode::Up,
            '↓' => KeyCode::Down,
            '⎋' => KeyCode::Esc,
            c => KeyCode::Char(c),
        };
        app.on_key(KeyEvent::new(code, KeyModifiers::NONE))?;
        settle(&mut app, &rx)?;
    }
    terminal.draw(|frame| app.draw(frame))?;
    let buffer = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            out.push_str(buffer[(x, y)].symbol());
        }
        out.push('\n');
    }
    drop(app);
    g.quit()?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::style::Modifier;

    use super::*;
    use crate::problem::Workspace;
    use crate::problem::tests::{Scratch, sample};

    const SIZES: [(u16, u16); 3] = [(80, 24), (100, 30), (160, 50)];

    #[test]
    fn terminfo_sgr0_leaves_no_charset_escape_behind() {
        let text = ansi("test a ... \x1b[32mok\x1b(B\x1b[m\n");
        let shown: String = text.lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(shown, "test a ... ok");
    }

    fn load(s: &Scratch, run: bool) -> Gauntlet {
        let mut g = Gauntlet::with_workspace(Workspace::discover_in(s.0.clone()).unwrap()).unwrap();
        if run {
            g.start_or_resume_run(None).unwrap();
        }
        g
    }

    fn draw(app: &mut App, w: u16, h: u16) -> Buffer {
        let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
        terminal.draw(|frame| app.draw(frame)).unwrap();
        terminal.backend().buffer().clone()
    }

    /// The screen as lines, one char per cell; printed with `GAUNTLET_SHOW` set.
    fn screen(app: &mut App, w: u16, h: u16) -> Vec<String> {
        let buffer = draw(app, w, h);
        let rows: Vec<String> =
            (0..h).map(|y| (0..w).map(|x| buffer[(x, y)].symbol().chars().next().unwrap_or(' ')).collect()).collect();
        if std::env::var_os("GAUNTLET_SHOW").is_some() {
            println!("{w}x{h}\n{}\n", rows.join("\n"));
        }
        rows
    }

    /// The column `text` starts at in `row`.
    fn col(row: &str, text: &str) -> usize {
        row[..row.find(text).unwrap_or_else(|| panic!("{text:?} not in {row:?}"))].chars().count()
    }

    fn press(app: &mut App, code: KeyCode) {
        app.on_key(KeyEvent::new(code, KeyModifiers::NONE)).unwrap();
    }

    /// Every Stats page, by tree selection.
    fn stats_pages() -> [Vec<String>; 4] {
        let s = |v: &[&str]| v.iter().map(ToString::to_string).collect();
        [s(&[stats::OVERVIEW]), s(&[stats::RUNS]), s(&["s:arrays"]), s(&["s:arrays", "dup"])]
    }

    #[test]
    fn every_view_keeps_its_tabs_status_and_help_at_every_size() {
        let s = sample("tui-views");
        for run in [false, true] {
            let mut g = load(&s, run);
            let (tx, _rx) = mpsc::channel();
            let mut app = App::new(&mut g, tx, Tab::Arena);
            let badge = if run { "RUN 1" } else { "GAUNTLET" };
            for (w, h) in SIZES {
                let mut screens = vec![];
                for tab in [Tab::Arena, Tab::Info] {
                    app.tab = tab;
                    screens.push(screen(&mut app, w, h));
                }
                app.tab = Tab::Stats;
                for page in stats_pages() {
                    app.tree.select(page);
                    screens.push(screen(&mut app, w, h));
                }
                for rows in &screens {
                    let all = rows.join("\n");
                    assert!(rows[0].starts_with(" Arena   Info   Stats "), "{w}x{h}: tabs\n{all}");
                    assert!(rows[0].trim_end().ends_with(badge), "{w}x{h}: status\n{all}");
                    assert!(rows[usize::from(h) - 1].ends_with("? help "), "{w}x{h}: help\n{all}");
                }
                // The section table has room for its problems, and the runs table for its scopes.
                assert!(screens[4].iter().any(|r| r.contains("Two Sum")), "{w}x{h}\n{}", screens[4].join("\n"));
                if run {
                    let row = screens[3].iter().find(|r| r.contains("in progress")).unwrap();
                    assert!(row.contains("Everything"), "{}", screens[3].join("\n"));
                }
            }
        }
    }

    #[test]
    fn clocks_and_counts_keep_their_place() {
        let s = sample("tui-clocks");
        let mut g = load(&s, true);
        let current = g.run.as_ref().unwrap().current.clone();
        let (tx, _rx) = mpsc::channel();
        let mut app = App::new(&mut g, tx, Tab::Arena);
        for (before, after) in [(599.0, 600.0), (3599.0, 3600.0), (35_999.0, 36_000.0)] {
            for (w, h) in SIZES {
                let mut at = |secs: f64| {
                    app.g.run.as_mut().unwrap().elapsed.insert(current.clone(), secs);
                    app.tab = Tab::Arena;
                    let arena = screen(&mut app, w, h);
                    app.tab = Tab::Stats;
                    app.tree.select(vec![stats::RUNS.to_string()]);
                    let runs = screen(&mut app, w, h);
                    let mut cols = vec![col(&arena[0], "solved"), col(&arena[0], "RUN"), col(&arena[2], "on this")];
                    if w >= 100 {
                        cols.push(col(&arena[0], "on this"));
                    }
                    let progress = runs.iter().find(|r| r.contains("Progress")).unwrap();
                    let current = runs.iter().find(|r| r.contains("Current")).unwrap();
                    cols.extend([col(progress, "solved"), col(current, "Option")]);
                    cols
                };
                assert_eq!(at(before), at(after), "{w}x{h}: {before}s to {after}s");
            }
        }
    }

    #[test]
    fn notices_yield_to_the_tabs_and_the_clock_and_show_they_were_cut() {
        let s = sample("tui-notice");
        for run in [false, true] {
            let mut g = load(&s, run);
            let (tx, _rx) = mpsc::channel();
            let mut app = App::new(&mut g, tx, Tab::Arena);
            app.note("Skipped Construct Binary Tree from Preorder and Inorder Traversal: back of the queue", muted());
            let rows = screen(&mut app, 80, 24);
            assert!(rows[0].starts_with(" Arena   Info   Stats  Skipped Construct"), "{}", rows[0]);
            assert!(rows[0].contains('…'), "{}", rows[0]);
            assert!(
                rows[0].trim_end().ends_with(if run { "0:00:00  RUN 1" } else { "solved  GAUNTLET" }),
                "{}",
                rows[0]
            );
            // A short one leaves the rest of the status be.
            app.note("⏸ paused", muted());
            let rows = screen(&mut app, 80, 24);
            assert!(rows[0].contains("⏸ paused  ") && !rows[0].contains('…'), "{}", rows[0]);
        }
    }

    #[test]
    fn the_footer_always_offers_help() {
        let s = sample("tui-footer");
        for run in [false, true] {
            let mut g = load(&s, run);
            let (tx, _rx) = mpsc::channel();
            let mut app = App::new(&mut g, tx, Tab::Arena);
            for tab in Tab::ALL {
                app.tab = tab;
                for view in [InfoView::Statement, InfoView::Solution, InfoView::Tests] {
                    for (info, stats) in [(InfoPane::Statement, StatsPane::Tree), (InfoPane::Hints, StatsPane::Details)]
                    {
                        (app.info_view, app.info_focus, app.stats_focus) = (view, info, stats);
                        let rows = screen(&mut app, 80, 24);
                        assert!(rows[23].ends_with("? help "), "{tab:?} {view:?}: {}", rows[23]);
                    }
                }
            }
        }
    }

    #[test]
    fn help_scrolls_when_the_terminal_is_short() {
        let s = sample("tui-help");
        let mut g = load(&s, false);
        let (tx, _rx) = mpsc::channel();
        let mut app = App::new(&mut g, tx, Tab::Arena);
        let last = "suggested for review too";
        press(&mut app, KeyCode::Char('?'));
        let rows = screen(&mut app, 80, 24);
        assert!(rows[23].contains(" more · j k PgUp PgDn scroll") && !rows.iter().any(|r| r.contains(last)));
        press(&mut app, KeyCode::Char('j'));
        assert!(screen(&mut app, 80, 24)[1].contains("Arena · Info · Stats"), "one line down");
        press(&mut app, KeyCode::End);
        let rows = screen(&mut app, 80, 24);
        assert!(rows.iter().any(|r| r.contains(last)) && !rows[23].contains("more"), "{}", rows.join("\n"));
        // Any other key closes it.
        press(&mut app, KeyCode::Char('x'));
        assert!(app.popup.is_none());
        // With room, it all fits.
        press(&mut app, KeyCode::Char('?'));
        let rows = screen(&mut app, 160, 50);
        assert!(rows.iter().any(|r| r.contains(last)) && rows.iter().any(|r| r.contains("any key closes")));
    }

    #[test]
    fn h_in_stats_goes_up_the_tree() {
        let s = sample("tui-keys");
        for run in [false, true] {
            let mut g = load(&s, run);
            let (tx, _rx) = mpsc::channel();
            let mut app = App::new(&mut g, tx, Tab::Stats);
            let section = vec!["s:arrays".to_string()];
            app.tree.open(section.clone());
            app.tree.select(vec!["s:arrays".to_string(), "dup".to_string()]);
            press(&mut app, KeyCode::Char('h'));
            assert_eq!((app.tab, app.tree.selected()), (Tab::Stats, &section[..]));
            assert!(app.notice.is_none(), "no 'no hints in a run' in the stats");
            app.stats_focus = StatsPane::Details;
            press(&mut app, KeyCode::Char('h'));
            assert_eq!(app.stats_focus, StatsPane::Tree);
            // Elsewhere, h is still the hints.
            app.tab = Tab::Arena;
            press(&mut app, KeyCode::Char('h'));
            assert_eq!(app.tab, if run { Tab::Arena } else { Tab::Info });
        }
    }

    const MB: u64 = 1 << 20;

    fn perf(ms: u64, mb: u64) -> Perf {
        Perf { ms: Some(ms), bytes: Some(mb * MB) }
    }

    /// The row containing `text`, and its index.
    fn row_of<'a>(rows: &'a [String], text: &str) -> (usize, &'a str) {
        let i = rows.iter().position(|r| r.contains(text)).unwrap_or_else(|| panic!("{text:?}:\n{}", rows.join("\n")));
        (i, &rows[i])
    }

    #[test]
    fn a_pass_shows_runtime_and_memory_against_the_reference_without_moving() {
        let s = sample("tui-perf");
        for (mine, nudged) in [(perf(95, 3), false), (perf(4_210, 410), true)] {
            let mut g = load(&s, false);
            let two = g.idx_of("two_sum").unwrap();
            let (tx, _rx) = mpsc::channel();
            let mut app = App::new(&mut g, tx, Tab::Arena);
            app.idx = two;
            app.verdict = Some(Verdict::Passed(17));
            app.perf = Some(mine);
            app.output = Text::from("running 17 tests\ntest tests::a ... ok\n\ntest result: ok. 17 passed; 0 failed;");
            let tests = fmt::runtime(mine.ms.unwrap());
            let mut before = Vec::new();
            for (w, h) in SIZES {
                let rows = screen(&mut app, w, h);
                let (at, row) = row_of(&rows, "│ Tests");
                assert!(
                    row.contains(&tests) && !rows.iter().any(|r| r.contains("reference")),
                    "{w}x{h}: not yet known"
                );
                before.push((at, col(row, &tests), row_of(&rows, "running 17 tests").0, row_of(&rows, "Memory").0));
            }
            app.g.db.set_reference("two_sum", perf(80, 12));
            for ((w, h), before) in SIZES.into_iter().zip(before) {
                let rows = screen(&mut app, w, h);
                let (at, row) = row_of(&rows, "│ Tests");
                let after = (at, col(row, &tests), row_of(&rows, "running 17 tests").0, row_of(&rows, "Memory").0);
                assert_eq!(before, after, "{w}x{h}: nothing moves when the reference arrives");
                assert!(row.contains("reference  80 ms"), "{w}x{h}: {row}");
                assert!(row_of(&rows, "Memory").1.contains("reference  12 MB"), "{w}x{h}");
                let all = rows.join("\n");
                assert_eq!(all.contains("▲ this could probably be faster"), nudged, "{w}x{h}\n{all}");
                assert_eq!(all.contains("▲ this could probably use less memory"), nudged, "{w}x{h}\n{all}");
                if nudged {
                    assert!(row.contains(" 53×"), "{w}x{h}: {row}");
                } else {
                    assert!(row.contains("1.2×") && !all.contains('▲'), "{w}x{h}: {row}");
                }
            }
            // Not after clippy, nor a failure.
            app.verdict = Some(Verdict::Lint { clean: true });
            assert!(!screen(&mut app, 80, 24).iter().any(|r| r.contains("│ Tests")));
        }
    }

    #[test]
    fn stats_mark_and_compare_your_best_with_the_reference() {
        let s = sample("tui-perf-stats");
        let mut g = load(&s, false);
        let [dup, two] = ["dup", "two_sum"].map(|id| g.idx_of(id).unwrap());
        // Dup: found trivial, but slower than the reference (by 12×). Two Sum: on the pace.
        for (idx, perf, rating) in [(dup, perf(3_400, 41), 5), (two, perf(90, 3), 4)] {
            g.record_pass(idx, perf).unwrap();
            g.rate(idx, Rating::new(rating).unwrap()).unwrap();
        }
        g.redo(dup).unwrap();
        g.record_pass(dup, perf(2_900, 60)).unwrap();
        g.rate(dup, Rating::new(4).unwrap()).unwrap();
        g.db.set_reference("dup", perf(240, 12));
        g.db.set_reference("two_sum", perf(80, 3));
        let (tx, _rx) = mpsc::channel();
        let mut app = App::new(&mut g, tx, Tab::Stats);
        let page = |id: &str| vec!["s:arrays".to_string(), id.to_string()];
        app.tree.open(vec!["s:arrays".to_string()]);
        for (w, h) in SIZES {
            app.tree.select(page("dup"));
            let rows = screen(&mut app, w, h);
            let all = rows.join("\n");
            assert!(row_of(&rows, "Best time").1.contains("2.90 s · 12× the ref ▲"), "{w}x{h}\n{all}");
            // 3.4× the memory, but only by 29 MB: no mark.
            assert!(row_of(&rows, "Best memory").1.contains("41 MB · 3.4× the ref") && !all.contains("ref ▲\n"));
            assert!(row_of(&rows, "Reference").1.contains("240 ms ·  12 MB"), "{w}x{h}\n{all}");
            assert!(all.contains("▲ could probably be faster"), "{w}x{h}\n{all}");
            assert!(row_of(&rows, "#2").1.contains("2.90 s ▲") && row_of(&rows, "#1").1.contains("3.40 s ▲"));
            // The tree marks it; the section table too.
            assert!(rows.iter().any(|r| r.contains("▲ Dup")) && rows.iter().any(|r| r.contains("✓ Two Sum")));
            app.tree.select(vec!["s:arrays".to_string()]);
            let rows = screen(&mut app, w, h);
            let tree = usize::from(stats::tree_width(Rect::new(0, 0, w, h)));
            let table: Vec<String> = rows.iter().map(|r| r.chars().skip(tree).collect()).collect();
            assert!(row_of(&table, "Dup").1.contains("▲ Dup"), "{w}x{h}\n{}", table.join("\n"));
            // Suggested for review, saying why.
            app.tree.select(vec![stats::OVERVIEW.to_string()]);
            let rows = screen(&mut app, w, h);
            let (review, _) = row_of(&rows, "Suggested review (1)");
            assert!(rows[review..].iter().any(|r| r.contains("Dup") && r.contains("▲ could be faster")));
            // Two Sum is fine.
            app.tree.select(page("two_sum"));
            let rows = screen(&mut app, w, h);
            let best = row_of(&rows, "Best time").1;
            assert!(best.contains("90 ms · 1.1× the ref") && !best.contains('▲') && !rows.join("").contains("could"));
        }
    }

    #[test]
    fn without_colour_selection_and_secondary_text_still_show() {
        let s = sample("tui-mono");
        let mut g = load(&s, false);
        let (tx, _rx) = mpsc::channel();
        let mut app = App::new(&mut g, tx, Tab::Stats);
        let mut buffer = draw(&mut app, 80, 24);
        theme::monochrome(&mut buffer);
        assert!(buffer.content.iter().all(|c| c.fg == Color::Reset && c.bg == Color::Reset));
        let modifier = |x, y| buffer[(x, y)].modifier;
        // The active tab, the tree's selection (★ Overview), a footer label.
        assert!(modifier(18, 0).contains(Modifier::REVERSED), "Stats tab");
        assert!(modifier(5, 2).contains(Modifier::REVERSED), "selected row");
        assert!(modifier(4, 23).contains(Modifier::DIM), "footer label");
        assert!(!modifier(4, 3).contains(Modifier::REVERSED), "unselected row");
    }
}
