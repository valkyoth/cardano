# cardano Release Plan To 1.0

Status: planning document

This plan is intentionally granular. `cardano` is security-sensitive Cardano
protocol software, so each milestone must be small enough to review, test,
pentest, and stop cleanly before tagging.

The list below is not a maximum. Add patch releases or split a milestone before
implementation if the work no longer fits in one safe review pass.

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
- explicit panic-isolation decisions before node or query binaries accept
  untrusted messages;
- explicit era, network, slot, epoch, and protocol-parameter context over
  global "latest" behavior;
- no default networking, signing, wallet, local key storage, node transport, or
  Plutus execution.

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

`scripts/check_latest_tools.sh` is an advisory networked current-version check.
Run it before updating pinned tools and before release when network access is
available, but do not make tag readiness depend on live upstream state.

When a version's implementation criteria are done, stop and say:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

No tag is created at that point.

### Pentest Handoff Flow

Use this loop for every version:

1. Implementation reaches the version stop point.
2. Local gates pass: `scripts/checks.sh`, `cargo deny check`, and
   `cargo audit`.
3. The maintainer runs pentest and writes temporary findings to root
   `PENTEST.md`.
4. Findings are reviewed and fixed.
5. Documentation, tests, and release notes are updated for the fixes.
6. `PENTEST.md` is removed after findings are handled.
7. Local gates are run again.
8. GitHub CI and CodeQL default setup are checked after the fix commit.
9. A permanent report is written at `security/pentest/vX.Y.Z.md` only when the
   exact implementation commit has passed with `Status: PASS`.
10. Commit only the permanent report as the release report commit.
11. GitHub CI and CodeQL default setup are checked on the release report commit.
12. `scripts/validate-release-readiness.sh vX.Y.Z` passes.
13. Tagging and pushing tags happen only when explicitly requested.

Root `PENTEST.md` is temporary scratch input. It must not be committed. The
permanent report is part of the release tag. Because committing the report
changes `HEAD`, the report records `Reviewed-Commit:` rather than claiming to
hash itself. The release-readiness gate requires the tag candidate commit to
have the reviewed commit as its first parent and to change only the permanent
report file.

## Crate Versioning And Publish Order

Workspace crates use independent versions after the foundation release. The
facade crate remains `cardano`, but support crates are not republished just
because another crate changed.

Track every release in `release-crates.toml` and
`docs/CRATE_VERSION_MATRIX.md`:

- `code`: the crate received meaningful implementation, API, or documentation
  changes and uses the release version for the facade crate or an independent
  support-crate minor bump after `v0.1.0`;
- `dependency`: the crate only needs a manifest update because a related crate
  changed outside its current dependency range;
- `metadata`: the crate must be republished with the milestone version to
  correct immutable crates.io package metadata;
- `unchanged`: the crate stays on the previous published version and is not
  published.

`scripts/release_crates.py --check` validates the table against Cargo metadata
and refuses accidental lockstep publication. The script still publishes in
dependency order, but only for crates marked `publish = true`.

## Completeness Review Register

Every planning or pentest pass must check this register for implied work that
has not been assigned to a release. If a row affects the 1.0 Cardano protocol
scope, it must have a versioned milestone before work continues past the
relevant dependency point.

| Gap | Resolution |
| --- | --- |
| Sanitization boundary was easy to forget because it has no protocol behavior yet. | Added `cardano-valkyoth-sanitization` to `v0.1.0` and tracked it in release tooling. |
| Node/RPC panic blast radius is not a current code bug but becomes live once untrusted message decode paths exist. | Added panic-isolation requirements before node/query decode milestones ship. |
| Empty spec revisions are acceptable for the scaffold but unsafe for ledger, script, or governance implementation. | Added `scripts/validate-spec-lock-policy.sh` and path-sensitive CI before protocol crates change. |
| Plutus execution is a large security boundary and must not be pulled into default builds. | Added `v0.32.0 - Optional Plutus Execution Adapter Decision` before any execution adapter can be admitted. |
| Node and query transport behavior can imply trust in unverified state. | Added separate node threat-model, node protocol, query admission, and query trust-model milestones before transport support. |
| Formal verification evidence was not scheduled. | Added `v0.49.0 - Kani Formal Verification Harness` as extra assurance, not a replacement for fuzzing, conformance tests, pentest, or audit. |

