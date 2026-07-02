# cardano Specification Matrix

Status: repository foundation only. No Cardano protocol behavior is implemented
yet.

Official source and fixture revisions are governed by
[Spec Source Policy](spec-source-policy.md). `v0.1.0` records the source
families and workflow; later milestones must pin exact revisions in
`spec-lock.toml` before implementation.

| Area | Status | Evidence |
| --- | --- | --- |
| Cardano ledger specs | source family recorded | `cardano-ledger` repository recorded in `spec-lock.toml`; no revision pinned yet |
| CDDL and CBOR | planned | CBOR/CDDL codec milestone must pin CDDL source revisions before implementation |
| Address formats | planned | Address milestone must pin relevant ledger/CIP source revisions before implementation |
| Era model | planned | Era milestones must pin ledger formal specs and CDDL revisions before implementation |
| Transactions | planned | Transaction milestones must pin ledger spec, CDDL, and fixture revisions |
| Blocks | planned | Block milestones must pin ledger/node source revisions |
| Certificates and staking | planned | Ledger validation milestones must pin Shelley-era and later ledger sources |
| Native scripts | planned | Native script milestones must pin Allegra/Mary/Alonzo source material |
| Plutus data | planned | Plutus data milestones must pin Alonzo/Babbage/Conway source material |
| Plutus execution | optional planned | Execution adapter must remain non-default and have explicit resource policy |
| Multi-asset ledger data | planned | Mary and later source material must be pinned |
| Governance | planned | Conway and CIP-1694 source material must be pinned |
| Node-to-client protocols | optional planned | `ouroboros-network` and `cardano-node` revisions must be pinned |
| Node-to-node protocols | optional planned | `ouroboros-network` revisions must be pinned |
| Query/submit transport | optional planned | Trust model and redaction policy must be documented before admission |
| Sanitization | boundary planned | `cardano-valkyoth-sanitization` exists as a first-party secret-bearing data boundary |
| Signer APIs | optional planned | External-signer-first policy must be documented before admission |
| Fuzz harness | planned | Fuzz workspace is present; parser-specific targets are added with parser milestones |
| Formal verification | planned | Kani harness planned as extra assurance before `1.0.0` |

Every release that claims support for a Cardano era, CDDL type, ledger rule,
CIP, node protocol, or query method must update this matrix and `spec-lock.toml`.

Pull requests touching `cardano-valkyoth-ledger`, `cardano-valkyoth-script`, or
`cardano-valkyoth-governance` must set `spec_required = true` and pin all
official source revisions in `spec-lock.toml` before merging.
