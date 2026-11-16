# Auditing a problem

You are auditing **one problem** of the gauntlet: the files in its directory,
each on its own and against each other. You were given its id
(`three_sum`). [CONTRIBUTING.md](../../CONTRIBUTING.md) says how a
problem should be written; read it first. This file says how to check that
it was, and how to find what `gauntlet check` can't see.

Your final message is a short report in the format at the end of this file.
It is the only thing the person who launched you will read, so everything
they need to know goes there.

## Ground rules

These matter more than any single checklist item below.

1. **The default is no change.** Most problems are mostly fine. Every edit
   you make fixes a defect you can name in one sentence in the report. "I
   would have phrased it differently" is not a defect. If you find yourself
   rewriting most of a file, stop: either the problem needs a human (say so
   in the report) or you are applying taste.
2. **Evidence over impression.** "Test X catches bug Y" is a claim you can
   check by writing the bug and running the tests, and section 3 tells you
   when you must. In the report, say for each claim whether you ran it or
   reasoned it. Never write "verified" for something you didn't run.
3. **A missing test is worse than a redundant one.** Adding a test that
   catches a real bug is cheap and safe. Deleting a test is the riskiest
   thing you can do here: do it only with the evidence section 3 asks for,
   and when in doubt, keep the test and mention it in the report.
4. **Stay in your own directory.** Other auditors are working on other
   problems in the same checkout at the same time. Section 2 has the rules.
5. **Report what happened, not what should have happened.** If `check`
   still fails at the end, or you ran out of ideas for a missing test, or you
   skipped a section, say so plainly.

A typical audit runs 4–8 mutants, changes zero to four tests, and makes a
handful of small text fixes. Some problems need nothing.

## 1. Setup

1. Read CONTRIBUTING.md, then every file of the **reference problem** for
   your kind of problem. They show what "right" looks like:
   - From-scratch structure or algorithm: `content/03_arrays_hashing/01_dynamic_array/`
   - Interview problem: `content/03_arrays_hashing/03_two_sum/`
   - Drill (a section with `drill = true`): `content/01_toolkit_values/01_option_combinators/`

   The references are good models of format and tone, not proof of
   perfection: don't copy a pattern from one that this file tells you to
   avoid.
2. Read every file of your problem, and skim one or two neighbours in the
   same section to calibrate difficulty and tone.
3. Run `GAUNTLET_CONTENT_DIR=content target/debug/gauntlet check <id>` from
   the repo root: the binary built for you, pointed at `content/`. (Without
   the variable it checks the copy embedded when the binary was built, not
   your edits.) If it fails, that's your first finding: fix it before judging
   anything else. If the binary doesn't exist, or `check` says a tool is
   missing, stop and report that; don't build or install anything. Every
   `gauntlet check` below means this command.

`gauntlet check` verifies the following, so don't re-verify them by hand:

- `problem.toml`, `README.md`, `template.rs`, `solution.rs` and `tests.rs`
  all exist
- the solution passes every test, and the whole run takes under 3 s
- no test passes on the untouched template, and it reports *not started*:
  the failures are `todo!()` panics
- clippy, with warnings as errors, on the solution and the template (each
  compiled with `tests.rs`, so the tests are linted too)
- `problem.toml` parses, has no unknown keys, has hints, and has a
  complexity outside drill sections
- line 1 of `template.rs` is exactly
  `// <Title> — press i in the gauntlet for the statement, t for the tests.`
- none of those files mentions an outside source

Re-run it after every change you make. Your audit ends with it printing `ok`.

If it reports "tests are slow" on a suite that was fine a minute ago, the
machine is busy with other auditors: run it again before believing it.

## 2. Working alongside other auditors

- **Never run `cargo`** (build, test, clippy, clean, `cargo run`, anything).
  `gauntlet check` calls `rustc` and `clippy-driver` directly, and names its
  build artifacts (in `~/.cache/gauntlet/build`) after the problem id, so
  checks of different problems don't collide. A `cargo` run would queue on a
  lock, rebuild the binary under everyone else (any change under `content/`
  triggers a rebuild), or wipe `target/`, where everyone's scratch files are.
