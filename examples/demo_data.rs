//! Writes six weeks of made-up progress for one keen user, for the README's
//! screenshots and demo (see `demo/README.md`).
//!
//! ```sh
//! cargo run --example demo_data -- <data dir> <cache dir>
//! ```
//!
//! The directories are what gauntlet would use as `$XDG_DATA_HOME/gauntlet`
//! and `$XDG_CACHE_HOME/gauntlet`. The history is the same on every run,
//! apart from ending yesterday: every problem up to *Tree Traversals* solved
//! and rated, a streak at the end, a problem redone and rated higher, a few
//! rated hard enough to come back for review, one well off the reference's
//! pace, and four finished runs over sections, two of them over the same one.
//! Working files and run files hold the reference solutions.
//!
//! The reference measurements in `reference.json` are made up too, keyed for
//! this gauntlet and the `rustc` on `PATH` the way gauntlet keys its own.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::{DateTime, Days, Duration, Local, NaiveTime, TimeZone, Utc};
use color_eyre::eyre::{Result, bail, eyre};
use serde_json::{Value, json};

/// "gauntlet" in ASCII.
const SEED: u64 = 0x6761_756e_746c_6574;
/// The last problem solved; the one after it is up next.
const LAST_SOLVED: &str = "tree_traversals";
/// How many days ago the first solve was.
const DAYS: u64 = 42;
/// Every day this many days back is practised.
const STREAK: u64 = 9;
const MB: u64 = 1 << 20;

/// Latest ratings that put a problem up for review.
const HARD: &[(&str, u8)] =
    &[("trapping_rain_water", 2), ("lru_cache", 2), ("merge_k_sorted_lists", 2), ("reverse_nodes_in_k_group", 1)];
/// Rated 2, redone a week later, rated 4.
const REDONE: &str = "implementing_iterators";
/// Solved with a best runtime several times the reference's.
const SLOW: &str = "longest_consecutive_sequence";
/// Sections run against the clock, in order, once each section is solved.
const RUNS: &[&str] = &["toolkit_values", "two_pointers", "toolkit_values", "sliding_window"];