## Milestone Stop Contract

Every milestone below ends with the same rule: after the verification commands
pass, stop before tagging and request pentest for the exact commit. The "Stop"
line is part of the release criteria.

## Phase 0: Repository And Release Discipline

### v0.1.0 - Repository Foundation

Goal: initialize the serious Rust workspace and policy baseline.

Deliverables:

- Rust stable `1.96.1` pinned.
- Rust `1.90.0` through `1.96.1` compatibility policy.
- Focused no_std workspace crate boundaries.
- CI, dependency policy, security policy, release notes, SBOM, and pentest
  report flow.
- Implementation, release, scope, threat-model, modularity, toolchain, unsafe,
  spec, supply-chain, fuzzing, and security-control docs.
- First-party crate boundaries for primitives, CBOR, crypto, address, ledger,
  script, governance, node, RPC, sanitization, signer, testkit, and facade.

Verification:

- `scripts/checks.sh`
- `scripts/check_latest_tools.sh`
- `scripts/release_0_1_gate.sh`
- `cargo deny check`
- `cargo audit`
- `scripts/generate-sbom.sh`
- `scripts/validate-release-readiness.sh v0.1.0`

Exit criteria:

- A new contributor can understand the scope, security posture, and release
  process from the repository docs.
- The release tag contains a permanent pentest report that reviews the exact
  implementation commit.

Stop:

```text
v0.1.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.2.0 - Release Readiness Gate

Goal: make the pentest-before-tag process, spec-lock checks, and crates.io
publish order enforceable by local tooling.

Deliverables:

- Release-readiness validation hardened for `cardano` artifacts.
- Support-crate publish order validation.
- Release-note metadata checks.
- Permanent pentest-report metadata checks.
- SBOM presence checks.
- Spec-source policy checks.
- Tag-exists guard.
- Negative tests for missing reports, bad tags, missing SBOM, and mixed report
  commits.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_2_gate.sh`
- `scripts/release_crates.py --check`
- `scripts/test-release-readiness.sh`
- `scripts/validate-spec-lock-policy.sh check`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- The project can refuse a tag-ready claim when pentest or release evidence is
  missing.
- Future protocol milestones have an explicit source-check workflow.

Stop:

```text
v0.2.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 1: Primitive, Error, And Resource Foundation

### v0.3.0 - Domain Newtypes

Goal: make Cardano numeric, byte, and identity domains explicit.

Deliverables:

- Network id, slot, epoch, block number, coin, transaction id, block hash,
  script hash, datum hash, policy id, asset name, credential, and era
  primitives.
- Bounded constructors where protocol limits exist.
- Constant-time equality policy for secret-adjacent fixed-width values only
  after review.
- No CBOR, address, ledger, script, node, signer, or query dependency in the
  primitive crate.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_3_gate.sh`
- constructor and conversion tests for every primitive;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Public APIs no longer use unqualified integers or byte arrays for core
  Cardano protocol concepts.

Stop:

```text
v0.3.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.4.0 - Stable Error Model

Goal: establish non-panicking error categories for protocol operations.

Deliverables:

- Error categories for codec, address, ledger, script, governance, feature,
  resource, source-lock, and verification failures.
- Stable error codes and messages for public APIs.
- No secret-bearing error payloads.
- Tests for formatting, category stability, and non-panicking malformed-input
  paths.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_4_gate.sh`
- error stability tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Unsupported or malformed protocol data returns structured errors, not panics.

Stop:

```text
v0.4.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.5.0 - Decode Budget Model

Goal: define resource limits for all untrusted byte decoding.

Deliverables:

- Shared decode-budget type for input bytes, nesting depth, item count, map
  entry count, allocation, and total decoded value count.
- Stateful accounting for cumulative parser work.
- Fail-closed errors for budget exhaustion.
- Tests for every budget branch and boundary value.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_5_gate.sh`
- resource-exhaustion tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- No untrusted parser can be added without accepting an explicit budget.

Stop:

