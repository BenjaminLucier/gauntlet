//! The run in progress: `<data>/run.json`, rewritten as it changes. It exists
//! iff a run is in progress. The event log has the run's start and its solves;
//! this file has what's only live — the queue order and the clock.
//!
//! ```json
//! {"run":3,"scope":"two_pointers","queue":["three_sum","two_sum_ii"],"current":"three_sum",
//!  "elapsed":{"two_sum_ii":61.2,"three_sum":12.0},"paused":false}
//! ```
//!
//! Every unpaused second is charged to the problem on screen, `current`,
//! which is always the head of `queue` (the problems not yet solved).
//! Skipping moves it to the back; its time carries over for when it returns.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::db::Run;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunState {
    pub run: u32,
    pub scope: String,
    /// Unsolved problems; the head is `current`, skipped ones go to the back.
    pub queue: Vec<String>,
    /// The problem on screen. Empty once the queue is.
    pub current: String,
    /// Seconds charged per problem, solved ones included.
    pub elapsed: BTreeMap<String, f64>,
    pub paused: bool,
}

impl RunState {
    /// A fresh run over `problems`, in that order, with the clock running.
    pub fn new(run: u32, scope: &str, problems: &[String]) -> RunState {
        let mut s = RunState {
            run,
            scope: scope.to_string(),
            queue: problems.to_vec(),
            current: String::new(),
            elapsed: BTreeMap::new(),
            paused: false,
        };
        s.sync_current();
        s
    }

    /// Rebuild the state of an in-progress run from the log, if `run.json` was
    /// lost: unsolved problems in order, the solved ones' recorded times.
    /// Time on unsolved problems is gone. Starts paused.
    pub fn rebuild(run: &Run) -> RunState {
        let queue: Vec<String> = run.problems.iter().filter(|p| !run.times.contains_key(*p)).cloned().collect();
        let mut s = RunState::new(run.id, &run.scope, &queue);
        s.elapsed = run.times.clone();
        s.paused = true;
        s
    }

    /// Catch up with the log, which has the last word: `run.json` is saved
    /// after a solve is logged, so a crash in between leaves the problem
    /// queued. Its solved problems leave the queue and take their logged
    /// times. Returns whether anything changed.
    pub fn sync_with(&mut self, run: &Run) -> bool {
        let before = self.clone();
        self.queue.retain(|p| !run.times.contains_key(p));
        self.elapsed.extend(run.times.iter().map(|(p, &secs)| (p.clone(), secs)));
        self.sync_current();
        *self != before
    }

    fn sync_current(&mut self) {
        self.current = self.queue.first().cloned().unwrap_or_default();
    }

    /// Fix up a hand-edited or stale file: `current` leads the queue.
    pub fn normalize(&mut self) {
        if let Some(i) = self.queue.iter().position(|p| *p == self.current) {
            let cur = self.queue.remove(i);
            self.queue.insert(0, cur);
        }
        self.sync_current();
    }

    pub fn load(path: &Path) -> io::Result<Option<RunState>> {
        match fs::read_to_string(path) {
            Ok(text) => {
                let mut s: RunState = serde_json::from_str(&text).map_err(io::Error::other)?;
                s.normalize();
                Ok(Some(s))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Write atomically (temp file + rename).
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_string(self).map_err(io::Error::other)?)?;
        fs::rename(tmp, path)
    }

    /// Charge `d` to the current problem, unless paused or done.
    pub fn charge(&mut self, d: Duration) {
        if self.paused || self.current.is_empty() {
            return;
        }
        *self.elapsed.entry(self.current.clone()).or_default() += d.as_secs_f64();
    }

    /// Time charged to `id` so far.
    pub fn time_on(&self, id: &str) -> Duration {
        Duration::from_secs_f64(self.elapsed.get(id).copied().unwrap_or(0.0).max(0.0))
    }

    /// Total time charged in this run, solved and unsolved problems alike.
    pub fn total(&self) -> Duration {
        Duration::from_secs_f64(self.elapsed.values().sum::<f64>().max(0.0))
    }

    /// Move the current problem to the back of the queue. Returns the new current.
    pub fn skip(&mut self) -> &str {
        if self.queue.len() > 1 {
            self.queue.rotate_left(1);
        }
        self.sync_current();
        &self.current
    }

    /// Take the current problem off the queue: it's solved. Returns it and
    /// its time.
    pub fn solve_current(&mut self) -> Option<(String, f64)> {
        if self.queue.is_empty() {
            return None;
        }
        let id = self.queue.remove(0);
        let secs = self.elapsed.get(&id).copied().unwrap_or(0.0);
        self.sync_current();
        Some((id, secs))
    }

    pub fn is_done(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn skip_carries_time_over() {
        let mut s = RunState::new(1, "t", &ids(&["a", "b", "c"]));
        assert_eq!(s.current, "a");
        s.charge(Duration::from_secs(10));
        assert_eq!(s.skip(), "b");
        assert_eq!(s.queue, ids(&["b", "c", "a"]));
        s.charge(Duration::from_secs(5));
        s.skip();
        s.skip();
        assert_eq!(s.current, "a");
        s.charge(Duration::from_secs(2));
        assert_eq!(s.solve_current(), Some(("a".into(), 12.0)));
        assert_eq!(s.current, "b");
        assert_eq!(s.time_on("b"), Duration::from_secs(5));
        assert_eq!(s.total(), Duration::from_secs(17));
    }

    #[test]
    fn paused_time_is_not_charged() {
        let mut s = RunState::new(1, "t", &ids(&["a"]));
        s.paused = true;
        s.charge(Duration::from_secs(10));
        assert_eq!(s.total(), Duration::ZERO);
        s.paused = false;
        s.charge(Duration::from_secs(1));
        assert_eq!(s.solve_current(), Some(("a".into(), 1.0)));
        assert!(s.is_done());
        assert_eq!(s.current, "");
        s.charge(Duration::from_secs(1));
        assert_eq!(s.total(), Duration::from_secs(1));
        assert_eq!(s.solve_current(), None);
        assert_eq!(s.skip(), "");
    }

    #[test]
    fn saves_loads_and_normalizes() {
        let dir = std::env::temp_dir().join(format!("gauntlet-run-{}", std::process::id()));
        let path = dir.join("run.json");
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(RunState::load(&path).unwrap(), None);
        let mut s = RunState::new(4, "t", &ids(&["a", "b"]));
        s.charge(Duration::from_secs(3));
        s.save(&path).unwrap();
        assert_eq!(RunState::load(&path).unwrap(), Some(s.clone()));
        // A current that isn't at the head is moved there.
        s.current = "b".into();
        s.save(&path).unwrap();
        let loaded = RunState::load(&path).unwrap().unwrap();
        assert_eq!((loaded.queue, loaded.current), (ids(&["b", "a"]), "b".to_string()));
        let _ = fs::remove_dir_all(&dir);
    }
}
