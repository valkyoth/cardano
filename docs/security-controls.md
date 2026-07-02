# cardano Security Controls

- `#![forbid(unsafe_code)]` in first-party crates.
- `no_std` default crates.
- Bounded decoding before accepting untrusted Cardano bytes.
- Exact source revision pinning before consensus-sensitive behavior.
- Path-sensitive spec-lock CI checks before ledger, script, or governance
  implementation changes merge.
- Explicit panic-isolation decision before node or query binaries accept
  untrusted messages.
- Fuzz targets for every parser that accepts untrusted data.
- `cargo deny check` for license and advisory policy.
- `cargo audit` for RustSec advisories.
- SBOM generation at `sbom/cardano.spdx.json`.
- Pentest-before-tag release readiness checks.
- GitHub CodeQL default setup check before release.