```text
v0.5.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 2: CBOR And CDDL

### v0.6.0 - CBOR Scalar Decoder

Goal: implement bounded CBOR scalar decoding.

Deliverables:

- Major-type handling for unsigned integers, negative integers, byte strings,
  text strings, booleans, null/simple values, and tags as explicitly admitted.
- Exact-consumption helpers.
- Noncanonical integer and length rejection policy.
- Official CDDL/source revisions pinned before behavior is claimed.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_6_gate.sh`
- official and adversarial scalar fixtures;
- malformed-input no-panic tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Scalar CBOR bytes are decoded with exact consumption and bounded resource
  accounting.

Stop:

```text
v0.6.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.7.0 - CBOR Array And Map Decoder

Goal: implement bounded CBOR arrays and maps.

Deliverables:

- Definite-length array and map decoding.
- Explicit policy for indefinite-length values.
- Nesting-depth and item-count accounting.
- Duplicate-key handling policy for admitted CDDL contexts.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_7_gate.sh`
- nested array/map tests;
- duplicate-key and depth-limit negative tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Nested Cardano CBOR structures can be decoded without stack or allocation
  blowups.

Stop:

```text
v0.7.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.8.0 - Canonical CBOR Policy

Goal: make canonical CBOR rules explicit and testable.

Deliverables:

- Canonical integer-width, byte/text length, map ordering, and definite-length
  policy for Cardano-relevant data.
- Policy type that differentiates "wire decode" from "canonical ledger object"
  where needed.
- Negative tests for noncanonical forms.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_8_gate.sh`
- canonical and noncanonical vector tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Public APIs cannot accidentally claim canonicality from a permissive decode.

Stop:

```text
v0.8.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.9.0 - CDDL Fixture Import

Goal: add a reproducible official CDDL fixture workflow.

Deliverables:

- Pinned ledger/CDDL source revisions in `spec-lock.toml`.
- External reference-store layout for CDDL files and generated fixtures.
- Fixture importer that records only required test material.
- Documentation for updating fixture revisions.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_9_gate.sh`
- fixture-import dry run;
- `scripts/validate-spec-lock-policy.sh check`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- CDDL-sensitive work has a repeatable source-to-test evidence path.

Stop:

```text
v0.9.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.10.0 - CBOR Encoding Round Trips

Goal: add canonical CBOR encoding for admitted decoded domains.

Deliverables:

- No-allocation encoding helpers where practical.
- Encode/decode round-trip tests for all admitted CBOR domains.
- Buffer-size failure tests.
- Clear distinction between syntactic encoding and ledger-valid encoding.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_10_gate.sh`
- round-trip and buffer-limit tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Admitted CBOR values can be encoded canonically without hidden allocation
  requirements.

Stop:

```text
v0.10.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.11.0 - CBOR Fuzz Harness

Goal: continuously fuzz every CBOR parser.

Deliverables:

- cargo-fuzz workspace with parser-specific targets.
- Committed hex seed corpus for scalar, array/map, canonicality, and budget
  branches.
- Crash reproduction documentation.
- CI check that fuzz targets build.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_11_gate.sh`
- `cargo check --manifest-path fuzz/Cargo.toml`
- seed materializer check;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- No untrusted CBOR parser ships without a corresponding fuzz target and seed
  update process.

Stop:

```text
v0.11.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 3: Addresses And Credentials

### v0.12.0 - Shelley Address Decode

Goal: decode Shelley-era address shapes with explicit network and credential
domains.

Deliverables:

- Base, enterprise, pointer, reward, and stake address byte-shape decoding.
- Network-id extraction and mismatch errors.
- Credential type modeling.
- Bech32 admission decision and dependency review if needed.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_12_gate.sh`
- official and adversarial Shelley address fixtures;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Shelley address bytes decode into explicit unvalidated address domains.

Stop:

```text
v0.12.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.13.0 - Byron Address Decode

Goal: decode Byron address data without mixing it into Shelley assumptions.

Deliverables:

- Byron address envelope and payload boundary.
- Byron-specific network and attribute handling.
- Clear unsupported-field errors for deferred behavior.
- Source revisions pinned for Byron address behavior.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_13_gate.sh`
- Byron positive and negative fixtures;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Byron address support is explicit and cannot be mistaken for Shelley address
  validation.

Stop:

