#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/release_crates.py --check
cargo test -p cardano-valkyoth-primitives --all-features
cargo test -p cardano --all-features
cargo deny check
cargo audit
