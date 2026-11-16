//! Problem discovery and the on-disk layout.
//!
//! ```text
//! content/NN_section/section.toml     title, description, drill
//! content/NN_section/NN_slug/
//!     problem.toml   title, difficulty, complexity, hints
//!     README.md      statement
//!     template.rs    pristine starting point
//!     solution.rs    reference solution
//!     tests.rs       tests shared by both
//! <data>/<slug>.rs                     your working file (a copy of the template)
//! <data>/runs/<run>/<slug>.rs          a run's files: fresh templates, kept after it ends
//! <data>/run.json                      live state of the run in progress
//! <data>/events.jsonl                  everything you've done, one event per line
//! <data>/rust-project.json             lists the working files for rust-analyzer
//! ```
//!
//! Where content and the data directory are is up to [`crate::dirs`].
//!
//! Template and solution end with `#[cfg(test)] #[path = "tests.rs"] mod
//! tests;`, so they compile, tests and all, where they are. Working files
//! don't have it: they're only your code, and the runner attaches the tests
//! when it compiles them (see [`runner::attach_tests`]).
//!
//! Numeric prefixes (`NN_`) only set the order. A section's or problem's id
//! is its directory name with the prefix stripped (`two_pointers`,
//! `three_sum`), so reordering, or moving a problem between sections, keeps
//! its history. Section and problem slugs share one namespace.
//!
//! Scopes are named canonically: `all`, a section (`trees`) or a problem
//! (`three_sum`). `all` ⊇ section ⊇ problem.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use color_eyre::eyre::{Result, WrapErr, bail};
use serde::Deserialize;

use crate::db::ALL;
use crate::dirs::{self, Dirs, show};
use crate::runner::{self, Env};
use crate::ui::user;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.label())
    }
}

/// `section.toml`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SectionMeta {
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    drill: bool,
}

/// `problem.toml`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Meta {
    title: String,
    difficulty: Difficulty,
    /// Markdown: the time and space to aim for.
    #[serde(default)]
    complexity: String,
    #[serde(default)]
    hints: Vec<Hint>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Hint {
    /// Markdown.
    text: String,
}

/// Where a workspace's files are (see [`crate::dirs`]).
#[derive(Clone, Debug)]
pub struct Roots {
    /// The problems.
    pub content: PathBuf,
    /// Your working files, events and run state.
    pub data: PathBuf,
    /// Scratch build artifacts.
    pub build: PathBuf,
    /// The reference solutions' measurements on this machine (see [`crate::perf`]).
    pub reference: PathBuf,
}

impl Roots {
    /// From the environment, extracting the embedded problems if need be.
    pub fn locate() -> Result<Roots> {
        let dirs = Dirs::from_env()?;
        let reference = dirs.cache.join(dirs::REFERENCE);
        Ok(Roots { content: dirs.content_dir()?, build: dirs.build_dir(), data: dirs.data, reference })
    }

    /// `<root>/content`, `<root>/data`, `<root>/build` and `<root>/reference.json`.
    #[cfg(test)]
    pub fn at(root: &Path) -> Roots {
        Roots {
            content: root.join("content"),
            data: root.join("data"),
            build: root.join("build"),
            reference: root.join(dirs::REFERENCE),
        }
    }

    pub fn events_file(&self) -> PathBuf {
        self.data.join(dirs::EVENTS)
    }

    pub fn run_state_file(&self) -> PathBuf {
        self.data.join(dirs::RUN_STATE)
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.data.join(dirs::RUNS)
    }
}

pub struct Section {
    /// `arrays_hashing`: also its id and scope name.
    pub slug: String,
    pub dir: PathBuf,
    pub title: String,
    pub description: String,
    /// Many small API exercises in one file: no complexity target. Otherwise
    /// data structures and algorithms, where `check` requires one.
    pub drill: bool,
}

pub const TEMPLATE: &str = "template.rs";
pub const SOLUTION: &str = "solution.rs";
pub const TESTS: &str = "tests.rs";
/// Files every problem must have.
pub const REQUIRED_FILES: [&str; 5] = ["problem.toml", "README.md", TEMPLATE, SOLUTION, TESTS];

