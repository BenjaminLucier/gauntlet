//! The gauntlet's rules, shared by the TUI and the command line: the two
//! modes, what counts as a solve, and the run clock.
//!
//! * **Completionist** (default): work through problems in
//!   `<data>/<slug>.rs`; each solve asks for a rating. `gauntlet review`
//!   narrows it to the problems up for review (latest rating 1 or 2).
//! * **Run** (`gauntlet run`): a timed pass over a scope in fresh files under
//!   `<data>/runs/<run>/`, kept after the run; no hints, solutions or ratings.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use chrono::{Local, Utc};
use color_eyre::eyre::{Result, bail};

use crate::db::{ALL, Db, Pb, RunStatus, SumOfBest};
use crate::perf::{self, Perf};
use crate::problem::{Problem, Workspace};
use crate::run::RunState;
use crate::store::{Entry, Event, EventLog, Rating};
use crate::ui::user;

/// How often the run clock is written to `run.json` while it runs.
pub const SAVE_EVERY: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Completionist,
    Run,
}

/// A problem's completionist status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Never solved; working file missing or a fresh template.
    Unseen,
    /// Never solved; working file differs from the template.
    InProgress,
    /// Solved at least once, in either mode.
    Solved,
}

/// A scope finished by a run solve, against its PB from other runs.
#[derive(Clone, Debug, PartialEq)]
pub struct Split {
    pub scope: String,
    pub secs: f64,
    pub pb: Option<Pb>,
}

impl Split {
    /// Seconds ahead of the PB (negative: behind). `None` without a PB.
    pub fn ahead_by(&self) -> Option<f64> {
        self.pb.map(|pb| pb.secs - self.secs)
    }
}

/// What a passing test run amounted to.
#[derive(Clone, Debug, PartialEq)]
pub enum Solved {
    /// Completionist solve: prompt for a rating now ([`Gauntlet::rate`]).
    Completionist,
    /// The run's current problem is done; the next is [`Gauntlet::current`].
    Run {
        problem: String,
        secs: f64,
        /// This problem's best in other runs.
        prev_best: Option<Pb>,
        /// Scopes (its section, `all`) this solve completed in the run.
        splits: Vec<Split>,
        /// The last problem: the run is over (`run_finished` logged and
        /// `run.json` deleted; its files are kept).
        finished: bool,
    },
}

pub struct Gauntlet {
    pub ws: Workspace,
    log: EventLog,
    pub db: Db,
    pub mode: Mode,
    /// The problems on show, as indices into `ws.problems`: the
    /// completionist scope or the run's problems, in content order, or the
    /// problems up for review, in review order.
    pub view: Vec<usize>,
    /// A review session (`gauntlet review`): `view` is what was up for review
    /// when it started.
    pub review: bool,
    /// The run in progress, if any — in either mode, but it only ticks in
    /// [`Mode::Run`].
    pub run: Option<RunState>,
    /// Completionist working file differs from the template, per problem.
    touched: Vec<bool>,
    last_tick: Option<Instant>,
    last_save: Instant,
}

impl Gauntlet {
    /// Completionist mode over every problem.
    pub fn load() -> Result<Gauntlet> {
        Self::with_workspace(Workspace::discover()?)
    }

    /// Completionist mode over the problems in `scope` (`trees`,
    /// `three_sum`…; see [`Workspace::resolve_scope`]), or all of them.
    pub fn completionist(scope: Option<&str>) -> Result<Gauntlet> {
        let mut g = Self::load()?;
        if let Some(q) = scope {
            g.view = g.ws.select_many(q)?;
            if g.view.is_empty() {
                return Err(user(format!("{q} has no problems")));
            }
        }
        Ok(g)
    }

    /// `gauntlet review [scope]`: completionist mode over the problems up for
    /// review in `scope` (or everywhere), in review order. The view may be
    /// empty: then there's nothing to review.
    pub fn review(scope: Option<&str>) -> Result<Gauntlet> {
        let mut g = Self::load()?;
        g.narrow_to_review(scope)?;
        Ok(g)
    }

    pub fn narrow_to_review(&mut self, scope: Option<&str>) -> Result<()> {
        let idxs = match scope {
            Some(q) => self.ws.select_many(q)?,
            None => (0..self.ws.problems.len()).collect(),
        };
        self.view = self.review_order(&idxs);
        self.review = true;
        Ok(())
    }

