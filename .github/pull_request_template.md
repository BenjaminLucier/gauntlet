<!-- Thanks for contributing! A short description is plenty. -->

## What and why

<!-- What this changes and the reason for it. Link an issue with "Fixes #123". -->

## Checklist

- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:` ...)
- [ ] `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass
- [ ] The README is updated if behaviour, keys or commands changed

For changes under `content/`:

- [ ] `gauntlet check <problem>` passes for every problem touched (`cargo run --release -- check <problem>`)
- [ ] `gauntlet readme` was run if a problem was added, moved or renamed
