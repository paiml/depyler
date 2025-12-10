# DEPYLER-0901: Stdlib Mapping Population

## Problem

The corpus convergence rate is stuck at ~0.5% (3/601 files) because the stdlib mapping system (`stdlib_mappings.rs`) exists but is nearly empty. Only `csv` and basic file I/O are mapped.

## Evidence

Import frequency analysis of reprorusted-python-cli corpus (601 files):

| Import | Count | Status | Rust Equivalent |
|--------|-------|--------|-----------------|
| `argparse` | 269 | **UNMAPPED** | `clap` |
| `sys` | 184 | **UNMAPPED** | `std::env`, `std::process` |
| `subprocess` | 146 | **UNMAPPED** | `std::process::Command` |
| `pathlib.Path` | 112 | **UNMAPPED** | `std::path::PathBuf` |
| `pytest` | 61 | skip | test-only |
| `json` | 58 | **UNMAPPED** | `serde_json` |
| `dataclasses` | 58 | partial | derive macros |
| `math` | 38 | **UNMAPPED** | `std::f64::consts` |
| `os` | 34 | **UNMAPPED** | `std::env`, `std::fs` |
| `numpy` | 25 | **UNMAPPED** | `ndarray` |
| `re` | 16 | **UNMAPPED** | `regex` |

**~800 import occurrences are blocked by missing mappings.**

## Root Cause

The `StdlibMappings` infrastructure exists with plugin support but only registers:
- `csv.DictReader.fieldnames`
- `csv.DictReader.__iter__`
- `csv.Reader.fieldnames`
- `builtins.file.__iter__`
- `io.TextIOWrapper.__iter__`

## Solution

Populate the mapping dict with the top 10 Python stdlib modules. This is a data entry problem, not a transpilation problem.

### Priority Order (by impact)

1. **argparse** (269 uses) - Map to clap derive macros
2. **sys** (184 uses) - Map `sys.argv`, `sys.exit`, `sys.stderr`
3. **subprocess** (146 uses) - Map to `std::process::Command`
4. **pathlib** (112 uses) - Map Path to PathBuf
5. **json** (58 uses) - Map to serde_json
6. **math** (38 uses) - Map to std::f64::consts + libm
7. **os** (34 uses) - Map os.path, os.environ, os.getcwd
8. **re** (16 uses) - Map to regex crate

## Implementation

### Phase 1: Module-level mappings (generate stubs that compile)

Add to `stdlib_mappings.rs`:

```rust
fn register_argparse_mappings(mappings: &mut HashMap<...>) {
    // argparse.ArgumentParser -> clap::Parser
    mappings.insert(
        ("argparse".into(), "ArgumentParser".into(), "add_argument".into()),
        RustPattern::CustomTemplate {
            template: "/* argparse mapped: {var}.add_argument() */"
        },
    );
}
```

### Phase 2: Function-level mappings (correct semantics)

For each module, map the specific methods/functions used in the corpus.

## Test Plan

1. Add unit tests for each mapping
2. Run converge on corpus
3. Target: 50%+ compilation rate after argparse+sys+subprocess+pathlib

## Acceptance Criteria

- [ ] argparse mapped (target: +45% files)
- [ ] sys mapped (target: +30% files)
- [ ] subprocess mapped (target: +24% files)
- [ ] pathlib mapped (target: +18% files)
- [ ] json mapped (target: +9% files)
- [ ] Converge rate reaches 50%+ on corpus

## References

- `crates/depyler-core/src/stdlib_mappings.rs` - existing infrastructure
- `/home/noah/src/reprorusted-python-cli/examples/` - corpus
