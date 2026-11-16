//! Human-friendly formatting for durations, dates and sizes.

use std::time::Duration;

use chrono::{DateTime, Local, Utc};

/// A stopwatch reading: `4:12`, `1:03:07`.
pub fn clock(d: Duration) -> String {
    let s = d.as_secs();
    if s < 3600 {
        format!("{}:{:02}", s / 60, s % 60)
    } else {
        format!("{}:{:02}:{:02}", s / 3600, s % 3600 / 60, s % 60)
    }
}

/// A run's clock, always with hours so it keeps its width past the first:
/// `0:04:12`, `1:03:07`.
pub fn hms(d: Duration) -> String {
    let s = d.as_secs();
    format!("{}:{:02}:{:02}", s / 3600, s % 3600 / 60, s % 60)
}

/// `n/total` with `n` padded to the width of `total`, so it keeps its width
/// as `n` grows: ` 9/128`.
pub fn count(n: usize, total: usize) -> String {
    format!("{n:>w$}/{total}", w = total.to_string().len())
}

/// A test run's runtime, six columns wide whatever it is: `  4 ms`,
/// `842 ms`, `1.25 s`, `12.3 s`.
pub fn runtime(ms: u64) -> String {
    let s = ms as f64 / 1000.0;
    if ms < 1_000 {
        format!("{ms:>3} ms")
    } else if ms < 9_995 {
        format!("{s:.2} s")
    } else {
        format!("{s:>4.1} s")
    }
}

/// Peak memory, six columns wide: `2.5 MB`, ` 41 MB`, `1.2 GB` (of 2^20 and 2^30 bytes).
pub fn memory(bytes: u64) -> String {
    let mb = bytes as f64 / f64::from(1 << 20);
    if mb < 9.95 {
        format!("{mb:.1} MB")
    } else if mb < 999.5 {
        format!("{mb:>3.0} MB")
    } else {
        format!("{:.1} GB", mb / 1024.0)
    }
}

/// How many times something else: `1.3×`, ` 12×`, four columns up to 999×.
pub fn ratio(r: f64) -> String {
    if r < 9.95 { format!("{r:.1}×") } else { format!("{r:>3.0}×") }
}

/// `2026-11-11 17:42` in local time.
pub fn local(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string()
}

/// Seconds as a stopwatch reading: `4:12`.
pub fn secs(s: f64) -> String {
    clock(Duration::from_secs_f64(s.max(0.0)))
}

/// A split against a PB, `+` meaning behind: `-0:12`, `+1:03`.
pub fn delta(behind_by: f64) -> String {
    let sign = if behind_by < 0.0 { '-' } else { '+' };
    format!("{sign}{}", secs(behind_by.abs()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats() {
        assert_eq!(clock(Duration::from_secs(252)), "4:12");
        assert_eq!(clock(Duration::from_secs(3787)), "1:03:07");
        assert_eq!(runtime(4), "  4 ms");
        assert_eq!(runtime(842), "842 ms");
        assert_eq!(runtime(1250), "1.25 s");
        assert_eq!(runtime(9_994), "9.99 s");
        assert_eq!(runtime(9_996), "10.0 s");
        assert_eq!(runtime(15_000), "15.0 s");
        for (bytes, shown) in
            [(2_621_440, "2.5 MB"), (43_000_000, " 41 MB"), (136_314_880, "130 MB"), (1 << 30, "1.0 GB")]
        {
            assert_eq!(memory(bytes), shown);
        }
        assert_eq!([ratio(1.26), ratio(10.5), ratio(120.0)], ["1.3×", " 10×", "120×"]);
        assert_eq!(secs(252.7), "4:12");
        assert_eq!(delta(-12.0), "-0:12");
        assert_eq!(delta(63.0), "+1:03");
        assert_eq!(hms(Duration::from_secs(252)), "0:04:12");
        assert_eq!(hms(Duration::from_secs(3787)), "1:03:07");
        assert_eq!(count(9, 128), "  9/128");
        assert_eq!(count(10, 128), " 10/128");
    }
}