    /// Those of `idxs` up for review, in review order (see [`Db::review_order`]).
    pub fn review_order(&self, idxs: &[usize]) -> Vec<usize> {
        let ids: Vec<&str> = idxs.iter().map(|&i| self.problem(i).id.as_str()).collect();
        self.db.review_order(&ids).into_iter().filter_map(|id| self.idx_of(id)).collect()
    }

    pub fn with_workspace(ws: Workspace) -> Result<Gauntlet> {
        let (log, entries) = EventLog::open(ws.events_file())?;
        let mut db = Db::from_entries(&entries);
        for (id, reference) in perf::cached(&ws) {
            db.set_reference(&id, reference);
        }
        tracing::info!(events = entries.len(), problems = ws.problems.len(), "loaded");
        let mut g = Gauntlet {
            view: (0..ws.problems.len()).collect(),
            review: false,
            touched: Vec::new(),
            ws,
            log,
            db,
            mode: Mode::Completionist,
            run: None,
            last_tick: None,
            last_save: Instant::now(),
        };
        g.touched = (0..g.ws.problems.len()).map(|i| g.compute_touched(i)).collect();
        g.run = g.reconcile_run()?;
        // Every problem solved, but gauntlet stopped before closing the run.
        if let Some(run) = g.run.as_ref().filter(|s| s.is_done()).map(|s| s.run) {
            g.record(Event::RunFinished { run })?;
            g.close_run(false)?;
        }
        g.ws.ensure_rust_project();
        Ok(g)
    }

    /// `run.json`, checked against the log: dropped if its run is over,
    /// rebuilt if the log has a run in progress and the file is gone, caught
    /// up if the log has solves it lacks.
    fn reconcile_run(&self) -> Result<Option<RunState>> {
        let path = self.ws.run_state_file();
        let file = RunState::load(&path).unwrap_or_else(|e| {
            tracing::warn!("ignoring unreadable {}: {e}", path.display());
            None
        });
        let live = self.db.in_progress();
        match (file, live) {
            (Some(mut s), Some(r)) if s.run == r.id => {
                if s.sync_with(r) {
                    tracing::warn!(run = r.id, "run.json behind the log; caught up");
                    s.save(&path)?;
                }
                Ok(Some(s))
            }
            (_, Some(r)) => {
                tracing::warn!(run = r.id, "run.json missing or stale; rebuilding from the log");
                let s = RunState::rebuild(r);
                s.save(&path)?;
                Ok(Some(s))
            }
            (Some(_), None) => {
                fs::remove_file(&path)?;
                Ok(None)
            }
            (None, None) => Ok(None),
        }
    }

    fn record(&mut self, event: Event) -> Result<()> {
        let entry = Entry { at: Utc::now(), event };
        tracing::debug!(?entry, "record");
        self.log.append(&entry)?;
        self.db.apply(&entry);
        Ok(())
    }

    // ---- Problems & files ----------------------------------------------------

    pub fn problem(&self, idx: usize) -> &Problem {
        &self.ws.problems[idx]
    }

    pub fn idx_of(&self, id: &str) -> Option<usize> {
        self.ws.problems.iter().position(|p| p.id == id)
    }

    pub fn in_run(&self) -> bool {
        self.mode == Mode::Run && self.run.is_some()
    }

    /// The file you edit for a problem in the current mode: the run file in
    /// a run, the completionist working file otherwise.
    pub fn working_file(&self, idx: usize) -> PathBuf {
        let p = self.problem(idx);
        match (&self.run, self.mode) {
            (Some(run), Mode::Run) => self.ws.run_file(run.run, p),
            _ => self.ws.attempt(p),
        }
    }

    /// The problem whose working file (in the current mode) is `path`: the
    /// same path, or the same file by another path (a symlink, or on Windows
    /// a `\\?\` path). Paths compare part by part, so `/` and `\` don't matter.
    pub fn problem_of_file(&self, path: &Path) -> Option<usize> {
        let canon = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        if let Some(i) = self.view.iter().copied().find(|&i| self.working_file(i) == path) {
            return Some(i);
        }
        let path = canon(path);
        self.view.iter().copied().find(|&i| canon(&self.working_file(i)) == path)
    }

    /// Get a problem's working file ready: created from the template if missing.
    pub fn open(&mut self, idx: usize) -> Result<PathBuf> {
        let path = self.working_file(idx);
        self.ws.ensure_fresh(self.problem(idx), &path)?;
        Ok(path)
    }