```text
v0.13.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.14.0 - Address Encoding

Goal: encode admitted Cardano address domains.

Deliverables:

- Shelley address byte and text encoding for admitted variants.
- Byron encoding policy for admitted decoded domains.
- Round-trip tests and malformed text rejection.
- Redaction policy for address-related diagnostics.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_14_gate.sh`
- encode/decode round-trip tests;
- official text-vector tests where available;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Address encoding is deterministic, network-aware, and separated by era.

Stop:

```text
v0.14.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.15.0 - Credential And Reward Account Domains

Goal: model payment, stake, script, and reward-account credentials.

Deliverables:

- Credential domain newtypes and validation states.
- Reward account decode/encode helpers.
- Network mismatch checks for reward accounts.
- Tests for credential length and type confusion failures.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_15_gate.sh`
- credential and reward-account tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Ledger, address, and script crates can share credential domains without ad
  hoc byte handling.

Stop:

```text
v0.15.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 4: Era-Aware Ledger Data

### v0.16.0 - Era And Protocol Parameter Context

Goal: introduce explicit era and protocol-parameter context before ledger data
validation.

Deliverables:

- Era identity and activation context.
- Protocol parameter snapshot domain.
- Unsupported-era and wrong-era errors.
- Spec-lock enforcement for ledger-source revisions.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_16_gate.sh`
- era/context ordering and mismatch tests;
- `scripts/validate-spec-lock-policy.sh check`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- No ledger API infers "latest" rules globally.

Stop:

```text
v0.16.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.17.0 - Transaction Body Shell

Goal: decode a versioned transaction body shell without claiming full ledger
validity.

Deliverables:

- Era-aware transaction body envelope.
- Borrowed field model for unvalidated transaction bodies.
- Unknown/deferred field policy per era.
- Tests for trailing data and wrong-era shapes.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_17_gate.sh`
- official CDDL transaction shell fixtures;
- transaction body fuzz target builds;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Transaction bodies can be classified and bounded without validating ledger
  rules.

Stop:

```text
v0.17.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.18.0 - Transaction Inputs, Outputs, Fees, And Validity Interval

Goal: decode and encode core transaction body accounting fields.

Deliverables:

- Inputs, outputs, collateral inputs, reference inputs, fees, mint, validity
  interval, and treasury/donation fields as era-appropriate.
- Value and output shape boundaries.
- Negative tests for missing required fields and duplicated fields.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_18_gate.sh`
- CDDL fixture tests;
- adversarial duplicate/missing-field tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Core transaction accounting fields have explicit decoded domains but remain
  unvalidated until validation milestones.

Stop:

```text
v0.18.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.19.0 - Witness Sets And Verification Boundary

Goal: decode witness sets and define the cryptographic verification boundary.

Deliverables:

- Key, script, bootstrap, Plutus, datum, and redeemer witness domains.
- Duplicate witness handling policy.
- `cardano-valkyoth-crypto` trait boundaries for caller-provided verification.
- No concrete crypto backend admitted without dependency review.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_19_gate.sh`
- witness fixture and duplicate tests;
- crypto-boundary tests with test doubles;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Witness data is decoded and verification is explicit, caller-provided, and
  not hidden behind defaults.

Stop:

```text
v0.19.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.20.0 - Metadata And Auxiliary Data

Goal: decode auxiliary data and transaction metadata safely.

Deliverables:

- Auxiliary data shell by era.
- Metadata map/list/scalar domains.
- Size and nesting limits for arbitrary metadata.
- Redaction and diagnostic policy for metadata parse errors.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_20_gate.sh`
- metadata fixture tests;
- metadata fuzz target builds;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Metadata handling is bounded and does not bypass CBOR resource policy.

Stop:

```text
v0.20.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.21.0 - Multi-Asset Values

Goal: model Mary-and-later multi-asset values.

Deliverables:

- Policy id, asset name, quantity, and value map domains.
- Canonical ordering and duplicate asset rejection.
- Mint/burn shape policy separate from ledger validity.
- Tests for overflow, negative quantities where relevant, and ordering.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_21_gate.sh`
- multi-asset fixture tests;
- arithmetic and ordering tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Multi-asset values are decoded and encoded with deterministic canonical
  policy and checked arithmetic.

Stop:

