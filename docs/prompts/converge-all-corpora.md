# Converge All Corpora

Run depyler convergence loop on all registered corpora. Monitor. Fix transpiler bugs immediately.

## Registered Corpora

| Corpus | Files | TDG | Grade | Coverage | GitHub |
|--------|-------|-----|-------|----------|--------|
| reprorusted-python-cli | ~601 | - | - | - | [paiml/reprorusted-python-cli](https://github.com/paiml/reprorusted-python-cli) |
| reprorusted-std-only | 1382 | 91.9 | A | 100% | [paiml/reprorusted-std-only](https://github.com/paiml/reprorusted-std-only) |
| fully-typed-reprorusted-python-cli | 1839 | 90.5 | A | 100% | [paiml/fully-typed-reprorusted-python-cli](https://github.com/paiml/fully-typed-reprorusted-python-cli) |

**Total: 3822 Python files across 3 corpora**

## Commands

### Corpus 1: reprorusted-python-cli (Original)

```bash
./target/release/depyler converge \
  --input-dir /home/noah/src/reprorusted-python-cli/examples \
  --target-rate 80 \
  --oracle --explain --cache \
  --display plain
```

### Corpus 2: reprorusted-std-only (Stdlib)

```bash
./target/release/depyler converge \
  --input-dir /home/noah/src/reprorusted-std-only/src \
  --target-rate 80 \
  --oracle --explain --cache \
  --display plain
```

### Corpus 3: fully-typed-reprorusted-python-cli (Typed CLI)

```bash
./target/release/depyler converge \
  --input-dir /home/noah/src/fully-typed-reprorusted-python-cli/src \
  --target-rate 80 \
  --oracle --explain --cache \
  --display plain
```

## Sequential Convergence (All Corpora)

```bash
# Build release binary
cargo build --release --bin depyler
DEPYLER="./target/release/depyler"

# Warm caches for all corpora
$DEPYLER cache warm --input-dir /home/noah/src/reprorusted-python-cli/examples
$DEPYLER cache warm --input-dir /home/noah/src/reprorusted-std-only/src
$DEPYLER cache warm --input-dir /home/noah/src/fully-typed-reprorusted-python-cli/src

# Converge each corpus
echo "=== Corpus 1: reprorusted-python-cli ==="
$DEPYLER converge --input-dir /home/noah/src/reprorusted-python-cli/examples --target-rate 80 --display minimal

echo "=== Corpus 2: reprorusted-std-only ==="
$DEPYLER converge --input-dir /home/noah/src/reprorusted-std-only/src --target-rate 80 --display minimal

echo "=== Corpus 3: fully-typed-reprorusted-python-cli ==="
$DEPYLER converge --input-dir /home/noah/src/fully-typed-reprorusted-python-cli/src --target-rate 80 --display minimal
```

## Corpus Characteristics

### reprorusted-std-only
- **Focus**: Python stdlib mappings (functools, datetime, enum, contextlib, re, io, hashlib, dataclasses, argparse, typing)
- **Quality**: TDG 91.9/100 (A), 182 tests, 100% coverage
- **Use Case**: Validate stdlib-to-Rust mappings

### fully-typed-reprorusted-python-cli
- **Focus**: CLI utilities with strict type annotations
- **Quality**: TDG 90.5/100 (A), 152 tests, 100% coverage
- **Use Case**: Validate type inference with fully-annotated Python

## Protocol

1. Run each corpus in sequence
2. Monitor progress via `BashOutput`
3. **STOP THE LINE** on transpiler panics/crashes
4. Report final rate for each corpus
5. Aggregate: `Total Rate = (Pass1 + Pass2 + Pass3) / (Total1 + Total2 + Total3)`

## Expected Output

```
=== Corpus 1: reprorusted-python-cli ===
│ Rate: 80% │ Passing: 481/601 │

=== Corpus 2: reprorusted-std-only ===
│ Rate: X% │ Passing: N/1382 │

=== Corpus 3: fully-typed-reprorusted-python-cli ===
│ Rate: X% │ Passing: N/1839 │
```

## Idempotent

Safe to re-run. Cache prevents redundant compilation.
