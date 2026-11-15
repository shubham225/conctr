#!/usr/bin/env python3
"""Verify the immutable Rust UI source snapshot and reject untracked vendor files."""
from pathlib import Path
import hashlib
import json
root = Path(__file__).resolve().parents[1] / "src/rust-ui"
manifest = json.loads((root / "manifest.json").read_text())
expected = manifest["files"]
actual = {str(p.relative_to(root)) for p in root.rglob("*") if p.is_file() and str(p.relative_to(root)) not in ("mod.rs", "manifest.json")}
assert actual == set(expected), "Vendored file inventory changed"
for name, digest in expected.items():
    assert hashlib.sha256((root / name).read_bytes()).hexdigest() == digest, f"Modified upstream file: {name}"
print(f"Verified {len(expected)} unchanged upstream files at {manifest['revision']}")