```text
v0.21.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.22.0 - Blocks And Headers

Goal: decode Cardano block and header boundaries for admitted eras.

Deliverables:

- Block header, body, transaction list, operational certificate, and protocol
  version domains as scoped by pinned sources.
- Header hash boundary.
- Slot/era context checks.
- Tests for block size, transaction count, and wrong-era data.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_22_gate.sh`
- official block/header fixtures;
- block/header fuzz target builds;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Blocks and headers can be parsed and bounded without implying chain
  selection or consensus validation.

Stop:

```text
v0.22.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 5: Ledger Validation Typestates

### v0.23.0 - UTxO Validation Typestates

Goal: introduce proof-gated ledger validation states.

Deliverables:

- Decoded, canonical, era-contextual, UTxO-checked, and fully-validated
  typestate tokens.
- Explicit validation context with UTxO set, protocol parameters, slot, era,
  and network.
- No validation transition without evidence input.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_23_gate.sh`
- typestate compile-fail or API tests;
- positive and negative UTxO fixtures;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Public APIs cannot confuse decoded transaction data with ledger-valid
  transaction data.

Stop:

```text
v0.23.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.24.0 - Fee And Value Conservation Checks

Goal: implement bounded fee and value conservation checks.

Deliverables:

- Fee minimum boundary with protocol parameters.
- Input/output/mint/burn/deposit/refund accounting.
- Checked arithmetic for coin and multi-asset quantities.
- Tests for overflow, underflow, fee mismatch, and value mismatch.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_24_gate.sh`
- official and adversarial accounting fixtures;
- arithmetic boundary tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Value conservation failures are deterministic and fail closed.

Stop:

```text
v0.24.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.25.0 - Certificate And Delegation Data

Goal: decode and validate certificate and delegation data boundaries.

Deliverables:

- Stake registration, deregistration, delegation, pool, MIR where applicable,
  and era-specific certificate domains.
- Certificate ordering and duplicate policy where required.
- Validation-state transitions for certificate checks.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_25_gate.sh`
- certificate fixture tests;
- wrong-era and duplicate tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Certificate data is era-aware and does not bypass explicit validation
  context.

Stop:

```text
v0.25.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.26.0 - Withdrawal And Reward Accounting Boundaries

Goal: add withdrawal and reward-accounting validation helpers.

Deliverables:

- Withdrawal map domains.
- Reward account network and credential checks.
- Accounting helpers for withdrawals, deposits, and refunds.
- Negative tests for unknown accounts, duplicate entries, and value mismatch.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_26_gate.sh`
- withdrawal and reward-accounting fixtures;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Reward-related accounting is explicit and testable before governance and
  node integrations use it.

Stop:

```text
v0.26.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.27.0 - Era Transition Boundaries

Goal: model ledger behavior at era boundaries.

Deliverables:

- Era transition context and unsupported-transition errors.
- Explicit upgrade path for data whose shape changes across eras.
- Tests for wrong-era, pre-activation, and post-activation behavior.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_27_gate.sh`
- era transition fixture tests;
- `scripts/validate-spec-lock-policy.sh check`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Era transitions are caller-provided and evidence-backed, not inferred from a
  global latest rule.

Stop:

```text
v0.27.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 6: Scripts And Plutus Boundaries

### v0.28.0 - Native Script Decode And Validation

Goal: decode and validate native script structures.

Deliverables:

- Signature, all, any, at-least, before, and after native script domains.
- Script hash boundary.
- Slot-based validation context.
- Recursion and script-size limits.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_28_gate.sh`
- native script fixture tests;
- recursion and size-limit tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Native script validation is bounded, era-aware, and separated from Plutus
  execution.

Stop:

```text
v0.28.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.29.0 - Plutus Data Decode And Encoding

Goal: decode and encode Plutus data without admitting execution.

Deliverables:

- Constructor, map, list, integer, and bytes Plutus data domains.
- Data hash boundary.
- Budgeted decode and canonical encode helpers.
- Fuzz target for Plutus data parsing.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_29_gate.sh`
- official and adversarial Plutus data fixtures;
- `cargo check --manifest-path fuzz/Cargo.toml`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Plutus data can be parsed and hashed without executing scripts.

Stop:

