# cardano Scope

`cardano` targets Cardano protocol toolkit functionality with conservative
defaults.

## In Scope

- Bounded canonical decoding of Cardano CBOR and CDDL-shaped data.
- Era-aware transaction, block, certificate, script, metadata, and governance
  data models.
- Address, credential, policy id, asset id, network id, slot, epoch, and coin
  domains.
- UTxO and ledger validation typestates with explicit protocol parameters.
- Native script and Plutus data handling.
- Optional Plutus execution boundary with explicit resource policy.
- Optional node-to-client and node-to-node protocol boundaries.
- Optional query/submit transport policy with explicit trust models.
- Optional signer boundary with external-signer-first design.
- Conformance evidence against pinned upstream test and spec revisions.

## Default-Off Or Decision-Gated Before 1.0

- Full node behavior.
- Wallet behavior, mnemonic handling, or local key storage.
- Network transports in default builds.
- Implicit public endpoint fallback.
- Automatic transaction resubmission or rebroadcast fanout.
- Plutus execution in default builds.
- Hardcoded mainnet assumptions in public APIs.
- Marketing claims that `no_std` alone provides security.
