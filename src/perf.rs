//! How long a solution's tests take and how much memory they use, against the
//! reference solution measured the same way on the same machine. Raw numbers
//! don't travel between machines; the ratio to the reference mostly does.
//!
//! Speed never decides whether a solution passes: the test timeout only
//! guards against hangs. A solution off the reference's pace by more than
//! noise gets a [`Nudge`] instead.
//!
//! The reference is measured lazily, in the best of [`REFERENCE_RUNS`], and
//! cached in `<cache>/reference.json`, keyed by everything the measurement
//! depends on: gauntlet's version, rustc's, and the solution and tests.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::dirs::Fnv;
use crate::problem::{Problem, Workspace};
use crate::runner::{self, Code, Env, RunResult};
use crate::toolchain;

/// Runs of the reference solution; each measure's best counts.
pub const REFERENCE_RUNS: usize = 3;

/// A nudge takes both a ratio to the reference and a gap from it: a test
/// binary that takes 30 ms is mostly process startup, and a few MB of memory
/// mostly the runtime itself. Every reference solution, run as if it were
/// yours (on an M4 Max, idle and with every core busy), stayed within 3.3×
/// and 30 ms (1.1× where the reference takes 100 ms or more), and 1.4× and
/// 7 MB; quadratic solutions to linear problems
/// took 75–580× and seconds longer.
const TIME_RATIO: f64 = 2.0;
const TIME_GAP_MS: u64 = 300;
const MEMORY_RATIO: f64 = 2.0;
const MEMORY_GAP: u64 = 32 << 20;

/// A passing test run's measurements. Either can be missing: logs from
/// before they were recorded, or a platform that doesn't report memory.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Perf {
    /// The tests' wall-clock runtime, in milliseconds.
    #[serde(default, rename = "test_ms", skip_serializing_if = "Option::is_none")]
    pub ms: Option<u64>,
    /// Their peak memory, in bytes.
    #[serde(default, rename = "peak_bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

impl Perf {
    pub fn of(r: &RunResult) -> Perf {
        Perf { ms: Some(u64::try_from(r.runtime.as_millis()).unwrap_or(u64::MAX)), bytes: r.peak_memory }
    }

    /// The better of each measure, wherever it came from.
    pub fn best(self, other: Perf) -> Perf {
        let min = |a: Option<u64>, b: Option<u64>| match (a, b) {
            (Some(a), Some(b)) => Some(a.min(b)),
            _ => a.or(b),
        };
        Perf { ms: min(self.ms, other.ms), bytes: min(self.bytes, other.bytes) }
    }
}

/// How many times `reference` `mine` is, where both are known.
pub fn ratio(mine: Option<u64>, reference: Option<u64>) -> Option<f64> {
    Some(mine? as f64 / reference?.max(1) as f64)
}

/// Where a solution is off the reference's pace by more than noise.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Nudge {
    pub slower: bool,
    pub hungrier: bool,
}

impl Nudge {
    pub fn of(mine: Perf, reference: Perf) -> Option<Nudge> {
        let off = |mine: Option<u64>, reference: Option<u64>, ratio: f64, gap: u64| match (mine, reference) {
            (Some(m), Some(r)) => m as f64 > r as f64 * ratio && m - r > gap,
            _ => false,
        };
        let nudge = Nudge {
            slower: off(mine.ms, reference.ms, TIME_RATIO, TIME_GAP_MS),
            hungrier: off(mine.bytes, reference.bytes, MEMORY_RATIO, MEMORY_GAP),
        };
        (nudge.slower || nudge.hungrier).then_some(nudge)
    }

    /// `could probably be faster`, and so on.
    pub fn says(self) -> &'static str {
        match (self.slower, self.hungrier) {
            (true, true) => "could probably be faster and leaner",
            (true, false) => "could probably be faster",
            _ => "could probably use less memory",
        }
    }
}

// ---- The reference, measured -------------------------------------------------

/// A problem's reference solution, to measure on this machine.
pub struct Reference {
    pub id: String,
    solution: PathBuf,
    tests: PathBuf,
    /// The cache file.
    cache: PathBuf,
}

