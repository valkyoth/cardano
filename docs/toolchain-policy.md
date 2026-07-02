# cardano Toolchain Policy

`cardano` pins stable Rust `1.96.1` in `rust-toolchain.toml` and supports Rust
`1.90.0` through `1.96.1`.

Rules:

- Do not raise MSRV without a release-plan entry.
- Check all supported toolchains before a release.
- Document target-specific or future `no_std` exceptions before admission.
- Keep edition, resolver, and lint policy centralized in the workspace
  manifest.
