#!/usr/bin/env python3
"""
Systematic corpus testing using verificar-generated programs
This prevents thrashing by testing comprehensively and categorizing failures
"""

import json
import subprocess
import sys
from pathlib import Path
from datetime import datetime
from collections import Counter
import re

def main():
    corpus_file = sys.argv[1] if len(sys.argv) > 1 else "corpus_d3_c50.json"
    output_dir = Path(f"test_results_{datetime.now().strftime('%Y%m%d_%H%M%S')}")
    depyler = Path("/home/noah/src/depyler/target/release/depyler")

    output_dir.mkdir(exist_ok=True)

    print("🧪 Verificar Corpus Testing")
    print("============================")
    print(f"Corpus: {corpus_file}")
    print(f"Output: {output_dir}")
    print("")

    # Load corpus
    with open(corpus_file) as f:
        programs = json.load(f)

    # Statistics
    stats = {
        'total': len(programs),
        'transpile_success': 0,
        'transpile_fail': 0,
        'compile_success': 0,
        'compile_fail': 0,
    }

    error_categories = Counter()
    error_details = []

    for i, program in enumerate(programs, 1):
        code = program['code']
        depth = program['ast_depth']
        features = ','.join(program.get('features', []))

        test_file = output_dir / f"test_{i}.py"
        rust_file = output_dir / f"test_{i}.rs"
        log_file = output_dir / f"test_{i}.log"

        # Write Python program
        test_file.write_text(code)

        print(f"[{i}/{stats['total']}] Testing depth={depth} features={features}")

        # Step 1: Transpile
        try:
            result = subprocess.run(
                [str(depyler), "transpile", str(test_file), "-o", str(rust_file)],
                capture_output=True,
                text=True,
                timeout=30
            )

            log_file.write_text(f"=== TRANSPILE OUTPUT ===\n{result.stdout}\n{result.stderr}\n")

            if result.returncode == 0 and rust_file.exists():
                stats['transpile_success'] += 1
                print("  ✅ Transpilation succeeded")

                # Step 2: Compile
                try:
                    compile_result = subprocess.run(
                        ["rustc", "--crate-type", "lib", str(rust_file),
                         "-o", str(output_dir / f"test_{i}.rlib")],
                        capture_output=True,
                        text=True,
                        timeout=30
                    )

                    with open(log_file, 'a') as f:
                        f.write(f"\n=== COMPILE OUTPUT ===\n{compile_result.stdout}\n{compile_result.stderr}\n")

                    if compile_result.returncode == 0:
                        stats['compile_success'] += 1
                        print("  ✅ Compilation succeeded")
                        (output_dir / f"test_{i}.status").write_text("PASS")
                    else:
                        stats['compile_fail'] += 1
                        print("  ❌ Compilation failed")

                        # Categorize errors
                        errors = compile_result.stderr
                        error_patterns = {
                            'E0308_type_mismatch': r'error\[E0308\]',
                            'E0369_cannot_add': r'error\[E0369\]',
                            'E0425_cannot_find': r'error\[E0425\]',
                            'E0277_trait_not_impl': r'error\[E0277\]',
                            'E0061_wrong_arg_count': r'error\[E0061\]',
                            'E0432_unresolved_import': r'error\[E0432\]',
                            'E0599_no_method': r'error\[E0599\]',
                        }

                        found_errors = []
                        for category, pattern in error_patterns.items():
                            if re.search(pattern, errors):
                                error_categories[category] += 1
                                found_errors.append(category)

                        error_details.append({
                            'test_id': i,
                            'depth': depth,
                            'features': features,
                            'errors': found_errors,
                            'code_snippet': code[:100]
                        })

                        (output_dir / f"test_{i}.status").write_text("COMPILE_FAIL")

                except subprocess.TimeoutExpired:
                    print("  ⏱️  Compilation timeout")
                    (output_dir / f"test_{i}.status").write_text("COMPILE_TIMEOUT")
                except Exception as e:
                    print(f"  ⚠️  Compilation error: {e}")
                    (output_dir / f"test_{i}.status").write_text("COMPILE_ERROR")

            else:
                stats['transpile_fail'] += 1
                print("  ❌ Transpilation failed")
                (output_dir / f"test_{i}.status").write_text("TRANSPILE_FAIL")

        except subprocess.TimeoutExpired:
            print("  ⏱️  Transpilation timeout")
            stats['transpile_fail'] += 1
            (output_dir / f"test_{i}.status").write_text("TRANSPILE_TIMEOUT")
        except Exception as e:
            print(f"  ⚠️  Transpilation error: {e}")
            stats['transpile_fail'] += 1
            (output_dir / f"test_{i}.status").write_text("TRANSPILE_ERROR")

        print("")

    # Generate summary
    summary_file = output_dir / "SUMMARY.txt"
    transpile_rate = (stats['transpile_success'] / stats['total']) * 100 if stats['total'] > 0 else 0
    compile_rate = (stats['compile_success'] / stats['transpile_success']) * 100 if stats['transpile_success'] > 0 else 0
    overall_rate = (stats['compile_success'] / stats['total']) * 100 if stats['total'] > 0 else 0

    summary = f"""Verificar Corpus Testing Summary
=================================

Total programs tested: {stats['total']}

Transpilation:
  Success: {stats['transpile_success']} ({transpile_rate:.1f}%)
  Failure: {stats['transpile_fail']} ({100-transpile_rate:.1f}%)

Compilation (of transpiled programs):
  Success: {stats['compile_success']} ({compile_rate:.1f}%)
  Failure: {stats['compile_fail']} ({100-compile_rate:.1f}%)

Overall pass rate: {overall_rate:.1f}%

Error Categories (top failures):
"""

    for category, count in error_categories.most_common():
        summary += f"  {category}: {count}\n"

    summary += "\n"
    summary += "Detailed Failures:\n"
    summary += "==================\n"
    for detail in error_details[:10]:  # Top 10 failures
        summary += f"\nTest #{detail['test_id']} (depth={detail['depth']})\n"
        summary += f"  Errors: {', '.join(detail['errors'])}\n"
        summary += f"  Code: {detail['code_snippet']}...\n"

    print(summary)
    summary_file.write_text(summary)

    # Generate JSON report for programmatic analysis
    report = {
        'stats': stats,
        'error_categories': dict(error_categories),
        'error_details': error_details,
        'timestamp': datetime.now().isoformat(),
        'corpus_file': corpus_file,
    }

    (output_dir / "report.json").write_text(json.dumps(report, indent=2))

    print(f"\n📊 Results saved to: {output_dir}")
    print(f"📄 Summary: {summary_file}")
    print(f"📊 JSON Report: {output_dir}/report.json")

if __name__ == "__main__":
    main()
