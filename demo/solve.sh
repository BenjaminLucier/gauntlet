#!/usr/bin/env bash
# Plays the editor in demo.tape, which starts this in the background before
# gauntlet: after $1 seconds it saves a buggy Invert Binary Tree, then
# after $2 more the reference solution. record.sh sets the paths.
set -euo pipefail
sleep "$1"
cp "$DEMO_DIR/invert_binary_tree_bug.rs" "$WORKING_FILE"
sleep "$2"
# A working file holds only your code: gauntlet attaches the tests itself.
sed '/^#\[cfg(test)\]$/,$d' "$SOLUTION" > "$WORKING_FILE"
