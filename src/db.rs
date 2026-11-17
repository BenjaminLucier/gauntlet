//! Everything derived from the event log: solves, ratings, runs, PBs, best
//! runtimes and memory. Plus, from outside the log, the reference solutions'
//! measurements on this machine, which they're compared with.
//!
//! Rebuilt from scratch on startup by replaying `<data>/events.jsonl`, and kept
//! current by applying each new event as it is written — the same code path
//! either way. Nothing here touches the disk or knows about content: scopes
//! are passed in as the list of their *current* problem ids (see
//! [`crate::core::Gauntlet::scope_ids`]).

use std::collections::{BTreeMap, HashMap, HashSet};

use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::TimeZone;

use crate::perf::{Nudge, Perf};
use crate::store::{Entry, Event, Rating};

/// The scope that means every problem.
pub const ALL: &str = "all";

/// A problem is up for review while its latest rating is at most this:
/// 1 or 2, you didn't find it trivial. Or while your best solution is off
/// the reference's pace ([`Nudge`]).
pub const REVIEW_AT_MOST: u8 = 2;

/// One solve of a problem, in either mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Solve {
    pub at: Timestamp,
    /// Gauntlet: the run. Completionist: `None`.
    pub run: Option<u32>,
    /// Gauntlet: time charged to the problem in the run.
    pub secs: Option<f64>,
    /// Completionist only; `None` until rated.
    pub rating: Option<Rating>,
    /// How its tests ran.
    pub perf: Perf,
}

impl Solve {
    pub fn is_run(&self) -> bool {
        self.run.is_some()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    InProgress,
    Finished,
    Deleted,
}

#[derive(Clone, Debug)]
pub struct Run {
    pub id: u32,
    pub started: Timestamp,
    pub scope: String,
    /// In order, as started.
    pub problems: Vec<String>,
    pub finished: Option<Timestamp>,
    pub deleted: bool,
    /// Problem → seconds, for each problem solved in this run.
    pub times: BTreeMap<String, f64>,
}

impl Run {
    pub fn status(&self) -> RunStatus {
        if self.deleted {
            RunStatus::Deleted
        } else if self.finished.is_some() {
            RunStatus::Finished
        } else {
            RunStatus::InProgress
        }
    }

    /// Sum of this run's solve times.
    pub fn total(&self) -> f64 {
        self.times.values().sum()
    }

    pub fn solved(&self) -> usize {
        self.times.len()
    }
}

/// A scope's (or problem's) best time and the run it came from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pb {
    pub run: u32,
    pub secs: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SumOfBest {
    /// Sum of each problem's best run time, over problems that have one.
    pub secs: f64,
    /// Problems in the scope with no run time yet.
    pub missing: usize,
}

#[derive(Default)]
pub struct Db {
    solves: HashMap<String, Vec<Solve>>,
    runs: BTreeMap<u32, Run>,
    /// Problems redone since their latest completionist solve.
    redone: HashSet<String>,
    /// The best measurements of passes that weren't new solves, by problem.
    improved: HashMap<String, Perf>,
    /// The reference solutions measured on this machine, by problem.
    references: HashMap<String, Perf>,
}

impl Db {
    pub fn new() -> Db {
        Db::default()
    }

    pub fn from_entries(entries: &[Entry]) -> Db {
        let mut db = Db::new();
        for entry in entries {
            db.apply(entry);
        }
        db
    }

