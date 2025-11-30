# Overnight: Reach 80% Single-Shot Compile

Autonomous improvement loop designed to run all night. Handles multi-layer errors, tracks incremental progress, recovers from failures.

## Setup
```bash
export LOG="/tmp/depyler-overnight-$(date +%Y%m%d).log"
echo "=== Started $(date) ===" >> $LOG
```

## Main Loop (max 50 iterations)

```
for iteration in 1..50:
    1. Measure
    2. If >= 80%: SUCCESS, stop
    3. Find highest-impact error pattern
    4. Implement minimal fix
    5. Validate (tests + clippy)
    6. If fail: rollback, try different approach or skip pattern
    7. Measure delta
    8. If improved: commit
    9. If stuck 3x on same pattern: skip, move to next
    10. Log everything
```

## Phase 1: Measure State

```bash
cd /home/noah/src/reprorusted-python-cli
make corpus-retranspile 2>&1 | tail -5
RESULT=$(uv run python scripts/measure_compile_rate.py --json)
RATE=$(echo $RESULT | jq -r '.rate')
echo "[$(date)] Rate: $RATE%" >> $LOG

if (( $(echo "$RATE >= 80" | bc -l) )); then
    echo "SUCCESS at $RATE%" >> $LOG
    exit 0
fi
```

## Phase 2: Identify Top Error (Smart Selection)

```bash
# Get error frequency with file counts
uv run python scripts/measure_compile_rate.py --json | \
  jq -r '.failing[] | .error' | \
  sed 's/error\[E[0-9]*\]: //' | \
  sort | uniq -c | sort -rn | head -10
```

**Priority order:**
1. Errors affecting 10+ files
2. Errors that are "leaf" issues (no deeper dependencies)
3. Module mapping issues (E0425 cannot find value)
4. Type issues (E0308 mismatched types)
5. Method issues (E0599 no method named)

**Skip patterns that need major architecture:**
- async runtime issues
- complex trait bounds
- macro expansions

## Phase 3: Investigate Before Fixing

```bash
# Find ONE failing file with this error
FILE=$(uv run python scripts/measure_compile_rate.py --json | \
  jq -r '.failing[] | select(.error | contains("TARGET_ERROR")) | .name' | head -1)

# Read the Python source
find /home/noah/src/reprorusted-python-cli/examples -name "*.py" -path "*$FILE*" | head -1 | xargs cat

# Read the generated Rust
find /home/noah/src/reprorusted-python-cli/examples -name "*.rs" -path "*$FILE*" | head -1 | xargs cat

# Understand the gap - what Python construct maps to what Rust?
```

## Phase 4: Implement Fix (Minimal)

**Location guide:**
```
direct_rules.rs     - Function/method call mappings (os.path.join -> Path::new().join())
type_mapper.rs      - Type mappings (List[int] -> Vec<i32>)
expr_gen.rs         - Expression generation
stmt_gen.rs         - Statement generation
cargo_toml_gen.rs   - Dependency and crate type logic
```

**Fix principles:**
- Change ONE thing at a time
- Add pattern matching, not special cases
- Follow existing code patterns
- Add test if touching complex logic

## Phase 5: Validate

```bash
cd /home/noah/src/depyler
cargo test -p depyler-core 2>&1 | tee -a $LOG
if [ $? -ne 0 ]; then
    echo "[$(date)] TESTS FAILED - rolling back" >> $LOG
    git checkout -- .
    # Try narrower fix or skip this pattern
    continue
fi

cargo clippy -p depyler-core -- -D warnings 2>&1 | tee -a $LOG
if [ $? -ne 0 ]; then
    echo "[$(date)] CLIPPY FAILED - rolling back" >> $LOG
    git checkout -- .
    continue
fi
```

## Phase 6: Measure Impact

```bash
cargo install --path crates/depyler --force 2>&1 | tail -3
cd /home/noah/src/reprorusted-python-cli
make corpus-retranspile 2>&1 | tail -5

NEW_RESULT=$(uv run python scripts/measure_compile_rate.py --json)
NEW_RATE=$(echo $NEW_RESULT | jq -r '.rate')
DELTA=$(echo "$NEW_RATE - $RATE" | bc)

echo "[$(date)] $RATE% -> $NEW_RATE% (delta: $DELTA%)" >> $LOG

# Check for regression
if (( $(echo "$DELTA < -1" | bc -l) )); then
    echo "REGRESSION - rolling back" >> $LOG
    cd /home/noah/src/depyler && git checkout -- .
    continue
fi
```

## Phase 7: Commit Progress

```bash
cd /home/noah/src/depyler
git add -A
git commit -m "fix(core): [describe what was fixed] (Refs #193)

Rate: $RATE% -> $NEW_RATE%

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

## Stuck Detection & Recovery

Track attempts per error pattern:
```
ATTEMPTS[pattern] += 1
if ATTEMPTS[pattern] >= 3:
    echo "Skipping $pattern after 3 attempts" >> $LOG
    SKIP_LIST.add(pattern)
    continue to next pattern
```

**If no progress for 5 iterations:**
1. Re-analyze error distribution
2. Try combination fixes (fix 2 related errors together)
3. Look for "almost working" files (only 1-2 errors)

## Error Pattern Cookbook

| Error | Root Cause | Fix Location |
|-------|-----------|--------------|
| `cannot find value X` | Missing module mapping | direct_rules.rs |
| `no method named X` | Method not mapped | direct_rules.rs |
| `expected value, found crate` | Call syntax wrong | expr_gen.rs |
| `mismatched types` | Type mapping wrong | type_mapper.rs |
| `cannot find type` | Type not imported | type_mapper.rs |
| `no field X on type` | Struct field mapping | direct_rules.rs |
| `X defined multiple times` | Duplicate const/fn | stmt_gen.rs |
| `main function not found` | Test file as bin | cargo_toml_gen.rs |

## Known Hard Problems (Skip for Now)

- `async runtime` - needs tokio setup
- `trait X is not implemented` - complex trait bounds
- `cannot infer type` - needs explicit annotations
- `use of moved value` - ownership issues

## Exit Conditions

| Condition | Action |
|-----------|--------|
| Rate >= 80% | SUCCESS |
| 50 iterations | TIMEOUT - report progress |
| Tests fail 5x in a row | BLOCKED - investigate |
| No patterns left to try | STUCK - report remaining |

## Resume Capability

All progress is committed. If interrupted:
1. Run `git log --oneline -5` to see last fix
2. Run `make corpus-e2e-rate` to see current rate
3. Resume the loop

## Final Report

When done (success, timeout, or stuck):
```bash
echo "=== Final Report ===" >> $LOG
echo "Rate: $(uv run python scripts/measure_compile_rate.py --json | jq -r '.rate')%" >> $LOG
echo "Commits: $(git log --oneline --since='12 hours ago' | wc -l)" >> $LOG
echo "Remaining errors:" >> $LOG
uv run python scripts/measure_compile_rate.py --json | jq -r '.failing[].error' | sort | uniq -c | sort -rn | head -10 >> $LOG
cat $LOG
```
