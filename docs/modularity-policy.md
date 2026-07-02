# cardano Modularity Policy

`cardano` must not become a monolithic source tree.

Rules:

- Main crate `cardano` is a facade, not the implementation home.
- CBOR, primitives, crypto, addresses, ledger, scripts, governance, node
  protocols, RPC/query, sanitization, signer, and tests live in separate
  crates.
- Core protocol crates must not depend on network, filesystem, clock, TLS,
  async runtime, signer, wallet, node transport, or Plutus execution code.
- Feature flags must not silently enable networking, signing, local key
  storage, wallet behavior, node behavior, or Plutus execution.
- Normal `.rs` files should stay below 500 lines.