```text
v0.29.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.30.0 - Script Integrity Hash Boundary

Goal: implement script integrity hash construction boundaries.

Deliverables:

- Script integrity input domains for redeemers, datums, language views, and
  cost models as scoped by pinned sources.
- Hash-construction trait boundary.
- Tests for wrong ordering, missing fields, and wrong-era behavior.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_30_gate.sh`
- official script integrity vectors;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Script integrity hashing is deterministic and cannot be confused with script
  execution validity.

Stop:

```text
v0.30.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.31.0 - Execution Unit Policy

Goal: model execution units and cost-model boundaries.

Deliverables:

- Execution unit domains and checked arithmetic.
- Cost model identifiers and era context.
- Fail-closed limits for oversized or unsupported execution budgets.
- Tests for overflow and unsupported language/cost-model combinations.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_31_gate.sh`
- execution-unit fixture tests;
- arithmetic boundary tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Execution resource accounting is explicit before any execution adapter is
  considered.

Stop:

```text
v0.31.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.32.0 - Optional Plutus Execution Adapter Decision

Goal: decide whether and how to admit an optional Plutus execution adapter.

Deliverables:

- Threat-model expansion for execution adapters.
- Dependency admission review for candidate execution backend.
- Explicit non-default feature plan.
- Process isolation, timeout, memory, and execution-unit policy.
- Decision record if execution remains deferred.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_32_gate.sh`
- dependency review evidence if admitted;
- adapter smoke tests or documented deferral;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- No Plutus execution dependency enters the workspace without a documented
  security and resource policy.

Stop:

```text
v0.32.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 7: Governance And Conway

### v0.33.0 - Conway Governance Data

Goal: decode Conway governance data structures.

Deliverables:

- Governance action, proposal procedure, voter, vote, DRep, committee, and
  constitution domains as scoped by pinned sources.
- Governance CDDL fixture tests.
- Wrong-era rejection.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_33_gate.sh`
- Conway governance fixtures;
- `scripts/validate-spec-lock-policy.sh check`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Conway governance data is decoded without claiming full governance ledger
  validity.

Stop:

```text
v0.33.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.34.0 - Votes, Proposals, And DRep Domains

Goal: model voting and proposal validation boundaries.

Deliverables:

- Vote and proposal domains with explicit governance action ids.
- DRep identity and credential handling.
- Duplicate vote/proposal policy.
- Tests for invalid voters, unsupported actions, and wrong-era data.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_34_gate.sh`
- vote/proposal fixtures;
- negative governance tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Vote and proposal helpers require explicit governance context.

Stop:

```text
v0.34.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.35.0 - Committee And Constitutional Committee Domains

Goal: model committee and constitutional committee boundaries.

Deliverables:

- Committee credential, threshold, term, and member domains.
- Constitutional committee update data.
- Validation helpers for committee shape and threshold policy.
- Tests for threshold, duplicate member, and expired member failures.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_35_gate.sh`
- committee fixture tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Committee data is explicit and cannot be validated without governance
  context.

Stop:

```text
v0.35.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.36.0 - CIP-1694 Validation Helpers

Goal: add reviewed CIP-1694 helper surfaces without overriding ledger sources.

Deliverables:

- CIP-1694 source revision pinned in `spec-lock.toml`.
- Helper APIs for governance workflows that remain subordinate to ledger
  validation.
- Tests showing disagreement handling when CIPs and ledger sources are
  ambiguous or changed.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_36_gate.sh`
- CIP fixture tests;
- spec ambiguity tests where applicable;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- CIP helpers are clearly application-facing and do not replace ledger source
  truth.

Stop:

```text
v0.36.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 8: Conformance And Differential Testing

### v0.37.0 - Ledger Fixture Harness

Goal: run pinned ledger fixtures through admitted decoded and validation
surfaces.

Deliverables:

- Fixture loader for external reference store.
- Test harness for transaction, block, ledger, script, and governance fixtures
  already admitted.
- Skip/unsupported accounting for unimplemented fixture families.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_37_gate.sh`
- fixture harness tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Claimed support areas have executable fixture evidence, and skipped areas are
  explicit.

Stop:

```text
v0.37.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.38.0 - CDDL Differential Test Harness

Goal: compare admitted CBOR/CDDL behavior against independent or generated
reference expectations.

Deliverables:

- Differential harness for CDDL-driven encode/decode fixtures.
- Mismatch reporting that redacts payloads where needed.
- Regression corpus for discovered differences.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_38_gate.sh`
- differential harness smoke tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- CDDL-related regressions can be reproduced and tracked by pinned fixture
  revision.

