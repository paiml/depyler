#!/usr/bin/env python3
"""Classify corpus into Tier 1 (typed) vs Tier 2 (untyped) Python.

DEPYLER-1327: Per Dr. Popper's recommendation - "You cannot interpret your
compile rate without knowing Tier 1 vs Tier 2 composition."

Tier Classification:
- Tier 1: Fully typed Python with PEP 484 annotations (expected ceiling: 90%+)
- Tier 2: Untyped Python (expected ceiling: 40-60%)

The formula for adjusted target:
    Adjusted Target = 80% of (Total - Untranspilable)
"""

import ast
import json
import os
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Optional


@dataclass
class TierClassification:
    """Classification result for a Python file."""
    tier: int  # 1 or 2
    has_function_annotations: bool
    has_variable_annotations: bool
    has_return_annotations: bool
    annotation_count: int
    function_count: int
    annotation_ratio: float  # annotations / functions
    complexity_score: int  # basic complexity indicator
    reason: str


def count_annotations(tree: ast.AST) -> dict:
    """Count different types of annotations in an AST."""
    stats = {
        "function_annotations": 0,  # def foo(x: int)
        "return_annotations": 0,    # def foo() -> int
        "variable_annotations": 0,  # x: int = 5
        "function_count": 0,
        "class_count": 0,
    }

    for node in ast.walk(tree):
        if isinstance(node, ast.FunctionDef):
            stats["function_count"] += 1
            # Check parameter annotations
            for arg in node.args.args + node.args.posonlyargs + node.args.kwonlyargs:
                if arg.annotation is not None:
                    stats["function_annotations"] += 1
            # Check return annotation
            if node.returns is not None:
                stats["return_annotations"] += 1

        elif isinstance(node, ast.AsyncFunctionDef):
            stats["function_count"] += 1
            for arg in node.args.args + node.args.posonlyargs + node.args.kwonlyargs:
                if arg.annotation is not None:
                    stats["function_annotations"] += 1
            if node.returns is not None:
                stats["return_annotations"] += 1

        elif isinstance(node, ast.AnnAssign):
            # x: int = 5 or x: int
            stats["variable_annotations"] += 1

        elif isinstance(node, ast.ClassDef):
            stats["class_count"] += 1

    return stats


def compute_complexity(tree: ast.AST) -> int:
    """Compute basic complexity score."""
    complexity = 0
    for node in ast.walk(tree):
        # Control flow increases complexity
        if isinstance(node, (ast.If, ast.For, ast.While, ast.Try, ast.With)):
            complexity += 1
        # Nested functions/classes increase complexity
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            complexity += 1
        # Comprehensions
        elif isinstance(node, (ast.ListComp, ast.DictComp, ast.SetComp, ast.GeneratorExp)):
            complexity += 1
    return complexity


def classify_file(filepath: str) -> Optional[TierClassification]:
    """Classify a Python file into Tier 1 or Tier 2."""
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            source = f.read()
        tree = ast.parse(source)
    except (SyntaxError, UnicodeDecodeError) as e:
        return None  # Cannot parse

    stats = count_annotations(tree)
    complexity = compute_complexity(tree)

    total_annotations = (
        stats["function_annotations"]
        + stats["return_annotations"]
        + stats["variable_annotations"]
    )

    # Calculate annotation ratio
    if stats["function_count"] > 0:
        # Expected: at least 1 annotation per function (params + return)
        annotation_ratio = total_annotations / (stats["function_count"] * 2)
    else:
        annotation_ratio = 1.0 if total_annotations > 0 else 0.0

    # Tier classification logic
    # Tier 1: Has significant type annotations (>=50% of functions have annotations)
    # Tier 2: No or minimal annotations
    has_func_ann = stats["function_annotations"] > 0
    has_ret_ann = stats["return_annotations"] > 0
    has_var_ann = stats["variable_annotations"] > 0

    if annotation_ratio >= 0.5 or (has_func_ann and has_ret_ann):
        tier = 1
        reason = f"Typed: {total_annotations} annotations, {annotation_ratio:.0%} coverage"
    elif total_annotations > 0:
        tier = 1  # Has some annotations, count as Tier 1
        reason = f"Partially typed: {total_annotations} annotations"
    else:
        tier = 2
        reason = "Untyped: No type annotations found"

    return TierClassification(
        tier=tier,
        has_function_annotations=has_func_ann,
        has_variable_annotations=has_var_ann,
        has_return_annotations=has_ret_ann,
        annotation_count=total_annotations,
        function_count=stats["function_count"],
        annotation_ratio=annotation_ratio,
        complexity_score=complexity,
        reason=reason,
    )


