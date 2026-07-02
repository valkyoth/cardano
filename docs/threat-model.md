# cardano Threat Model

## Assets

- Secret keys, signing requests, and raw transaction bodies.
- Ledger state, UTxO data, certificates, governance data, and protocol
  parameters supplied by callers or nodes.
- Application trust decisions based on decoded Cardano data.
- Build, release, SBOM, and pentest evidence.

## Adversaries

- Malicious peers, nodes, relays, indexers, or submit endpoints.
- Attackers providing malformed CBOR, CDDL-shaped data, scripts, addresses, or
  governance payloads.
- Compromised or stale third-party dependencies.
- Integrators accidentally treating untrusted query results as verified ledger
  truth.

## Trust Boundaries

- Core crates accept untrusted bytes but do not perform network I/O.
- Query and node adapters are optional and must preserve explicit trust models.
- Signer APIs are optional and external-signer-first.
- Plutus execution, if admitted, is optional and resource-limited.

## Baseline Mitigations

- Core crates are `no_std`.
- First-party protocol-facing crates forbid unsafe code.
- Decode budgets and exact-consumption checks are required before parsers ship.
- Official source revisions are pinned before consensus-sensitive behavior.
- Future node and query adapters must isolate untrusted-input panic blast
  radius with an explicit per-binary unwind/catch policy or an isolated worker
  process boundary before message decode paths ship.
- Secrets and signed transactions are redacted by default in diagnostics.
- Every release requires local gates, SBOM, cargo-deny, cargo-audit, and
  pentest evidence before tags.

## Residual Risks

- `no_std` does not prevent logic bugs or dependency vulnerabilities.
- A Rust implementation can diverge from the Haskell reference if fixtures and
  differential tests are incomplete.
- Node/query responses can be stale, adversarial, or inconsistent unless the
  caller uses an explicitly verified trust model.
- Workspace-wide `panic = "abort"` is appropriate for current library crates,
  but future node or query binaries need a deliberate panic-isolation decision
  before accepting untrusted peer or endpoint messages.