/// A cached measurement, and what it's valid for.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Cached {
    key: String,
    #[serde(flatten)]
    perf: Perf,
}

impl Reference {
    pub fn of(ws: &Workspace, p: &Problem) -> Reference {
        Reference { id: p.id.clone(), solution: p.solution(), tests: p.tests(), cache: ws.roots.reference.clone() }
    }

    /// What a measurement is valid for: this gauntlet, this rustc, and this
    /// solution and its tests. `None` if any of them can't be read.
    fn key(&self) -> Option<String> {
        let rustc = toolchain::rustc_version()?;
        let mut hash = Fnv::default();
        for file in [&self.solution, &self.tests] {
            hash.feed_len_and(&fs::read(file).ok()?);
        }
        Some(format!("{} · {rustc} · {:016x}", env!("CARGO_PKG_VERSION"), hash.0))
    }

    /// Its measurement: cached, or the best of [`REFERENCE_RUNS`], then
    /// cached. `None` if it doesn't pass (a problem being edited).
    pub fn measure(&self, env: &Env) -> io::Result<Option<Perf>> {
        let key = self.key();
        if let Some(cached) = read_cache(&self.cache).remove(&self.id).filter(|c| Some(&c.key) == key.as_ref()) {
            return Ok(Some(cached.perf));
        }
        let source = runner::without_tests_mod(&fs::read_to_string(&self.solution)?);
        let code = Code { file: &self.solution, source: &source, tests: &self.tests };
        let runs = env.run_tests_repeatedly(&format!("{}-reference", self.id), &code, REFERENCE_RUNS)?;
        if runs.len() < REFERENCE_RUNS || !runs.iter().all(|r| r.outcome.passed()) {
            tracing::warn!(problem = self.id, "the reference solution didn't pass; not measured");
            return Ok(None);
        }
        let perf = runs.iter().map(Perf::of).fold(Perf::default(), Perf::best);
        tracing::info!(problem = self.id, ?perf, "measured the reference solution");
        if let Some(key) = key
            && let Err(e) = write_cache(&self.cache, &self.id, Cached { key, perf })
        {
            tracing::warn!("caching in {}: {e}", self.cache.display());
        }
        Ok(Some(perf))
    }
}

/// The cached measurements still valid for `ws`'s problems.
pub fn cached(ws: &Workspace) -> HashMap<String, Perf> {
    let mut cache = read_cache(&ws.roots.reference);
    let mut valid = HashMap::new();
    for p in &ws.problems {
        if let Some(c) = cache.remove(&p.id)
            && Reference::of(ws, p).key() == Some(c.key)
        {
            valid.insert(p.id.clone(), c.perf);
        }
    }
    valid
}

/// The cache, or nothing if it's missing or unreadable: it's only a cache.
fn read_cache(path: &Path) -> BTreeMap<String, Cached> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_else(|e| {
            tracing::warn!("ignoring unreadable {}: {e}", path.display());
            BTreeMap::new()
        }),
        Err(_) => BTreeMap::new(),
    }
}