pub struct Problem {
    /// Its slug, e.g. `contains_duplicate`: also its scope name.
    pub id: String,
    pub dir: PathBuf,
    /// Index into [`Workspace::sections`].
    pub section: usize,
    /// e.g. `Contains Duplicate`.
    pub title: String,
    pub difficulty: Difficulty,
    /// Markdown: the time and space to aim for. Empty for drills.
    pub complexity: String,
    /// Markdown, revealed one at a time, gentlest first.
    pub hints: Vec<String>,
}

impl Problem {
    pub fn readme(&self) -> PathBuf {
        self.dir.join("README.md")
    }

    pub fn template(&self) -> PathBuf {
        self.dir.join(TEMPLATE)
    }

    pub fn solution(&self) -> PathBuf {
        self.dir.join(SOLUTION)
    }

    pub fn tests(&self) -> PathBuf {
        self.dir.join(TESTS)
    }

    /// The reference solution as shown in the Info tab.
    pub fn solution_source(&self) -> String {
        runner::display_solution(&fs::read_to_string(self.solution()).unwrap_or_default())
    }

    /// The template without its test module: what a fresh working file contains.
    pub fn template_code(&self) -> io::Result<String> {
        Ok(runner::without_tests_mod(&fs::read_to_string(self.template())?))
    }

    /// The reference solution without its test module, for testing like a working file.
    pub fn solution_code(&self) -> io::Result<String> {
        Ok(runner::without_tests_mod(&fs::read_to_string(self.solution())?))
    }

    /// What line 1 of the template must be.
    pub fn template_header(&self) -> String {
        format!("// {} — press i in the gauntlet for the statement, t for the tests.", self.title)
    }
}

/// `01_arrays_hashing` → `arrays_hashing`.
pub fn strip_order(name: &str) -> &str {
    match name.split_once('_') {
        Some((n, rest)) if !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) => rest,
        _ => name,
    }
}

/// Subdirectories of `dir`, sorted by name.
fn subdirs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(dir).wrap_err_with(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn dir_name(dir: &Path) -> String {
    dir.file_name().map_or_else(String::new, |n| n.to_string_lossy().into_owned())
}

fn read_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    toml::from_str(&fs::read_to_string(path).wrap_err_with(|| format!("reading {}", path.display()))?)
        .wrap_err_with(|| format!("parsing {}", path.display()))
}

/// `to` relative to the directory `from`, e.g. `content/x/README.md` (for
/// the README's links). Falls back to `to` itself if they share no root.
pub fn relative_path(from: &Path, to: &Path) -> PathBuf {
    let from: Vec<Component> = from.components().collect();
    let to_c: Vec<Component> = to.components().collect();
    let common = from.iter().zip(&to_c).take_while(|(a, b)| a == b).count();
    if common == 0 {
        return to.to_path_buf();
    }
    let mut rel = PathBuf::new();
    for _ in common..from.len() {
        rel.push("..");
    }
    for c in &to_c[common..] {
        rel.push(c.as_os_str());
    }
    rel
}

/// Levenshtein distance, for suggesting what the player may have meant.
fn edit_distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut cur = vec![i + 1];
        for (j, &cb) in b.iter().enumerate() {
            cur.push((prev[j] + usize::from(ca != cb)).min(prev[j + 1] + 1).min(cur[j] + 1));
        }
        prev = cur;
    }
    prev[b.len()]
}

pub struct Workspace {
    pub roots: Roots,
    pub sections: Vec<Section>,
    /// In order: by section, then problem.
    pub problems: Vec<Problem>,
}

impl Workspace {
    pub fn discover() -> Result<Workspace> {
        Self::discover_roots(Roots::locate()?)
    }

    /// A workspace with everything under `root` (see [`Roots::at`]).
    #[cfg(test)]
    pub fn discover_in(root: PathBuf) -> Result<Workspace> {
        Self::discover_roots(Roots::at(&root))
    }

    pub fn discover_roots(roots: Roots) -> Result<Workspace> {
        let content_dir = roots.content.clone();
        let mut ws = Workspace { roots, sections: Vec::new(), problems: Vec::new() };
        if content_dir.is_dir() {
            for dir in subdirs(&content_dir)? {
                if dir.join("section.toml").is_file() {
                    ws.load_section(dir)?;
                }
            }
        }
        ws.check_unique()?;
        Ok(ws)
    }

