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
- first-party implementations for core Cardano wire formats, ledger state
  transitions, validation rules, and deterministic local computations;
- third-party crates only as reviewed optional backends, references, or
  compatibility adapters unless a cryptographic primitive is explicitly
  accepted with a first-party boundary and replacement/audit plan;
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

Cardano upstream monitoring is also a maintenance requirement. When ledger,
Plutus, governance, node, or wallet-adjacent surfaces are active, planned
automation must check the latest official ledger, node, Ouroboros-network,
CIP, Plutus, and relevant backend/source revisions, then report whether a
maintenance release is needed for changed CDDL, era rules, protocol
parameters, governance rules, node protocol versions, cost models, Plutus
language versions, or fixtures. Live upstream checks are advisory inputs;
concrete release claims still depend on pinned revisions in `spec-lock.toml`.

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
| The roadmap stopped at decoded/typestate ledger helpers without full ledger state transition, block validity, rewards, staking, governance enactment, or fixture admission. | Added `v0.48.0` through `v0.62.0` for full ledger state, chain validity, Plutus integration, conformance admission, and hardening. |
| Query, submit, wallet, transaction builder, application-standard, and indexer behavior were deferred instead of versioned. | Added `v0.63.0` through `v0.78.0` so wallet/application surfaces are explicit, optional where sensitive, and tested before 1.0. |
| Full node, sync, mempool, chain index, and operations behavior were out of scope despite the full-crate direction. | Added `v0.86.0` through `v0.92.0` for networking, sync orchestration, mempool, observability, and block-production/validator decisions. |
| Plutus, Hydra, Mithril, consensus interop, and node mini-protocol compatibility need versioned boundaries before broad ecosystem support is claimed. | Added `v0.79.0` through `v0.85.0` for these protocol and ecosystem-adjacent tracks. |
| Formal verification evidence was not scheduled late enough relative to the broader 1.0 scope. | Moved Kani to `v0.94.0` as extra assurance after full ledger/node/application tracks, not as a replacement for fuzzing, conformance tests, pentest, or audit. |

## Milestone Stop Contract

Every milestone below inherits the same stop rule: after the verification
commands pass, stop before tagging and request pentest for the exact commit.
When a milestone does not repeat a dedicated `Stop:` block for readability,
the required stop text is still part of that milestone's release criteria.

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

## Phase 10: Full Ledger State And Chain Validity

This phase turns decoded and typestate-checked ledger data into complete
claimed Cardano ledger behavior. It is the point where the crate stops being
only a parser/toolkit and starts proving full ledger-state outcomes for the
eras it claims.

### v0.48.0 - Genesis And Network Configuration Import

Goal: construct reproducible network and genesis context from explicit inputs.

Deliverables:

- Byron/Shelley genesis domain models;
- protocol-parameter import and validation;
- network magic and network id consistency checks;
- initial UTxO, stake, delegation, pool, and reward-state boundaries;
- fixture harness for pinned genesis material.

Verification:

- genesis fixture tests for claimed networks;
- `scripts/release_0_48_gate.sh`
- `cargo deny check`

Exit criteria:

- A claimed network can start from explicit first-party genesis and protocol
  parameter data, not hidden node assumptions.

### v0.49.0 - Era-Specific Ledger Rule Matrix

Goal: make every claimed era rule explicit before broad validation expands.

Deliverables:

- Byron, Shelley, Allegra, Mary, Alonzo, Babbage, and Conway rule matrix;
- unsupported-rule and unsupported-era report;
- fixture mapping by era and rule family;
- release-blocking drift check for pinned ledger sources.

Verification:

- rule-matrix consistency test;
- `scripts/validate-spec-lock-policy.sh check`
- `cargo test -p cardano-valkyoth-ledger`.

Exit criteria:

- No ledger validation helper can silently use a rule from the wrong era.

### v0.50.0 - Full Transaction Semantic Validity

Goal: validate complete transaction semantics for claimed eras.

Deliverables:

