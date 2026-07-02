# Changelog

All notable changes to `cardano` will be documented here.

## 0.3.0 - Unreleased

- Added explicit Cardano primitive domains for network ids, eras, slots,
  epochs, block numbers, coins, transaction ids, block hashes, datum hashes,
  script hashes, key hashes, policy ids, credentials, and bounded asset names.
- Re-exported the primitive domains through the `cardano` facade crate.
- Pinned official Cardano source revisions for the first protocol-domain
  milestone and recorded the primitive evidence in the spec matrix.
- Hardened spec-lock validation so pinned official source revisions must be
  fetchable from their declared upstream repositories.
- Changed `AssetName` equality, hashing, and ordering to use significant bytes
  only, preventing future unused-tail padding from affecting semantic identity.
- Added the explicit `scripts/release_0_3_gate.sh` milestone gate.

## 0.2.0 - 2026-07-02

- Added the explicit `scripts/release_0_2_gate.sh` milestone gate.
- Hardened release-readiness error diagnostics for permanent pentest report
  metadata.
- Added negative release-readiness tests for non-PASS status and blank tester
  fields.
- Hardened the GitHub Actions spec-lock check so pull-request base refs are
  passed through the environment instead of interpolated into shell text.
- Updated release-crate tracking so only the `cardano` facade crate is
  republished for `v0.2.0`, allowing crates.io to show the repository README
  while support crates remain on `0.1.0`.

## 0.1.0 - 2026-07-02

- Repository foundation for a security-oriented Cardano Rust workspace.
- Added policy, release, spec-source, threat-model, supply-chain, fuzzing, and
  security documentation.
- Added empty first-party crate boundaries for primitives, CBOR, crypto,
  addresses, ledger, scripts, governance, node, RPC, sanitization, signer,
  testkit, and the `cardano` facade.
- Added local release gates, crate publication planning, SBOM generation hooks,
  and pentest-before-tag readiness checks.
- Added spec-lock policy validation and documented the future node/query
  panic-isolation decision before untrusted message decode paths ship.