/// Add `id`'s measurement to the cache: written aside and renamed into
/// place, so a reader never sees half of it.
fn write_cache(path: &Path, id: &str, entry: Cached) -> io::Result<()> {
    let mut cache = read_cache(path);
    cache.insert(id.to_string(), entry);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension(format!("{}.tmp", std::process::id()));
    fs::write(&tmp, serde_json::to_string_pretty(&cache).map_err(io::Error::other)?)?;
    fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::tests::sample;

    const MB: u64 = 1 << 20;

    fn perf(ms: u64, mb: u64) -> Perf {
        Perf { ms: Some(ms), bytes: Some(mb * MB) }
    }

    #[test]
    fn the_best_of_each_measure_counts() {
        let old = Perf::default();
        assert_eq!(perf(800, 40).best(perf(900, 12)), perf(800, 12));
        assert_eq!(old.best(perf(5, 3)), perf(5, 3), "a solve from before measurements");
        assert_eq!(Perf { ms: Some(7), bytes: None }.best(perf(9, 3)), Perf { ms: Some(7), bytes: Some(3 * MB) });
        assert_eq!(ratio(Some(840), Some(80)), Some(10.5));
        assert_eq!(ratio(Some(3), Some(0)), Some(3.0), "a reference too quick to time");
        assert_eq!(ratio(None, Some(80)), None);
    }

    #[test]
    fn a_nudge_takes_a_ratio_and_a_gap() {
        let reference = perf(80, 12);
        assert_eq!(Nudge::of(perf(840, 12), reference), Some(Nudge { slower: true, hungrier: false }));
        assert_eq!(Nudge::of(perf(90, 60), reference), Some(Nudge { slower: false, hungrier: true }));
        assert_eq!(Nudge::of(perf(900, 60), reference).map(Nudge::says), Some("could probably be faster and leaner"));
        // Many times the reference, but only by startup noise; or far off, but within the ratio.
        assert_eq!(Nudge::of(perf(150, 30), perf(4, 3)), None);
        assert_eq!(Nudge::of(perf(1_900, 230), perf(1_000, 120)), None);
        // Faster and leaner than the reference, or not measured.
        assert_eq!(Nudge::of(perf(40, 6), reference), None);
        assert_eq!(Nudge::of(Perf::default(), reference), None);
        assert_eq!(Nudge::of(perf(9_000, 900), Perf::default()), None);
    }

    #[test]
    fn the_cache_is_keyed_by_what_the_measurement_depends_on() {
        let s = sample("perf-cache");
        s.write("content/02_arrays/01_dup/solution.rs", "pub fn f() {}\n");
        s.write("content/02_arrays/01_dup/tests.rs", "#[test]\nfn t() {}\n");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let dup = ws.problems.iter().find(|p| p.id == "dup").unwrap();
        let reference = Reference::of(&ws, dup);
        let key = reference.key().expect("rustc is installed");
        assert!(key.starts_with(concat!(env!("CARGO_PKG_VERSION"), " · rustc ")), "{key}");
        assert_eq!(reference.key(), Some(key.clone()), "stable");

        write_cache(&ws.roots.reference, "dup", Cached { key: key.clone(), perf: perf(80, 12) }).unwrap();
        write_cache(&ws.roots.reference, "gone", Cached { key: key.clone(), perf: perf(1, 1) }).unwrap();
        assert_eq!(cached(&ws), HashMap::from([("dup".to_string(), perf(80, 12))]));
        // Edited tests (or solution): measured again.
        s.write("content/02_arrays/01_dup/tests.rs", "#[test]\nfn t() { assert!(true); }\n");
        assert_ne!(reference.key(), Some(key.clone()));
        assert!(cached(&ws).is_empty());
        // Unreadable: nothing cached, no error.
        fs::write(&ws.roots.reference, "{").unwrap();
        assert!(cached(&ws).is_empty());
        assert!(Reference::of(&ws, ws.problems.iter().find(|p| p.id == "two_sum").unwrap()).key().is_none());
    }

    #[test]
    fn a_reference_is_measured_once_and_cached() {
        let s = sample("perf-measure");
        s.write("content/02_arrays/01_dup/solution.rs", "pub fn f() -> u32 { 2 }\n");
        s.write("content/02_arrays/01_dup/tests.rs", "use super::*;\n#[test]\nfn t() { assert_eq!(f(), 2); }\n");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let dup = ws.problems.iter().find(|p| p.id == "dup").unwrap();
        let measured = Reference::of(&ws, dup).measure(&ws.env()).unwrap().expect("it passes");
        assert!(measured.ms.is_some());
        if cfg!(any(unix, windows)) {
            assert!(measured.bytes.is_some_and(|b| b > MB / 2 && b < 512 * MB), "{measured:?}");
        }
        assert_eq!(cached(&ws).get("dup"), Some(&measured));
        // A second measurement is the cached one.
        assert_eq!(Reference::of(&ws, dup).measure(&ws.env()).unwrap(), Some(measured));
        // A failing reference isn't measured or cached.
        s.write("content/02_arrays/01_dup/solution.rs", "pub fn f() -> u32 { 3 }\n");
        assert_eq!(Reference::of(&ws, dup).measure(&ws.env()).unwrap(), None);
        assert!(cached(&ws).is_empty());
    }
}