- input spending, collateral, reference input, output, mint, fee, deposit,
  refund, withdrawal, certificate, metadata, validity interval, and required
  signer rules;
- checked arithmetic and value-conservation integration;
- protocol-parameter and era activation context;
- official transaction fixture admission for claimed eras.

Verification:

- transaction validity fixtures;
- adversarial accounting and era-mismatch tests;
- `cargo test -p cardano-valkyoth-ledger`.

Exit criteria:

- Decoded transactions are not treated as valid until all claimed ledger
  semantic checks pass.

### v0.51.0 - Witness And Signature Validation Integration

Goal: bind witness verification to transaction validity without choosing a
default cryptographic backend.

Deliverables:

- required signer and credential matching;
- key, bootstrap, native script, Plutus, and governance witness integration;
- caller-provided crypto backend conformance checklist;
- duplicate, missing, and wrong-domain witness tests.

Verification:

- witness validation fixtures;
- crypto test-double coverage;
- `cargo deny check`.

Exit criteria:

- Witness validation is complete for claimed eras and cannot hide a concrete
  signing or hashing implementation.

### v0.52.0 - Block Header And Block Body Validity

Goal: validate block-level constraints for claimed eras.

Deliverables:

- header body, operational certificate, protocol version, issuer, and VRF
  boundary models;
- block size, transaction count, body hash, and slot/era checks;
- block body transaction validity integration;
- official block fixture coverage.

Verification:

- block/header fixture tests;
- malformed body hash and era tests;
- `cargo test -p cardano-valkyoth-ledger`.

Exit criteria:

- Blocks can be validated against explicit ledger and era context for claimed
  support, not only decoded.

### v0.53.0 - UTxO State Transition Integration

Goal: apply valid transactions to UTxO state deterministically.

Deliverables:

- UTxO add/remove/update model;
- multi-asset mint/burn integration;
- collateral and failed-script handling where applicable;
- state snapshot input and output types;
- replayable state-transition fixtures.

Verification:

- official state-transition fixtures where available;
- adversarial double-spend and rollback tests;
- `cargo test -p cardano-valkyoth-ledger`.

Exit criteria:

- Claimed transactions can be applied to explicit state and produce
  deterministic post-state.

### v0.54.0 - Staking, Rewards, And Pool State Transitions

Goal: validate and apply staking-related state transitions.

Deliverables:

- stake registration, deregistration, delegation, pool registration/update,
  retirement, withdrawal, deposit, and refund transitions;
- reward-account and pool-state models;
- epoch-boundary reward accounting boundary;
- fixture and adversarial coverage.

Verification:

- staking and reward fixtures;
- duplicate/expired/unknown credential tests;
- `cargo test -p cardano-valkyoth-ledger`.

Exit criteria:

- Staking and reward behavior is explicit and cannot be approximated by
  transaction-local accounting.

### v0.55.0 - Governance Enactment State Transitions

Goal: apply Conway governance outcomes to ledger state.

Deliverables:

- proposal lifecycle, voting thresholds, DRep state, committee state, treasury,
  constitution, and parameter-update enactment;
- era and protocol-parameter guards;
- CIP/ledger disagreement handling;
- governance fixture coverage.

Verification:

- Conway governance fixtures;
- threshold and enactment negative tests;
- `cargo test -p cardano-valkyoth-governance -p cardano-valkyoth-ledger`.

Exit criteria:

- Governance helpers can produce deterministic state changes only when the
  ledger evidence supports them.

### v0.56.0 - Plutus Validation Integration

Goal: integrate Plutus script validation with ledger semantics only after the
execution/resource boundary is admitted.

Deliverables:

- language version, cost model, datum, redeemer, reference script, collateral,
  and execution-unit integration;
- script integrity hash validation;
- optional execution adapter conformance checks;
- fail-closed behavior when execution is unavailable or unsupported.

Verification:

- Plutus validation fixtures for claimed language versions;
- malformed datum/redeemer and budget tests;
- dependency review for any execution backend.

Exit criteria:

- Plutus-dependent transactions cannot be claimed valid without explicit,
  versioned execution evidence.

