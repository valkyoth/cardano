<p align="center">
  <b>no_std-first Cardano protocol building blocks for Rust.</b><br>
  Explicit domains, bounded CBOR/CDDL policy, era-aware validation boundaries, and security-gated release evidence.
</p>

<div align="center">
  <a href="https://docs.rs/cardano">Docs.rs</a>
  |
  <a href="docs/RELEASE_PLAN.md">Release Plan</a>
  |
  <a href="docs/threat-model.md">Threat Model</a>
  |
  <a href="SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/cardano">
    <img src="https://raw.githubusercontent.com/valkyoth/cardano/main/.github/images/cardano.webp" alt="cardano Rust crate overview">
  </a>
</p>

# cardano

`cardano` is a `no_std`-first Rust workspace for Cardano protocol building
blocks.

The project target is a production-ready Cardano crate at `1.0.0`, reached
through small releases with explicit security, conformance, and dependency
evidence. The first implementation work is intentionally conservative:
explicit domains, bounded CBOR/CDDL policy, stable crate boundaries, and
security documentation before node protocols, signing, local keys, Plutus
execution, or wallet-adjacent helpers become real dependencies.

## Current Status

Status: `v0.2.0` release-readiness tooling implementation stop reached;
waiting for pentest.

Implemented now:

- Rust workspace pinned to stable `1.96.1`.
- MSRV policy for Rust `1.90.0` through `1.96.1`.
- `no_std` facade and focused first-party crate boundaries.
- Cardano-specific implementation, release, scope, threat-model, modularity,
  toolchain, unsafe, spec-source, and supply-chain docs.
- Local check, release-gate, dependency-policy, SBOM, and pentest evidence
  tooling adapted from the same process model and reset for Cardano.
- Official-source workflow for `cardano-ledger`, `cardano-node`,
  `ouroboros-network`, and CIPs.
- Initial crate boundaries for primitives, CBOR, crypto, addresses, ledger,
  scripts, governance, node, RPC, sanitization, signer, testkit, and the
  `cardano` facade.
- Release-readiness tooling for release notes, SBOM, permanent pentest report
  metadata, reviewed-commit checks, tag-exists checks, and support-crate
  publish-order validation.

Not implemented yet:

- No protocol decoders.
- No ledger validation.
- No CBOR/CDDL conformance vectors.
- No address parser.
- No native script or Plutus data implementation.
- No transaction builder or signer.
- No local key storage.
- No node-to-client or node-to-node protocol support.
- No RPC/query/submit transport.
- No wallet, indexer, or full-node behavior.

## Trust Dashboard

| Area | Status |
| --- | --- |
| License | `MIT OR Apache-2.0` |
| MSRV | Rust `1.90.0` |
| Pinned toolchain | Rust `1.96.1` |
| Default target | `no_std` |
| Default runtime dependencies | first-party scaffold crates only |
| Unsafe policy | first-party crates use `#![forbid(unsafe_code)]` |
| Default features | protocol-core boundaries only |
| Network/signing defaults | none |
| Release evidence | local gates, cargo-deny, cargo-audit, SBOM, pentest report |
| Formal verification | Kani harness planned before `1.0.0` as extra assurance |
| Crate versions | tracked in [`docs/CRATE_VERSION_MATRIX.md`](docs/CRATE_VERSION_MATRIX.md) |
| 1.0 target | serious production-ready Cardano protocol toolkit |

## Install

```toml
[dependencies]
cardano = "0.1"
```

## Features

| Feature | Default | Purpose |
| --- | --- | --- |
| `std` | no | Enables `std` support in admitted core crates. |
| `node` | no | Future node-to-client and node-to-node protocol boundary. |
| `rpc` | no | Future query/submit trust-policy boundary. |
| `sanitization` | no | Future secret-bearing data sanitization bridge APIs. |
| `signer` | no | Future signer isolation boundary. |
| `testkit` | no | Test fixtures, conformance helpers, and adversarial inputs. |

Default builds do not enable networking, signing, local key storage, Plutus
execution, wallet behavior, or node operation.

## Source Discipline

Cardano protocol behavior must not be implemented from memory. Every milestone
that changes consensus-sensitive behavior starts by checking current official
source material, pinning exact revisions in `spec-lock.toml`, importing only
the required fixtures into the external reference store, and updating
[`docs/SPEC_MATRIX.md`](docs/SPEC_MATRIX.md).

Primary source families:

- `https://github.com/IntersectMBO/cardano-ledger`
- `https://github.com/IntersectMBO/cardano-node`
- `https://github.com/IntersectMBO/ouroboros-network`
- `https://github.com/cardano-foundation/CIPs`

## Release Discipline

Every version must stop before tagging until security review and pentest
evidence exist for the exact implementation commit. Tagging and publishing are
manual maintainer actions, not automatic side effects of implementation.

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```