Stop:

```text
v0.38.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.39.0 - Ledger Differential Test Harness

Goal: compare validation outcomes with pinned Cardano ledger reference
behavior where practical.

Deliverables:

- Differential test adapter for accepted ledger fixture formats.
- Explicit unsupported-case reporting.
- Fail-closed mismatch classification.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_39_gate.sh`
- ledger differential smoke tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Ledger validation claims are backed by differential evidence, not only local
  unit tests.

Stop:

```text
v0.39.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.40.0 - Era Regression Corpus

Goal: maintain regression coverage across all admitted eras.

Deliverables:

- Curated regression corpus grouped by era and protocol area.
- Seed materialization for fuzz and fixture tests.
- Documentation for adding a regression after a bug or pentest finding.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_40_gate.sh`
- regression corpus tests;
- seed materializer checks;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Bugs found in admitted era behavior become permanent regression fixtures.

Stop:

```text
v0.40.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 9: Optional Node, Query, And Signer Boundaries

### v0.41.0 - Node Protocol Threat Model

Goal: expand the threat model before node protocol types or transports are
implemented.

Deliverables:

- Node-to-client and node-to-node trust boundaries.
- Peer/message panic-isolation decision.
- Resource, timeout, redaction, and backpressure policy.
- Clear decision on whether transports remain type-only or include optional
  runtime adapters.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_41_gate.sh`
- threat-model review checklist;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Node protocol work cannot start without documented untrusted-message policy.

Stop:

```text
v0.41.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.42.0 - Node-To-Client Type Boundary

Goal: add optional node-to-client protocol type boundaries.

Deliverables:

- Type definitions for admitted node-to-client messages.
- Bounded decode/encode policy for message payloads.
- No default transport dependency.
- Panic-isolation decision applied to any untrusted decode path.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_42_gate.sh`
- node-to-client fixture tests;
- protocol message fuzz target builds;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Node-to-client messages can be modeled without implying endpoint trust or
  full node behavior.

Stop:

```text
v0.42.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.43.0 - Node-To-Node Type Boundary

Goal: add optional node-to-node protocol type boundaries.

Deliverables:

- Type definitions for admitted node-to-node messages.
- Bounded decode/encode policy for peer-supplied data.
- Peer/resource error categories.
- No default transport dependency.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_43_gate.sh`
- node-to-node fixture tests;
- adversarial peer-message tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Node-to-node message support is explicit, optional, and does not imply a
  running node.

Stop:

```text
v0.43.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.44.0 - Query/Submit Transport Admission

Goal: admit optional query/submit transport dependencies only after review.

Deliverables:

- Dependency admission record for selected transport crates.
- Endpoint allowlist and no-implicit-public-endpoint policy.
- Timeout, retry, TLS, and credential handling policy.
- No automatic transaction resubmission.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_44_gate.sh`
- dependency review evidence;
- transport smoke tests behind non-default features;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Query/submit transport support is optional, reviewed, and not enabled by
  default.

Stop:

```text
v0.44.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.45.0 - Query Trust Models And Redaction

Goal: make query responses and logs explicit about trust and redaction.

Deliverables:

- Trusted, untrusted, quorum, and verified response models where applicable.
- Redaction for endpoint credentials, raw transactions, and secret-bearing
  query context.
- Malicious/stale response fixtures.
- Tests for no secret leakage in errors and debug output.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_45_gate.sh`
- malicious query fixture tests;
- redaction tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Query APIs cannot imply ledger truth without an explicit verification model.

Stop:

```text
v0.45.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.46.0 - Signer Interface

Goal: add external-signer-first APIs.

Deliverables:

- Transaction, certificate, metadata, and governance signing request domains.
- Domain-separated signing API.
- Secret-bearing request redaction through sanitization boundary.
- No local key storage.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_46_gate.sh`
- signing request shape tests;
- redaction tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Standard signing flows can be represented without raw digest-first or local
  key-default APIs.

Stop:

