# Depyler Single-Shot Compile Rate: Autonomous Improvement Loop

## Goal
Reach 80% single-shot compile rate on reprorusted-python-cli corpus. Work through bugs autonomously.

## Measure First
```bash
cd /home/noah/src/reprorusted-python-cli
make corpus-retranspile
make corpus-e2e-rate
```
If >= 80%, report success and stop.

## Iteration Loop

### 1. Find Top Error
```bash
cd /home/noah/src/reprorusted-python-cli
uv run python scripts/measure_compile_rate.py --json | jq -r '.failing[].error' | sort | uniq -c | sort -rn | head -5
```

### 2. Investigate
Find a failing example, read its Python source and generated Rust. Understand the gap.

### 3. Fix in depyler
Locations:
- Module mappings → `crates/depyler-core/src/direct_rules.rs`
- Type mappings → `crates/depyler-core/src/type_mapper.rs`
- Expr generation → `crates/depyler-core/src/rust_gen/expr_gen.rs`
- Cargo.toml → `crates/depyler-core/src/cargo_toml_gen.rs`

### 4. Validate
```bash
cd /home/noah/src/depyler
cargo test -p depyler-core
cargo clippy -p depyler-core -- -D warnings
```
If fails: `git checkout -- .` and try different approach.

### 5. Measure Impact
```bash
cargo install --path crates/depyler --force
cd /home/noah/src/reprorusted-python-cli
make corpus-retranspile && make corpus-e2e-rate
```
If regression: rollback. If no change: still commit (may help other metrics).

### 6. Commit
```bash
git add -A && git commit -m "fix(core): [description] (Refs #193)"
```

### 7. Repeat until 80%

## Bug Recovery

| Issue | Action |
|-------|--------|
| Tests fail | Rollback, try narrower fix |
| Regression | Rollback, add guards |
| Same error 3x | Skip, move to next pattern |
| Transpile drops | Rollback, fix is too broad |

## Priority Errors (from corpus analysis)
1. `cannot find value os` - 16 files
2. `cannot find function date` - 10 files
3. `expected value, found crate tempfile` - 8 files
4. `SCRIPT defined multiple times` - 5 files

## Exit
- SUCCESS: Rate >= 80%
- STUCK: No progress 5 iterations
- BLOCKED: Can't fix failing tests
