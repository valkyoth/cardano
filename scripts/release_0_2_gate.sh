#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/check_latest_tools.sh
scripts/release_crates.py --check
scripts/test-release-readiness.sh
scripts/validate-spec-lock-policy.sh check
cargo deny check
cargo audit