- **Don't change a toolchain.** No `rustup` changes, no installs, no
  upgrades.
- **Use only `gauntlet check`.** Don't open the TUI, and don't run `run`,
  `review`, `reset` or `readme`. Don't touch the data directories
  (`~/.local/share/gauntlet`, `target/dev/data`): they hold the player's
  attempts.
- **Write only inside your problem's directory and your scratch directory.**
  Everything else is shared: `section.toml`, the root
  `README.md`, `CONTRIBUTING.md`, this file, `src/`, other problems. If one of them needs
  a change (a helper, a typo in a neighbour), put it under *Flags* in the
  report.
- **Keep `problem.toml` valid after every save.** Every `gauntlet check`
  parses every `problem.toml` in the repo, so one broken file stops every
  other auditor.
- **Someone else's breakage is not yours to fix.** If `check` fails while
  parsing another problem's files, another auditor is mid-edit: wait a few
  seconds and retry.
- **Your scratch directory** is `target/audit/<id>/`
  (for example `target/audit/three_sum/`).
  Mutants and experiments go there, never in the problem's directory.
- **Don't touch git**: no commit, stash, checkout, reset or restore. Other
  auditors' uncommitted work is in the same tree.

### What you may change

- **Fix in place** (inside your problem's directory): missing, redundant or
  mislabelled tests; wrong or useless hints; README errors, ambiguities and
  format deviations; typos; stale paths; a solution that breaks a rule in
  section 6; template comments and imports.
- **Flag, don't change**:
  - the directory name or slug (renaming loses every player's history)
  - the public API: names, parameter and return types, bounds, exports
    (players' existing attempts stop compiling or stop passing)
  - `difficulty`
  - moving the problem to another section, or renumbering it
  - the constraints in the README, when changing them would change the
    problem

  For each flag, give the reason and what you'd change it to.

## 3. Tests: a basis of failure modes

This is the main part of the audit. Sections 3 and 4 apply as written to
every problem outside the drill sections. For drills, read them for the idea
and then follow section 10.

### The idea

A **failure mode** is a plausible way for a player's solution to be wrong: a
specific bug (off-by-one at the window's end, forgetting to skip duplicates,
overflow in a product), a wrong algorithm (greedy where DP is needed,
quadratic where `O(n log n)` is the target), or a language-level slip
(requiring `T: Clone`, `usize` underflow, a recursive `Drop` on a long list,
crashing on empty input).

The suite should be a *basis* for the space of failure modes:

- **Spanning: nothing missing.** Every plausible failure mode makes at least
  one test fail.
- **Independent: nothing redundant.** Every test has a **target**: a failure
  mode it catches that no other named test catches. A test without a target
  has no reason to exist.

Independence is about targets, not about failing alone. Tests may fail
together for incidental reasons, and that's fine:

- A test that builds up state (`push` × 1000, then checks `pop`) also fails
  when `push` is broken. Its target is `pop`; the `push` tests pin down
  `push`.
- A NaN, an overflow, or an error in shared setup can take out several tests
  at once. Each still has its own target.

What isn't fine is two tests with the same target: `large_input_1` and
`large_input_2` that both exist to rule out a quadratic solution, or
`single_element` and `one_item` probing the same boundary.

Three kinds of test are **exempt** from needing a target of their own,
because overlapping with everything is their job:

1. **README examples.** Each example in the README is asserted in a test
   (`example_1`, `example_2`, … or `readme_example`), so the statement can't
   drift from the tests.
2. **The randomized cross-check** against a brute-force oracle: the safety
   net for failure modes nobody thought of. One per function or design type.
   A second with a different input distribution is fine if that distribution
   has its own target (a tiny alphabet to force collisions, say).
3. **The long random operation sequence** against a simple model built from
   the standard library, in design and from-scratch problems.

The exemption has a limit: a failure mode that the README, a Rust note or
a hint warns about deserves a *named* test even if the oracle would catch it.
If we warn about it, we test it by name, so the player's failure output says
what went wrong.

### Procedure

**Step 1. List the failure modes** before looking closely at the tests, so
the existing suite doesn't anchor you. Ask of the statement, the solution and
the hints: how would someone get this wrong? Sources:

- *Spec boundaries*: empty input (if the types allow it), one element, two
  elements, all equal, sorted and reverse-sorted, no answer, answer at the
  first or last position, extreme values from the constraints, negatives and
  zero, duplicates.
- *The algorithm's own traps*: the wrong approaches the hints steer away
  from (a greedy that's wrong, a window that shrinks one step too late, a
  binary search that loops forever on two elements, a visited set marked too
  late in BFS).
- *Rust traps*: the list below, and the rules in CONTRIBUTING.
- *Complexity*: the naive approach the problem exists to rule out
  (section 4).

Rust traps: overflow (the tests run with overflow checks on), `usize`
underflow (`i - 1` at `i == 0`), non-ASCII input when the README doesn't
promise ASCII, unnecessary trait bounds (a solution that needs `Clone`
where the signature doesn't offer it won't compile, but one that clones
where it could borrow passes silently unless a test counts clones),
`Option`/`Result` variants, empty structures, recursive `Drop` or
recursion depth at the constraint limit.

Aim for the plausible ones, typically 8–15. Not every conceivable bug: a
solution that returns a constant is caught by anything.

**Step 2. Map modes to tests.** Read each test and decide what its target
is. Build the table that goes in your report:

| Failure mode | Caught by |
|---|---|
| doesn't skip duplicate triples | `duplicate_triples_reported_once` |
| cubic brute force | `large_input_is_not_cubic` |
| off-by-one on the first element | nothing (oracle only) |

Rows are failure modes, not tests: don't produce the table by walking the
test list and writing down what each test is called. Under "Caught by", name
the one or two tests whose target the mode is; if more fail incidentally,
"(+6 others)" is enough.

- A mode with no named test is a **missing test** if it meets the bar above
  (plausible, or warned about in the README or hints).
- A test that is nobody's sole named catcher, and isn't exempt, is
  **possibly redundant**.

**Step 3. Run mutants where it matters.** A mutant is a copy of the solution
with exactly one bug, kept in your scratch directory. *Running a mutant*
below has the commands.

A mutant is a bug a person would plausibly write: the wrong method, the
missing bound, the simpler algorithm. Don't construct an artificial one to
slip between the existing tests (`if n == 7 { wrong }`) and then add a test
for it.

You **must** run a mutant in these cases:

- **For every test you add.** The new test fails on its mutant and passes on
  the solution. A test nobody has seen fail proves nothing.
- **Before you delete or merge a test.** Write the mutant for that test's
  apparent target and show another *named* test also fails on it. If you
  can't construct such a mutant, or only the deleted test catches it, the
  test stays.
- **For every timing-based performance test** (section 4).
- **For rows you're unsure about.**

You needn't run mutants for rows that are obvious on reading (a test that
asserts the exact README example for a mode that changes that output).

**Step 4. Fix.**

- *Missing test*: add it, named after the bug it catches
  (`underflow_on_first_element`), with the smallest input that triggers it.
- *Redundant test*: if the evidence from step 3 holds, merge its assertions
  into the test that shares its target, or delete it. Otherwise leave it and
  list it under *Possibly redundant tests* in the report.
- *Mislabelled test*: a test whose name promises one target while its input
  hits another (see the early-exit trap in section 4). Fix the input so it
  does what the name says, or rename it to what it does.

**Step 5. Check for false failures.** A test must not fail a correct,
optimal, idiomatic solution that takes a different route from the reference.
Look for:

- exact output order where the README accepts any order (normalize, or check
  the property instead)
- one specific answer where several are valid (validate instead of compare)
- exact float equality (use a tolerance)
- operation-count budgets too tight for the alternative approaches the hints
  themselves suggest
- a specific error (a variant, an exception class, a message) when the input
  is wrong in two ways at once and the README doesn't say which wins, or a
  message the README never specifies
- inputs outside the README's constraints
- an input deeper than a correct recursive solution can go when the README
  doesn't ask for iteration (section 4, *Deep inputs*)

### Running a mutant

All commands run from the repo root. Set up once:

```sh
S=target/audit/<id>
P=content/<section dir>/<problem dir>
mkdir -p $S
```

A mutant that might not terminate needs a time limit on the command: none of
the commands below has the harness's 30 s timeout. Several mutants can be
generated and run by one script.

Compile the mutant next to a snapshot of the tests, with the harness's
flags:

```sh
cp $P/tests.rs $P/solution.rs $S/
cp $S/solution.rs $S/mutant_underflow.rs      # then edit it
rustc --edition 2024 --test -C overflow-checks=on -A warnings \
  -o $S/mutant_underflow $S/mutant_underflow.rs
$S/mutant_underflow 2>&1 | rg '^test |test result'
```

`$S/tests.rs` is a snapshot: copy it again whenever you edit the real one.

### Other things to check in the tests file

- **Names** say what the test catches (`window_shrinks_before_counting`),
  not a number (`test_3`) or a bucket (`edge_cases`, `misc`). Rename only
  names that are actually uninformative; a decent name stays. Names are
  `snake_case`.
- **Messages**: asserts on generated input include the input, so a failure
  is reproducible from the output. For large inputs, print the size and the
  seed, not the input.
- **Count**: 12–25 tests, up to 30 for a big design API. That's a symptom to
  look into, not a target to hit: never add or remove tests to reach a
  number.
- **Determinism**: randomness has a fixed seed and comes from the xorshift
  `Rng`. Nothing depends on the clock, on hash-map or set iteration order, or on scheduling.
- **The oracle is obviously correct**: brute force, written in the tests
  file with a `brute` prefix, not a second clever solution that could share
  the reference's bug.
- **Error assertions name the error.** A test that only says "this fails"
  passes for any failure, including an unrelated crash. `#[should_panic]`
  has an `expected` string and error enums are compared by variant.
- **Every test calls the player's code before anything else can fail.**
  `check` proves less than that: that no test *passes* on the template, not
  that each one reaches a `todo!()`. Look for tests that begin by asserting
  on helper state.

## 4. Performance

### Solved means correct

A problem is solved when its tests pass. Speed is not part of validity.
The gauntlet records each solve's test runtime and peak memory, measures the
reference solution the same way on the player's machine, and shows the
comparison; a solution well off the reference is told it could probably be
faster or leaner. The suite runs with `--test-threads=1`, so the runtime is
the sum over all tests and the peak memory that of the largest one.

The target complexity is stated in `problem.toml` and the README as the
goal. Nothing in a problem may say or imply that a slower correct solution
fails, unless a counting test really enforces it.

The 30-second harness timeout only guards against hangs. That is the right
answer to exponential blow-ups (unmemoized recursion making ~10¹⁹ calls):
keep those tests, and state the call count in the comment rather than "too
slow". A polynomial naive approach must never depend on the timeout: it
fires or not depending on the machine.

### Count operations, where the complexity is the point

Where the problem exists to teach a complexity (a sort, a heap, a structure
built from scratch) and its API allows it, make the basic operation
observable, count it, and assert a budget (`<= 2 * n` comparisons to
heapify; `n * ceil(log2 n)` for merge sort). It fails identically on every
machine, instantly, as a normal named test with a clear message
(`"{calls} comparisons for n = {n}"`), and the input can be small.

Wrap the elements in a type whose `Ord`, `Eq`, `Hash` or `Clone` impl
increments a counter, or pass a counting comparator. This works whenever the
API is generic over the element (`T: Ord`, `T: Hash + Eq`, a comparator or
key closure). Find examples with `rg -l 'Cell<usize>' -g tests.rs content`.

- Model: the merge sort problem
  (`content/04_sorting/01_merge_sort/`), test
  `comparisons_stay_within_n_log_n`.
- The counter belongs to one test. Use a `Cell` captured by a closure, or a
  `thread_local!`, never a plain `static`: the tests may be run in parallel
  threads outside the gauntlet.
- Set the budget with a constant-factor margin (2–4× the reference's count)
  so every optimal approach passes, including alternatives the hints
  mention, while the naive approach still fails by orders of magnitude.
  Measure the reference's and the naive mutant's actual counts and put both
  in the report.
- Have the counter panic as soon as the budget is exceeded where it can, and
  size the problem's other large tests so the naive mutant still finishes in
  a few seconds. The player should see the counting failure, not a timeout.
- Counting sees only the operations it counts. A comparison budget can't
  see element moves (a "heap" kept as a sorted array makes few comparisons
  and `O(n)` moves per insert). Such an approach passes, and shows up as
  slow in the comparison with the reference.
- The same idea covers space. Where memory is the lesson (in-place, one
  scratch buffer, `O(1)` extra), count clones or check `capacity()`: see
  `scratch_buffer_is_cloned_once` in merge sort. Never assert on measured
  memory or wall-clock time inside a test.

### Otherwise: one large input

When there's nothing to count (integers in, an integer out), give the
problem one large input, named for what it is (`large_input_with_no_pair`,
not `..._is_not_quadratic`). Its job is to make the difference visible in
the runtime the gauntlet shows, not to fail anyone.

**The early-exit trap.** A large input only shows the naive approach as slow
if it's that approach's *worst case*. A brute-force pair scan passes an
input of 10⁵ elements instantly when the matching pair involves the first
element, because it returns on the first iteration. Work out what the naive
loop does on the test's input: where does it exit? The worst case is usually
"no answer" or "the answer is found last".

**The size.** Measured on a recent Apple Silicon machine, whole suite,
single-threaded, tests compiled without optimizations:

- the **reference** suite stays well under a second, ideally under 0.3 s;
- the obvious **naive mutant passes** in about 2–4 s, which leaves a
  several-fold margin before the timeout on slower machines;
- the naive suite is about 10× the reference's and at least a second
  longer, so the comparison is unmistakable.

Prefer one input at the right size to many repeats, and keep to one such
test per problem: under serial execution, large tests add up. Stay within
the README's constraints.

If the gap between the complexities is too small to show (`O(n)` against
`O(n log n)`), don't force it. Leave the complexity to the hints and the
complexity note, and say so in the report.

**Measuring.** At these sizes there's nothing to extrapolate: run every
naive mutant at full size, whole suite, with `--test-threads=1`, and report
its time next to the reference's. Other auditors load the machine and load
only makes things look slower; if a number misses, re-run it alone once or
twice and take the fastest.

### Deep inputs

At the constraint limit a recursive solution can run out of stack. One of two
things is true, and the tests should make clear which:

- **Recursion is the expected solution** (most tree problems): the deep test
  must not fail a correct recursive solution.
- **Iteration is the lesson** (iterative traversal, iterative `Drop`): one
  test, named for it (`drop_of_long_list_is_iterative`), at a depth that
  overflows the recursive version.

A test thread has a 2 MiB stack. When recursion is expected, the deep test
runs in a thread with a larger stack (`std::thread::Builder::stack_size`)
and re-raises panics with `resume_unwind` so "not yet implemented" still
surfaces. An overflow aborts the whole run, or on some platforms hangs it
until the timeout. Long `Box` chains also drop recursively, so the default
`Drop` is itself a deep-input failure mode.

## 5. The template

Defects to look for:

- **Header.** Line 1 is exactly
  `// <Title> — press i in the gauntlet for the statement, t for the tests.`,
  with the title exactly as in `problem.toml` (`check` enforces it). Line 2 is
  ``// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.``
  The few drills whose placeholder isn't `todo!()` (macro arms) word it to
  match; leave those. An optional `RESTRICTIONS: …` comment is
  line 3.
- **The API is identical** to the solution's and to the README's Signature
  block: names, parameter names, types, bounds, defaults, shared types.
- **The API is idiomatic** by CONTRIBUTING's *Signature guidelines*. A
  violation is a **flag**, with the signature you'd propose;
  you don't change it.
- **Imports** are what a typical solution needs. An import nobody would use
  is a defect; so is a missing one that every solution needs.
- **Bodies** are all `todo!()`. Supporting types (error types, node
  types) are complete. Design-problem fields are left to the player.
- **Doc comments**, where present, describe behaviour in the README's terms
  and don't give the algorithm away.
- The allow line is `#![allow(unused_imports, unused_variables, dead_code)]`,
  plus `clippy::ptr_arg` (with a comment saying why) only when a signature
  takes `&mut Vec<T>` or `&mut String` on purpose.
- The shared `ListNode` / `TreeNode` definitions are verbatim from
  CONTRIBUTING.
- Design-problem fields are `// TODO: your fields here`. No `todo!()` inside
  a `Drop` impl.
- The test hook at the bottom is exactly:

  ```rust
  #[cfg(test)]
  #[path = "tests.rs"]
  mod tests;
  ```

## 6. The solution

The solution works and is lint-clean where `check` lints, or `check` would
have failed. Change it only for one of these defects, not to restyle it:

- **Header**: line 1 is a comment, `<Title> — reference solution.`,
  followed by 1–3 comment lines on the approach and, outside drills, its time
  and space. Missing, or disagreeing with `complexity` in `problem.toml`.
- **Not optimal**: it doesn't meet its stated complexity, or a clearly
  better bound is achievable under the same constraints. Flag this one
  unless the fix is small and certain.
- **Rule violations**: breaking its own `RESTRICTIONS` line, or one of the
  rules below.
- **Disagrees with the hints**: the hints lead to one approach and the
  solution takes another. Usually the hints change, not the solution.
- **Clearly unidiomatic**. The bar is "a reviewer fluent in the language
  would ask for this change", not "I'd write it another way".

Rules: no `unsafe`; no `unwrap()` on something that can fail
(`expect("why it can't")` is right for true invariants); no blanket
`#![allow]`, only one lint on one item with a comment. Unidiomatic: manual
index loops where the iterator form is plainly simpler, `contains_key`
followed by `insert` instead of `entry`, cloning to appease the borrow
checker where a borrow works, `-1`-style sentinels internally.

## 7. README.md

### Structure

These `##` sections, with these exact headings, in this order:

| Section | Other problems | Drills |
|---|---|---|
| `## Description` | required | required |
| `## Restrictions` | if and only if the template has a `RESTRICTIONS` line | same |
| `## Signature` | required | only when the drill has one API or shared types to show |
| `## The drills` | not used | required, unless the drill is a single API with a Signature |
| `## Examples` | required | required |
| `## Constraints` | required | only with a Signature |
| `## Rust notes` | required, 2–5 bullets | required, 2–5 bullets |

No `#` title (it comes from `problem.toml`), no complexity, no hints. An
extra section is a defect unless the topic needs the explanation (a drill
whose API rests on a concept the docs explain poorly, say).

### Content defects

- **Description**: ambiguous, incomplete, or wrong. It must say what's
  returned in every case, including no answer, ties, and whether order
  matters. Most open with "Given a …, return …" or "You are given …"; an
  opener like "Write a function that…" or "In this problem…" is a deviation
  worth fixing, other openers are fine.
- **Signature**: differs from the template; or the types are surprising
  (owned where borrowed is usual, an optional return) and no sentence says
  why.
- **Examples**: not a single `rust` code block of `assert!` / `assert_eq!`
  calls; an example that wouldn't run against the signature or contradicts
  the tests; an example not asserted in `tests.rs` (outside drills).
- **Restrictions**: differ in substance from the template's `RESTRICTIONS`
  line.
- **Constraints**: missing bounds for an input; bounds that contradict the
  tests' largest inputs; values written as `1e5` or `100000` instead of
  `10^5`; a string problem that relies on ASCII without saying so; a sum or
  product that can overflow the stated type without a note.
- **Rust notes**: give the algorithm away (a note that names the
  technique belongs in a hint); recommend an API that neither the solution
  nor any equally good solution would use; fewer than 2 or more than 5.
- **Links**: every reference-style label used (``[`map`]``) has a definition
  and every definition is used; URLs follow the patterns of the neighbouring
  READMEs (doc.rust-lang.org). Don't fetch them.
- **Anything else**: typos, broken markdown, mentions of where the problem
  comes from.

Fix errors, ambiguities and format deviations. Don't rewrite prose that is
correct and clear because you'd word it differently.

## 8. problem.toml and hints

- **`title`**: the same string as in the first lines of the template, the
  solution and the tests file.
- **`difficulty`**: judged against its neighbours in the section, not the
  problem's reputation. If it looks wrong, flag it with the neighbours you
  compared.
- **`complexity`** (required outside drills; drills usually omit it):
  two markdown bullets, time and space, formatted like the reference
  problem's; agrees with the solution's header; says "expected" for hashing
  and what the space excludes ("plus the output") where relevant.

Those, and `hints`, are all the keys there are.

### Hints

A stuck player reveals hints one at a time, and a hint that gives too much
away spoils the problem. So each hint must be worth it: after reading hint *k*, a stuck
player has an idea they couldn't get from the README, the Rust notes and
the earlier hints.

There are 2 or 3, gentlest first. The usual ladder:

1. **The observation**: a reframing, or the property that matters ("the
   answer only depends on the running minimum"). When finding the technique
   is the point of the problem, hint 1 doesn't name it.
2. **The technique**: what to maintain and why that works.
3. **The tricky detail** (optional): the invariant or ordering most attempts
   get wrong ("mark visited when you push, not when you pop").

A valid variation: hint 1 gives a simpler, slower approach and hint 2 the
target one. Two Sum does this.

Defects:

- restates the README or a Rust note
- too vague to act on ("think about efficiency", "consider the edge cases")
- hint 1 gives away the whole solution
- mostly code (an expression in a code span is fine)
- describes an approach the solution doesn't take, without saying it's an
  alternative
- two hints carry the same idea
- factually wrong, or wrong about the complexity

Rewrite a defective hint; leave a serviceable one alone. If the first hint
is fine and the second is the problem, change only the second.

## 9. Cross-file consistency

The files describe one problem. These must agree:

| What | Where |
|---|---|
| Title | `problem.toml`; line 1 of the template, the solution and the tests file |
| Public API | README Signature; the template; the solution |
| Examples | README Examples; the tests file |
| Restrictions | README section; template `RESTRICTIONS` line; what the solution does |
| Constraints | README; the largest inputs in the tests; the numeric types |
| Complexity | `problem.toml`; the solution's header; the performance test |
| Approach | hints; the solution; README Rust notes |

## 10. Drills: what's different

This section covers the drill sections (`drill = true` in `section.toml`:
the two Rust Toolkit sections). A drill is 15–25 small
functions (sometimes fewer, sometimes methods or trait impls), each a line or two once you know the API. The idea of section 3
holds, at a smaller scale.

- **Failure modes are per function**, and the main one is *the neighbouring
  API*: the wrong-but-similar call a learner would reach for. Does a test
  tell them apart? Then the usual boundaries: empty, absent, the error
  path, non-ASCII. For example: `unwrap_or` where `unwrap_or_else` is asked
  for (does a test check laziness?), `map` where `and_then` is needed,
  `split` where `split_whitespace` differs on repeated spaces.
- **1–3 tests per function, named after the function**
  (`or_else_build_is_lazy`). A function with no test is a missing test. Two
  tests of one function with the same target are redundant.
- **No oracle, no performance tests, no example tests, usually no `complexity`.**
  README examples must agree with the tests' behaviour but needn't be
  repeated as tests.
- **Mutants are one-liners here**: swap in the neighbouring API and see
  whether anything fails. Run them for functions where the difference is
  subtle; the rules about when you must run one still apply.
- **Template**: functions are grouped under comment banners; each doc
  comment says what the function does and names the API or
  idea it's after in parentheses, as in ``(`map_or`)``. A named API the
  solution doesn't use is a defect.
- **README**: *The drills* table lists every public function of the
  template, in the template's order and grouping, with the APIs linked to
  doc.rust-lang.org. A function missing from the table, or a table entry
  with no function, is a defect.
- **Hints** can't follow the observation/technique ladder. Good drill hints
  point at where to look (the docs page and how it's organised), then
  separate the APIs that are easy to confuse, then cover the hardest group.
  The same defects apply.
- **Restrictions** are usually what makes the drill a drill (no `match`, no
  `for` loops, combinators only). Check the solution honours them in every
  function.

## Report

Your final message is exactly this, and nothing else:

```markdown
## <id> — <verdict>

### Failure modes
| Failure mode | Caught by | Evidence |
|---|---|---|
| doesn't skip duplicate triples | `duplicate_triples_reported_once` | mutant |
| off-by-one on the first element | `underflow_on_first_element` (added) | mutant |
| answer at the last position | `answer_at_end` | read |

### Performance
`large_input_is_not_quadratic`: counting. Reference 1,988 comparisons, naive
mutant 499,500, budget 8,000 at n = 1,000.

### Changes
- tests: added `underflow_on_first_element`: no test reached index 0 with …
- tests: merged `large_1` into `large_2`: same target; mutant_quadratic
  fails both.
- problem.toml: rewrote hint 2: it restated the first Rust note.

### Possibly redundant tests
- `one_item`: same boundary as `single_element`; kept, no mutant separates
  them.

### Flags
- Signature: `fn f(v: Vec<i32>)` only reads `v`. Propose `&[i32]`.

### Not done
- Complexity isn't enforced: `O(n)` vs `O(n log n)` is too small a gap to
  time, and the API has nothing to count.

`gauntlet check <id>`: ok
```

- **`<id>`** is the problem's id, its slug (`three_sum`). Test names are
  written as they appear in `tests.rs`.
- **Verdict** follows mechanically from the rest of the report. Take the
  first that applies:
  1. `blocked`: `check` doesn't end on `ok`, or you couldn't complete the
     audit. Say why under *Not done*.
  2. `flagged`: *Flags* is not "none", whether or not you changed files.
  3. `fixed`: *Changes* is not "none".
  4. `clean`: no changes, no flags.
- A **flag** is a decision a human has to make: something from the "flag,
  don't change" list, a needed change to a shared file, a defect you found
  but couldn't fix with confidence. An observation you considered and
  decided was fine is not a flag; leave it out.
- *Possibly redundant tests* lists every non-exempt test that has no target
  of its own and that you kept. It doesn't affect the verdict.
- **Evidence** is `mutant` (you ran it and saw the test fail) or `read` (you
  reasoned it from the code). Be accurate; nobody is graded on the number of
  mutants.
- *Performance*: for a counting test, the reference's count, the mutant's
  count and the budget. For a large input, the reference's and the
  naive mutant's whole-suite times. For drills, omit the section. For a problem with no
  performance requirement, write "none".
- Every entry under *Changes* names the defect it fixed.
- Write "none" under a heading with nothing to report. The last line is the
  final status of `check`, and nothing follows it.

## For whoever launches the audits

Auditors don't need this section.

1. **Commit first.** Each audit should be a reviewable, revertible diff.
   With uncommitted files there's no way to see or undo what an auditor
   changed.
2. `cargo build` once (auditors use `target/debug/gauntlet`), then `cargo
   run -- check` over everything for a baseline (it stops at once if rustc
   or clippy is missing). Fix what's already failing, or expect those
   audits to start there.
3. One agent per problem, with a prompt as plain as: "Audit the problem
   `<id>` by following docs/maintainers/problem-audit.md. Read that file in
   full first. Your final message is the report it specifies." Use the
   id. Don't add hypotheses about what's wrong with the
   problem: an auditor told what to find tends to find it.
4. Keep concurrency at or below the number of performance cores: the
   timing measurements tolerate load, not starvation.
5. Don't rebuild the binary or run `cargo clean` while audits are running:
   the scratch directories live in `target/audit/`. That includes `cargo
   run`, which rebuilds when anything under `content/` has changed.
6. Afterwards: run `cargo run -- check` over everything again; read the diffs
   of every `fixed` problem, deleted tests first; collect the flags, which
   will include proposals for shared files that no auditor was allowed to
   touch; run `cargo run -- readme` to refresh the index; delete
   `target/audit/`.