    /// Completionist: overwrite the working file with a fresh template (no
    /// archive). Passing again is another solve.
    pub fn redo(&mut self, idx: usize) -> Result<()> {
        if self.in_run() {
            bail!("no redo in a run");
        }
        let path = self.ws.attempt(self.problem(idx));
        self.ws.write_fresh(self.problem(idx), &path)?;
        self.touched[idx] = false;
        let problem = self.problem(idx).id.clone();
        self.record(Event::Redo { problem })
    }

    fn compute_touched(&self, idx: usize) -> bool {
        let p = self.problem(idx);
        let path = self.ws.attempt(p);
        path.exists() && !self.ws.is_pristine(p, &path)
    }

    /// Re-check a completionist working file after it changed on disk.
    pub fn refresh(&mut self, idx: usize) {
        self.touched[idx] = self.compute_touched(idx);
    }

    pub fn status(&self, idx: usize) -> Status {
        if self.db.is_solved(&self.problem(idx).id) {
            Status::Solved
        } else if self.touched[idx] {
            Status::InProgress
        } else {
            Status::Unseen
        }
    }

    /// Completionist: the first unsolved problem in view; in a review, the
    /// first still up for review. In a run: the current problem.
    pub fn next_up(&self) -> Option<usize> {
        if self.in_run() {
            return self.current();
        }
        if self.review {
            return self.view.iter().copied().find(|&i| self.db.up_for_review(&self.problem(i).id));
        }
        self.view.iter().copied().find(|&i| self.status(i) != Status::Solved)
    }

    // ---- Solving & rating ----------------------------------------------------

    /// Log `perf` if it beats a solved problem's best runtime or memory.
    fn record_improvement(&mut self, id: String, perf: Perf) -> Result<()> {
        let best = self.db.best(&id);
        if self.db.is_solved(&id) && best.best(perf) != best {
            self.record(Event::Improved { problem: id, perf })?;
        }
        Ok(())
    }

    /// Record that problem `idx`'s working file (in the current mode) passed
    /// its tests, which ran as `perf` says. Returns what that amounted to, if
    /// anything: a completionist solve (prompt for a rating), or the run's
    /// current problem solved. Passing a problem already solved in
    /// completionist mode (and not redone since), or a run problem that isn't
    /// current, is no solve; it still counts towards the problem's best
    /// runtime and memory, so fixing a slow solution clears its nudge.
    pub fn record_pass(&mut self, idx: usize, perf: Perf) -> Result<Option<Solved>> {
        let id = self.problem(idx).id.clone();
        if self.mode == Mode::Completionist || self.run.is_none() {
            if !self.db.completionist_armed(&id) {
                self.record_improvement(id, perf)?;
                return Ok(None);
            }
            self.record(Event::Solved { problem: id, run: None, secs: None, perf })?;
            return Ok(Some(Solved::Completionist));
        }

        // Charge the clock up to now first, so the solve gets its last seconds.
        self.tick();
        let Some(state) = self.run.as_mut().filter(|s| s.current == id) else {
            self.record_improvement(id, perf)?;
            return Ok(None);
        };
        let run = state.run;
        let (problem, secs) = state.solve_current().expect("current is in the queue");
        let finished = state.is_done();
        let prev_best = self.db.problem_best(&problem, Some(run));
        self.record(Event::Solved { problem: problem.clone(), run: Some(run), secs: Some(secs), perf })?;
        let splits = self.splits(run, &problem);
        if finished {
            self.record(Event::RunFinished { run })?;
            self.close_run(false)?;
        } else {
            self.save_run()?;
        }
        Ok(Some(Solved::Run { problem, secs, prev_best, splits, finished }))
    }

    /// Scopes containing `problem` that run `run` now covers completely.
    fn splits(&self, run: u32, problem: &str) -> Vec<Split> {
        let Some(p) = self.idx_of(problem).map(|i| self.problem(i)) else { return Vec::new() };
        self.ws
            .scopes()
            .into_iter()
            .filter(|s| self.ws.in_scope(p, s))
            .filter_map(|scope| {
                let ids = self.scope_ids(&scope);
                let secs = self.db.scope_time(run, &ids)?;
                let pb = self.db.pb(&ids, Some(run));
                Some(Split { scope, secs, pb })
            })
            .collect()
    }

    /// Rate the problem's latest completionist solve.
    pub fn rate(&mut self, idx: usize, rating: Rating) -> Result<()> {
        let problem = self.problem(idx).id.clone();
        self.record(Event::Rated { problem, rating })
    }

