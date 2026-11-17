//! The raw data: an append-only log of events, one JSON object per line, in
//! `<data>/events.jsonl`. Everything you see — status, ratings, runs, PBs — is
//! derived from this file by [`crate::db`].
//!
//! ```text
//! {"at":"2026-11-11T17:11:41Z","kind":"solved","problem":"three_sum","test_ms":842,"peak_bytes":42991616}
//! {"at":"2026-11-11T17:11:49Z","kind":"rated","problem":"three_sum","rating":4}
//! {"at":"2026-11-11T18:02:10Z","kind":"redo","problem":"three_sum"}
//! {"at":"2026-11-12T09:00:00Z","kind":"run_started","run":1,"scope":"two_pointers","problems":["two_sum_ii","three_sum"]}
//! {"at":"2026-11-12T09:04:10Z","kind":"solved","problem":"two_sum_ii","run":1,"secs":250.0,"test_ms":95,"peak_bytes":3014656}
//! {"at":"2026-11-12T09:09:30Z","kind":"solved","problem":"three_sum","run":1,"secs":318.5,"test_ms":231,"peak_bytes":12812288}
//! {"at":"2026-11-12T09:09:30Z","kind":"run_finished","run":1}
//! ```

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use color_eyre::eyre::{Result, WrapErr};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::perf::Perf;
use crate::ui::user;

/// Agreement with "I found this problem to be trivial.", on a 1–5 Likert
/// scale. Given after each completionist solve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct Rating(u8);

impl Rating {
    pub const STATEMENT: &str = "I found this problem to be trivial.";
    pub const ALL: [Rating; 5] = [Rating(1), Rating(2), Rating(3), Rating(4), Rating(5)];

    pub fn new(n: u8) -> Option<Rating> {
        (1..=5).contains(&n).then_some(Rating(n))
    }

    /// 1..=5.
    pub fn value(self) -> u8 {
        self.0
    }

    pub fn label(self) -> &'static str {
        match self.0 {
            1 => "Strongly disagree",
            2 => "Disagree",
            3 => "Neutral",
            4 => "Agree",
            _ => "Strongly agree",
        }
    }

    /// The key that selects this rating: `1`–`5`.
    pub fn key(self) -> char {
        char::from(b'0' + self.0)
    }

    pub fn from_key(c: char) -> Option<Rating> {
        Rating::new(u8::try_from(c.to_digit(10)?).ok()?)
    }
}

impl TryFrom<u8> for Rating {
    type Error = String;
    fn try_from(n: u8) -> Result<Rating, String> {
        Rating::new(n).ok_or_else(|| format!("rating must be 1..=5, got {n}"))
    }
}

impl From<Rating> for u8 {
    fn from(r: Rating) -> u8 {
        r.0
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    /// A problem's tests passed. Completionist: `run` and `secs` absent.
    /// Gauntlet: `run` is the run id and `secs` the time charged to the
    /// problem in that run. Either way, how long the tests took and their
    /// peak memory (`test_ms`, `peak_bytes`), where measured.
    Solved {
        problem: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        run: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        secs: Option<f64>,
        #[serde(flatten)]
        perf: Perf,
    },
    /// Applies to the problem's latest completionist solve.
    Rated { problem: String, rating: Rating },
    RunStarted {
        run: u32,
        /// Canonical scope: `all`, a section, or a problem id.
        scope: String,
        /// The run's problems, in order.
        problems: Vec<String>,
    },
    /// Every problem of the run is solved.
    RunFinished { run: u32 },
    /// Abandoned. Gone from run stats and PBs; its solves still count as
    /// completion and activity.
    RunDeleted { run: u32 },
    /// The completionist working file was reset to the template: the next
    /// completionist pass is a new solve.
    Redo { problem: String },
    /// A pass that wasn't a new solve beat the problem's best runtime or
    /// memory: the measurements of the better solution.
    Improved {
        problem: String,
        #[serde(flatten)]
        perf: Perf,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub at: Timestamp,
    #[serde(flatten)]
    pub event: Event,
}

pub struct EventLog {
    path: PathBuf,
}

impl EventLog {
    /// Open the log and read every entry. A last line cut short (gauntlet or
    /// the machine died mid-append) is dropped, and cut from the file so the
    /// next append doesn't land on the end of it. Any other line that doesn't
    /// parse is an error: the log has exactly one format.
    pub fn open(path: PathBuf) -> Result<(EventLog, Vec<Entry>)> {
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e).wrap_err_with(|| format!("reading {}", path.display())),
        };
        let mut entries = Vec::new();
        let mut start = 0;
        for (n, line) in bytes.split(|&b| b == b'\n').enumerate() {
            let next = start + line.len() + 1;
            if !line.trim_ascii().is_empty() {
                match serde_json::from_slice::<Entry>(line) {
                    Ok(entry) => entries.push(entry),
                    Err(e) if torn(line, &bytes[start..]) => {
                        tracing::warn!("{}:{}: dropping a partly written event: {e}", path.display(), n + 1);
                        truncate(&path, start)?;
                        break;
                    }
                    Err(e) => {
                        // serde_json's own position is within the line: misleading here.
                        let e = e.to_string();
                        let why = e.split(" at line ").next().unwrap_or(&e);
                        return Err(user(format!(
                            "{}:{}: unreadable event ({why}); fix or delete that line",
                            path.display(),
                            n + 1
                        )));
                    }
                }
            }
            start = next;
        }
        // A whole last event without its newline: the next append needs one.
        if bytes.last().is_some_and(|&b| b != b'\n') && start > bytes.len() {
            let mut file = OpenOptions::new().append(true).open(&path)?;
            file.write_all(b"\n").wrap_err_with(|| format!("repairing {}", path.display()))?;
        }
        Ok((EventLog { path }, entries))
    }

