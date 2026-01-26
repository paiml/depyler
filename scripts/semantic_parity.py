#!/usr/bin/env python3
"""Verify semantic parity between Python and transpiled Rust.

DEPYLER-1324: Per Dr. Popper's recommendation - "A compiled binary that produces
different output than the Python original is a false positive."

Formula: True Success Rate = Compile Rate × Semantic Parity

This script:
1. Finds Python files that have corresponding compiled Rust
2. Runs both versions with identical inputs
3. Compares outputs to verify semantic equivalence
"""

import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional


@dataclass
class SemanticResult:
    """Result of semantic parity check."""
    python_file: str
    rust_file: Optional[str]
    compiles: bool
    python_output: Optional[str]
    rust_output: Optional[str]
    outputs_match: bool
    error: Optional[str]


def run_python(filepath: str, timeout: int = 10) -> tuple[Optional[str], Optional[str]]:
    """Run Python file and capture output."""
    try:
        result = subprocess.run(
            ["python3", filepath],
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=os.path.dirname(filepath) or ".",
        )
        return result.stdout, result.stderr if result.returncode != 0 else None
    except subprocess.TimeoutExpired:
        return None, "Timeout"
    except Exception as e:
        return None, str(e)


def compile_rust(rs_file: str, output_binary: str, timeout: int = 60) -> tuple[bool, Optional[str]]:
    """Compile Rust file to binary."""
    try:
        result = subprocess.run(
            ["rustc", rs_file, "-o", output_binary, "--edition", "2021"],
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        if result.returncode == 0:
            return True, None
        return False, result.stderr
    except subprocess.TimeoutExpired:
        return False, "Compile timeout"
    except Exception as e:
        return False, str(e)


def run_rust_binary(binary_path: str, timeout: int = 10) -> tuple[Optional[str], Optional[str]]:
    """Run compiled Rust binary and capture output."""
    try:
        result = subprocess.run(
            [binary_path],
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return result.stdout, result.stderr if result.returncode != 0 else None
    except subprocess.TimeoutExpired:
        return None, "Timeout"
    except Exception as e:
        return None, str(e)


def check_parity(python_file: str, rust_file: str) -> SemanticResult:
    """Check semantic parity between Python and Rust versions."""
    result = SemanticResult(
        python_file=python_file,
        rust_file=rust_file,
        compiles=False,
        python_output=None,
        rust_output=None,
        outputs_match=False,
        error=None,
    )

    # Check if Rust file exists
    if not os.path.exists(rust_file):
        result.error = "Rust file does not exist"
        return result

    # Run Python
    py_out, py_err = run_python(python_file)
    if py_err and py_out is None:
        result.error = f"Python error: {py_err}"
        return result
    result.python_output = py_out

    # Compile Rust
    with tempfile.NamedTemporaryFile(suffix="_bin", delete=False) as tmp:
        binary_path = tmp.name

    try:
        compiles, compile_err = compile_rust(rust_file, binary_path)
        result.compiles = compiles

        if not compiles:
            result.error = f"Rust compile error: {compile_err[:200] if compile_err else 'Unknown'}"
            return result

        # Run Rust binary
        rs_out, rs_err = run_rust_binary(binary_path)
        if rs_err and rs_out is None:
            result.error = f"Rust runtime error: {rs_err}"
            return result
        result.rust_output = rs_out

        # Compare outputs
        result.outputs_match = (py_out == rs_out)

    finally:
        # Cleanup binary
        if os.path.exists(binary_path):
            os.unlink(binary_path)

    return result


def find_pairs(corpus_root: str) -> list[tuple[str, str]]:
    """Find Python/Rust file pairs in corpus."""
    corpus_path = Path(corpus_root)
    pairs = []

    for py_file in corpus_path.rglob("*.py"):
        # Skip test files
        if py_file.name.startswith("test_"):
            continue

        # Find corresponding .rs file
        rs_file = py_file.with_suffix(".rs")
        if rs_file.exists():
            pairs.append((str(py_file), str(rs_file)))

    return pairs


def run_parity_checks(corpus_root: str, output_path: str, max_files: int = 50) -> dict:
    """Run semantic parity checks on corpus."""
    pairs = find_pairs(corpus_root)

    results = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "corpus_root": corpus_root,
        "total_pairs": len(pairs),
        "checked": 0,
        "compiling": 0,
        "semantic_match": 0,
        "semantic_mismatch": 0,
        "compile_errors": 0,
        "semantic_parity_rate": 0.0,
        "true_success_rate": 0.0,
        "files": [],
    }

    print(f"Found {len(pairs)} Python/Rust pairs")
    print(f"Checking up to {max_files} files...")
    print()

    for i, (py_file, rs_file) in enumerate(pairs[:max_files]):
        rel_py = os.path.relpath(py_file, corpus_root)
        print(f"[{i+1}/{min(len(pairs), max_files)}] {rel_py}...", end=" ", flush=True)

        result = check_parity(py_file, rs_file)
        results["checked"] += 1

        file_result = {
            "python": rel_py,
            "rust": os.path.relpath(rs_file, corpus_root),
            "compiles": result.compiles,
            "outputs_match": result.outputs_match,
        }

        if result.error:
            file_result["error"] = result.error

        if result.compiles:
            results["compiling"] += 1
            if result.outputs_match:
                results["semantic_match"] += 1
                print("PASS (compile + semantic)")
            else:
                results["semantic_mismatch"] += 1
                print("FAIL (semantic mismatch)")
                # Include first 100 chars of each output for debugging
                if result.python_output:
                    file_result["python_output_preview"] = result.python_output[:100]
                if result.rust_output:
                    file_result["rust_output_preview"] = result.rust_output[:100]
        else:
            results["compile_errors"] += 1
            print("FAIL (compile error)")

        results["files"].append(file_result)

    # Calculate rates
    if results["compiling"] > 0:
        results["semantic_parity_rate"] = round(
            100 * results["semantic_match"] / results["compiling"], 1
        )
    if results["checked"] > 0:
        results["compile_rate"] = round(
            100 * results["compiling"] / results["checked"], 1
        )
        results["true_success_rate"] = round(
            100 * results["semantic_match"] / results["checked"], 1
        )

    # Write results
    with open(output_path, "w") as f:
        json.dump(results, f, indent=2)

    return results


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Verify semantic parity between Python and Rust - DEPYLER-1324"
    )
    parser.add_argument(
        "--corpus",
        default="/Users/noahgift/src/depyler/examples",
        help="Path to corpus root directory"
    )
    parser.add_argument(
        "--output",
        default="semantic_parity_report.json",
        help="Output report file path"
    )
    parser.add_argument(
        "--max-files",
        type=int,
        default=50,
        help="Maximum files to check"
    )

    args = parser.parse_args()

    print("=" * 60)
    print("SEMANTIC PARITY CHECK (DEPYLER-1324)")
    print("=" * 60)
    print(f"Corpus: {args.corpus}")
    print(f"Output: {args.output}")
    print()

    results = run_parity_checks(args.corpus, args.output, args.max_files)

    print()
    print("=" * 60)
    print("RESULTS")
    print("=" * 60)
    print(f"Total pairs found: {results['total_pairs']}")
    print(f"Files checked:     {results['checked']}")
    print()
    print(f"Compile success:   {results['compiling']} ({results.get('compile_rate', 0):.1f}%)")
    print(f"Semantic match:    {results['semantic_match']}")
    print(f"Semantic mismatch: {results['semantic_mismatch']}")
    print()
    print("=" * 60)
    print("KEY METRICS (Per Dr. Popper)")
    print("=" * 60)
    print()
    print(f"  Compile Rate:        {results.get('compile_rate', 0):.1f}%")
    print(f"  Semantic Parity:     {results['semantic_parity_rate']:.1f}%")
    print(f"  True Success Rate:   {results['true_success_rate']:.1f}%")
    print()
    print("  Formula: True Success = Compile Rate × Semantic Parity")
    if results['compiling'] > 0:
        calculated = results.get('compile_rate', 0) * results['semantic_parity_rate'] / 100
        print(f"  Calculated: {results.get('compile_rate', 0):.1f}% × {results['semantic_parity_rate']:.1f}% = {calculated:.1f}%")
    print()
    print(f"Written to: {args.output}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