    /// The first problem whose latest completionist solve is unrated: prompt
    /// for it at startup.
    pub fn awaiting_rating(&self) -> Option<usize> {
        (0..self.ws.problems.len()).find(|&i| self.db.awaiting_rating(&self.problem(i).id))
    }

    // ---- The run -------------------------------------------------------------

    /// Start a run over a scope (`None`: everything), or resume the one in
    /// progress — an error if it's over a different scope. Switches to
    /// [`Mode::Run`] with the clock running.
    pub fn start_or_resume_run(&mut self, scope: Option<&str>) -> Result<()> {
        let wanted = scope.map(|q| self.ws.resolve_scope(q)).transpose()?;
        if let Some(run) = &self.run {
            if let Some(w) = wanted.filter(|w| *w != run.scope) {
                return Err(user(format!(
                    "run {} over {} is in progress; finish it, or abandon it with `gauntlet run --delete`, before \
                     starting one over {w}",
                    run.run, run.scope
                )));
            }
        } else {
            let scope = wanted.unwrap_or_else(|| ALL.to_string());
            let ids = self.scope_ids(&scope);
            self.start_run(&scope, ids)?;
        }
        self.enter_run()
    }

    fn start_run(&mut self, scope: &str, problems: Vec<String>) -> Result<()> {
        if problems.is_empty() {
            return Err(user(format!("{scope} has no problems")));
        }
        let run = self.db.next_run_id();
        self.record(Event::RunStarted { run, scope: scope.to_string(), problems: problems.clone() })?;
        let state = RunState::new(run, scope, &problems);
        state.save(&self.ws.run_state_file())?;
        self.run = Some(state);
        Ok(())
    }

    /// Switch to the run in progress, clock running.
    fn enter_run(&mut self) -> Result<()> {
        let Some(run) = self.run.as_ref().and_then(|s| self.db.run(s.run)) else {
            bail!("no run in progress");
        };
        let mut view: Vec<usize> = run.problems.iter().filter_map(|id| self.idx_of(id)).collect();
        view.sort_unstable();
        // Problems deleted from content since the run began can't be solved.
        let gone: Vec<String> = run.problems.iter().filter(|id| self.idx_of(id).is_none()).cloned().collect();
        self.view = view;
        self.mode = Mode::Run;
        if let Some(s) = self.run.as_mut() {
            s.queue.retain(|id| !gone.contains(id));
            s.normalize();
        }
        if self.run.as_ref().is_some_and(RunState::is_done) {
            let run = self.run.as_ref().map(|s| s.run).expect("checked");
            self.record(Event::RunFinished { run })?;
            return self.close_run(false);
        }
        if let Some(i) = self.current() {
            self.open(i)?;
        }
        self.set_paused(false)
    }

    /// `gauntlet run --delete`: abandon the run in progress, and delete its
    /// files. Its solves still count as completion and activity.
    pub fn delete_run(&mut self) -> Result<u32> {
        let Some(run) = self.run.as_ref().map(|s| s.run) else {
            bail!("no run in progress");
        };
        self.record(Event::RunDeleted { run })?;
        self.close_run(true)?;
        Ok(run)
    }

    /// Forget the run in progress, now over: `run.json`, and with
    /// `delete_files` its files too.
    fn close_run(&mut self, delete_files: bool) -> Result<()> {
        let Some(state) = self.run.take() else { return Ok(()) };
        self.last_tick = None;
        let path = self.ws.run_state_file();
        if path.exists() {
            fs::remove_file(path)?;
        }
        let dir = self.ws.run_dir(state.run);
        if delete_files && dir.exists() {
            fs::remove_dir_all(dir)?;
            self.ws.write_rust_project();
        }
        Ok(())
    }

    /// Runs over with files still on disk: `run`, or every one. Never the
    /// run in progress (an error if `run` is it).
    pub fn cleanable_runs(&self, run: Option<u32>) -> Result<Vec<u32>> {
        let in_progress = self.db.in_progress().map(|r| r.id);
        if let Some(n) = run.filter(|&n| Some(n) == in_progress) {
            return Err(user(format!("run {n} is in progress; abandon it with `gauntlet run --delete` instead")));
        }
        let over = |n: u32| self.db.run(n).is_none_or(|r| r.status() != RunStatus::InProgress);
        Ok(self.ws.run_dirs().into_iter().filter(|&n| run.is_none_or(|r| r == n) && over(n)).collect())
    }

