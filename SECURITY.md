# Security policy

## Supported versions

Only the latest release of Gauntlet receives security fixes.

## Reporting a vulnerability

Please report vulnerabilities privately through GitHub:
[Report a vulnerability](https://github.com/BenjaminLucier/gauntlet/security/advisories/new).
If you can't use GitHub, email benjaminlucier@gmail.com instead. Please don't
open a public issue or PR for a suspected vulnerability. Include steps to
reproduce and the version affected.

You can expect an acknowledgement within 3 working days. Fixes are coordinated
with you, and details are disclosed publicly once a fixed release is available,
or after 90 days at the latest. The advisory credits you unless you'd rather
stay anonymous.

## What Gauntlet trusts

Gauntlet compiles and runs code on your machine: your attempt at each
problem, together with that problem's tests, built with the `rustc` (and,
when you ask for a review, `clippy-driver`) on your `PATH` and run as you,
with your permissions. There is no sandbox. That is
the point of the tool, not a vulnerability, but it sets the trust model:

- **Your own code** is trusted as much as anything else you compile and run.
- **The bundled problems** (tests, templates, reference solutions) are part
  of the release and are trusted like the binary itself. They use the
  standard library only, and changes to them are reviewed like code.
- **Problems from elsewhere.** Pointing `GAUNTLET_CONTENT_DIR` at a content
  directory you did not write means running its tests on your machine: treat
  it like running someone else's `cargo test`.

## Scope

In scope: the code and bundled content in this repository, in particular

- a way for the bundled content to do something other than test your
  solution,
- Gauntlet running code or writing files outside its data, cache and state
  directories without being asked,
- parsing of its own files (content, event log, run state).

Out of scope:

- Running your own code or a content directory you chose to trust: that is
  what the tool does (see above).
- Vulnerabilities in dependencies that don't affect this code. Report them
  upstream; fixes are picked up through Dependabot, and dependencies are
  audited in CI with [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).

## Safe harbour

Good-faith research that follows this policy, avoids accessing other people's
data, and gives us reasonable time to fix issues will not be pursued by the
maintainers.
