#!/usr/bin/env bash
# Print the Homebrew formula for a release.
# Usage: homebrew-formula.sh VERSION CHECKSUM_DIR
# CHECKSUM_DIR holds the release's gauntlet-<target>.sha256 files.
set -euo pipefail

version=$1
dir=$2
base="https://github.com/BenjaminLucier/gauntlet/releases/download/v${version}"

# The upload action writes `<hash> *gauntlet-<target>.tar.gz` to
# gauntlet-<target>.sha256. Read them all up front: a failure inside the
# heredoc's $(...) would not stop the script.
sha() {
  local file="$dir/gauntlet-$1.sha256" hash
  [ -f "$file" ] || { echo "missing $file" >&2; exit 1; }
  hash=$(cut -d ' ' -f 1 < "$file")
  [[ $hash =~ ^[0-9a-f]{64}$ ]] || { echo "no SHA-256 in $file" >&2; exit 1; }
  echo "$hash"
}

# The oldest Rust the problems are checked on: the crate's rust-version.
rust_version=$(sed -n 's/^rust-version = "\(.*\)"/\1/p' "$(dirname "$0")/../../Cargo.toml")
[ -n "$rust_version" ] || { echo "no rust-version in Cargo.toml" >&2; exit 1; }

mac_arm=$(sha aarch64-apple-darwin)
mac_intel=$(sha x86_64-apple-darwin)
linux_arm=$(sha aarch64-unknown-linux-musl)
linux_intel=$(sha x86_64-unknown-linux-musl)

# No `depends_on "rust"`: most Rust users have rustup, which conflicts with
# Homebrew's rust formula. The caveat says what is needed instead.
cat <<FORMULA
class Gauntlet < Formula
  desc "128 Rust practice problems, from knowing Rust to being fluent in it"
  homepage "https://github.com/BenjaminLucier/gauntlet"
  version "${version}"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "${base}/gauntlet-aarch64-apple-darwin.tar.gz"
      sha256 "${mac_arm}"
    end
    on_intel do
      url "${base}/gauntlet-x86_64-apple-darwin.tar.gz"
      sha256 "${mac_intel}"
    end
  end

  on_linux do
    on_arm do
      url "${base}/gauntlet-aarch64-unknown-linux-musl.tar.gz"
      sha256 "${linux_arm}"
    end
    on_intel do
      url "${base}/gauntlet-x86_64-unknown-linux-musl.tar.gz"
      sha256 "${linux_intel}"
    end
  end

  def install
    bin.install "gauntlet"
  end

  def caveats
    <<~EOS
      gauntlet compiles your solutions with the Rust toolchain on your PATH,
      so it needs Rust ${rust_version} or later with clippy (https://rustup.rs):
        rustup component add clippy
      gauntlet says so when it starts if rustc is missing or too old.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/gauntlet --version")
  end
end
FORMULA
