//! Where gauntlet keeps things. Nothing lives next to the binary or in the
//! checkout it was built from:
//!
//! ```text
//! data: $XDG_DATA_HOME/gauntlet (~/.local/share/gauntlet; Windows: %APPDATA%\gauntlet)
//!     <problem>.rs                 your working file: starts as a copy of the template
//!     runs/<run>/<problem>.rs      a run's files, kept after it ends
//!     run.json                     the run in progress
//!     events.jsonl                 everything you've done, one event per line
//!     rust-project.json            lists the working files for rust-analyzer
//!     .lock                        held by the gauntlet that has this directory open
//! cache: $XDG_CACHE_HOME/gauntlet (~/.cache/gauntlet; Windows: %LOCALAPPDATA%\gauntlet)
//!     content/<version>-<hash>/    the problems, extracted from the binary
//!     content/<version>-<hash>.lock  held (shared) by every gauntlet using them
//!     build/                       what the tests compile: your code with its tests attached
//!     reference.json               the reference solutions' runtimes and memory on this machine
//! state: $XDG_STATE_HOME/gauntlet (~/.local/state/gauntlet; the cache dir where there's
//!        none, as on Windows)
//!     gauntlet.log
//! ```
//!
//! XDG on macOS too. On Windows the XDG variables are honoured only when
//! set, which `cargo run` does (see `.cargo/config.toml`).
//!
//! The problems are embedded in the binary at build time and extracted on
//! first use, once per build: the directory name changes with the version
//! and the content. `$GAUNTLET_CONTENT_DIR` reads them from a directory
//! instead (a checkout's `content/`), as-is: nothing is ever written there.

use std::ffi::OsString;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime};

use color_eyre::eyre::{Result, WrapErr};
use include_dir::{Dir, include_dir};

use crate::ui::user;

/// The problems, as they were when this binary was built.
static CONTENT: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/content");

const APP: &str = "gauntlet";

pub const EVENTS: &str = "events.jsonl";
pub const RUN_STATE: &str = "run.json";
pub const RUST_PROJECT: &str = "rust-project.json";
pub const RUNS: &str = "runs";
pub const REFERENCE: &str = "reference.json";
const LOCK: &str = ".lock";

/// The XDG base directories, before gauntlet's name is added.
pub struct Base {
    pub data: PathBuf,
    pub cache: PathBuf,
    /// Not every platform has one.
    pub state: Option<PathBuf>,
}

impl Base {
    /// XDG everywhere, macOS included; on Windows, `%APPDATA%` for data and
    /// `%LOCALAPPDATA%` for the cache (and log), unless XDG variables are set.
    pub fn detect() -> Result<Base> {
        use etcetera::BaseStrategy;
        let s = etcetera::choose_base_strategy().wrap_err("finding the home directory")?;
        let base = Base { data: s.data_dir(), cache: s.cache_dir(), state: s.state_dir() };
        // etcetera's XDG strategy reads them already.
        Ok(if cfg!(windows) { base.with_xdg(|key| std::env::var_os(key)) } else { base })
    }

    /// Each XDG base directory variable that's set to an absolute path
    /// replaces its directory.
    fn with_xdg(self, env: impl Fn(&str) -> Option<OsString>) -> Base {
        let var = |key: &str| env(key).map(PathBuf::from).filter(|p| p.is_absolute());
        Base {
            data: var("XDG_DATA_HOME").unwrap_or(self.data),
            cache: var("XDG_CACHE_HOME").unwrap_or(self.cache),
            state: var("XDG_STATE_HOME").or(self.state),
        }
    }
}

/// Where gauntlet keeps things: see the module docs. All absolute.
#[derive(Clone, Debug)]
pub struct Dirs {
    /// Your working files, runs and progress.
    pub data: PathBuf,
    /// `$GAUNTLET_CONTENT_DIR`: the problems are read from here instead of
    /// the copy embedded in the binary.
    pub content_override: Option<PathBuf>,
    /// Extracted content and compiled tests: safe to delete.
    pub cache: PathBuf,
    /// The log.
    pub state: PathBuf,
}

