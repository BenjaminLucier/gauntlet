#!/usr/bin/env bash
# Records the README's screenshots (docs/images/{mocha,latte}/) and demo
# GIF (docs/images/demo.gif) with VHS, each from a fresh demo history.
# Needs vhs and the Iosevka font, and ffmpeg (which vhs installs); see
# demo/README.md.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"
out="$root/docs/images"
# Short, as it shows in the failing tests' panic messages.
tmp=$(mktemp -d /tmp/gauntlet-demo.XXXXXX)
trap 'rm -rf "$tmp"' EXIT

cargo build --release --locked --quiet
cargo build --release --locked --quiet --example demo_data
export PATH="$root/target/release:$PATH"
# Each recording gets a home of its own, so gauntlet shows its paths as
# ~/.local/share/gauntlet; rustup's stay where they are.
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}" CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
unset GAUNTLET_CONTENT_DIR GAUNTLET_LOG NO_COLOR

# record TAPE THEME VHS_THEME GAUNTLET_THEME
record() {
  local tape="$tmp/$1-$2.tape" home="$tmp/home-$1-$2"
  sed -e "s|@OUT@|$out|g" -e "s|@TMP@|$tmp|g" -e "s|@THEME@|$2|g" -e "s|@VHS_THEME@|$3|g" "demo/$1.tape" > "$tape"
  "$root/target/release/examples/demo_data" "$home/.local/share/gauntlet" "$home/.cache/gauntlet"
  mkdir -p "$out/$2"
  echo "recording $1 ($2)"
  HOME="$home" XDG_DATA_HOME="$home/.local/share" XDG_CACHE_HOME="$home/.cache" \
    XDG_STATE_HOME="$home/.local/state" GAUNTLET_THEME="$4" \
    DEMO_DIR="$root/demo" WORKING_FILE="$home/.local/share/gauntlet/invert_binary_tree.rs" \
    SOLUTION="$root/content/10_trees/02_invert_binary_tree/solution.rs" \
    vhs --quiet "$tape"
}

record screenshots mocha "Catppuccin Mocha" dark
record screenshots latte "Catppuccin Latte" light
record demo mocha "Catppuccin Mocha" dark