    pub fn apply(&mut self, entry: &Entry) {
        let at = entry.at;
        match &entry.event {
            Event::Solved { problem, run, secs, perf } => {
                // A run solve needs its run; logs edited by hand may lack it.
                if run.is_none() {
                    self.redone.remove(problem);
                }
                if let Some(r) = run {
                    match self.runs.get_mut(r) {
                        Some(run) => {
                            run.times.entry(problem.clone()).or_insert(secs.unwrap_or(0.0).max(0.0));
                        }
                        None => tracing::warn!(run = r, "solve for an unknown run"),
                    }
                }
                self.solves.entry(problem.clone()).or_default().push(Solve {
                    at,
                    run: *run,
                    secs: *secs,
                    rating: None,
                    perf: *perf,
                });
            }
            Event::Rated { problem, rating } => {
                if let Some(s) = self.solves.get_mut(problem).and_then(|v| v.iter_mut().rev().find(|s| !s.is_run())) {
                    s.rating = Some(*rating);
                }
            }
            Event::RunStarted { run, scope, problems } => {
                self.runs.insert(
                    *run,
                    Run {
                        id: *run,
                        started: at,
                        scope: scope.clone(),
                        problems: problems.clone(),
                        finished: None,
                        deleted: false,
                        times: BTreeMap::new(),
                    },
                );
            }
            Event::RunFinished { run } => {
                if let Some(r) = self.runs.get_mut(run) {
                    r.finished = Some(at);
                }
            }
            Event::RunDeleted { run } => {
                if let Some(r) = self.runs.get_mut(run) {
                    r.deleted = true;
                }
            }
            Event::Redo { problem } => {
                self.redone.insert(problem.clone());
            }
            Event::Improved { problem, perf } => {
                let best = self.improved.entry(problem.clone()).or_default();
                *best = best.best(*perf);
            }
        }
    }

    // ---- Completion & ratings ------------------------------------------------

    /// Every solve of a problem, oldest first, both modes — including solves
    /// in deleted runs (see [`Db::counts_for_runs`]).
    pub fn history(&self, id: &str) -> &[Solve] {
        self.solves.get(id).map_or(&[], Vec::as_slice)
    }

    /// Solved at least once, ever, in either mode.
    pub fn is_solved(&self, id: &str) -> bool {
        !self.history(id).is_empty()
    }

    pub fn solved_count<S: AsRef<str>>(&self, ids: &[S]) -> usize {
        ids.iter().filter(|id| self.is_solved(id.as_ref())).count()
    }

    pub fn last_completionist_solve(&self, id: &str) -> Option<&Solve> {
        self.history(id).iter().rev().find(|s| !s.is_run())
    }

    /// Whether a completionist pass of the problem is a new solve: it has no
    /// completionist solve yet, or it was redone since the latest one.
    pub fn completionist_armed(&self, id: &str) -> bool {
        self.last_completionist_solve(id).is_none() || self.redone.contains(id)
    }

    /// The latest completionist solve exists and hasn't been rated.
    pub fn awaiting_rating(&self, id: &str) -> bool {
        self.last_completionist_solve(id).is_some_and(|s| s.rating.is_none())
    }

    /// The most recent rating given for the problem.
    pub fn latest_rating(&self, id: &str) -> Option<Rating> {
        self.history(id).iter().rev().find_map(|s| s.rating)
    }

    /// How many of `ids` have each latest rating: index 0 is rating 1.
    pub fn rating_distribution<S: AsRef<str>>(&self, ids: &[S]) -> [usize; 5] {
        let mut dist = [0; 5];
        for r in ids.iter().filter_map(|id| self.latest_rating(id.as_ref())) {
            dist[usize::from(r.value() - 1)] += 1;
        }
        dist
    }

    // ---- Performance ---------------------------------------------------------

    /// The best runtime and the best memory over every pass of the problem,
    /// in either mode, each wherever it was achieved.
    pub fn best(&self, id: &str) -> Perf {
        let improved = self.improved.get(id).copied().unwrap_or_default();
        self.history(id).iter().map(|s| s.perf).fold(improved, Perf::best)
    }

    /// The reference solution's measurement on this machine, once known.
    pub fn reference(&self, id: &str) -> Option<Perf> {
        self.references.get(id).copied()
    }

    pub fn set_reference(&mut self, id: &str, perf: Perf) {
        self.references.insert(id.to_string(), perf);
    }