### v0.57.0 - Historical Era Compatibility Matrix

Goal: ensure historical era behavior is deliberate, not collapsed into Conway.

Deliverables:

- Byron, Shelley, Allegra, Mary, Alonzo, Babbage, and Conway compatibility
  matrix;
- per-era supported and unsupported rule list;
- fixture coverage by era;
- migration notes for behavior that remains out of scope.

Verification:

- matrix consistency tests;
- fixture selection audit;
- `scripts/validate-spec-lock-policy.sh check`.

Exit criteria:

- Users can see exactly which eras and rule families are implemented.

### v0.58.0 - Full Ledger Fixture Admission

Goal: claim ledger support only where pinned official or accepted fixture
families pass.

Deliverables:

- transaction, block, certificate, staking, script, governance, and era
  fixture admission list;
- unsupported-fixture report with reasons;
- conformance report generated by local scripts;
- release-blocking fixture drift check.

Verification:

- full ledger fixture command documented and passing for claimed support;
- differential report against accepted ledger reference behavior.

Exit criteria:

- Ledger support claims are backed by executable conformance evidence.

### v0.59.0 - Ledger State Snapshot And Serialization

Goal: persist and restore explicit ledger state without trusting a node.

Deliverables:

- snapshot schema for admitted state components;
- versioned snapshot metadata and spec-lock provenance;
- bounded decode/encode of snapshots;
- compatibility tests for schema evolution.

Verification:

- snapshot round-trip and malformed-snapshot tests;
- `cargo test -p cardano-valkyoth-ledger`.

Exit criteria:

- Long-running users can checkpoint state while preserving release and source
  provenance.

### v0.60.0 - Rollback And Fork Handling

Goal: model rollback and fork-choice inputs for library users.

Deliverables:

- rollback-safe state transition records;
- chain fragment and point domains;
- explicit intersection and rollback limit policy;
- no implicit consensus claim beyond supplied evidence.

Verification:

- rollback and fork-choice unit tests;
- adversarial stale/unknown point tests.

Exit criteria:

- Consumers can handle chain rollbacks without ad hoc state mutation.

### v0.61.0 - Ledger Performance And DoS Hardening

Goal: harden full ledger validation against hostile inputs and large states.

Deliverables:

- worst-case validation cost review;
- state-size and transaction-size stress tests;
- budget and allocation caps for full validation paths;
- fuzz targets for high-risk ledger inputs.

Verification:

- DoS/load test commands;
- `cargo check --manifest-path fuzz/Cargo.toml`
- `cargo deny check`.

Exit criteria:

- Full ledger validation has bounded resource behavior documented for claimed
  features.

### v0.62.0 - Full Ledger Audit Hardening

Goal: prepare full ledger behavior for broader integration and audit.

Deliverables:

- ledger hardening report;
- regression corpus from fixture mismatches and pentest findings;
- unsafe/dependency review;
- Kani candidate list for arithmetic and typestate invariants.

Verification:

- `scripts/checks.sh`
- ledger-specific hardening report.

Exit criteria:

- Full ledger behavior is ready to be consumed by wallet, node, and query
  tracks.

## Phase 11: Query, Submit, Wallet, And Signer Surfaces

### v0.63.0 - Query/Submit Dependency Admission

Goal: admit provider/transport crates behind `cardano-valkyoth-rpc` only after
review.

Deliverables:

- dependency, license, feature, MSRV, and maintenance review;
- endpoint policy types;
- localhost-only local node fixture plan;
- timeout and response-size limits;
- no hardcoded public endpoints.

Verification:

- `cargo check --workspace --all-features`
- `cargo deny check`
- local query fixture smoke test if admitted.

Exit criteria:

- Query/submit support remains optional and policy-first.

### v0.64.0 - Local Cardano Node Fixture

Goal: provide a reproducible local node fixture for integration tests.

Deliverables:

- Podman-managed local Cardano node/testnet fixture or documented equivalent;
- pinned image name/version or digest;
- start, health-check, and teardown scripts;
- no persisted wallet/key material;
- no default mainnet connection.