    pub fn append(&self, entry: &Entry) -> Result<()> {
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .wrap_err_with(|| format!("opening {}", self.path.display()))?;
        // One write of the whole line (`File` is unbuffered), so a crash
        // leaves at most a cut-short last line, which `open` drops.
        let mut line = serde_json::to_string(entry)?;
        line.push('\n');
        file.write_all(line.as_bytes())?;
        Ok(())
    }
}

/// Whether `line`, which doesn't parse, was cut short by a crash: it's the
/// last thing in the log (`rest` is it and what follows) and either never got
/// its newline or isn't even JSON. A whole line of valid JSON that isn't an
/// event (from a newer gauntlet, say) is not torn.
fn torn(line: &[u8], rest: &[u8]) -> bool {
    let after = &rest[line.len()..];
    after.trim_ascii().is_empty()
        && (after.is_empty() || serde_json::from_slice::<serde::de::IgnoredAny>(line).is_err())
}

/// Cut the log back to its first `len` bytes.
fn truncate(path: &Path, len: usize) -> Result<()> {
    OpenOptions::new()
        .write(true)
        .open(path)
        .and_then(|f| f.set_len(len as u64))
        .wrap_err_with(|| format!("repairing {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(json: &str) -> Entry {
        let entry: Entry = serde_json::from_str(json).unwrap();
        assert_eq!(serde_json::to_string(&entry).unwrap(), json);
        entry
    }

    #[test]
    fn entries_round_trip_as_flat_json() {
        // Solves from before measurements, and with them.
        let e = round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"solved","problem":"a"}"#);
        assert_eq!(e.event, Event::Solved { problem: "a".into(), run: None, secs: None, perf: Perf::default() });
        let e = round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"solved","problem":"a","run":3,"secs":12.5}"#);
        assert_eq!(
            e.event,
            Event::Solved { problem: "a".into(), run: Some(3), secs: Some(12.5), perf: Perf::default() }
        );
        let measured = Perf { ms: Some(842), bytes: Some(42_991_616) };
        let e = round_trip(
            r#"{"at":"2026-11-11T17:11:41Z","kind":"solved","problem":"a","test_ms":842,"peak_bytes":42991616}"#,
        );
        assert_eq!(e.event, Event::Solved { problem: "a".into(), run: None, secs: None, perf: measured });
        let e = round_trip(
            r#"{"at":"2026-11-11T17:11:41Z","kind":"solved","problem":"a","run":3,"secs":12.5,"test_ms":842}"#,
        );
        let perf = Perf { bytes: None, ..measured };
        assert_eq!(e.event, Event::Solved { problem: "a".into(), run: Some(3), secs: Some(12.5), perf });
        let e = round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"rated","problem":"a","rating":4}"#);
        assert_eq!(e.event, Event::Rated { problem: "a".into(), rating: Rating(4) });
        round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"run_started","run":2,"scope":"stack","problems":["a"]}"#);
        round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"run_finished","run":2}"#);
        round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"run_deleted","run":2}"#);
        round_trip(r#"{"at":"2026-11-11T17:11:41Z","kind":"redo","problem":"a"}"#);
        assert!(
            serde_json::from_str::<Entry>(r#"{"at":"2026-11-11T17:11:41Z","kind":"rated","problem":"a","rating":6}"#)
                .is_err()
        );
    }

    #[test]
    fn reads_timestamps_written_by_chrono() {
        // 0.1.0 wrote chrono's DateTime<Utc>: RFC 3339 with 0, 3, 6 or 9 fraction digits.
        let at = |s: &str| {
            let json = format!(r#"{{"at":"{s}","kind":"run_finished","run":1}}"#);
            serde_json::from_str::<Entry>(&json).unwrap().at
        };
        let whole: Timestamp = "2026-11-12T09:09:30Z".parse().unwrap();
        assert_eq!(at("2026-11-12T09:09:30Z"), whole);
        assert_eq!(at("2026-11-12T09:09:30+00:00"), whole);
        assert_eq!(at("2026-11-12T09:09:30.123Z"), whole + jiff::SignedDuration::from_millis(123));
        assert_eq!(at("2026-11-12T09:09:30.123456Z"), whole + jiff::SignedDuration::from_micros(123_456));
        assert_eq!(at("2026-11-12T09:09:30.123456789Z"), whole + jiff::SignedDuration::from_nanos(123_456_789));
    }

    #[test]
    fn rating_keys() {
        assert_eq!(Rating::from_key('3'), Rating::new(3));
        assert_eq!(Rating::from_key('6'), None);
        assert_eq!(Rating::from_key('0'), None);
        assert_eq!(Rating::ALL.map(Rating::key), ['1', '2', '3', '4', '5']);
    }

    #[test]
    fn log_appends_and_rejects_unknown_lines() {
        let dir = std::env::temp_dir().join(format!("gauntlet-store-{}", std::process::id()));
        let path = dir.join("events.jsonl");
        let _ = fs::remove_dir_all(&dir);
        let (log, entries) = EventLog::open(path.clone()).unwrap();
        assert!(entries.is_empty());
        let e = Entry { at: Timestamp::now(), event: Event::RunFinished { run: 1 } };
        log.append(&e).unwrap();
        fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap()
            .write_all(b"{\"at\":\"2026-11-11T00:00:00Z\",\"kind\":\"new_round\"}\n")
            .unwrap();
        let err = EventLog::open(path).err().expect("an unknown event is an error");
        assert!(err.chain().any(|c| c.is::<crate::ui::UserError>()), "{err:?}");
        let err = format!("{err:#}");
        assert!(err.contains("events.jsonl:2: unreadable event (unknown variant `new_round`"), "{err}");
        assert!(!err.contains('\n'), "{err}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_torn_last_line_is_dropped_and_cut_off() {
        let dir = std::env::temp_dir().join(format!("gauntlet-store-torn-{}", std::process::id()));
        let path = dir.join("events.jsonl");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let whole = "{\"at\":\"2026-11-11T00:00:00Z\",\"kind\":\"run_finished\",\"run\":1}\n";
        let e = Entry { at: Timestamp::now(), event: Event::RunFinished { run: 2 } };

        // Cut mid-line (and mid-character), then the next append: a line of its own.
        for torn in ["{\"at\":\"2026-11-11T00:00:01Z\",\"ki", "{\"problem\":\"\u{e9}", "\0\0\0\0\n"] {
            let torn = &torn.as_bytes()[..torn.len() - usize::from(torn.ends_with('\u{e9}'))];
            fs::write(&path, [whole.as_bytes(), torn].concat()).unwrap();
            let (log, entries) = EventLog::open(path.clone()).unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(fs::read_to_string(&path).unwrap(), whole);
            log.append(&e).unwrap();
            assert_eq!(EventLog::open(path.clone()).unwrap().1.len(), 2);
        }

        // Whole but for its newline: kept, and the newline added.
        fs::write(&path, whole.trim_end()).unwrap();
        let (log, entries) = EventLog::open(path.clone()).unwrap();
        assert_eq!(entries.len(), 1);
        log.append(&e).unwrap();
        assert_eq!(EventLog::open(path.clone()).unwrap().1.len(), 2);

        // Garbage before the last line is corruption, not a torn append.
        fs::write(&path, format!("{whole}{{\"at\":\n{whole}")).unwrap();
        let err = format!("{:#}", EventLog::open(path.clone()).err().expect("corrupt"));
        assert!(err.contains("events.jsonl:2: unreadable event") && !err.contains("line 1 column"), "{err}");
        let _ = fs::remove_dir_all(&dir);
    }
}
