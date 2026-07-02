#!/usr/bin/env sh
set -eu

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/check_doc_links.sh
scripts/validate-release-metadata.sh
scripts/validate-spec-lock-policy.sh check
scripts/validate-modularity-policy.sh check
scripts/validate-security-policy.sh
scripts/release_crates.py --check
scripts/materialize_fuzz_seeds.py --check
python3 scripts/test-release-crates.py
scripts/test-release-readiness.sh
for package in \
    cardano-valkyoth-cbor \
    cardano-valkyoth-primitives \
    cardano-valkyoth-crypto \
    cardano-valkyoth-address \
    cardano-valkyoth-ledger \
    cardano-valkyoth-script \
    cardano-valkyoth-governance \
    cardano-valkyoth-node \
    cardano-valkyoth-rpc \
    cardano-valkyoth-sanitization \
    cardano-valkyoth-signer \
    cardano-valkyoth-testkit \
    cardano; do
    if [ "$package" = "cardano" ]; then
        cargo package -p "$package" --allow-dirty \
            --config 'patch.crates-io.cardano-valkyoth-primitives.path="crates/cardano-valkyoth-primitives"'
    else
        cargo package -p "$package" --allow-dirty
    fi
done
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --manifest-path fuzz/Cargo.toml
