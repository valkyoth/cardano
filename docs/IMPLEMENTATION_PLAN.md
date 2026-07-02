# cardano Implementation Plan

Status: planning document

Crate name: `cardano`

1.0 target: a serious production-ready Cardano protocol toolkit for bounded
CBOR/CDDL decoding, era-aware ledger types, explicit UTxO and validation
boundaries, address and script handling, governance data, optional node
protocols, optional signer isolation, optional query/submit transports, and
conformance evidence against pinned official source revisions.

## Core Position

`cardano` is not a generic re-export of upstream Cardano crates and must not
hide networking, signing, Plutus execution, wallet behavior, or full-node
behavior behind defaults. It is a security-oriented workspace that gives
applications stable, testable, explicit boundaries around Cardano operations.

The first production value is:

- bounded canonical decoding of untrusted Cardano CBOR;
- CDDL-backed type admission;
- explicit network, era, slot, epoch, and protocol-parameter context;
- stable ledger, transaction, certificate, script, and governance domains;
- signer and key isolation;
- node/query trust models that do not imply ledger correctness;
- optional node protocol and Plutus execution boundaries with explicit trust
  and resource policies;
- conformance evidence against pinned upstream specification revisions.

## Non-Negotiable Engineering Rules

- Rust stable `1.96.1`, edition 2024, workspace resolver `3`.
- MSRV is Rust `1.90.0`; compatibility must be checked through `1.96.1`.
- Latest crate and tool versions are checked before dependency or tooling edits.
- Official Cardano sources are checked before consensus-sensitive
  implementation work; exact revisions are pinned in `spec-lock.toml`.
- Consensus-sensitive behavior is never implemented from memory.
- Core crates are `#![no_std]` and do not depend on network, filesystem, clock,
  TLS, async runtime, signer, node, wallet, or Plutus execution code.
- Main crate `cardano` is a facade over focused crates.
- Third-party crates require review, current-version checks, license checks,
  feature review, and tests before admission.
- First-party protocol-facing crates use `#![forbid(unsafe_code)]`.
- Normal `.rs` files must stay below 500 lines.
- Security documentation, release notes, and test evidence are release
  requirements, not cleanup work.

## Workspace Shape

- `cardano-valkyoth-primitives`: no_std network, slot, epoch, coin, hash,
  transaction id, policy id, credential, and era-neutral domain primitives.
- `cardano-valkyoth-cbor`: no_std bounded canonical CBOR and CDDL admission
  policy, exact consumption, decode budgets, and canonicality helpers.
- `cardano-valkyoth-crypto`: no_std trait boundaries for caller-provided
  hashing and signature verification.
- `cardano-valkyoth-address`: Shelley and Byron address parsing, network-id
  checks, stake credential handling, and encoding boundaries.
- `cardano-valkyoth-ledger`: era-aware transaction, block, UTxO, certificate,
  withdrawal, metadata, and validation-state domains.
- `cardano-valkyoth-script`: native scripts, Plutus data, script hashes,
  execution-unit policy, and optional Plutus execution adapter boundaries.
- `cardano-valkyoth-governance`: Conway governance actions, votes, DRep data,
  committee data, and CIP-1694 validation helpers.
- `cardano-valkyoth-node`: optional node-to-client and node-to-node protocol
  type and state-machine boundaries.
- `cardano-valkyoth-rpc`: optional query/submit policy over admitted transports.
- `cardano-valkyoth-sanitization`: optional redaction and sanitization boundary
  for secret-bearing Cardano data.
- `cardano-valkyoth-signer`: optional signer isolation and domain-specific
  signing APIs.
- `cardano-valkyoth-testkit`: fixtures, adversarial inputs, conformance helpers,
  and regression utilities.
- `cardano`: facade crate that re-exports stable admitted surfaces.

## Spec Source Discipline

Every protocol milestone begins with a source check against the official
Cardano repositories documented in [Spec Source Policy](spec-source-policy.md).
The milestone must pin exact upstream revisions in `spec-lock.toml`, import
only required fixtures or spec files into the configured external reference
store, and update [Spec Matrix](SPEC_MATRIX.md) before claiming support.

If a behavior is consensus-sensitive and no pinned source or fixture exists,
implementation stops until the ambiguity is documented and reviewed.

## Phases

1. Repository foundation and release discipline.
2. Primitive domain and error foundation.
3. Bounded CBOR/CDDL codec.
4. Address, credential, and network-id domains.
5. Era-aware transaction body, witness, metadata, and block models.
6. Ledger validation typestates and UTxO/accounting rules.
7. Native script, Plutus data, and optional Plutus execution boundary.
8. Governance and Conway-era validation helpers.
9. Conformance, fixture import, fuzzing, and differential test infrastructure.
10. Optional node/query/signer boundaries.
11. Production hardening, formal verification, audit remediation, and `1.0.0`.
