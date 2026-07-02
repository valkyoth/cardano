#!/usr/bin/env sh
set -eu

mode="${1:-check}"
if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-modularity-policy.sh check" >&2
    exit 2
fi

violations="$(find crates -type f -name '*.rs' -exec wc -l {} \; | awk '$1 > 500 { print }')"
if [ -n "$violations" ]; then
    echo "Rust files exceed 500 lines:" >&2
    echo "$violations" >&2
    exit 1
fi

grep -q '"crates/cardano-valkyoth-primitives"' Cargo.toml
grep -q '"crates/cardano-valkyoth-cbor"' Cargo.toml
grep -q '"crates/cardano-valkyoth-crypto"' Cargo.toml
grep -q '"crates/cardano-valkyoth-address"' Cargo.toml
grep -q '"crates/cardano-valkyoth-ledger"' Cargo.toml
grep -q '"crates/cardano-valkyoth-script"' Cargo.toml
grep -q '"crates/cardano-valkyoth-governance"' Cargo.toml
grep -q '"crates/cardano-valkyoth-node"' Cargo.toml
grep -q '"crates/cardano-valkyoth-rpc"' Cargo.toml
grep -q '"crates/cardano-valkyoth-signer"' Cargo.toml
grep -q '"crates/cardano-valkyoth-testkit"' Cargo.toml