impl Dirs {
    pub fn from_env() -> Result<Dirs> {
        Ok(Dirs::resolve(|key| std::env::var_os(key), &Base::detect()?))
    }

    /// `env` looks up an environment variable; empty counts as unset.
    pub fn resolve(env: impl Fn(&str) -> Option<OsString>, base: &Base) -> Dirs {
        let var = |key: &str| env(key).filter(|v| !v.is_empty()).map(|v| absolute(Path::new(&v)));
        let cache = base.cache.join(APP);
        Dirs {
            data: base.data.join(APP),
            content_override: var("GAUNTLET_CONTENT_DIR"),
            state: base.state.as_ref().map_or_else(|| cache.clone(), |s| s.join(APP)),
            cache,
        }
    }

    /// Scratch build artifacts.
    pub fn build_dir(&self) -> PathBuf {
        self.cache.join("build")
    }

    pub fn log_file(&self) -> PathBuf {
        self.state.join("gauntlet.log")
    }

    /// Where the problems are read from: `$GAUNTLET_CONTENT_DIR`, or the
    /// embedded copy, extracted if it isn't already.
    pub fn content_dir(&self) -> Result<PathBuf> {
        if let Some(dir) = &self.content_override {
            return Ok(dir.clone());
        }
        let root = self.cache.join("content");
        let files = embedded();
        let (dir, in_use) = extract(&files, &root, &content_key(&files))
            .wrap_err_with(|| format!("extracting the problems into {}", show(&root)))?;
        // Held until gauntlet exits, so no other version removes them meanwhile.
        static IN_USE: Mutex<Vec<File>> = Mutex::new(Vec::new());
        IN_USE.lock().unwrap_or_else(|e| e.into_inner()).push(in_use);
        Ok(dir)
    }
}

fn absolute(path: &Path) -> PathBuf {
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
}

/// `path` for showing: the home directory abbreviated to `~`, except on
/// Windows, where `~` means nothing to the shell or Explorer.
pub fn show(path: &Path) -> String {
    static HOME: OnceLock<Option<PathBuf>> = OnceLock::new();
    let home = HOME.get_or_init(|| if cfg!(windows) { None } else { etcetera::home_dir().ok() });
    show_from(path, home.as_deref())
}

fn show_from(path: &Path, home: Option<&Path>) -> String {
    match home.and_then(|h| path.strip_prefix(h).ok()) {
        Some(rest) if rest.as_os_str().is_empty() => "~".into(),
        Some(rest) => format!("~/{}", rest.display()),
        None => path.display().to_string(),
    }
}

// ---- The embedded problems --------------------------------------------------

/// Every embedded file: its path relative to `content/`, and its bytes. Sorted.
fn embedded() -> Vec<(&'static Path, &'static [u8])> {
    fn walk(dir: &'static Dir<'static>, out: &mut Vec<(&'static Path, &'static [u8])>) {
        out.extend(dir.files().map(|f| (f.path(), f.contents())));
        for sub in dir.dirs() {
            walk(sub, out);
        }
    }
    let mut files = Vec::new();
    walk(&CONTENT, &mut files);
    files.sort_by_key(|&(path, _)| path);
    files
}

/// Names one build's content: `0.1.0-<hash of every path and file>`.
fn content_key(files: &[(&Path, &[u8])]) -> String {
    let mut hash = Fnv::default();
    for (path, bytes) in files {
        hash.feed(path.to_string_lossy().as_bytes());
        hash.feed(&[0]);
        hash.feed_len_and(bytes);
    }
    format!("{}-{:016x}", env!("CARGO_PKG_VERSION"), hash.0)
}

/// FNV-1a: a stable hash, unlike std's, for naming what's cached.
pub struct Fnv(pub u64);

impl Default for Fnv {
    fn default() -> Fnv {
        Fnv(0xcbf2_9ce4_8422_2325)
    }
}

impl Fnv {
    pub fn feed(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }

