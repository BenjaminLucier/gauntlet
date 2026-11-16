# The README's screenshots and demo

The images in `docs/images/` are recorded, not taken by hand, so they can be
redone whenever the interface changes:

```sh
brew install vhs                  # also installs ttyd and ffmpeg
brew install --cask font-iosevka
demo/record.sh                    # about two minutes
```

then look over the result and commit `docs/images/`.

## How it works

[VHS](https://github.com/charmbracelet/vhs) runs a shell in a headless
browser, types what a `.tape` file says, and saves frames as PNGs or a GIF.
`record.sh`:

1. builds `gauntlet` and the `demo_data` example in release mode,
2. writes a fresh history for each recording with
   [`examples/demo_data.rs`](../examples/demo_data.rs): six weeks of a keen
   user who has solved everything up to *Tree Traversals*, the same every
   time apart from ending yesterday,
3. runs [`screenshots.tape`](screenshots.tape) once in Catppuccin Mocha
   (`GAUNTLET_THEME=dark`) and once in Latte (`light`),
4. runs [`demo.tape`](demo.tape) in Mocha for `demo.gif`.

Recordings never touch your own progress: each gets a home directory of its
own under a temporary directory, with the XDG variables pointing into it, so
gauntlet shows its paths as `~/.local/share/gauntlet`. `RUSTUP_HOME` and
`CARGO_HOME` keep pointing at your toolchain.

## The solve in the demo

VHS has one terminal, and gauntlet fills it, so there's no editor to type
into. [`solve.sh`](solve.sh) stands in for one: `demo.tape` starts it in the
background before gauntlet, and it saves
[a buggy Invert Binary Tree](invert_binary_tree_bug.rs) a few seconds in,
then the reference solution. The tape's sleeps are timed around those
saves; if compiling gets slower and the rating key arrives before the tests
pass, lengthen the sleep after `Type "a"`.

## The reference timings

Gauntlet measures a reference solution only after your first pass of that
problem, so a fresh history would show no comparisons. `demo_data` writes
made-up measurements to `reference.json` instead, under the same key
gauntlet uses (this version, `rustc --version` and a hash of the solution
and tests). If gauntlet changes that key, they stop showing until
`demo_data` is changed to match.