    /// Section and problem slugs are scope names, so no two may be equal.
    fn check_unique(&self) -> Result<()> {
        let mut seen: HashMap<&str, &Path> = HashMap::new();
        let named = self.sections.iter().map(|s| (s.slug.as_str(), s.dir.as_path()));
        for (slug, dir) in named.chain(self.problems.iter().map(|p| (p.id.as_str(), p.dir.as_path()))) {
            if let Some(other) = seen.insert(slug, dir) {
                let rel = |d: &Path| d.strip_prefix(&self.roots.content).unwrap_or(d).display().to_string();
                bail!(
                    "{} and {} in {} are both named {slug:?}; section and problem names must all be different",
                    rel(other),
                    rel(dir),
                    show(&self.roots.content)
                );
            }
        }
        Ok(())
    }

    fn load_section(&mut self, dir: PathBuf) -> Result<()> {
        let meta: SectionMeta = read_toml(&dir.join("section.toml"))?;
        let section = self.sections.len();
        let problem_dirs = subdirs(&dir)?;
        self.sections.push(Section {
            slug: strip_order(&dir_name(&dir)).to_string(),
            dir,
            title: meta.title,
            description: meta.description.trim().to_string(),
            drill: meta.drill,
        });
        for dir in problem_dirs {
            // Still being written until it has a problem.toml.
            if dir.join("problem.toml").is_file() {
                self.load_problem(dir, section)?;
            }
        }
        Ok(())
    }

    fn load_problem(&mut self, dir: PathBuf, section: usize) -> Result<()> {
        let meta: Meta = read_toml(&dir.join("problem.toml"))?;
        self.problems.push(Problem {
            id: strip_order(&dir_name(&dir)).to_string(),
            dir,
            section,
            title: meta.title,
            difficulty: meta.difficulty,
            complexity: meta.complexity.trim().to_string(),
            hints: meta.hints.into_iter().map(|h| h.text.trim().to_string()).collect(),
        });
        Ok(())
    }

    /// Where tests and lints run.
    pub fn env(&self) -> Env {
        Env { build_dir: self.roots.build.clone(), timeout: runner::TEST_TIMEOUT, cancel: runner::Cancel::default() }
    }

    pub fn section_of(&self, p: &Problem) -> &Section {
        &self.sections[p.section]
    }

    /// `Arrays & Hashing · 3/9`
    pub fn location(&self, idx: usize) -> String {
        let section = self.section_of(&self.problems[idx]);
        let group = self.scope_problems(&section.slug);
        let pos = group.iter().position(|&i| i == idx).map_or(0, |k| k + 1);
        format!("{} · {pos}/{}", section.title, group.len())
    }

    /// Your working files, runs and progress.
    pub fn data_dir(&self) -> PathBuf {
        self.roots.data.clone()
    }

    pub fn events_file(&self) -> PathBuf {
        self.roots.events_file()
    }

    /// `<data>/run.json`: the run in progress.
    pub fn run_state_file(&self) -> PathBuf {
        self.roots.run_state_file()
    }

    /// `<data>/runs/<run>`: a run's files.
    pub fn run_dir(&self, run: u32) -> PathBuf {
        self.roots.runs_dir().join(run.to_string())
    }

    /// The completionist working file: `<data>/<slug>.rs`.
    pub fn attempt(&self, p: &Problem) -> PathBuf {
        self.data_dir().join(format!("{}.rs", p.id))
    }

    /// The working file in a run: `<data>/runs/<run>/<slug>.rs`.
    pub fn run_file(&self, run: u32, p: &Problem) -> PathBuf {
        self.run_dir(run).join(format!("{}.rs", p.id))
    }

    /// Every section slug, then every problem slug, in content order.
    fn slugs(&self) -> impl Iterator<Item = &str> {
        let sections = self.sections.iter().map(|s| s.slug.as_str());
        sections.chain(self.problems.iter().map(|p| p.id.as_str()))
    }

