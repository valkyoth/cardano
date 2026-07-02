# cardano Unsafe Policy

First-party crates use:

```rust
#![forbid(unsafe_code)]
```

Unsafe code is not allowed in first-party protocol-facing crates.

If a future optional adapter genuinely requires unsafe code, it must be
isolated in a dedicated crate with:

- a documented reason unsafe is necessary;
- a `SAFETY:` explanation for every unsafe block;
- tests and fuzz coverage for the unsafe boundary;
- release-plan and threat-model updates before admission.
