# cardano Scope

`cardano` targets Cardano protocol toolkit functionality with conservative
defaults and a full, versioned path to `1.0.0`.

## In Scope

- Bounded canonical decoding of Cardano CBOR and CDDL-shaped data.
- Era-aware transaction, block, certificate, script, metadata, and governance
  data models.
- Address, credential, policy id, asset id, network id, slot, epoch, and coin
  domains.
- UTxO and ledger validation typestates with explicit protocol parameters.
- Full ledger-state transition, transaction validity, block validity, staking,
  rewards, governance enactment, rollback, and fixture-backed conformance for
  every claimed era.
- Native script and Plutus data handling.
- Optional Plutus execution boundary with explicit resource policy.
- Optional node-to-client and node-to-node protocol boundaries.
- Optional query/submit transport policy with explicit trust models.
- Optional signer boundary with external-signer-first design.
- Transaction builder, wallet-facing policy helpers, application-standard
  helpers, and indexer-facing projection models where explicitly admitted.
- Plutus language/cost-model matrices, Mithril, Hydra, consensus evidence, and
  node mini-protocol compatibility boundaries where explicitly admitted.
- Full node, sync, mempool, chain index, operations, and validator-adjacent
  boundaries where explicitly versioned and backed by security review.
- Conformance evidence against pinned upstream test and spec revisions.

## Default-Off Or Decision-Gated Before 1.0

- Full node behavior until the `v0.86.0` through `v0.92.0` scope and security
  gates admit exact supported behavior.
- Wallet behavior until the `v0.68.0` through `v0.70.0` gates admit exact
  supported behavior.
- Mnemonic handling or local key storage unless explicitly admitted as
  non-default, reviewed functionality.
- Network transports in default builds.
- Implicit public endpoint fallback.
- Automatic transaction resubmission or rebroadcast fanout.
- Plutus execution in default builds.
- Hardcoded mainnet assumptions in public APIs.
- Marketing claims that `no_std` alone provides security.
