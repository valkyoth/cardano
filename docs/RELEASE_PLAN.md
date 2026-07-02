# cardano Release Plan To 1.0

Status: planning document

This plan is intentionally granular. `cardano` is security-sensitive protocol
software, so each milestone must be small enough to review, test, pentest, and
stop cleanly before tagging.

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch/fix release for milestone N
v1.0.0      first serious production-ready Cardano crate
```

## Release Principles

Every release must have:

- a clear definition of done;
- a local verification command;
- security review notes;
- known limitations;
- release notes;
- dependency-policy evidence;
- spec-source evidence for protocol behavior;
- completed pentest evidence for the exact implementation commit being
  reviewed;
- no hidden dependency on one developer machine.

Every release should prefer:

- one protocol boundary at a time;
- fixtures before broad implementation;
- pinned official Cardano source revisions before consensus-sensitive code;
- negative and adversarial tests with each parser;
- explicit era, network, slot, epoch, and protocol-parameter context over
  global "latest" behavior;
- no default networking, signing, wallet, local key storage, or Plutus
  execution.

## Pentest Before Tags

Every version must pass a security review and pentest before it is tagged. This
applies to `v0.N.P` patch tags as well as milestone tags.

A version is not tag-ready until:

- `scripts/checks.sh` passes;
- `cargo deny check` passes;
- `cargo audit` passes;
- `scripts/generate-sbom.sh` succeeds;
- release notes exist at `release-notes/RELEASE_NOTES_X.Y.Z.md`;
- a pentest report exists at `security/pentest/vX.Y.Z.md`;
- the pentest report names the exact full 40-character `Reviewed-Commit:`;
- the pentest report has `Status: PASS`;
- the pentest report has non-blank `Tester:` and `Scope:` fields;
- the pentest report has a `Date: YYYY-MM-DD` field;
- `sbom/cardano.spdx.json` exists and is non-empty;
- the tag does not already exist locally;
- `scripts/validate-release-readiness.sh vX.Y.Z` passes.

When a version's implementation criteria are done, stop and say:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

No tag is created at that point.

## Crate Versioning And Publish Order

Workspace crates use independent versions after the foundation release. The
facade crate remains `cardano`, but support crates are not republished just
because another crate changed.

Track every release in `release-crates.toml` and
`docs/CRATE_VERSION_MATRIX.md`.

## Phase 0: Repository And Release Discipline

### v0.1.0 - Repository Foundation

Goal: initialize the serious Rust workspace and policy baseline.

Deliverables:

- Rust stable `1.96.1` pinned.
- Rust `1.90.0` through `1.96.1` compatibility policy.
- Focused no_std workspace crate boundaries.
- CI, dependency policy, security policy, release notes.
- Implementation, release, scope, threat-model, modularity, toolchain, unsafe,
  spec, and supply-chain docs.

Verification:

- `scripts/checks.sh`
- `scripts/check_latest_tools.sh`
- `scripts/release_0_1_gate.sh`

Exit criteria:

- A new contributor can understand the scope, security posture, and release
  process from the repository docs.

### v0.2.0 - Release Readiness Gate

Goal: make the pentest-before-tag process and crates.io publish order
enforceable by local tooling.

Deliverables:

- release-readiness validation for `cardano` artifacts;
- support-crate publish order validation;
- SBOM presence checks;
- spec-source policy checks;
- tag-exists guard.

## Phase 1: Primitive, Error, And Resource Foundation

### v0.3.0 - Domain Newtypes

Goal: make Cardano numeric, byte, and identity domains explicit.

Deliverables include network id, slot, epoch, coin, hashes, transaction id,
policy id, asset name, credential, and era identifiers.

### v0.4.0 - Stable Error Model

Goal: establish non-panicking error categories for protocol operations.

### v0.5.0 - Decode Budget Model

Goal: define resource limits for all untrusted byte decoding.

## Phase 2: CBOR And CDDL

### v0.6.0 - CBOR Scalar Decoder

### v0.7.0 - CBOR Array And Map Decoder

### v0.8.0 - Canonical CBOR Policy

### v0.9.0 - CDDL Fixture Import

### v0.10.0 - CBOR Encoding Round Trips

### v0.11.0 - CBOR Fuzz Harness

## Phase 3: Addresses And Credentials

### v0.12.0 - Shelley Address Decode

### v0.13.0 - Byron Address Decode

### v0.14.0 - Address Encoding

### v0.15.0 - Credential And Reward Account Domains

## Phase 4: Era-Aware Ledger Data

### v0.16.0 - Era And Protocol Parameter Context

### v0.17.0 - Transaction Body Shell

### v0.18.0 - Transaction Inputs, Outputs, Fees, And Validity Interval

### v0.19.0 - Witness Sets And Verification Boundary

### v0.20.0 - Metadata And Auxiliary Data

### v0.21.0 - Multi-Asset Values

### v0.22.0 - Blocks And Headers

## Phase 5: Ledger Validation Typestates

### v0.23.0 - UTxO Validation Typestates

### v0.24.0 - Fee And Value Conservation Checks

### v0.25.0 - Certificate And Delegation Data

### v0.26.0 - Withdrawal And Reward Accounting Boundaries

### v0.27.0 - Era Transition Boundaries

## Phase 6: Scripts And Plutus Boundaries

### v0.28.0 - Native Script Decode And Validation

### v0.29.0 - Plutus Data Decode And Encoding

### v0.30.0 - Script Integrity Hash Boundary

### v0.31.0 - Execution Unit Policy

### v0.32.0 - Optional Plutus Execution Adapter Decision

## Phase 7: Governance And Conway

### v0.33.0 - Conway Governance Data

### v0.34.0 - Votes, Proposals, And DRep Domains

### v0.35.0 - Committee And Constitutional Committee Domains

### v0.36.0 - CIP-1694 Validation Helpers

## Phase 8: Conformance And Differential Testing

### v0.37.0 - Ledger Fixture Harness

### v0.38.0 - CDDL Differential Test Harness

### v0.39.0 - Ledger Differential Test Harness

### v0.40.0 - Era Regression Corpus

## Phase 9: Optional Node, Query, And Signer Boundaries

### v0.41.0 - Node Protocol Threat Model

### v0.42.0 - Node-To-Client Type Boundary

### v0.43.0 - Node-To-Node Type Boundary

### v0.44.0 - Query/Submit Transport Admission

### v0.45.0 - Query Trust Models And Redaction

### v0.46.0 - Signer Interface

### v0.47.0 - Local Signer Fallback Decision

## Phase 10: Production Hardening

### v0.48.0 - Platform Matrix

### v0.49.0 - Kani Formal Verification Harness

### v0.50.0 - Public API Stability Pass

### v0.51.0 - Independent Audit Remediation

### v0.52.0 - Release Evidence Dry Run

## v1.0.0 - Production Cardano Toolkit

Goal: ship the first serious production-ready Cardano crate.

Exit criteria:

- official conformance suites pass for every claimed feature;
- all supported Rust versions from `1.90.0` through `1.96.1` are checked;
- no unresolved critical or high dependency/advisory/audit findings;
- SBOM and provenance exist;
- independent audit findings are resolved or explicitly documented;
- public API stability review is complete.
