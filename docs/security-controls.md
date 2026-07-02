# cardano Security Controls

- `#![forbid(unsafe_code)]` in first-party crates.
- `no_std` default crates.
- Bounded decoding before accepting untrusted Cardano bytes.
- Exact source revision pinning before consensus-sensitive behavior.
- Fuzz targets for every parser that accepts untrusted data.
- `cargo deny check` for license and advisory policy.
- `cargo audit` for RustSec advisories.
- SBOM generation at `sbom/cardano.spdx.json`.
- Pentest-before-tag release readiness checks.
- GitHub CodeQL default setup check before release.