Verification:

- local node smoke script starts, checks, and tears down the fixture.

Exit criteria:

- Integration tests do not depend on a developer's existing node.

### v0.65.0 - Query Trust Models

Goal: distinguish trusted, untrusted, quorum, and locally verified query data.

Deliverables:

- query trust model APIs;
- chain/genesis verification at connection setup;
- stale response and rollback handling;
- response size and batch limits.

Verification:

- malicious/stale query fixture tests.

Exit criteria:

- Transport trust cannot be confused with ledger truth.

### v0.66.0 - Submit Policy And Rebroadcast Controls

Goal: make transaction submission behavior explicit and non-surprising.

Deliverables:

- submit request and response domains;
- no automatic resubmission by default;
- manual rebroadcast policy where admitted;
- redaction for raw transaction bytes and endpoint credentials.

Verification:

- submit policy tests;
- redacted error/debug tests.

Exit criteria:

- Transaction submission cannot leak payloads or fan out unexpectedly.

### v0.67.0 - Transaction Builder

Goal: provide safe transaction construction over validated ledger domains.

Deliverables:

- input/output/value/fee/certificate/script/governance builder states;
- era and protocol-parameter context;
- minimum-ADA, fee-estimation, collateral, and validity-interval helpers;
- no signing or network submission side effects.

Verification:

- builder compile-time and runtime state tests;
- official transaction construction vectors where available.

Exit criteria:

- Users can build transactions without bypassing ledger validation gates.

### v0.68.0 - Wallet Domain Boundary

Goal: decide and implement the safe wallet-facing subset.

Deliverables:

- account, address discovery, coin-selection, change-output, and fee policy
  domains;
- no mnemonic or local key storage by default;
- explicit privacy and address-reuse caveats;
- hardware/external signer integration points.

Verification:

- coin-selection and change tests;
- redaction tests for wallet context.

Exit criteria:

- Wallet helpers are deterministic local policy tools, not a hidden wallet
  service.

### v0.69.0 - Local Key And Mnemonic Admission Decision

Goal: decide whether local key derivation and mnemonic support belong in the
workspace before 1.0.

Deliverables:

- BIP-39/CIP-1852 and hardware-wallet scope decision;
- dependency review for randomness, derivation, and zeroization crates if
  admitted;
- non-default feature plan or documented deferral;
- no-debug and sanitization requirements.

Verification:

- dependency review evidence or deferral record;
- secret redaction/zeroization tests if admitted.

Exit criteria:

- Local key material cannot enter default builds or undocumented APIs.

### v0.70.0 - Wallet And Signer Integration Fuzzing

Goal: fuzz wallet, builder, signer, and submit boundaries before they stabilize.

Deliverables:

- transaction builder fuzz targets;
- address/coin-selection edge corpus;
- signer request redaction corpus;
- submit response malformed-input corpus.

Verification:

- `cargo check --manifest-path fuzz/Cargo.toml`

Exit criteria:

- Wallet-adjacent parsers and builders have adversarial coverage.

## Phase 12: Application Standards And Ecosystem Helpers

### v0.71.0 - Token And Asset Helper APIs

Goal: add typed helpers for common native-token and NFT workflows.

Deliverables:

- asset id, policy id, asset name, quantity, fingerprint, and display helper
  APIs;
- CIP-14 fingerprint support;
- mint/burn helper boundaries;
- no indexer or market-data assumptions.

Verification:

- CIP vector tests;
- docs examples compile.

Exit criteria:

- Common asset operations are typed deterministic helpers over ledger domains.

### v0.72.0 - Metadata Standard Helpers

Goal: support common Cardano metadata standards without trusting off-chain
content.

Deliverables:

- CIP-25 NFT metadata helpers;
- CIP-68 reference NFT helper boundaries;
- metadata size, string, and URL redaction policy;
- no HTTP fetching by default.

Verification:

- metadata standard fixture tests;
- malformed and oversized metadata tests.

Exit criteria:

