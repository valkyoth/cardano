# cardano Specification Matrix

Status: `v0.3.0` primitive domain newtypes are implemented and waiting for
pentest.

Official source and fixture revisions are governed by
[Spec Source Policy](spec-source-policy.md). `v0.1.0` records the source
families and workflow; later milestones must pin exact revisions in
`spec-lock.toml` before implementation.

| Area | Status | Evidence |
| --- | --- | --- |
| Cardano ledger specs | pinned | `cardano-ledger` revision `7903c0074df4ecc2d96b780ca4e5299d2b866553` recorded in `spec-lock.toml` |
| Cardano node source | pinned | `cardano-node` revision `4fa3e6c4143df1f17aac9e114517e18f3b775934` recorded in `spec-lock.toml` |
| Ouroboros network source | pinned | `ouroboros-network` revision `b47dbc2c29108e593cc47524cffb75008b88fe90` recorded in `spec-lock.toml` |
| CIPs | pinned | `CIPs` revision `20c819b25abee6551a3ef51778b975e7463e1269` recorded in `spec-lock.toml` |
| Primitive domains | implemented | `cardano-valkyoth-primitives` implements network id, era, slot, epoch, block number, coin, hash/id, credential, policy id, and asset-name domains from pinned Conway CDDL evidence |
| CDDL and CBOR | planned | CBOR/CDDL codec milestone must pin CDDL source revisions before implementation |
| Address formats | planned | Address milestone must pin relevant ledger/CIP source revisions before implementation |
| Era model | primitive names only | `cardano-valkyoth-primitives::Era` names known ledger eras; era-specific ledger behavior remains planned |
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
