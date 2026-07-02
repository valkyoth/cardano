#!/usr/bin/env sh
set -eu

require_pinned_revisions() {
    grep -q 'spec_required = true' spec-lock.toml
    grep -Eq 'ledger_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'node_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'ouroboros_network_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'cips_rev = "[0-9a-f]{40}"' spec-lock.toml
}

check_consistency() {
    if grep -q 'spec_required = true' spec-lock.toml; then
        require_pinned_revisions
    fi
}

check_changed_paths() {
    base_ref="$1"
    changed_paths="$(git diff --name-only "$base_ref"...HEAD)"
    if printf '%s\n' "$changed_paths" | grep -Eq '^crates/cardano-valkyoth-(ledger|script|governance)/'; then
        require_pinned_revisions
    fi
}

mode="${1:-check}"
case "$mode" in
    check)
        check_consistency
        ;;
    check-changed)
        base_ref="${2:-}"
        if [ -z "$base_ref" ]; then
            echo "usage: scripts/validate-spec-lock-policy.sh check-changed <base-ref>" >&2
            exit 2
        fi
        check_consistency
        check_changed_paths "$base_ref"
        ;;
    *)
        echo "usage: scripts/validate-spec-lock-policy.sh check|check-changed <base-ref>" >&2
        exit 2
        ;;
esac