    /// The canonical name of a scope the player typed: `all` (also for an
    /// empty query), else a section or problem slug that equals it, else the
    /// one section or problem slug containing it (`pointers`).
    /// Ambiguous and unknown queries are errors that say what's close.
    pub fn resolve_scope(&self, query: &str) -> Result<String> {
        let q = query.trim().to_lowercase().replace(['-', ' '], "_");
        if q.is_empty() || q == ALL {
            return Ok(ALL.to_string());
        }
        if self.slugs().any(|s| s == q) {
            return Ok(q);
        }
        let matches: Vec<&str> = self.slugs().filter(|s| s.contains(&q)).collect();
        match matches.as_slice() {
            [one] => Ok((*one).to_string()),
            [] => {
                let mut close: Vec<(usize, &str)> =
                    self.slugs().map(|s| (edit_distance(&q, s), s)).filter(|&(d, _)| d <= q.len() / 3 + 1).collect();
                close.sort();
                let close: Vec<&str> = close.into_iter().take(3).map(|(_, s)| s).collect();
                if close.is_empty() {
                    return Err(user(format!("no section or problem matches {query:?}")));
                }
                Err(user(format!("no section or problem matches {query:?}; did you mean {}?", close.join(", "))))
            }
            many => {
                const SHOWN: usize = 8;
                let more = match many.len().saturating_sub(SHOWN) {
                    0 => String::new(),
                    n => format!(" and {n} more"),
                };
                Err(user(format!(
                    "{query:?} matches {} sections and problems: {}{more}; be more specific",
                    many.len(),
                    many[..many.len().min(SHOWN)].join(", ")
                )))
            }
        }
    }

    /// Whether problem `p` is in the canonical scope `scope`.
    pub fn in_scope(&self, p: &Problem, scope: &str) -> bool {
        scope == ALL || scope == p.id || scope == self.section_of(p).slug
    }

    /// Whether canonical scope `outer` includes scope `inner`: `all` ⊇
    /// section ⊇ problem, and every scope includes itself. Scopes no longer in
    /// content include only themselves.
    pub fn scope_contains(&self, outer: &str, inner: &str) -> bool {
        outer == ALL
            || outer == inner
            || self.problems.iter().any(|p| p.id == inner && self.section_of(p).slug == outer)
    }

    /// Indices of a canonical scope's current problems, in content order.
    pub fn scope_problems(&self, scope: &str) -> Vec<usize> {
        (0..self.problems.len()).filter(|&i| self.in_scope(&self.problems[i], scope)).collect()
    }

    /// Indices of every problem in a scope the player typed, in content order.
    pub fn select_many(&self, query: &str) -> Result<Vec<usize>> {
        Ok(self.scope_problems(&self.resolve_scope(query)?))
    }

    /// Every scope that can have a PB, in content order: `all`, then each
    /// section.
    pub fn scopes(&self) -> Vec<String> {
        std::iter::once(ALL.to_string()).chain(self.sections.iter().map(|s| s.slug.clone())).collect()
    }

    /// Human name of a canonical scope: `Everything`, a section's title, or a
    /// problem's title.
    pub fn scope_title(&self, scope: &str) -> String {
        if scope == ALL {
            return "Everything".into();
        }
        let section = self.sections.iter().find(|s| s.slug == scope).map(|s| &s.title);
        let problem = || self.problems.iter().find(|p| p.id == scope).map(|p| &p.title);
        section.or_else(problem).map_or_else(|| scope.to_string(), Clone::clone)
    }

    /// Whether the file at `path` is exactly a fresh template, but for line
    /// endings: an editor on Windows may save it with CRLF.
    pub fn is_pristine(&self, p: &Problem, path: &Path) -> bool {
        match (fs::read_to_string(path), p.template_code()) {
            (Ok(current), Ok(fresh)) => current.replace("\r\n", "\n") == fresh.replace("\r\n", "\n"),
            _ => false,
        }
    }

    /// (Over)write `path` with a fresh template. Nothing is archived. A new
    /// file is added to `rust-project.json`.
    pub fn write_fresh(&self, p: &Problem, path: &Path) -> io::Result<()> {
        let text = p.template_code()?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let created = !path.exists();
        fs::write(path, text)?;
        if created {
            self.write_rust_project();
        }
        Ok(())
    }

