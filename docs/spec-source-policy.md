# cardano Spec Source Policy

`cardano` must not implement consensus-sensitive Cardano behavior from memory.

Every milestone that changes Cardano protocol behavior must start by checking
the current official source material, pinning exact revisions, and recording
the evidence used by tests.

## Official Sources

Use these as primary sources:

- Ledger specs, CDDL, executable models, and implementation:
  `https://github.com/IntersectMBO/cardano-ledger`
- Node integration and implementation boundaries:
  `https://github.com/IntersectMBO/cardano-node`
- Node-to-client, node-to-node, and diffusion protocols:
  `https://github.com/IntersectMBO/ouroboros-network`
- Cardano Improvement Proposals:
  `https://github.com/cardano-foundation/CIPs`

Use CIPs as source material for accepted ecosystem behavior, but do not let a
CIP override the pinned ledger or node source for consensus-sensitive behavior.

## Required Workflow

Before implementing or changing consensus-sensitive behavior:

1. Check the current official source repositories.
2. Select the exact tags, releases, or commit hashes relevant to the milestone.
3. Record those revisions in `spec-lock.toml`.
4. Download or import only the required fixtures/spec files into the configured
   external reference store.
5. Add or update tests that use those pinned materials.
6. Update `docs/SPEC_MATRIX.md` with the claimed status and evidence.
7. State in release notes which spec and fixture revisions were used.

If the official sources disagree, are ambiguous, or have no fixture for the
behavior, stop and document the ambiguity before implementing. Do not silently
choose behavior based on memory, blog posts, or a single client implementation.

Pull requests that touch ledger, script, or governance implementation crates
must pin `ledger_rev`, `node_rev`, `ouroboros_network_rev`, and `cips_rev` to
40-character commit hashes with `spec_required = true`. The CI gate enforces
this path-sensitive policy through `scripts/validate-spec-lock-policy.sh`.

## Local Reference Store

External Cardano reference material belongs outside this repository. The
default local path is:

```text
../../test/cardano
```

from the repository root, which resolves to `/home/eldryoth/Work/test/cardano`
in the maintainer's current checkout layout. Other developers and CI may
override the location with `CARDANO_REFERENCE_STORE`.

This repository records revision metadata and test expectations, not large
upstream repositories unless a release explicitly requires vendored fixtures.

## Dependency And Tool Review

When a spec milestone requires third-party crates or tooling, review the latest
versions at the same time as the official Cardano sources. Dependency admission
still follows [Supply-Chain Security](supply-chain-security.md).