def classify_corpus(corpus_root: str, output_path: str) -> dict:
    """Classify all Python files in corpus."""
    corpus_path = Path(corpus_root)

    if not corpus_path.exists():
        print(f"Error: Corpus path does not exist: {corpus_root}", file=sys.stderr)
        sys.exit(1)

    python_files = sorted(corpus_path.rglob("*.py"))

    results = {
        "corpus_root": str(corpus_path.resolve()),
        "total_files": 0,
        "tier1_count": 0,
        "tier2_count": 0,
        "parse_errors": 0,
        "tier1_percentage": 0.0,
        "tier2_percentage": 0.0,
        "expected_ceiling": {
            "tier1": "90%+",
            "tier2": "40-60%",
        },
        "files": [],
    }

    tier_counts = Counter()

    for py_file in python_files:
        rel_path = str(py_file.relative_to(corpus_path))
        classification = classify_file(str(py_file))

        if classification is None:
            results["parse_errors"] += 1
            results["files"].append({
                "path": rel_path,
                "tier": None,
                "error": "Parse error",
            })
        else:
            tier_counts[classification.tier] += 1
            results["files"].append({
                "path": rel_path,
                "tier": classification.tier,
                "annotation_count": classification.annotation_count,
                "function_count": classification.function_count,
                "annotation_ratio": round(classification.annotation_ratio, 2),
                "complexity": classification.complexity_score,
                "reason": classification.reason,
            })

    results["total_files"] = len(python_files)
    results["tier1_count"] = tier_counts[1]
    results["tier2_count"] = tier_counts[2]

    if results["total_files"] > 0:
        results["tier1_percentage"] = round(
            100 * results["tier1_count"] / results["total_files"], 1
        )
        results["tier2_percentage"] = round(
            100 * results["tier2_count"] / results["total_files"], 1
        )

    # Write results
    with open(output_path, "w") as f:
        json.dump(results, f, indent=2)

    return results


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Classify corpus into Tier 1 (typed) vs Tier 2 (untyped) - DEPYLER-1327"
    )
    parser.add_argument(
        "--corpus",
        default="/Users/noahgift/src/depyler/examples",
        help="Path to corpus root directory"
    )
    parser.add_argument(
        "--output",
        default="corpus_tiers.json",
        help="Output classification file path"
    )

    args = parser.parse_args()

    print(f"Classifying corpus: {args.corpus}")
    print(f"Output: {args.output}")
    print()

    results = classify_corpus(args.corpus, args.output)

    print("=" * 60)
    print("TIER CLASSIFICATION RESULTS")
    print("=" * 60)
    print(f"Total files: {results['total_files']}")
    print(f"Parse errors: {results['parse_errors']}")
    print()
    print(f"Tier 1 (Typed):   {results['tier1_count']:3d} ({results['tier1_percentage']:5.1f}%) - Expected ceiling: 90%+")
    print(f"Tier 2 (Untyped): {results['tier2_count']:3d} ({results['tier2_percentage']:5.1f}%) - Expected ceiling: 40-60%")
    print()
    print("=" * 60)
    print("INTERPRETATION")
    print("=" * 60)
    print()

    # Calculate adjusted target
    tier1_target = results['tier1_count'] * 0.9
    tier2_target = results['tier2_count'] * 0.5
    total_target = tier1_target + tier2_target
    if results['total_files'] > 0:
        adjusted_rate = total_target / results['total_files']
        print(f"If Tier 1 achieves 90% and Tier 2 achieves 50%:")
        print(f"  Adjusted Target = ({results['tier1_count']} × 0.9) + ({results['tier2_count']} × 0.5)")
        print(f"                  = {tier1_target:.1f} + {tier2_target:.1f}")
        print(f"                  = {total_target:.1f} files")
        print(f"  Expected Rate   = {adjusted_rate:.1%}")
        print()
        print(f"Current 16.25% may be reasonable if Tier 2 dominates the corpus.")

    print()
    print(f"Written to: {args.output}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