    /// `gauntlet run --clean`: delete the files of runs that are over (see
    /// [`Gauntlet::cleanable_runs`]). Their times and PBs stay in the log.
    pub fn clean_runs(&self, runs: &[u32]) -> Result<()> {
        for &n in runs {
            if self.db.in_progress().is_some_and(|r| r.id == n) {
                bail!("run {n} is in progress");
            }
            fs::remove_dir_all(self.ws.run_dir(n))?;
        }
        self.ws.write_rust_project();
        Ok(())
    }

    /// The problem on screen in the run.
    pub fn current(&self) -> Option<usize> {
        self.run.as_ref().and_then(|s| self.idx_of(&s.current))
    }

    /// Advance the run clock to now: the time since the last tick goes to
    /// the current problem (unless paused). Saves every [`SAVE_EVERY`].
    /// Call it often (every frame/second) in [`Mode::Run`]; a no-op otherwise.
    pub fn tick(&mut self) {
        if self.mode != Mode::Run {
            return;
        }
        let now = Instant::now();
        let Some(state) = self.run.as_mut() else { return };
        if let Some(last) = self.last_tick {
            state.charge(now - last);
        }
        self.last_tick = Some(now);
        if now - self.last_save >= SAVE_EVERY
            && let Err(e) = self.save_run()
        {
            tracing::warn!("saving run state: {e}");
        }
    }

    pub fn save_run(&mut self) -> Result<()> {
        self.last_save = Instant::now();
        if let Some(s) = &self.run {
            s.save(&self.ws.run_state_file())?;
        }
        Ok(())
    }

    /// Pause or resume the run clock, and persist.
    pub fn set_paused(&mut self, paused: bool) -> Result<()> {
        self.tick();
        let Some(s) = self.run.as_mut() else { return Ok(()) };
        s.paused = paused;
        self.save_run()
    }

    pub fn toggle_pause(&mut self) -> Result<()> {
        let paused = self.run.as_ref().is_some_and(|s| s.paused);
        self.set_paused(!paused)
    }

    /// Send the current problem to the back of the queue; returns the new
    /// current problem (its file created if needed). Its time carries over.
    pub fn skip(&mut self) -> Result<Option<usize>> {
        self.tick();
        if let Some(s) = self.run.as_mut() {
            s.skip();
        }
        self.save_run()?;
        let cur = self.current();
        if let Some(i) = cur {
            self.open(i)?;
        }
        Ok(cur)
    }

    /// Leaving the TUI: the run clock stops (paused) and is saved.
    pub fn quit(&mut self) -> Result<()> {
        if self.mode == Mode::Run && self.run.is_some() {
            self.set_paused(true)?;
        }
        Ok(())
    }

    /// (solved, total) in the run in progress.
    pub fn run_progress(&self) -> Option<(usize, usize)> {
        let s = self.run.as_ref()?;
        let total = self.db.run(s.run)?.problems.len();
        Some((total - s.queue.len(), total))
    }

    // ---- Stats ---------------------------------------------------------------

    /// Current problem ids of a canonical scope, in content order.
    pub fn scope_ids(&self, scope: &str) -> Vec<String> {
        self.ws.scope_problems(scope).into_iter().map(|i| self.problem(i).id.clone()).collect()
    }

    /// (solved, total) over a canonical scope.
    pub fn progress(&self, scope: &str) -> (usize, usize) {
        let ids = self.scope_ids(scope);
        (self.db.solved_count(&ids), ids.len())
    }

    pub fn rating_distribution(&self, scope: &str) -> [usize; 5] {
        self.db.rating_distribution(&self.scope_ids(scope))
    }

    /// PB of a canonical scope: the best time of any run that solved all of
    /// its current problems.
    pub fn scope_pb(&self, scope: &str) -> Option<Pb> {
        self.db.pb(&self.scope_ids(scope), None)
    }

    pub fn sum_of_best(&self, scope: &str) -> SumOfBest {
        self.db.sum_of_best(&self.scope_ids(scope))
    }

    /// Time of a canonical scope in run `run`, if it covers all of it.
    pub fn scope_time(&self, run: u32, scope: &str) -> Option<f64> {
        self.db.scope_time(run, &self.scope_ids(scope))
    }

    /// Scopes with PBs a run's scope includes (`all`, sections), for listing
    /// its splits.
    pub fn scopes_within(&self, scope: &str) -> Vec<String> {
        self.ws.scopes().into_iter().filter(|s| self.ws.scope_contains(scope, s)).collect()
    }