struct Problem {
    id: String,
    section: String,
    dir: PathBuf,
    difficulty: String,
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1).map(PathBuf::from);
    let (Some(data), Some(cache)) = (args.next(), args.next()) else {
        bail!("usage: cargo run --example demo_data -- <data dir> <cache dir>");
    };
    if data.join("events.jsonl").exists() {
        bail!("{} already has progress; delete it first", data.display());
    }
    fs::create_dir_all(&data)?;
    fs::create_dir_all(&cache)?;

    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let problems = load(&content)?;
    let last = problems.iter().position(|p| p.id == LAST_SOLVED).ok_or_else(|| eyre!("no {LAST_SOLVED}"))?;
    let solved = &problems[..=last];

    let mut rng = Rng(SEED);
    let rustc = rustc_version()?;
    let mut reference = BTreeMap::new();
    for p in &problems {
        let (ms, bytes) = reference_perf(&mut rng, &p.difficulty);
        let key = format!("{} · {rustc} · {:016x}", env!("CARGO_PKG_VERSION"), key_hash(&p.dir)?);
        reference.insert(p.id.clone(), json!({ "key": key, "test_ms": ms, "peak_bytes": bytes }));
    }
    fs::write(cache.join("reference.json"), serde_json::to_string_pretty(&reference)?)?;

    // Which days get practice, and how many problems each.
    let today = Local::now().date_naive();
    let mut days = Vec::new();
    for ago in (1..=DAYS).rev() {
        if ago <= STREAK || rng.chance(0.7) {
            days.push(today - Days::new(ago));
        }
    }
    let mut per_day = vec![1; days.len()];
    for _ in days.len()..solved.len() {
        let d = rng.below(days.len() as u64) as usize;
        per_day[d] += 1;
    }

    let mut log = Log::default();
    let mut queue = solved.iter();
    let mut done: Vec<&str> = Vec::new();
    let mut runs = 0;
    let mut redo_at = None;
    for (&day, &count) in days.iter().zip(&per_day) {
        let hour = if rng.chance(0.6) { 19 + rng.below(3) } else { 7 };
        let start = NaiveTime::from_hms_opt(hour as u32, rng.below(60) as u32, 0).unwrap();
        let mut at = Local.from_local_datetime(&day.and_time(start)).single().unwrap().with_timezone(&Utc);

        if redo_at.is_some_and(|d| d <= day) {
            redo_at = None;
            log.push(at, json!({ "kind": "redo", "problem": REDONE }));
            at += minutes(&mut rng, 6, 20);
            log.solve(at, REDONE, None, solve_perf(&mut rng, &reference[REDONE], 1.0));
            log.push(at + Duration::seconds(9), json!({ "kind": "rated", "problem": REDONE, "rating": 4 }));
            at += minutes(&mut rng, 2, 6);
        }

        for p in queue.by_ref().take(count) {
            at += minutes(&mut rng, 8, 35);
            let slowness = if p.id == SLOW { 7.0 } else { 1.0 };
            log.solve(at, &p.id, None, solve_perf(&mut rng, &reference[&p.id], slowness));
            let rating = if p.id == REDONE {
                redo_at = Some(day + Days::new(7));
                2
            } else {
                HARD.iter().find(|(id, _)| *id == p.id).map_or_else(|| rating(&mut rng, &p.difficulty), |&(_, r)| r)
            };
            log.push(at + Duration::seconds(7), json!({ "kind": "rated", "problem": p.id, "rating": rating }));
            fs::write(data.join(format!("{}.rs", p.id)), working(p)?)?;
            done.push(&p.id);
        }

        // Race a section the day after it's solved.
        let Some(&scope) = RUNS.get(runs) else { continue };
        let section: Vec<&Problem> = problems.iter().filter(|p| p.section == scope).collect();
        if !section.iter().all(|p| done.contains(&p.id.as_str())) || rng.chance(0.4) {
            continue;
        }
        runs += 1;
        // The second run over a section goes faster.
        let pace = if RUNS[..runs - 1].contains(&scope) { 0.7 } else { 1.0 };
        at += minutes(&mut rng, 20, 60);
        let ids: Vec<&str> = section.iter().map(|p| p.id.as_str()).collect();
        log.push(at, json!({ "kind": "run_started", "run": runs, "scope": scope, "problems": ids }));
        let dir = data.join("runs").join(runs.to_string());
        fs::create_dir_all(&dir)?;
        for p in &section {
            let secs = run_secs(&mut rng, &p.difficulty) * pace;
            at += Duration::milliseconds((secs * 1000.0) as i64);
            log.solve(at, &p.id, Some((runs, secs)), solve_perf(&mut rng, &reference[&p.id], 1.0));
            fs::write(dir.join(format!("{}.rs", p.id)), working(p)?)?;
        }
        log.push(at, json!({ "kind": "run_finished", "run": runs }));
    }
    if queue.next().is_some() || runs < RUNS.len() {
        bail!("the days ran out before the history did; adjust DAYS or RUNS");
    }

    fs::write(data.join("events.jsonl"), log.0.concat())?;
    println!("{} solves, {runs} runs, in {}", solved.len(), data.display());
    Ok(())
}

/// Every problem, in gauntlet order.
fn load(content: &Path) -> Result<Vec<Problem>> {
    let mut problems = Vec::new();
    for section in sorted_dirs(content)? {
        let section_id = strip_order(&section).to_string();
        for dir in sorted_dirs(&section)? {
            let meta: toml::Table = toml::from_str(&fs::read_to_string(dir.join("problem.toml"))?)?;
            let difficulty = meta.get("difficulty").and_then(|d| d.as_str()).unwrap_or("medium").to_string();
            problems.push(Problem { id: strip_order(&dir).to_string(), section: section_id.clone(), dir, difficulty });
        }
    }
    Ok(problems)
}

/// The reference solution as a working file: without the `mod tests;` that
/// gauntlet attaches itself.
fn working(p: &Problem) -> Result<String> {
    let solution = fs::read_to_string(p.dir.join("solution.rs"))?;
    let code = solution.trim_end().strip_suffix(TESTS_MOD).ok_or_else(|| eyre!("{}: no tests module", p.id))?;
    Ok(format!("{}\n", code.trim_end()))
}

const TESTS_MOD: &str = "#[cfg(test)]\n#[path = \"tests.rs\"]\nmod tests;";