```text
v0.46.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.47.0 - Local Signer Fallback Decision

Goal: decide whether a local signer fallback belongs in the workspace.

Deliverables:

- Threat-model update for local key handling.
- Dependency review for key, derivation, zeroization, and randomness crates if
  admitted.
- Explicit non-default feature plan or documented deferral.
- Tests for zeroization/redaction if admitted.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_47_gate.sh`
- dependency review evidence or deferral record;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Local key storage cannot appear accidentally; it is either explicitly
  admitted with controls or deferred.

Stop:

```text
v0.47.0 implementation stop reached. Run pentest for this exact commit.
```

## Phase 10: Production Hardening

### v0.48.0 - Platform Matrix

Goal: verify supported platforms, targets, and feature combinations.

Deliverables:

- Supported target matrix.
- `no_std`, `std`, and optional feature compatibility tests.
- MSRV through pinned stable checks.
- Documentation of unsupported platforms and feature combinations.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_48_gate.sh`
- platform matrix checks;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Users can see which targets and feature combinations are supported before
  `1.0.0`.

Stop:

```text
v0.48.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.49.0 - Kani Formal Verification Harness

Goal: add selected formal verification harnesses as extra assurance.

Deliverables:

- Kani harnesses for selected arithmetic, parser-budget, canonicality, and
  typestate invariants.
- Documentation that Kani does not replace fuzzing, conformance tests, pentest,
  cargo-audit/cargo-deny, or independent security review.
- CI or local script for running the harnesses.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_49_gate.sh`
- Kani harness command;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- High-value invariants have formal checks where practical.

Stop:

```text
v0.49.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.50.0 - Public API Stability Pass

Goal: prepare public APIs for `1.0.0`.

Deliverables:

- API review for naming, error stability, typestate consistency, feature
  boundaries, and documentation.
- Deprecation or migration notes for any unstable pre-1.0 surfaces.
- Examples for major supported workflows.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_50_gate.sh`
- documentation tests;
- API review checklist;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Public APIs are ready for final audit and 1.0 stabilization.

Stop:

```text
v0.50.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.51.0 - Independent Audit Remediation

Goal: resolve independent audit findings before release evidence dry run.

Deliverables:

- Audit report intake.
- Fixes for all critical/high findings or documented accepted residual risk.
- Regression tests for fixed findings.
- Updated threat model and security controls.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_51_gate.sh`
- audit remediation tests;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- No unresolved critical or high audit findings remain before release dry run.

Stop:

```text
v0.51.0 implementation stop reached. Run pentest for this exact commit.
```

### v0.52.0 - Release Evidence Dry Run

Goal: dry-run the full 1.0 release evidence process.

Deliverables:

- Complete release notes.
- SBOM and provenance.
- Crate version matrix.
- Spec matrix.
- Pentest report.
- Signed tag rehearsal.
- Publish-order dry run.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_52_gate.sh`
- `scripts/release_crates.py --check`
- `scripts/validate-release-readiness.sh v0.52.0`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- The project can produce all 1.0 release evidence before the 1.0 stop point.

Stop:

```text
v0.52.0 implementation stop reached. Run pentest for this exact commit.
```

## v1.0.0 - Production Cardano Toolkit

Goal: ship the first serious production-ready Cardano crate.

Deliverables:

- Production-ready bounded CBOR/CDDL, address, ledger, script, governance, and
  verification surfaces for every claimed feature.
- Optional node, query, signer, Plutus execution, and local-signer surfaces only
  where explicitly admitted and documented.
- Complete spec matrix with pinned official source revisions.
- Complete conformance and differential evidence for every claimed feature.
- SBOM, provenance, signed release manifest, release notes, migration guidance,
  and permanent pentest report.

Verification:

- `scripts/checks.sh`
- `scripts/release_1_0_gate.sh`
- `scripts/release_crates.py --check`
- `scripts/validate-release-readiness.sh v1.0.0`
- `cargo deny check`
- `cargo audit`
- official conformance suites for every claimed feature;
- all supported Rust versions from `1.90.0` through `1.96.1`.

Exit criteria:

- No unresolved critical or high dependency, advisory, audit, or pentest
  findings.
- All supported feature combinations are documented.
- Public API stability review is complete.
- `v1.0.0` tag is signed and pushed only when explicitly requested.

Stop:

```text
v1.0.0 implementation stop reached. Run pentest for this exact commit.
```
