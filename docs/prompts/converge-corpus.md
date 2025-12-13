# Converge Corpus to Target Rate

Run depyler convergence loop on Python corpus. Monitor. Fix transpiler bugs immediately.

## Command

```bash
./target/release/depyler converge \
  --input-dir /home/noah/src/reprorusted-python-cli/examples \
  --target-rate 80 \
  --oracle --explain --cache \
  --display plain
```

## Flags

| Flag | Purpose |
|------|---------|
| `--input-dir` | Python corpus directory |
| `--target-rate N` | Target N% compilation pass rate |
| `--oracle` | Train ML error classifier |
| `--explain` | Generate decision traces |
| `--cache` | O(1) content-hash cache |
| `--display plain` | CI-friendly output |

## Protocol

1. Run in background
2. Monitor progress via `BashOutput`
3. **STOP THE LINE** on transpiler panics/crashes
4. Report final `Rate: X% │ Passing: N/M`

## Expected Output

```
Compiling 601 files...
[████████████████████] 100% ✓ file.py
📊 Oracle: Training complete (N samples)
│ Rate: X% │ Passing: N/M │
```

## Idempotent

Safe to re-run. Cache prevents redundant compilation.