fn sorted_dirs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> =
        fs::read_dir(dir)?.filter_map(|e| Some(e.ok()?.path())).filter(|p| p.is_dir()).collect();
    dirs.sort();
    Ok(dirs)
}

/// `content/05_two_pointers` → `two_pointers`.
fn strip_order(dir: &Path) -> &str {
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    name.split_once('_').map_or(name, |(_, rest)| rest)
}

fn rustc_version() -> Result<String> {
    let out = Command::new("rustc").arg("--version").output()?;
    let text = String::from_utf8_lossy(&out.stdout);
    Ok(text.lines().next().unwrap_or_default().to_string())
}

/// What gauntlet hashes into a reference measurement's key: the solution and
/// the tests, each after its length (FNV-1a, as `dirs::Fnv`).
fn key_hash(dir: &Path) -> Result<u64> {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |bytes: &[u8]| {
        for &b in bytes {
            hash = (hash ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    };
    for file in ["solution.rs", "tests.rs"] {
        let bytes = fs::read(dir.join(file))?;
        feed(&(bytes.len() as u64).to_le_bytes());
        feed(&bytes);
    }
    Ok(hash)
}

fn reference_perf(rng: &mut Rng, difficulty: &str) -> (u64, u64) {
    let (ms, mb) = match difficulty {
        "easy" => (30 + rng.below(60), 2 + rng.below(4)),
        "hard" => (120 + rng.below(400), 6 + rng.below(30)),
        _ => (60 + rng.below(180), 3 + rng.below(12)),
    };
    (ms, mb * MB + rng.below(MB))
}

/// A passing run of a solution: near the reference, or `slowness` times it.
fn solve_perf(rng: &mut Rng, reference: &Value, slowness: f64) -> (u64, u64) {
    let ms = reference["test_ms"].as_u64().unwrap_or(100) as f64;
    let bytes = reference["peak_bytes"].as_u64().unwrap_or(4 * MB) as f64;
    let near = |rng: &mut Rng| 0.9 + rng.below(70) as f64 / 100.0;
    let ms = ms * near(rng) * slowness + if slowness > 1.0 { 400.0 } else { 0.0 };
    ((ms as u64), (bytes * near(rng)) as u64)
}

/// Agreement with "I found this problem to be trivial."
fn rating(rng: &mut Rng, difficulty: &str) -> u8 {
    let choices: &[u8] = match difficulty {
        "easy" => &[3, 4, 4, 5, 5, 5],
        "hard" => &[3, 3, 3, 4, 4],
        _ => &[3, 3, 4, 4, 4, 5],
    };
    choices[rng.below(choices.len() as u64) as usize]
}

/// Seconds a problem takes in a first run.
fn run_secs(rng: &mut Rng, difficulty: &str) -> f64 {
    let (base, spread) = match difficulty {
        "easy" => (150, 240),
        "hard" => (900, 900),
        _ => (360, 540),
    };
    (base + rng.below(spread)) as f64 + rng.below(10) as f64 / 10.0
}

fn minutes(rng: &mut Rng, lo: u64, hi: u64) -> Duration {
    Duration::seconds((lo * 60 + rng.below((hi - lo) * 60)) as i64)
}

#[derive(Default)]
struct Log(Vec<String>);

impl Log {
    fn push(&mut self, at: DateTime<Utc>, event: Value) {
        let mut entry = json!({ "at": at.format("%Y-%m-%dT%H:%M:%SZ").to_string() });
        entry.as_object_mut().unwrap().extend(event.as_object().unwrap().clone());
        self.0.push(format!("{entry}\n"));
    }

    fn solve(&mut self, at: DateTime<Utc>, problem: &str, run: Option<(usize, f64)>, (ms, bytes): (u64, u64)) {
        let mut event = json!({ "kind": "solved", "problem": problem });
        if let Some((run, secs)) = run {
            event["run"] = json!(run);
            event["secs"] = json!((secs * 10.0).round() / 10.0);
        }
        event["test_ms"] = json!(ms);
        event["peak_bytes"] = json!(bytes);
        self.push(at, event);
    }
}

/// SplitMix64: the same history every time, without a dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }

    fn chance(&mut self, p: f64) -> bool {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64 <= p
    }
}
