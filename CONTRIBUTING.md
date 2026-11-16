# Contributing to Gauntlet

Thanks for your interest! Bug reports, problem fixes, ideas and pull requests
are all welcome. For anything larger than a small fix, please open an issue
first so we can agree on the approach before you spend time on it.

By taking part you agree to the [Code of Conduct](CODE_OF_CONDUCT.md).

## What are you changing?

- **A problem** (a wrong test, a confusing statement, a better hint, a new
  problem to replace an old one): everything lives under `content/`. Read
  [Building](#building), then [Writing problems](#writing-problems-for-the-gauntlet)
  below, which is the format and the bar every problem meets. The pull
  request needs `cargo run -- check <problem>` to pass.
- **The tool** (the TUI, the runner, stats, commands): the code is in `src/`.
  Read [Building](#building) and [Before opening a pull request](#before-opening-a-pull-request).

Not sure whether something is a bug? Ask in
[Discussions](https://github.com/BenjaminLucier/gauntlet/discussions), or
open an issue: the templates ask for what is needed.

## Building

You need Rust 1.91 or later with clippy ([rustup](https://rustup.rs),
`rustup component add clippy`), and the linker it needs (see the README's
*Requirements*; on Windows, the Visual Studio Build Tools). Gauntlet uses
the same toolchain at run time to compile attempts, and checks for it when
it starts.

```sh
cargo run                     # completionist mode
cargo run -- three_sum        # straight to a problem
cargo run --release -- check  # every problem; takes a few minutes
```

`cargo run` keeps its progress and working files in
`target/dev/data/gauntlet` and
reads the problems straight from `content/`
([.cargo/config.toml](.cargo/config.toml) sets `GAUNTLET_CONTENT_DIR` and
points the XDG data, cache and state directories into `target/dev`), so trying
things out never touches an installed Gauntlet (on Windows too: gauntlet
reads the XDG variables there when they're set). Compiled tests go to
`target/dev/cache/gauntlet/build` and the log to
`target/dev/state/gauntlet/gauntlet.log` (`GAUNTLET_LOG=debug` for more).
`cargo clean` wipes all of it.
Running `target/debug/gauntlet` directly skips those variables: it uses
your real data and the problems embedded when it was built.

## Before opening a pull request

CI runs these, and all must pass:

```sh
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo run --release --locked -- check   # every problem: stable and Rust 1.91 on Linux, stable on Windows
cargo deny check                        # if you changed dependencies; needs cargo-deny
```

For a change to one problem, `cargo run --release -- check <problem>` is
enough locally.

## Commit messages

Commits follow [Conventional Commits](https://www.conventionalcommits.org/),
since the changelog is generated from them:

```text
feat: show the split against your best when a section ends
fix(three_sum): test that duplicate triplets are reported once
docs(lru_cache): say which operations must be O(1)
```

Use `feat` for new behaviour, `fix` for bugs (a wrong test or statement is a
bug), and `docs`, `refactor`, `test`, `perf`, `ci` or `chore` for the rest.
For a change to a problem, use its id as the scope. Mark breaking changes
with `!`, as in `feat!: ...`; renaming a problem's id is one, since it loses
players' history. Keep each commit to one change and describe why, not just
what.

## License

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in Gauntlet by you, as defined in the Apache-2.0
license, shall be dual licensed under MIT OR Apache-2.0, without any
additional terms or conditions.

## Releasing

For the maintainer. Versions follow [Semantic Versioning](https://semver.org).

1. Add the new section to the changelog, then edit it to read well:

   ```sh
   git cliff --unreleased --tag vX.Y.Z --prepend CHANGELOG.md
   ```

   ([git-cliff](https://git-cliff.org): `brew install git-cliff`.) For the
   first release, rename the hand-written `[Unreleased]` section instead.
2. Set `version` in `Cargo.toml` and run `cargo check` to update
   `Cargo.lock`.
3. Commit as `chore: release vX.Y.Z` and merge it to `main`.
4. Tag that commit and push the tag:

   ```sh
   git tag -a vX.Y.Z -m vX.Y.Z
   git push origin vX.Y.Z
   ```

The [release workflow](.github/workflows/release.yml) takes it from there.
It checks the tag against `Cargo.toml` and the changelog, runs CI including
every problem, then builds the macOS, Linux and Windows binaries into a
draft GitHub Release (with `install.ps1`, the Windows install script),
publishes the crate to crates.io, publishes the release, updates the
[Homebrew tap](https://github.com/BenjaminLucier/homebrew-tap) (macOS and
Linux), and installs the result through Homebrew and the Windows script to
check it, running `gauntlet check two_pointers` with each. A tag with a
pre-release suffix, such as `v1.2.0-rc.1`, skips Homebrew, and so its
installs.

To audit problems in bulk with agents, see
[docs/maintainers/problem-audit.md](docs/maintainers/problem-audit.md).

# Writing problems for the gauntlet

> **The gauntlet is 128 problems, and that number is fixed.** Adding a
> problem means deciding which one it replaces; say which in your change.
> **One problem per idea:** of two problems that teach the same thing, the
> harder one stays.

## Layout: sections and problems

```text
content/
  NN_section/
    section.toml                  title, drill (optional), description (optional)
    NN_slug/                      one problem
```

There is one sequence of 128 problems in 22 sections:

- **Two Rust Toolkit sections** (`01_toolkit_values`, `02_toolkit_collections`,
  marked `drill = true`): the standard-library APIs that algorithm code leans
  on, one topic per problem, as many small functions. See *Drill sections*.
- **Interview problems**, one section per technique and one problem per idea, from
  `03_arrays_hashing` to `21_bit_manipulation`.
- **From-scratch structures and algorithms** (a dynamic array, a hash map, a
  heap, merge sort, Dijkstra): the textbook API of a named structure, with the
  std equivalent off limits (see *From-scratch structures*). Each sits in the
  section that uses it.
- **`22_advanced_structures`**: the from-scratch structures no earlier
  problem needs (segment and Fenwick trees, KMP, Rabin–Karp).

The gauntlet assumes the player has done
[rustlings](https://github.com/rust-lang/rustlings): problems teach algorithms
and the library, not the language. Don't add a problem that duplicates another:
search `content/` first.

### Ids

A problem's **id** is its bare slug (`three_sum`); a section's id is its bare
slug (`two_pointers`). The `NN_` prefixes set the order and are not part of
the id. Sections and problems share one namespace: a duplicate slug anywhere
is a discovery error. Progress is keyed by the id, so reordering and moving
between sections are free, but renaming a slug loses players' history.

### Where a new problem goes

- An interview problem goes in the section for its technique, ordered roughly
  by difficulty and by what it builds on.
- A from-scratch structure or algorithm goes immediately before the first
  problem that needs it (the binary heap opens *Heap / Priority Queue*,
  union-find comes just before the first problem that uses it). If no problem
  needs it, it goes in `22_advanced_structures`.
- A drill goes in a toolkit section, near the drills it builds on.

### Numbering

Within a directory, prefixes are two digits, start at `01` and have no gaps.
To insert or move a problem, rename the directories after it to close or open
the gap (`git mv` keeps the history readable). Ids don't change, so players
lose nothing. Then run `cargo run -- readme` to update the index.

### The count

128, fixed (see the top of this part). A new problem replaces an old one,
and of two that teach the same idea, the harder one stays.

### Files in every problem

Every problem has a `problem.toml`, a `README.md`, a `template.rs`, a
`solution.rs` and a `tests.rs`.

- The **template** becomes the player's working file, `<data>/<slug>.rs`
  (and `<data>/runs/<n>/<slug>.rs` in a run), with its closing tests block
  removed. It has the full public API with doc comments, and every body is
  `todo!()`, so the untouched template reports *Not started*. It starts with
  a two-line header; line 1 is exactly
  `// <Title> — press i in the gauntlet for the statement, t for the tests.`
  with the title from `problem.toml`:

  ```text
  // 3Sum — press i in the gauntlet for the statement, t for the tests.
  // Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
  ```

- The **solution** is the reference: optimal, idiomatic and clippy-clean.
  Write the code you'd want a learner to study.
- The **tests** are shared by the template, the solution and the player's
  attempt. The working file holds only the player's code; the tests are
  attached when it's compiled.

Read every file of a reference problem before writing one: the from-scratch
reference is `content/03_arrays_hashing/01_dynamic_array/`, the interview
problem one `content/03_arrays_hashing/03_two_sum/`, the drill one
`content/01_toolkit_values/01_option_combinators/`.

## problem.toml

Everything the gauntlet needs to know about the problem, other than the
statement. Unknown keys are an error.

```toml
title = "3Sum"
difficulty = "medium"            # easy | medium | hard

complexity = '''
- Time: `O(n^2)`
- Space: `O(n)` for the sorted copy, plus the output
'''

[[hints]]
text = '''
Sort first. Fix the smallest element `nums[i]`; the problem becomes "two sum
on a sorted slice" for the rest, with target `-nums[i]`.
'''

[[hints]]
text = '''
To avoid duplicates, skip `i` when `nums[i] == nums[i - 1]`...
'''
```

There's no category: the section the problem sits in is its category.

- **`complexity`**: required, except in drill sections, where it's optional.
- `complexity` and each hint are Markdown.

`section.toml`:

```toml
title = "Two Pointers"
drill = true          # optional: only the two toolkit sections
description = """
Optional: one or two sentences, shown in the README index.
"""
```

### Hints

Write 2–3, gentlest first. The player reveals hints one at a time in the Info
tab (in completionist mode; runs hide them), so the first should
nudge, not solve. In drills, hints point at the docs or the APIs to use.

## README.md

The statement, and nothing else: no title (it comes from `problem.toml`), no
complexity, no hints. Sections, in this order:

```markdown
## Description
## Restrictions       optional: honor-system rules, mirrored in a RESTRICTIONS
                      comment in the template
## Signature          a rust code block with the public API, then a sentence
                      or two on why the types are what they are
## Examples           a rust code block of assert!/assert_eq! calls, with comments
## Constraints
## Rust notes         2–5 bullets on idioms, APIs and traps.
                      Don't give the algorithm away.
```

Drills replace *Signature* and *Constraints* with *The drills*: a table of
groups → functions → APIs, each API linking to doc.rust-lang.org.

Write everything in your own words, and don't mention outside sources such as
problem sites or courses — the gauntlet stands on its own.

## The testing bar

Aim for **12–25 tests per problem** (a few heavy ones go up to 30):

1. **Every README example**.
2. **Edge cases**: empty input, single element, extreme values from the
   constraints, negatives, zeros, duplicates, all-equal, sorted /
   reverse-sorted, no answer, answer at the start or end.
3. **Named traps**: tests named after the bug they catch
   (`off_by_one_at_window_end`, `greedy_choice_is_wrong`).
4. **Randomized cross-checks** against a brute-force oracle written in the test
   file, with a **fixed seed**. Put the input in the assertion message when
   it's small.
5. **Large inputs** (outside drills). Solved means correct: a slow solution
   that gets every answer right passes, and the gauntlet shows its runtime and
   memory next to the reference's. `complexity` states the target; nothing
   enforces it by time. Add **one** large input, sized so the reference
   finishes in well under a second and the obvious slow approach (a pair
   scan, a rescan per step) takes a few seconds but passes. Give it the shape
   that defeats early exits (answer at the far end, or none). Measure it:
   write the slow version and time the whole suite with `--test-threads=1`.
   Where the API allows it and the complexity is the point of the problem,
   **count operations** (comparisons, probes) and assert a budget instead:
   that fails the same way on every machine. Never assert on wall-clock time.
   The 30 s timeout only guards against hangs, which is how exponential
   blow-ups (unmemoized recursion) fail.
6. **Rust-specific behaviour** of the API: other generic instantiations,
   `Option`/`Result` variants, non-ASCII strings, empty structures.

Every test must reach a `todo!()` before asserting anything, so the untouched
template fails *every* test as "not started". If any order is accepted,
normalize before comparing, or check the property directly; if several answers
are valid, validate rather than compare. Floats compare with a tolerance. Test
names describe behaviour. Design problems get operation sequences, plus a long
random sequence checked against a simple model.

## Maintainer commands

Run them from the repository root, through cargo, so they read `content/`
rather than the copy embedded in a binary:

```sh
cargo run -- check three_sum       # one problem
cargo run -- check two_pointers    # a section
cargo run --release -- check       # everything; takes a few minutes
cargo run -- readme                # regenerate the problem index in README.md
```

`readme` refuses to run without `GAUNTLET_CONTENT_DIR`, which `cargo run`
sets. A scope resolves like everywhere else: an exact name, then a unique
part of one.

To try a problem as a player would, `cargo run -- <slug>` opens it with the
scratch data in `target/dev/data`. A working file that already exists there
keeps its old contents after you edit the template: press `x` for a fresh
copy.

**Before you finish, `cargo run -- check <scope>` must print `ok`.** It
verifies that:

- the solution passes, in under 3 s;
- the untouched template fails every test with `todo!()`;
- the solution and the template are clippy-clean with `-D warnings`;
- `problem.toml` parses, has hints, and has a complexity outside drill
  sections;
- line 1 of `template.rs` is exactly
  `// <Title> — press i in the gauntlet for the statement, t for the tests.`;
- nothing mentions outside sources.

When something fails it prints the full compiler or test output. After adding,
moving or renaming a problem, run `cargo run -- readme` to regenerate the index
between the `<!-- problems:start -->` and `<!-- problems:end -->` markers in
`README.md`; don't edit it by hand.

## Rust

Each `.rs` file is compiled on its own:
`rustc --edition 2024 --test -C overflow-checks=on <file>` — std only, no
Cargo, no external crates. Integer overflow panics, as under `cargo test`.

### The goal: idiomatic Rust, not a port

These problems started life as classic interview questions, whose usual
signatures come from languages without ownership, `Option`, or generics. Here
the API itself should be something a Rust developer would design: it is part of
what the player learns. When the classic signature and good Rust disagree, good
Rust wins. Keep the *problem* the same (same underlying task, same difficulty);
change the *interface*.

#### Signature guidelines

- **Borrow what you only read.** `&[T]` rather than `Vec<T>`, `&str` rather
  than `String`, `&[&str]` for a list of words. Take ownership (`Vec<T>`,
  `String`) only when the function genuinely consumes or reuses the buffer
  (e.g. it sorts in place and the caller doesn't need the original — then
  `mut v: Vec<T>` lets the caller decide whether to clone). Mutate in place with
  `&mut [T]` / `&mut [Vec<T>]` — not `&mut Vec<T>` unless you push or pop.
- **Types that mean something.** Indices, lengths and counts are `usize`.
  Quantities that can't be negative are unsigned. Values keep the domain type
  given by the constraints (`i32`, `i64`); widen when an intermediate result can
  overflow, and say so in the README.
- **No sentinels.** "Not found" or "impossible" is `Option<T>`, never `-1`.
  Use `Result<T, E>` with a small error enum when there are distinct ways to
  fail (parsing, evaluation, decoding) — and derive `Debug, PartialEq, Eq` on it
  so tests can compare.
- **Tuples or small structs instead of `Vec<i32>` pairs.** An interval is
  `(i32, i32)` or a `struct Interval { start, end }`; a point is `(i32, i32)`;
  an edge is `(usize, usize)`, a weighted edge `(usize, usize, u32)`.
- **Generic where the algorithm only needs a trait** — `T: Hash + Eq`,
  `T: Ord`, `T: Copy` — and the generic version reads naturally. Don't add type
  parameters for show.
- **Character grids** that are only read take `&[&str]` (ASCII rows, readable
  in tests: `&["11110", "11010"]`). Grids that are modified take
  `&mut [Vec<u8>]` (byte chars like `b'X'`) or a `Vec<Vec<Cell>>` of a small
  domain enum, whichever makes the solution clearer.
- **Strings:** if the constraints say ASCII, say so in the README and let the
  solution work on `as_bytes()`. If not, handle `char`s correctly and test
  non-ASCII input.
- **Design problems** (stacks, caches, tries, …): a struct with private fields,
  `new(...)`, `&self` for queries, `&mut self` for updates. Return `Option<T>` /
  `Option<&T>` where the classic version would assume non-empty. If `new` takes
  no arguments, also implement `Default` (clippy will insist). Make them generic
  over the element type when that's natural (`MinStack<T: Ord>`).
- **Linked lists** use one shared definition, plus a type alias:

  ```rust
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct ListNode {
      pub val: i32,
      pub next: Option<Box<ListNode>>,
  }

  pub type List = Option<Box<ListNode>>;
  ```

  Functions that restructure a list take and return `List` by value.
- **Binary trees** are owned boxes, **not** `Rc<RefCell<…>>`:

  ```rust
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct TreeNode {
      pub val: i32,
      pub left: Tree,
      pub right: Tree,
  }

  pub type Tree = Option<Box<TreeNode>>;
  ```

  Read-only functions take `Option<&TreeNode>` (callers pass `tree.as_deref()`).
  Functions that transform a tree take and return `Tree`, or take `&mut Tree`.
- **Graphs:** `n: usize` plus `&[(usize, usize)]` edges, or adjacency lists
  `&[Vec<usize>]`. Where a problem is *about* shared, cyclic ownership (deep
  copying a graph or a list with random pointers), keep `Rc<RefCell<…>>` /
  `Weak` — that is the Rust lesson in it — and explain the representation.
- Shared types are defined identically in `template.rs` and `solution.rs`
  (each file is standalone).

### template.rs

```rust
// 3Sum — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::HashMap; // imports a typical solution needs

pub fn three_sum(nums: &[i32]) -> Vec<[i32; 3]> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
```

- The file ends with exactly those three lines, `#[cfg(test)]`,
  `#[path = "tests.rs"]` and `mod tests;`, so the template compiles with its
  tests where it is. Gauntlet strips them when it creates a working file and
  attaches the tests itself when it compiles one. The solution ends the same
  way.
- Every body the player writes is `todo!()`. Supporting types (`ListNode`,
  error enums, …) are given in full; struct fields of design problems are left
  for the player (`// TODO: your fields here`).
- With the untouched template, **every test must panic with
  "not yet implemented"**, so the gauntlet can show "Not started". Every test
  must call a `todo!()` before asserting anything; `#[should_panic(expected =
  "…")]` tests are fine because the `todo!()` panic doesn't match.
- A `todo!()` body can't return `impl Trait` under edition 2024. Return a
  named type (your own `Iter<'a, T>`), a `Box<dyn …>`, or put the `todo!()`
  inside the returned closure.
- Don't put `todo!()` in a `Drop` impl: a panic while unwinding aborts the
  whole test run. Leave the body empty with a `// TODO` comment; the default
  (recursive) drop is exactly what a deep-structure test should catch.
- If a function takes `&mut Vec<T>` / `&mut String` because the solution needs
  `Vec`/`String` methods, clippy's `ptr_arg` can't see that through `todo!()`:
  add `clippy::ptr_arg` to the template's `#![allow(...)]` line, with a comment.
- An optional `// RESTRICTIONS: …` line after the second header line lists
  honor-system rules (no `for` loops, no `BinaryHeap`, …). They aren't
  enforced; repeat them in a `## Restrictions` README section.

### solution.rs

Same API as the template, implemented. Header:
`// 3Sum — reference solution.` followed by 1–3 lines on the approach and its
time/space complexity. No `#![allow(...)]` unless a specific lint is
unavoidable — then allow just that lint, on just that item, with a comment.

Write the code you'd want a learner to study: iterator adaptors where they're
clearer than index loops (and index loops where they aren't), the `entry` API,
`BinaryHeap` with `Reverse`, `BTreeMap::range`, `sort_unstable_by_key`,
`mem::take` / `Option::take` for list surgery, `checked_*` arithmetic,
`let … else`, slice patterns. No `unsafe`. No `unwrap()` on things that can
actually fail — `expect("why this can't happen")` for invariants.

### tests.rs — exhaustive

Starts with `// Tests for 3Sum — shared by template.rs, solution.rs and your attempt.`
then `use super::*;`. Must be clippy-clean too. Follow *The testing bar*
above, with these Rust specifics:

- Name traps after the bug (`overflow_in_product` as well as the generic
  ones).
- The brute-force oracle lives in tests.rs (prefix `brute_`) and uses the
  tiny xorshift `Rng` from Two Sum. Put the input in the assert
  message.
- Large inputs must be handled by the optimal solution in well under a
  second in a **debug build**, and by the obvious slow one in a few seconds
  (see *The testing bar*). Where the complexity is the point, count
  operations instead (see *From-scratch structures* below).
- Rust-specific behaviour worth a test: generic instantiations with other
  types, `Option`/`Result` variants, non-ASCII strings, items that aren't
  `Clone`, empty structures.
- Helpers (`list(&[i32]) -> List`, `to_vec`, `tree(&[Option<i32>]) -> Tree`
  from level order, `level_order(Option<&TreeNode>)`, …) live in tests.rs.
- Long `Box` chains drop recursively: keep lists and degenerate trees to a few
  thousand nodes, or drop them iteratively in a helper.
- Design problems: operation sequences, plus a long random sequence checked
  against a simple model (a `Vec`, a `BTreeMap`, …).
- For tests that need a deep recursion stack, run the body in
  `std::thread::Builder::new().stack_size(..)` and re-raise panics with
  `std::panic::resume_unwind` so "not yet implemented" still surfaces.

### Section-specific rules

#### From-scratch structures

- A design-style API: a struct with private fields (left to the player), the
  textbook operations, and the trait impls a Rust user expects where they're
  natural — `Default`, `Index`/`IndexMut`, `FromIterator`, `Extend`,
  `IntoIterator` (by reference, and by value where it makes sense), your own
  `Iter<'a, T>`, an iterative `Drop` for linked structures.
- Forbid the std structure being reimplemented with a `RESTRICTIONS` line (no
  `BinaryHeap` in a heap, no `sort` in a sort, no `HashMap` in a hash map).
- Safe Rust only. Storage is `Box<[Option<T>]>` / `Vec<Option<T>>` slots;
  linked structures use `Option<Box<…>>`, an index arena, or `Rc`/`Weak`.
- Tests include a long random sequence of operations checked against the std
  equivalent.
- The complexity is the point here, so **count** where the API allows it:
  wrap elements in a type whose `Ord`/`Eq` counts calls, and assert a budget
  (`<= 2 * n` comparisons for heapify, `n * ceil(log2 n)` for merge sort).
  That fails the same way on every machine, and fails fast. Where it doesn't
  (plain `usize` handles, say), a large input shows the wrong complexity as
  slow, as in *The testing bar*.
- Deep structures (100k-node lists, degenerate trees) test iterative
  traversals and drops. On some platforms a stack overflow hangs instead of
  aborting, so the player sees a timeout; the runner's timeout message says
  so.

#### Drill sections

Sections with `drill = true` in `section.toml` (the two Rust Toolkit sections)
hold drills: one standard-library topic per problem, worked as many small
functions.

- 15–25 small `pub fn`s (or trait impls on given types), grouped by comment
  banners. Each doc comment names the method or idea it's after, in
  parentheses: `/// Keep the value only if it's even. (\`filter\`)`.
- 1–3 tests per function, named after the function.
- The README replaces *Signature* / *Constraints* with *The drills*: a table of
  groups → functions → methods, each method linking to doc.rust-lang.org.
- `difficulty` is relative to the other drills; `complexity` is optional and
  usually left out.