    pub fn streak(&self) -> u32 {
        self.db.streak(Local::now().date_naive())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::tests::{Scratch, sample};

    fn load(s: &Scratch) -> Gauntlet {
        Gauntlet::with_workspace(Workspace::discover_in(s.0.clone()).unwrap()).unwrap()
    }

    #[test]
    fn saved_files_are_matched_to_their_problem() {
        let s = sample("core-file");
        let mut g = load(&s);
        let file = g.open(2).unwrap();
        assert_eq!(g.problem_of_file(&file), Some(2));
        // Another spelling of the same file: `.` components, and the file
        // watcher's path before it exists (no canonical form).
        let dotted = g.ws.data_dir().join(".").join(file.file_name().unwrap());
        assert_eq!(g.problem_of_file(&dotted), Some(2));
        let unsaved = g.ws.attempt(g.problem(3));
        assert_eq!(g.problem_of_file(&unsaved), Some(3), "not created yet");
        assert_eq!(g.problem_of_file(&g.ws.data_dir().join("notes.md")), None);
    }

    #[test]
    fn completionist_solve_rate_and_redo() {
        let s = sample("core-comp");
        let mut g = load(&s);
        let file = g.open(0).unwrap();
        assert_eq!(g.status(0), Status::Unseen);
        assert_eq!(g.next_up(), Some(0));
        fs::write(&file, "// mine").unwrap();
        g.refresh(0);
        assert_eq!(g.status(0), Status::InProgress);

        let perf = Perf { ms: Some(84), bytes: Some(3 << 20) };
        assert_eq!(g.record_pass(0, perf).unwrap(), Some(Solved::Completionist));
        assert_eq!(g.status(0), Status::Solved);
        assert_eq!(load(&s).db.best(&g.problem(0).id), perf, "recorded in the log");
        assert_eq!(g.next_up(), Some(1));
        // Unrated survives a restart.
        assert_eq!(load(&s).awaiting_rating(), Some(0));
        // Passing again, even after editing the file, is nothing.
        fs::write(&file, "// mine, tidied").unwrap();
        assert_eq!(g.record_pass(0, Perf::default()).unwrap(), None);
        // Unless it's faster or leaner: then it's the new best, and no solve.
        let faster = Perf { ms: Some(40), bytes: Some(5 << 20) };
        assert_eq!(g.record_pass(0, faster).unwrap(), None);
        assert_eq!(g.record_pass(0, perf).unwrap(), None);
        let best = Perf { ms: Some(40), bytes: Some(3 << 20) };
        assert_eq!(load(&s).db.best(&g.problem(0).id), best, "the improvement is in the log");
        assert_eq!(g.db.history(&g.problem(0).id).len(), 1);
        g.rate(0, Rating::new(4).unwrap()).unwrap();
        assert_eq!(g.awaiting_rating(), None);

        // Redo: a fresh template, and the next pass is another solve.
        g.redo(0).unwrap();
        assert!(g.ws.is_pristine(g.problem(0), &file));
        assert_eq!(g.record_pass(0, Perf::default()).unwrap(), Some(Solved::Completionist));
        assert_eq!(g.db.history(&g.problem(0).id).len(), 2);
        assert_eq!(g.awaiting_rating(), Some(0));
        // The redo survives a restart until the next solve.
        g.rate(0, Rating::new(2).unwrap()).unwrap();
        g.redo(0).unwrap();
        assert_eq!(load(&s).record_pass(0, Perf::default()).unwrap(), Some(Solved::Completionist));
        assert_eq!(load(&s).record_pass(0, Perf::default()).unwrap(), None);
    }

    #[test]
    fn a_run_end_to_end() {
        let s = sample("core-run");
        let mut g = load(&s);
        assert!(g.start_or_resume_run(Some("nope")).is_err());
        g.start_or_resume_run(Some("arrays")).unwrap();
        assert!(g.in_run());
        let (dup, two) = (g.idx_of("dup").unwrap(), g.idx_of("two_sum").unwrap());
        assert_eq!(g.view, [dup, two]);
        assert_eq!(g.current(), Some(dup));
        let file = g.working_file(dup);
        assert!(file.ends_with("data/runs/1/dup.rs") && file.exists());
        assert_eq!(g.run_progress(), Some((0, 2)));

        // Charge some time, skip, and the time carries over.
        g.run.as_mut().unwrap().charge(Duration::from_secs(30));
        assert_eq!(g.skip().unwrap(), Some(two));
        assert!(g.working_file(two).exists());
        // Passing a problem that isn't current is nothing.
        assert_eq!(g.record_pass(dup, Perf::default()).unwrap(), None);
        g.run.as_mut().unwrap().charge(Duration::from_secs(20));
        let Some(Solved::Run { secs, finished, splits, .. }) = g.record_pass(two, Perf::default()).unwrap() else {
            panic!()
        };
        assert!((20.0..21.0).contains(&secs));
        assert!(!finished && splits.is_empty());
        // Run solves count as completion, without a rating.
        assert_eq!(g.status(two), Status::Solved);
        assert_eq!(g.awaiting_rating(), None);

        // Quit (pauses) and resume from disk; a different scope is refused.
        g.quit().unwrap();
        let mut g = load(&s);
        assert!(g.run.as_ref().unwrap().paused);
        assert!(g.start_or_resume_run(Some("stack")).is_err());
        g.start_or_resume_run(None).unwrap();
        assert!(!g.run.as_ref().unwrap().paused);
        assert_eq!(g.current(), Some(dup));
        let Some(Solved::Run { secs, finished, splits, .. }) = g.record_pass(dup, Perf::default()).unwrap() else {
            panic!()
        };
        assert!(secs >= 30.0);
        assert!(finished);
        assert_eq!(splits.iter().map(|s| s.scope.as_str()).collect::<Vec<_>>(), ["arrays"]);
        assert!(g.run.is_none());
        assert!(!g.ws.run_state_file().exists());
        assert!(g.ws.run_file(1, g.problem(dup)).exists(), "a finished run's files are kept");
        assert!(g.scope_pb("arrays").is_some_and(|pb| pb.run == 1));
        assert!(g.scope_pb(ALL).is_none());
    }

    #[test]
    fn delete_a_run_and_grow_its_scope() {
        let s = sample("core-del");
        let mut g = load(&s);
        g.start_or_resume_run(Some("stack")).unwrap();
        assert!(g.ws.run_dir(1).exists());
        assert_eq!(g.delete_run().unwrap(), 1);
        assert!(g.run.is_none() && !g.ws.run_state_file().exists());
        assert!(!g.ws.run_dir(1).exists(), "an abandoned run's files are deleted");

        // A finished run, then a new problem in its scope: no PB until a run
        // solves the whole scope again; the old problem keeps its best.
        g.start_or_resume_run(Some("stack")).unwrap();
        let cur = g.current().unwrap();
        assert!(matches!(g.record_pass(cur, Perf::default()).unwrap(), Some(Solved::Run { finished: true, .. })));
        assert_eq!(g.scope_pb("stack").map(|pb| pb.run), Some(2));
        s.problem("content/03_stack/02_queue", "Queue");
        let g = load(&s);
        assert_eq!(g.scope_pb("stack"), None);
        assert_eq!(g.db.problem_best("min_stack", None).map(|pb| pb.run), Some(2));
    }

    #[test]
    fn review_narrows_to_low_ratings_and_redo_re_rates() {
        let s = sample("core-review");
        let mut g = load(&s);
        g.narrow_to_review(None).unwrap();
        assert!(g.view.is_empty(), "nothing rated yet");

        let mut g = load(&s);
        let [option, dup, two, min] = ["option", "dup", "two_sum", "min_stack"].map(|id| g.idx_of(id).unwrap());
        for (i, r) in [(option, 2), (dup, 5), (two, 1), (min, 2)] {
            g.open(i).unwrap();
            assert_eq!(g.record_pass(i, Perf::default()).unwrap(), Some(Solved::Completionist));
            g.rate(i, Rating::new(r).unwrap()).unwrap();
        }
        let mut g = load(&s);
        g.narrow_to_review(None).unwrap();
        assert_eq!(g.view, [two, option, min], "lowest rating, then least recently solved");
        assert_eq!(g.next_up(), Some(two));
        let mut arrays = load(&s);
        arrays.narrow_to_review(Some("arrays")).unwrap();
        assert_eq!(arrays.view, [two]);
        assert!(load(&s).narrow_to_review(Some("nope")).is_err());

        // Already solved: passing again is nothing until a redo.
        assert_eq!(g.record_pass(two, Perf::default()).unwrap(), None);
        g.redo(two).unwrap();
        assert_eq!(g.record_pass(two, Perf::default()).unwrap(), Some(Solved::Completionist));
        g.rate(two, Rating::new(4).unwrap()).unwrap();
        // It stays in this session's view, but isn't up next, nor next time.
        assert_eq!(g.view, [two, option, min]);
        assert_eq!(g.next_up(), Some(option));
        let mut g = load(&s);
        g.narrow_to_review(None).unwrap();
        assert_eq!(g.view, [option, min]);
    }

    #[test]
    fn clean_keeps_the_run_in_progress() {
        let s = sample("core-clean");
        let mut g = load(&s);
        // Runs 1 and 2 finish; run 3 is in progress.
        for _ in 0..2 {
            g.start_or_resume_run(Some("min_stack")).unwrap();
            let cur = g.current().unwrap();
            g.record_pass(cur, Perf::default()).unwrap();
        }
        g.start_or_resume_run(Some("arrays")).unwrap();
        assert_eq!(g.ws.run_dirs(), [1, 2, 3]);

        assert!(g.cleanable_runs(Some(3)).is_err(), "never the run in progress");
        assert_eq!(g.cleanable_runs(Some(2)).unwrap(), [2]);
        assert_eq!(g.cleanable_runs(Some(9)).unwrap(), Vec::<u32>::new());
        let all = g.cleanable_runs(None).unwrap();
        assert_eq!(all, [1, 2]);
        assert!(g.clean_runs(&[3]).is_err());
        g.clean_runs(&all).unwrap();
        assert_eq!(g.ws.run_dirs(), [3]);
        assert!(g.working_file(g.current().unwrap()).exists());
        // History, times and PBs are untouched.
        assert!(g.scope_pb("min_stack").is_some());
        assert_eq!(g.db.history("min_stack").len(), 2);
        assert_eq!(g.cleanable_runs(None).unwrap(), Vec::<u32>::new());
    }

    #[test]
    fn the_log_wins_over_a_stale_run_file() {
        let s = sample("core-stale");
        let mut g = load(&s);
        g.start_or_resume_run(Some("arrays")).unwrap();
        let (dup, two) = (g.idx_of("dup").unwrap(), g.idx_of("two_sum").unwrap());
        let state_file = g.ws.run_state_file();
        g.run.as_mut().unwrap().charge(Duration::from_secs(30));
        g.save_run().unwrap();
        let stale = fs::read(&state_file).unwrap();
        let Some(Solved::Run { secs, .. }) = g.record_pass(dup, Perf::default()).unwrap() else { panic!() };

        // Gauntlet died after logging the solve, before saving run.json.
        fs::write(&state_file, &stale).unwrap();
        let mut g = load(&s);
        let state = g.run.as_ref().unwrap();
        assert_eq!((state.queue.as_slice(), state.current.as_str()), (&["two_sum".to_string()][..], "two_sum"));
        assert_eq!(state.elapsed["dup"], secs);
        assert_eq!(RunState::load(&state_file).unwrap().as_ref(), Some(state), "saved caught up");
        assert_eq!(g.run_progress(), Some((1, 2)));

        g.start_or_resume_run(None).unwrap();
        let stale = fs::read(&state_file).unwrap();
        let events = fs::read_to_string(g.ws.events_file()).unwrap();
        assert!(matches!(g.record_pass(two, Perf::default()).unwrap(), Some(Solved::Run { finished: true, .. })));
        // ... after logging the end of the run, before removing run.json.
        fs::write(&state_file, &stale).unwrap();
        let g = load(&s);
        assert!(g.run.is_none() && !state_file.exists());
        assert_eq!(g.db.run(1).unwrap().status(), RunStatus::Finished);

        // ... after logging the last solve, before logging the end of the run.
        let solved = fs::read_to_string(g.ws.events_file()).unwrap();
        let (solved, finished) = solved.trim_end().rsplit_once('\n').unwrap();
        assert!(finished.contains("run_finished") && solved.starts_with(&events));
        fs::write(g.ws.events_file(), format!("{solved}\n")).unwrap();
        fs::write(&state_file, &stale).unwrap();
        let g = load(&s);
        assert!(g.run.is_none() && !state_file.exists());
        assert_eq!(g.db.run(1).unwrap().status(), RunStatus::Finished);
        assert_eq!(g.db.run(1).unwrap().solved(), 2);
    }

    #[test]
    fn a_lost_run_file_is_rebuilt_from_the_log() {
        let s = sample("core-rebuild");
        let mut g = load(&s);
        g.start_or_resume_run(None).unwrap();
        let cur = g.current().unwrap();
        g.record_pass(cur, Perf::default()).unwrap();
        fs::remove_file(g.ws.run_state_file()).unwrap();
        let g = load(&s);
        let state = g.run.as_ref().unwrap();
        assert_eq!(state.queue.len(), 3);
        assert!(state.paused);
        assert!(g.ws.run_state_file().exists());
    }
}