    /// `bytes` after its length, so consecutive pieces can't run together.
    pub fn feed_len_and(&mut self, bytes: &[u8]) {
        self.feed(&(bytes.len() as u64).to_le_bytes());
        self.feed(bytes);
    }
}

/// Write `files` to `<root>/<key>` unless it's already there, and return
/// that directory with a shared lock on `<root>/<key>.lock`: hold it while
/// using the directory. Written to a temporary directory and renamed into
/// place, so a crash or a second gauntlet starting at the same moment never
/// leaves half a tree. Other keys' directories are removed, unless another
/// gauntlet (another version) holds their lock.
fn extract(files: &[(&Path, &[u8])], root: &Path, key: &str) -> io::Result<(PathBuf, File)> {
    let target = root.join(key);
    fs::create_dir_all(root)?;
    // Taken first: a gauntlet removing this key's directory holds the lock
    // exclusively while it does, so once this returns the directory is
    // either whole or gone.
    let in_use = open_lock(&root.join(format!("{key}.lock")))?;
    fs4::FileExt::lock_shared(&in_use)?;
    if !target.is_dir() {
        let tmp = root.join(format!(".tmp-{key}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let written = files.iter().try_for_each(|(rel, bytes)| {
            let path = tmp.join(rel);
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }
            fs::write(path, bytes)
        });
        let renamed = written.and_then(|()| rename_dir(&tmp, &target));
        if let Err(e) = renamed {
            let _ = fs::remove_dir_all(&tmp);
            // Another gauntlet got there first.
            if !target.is_dir() {
                return Err(e);
            }
        }
    }
    remove_other_keys(root, key);
    Ok((target, in_use))
}

/// Rename the directory `from` to `to`. Windows refuses while anything has a
/// file in `from` open, which a virus scanner looking at the files just
/// written may, for a moment: so that's retried. A `to` that exists is an
/// error everywhere.
fn rename_dir(from: &Path, to: &Path) -> io::Result<()> {
    let mut tries = 0;
    loop {
        match fs::rename(from, to) {
            Err(e) if tries < 20 && e.kind() == io::ErrorKind::PermissionDenied && !to.exists() => {
                tries += 1;
                thread::sleep(Duration::from_millis(50));
            }
            renamed => return renamed,
        }
    }
}

/// Remove the content directories (and their locks) of keys other than
/// `key` that no gauntlet is using, and temporary directories abandoned for
/// an hour. Best effort: whatever can't be removed now is tried next time.
fn remove_other_keys(root: &Path, key: &str) {
    let Ok(entries) = fs::read_dir(root) else { return };
    let an_hour_ago = SystemTime::now() - Duration::from_secs(3600);
    let mine = format!("{key}.lock");
    for entry in entries.filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if name == key || name == mine {
            continue;
        }
        if name.starts_with(".tmp-") {
            // Someone else's extraction in progress, unless it's been abandoned.
            if entry.metadata().and_then(|m| m.modified()).is_ok_and(|t| t < an_hour_ago) {
                let _ = fs::remove_dir_all(&path);
            }
            continue;
        }
        // A key's directory, or the lock of one whose directory is gone.
        let other = name.strip_suffix(".lock").unwrap_or(&name);
        let lock_path = root.join(format!("{other}.lock"));
        let Ok(lock) = open_lock(&lock_path) else { continue };
        if fs4::FileExt::try_lock(&lock).is_ok() {
            let _ = fs::remove_dir_all(root.join(other));
            // Deleted while still held, so no one can lock it in between. On
            // Windows the file goes when `lock` is closed.
            let _ = fs::remove_file(&lock_path);
        }
    }
}

fn open_lock(path: &Path) -> io::Result<File> {
    File::options().create(true).truncate(false).write(true).open(path)
}

// ---- The data directory -----------------------------------------------------

/// Held while a gauntlet has the data directory open: released on exit.
pub struct Lock {
    _file: File,
}