- Application metadata can be decoded and built without implying remote content
  trust.

### v0.73.0 - DRep And Governance Workflow Helpers

Goal: provide governance workflow helpers above the ledger primitives.

Deliverables:

- DRep registration/update/retirement helpers;
- vote and proposal construction helpers;
- governance metadata and anchor validation policy;
- no remote metadata trust by default.

Verification:

- CIP-1694 workflow tests;
- docs examples compile.

Exit criteria:

- Governance applications can build valid workflows while ledger truth remains
  pinned to official sources.

### v0.74.0 - Stake Pool And Delegation Helpers

Goal: expose stake pool and delegation convenience APIs.

Deliverables:

- pool registration/update/retirement builders;
- delegation certificate helpers;
- pool metadata URL/hash policy;
- reward account helper APIs.

Verification:

- stake pool and delegation vector tests.

Exit criteria:

- Staking helpers are typed local builders, not remote pool discovery clients.

### v0.75.0 - Identity And DID/Credential Boundary Decision

Goal: decide which identity/CIP credential helpers belong in the crate.

Deliverables:

- supported CIP list or explicit deferral;
- credential format parsing boundaries;
- privacy and correlation caveats;
- no network resolver by default.

Verification:

- decision document and security review.

Exit criteria:

- Identity support is either versioned or excluded with rationale.

### v0.76.0 - Application Data Builders

Goal: provide safe builders for common on-chain application payloads.

Deliverables:

- metadata, datum, redeemer, and script-reference builders;
- bounded output and canonical encoding;
- application-facing errors and redaction policy.

Verification:

- round-trip and malformed-builder tests.

Exit criteria:

- Application payloads can be constructed without ad hoc CBOR downstream.

### v0.77.0 - Indexer Data Model Boundary

Goal: model indexer-facing data without shipping a database or crawler by
default.

Deliverables:

- block, transaction, UTxO, metadata, governance, and event projection models;
- rollback-aware event stream domains;
- schema evolution and provenance metadata;
- no default database dependency.

Verification:

- projection and rollback tests.

Exit criteria:

- Indexers can use stable data domains while storage remains an adapter choice.

### v0.78.0 - Application Helper Fuzzing

Goal: fuzz metadata, application helper, and projection parsers.

Deliverables:

- metadata standard fuzz targets;
- datum/redeemer builder fuzz targets;
- projection decode fuzz targets;
- seed corpus for malformed application data.

Verification:

- `cargo check --manifest-path fuzz/Cargo.toml`

Exit criteria:

- Application-facing parsers have adversarial coverage before 1.0.

## Phase 13: Plutus, Mithril, Hydra, And Consensus Interop

### v0.79.0 - Plutus Language Version Matrix

Goal: make Plutus language and cost-model support explicit.

Deliverables:

- supported Plutus language version matrix;
- cost-model source revision evidence;
- unsupported-language and unsupported-builtin errors;
- fixture coverage by language version.

Verification:

- Plutus fixture tests;
- `scripts/validate-spec-lock-policy.sh check`.

Exit criteria:

- Plutus support claims are versioned and cannot collapse language versions.

### v0.80.0 - Plutus Execution Backend Admission

Goal: admit or defer concrete Plutus execution backends with explicit policy.

Deliverables:

- backend dependency review;
- timeout, memory, CPU, and process-isolation policy;
- execution-unit conformance checklist;
- compatibility report against accepted fixtures.

Verification:

- backend smoke tests or documented deferral;
- `cargo deny check`.

Exit criteria:

- Plutus execution cannot be hidden behind default features or unbounded local
  execution.

### v0.81.0 - Mithril Certificate Boundary

Goal: model Mithril certificate and snapshot verification boundaries.

Deliverables:

- certificate, signer, stake distribution, and snapshot domains;
- cryptographic backend boundary;
- trust-anchor and source-policy documentation;
- no default network fetching.

Verification:

- Mithril fixture/vector tests where admitted.

Exit criteria:

- Mithril data can be represented and verified only under explicit trust
  anchors.

