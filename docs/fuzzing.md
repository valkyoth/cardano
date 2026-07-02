# Fuzzing

Status: harness directory only. Parser-specific fuzz targets are added when the
first untrusted Cardano byte parser lands.

The fuzz workspace lives under `fuzz/` and is intentionally separate from the
main workspace.

## Targets

No parser target is claimed in `v0.1.0`.

Planned targets:

- CBOR scalar decoding;
- CBOR array and map decoding;
- CDDL-shaped transaction decoding;
- address decoding;
- script and Plutus data decoding;
- governance payload decoding.

## Seed Corpus

Committed seeds will live in `fuzz/seed-corpus/<target>/*.hex`. They are hex
text so they can be reviewed in diffs.

Materialize committed seeds into `fuzz/corpus/` before running `cargo fuzz`:

```sh
scripts/materialize_fuzz_seeds.py
```

The release gate only requires that the fuzz workspace builds:

```sh
cargo check --manifest-path fuzz/Cargo.toml
```

Long-running fuzz campaigns are expected before parser-heavy releases, but they
are not hidden inside quick local release gates.