/// Take `<data>/.lock`, or fail with a one-line explanation if another
/// gauntlet holds it.
pub fn lock(data: &Path) -> Result<Lock> {
    fs::create_dir_all(data).wrap_err_with(|| format!("creating {}", show(data)))?;
    let path = data.join(LOCK);
    let file = open_lock(&path).wrap_err_with(|| format!("opening {}", show(&path)))?;
    // Spelled out: newer std has an inherent `File::try_lock` with another error type.
    match fs4::FileExt::try_lock(&file) {
        Ok(()) => Ok(Lock { _file: file }),
        Err(fs4::TryLockError::WouldBlock) => {
            Err(user(format!("another gauntlet already has {} open; close it first", show(data))))
        }
        Err(fs4::TryLockError::Error(e)) => Err(e).wrap_err_with(|| format!("locking {}", show(&path))),
    }
}

/// `gauntlet reset`: delete what gauntlet created in `data` — never anything
/// else, since it can point anywhere — then `data` itself if that leaves it
/// empty. Also empties `build`. `lock` is the caller's on `data`, released
/// here. Returns what was removed.
pub fn reset(data: &Path, build: &Path, lock: Lock) -> io::Result<Vec<PathBuf>> {
    let mut removed = Vec::new();
    let mut remove = |path: PathBuf| -> io::Result<()> {
        match fs::symlink_metadata(&path) {
            Ok(m) if m.is_dir() => fs::remove_dir_all(&path)?,
            Ok(_) => fs::remove_file(&path)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e),
        }
        removed.push(path);
        Ok(())
    };
    for name in [EVENTS, RUN_STATE, RUST_PROJECT, RUNS] {
        remove(data.join(name))?;
    }
    if data.is_dir() {
        for entry in fs::read_dir(data)?.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "rs") && entry.file_type().is_ok_and(|t| t.is_file()) {
                remove(path)?;
            }
        }
        // The lock goes only with the directory, and is deleted while still
        // held, so no other gauntlet can be locking a file that's no longer
        // there. Windows removes it once it's closed, so only then can the
        // directory go. Failing that, the directory stays.
        let rest: Vec<OsString> = fs::read_dir(data)?.filter_map(|e| Some(e.ok()?.file_name())).collect();
        if rest.iter().all(|name| name == LOCK) {
            let _ = fs::remove_file(data.join(LOCK));
            drop(lock);
            match fs::remove_dir(data) {
                // Only the lock was there: nothing to report.
                Ok(()) if !removed.is_empty() => removed.push(data.to_path_buf()),
                Ok(()) => {}
                Err(e) => tracing::warn!("removing {}: {e}", data.display()),
            }
        }
    }
    // Compiled tests: anything still running keeps its file, harmlessly.
    let _ = fs::remove_dir_all(build);
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::tests::Scratch;

    fn base() -> Base {
        Base {
            data: "/home/u/.local/share".into(),
            cache: "/home/u/.cache".into(),
            state: Some("/home/u/.local/state".into()),
        }
    }

    /// An absolute path on this platform: `/repo/content` has no drive on Windows.
    fn absolute_path(unix: &str) -> PathBuf {
        if cfg!(windows) { PathBuf::from(format!("C:{unix}")) } else { PathBuf::from(unix) }
    }

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<OsString> + 'a {
        move |key| vars.iter().find(|(k, _)| *k == key).map(|(_, v)| OsString::from(v))
    }

    #[test]
    fn xdg_unless_overridden() {
        let d = Dirs::resolve(env(&[]), &base());
        assert_eq!(d.data, Path::new("/home/u/.local/share/gauntlet"));
        assert_eq!(d.content_override, None);
        assert_eq!(d.build_dir(), Path::new("/home/u/.cache/gauntlet/build"));
        assert_eq!(d.log_file(), Path::new("/home/u/.local/state/gauntlet/gauntlet.log"));

        let repo = absolute_path("/repo/content");
        let repo_var = repo.to_string_lossy().into_owned();
        let d = Dirs::resolve(env(&[("GAUNTLET_CONTENT_DIR", &repo_var)]), &base());
        assert_eq!(d.data, Path::new("/home/u/.local/share/gauntlet"), "the data stays put");
        assert_eq!(d.content_override.as_deref(), Some(repo.as_path()));
        assert_eq!(d.content_dir().unwrap(), repo, "used as-is");
        assert_eq!(d.cache, Path::new("/home/u/.cache/gauntlet"), "the cache stays put");

        // Empty is unset; relative is made absolute; no state dir means the cache.
        let d = Dirs::resolve(env(&[("GAUNTLET_CONTENT_DIR", "")]), &base());
        assert_eq!(d.content_override, None);
        let d = Dirs::resolve(env(&[("GAUNTLET_CONTENT_DIR", "content")]), &Base { state: None, ..base() });
        assert_eq!(d.content_override, Some(std::env::current_dir().unwrap().join("content")));
        assert_eq!(d.log_file(), Path::new("/home/u/.cache/gauntlet/gauntlet.log"));
    }

    #[test]
    fn xdg_variables_win_on_windows_when_set() {
        let windows = || Base {
            data: r"C:\Users\A B\AppData\Roaming".into(),
            cache: r"C:\Users\A B\AppData\Local".into(),
            state: None,
        };
        let b = windows().with_xdg(env(&[]));
        assert_eq!(b.data, Path::new(r"C:\Users\A B\AppData\Roaming"));
        assert_eq!(b.state, None, "the log goes in the cache dir");
        let d = Dirs::resolve(env(&[]), &b);
        assert_eq!(d.log_file(), Path::new(r"C:\Users\A B\AppData\Local").join("gauntlet").join("gauntlet.log"));

        let (data, cache, state) =
            (absolute_path("/w/dev/data"), absolute_path("/w/dev/cache"), absolute_path("/w/st"));
        let (data_s, cache_s, state_s) = (data.to_string_lossy(), cache.to_string_lossy(), state.to_string_lossy());
        let vars = [("XDG_DATA_HOME", &*data_s), ("XDG_CACHE_HOME", &*cache_s), ("XDG_STATE_HOME", &*state_s)];
        let b = windows().with_xdg(env(&vars));
        assert_eq!((b.data, b.cache, b.state), (data, cache, Some(state)));
        // Relative or empty: ignored, as the XDG spec says.
        let b = windows().with_xdg(env(&[("XDG_DATA_HOME", "target/dev/data"), ("XDG_CACHE_HOME", "")]));
        assert_eq!(b.data, Path::new(r"C:\Users\A B\AppData\Roaming"));
        assert_eq!(b.cache, Path::new(r"C:\Users\A B\AppData\Local"));
    }

    #[test]
    fn home_is_abbreviated() {
        let home = Some(Path::new("/home/u"));
        assert_eq!(show_from(Path::new("/home/u/.local/share/gauntlet"), home), "~/.local/share/gauntlet");
        assert_eq!(show_from(Path::new("/home/u"), home), "~");
        assert_eq!(show_from(Path::new("/home/user2/x"), home), "/home/user2/x");
        assert_eq!(show_from(Path::new("/tmp/x"), None), "/tmp/x");
    }

    #[test]
    fn extracts_once_per_key() {
        let s = Scratch::new("extract");
        let root = s.0.join("content");
        let v1: [(&Path, &[u8]); 2] =
            [(Path::new("01_a/section.toml"), b"title = \"A\"\n"), (Path::new("top.txt"), b"x")];
        let key1 = content_key(&v1);
        assert!(key1.starts_with(concat!(env!("CARGO_PKG_VERSION"), "-")));

        // Fresh.
        let (dir, held1) = extract(&v1, &root, &key1).unwrap();
        assert_eq!(dir, root.join(&key1));
        assert_eq!(fs::read_to_string(dir.join("01_a/section.toml")).unwrap(), "title = \"A\"\n");
        let names = |root: &Path| -> Vec<String> {
            let mut v: Vec<String> =
                fs::read_dir(root).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
            v.sort();
            v
        };
        let lock1 = format!("{key1}.lock");
        assert_eq!(names(&root), [key1.clone(), lock1.clone()], "no temporary directory left behind");

        // Already there: left alone, even if it differs.
        fs::write(dir.join("top.txt"), "edited").unwrap();
        let (_, again) = extract(&v1, &root, &key1).unwrap();
        assert_eq!(fs::read_to_string(dir.join("top.txt")).unwrap(), "edited");
        drop(again);

        // New content: a new key. The old one is still in use (held1), so it
        // stays, and so does someone else's extraction in progress.
        let v2: [(&Path, &[u8]); 1] = [(Path::new("top.txt"), b"y")];
        let key2 = content_key(&v2);
        assert_ne!(key1, key2);
        fs::create_dir_all(root.join(".tmp-other-1")).unwrap();
        let (dir2, held2) = extract(&v2, &root, &key2).unwrap();
        assert_eq!(fs::read_to_string(dir2.join("top.txt")).unwrap(), "y");
        let lock2 = format!("{key2}.lock");
        let mut all = vec![".tmp-other-1".to_string(), key1.clone(), lock1, key2.clone(), lock2.clone()];
        all.sort();
        assert_eq!(names(&root), all, "the old content is in use");

        // Once it isn't, the next start removes it, lock and all.
        drop(held1);
        drop(extract(&v2, &root, &key2).unwrap());
        assert_eq!(names(&root), [".tmp-other-1".to_string(), key2, lock2]);
        drop(held2);
    }

    #[test]
    fn embedded_content_is_the_problems() {
        let files = embedded();
        assert!(files.iter().filter(|(p, _)| p.ends_with("problem.toml")).count() >= 100);
        assert!(files.windows(2).all(|w| w[0].0 < w[1].0), "sorted");
        assert_eq!(content_key(&files), content_key(&embedded()), "stable");
    }

    #[test]
    fn a_second_lock_is_refused() {
        let s = Scratch::new("lock");
        let data = s.0.join("data");
        let held = lock(&data).unwrap();
        let err = lock(&data).err().expect("already held");
        assert!(err.downcast_ref::<crate::ui::UserError>().is_some());
        assert!(err.to_string().contains("already has") && err.to_string().contains("data"), "{err}");
        drop(held);
        // Another test's child process can hold a copy of the descriptor for a
        // moment between its fork and exec.
        let relocked = (0..100).any(|_| {
            let ok = lock(&data).is_ok();
            if !ok {
                std::thread::sleep(Duration::from_millis(10));
            }
            ok
        });
        assert!(relocked, "released when dropped");
    }

    #[test]
    fn reset_deletes_only_what_gauntlet_made() {
        let s = Scratch::new("reset");
        let ours = ["events.jsonl", "run.json", "rust-project.json", "two_sum.rs", "runs/3/dup.rs"];
        for f in ours.map(|f| format!("data/{f}")) {
            s.write(&f, "x");
        }
        s.write("data/notes.md", "mine");
        s.write("data/sub/keep.rs", "mine");
        s.write("build/two_sum", "bin");
        let (data, build) = (s.0.join("data"), s.0.join("build"));
        let removed = reset(&data, &build, lock(&data).unwrap()).unwrap();
        assert_eq!(removed.len(), ours.len(), "runs/ counts once: {removed:?}");
        let mut left: Vec<String> =
            fs::read_dir(&data).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        left.sort();
        assert_eq!(left, [".lock", "notes.md", "sub"]);
        assert_eq!(fs::read_to_string(data.join("sub/keep.rs")).unwrap(), "mine");
        assert!(!build.exists());

        // Nothing else there: the directory goes too.
        fs::remove_file(data.join("notes.md")).unwrap();
        fs::remove_dir_all(data.join("sub")).unwrap();
        s.write("data/two_sum.rs", "x");
        let removed = reset(&data, &build, lock(&data).unwrap()).unwrap();
        assert!(!data.exists());
        assert_eq!(removed, [data.join("two_sum.rs"), data.clone()]);
        // Nothing to reset (taking the lock made the directory): nothing reported.
        let removed = reset(&data, &build, lock(&data).unwrap()).expect("nothing to reset is fine");
        assert_eq!(removed, Vec::<PathBuf>::new());
        assert!(!data.exists());
    }
}
