# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-11-17

### Fixed

- Linux release builds: the musl archives need a C compiler for Oniguruma,
  which syntect uses for highlighting. 0.1.1 was tagged but never published;
  0.1.2 is the first release on GitHub Releases and in the Homebrew tap, with
  0.1.1's changes.

## [0.1.1] - 2026-11-17

### Changed

- Dates and times are handled with jiff instead of chrono, which is no
  longer maintained (RUSTSEC-2026-0543). History written by 0.1.0 reads the
  same.

## [0.1.0] - 2026-11-17

The first release.

### Added

- 128 problems in 22 sections: two Rust Toolkit sections of standard-library
  drills, interview problems grouped by technique, and data structures and
  algorithms built from scratch. Each has a statement with Rust notes, 2–3
  hints, a template, a reference solution and 12 to 30 tests.
- Completionist mode: opens your next unsolved problem, tests it every time
  you save, and asks after each solve how trivial it felt. Working files hold
  only your code; the tests are attached when it is compiled.
- Review: `gauntlet review [scope]` brings back the problems you last rated
  1 or 2, hardest and least recently solved first, then those whose best
  solution is well off the reference solution's runtime or memory.
- Runtime and memory: every pass shows how long the tests took (run one at a
  time) and their peak memory, against the reference solution measured the
  same way on your machine and cached. Speed never fails a solution: the
  30-second limit only stops a hang. A solution well off the reference's
  pace (by a ratio and a margin, so startup and noise don't count) gets a
  nudge, is marked `▲` in the stats and is suggested for review. Each solve
  records its measurements; old logs without them still load.
- Timed runs over the whole gauntlet, a section or a problem, with personal
  bests per problem, section and the whole gauntlet, a sum of best, and
  split times. `gauntlet run --delete` abandons a run; `gauntlet run --clean`
  deletes the code of finished runs.
- Statistics: progress per section, ratings, suggested review, solve history
  per problem with each solve's runtime and memory, your best of each
  against the reference, a streak, and runs with their PBs.
- A clippy review of your attempt, one key away.
- The problems are embedded in the binary. Your progress and working files
  live in `$XDG_DATA_HOME/gauntlet` (`%APPDATA%\gauntlet` on Windows), with a
  `rust-project.json` for rust-analyzer; one gauntlet at a time can have it
  open.
- The toolchain is checked when it's needed: rustc (1.91 or later) when
  gauntlet starts, clippy when you ask for a review, and a compile that
  fails for want of a linker says what to install on your platform.
- `gauntlet reset` deletes gauntlet's own files in the data directory and
  nothing else.
- Maintainer commands: `gauntlet check` verifies every reference solution,
  template and hint, and `gauntlet readme` regenerates the problem index.
- macOS, Linux and Windows. Installs with Homebrew, a PowerShell script on
  Windows, `cargo install` or `cargo binstall`, or from the archives on
  GitHub Releases.

[Unreleased]: https://github.com/BenjaminLucier/gauntlet/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/BenjaminLucier/gauntlet/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/BenjaminLucier/gauntlet/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/BenjaminLucier/gauntlet/releases/tag/v0.1.0
