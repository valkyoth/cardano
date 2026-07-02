#!/usr/bin/env sh
set -eu

mkdir -p sbom
cargo sbom --output-format spdx_json_2_3 > sbom/cardano.spdx.json
test -s sbom/cardano.spdx.json
