#!/usr/bin/env sh
set -eu

spec_value() {
    key="$1"
    sed -n "s/^${key} = \"\\([^\"]*\\)\"$/\\1/p" spec-lock.toml
}

verify_revision_exists() {
    name="$1"
    repo_url="$2"
    rev="$3"
    scratch="$(mktemp -d)"
    trap 'rm -rf "$scratch"' EXIT HUP INT TERM

    git -C "$scratch" init -q
    if ! git -C "$scratch" fetch --depth=1 "$repo_url" "$rev" >/dev/null 2>&1; then
        echo "spec-lock.toml: ${name} revision ${rev} not found on ${repo_url}" >&2
        exit 1
    fi

    rm -rf "$scratch"
    trap - EXIT HUP INT TERM
}

require_pinned_revisions() {
    grep -q 'spec_required = true' spec-lock.toml
    grep -Eq 'ledger_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'node_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'ouroboros_network_rev = "[0-9a-f]{40}"' spec-lock.toml
    grep -Eq 'cips_rev = "[0-9a-f]{40}"' spec-lock.toml

    verify_revision_exists \
        "ledger_rev" \
        "$(spec_value ledger_repo)" \
        "$(spec_value ledger_rev)"
    verify_revision_exists \
        "node_rev" \
        "$(spec_value node_repo)" \
        "$(spec_value node_rev)"
    verify_revision_exists \
        "ouroboros_network_rev" \
        "$(spec_value ouroboros_network_repo)" \
        "$(spec_value ouroboros_network_rev)"
    verify_revision_exists \
        "cips_rev" \
        "$(spec_value cips_repo)" \
        "$(spec_value cips_rev)"
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