    /// Whether your best is off the reference's pace (see [`Nudge`]).
    pub fn nudge(&self, id: &str) -> Option<Nudge> {
        Nudge::of(self.best(id), self.reference(id)?)
    }

    // ---- Review --------------------------------------------------------------

    /// The latest rating, if it puts the problem up for review (see [`REVIEW_AT_MOST`]).
    pub fn review_rating(&self, id: &str) -> Option<Rating> {
        self.latest_rating(id).filter(|r| r.value() <= REVIEW_AT_MOST)
    }

    /// Up for review: rated 1 or 2, or your best is off the reference's pace.
    pub fn up_for_review(&self, id: &str) -> bool {
        self.review_rating(id).is_some() || self.nudge(id).is_some()
    }

    /// Those of `ids` up for review, in review order: lowest rating first,
    /// then the ones only off the pace, then least recently solved (in either
    /// mode), then in the order given.
    pub fn review_order<'a, S: AsRef<str>>(&self, ids: &'a [S]) -> Vec<&'a str> {
        let mut up: Vec<&str> = ids.iter().map(AsRef::as_ref).filter(|id| self.up_for_review(id)).collect();
        up.sort_by_key(|id| {
            (self.review_rating(id).map_or(u8::MAX, Rating::value), self.history(id).last().map(|s| s.at))
        });
        up
    }

    // ---- Activity ------------------------------------------------------------

    /// Consecutive local days with at least one solve, ending today (or
    /// yesterday, if nothing yet today).
    pub fn streak(&self, today: Date) -> u32 {
        let tz = TimeZone::system();
        let days: HashSet<Date> =
            self.solves.values().flatten().map(|s| s.at.to_zoned_datetime(tz.clone()).date()).collect();
        let mut day = if days.contains(&today) {
            today
        } else {
            match today.yesterday() {
                Ok(d) => d,
                Err(_) => return 0,
            }
        };
        let mut streak = 0;
        while days.contains(&day) {
            streak += 1;
            match day.yesterday() {
                Ok(d) => day = d,
                Err(_) => break,
            }
        }
        streak
    }

    // ---- Runs ----------------------------------------------------------------

    /// Every run, deleted ones included, oldest first.
    pub fn runs(&self) -> impl DoubleEndedIterator<Item = &Run> {
        self.runs.values()
    }

    pub fn run(&self, id: u32) -> Option<&Run> {
        self.runs.get(&id)
    }

    /// The (latest) run neither finished nor deleted.
    pub fn in_progress(&self) -> Option<&Run> {
        self.runs.values().rev().find(|r| r.status() == RunStatus::InProgress)
    }

    pub fn next_run_id(&self) -> u32 {
        self.runs.keys().next_back().map_or(1, |id| id + 1)
    }

    /// Whether a solve counts toward run stats: completionist solves and
    /// solves in deleted runs don't.
    pub fn counts_for_runs(&self, s: &Solve) -> bool {
        s.run.and_then(|r| self.runs.get(&r)).is_some_and(|r| !r.deleted)
    }

    fn live(&self) -> impl Iterator<Item = &Run> {
        self.runs.values().filter(|r| !r.deleted)
    }

    /// Time of a scope (given as its current problems) in run `id`: defined
    /// only if the run solved all of them and isn't deleted.
    pub fn scope_time<S: AsRef<str>>(&self, id: u32, ids: &[S]) -> Option<f64> {
        let run = self.runs.get(&id).filter(|r| !r.deleted)?;
        if ids.is_empty() {
            return None;
        }
        ids.iter().map(|p| run.times.get(p.as_ref()).copied()).sum()
    }

    /// Best time of a scope over non-deleted runs (other than `exclude`, e.g.
    /// the run in progress when showing its splits). Ties go to the older run.
    pub fn pb<S: AsRef<str>>(&self, ids: &[S], exclude: Option<u32>) -> Option<Pb> {
        self.live()
            .filter(|r| Some(r.id) != exclude)
            .filter_map(|r| Some(Pb { run: r.id, secs: self.scope_time(r.id, ids)? }))
            .fold(None, |best: Option<Pb>, pb| match best {
                Some(b) if b.secs <= pb.secs => Some(b),
                _ => Some(pb),
            })
    }

    /// Best run time of one problem over non-deleted runs (other than `exclude`).
    pub fn problem_best(&self, id: &str, exclude: Option<u32>) -> Option<Pb> {
        self.live()
            .filter(|r| Some(r.id) != exclude)
            .filter_map(|r| Some(Pb { run: r.id, secs: *r.times.get(id)? }))
            .fold(None, |best: Option<Pb>, pb| match best {
                Some(b) if b.secs <= pb.secs => Some(b),
                _ => Some(pb),
            })
    }

    pub fn sum_of_best<S: AsRef<str>>(&self, ids: &[S]) -> SumOfBest {
        let mut sob = SumOfBest { secs: 0.0, missing: 0 };
        for id in ids {
            match self.problem_best(id.as_ref(), None) {
                Some(pb) => sob.secs += pb.secs,
                None => sob.missing += 1,
            }
        }
        sob
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> Timestamp {
        s.parse().unwrap()
    }

    /// Builds a log with one event per minute.
    #[derive(Default)]
    struct Log(Vec<Entry>);

    impl Log {
        fn push(&mut self, event: Event) -> &mut Log {
            let t = at("2026-11-01T10:00:00Z") + jiff::SignedDuration::from_mins(self.0.len() as i64);
            self.0.push(Entry { at: t, event });
            self
        }
        fn start(&mut self, run: u32, scope: &str, problems: &[&str]) -> &mut Log {
            let problems = problems.iter().map(|p| p.to_string()).collect();
            self.push(Event::RunStarted { run, scope: scope.into(), problems })
        }
        fn solve(&mut self, problem: &str, run: Option<u32>, secs: Option<f64>) -> &mut Log {
            self.push(Event::Solved { problem: problem.into(), run, secs, perf: Perf::default() })
        }
        fn measured(&mut self, problem: &str, run: Option<u32>, ms: u64, mb: u64) -> &mut Log {
            let perf = Perf { ms: Some(ms), bytes: Some(mb << 20) };
            self.push(Event::Solved { problem: problem.into(), run, secs: run.map(|_| 60.0), perf })
        }
        fn finish(&mut self, run: u32) -> &mut Log {
            self.push(Event::RunFinished { run })
        }
        fn delete(&mut self, run: u32) -> &mut Log {
            self.push(Event::RunDeleted { run })
        }
        fn rate(&mut self, problem: &str, r: u8) -> &mut Log {
            self.push(Event::Rated { problem: problem.into(), rating: Rating::new(r).unwrap() })
        }
        fn db(&self) -> Db {
            Db::from_entries(&self.0)
        }
    }

    // Content: sections `s1` = [a, b] and `s2` = [c].
    const S1: [&str; 2] = ["a", "b"];
    const EVERY: [&str; 3] = ["a", "b", "c"];

    fn full_run(log: &mut Log, run: u32, times: [f64; 3]) {
        log.start(run, ALL, &EVERY);
        for (p, s) in EVERY.iter().zip(times) {
            log.solve(p, Some(run), Some(s));
        }
        log.finish(run);
    }

    #[test]
    fn completionist_rating_flow() {
        let mut log = Log::default();
        log.solve("a", None, None);
        let db = log.db();
        assert!(db.is_solved("a"));
        assert!(db.awaiting_rating("a"));
        assert_eq!(db.latest_rating("a"), None);

        log.rate("a", 2);
        let db = log.db();
        assert!(!db.awaiting_rating("a"));
        assert_eq!(db.latest_rating("a"), Rating::new(2));

        // A run solve neither needs nor takes a rating.
        log.start(1, ALL, &["a"]).solve("a", Some(1), Some(30.0));
        let db = log.db();
        assert!(!db.awaiting_rating("a"));

        // Redo and pass again: a new unrated solve; the latest rating is still shown.
        log.solve("a", None, None);
        let db = log.db();
        assert!(db.awaiting_rating("a"));
        assert_eq!(db.latest_rating("a"), Rating::new(2));
        log.rate("a", 5);
        let db = log.db();
        let ratings: Vec<_> = db.history("a").iter().map(|s| s.rating.map(Rating::value)).collect();
        assert_eq!(ratings, [Some(2), None, Some(5)]);
        assert_eq!(db.rating_distribution(&EVERY), [0, 0, 0, 0, 1]);
        assert_eq!(db.solved_count(&EVERY), 1);
    }

    #[test]
    fn only_a_redo_rearms_a_completionist_solve() {
        let mut log = Log::default();
        assert!(log.db().completionist_armed("a"));
        // A redo before any solve changes nothing: the first pass counts.
        log.push(Event::Redo { problem: "a".into() });
        assert!(log.db().completionist_armed("a"));
        log.solve("a", None, None);
        assert!(!log.db().completionist_armed("a"), "solved; editing and passing again isn't a solve");
        // A run solve neither arms nor disarms it.
        log.start(1, ALL, &["a"]).solve("a", Some(1), Some(5.0));
        assert!(!log.db().completionist_armed("a"));
        log.push(Event::Redo { problem: "a".into() });
        assert!(log.db().completionist_armed("a"));
        log.solve("a", Some(1), Some(5.0));
        assert!(log.db().completionist_armed("a"), "still armed after a run solve");
        log.solve("a", None, None);
        assert!(!log.db().completionist_armed("a"));
        assert!(log.db().completionist_armed("b"), "per problem");
    }

    #[test]
    fn run_solves_count_as_completion_even_when_deleted() {
        let mut log = Log::default();
        log.start(1, ALL, &EVERY).solve("a", Some(1), Some(10.0)).delete(1);
        let db = log.db();
        assert!(db.is_solved("a"));
        assert!(!db.awaiting_rating("a"));
        assert!(!db.counts_for_runs(&db.history("a")[0]));
        assert_eq!(db.pb(&["a"], None), None);
        assert_eq!(db.problem_best("a", None), None);
        assert_eq!(db.in_progress().map(|r| r.id), None);
        assert_eq!(db.next_run_id(), 2);
    }

    #[test]
    fn a_full_run_gives_pbs_for_every_scope() {
        let mut log = Log::default();
        full_run(&mut log, 1, [10.0, 20.0, 30.0]);
        let db = log.db();
        assert_eq!(db.pb(&EVERY, None), Some(Pb { run: 1, secs: 60.0 }));
        assert_eq!(db.pb(&S1, None), Some(Pb { run: 1, secs: 30.0 }));
        assert_eq!(db.pb(&["c"], None), Some(Pb { run: 1, secs: 30.0 }));
        assert_eq!(db.run(1).unwrap().status(), RunStatus::Finished);
        assert_eq!(db.run(1).unwrap().total(), 60.0);
    }

    #[test]
    fn a_partial_run_is_a_pb_for_what_it_covers() {
        let mut log = Log::default();
        full_run(&mut log, 1, [10.0, 20.0, 30.0]);
        log.start(2, "s1", &S1).solve("a", Some(2), Some(5.0)).solve("b", Some(2), Some(5.0)).finish(2);
        let db = log.db();
        assert_eq!(db.pb(&S1, None), Some(Pb { run: 2, secs: 10.0 }));
        assert_eq!(db.pb(&EVERY, None), Some(Pb { run: 1, secs: 60.0 }), "run 2 doesn't cover c");
        assert_eq!(db.pb(&S1, Some(2)), Some(Pb { run: 1, secs: 30.0 }));
        assert_eq!(db.sum_of_best(&EVERY), SumOfBest { secs: 40.0, missing: 0 });
        assert_eq!(db.problem_best("c", None), Some(Pb { run: 1, secs: 30.0 }));
    }

    #[test]
    fn in_progress_runs_count_for_scopes_they_already_cover() {
        let mut log = Log::default();
        full_run(&mut log, 1, [10.0, 20.0, 30.0]);
        log.start(2, ALL, &EVERY).solve("a", Some(2), Some(1.0)).solve("b", Some(2), Some(1.0));
        let db = log.db();
        assert_eq!(db.in_progress().map(|r| r.id), Some(2));
        assert_eq!(db.pb(&S1, None), Some(Pb { run: 2, secs: 2.0 }));
        assert_eq!(db.pb(&EVERY, None), Some(Pb { run: 1, secs: 60.0 }));
        assert_eq!(db.scope_time(2, &EVERY), None);
    }

    #[test]
    fn deleted_runs_are_excluded_from_pbs() {
        let mut log = Log::default();
        full_run(&mut log, 1, [10.0, 20.0, 30.0]);
        full_run(&mut log, 2, [1.0, 1.0, 1.0]);
        log.delete(2);
        let db = log.db();
        assert_eq!(db.pb(&EVERY, None), Some(Pb { run: 1, secs: 60.0 }));
        assert_eq!(db.sum_of_best(&EVERY).secs, 60.0);
        assert_eq!(db.scope_time(2, &EVERY), None);
        assert_eq!(db.run(2).unwrap().status(), RunStatus::Deleted);
        // Still activity and completion.
        assert_eq!(db.history("a").len(), 2);
    }

    #[test]
    fn a_run_is_a_pb_only_for_problems_it_solved() {
        let mut log = Log::default();
        full_run(&mut log, 1, [10.0, 20.0, 30.0]);
        // Content grows: d joins section s2. Run 1 never had it.
        let s2 = ["c", "d"];
        let grown = ["a", "b", "c", "d"];
        let db = log.db();
        assert_eq!(db.pb(&s2, None), None);
        assert_eq!(db.pb(&grown, None), None);
        // Untouched sections keep their PBs.
        assert_eq!(db.pb(&S1, None), Some(Pb { run: 1, secs: 30.0 }));
        assert_eq!(db.sum_of_best(&grown), SumOfBest { secs: 60.0, missing: 1 });

        // A run over just the new problem is its PB, and not one for s2.
        log.start(2, "d", &["d"]).solve("d", Some(2), Some(7.0)).finish(2);
        let db = log.db();
        assert_eq!(db.pb(&["d"], None), Some(Pb { run: 2, secs: 7.0 }));
        assert_eq!(db.pb(&s2, None), None);
        // A run over the whole of s2 is.
        log.start(3, "s2", &s2).solve("c", Some(3), Some(40.0)).solve("d", Some(3), Some(9.0)).finish(3);
        let db = log.db();
        assert_eq!(db.pb(&s2, None), Some(Pb { run: 3, secs: 49.0 }));
        assert_eq!(db.pb(&grown, None), None);
        assert_eq!(db.sum_of_best(&grown), SumOfBest { secs: 67.0, missing: 0 });
    }

    #[test]
    fn review_is_the_latest_rating_of_1_or_2_hardest_then_oldest_first() {
        let mut log = Log::default();
        // Solved in this order: a, b, c, d, e (one minute apart).
        for (p, r) in [("a", 2), ("b", 1), ("c", 3), ("d", 2), ("e", 1)] {
            log.solve(p, None, None).rate(p, r);
        }
        log.solve("f", None, None); // unrated
        let ids = ["a", "b", "c", "d", "e", "f", "g"];
        let db = log.db();
        assert!(db.up_for_review("a") && !db.up_for_review("c") && !db.up_for_review("f") && !db.up_for_review("g"));
        assert_eq!(db.review_order(&ids), ["b", "e", "a", "d"]);
        // Only within the ids given.
        assert_eq!(db.review_order(&["d", "c", "a"]), ["a", "d"]);

        // Solving b again (a run solve counts) makes it the most recent of the 1s.
        log.start(1, "b", &["b"]).solve("b", Some(1), Some(5.0)).finish(1);
        assert_eq!(log.db().review_order(&ids), ["e", "b", "a", "d"]);
        // Redone and re-rated 3 or more: no longer up for review.
        log.push(Event::Redo { problem: "e".into() }).solve("e", None, None);
        assert_eq!(log.db().review_order(&ids), ["b", "e", "a", "d"], "the old rating stands until re-rated");
        log.rate("e", 4);
        assert_eq!(log.db().review_order(&ids), ["b", "a", "d"]);
        log.rate("a", 1);
        assert_eq!(log.db().review_order(&ids), ["a", "b", "d"]);
    }

    #[test]
    fn the_best_runtime_and_memory_count_wherever_they_came_from() {
        let mut log = Log::default();
        log.solve("a", None, None).measured("a", None, 900, 12);
        log.start(1, ALL, &EVERY).measured("a", Some(1), 300, 40).delete(1);
        log.measured("b", None, 50, 3);
        let db = log.db();
        // The time from a (deleted) run, the memory from completionist mode.
        assert_eq!(db.best("a"), Perf { ms: Some(300), bytes: Some(12 << 20) });
        assert_eq!(db.best("c"), Perf::default());
        assert_eq!(db.history("a")[0].perf, Perf::default(), "a solve from before measurements");
    }

    #[test]
    fn off_the_pace_is_up_for_review_after_the_low_ratings() {
        let mut log = Log::default();
        // a: rated 2; b: slow, rated 4; c: slow, unrated, solved before b; d: fine; e: slow, no reference yet.
        log.measured("c", None, 2_000, 3);
        log.measured("a", None, 100, 3).rate("a", 2);
        log.measured("b", None, 2_000, 3).rate("b", 4);
        log.measured("d", None, 120, 3).rate("d", 5);
        log.measured("e", None, 9_000, 3);
        let mut db = log.db();
        let ids = ["a", "b", "c", "d", "e"];
        assert_eq!(db.review_order(&ids), ["a"], "nothing to compare with yet");
        for id in ["a", "b", "c", "d"] {
            db.set_reference(id, Perf { ms: Some(100), bytes: Some(3 << 20) });
        }
        assert_eq!(db.review_order(&ids), ["a", "c", "b"]);
        assert_eq!(db.nudge("b").map(|n| (n.slower, n.hungrier)), Some((true, false)));
        assert_eq!(db.review_rating("b"), None);
        // A faster solve takes it off the list.
        log.measured("c", None, 150, 3);
        let mut db = log.db();
        for id in ids {
            db.set_reference(id, Perf { ms: Some(100), bytes: Some(3 << 20) });
        }
        assert_eq!(db.review_order(&ids), ["a", "b", "e"]);
    }

    #[test]
    fn streak() {
        let mut db = Db::new();
        for t in ["2026-11-09T12:00:00Z", "2026-11-10T12:00:00Z", "2026-11-10T13:00:00Z"] {
            let event = Event::Solved { problem: "a".into(), run: None, secs: None, perf: Perf::default() };
            db.apply(&Entry { at: at(t), event });
        }
        let day = |s: &str| at(s).to_zoned_datetime(TimeZone::system()).date();
        assert_eq!(db.streak(day("2026-11-10T12:00:00Z")), 2);
        assert_eq!(db.streak(day("2026-11-11T12:00:00Z")), 2); // not yet today
        assert_eq!(db.streak(day("2026-11-13T12:00:00Z")), 0);
    }
}
