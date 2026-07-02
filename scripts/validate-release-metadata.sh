#!/usr/bin/env sh
set -eu

test ! -f PENTEST.md
test -f LICENSE-MIT
test -f LICENSE-APACHE
test -f SECURITY.md
test -f CHANGELOG.md
test -x scripts/validate-release-readiness.sh
test -x scripts/validate-spec-lock-policy.sh
test -x scripts/test-release-readiness.sh
test -x scripts/check_latest_tools.sh
test -x scripts/release_crates.py
test -f release-crates.toml
test -f docs/CRATE_VERSION_MATRIX.md
test -f release-notes/RELEASE_NOTES_0.1.0.md
test -f docs/spec-source-policy.md
grep -q 'ledger_repo' spec-lock.toml
grep -q 'local_reference_store_env' spec-lock.toml
grep -q 'local_reference_store_default' spec-lock.toml
if grep -q 'spec_required = true' spec-lock.toml; then
    grep -Eq 'ledger_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'node_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'ouroboros_network_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'cips_rev = "[0-9a-f]{40}"' spec-lock.toml
fi
grep -q 'license = "MIT OR Apache-2.0"' Cargo.toml
grep -q 'repository = "https://github.com/valkyoth/cardano"' Cargo.toml
grep -q 'channel = "1.96.1"' rust-toolchain.toml
grep -q 'rust-version = "1.90"' Cargo.toml