    /// Create `path` from the template if it doesn't exist yet.
    pub fn ensure_fresh(&self, p: &Problem, path: &Path) -> io::Result<()> {
        if path.exists() { Ok(()) } else { self.write_fresh(p, path) }
    }

    /// Runs with files on disk: `<data>/runs/<run>`, by run id.
    pub fn run_dirs(&self) -> Vec<u32> {
        let mut runs: Vec<u32> = fs::read_dir(self.roots.runs_dir())
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|e| e.file_name().to_str()?.parse().ok())
            .collect();
        runs.sort_unstable();
        runs
    }

    /// `<data>/rust-project.json`: an editor opened on the data directory
    /// finds it, and rust-analyzer then understands the standalone working files.
    pub fn rust_project_path(&self) -> PathBuf {
        self.data_dir().join(dirs::RUST_PROJECT)
    }

    /// Every working file on disk, completionist and run, relative to the data directory.
    fn working_files(&self) -> Vec<PathBuf> {
        let data = self.data_dir();
        let runs = self.run_dirs();
        let mut files = Vec::new();
        for p in &self.problems {
            let all = std::iter::once(self.attempt(p)).chain(runs.iter().map(|&r| self.run_file(r, p)));
            files.extend(all.filter(|f| f.is_file()).filter_map(|f| Some(f.strip_prefix(&data).ok()?.to_path_buf())));
        }
        files
    }

    /// (Re)write `rust-project.json` listing every working file. Failing is
    /// only logged: it's a convenience for the editor.
    pub fn write_rust_project(&self) {
        let path = self.rust_project_path();
        let files = self.working_files();
        if let Err(e) = fs::write(&path, runner::rust_project_json(&files)) {
            tracing::warn!("writing {}: {e}", path.display());
        }
    }

    /// At startup: write `rust-project.json` if there are working files but
    /// no such file.
    pub fn ensure_rust_project(&self) {
        if !self.rust_project_path().exists() && !self.working_files().is_empty() {
            self.write_rust_project();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// A throwaway workspace under the system temp dir.
    pub struct Scratch(pub PathBuf);

    impl Scratch {
        pub fn new(name: &str) -> Scratch {
            let dir = std::env::temp_dir().join(format!("gauntlet-ws-{name}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Scratch(dir)
        }

        pub fn write(&self, rel: &str, text: &str) {
            let path = self.0.join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        pub fn problem(&self, rel: &str, title: &str) {
            self.write(&format!("{rel}/problem.toml"), &format!("title = {title:?}\ndifficulty = \"easy\"\n"));
            self.write(
                &format!("{rel}/template.rs"),
                &format!("// {title}\npub fn f() {{}}\n\n#[cfg(test)]\n#[path = \"tests.rs\"]\nmod tests;\n"),
            );
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Three sections: `drills` = [option] (a drill section), `arrays` =
    /// [dup, two_sum] and `stack` = [min_stack].
    pub fn sample(name: &str) -> Scratch {
        let s = Scratch::new(name);
        s.write("content/02_arrays/section.toml", "title = \"Arrays\"\n");
        s.problem("content/02_arrays/02_two_sum", "Two Sum");
        s.problem("content/02_arrays/01_dup", "Dup");
        s.write("content/03_stack/section.toml", "title = \"Stack\"\n");
        s.problem("content/03_stack/01_min_stack", "Min Stack");
        s.write("content/01_drills/section.toml", "title = \"Drills\"\ndrill = true\n");
        s.problem("content/01_drills/01_option", "Option");
        s
    }

    #[test]
    fn strips_numeric_prefixes_only() {
        assert_eq!(strip_order("01_arrays_hashing"), "arrays_hashing");
        assert_eq!(strip_order("150_reverse_integer"), "reverse_integer");
        assert_eq!(strip_order("two_pointers"), "two_pointers");
        assert_eq!(strip_order("3sum"), "3sum");
    }

    #[test]
    fn relative_paths() {
        let rel = relative_path(Path::new("/w/docs/a"), Path::new("/w/content/03_a/x/README.md"));
        assert_eq!(rel, Path::new("../../content/03_a/x/README.md"));
        assert_eq!(relative_path(Path::new("/w"), Path::new("/w/content/x")), Path::new("content/x"));
    }

    #[test]
    fn edit_distances() {
        assert_eq!(edit_distance("stak", "stack"), 1);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn discovers_sections_and_problems_in_order() {
        let s = sample("order");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let ids: Vec<&str> = ws.problems.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, ["option", "dup", "two_sum", "min_stack"]);
        let sections: Vec<(&str, bool)> = ws.sections.iter().map(|s| (s.slug.as_str(), s.drill)).collect();
        assert_eq!(sections, [("drills", true), ("arrays", false), ("stack", false)]);
        assert_eq!(ws.section_of(&ws.problems[3]).title, "Stack");
        assert_eq!(ws.location(2), "Arrays · 2/2");
        assert_eq!(ws.location(0), "Drills · 1/1");
        assert_eq!(ws.select_many("arrays").unwrap(), [1, 2]);
        assert_eq!(ws.select_many("two-sum").unwrap(), [2]);
        assert_eq!(ws.select_many("all").unwrap().len(), 4);
        assert!(ws.select_many("nothing_like_this").is_err());
    }

    #[test]
    fn half_written_and_broken_problems() {
        let s = sample("partial");
        // Still being written: no problem.toml yet.
        s.write("content/01_drills/02_wip/README.md", "## Description\n");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        assert_eq!(ws.problems.len(), 4);
        // Not a section: no section.toml.
        s.write("content/notes/README.md", "scratch\n");
        assert_eq!(Workspace::discover_in(s.0.clone()).unwrap().problems.len(), 4);

        s.write("content/01_drills/03_bad/problem.toml", "title = ");
        let err = format!("{:?}", Workspace::discover_in(s.0.clone()).err().unwrap());
        // Paths are shown with the platform's separator.
        let sep = std::path::MAIN_SEPARATOR;
        assert!(err.contains(&format!("03_bad{sep}problem.toml")), "{err}");
    }

    #[test]
    fn section_and_problem_slugs_share_one_namespace() {
        let clash = |rel: &str, section: bool| {
            let s = sample(&format!("clash-{}", rel.replace('/', "-")));
            if section {
                s.write(&format!("{rel}/section.toml"), "title = \"Again\"\n");
            } else {
                s.problem(rel, "Again");
            }
            format!("{:?}", Workspace::discover_in(s.0.clone()).err().expect("a clash"))
        };
        let sep = std::path::MAIN_SEPARATOR;
        // Problem and problem, in different sections.
        let err = clash("content/03_stack/02_dup", false);
        assert!(err.contains(&format!("02_arrays{sep}01_dup and 03_stack{sep}02_dup")), "{err}");
        // Section and problem.
        let err = clash("content/02_arrays/03_stack", false);
        assert!(err.contains(&format!("03_stack and 02_arrays{sep}03_stack")), "{err}");
        // Section and section.
        let err = clash("content/04_arrays", true);
        assert!(err.contains("02_arrays and 04_arrays"), "{err}");
    }

    #[test]
    fn working_files_are_the_template_without_its_tests() {
        let s = sample("attempt");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let p = &ws.problems[2];
        let attempt = ws.attempt(p);
        ws.ensure_fresh(p, &attempt).unwrap();
        assert!(attempt.ends_with("data/two_sum.rs"));
        assert_eq!(fs::read_to_string(&attempt).unwrap(), "// Two Sum\npub fn f() {}\n");
        assert!(ws.is_pristine(p, &attempt));
        fs::write(&attempt, "// Two Sum\r\npub fn f() {}\r\n").unwrap();
        assert!(ws.is_pristine(p, &attempt), "saved with CRLF, still untouched");
        fs::write(&attempt, "// mine").unwrap();
        assert!(!ws.is_pristine(p, &attempt));
        ws.ensure_fresh(p, &attempt).unwrap();
        assert_eq!(fs::read_to_string(&attempt).unwrap(), "// mine", "ensure keeps work");
        ws.write_fresh(p, &attempt).unwrap();
        assert!(ws.is_pristine(p, &attempt));

        let run_file = ws.run_file(7, p);
        ws.write_fresh(p, &run_file).unwrap();
        assert!(run_file.ends_with("data/runs/7/two_sum.rs"));
        assert!(ws.is_pristine(p, &run_file));
        assert_eq!(ws.run_dirs(), [7]);
    }

    #[test]
    fn rust_project_lists_the_working_files() {
        let s = sample("rust-project");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let path = ws.rust_project_path();
        assert!(path.ends_with("data/rust-project.json"));
        ws.ensure_rust_project();
        assert!(!path.exists(), "no working files yet");
        let crates = || -> Vec<String> {
            let json: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            json["crates"].as_array().unwrap().iter().map(|c| c["root_module"].as_str().unwrap().to_string()).collect()
        };
        ws.ensure_fresh(&ws.problems[2], &ws.attempt(&ws.problems[2])).unwrap();
        assert_eq!(crates(), ["two_sum.rs"]);
        ws.ensure_fresh(&ws.problems[0], &ws.run_file(3, &ws.problems[0])).unwrap();
        assert_eq!(crates(), ["runs/3/option.rs", "two_sum.rs"]);
        // Lost: written again at startup.
        fs::remove_file(&path).unwrap();
        ws.ensure_rust_project();
        assert_eq!(crates().len(), 2);
    }

    #[test]
    fn scopes_resolve_in_order() {
        let s = sample("scopes");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let ok = |q: &str| ws.resolve_scope(q).unwrap();
        let err = |q: &str| ws.resolve_scope(q).unwrap_err().to_string();
        assert_eq!(ok(""), ALL);
        assert_eq!(ok("ALL"), ALL);
        // Exact names, even when another name contains them.
        assert_eq!(ok("stack"), "stack", "a section, though min_stack contains it");
        assert_eq!(ok("Arrays"), "arrays");
        assert_eq!(ok("dup"), "dup");
        assert_eq!(ok("Min Stack"), "min_stack");
        // Else the one name containing the query.
        assert_eq!(ok("two-"), "two_sum");
        assert_eq!(ok("rill"), "drills");
        assert_eq!(ok("opt"), "option");
        // Several: an error listing them, never the first.
        let e = err("s");
        assert!(e.contains("matches 5 sections and problems: drills, arrays, stack, two_sum, min_stack;"), "{e}");
        assert!(!e.contains("more;"), "{e}");
        // None: an error with near misses.
        let e = err("stak");
        assert!(e.starts_with("no section or problem matches \"stak\"; did you mean stack"), "{e}");
        assert_eq!(err("zzzzzzzz"), "no section or problem matches \"zzzzzzzz\"");
    }

    #[test]
    fn ambiguous_scopes_list_at_most_eight() {
        let s = sample("many");
        for n in 0..10 {
            s.problem(&format!("content/03_stack/1{n}_queue_{n}"), "Queue");
        }
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        let e = ws.resolve_scope("queue").unwrap_err().to_string();
        assert!(e.contains("matches 10 sections and problems:") && e.contains("queue_7 and 2 more"), "{e}");
    }

    #[test]
    fn scopes_nest() {
        let s = sample("nest");
        let ws = Workspace::discover_in(s.0.clone()).unwrap();
        assert_eq!(ws.scope_problems(ALL).len(), 4);
        assert_eq!(ws.scope_problems("stack"), [3]);
        assert_eq!(ws.scope_problems("two_sum"), [2]);
        assert_eq!(ws.scopes(), [ALL, "drills", "arrays", "stack"]);
        assert_eq!(ws.scope_title("arrays"), "Arrays");
        assert_eq!(ws.scope_title("two_sum"), "Two Sum");
        assert_eq!(ws.scope_title(ALL), "Everything");
        assert!(ws.scope_contains(ALL, "arrays"));
        assert!(ws.scope_contains(ALL, "dup"));
        assert!(ws.scope_contains("arrays", "dup"));
        assert!(ws.scope_contains("arrays", "arrays"));
        assert!(ws.scope_contains("gone", "gone"));
        assert!(!ws.scope_contains("arrays", "min_stack"));
        assert!(!ws.scope_contains("arrays", ALL));
        assert!(!ws.scope_contains("dup", "arrays"));
        assert!(!ws.scope_contains("stack", "arrays"));
    }
}