### v0.82.0 - Hydra Protocol Boundary

Goal: decide and model Hydra head protocol support.

Deliverables:

- Hydra scope decision;
- head state, snapshot, commit/decommit, and close/contest/fanout domains if
  admitted;
- trust and liveness caveats;
- no default transport dependency.

Verification:

- Hydra fixture tests or documented deferral.

Exit criteria:

- Hydra support is either versioned or explicitly excluded from 1.0.

### v0.83.0 - Ouroboros Consensus Evidence Boundary

Goal: expose consensus evidence helpers without implementing a full consensus
node by accident.

Deliverables:

- chain selection, leader, certificate, and header evidence models as scoped by
  official sources;
- explicit trusted checkpoint and rollback policy;
- unsupported consensus feature list.

Verification:

- consensus evidence fixture tests.

Exit criteria:

- Consensus-adjacent APIs cannot imply full node security without evidence.

### v0.84.0 - Node Mini-Protocol Compatibility

Goal: version compatibility for selected node mini-protocols.

Deliverables:

- chain-sync, block-fetch, tx-submission, local-state-query, and local-tx-submit
  version matrix;
- codec compatibility tests;
- backward/forward compatibility policy.

Verification:

- mini-protocol fixture tests;
- protocol fuzz targets build.

Exit criteria:

- Node protocol support has an explicit version matrix.

### v0.85.0 - Ecosystem Interop Fuzzing

Goal: fuzz Plutus, Mithril, Hydra, consensus, and mini-protocol parsers.

Deliverables:

- parser-specific fuzz targets;
- seed corpus for malformed certificates, snapshots, protocol messages, and
  Plutus inputs;
- crash reproduction docs.

Verification:

- `cargo check --manifest-path fuzz/Cargo.toml`

Exit criteria:

- Ecosystem-adjacent untrusted parsers have adversarial coverage.

## Phase 14: Full Node, Sync, And Operations Boundaries

### v0.86.0 - Full Node Scope Decision

Goal: decide exactly what full-node behavior belongs in this crate family.

Deliverables:

- full-node, library, adapter, and compatibility scope split;
- threat model for long-running networked operation;
- storage, mempool, sync, peer, and observability boundary decisions;
- default-off feature policy.

Verification:

- security review of the decision document.

Exit criteria:

- Full node behavior is either versioned or explicitly excluded before code
  lands.

### v0.87.0 - Networking Dependency Admission

Goal: admit networking dependencies behind optional crates only.

Deliverables:

- latest-version, license, feature, MSRV, and maintenance review;
- no default graph expansion;
- timeout, message-size, and backpressure policy;
- loopback-only test transport.

Verification:

- `cargo check --workspace --all-features`
- `cargo deny check`

Exit criteria:

- Networking dependencies are isolated from protocol-core users.

### v0.88.0 - Peer And Connection State Machines

Goal: model peer lifecycle and connection behavior as bounded state machines.

Deliverables:

- peer identity, handshake, version negotiation, timeout, and connection state
  domains;
- inbound/outbound policy;
- resource and backpressure limits;
- redacted diagnostics.

Verification:

- state-machine tests;
- malicious peer fixture tests.

Exit criteria:

- Peer management cannot imply trusted ledger or wallet behavior.

### v0.89.0 - Sync Orchestration

Goal: model chain sync, block fetch, rollback, and state handoff workflows.

Deliverables:

- chain-sync and block-fetch orchestration states;
- rollback-safe state handoff;
- progress/error observability hooks with redaction;
- cancellation and resource-limit tests.

Verification:

- sync state-machine tests;
- local node fixture smoke tests if transport is admitted.

Exit criteria:

- Sync orchestration cannot imply verified state without ledger and consensus
  evidence.

### v0.90.0 - Mempool And Tx Submission Policy

Goal: provide mempool and transaction submission policy helpers.

Deliverables:

- transaction admission and replacement policy;
- duplicate, stale, and invalid transaction handling;
- private/local transaction redaction policy;
- no automatic rebroadcast by default.

Verification:

