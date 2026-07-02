#!/usr/bin/env python3
"""Unit checks for the cardano release crate planner."""

from __future__ import annotations

import importlib.util
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "release_crates.py"

spec = importlib.util.spec_from_file_location("release_crates", SCRIPT)
assert spec is not None
assert spec.loader is not None
release_crates = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release_crates)


def assert_raises(message: str, fn, *args) -> None:
    try:
        fn(*args)
    except RuntimeError as exc:
        if message not in str(exc):
            raise AssertionError(f"expected {message!r}, got {exc!r}") from exc
        return
    raise AssertionError("expected RuntimeError")


def test_publish_order_contains_cardano_packages() -> None:
    assert release_crates.PUBLISH_ORDER == (
        "cardano-valkyoth-cbor",
        "cardano-valkyoth-primitives",
        "cardano-valkyoth-crypto",
        "cardano-valkyoth-address",
        "cardano-valkyoth-ledger",
        "cardano-valkyoth-script",
        "cardano-valkyoth-governance",
        "cardano-valkyoth-node",
        "cardano-valkyoth-rpc",
        "cardano-valkyoth-sanitization",
        "cardano-valkyoth-signer",
        "cardano-valkyoth-testkit",
        "cardano",
    )


def test_facade_code_change_tracks_release_version() -> None:
    entry = {
        "previous_version": "0.0.0",
        "version": "0.1.0",
        "change": "code",
        "publish": True,
        "reason": "initial facade",
    }
    release_crates.validate_plan_entry("cardano", entry, "0.1.0")


def test_support_code_change_uses_independent_minor_bump() -> None:
    entry = {
        "previous_version": "0.1.0",
        "version": "0.2.0",
        "change": "code",
        "publish": True,
        "reason": "new support crate feature",
    }
    release_crates.validate_plan_entry("cardano-valkyoth-cbor", entry, "0.7.0")


def test_invalid_support_code_change_fails() -> None:
    entry = {
        "previous_version": "0.1.0",
        "version": "0.7.0",
        "change": "code",
        "publish": True,
        "reason": "wrong lockstep bump",
    }
    assert_raises(
        "independent support-crate version",
        release_crates.validate_plan_entry,
        "cardano-valkyoth-cbor",
        entry,
        "0.7.0",
    )


def test_publish_plan_filters_unchanged_crates() -> None:
    plan = {
        "crates": {
            package: {
                "previous_version": "0.1.0",
                "version": "0.1.0",
                "change": "unchanged",
                "publish": False,
                "reason": "unchanged",
            }
            for package in release_crates.PUBLISH_ORDER
        }
    }
    plan["crates"]["cardano-valkyoth-cbor"] = {
        "previous_version": "0.1.0",
        "version": "0.2.0",
        "change": "code",
        "publish": True,
        "reason": "codec change",
    }
    assert release_crates.publish_plan(plan) == ("cardano-valkyoth-cbor",)


if __name__ == "__main__":
    test_publish_order_contains_cardano_packages()
    test_facade_code_change_tracks_release_version()
    test_support_code_change_uses_independent_minor_bump()
    test_invalid_support_code_change_fails()
    test_publish_plan_filters_unchanged_crates()
