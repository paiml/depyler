#!/usr/bin/env python3
"""Freeze corpus manifest with SHA256 hashes.

DEPYLER-1360: Per Dr. Popper's recommendation - "Science requires reproducible
experiments. If your denominator changes, your metric is meaningless."

This script creates a versioned corpus manifest with:
- SHA256 hash for each file
- Timestamp and metadata
- Version tracking
"""

import hashlib
import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path


def sha256_file(filepath: str) -> str:
    """Compute SHA256 hash of a file."""
    sha256_hash = hashlib.sha256()
    with open(filepath, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            sha256_hash.update(chunk)
    return sha256_hash.hexdigest()


def classify_file(filepath: str) -> dict:
    """Classify a Python file and compute its hash."""
    return {
        "path": filepath,
        "sha256": sha256_file(filepath),
        "size_bytes": os.path.getsize(filepath),
    }


def load_existing_manifest(manifest_path: str) -> dict:
    """Load existing corpus manifest if it exists."""
    if os.path.exists(manifest_path):
        with open(manifest_path) as f:
            return json.load(f)
    return {}


def freeze_corpus(corpus_root: str, output_path: str, version: str = "1.0") -> dict:
    """Freeze corpus with SHA256 hashes."""
    corpus_path = Path(corpus_root)

    if not corpus_path.exists():
        print(f"Error: Corpus path does not exist: {corpus_root}", file=sys.stderr)
        sys.exit(1)

    # Find all Python files
    python_files = sorted(corpus_path.rglob("*.py"))

    # Build manifest
    manifest = {
        "version": version,
        "frozen_at": datetime.now(timezone.utc).isoformat(),
        "corpus_root": str(corpus_path.resolve()),
        "total_files": len(python_files),
        "manifest_hash": None,  # Will be computed after building
        "files": []
    }

    # Process each file
    for py_file in python_files:
        rel_path = str(py_file.relative_to(corpus_path))
        file_info = classify_file(str(py_file))
        file_info["relative_path"] = rel_path
        manifest["files"].append(file_info)

    # Compute manifest hash (hash of all file hashes for integrity)
    all_hashes = "".join(f["sha256"] for f in manifest["files"])
    manifest["manifest_hash"] = hashlib.sha256(all_hashes.encode()).hexdigest()

    # Write manifest
    with open(output_path, "w") as f:
        json.dump(manifest, f, indent=2)

    return manifest


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Freeze corpus manifest with SHA256 hashes (DEPYLER-1360)"
    )
    parser.add_argument(
        "--corpus",
        default="/home/noah/src/reprorusted-python-cli",
        help="Path to corpus root directory"
    )
    parser.add_argument(
        "--output",
        default="corpus_manifest_v1.json",
        help="Output manifest file path"
    )
    parser.add_argument(
        "--version",
        default="1.0",
        help="Manifest version"
    )

    args = parser.parse_args()

    print(f"Freezing corpus: {args.corpus}")
    print(f"Output: {args.output}")
    print(f"Version: {args.version}")
    print()

    manifest = freeze_corpus(args.corpus, args.output, args.version)

    print(f"Frozen {manifest['total_files']} files")
    print(f"Manifest hash: {manifest['manifest_hash'][:16]}...")
    print(f"Timestamp: {manifest['frozen_at']}")
    print()
    print(f"Written to: {args.output}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