- mempool policy tests;
- redaction tests.

Exit criteria:

- Mempool helpers are deterministic policy tools, not a hidden node service.

### v0.91.0 - Chain Index And Storage Adapter Boundary

Goal: support long-running applications without selecting a database by
default.

Deliverables:

- chain index event and query domains;
- storage adapter trait boundaries;
- rollback-aware persistence policy;
- schema/provenance metadata.

Verification:

- adapter test doubles;
- rollback persistence tests.

Exit criteria:

- Storage remains an adapter choice while indexer semantics are stable.

### v0.92.0 - Operations, Metrics, And Validator Boundary Decision

Goal: decide what operational and validator-adjacent support belongs before
1.0.

Deliverables:

- metrics, tracing, health, and redaction policy;
- block production, stake pool operator, and validator key custody decision;
- if implemented, split follow-up release plan before 1.0;
- no validator key material in default crates.

Verification:

- security review of the operations decision document.

Exit criteria:

- Operational support is versioned and key custody remains explicit.

## Phase 15: Production Hardening

### v0.93.0 - Platform Matrix

Goal: verify supported platforms, targets, and feature combinations.

Deliverables:

- Linux, Windows, BSD, macOS, Android, iOS, embedded, and no_std target notes;
- `no_std`, `std`, and optional feature compatibility tests;
- MSRV through pinned stable checks;
- documentation of unsupported platforms and feature combinations.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_93_gate.sh`
- platform matrix checks;
- `cargo deny check`
- `cargo audit`

Exit criteria:

- Platform support claims match tested evidence.

### v0.94.0 - Kani Formal Verification Harness

Goal: add selected formal verification harnesses as extra assurance.

Deliverables:

- Kani harness admission and install/update policy;
- proof harnesses for decode budgets, canonicality, checked arithmetic,
  asset ordering, and typestate transitions;
- documentation that Kani does not replace fuzzing, conformance tests, pentest,
  cargo-audit/cargo-deny, or independent security review.

Verification:

- Kani proof command documented and passing for admitted harnesses;
- `scripts/checks.sh`

Exit criteria:

- High-value invariants have bounded formal checks before API stability.

### v0.95.0 - Public API Stability Pass

Goal: stabilize the public API shape before `1.0.0`.

Deliverables:

- API stability policy update;
- deprecation policy;
- feature compatibility matrix;
- migration notes for all breaking changes;
- examples for major supported workflows.

Verification:

- docs and examples compile;
- API review checklist.

Exit criteria:

- The remaining 1.0 work is hardening, not API invention.

### v0.96.0 - Independent Audit Remediation

Goal: resolve independent audit findings before release evidence dry run.

Deliverables:

- audit report intake;
- fixes for all critical/high findings or documented accepted residual risk;
- regression tests for fixed findings;
- updated threat model and security controls.

Verification:

- `scripts/checks.sh`
- audit remediation review.

Exit criteria:

- No unresolved critical or high audit findings remain.

### v0.97.0 - Release Evidence Dry Run

Goal: dry-run the full 1.0 release evidence process.

Deliverables:

- signed release manifest draft;
- SBOM and provenance;
- crate version matrix;
- spec and conformance matrix;
- dependency compatibility matrix;
- pentest report;
- publish-order dry run.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_97_gate.sh`
- `scripts/release_crates.py --check`
- `scripts/validate-release-readiness.sh v0.97.0`
- `cargo deny check`
- `cargo audit`

Exit criteria:

- The project can produce all 1.0 release evidence before the 1.0 stop point.

## v1.0.0 - Production Cardano Toolkit

Goal: ship the first serious production-ready Cardano crate.

Deliverables:

- Production-ready bounded CBOR/CDDL, address, ledger, script, governance, and
  verification surfaces for every claimed feature.
- Optional node, query, signer, Plutus execution, and local-signer surfaces only
  where explicitly admitted and documented.
- Full ledger-state, wallet/application, node/sync, ecosystem interop, and
  operational surfaces are either supported with conformance evidence or
  explicitly excluded with rationale.
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
