# bashrs Validation Process

## Overview

bashrs is a Rust-to-Shell transpiler that provides advanced validation, linting, and purification capabilities for Makefiles and shell scripts. This document describes how we use bashrs to maintain code quality in the depyler project.

## Installation

```bash
cargo install bashrs --locked
```

Verify installation:
```bash
bashrs --version
```

## Capabilities

### Makefile Validation

bashrs provides comprehensive Makefile analysis:
- **Syntax validation**: Detects tab/space mixing, invalid syntax
- **Safety checks**: Missing error handling, unquoted variables
- **Best practices**: .PHONY declarations, recursive make warnings
- **Purification**: Automatic cleanup and optimization

### Shell Script Validation

bashrs provides shell script linting:
- **Safety issues**: Unquoted variables, missing error handling
- **Dangerous commands**: rm, find, curl usage patterns
- **Input validation**: Missing checks for required inputs
- **Error handling**: Missing set -e, || exit 1 patterns

**Note**: bashrs purify has limitations with complex bash syntax. Use lint for validation only.

## Usage

### Validate All Makefiles

```bash
make validate-makefiles
```

This target:
1. Checks all Makefiles in the project (excluding target/, .git/)
2. Reports errors (warnings are allowed)
3. Fails build if any Makefile has errors

### Lint All Shell Scripts

```bash
make lint-scripts
```

This runs `scripts/bashrs_validate_all.sh` which:
1. Finds all shell scripts in the project
2. Generates `.lint-report.txt` files for each script
3. Reports errors and warnings
4. Attempts purification (may fail on complex syntax)

### Generate Validation Report

```bash
make bashrs-report
```

Displays comprehensive validation summary showing:
- Makefile validation results
- Shell script error/warning counts
- Purification success/failure rates
- Recommendations for improvements

### Manual Validation

**Single Makefile**:
```bash
bashrs make lint Makefile
bashrs make purify Makefile -o Makefile.purified
```

**Single Shell Script**:
```bash
bashrs lint script.sh
bashrs purify script.sh -o script.purified.sh
```

## Integration Points

### 1. Local Development

**Makefile Targets**:
- `make validate-makefiles` - Check all Makefiles
- `make lint-scripts` - Check all shell scripts
- `make bashrs-report` - View validation summary

**Pre-commit Hook** (`.git/hooks/pre-commit-bashrs`):
- Automatically runs on `git commit`
- Validates staged Makefiles only
- Fails commit if errors found
- Warnings are allowed

### 2. CI/CD Pipeline

**GitHub Actions** (`.github/workflows/quality-gates.yml`):
```yaml
- name: Install bashrs
  run: cargo install bashrs --locked || true

- name: Validate Makefiles with bashrs
  run: make validate-makefiles
  continue-on-error: false
```

Runs on:
- Pull requests to main
- Pushes to main
- Blocks merge if Makefile errors found

### 3. Manual Workflow

For complex changes:
1. Run `make lint-scripts` to generate reports
2. Review `.lint-report.txt` files
3. Fix critical errors manually
4. Re-run validation
5. Commit once all errors resolved

## Quality Standards

### Makefiles

**MANDATORY (Blocks Commit)**:
- Zero syntax errors (tab/space mixing)
- No fatal parse errors
- Valid Make syntax

**RECOMMENDED (Warnings Allowed)**:
- Quoted variables
- Error handling (|| exit 1)
- .PHONY declarations
- Avoid recursive make

### Shell Scripts

**MANDATORY (Should Fix)**:
- No syntax errors (bash -n)
- Critical safety issues (unquoted variables in dangerous contexts)
- Missing input validation on user inputs

**RECOMMENDED (May Defer)**:
- All variables quoted
- Comprehensive error handling
- set -euo pipefail in all scripts

## Common Issues and Fixes

### Makefile: Tab/Space Mixing

**Error**: `MAKE008: Recipe line starts with spaces instead of tab`

**Fix**:
```make
# WRONG (spaces)
target:
    @echo "hello"

# CORRECT (tab)
target:
	@echo "hello"
```

### Makefile: Missing .PHONY

**Warning**: `MAKE004: Target should be marked as .PHONY`

**Fix**:
```make
.PHONY: clean
clean:
	rm -rf target/
```

### Makefile: Unquoted Variables

**Warning**: `MAKE003: Unquoted variable in command`

**Fix**:
```make
# Before
test:
	$(CARGO) test $(TEST_FLAGS)

# After
test:
	"$(CARGO)" test "$(TEST_FLAGS)"
```

### Shell Script: Unquoted Variables

**Warning**: Unquoted variable may cause word splitting

**Fix**:
```bash
# Before
rm -rf $BUILD_DIR

# After
rm -rf "$BUILD_DIR"
```

### Shell Script: Missing Error Handling

**Warning**: `Command missing error handling`

**Fix**:
```bash
# Before
mkdir -p output/

# After
mkdir -p output/ || exit 1
```

## Limitations

### bashrs purify Limitations

`bashrs purify` cannot parse complex bash syntax:
- Advanced parameter expansion (`${var:?msg}`, `${var:-default}`)
- Complex conditionals with multiple pipes
- Process substitution (`<(command)`)
- Backtick command substitution (use `$(...)` instead)
- Here-documents with complex content

**Recommendation**: Use `bashrs lint` for validation, fix issues manually.

### Alternatives

For shell scripts, consider:
1. **shellcheck**: More mature shell script linter
2. **shfmt**: Shell script formatter
3. **Rust rewrites**: Migrate critical scripts to Rust for reliability

## Validation Results

See `bashrs_validation_summary.md` for current project status:
- **Makefiles**: 2/2 purified and validated (0 errors)
- **Shell Scripts**: 25 scripts, 23 with lint reports

### Success Metrics

**Before bashrs**:
- Root Makefile: 5 fatal errors, 63 warnings
- tdd-book Makefile: 0 errors, 8 warnings
- Shell scripts: Unknown quality

**After bashrs**:
- Root Makefile: 0 errors, ~30 warnings (32% size reduction)
- tdd-book Makefile: 0 errors, minimal warnings (22% size reduction)
- Shell scripts: 10 with errors, 23 with warnings (all documented)

## Best Practices

### Makefile Development

1. **Always use tabs** for recipe indentation
2. **Quote all variables** (`"$(VAR)"` not `$(VAR)`)
3. **Declare .PHONY** for non-file targets
4. **Add error handling** (`|| exit 1` or `set -e`)
5. **Validate before commit** (`make validate-makefiles`)

### Shell Script Development

1. **Start with safety** (`#!/bin/bash` + `set -euo pipefail`)
2. **Quote all variables** (`"$VAR"` not `$VAR`)
3. **Validate inputs** (check for required args)
4. **Add error handling** (`|| exit 1` on critical commands)
5. **Test syntax** (`bash -n script.sh`)
6. **Run bashrs lint** before commit

### Commit Workflow

```bash
# 1. Make changes to Makefile or scripts
vim Makefile

# 2. Validate locally
make validate-makefiles

# 3. Review issues
cat Makefile.lint-report.txt

# 4. Fix errors (warnings OK)
bashrs make purify Makefile -o Makefile.new
mv Makefile.new Makefile

# 5. Commit (pre-commit hook runs automatically)
git add Makefile
git commit -m "Fix Makefile validation errors"
```

## Troubleshooting

### "bashrs not found"

**Solution**:
```bash
cargo install bashrs --locked
```

### "Validation failed with errors"

**Solution**:
1. View full lint report: `bashrs make lint Makefile`
2. Focus on `[error]` lines only
3. Fix or purify: `bashrs make purify Makefile -o Makefile.purified`
4. Test: `make -f Makefile.purified help`
5. Replace: `mv Makefile.purified Makefile`

### "Purify fails on shell script"

**Solution**:
1. Use lint only: `bashrs lint script.sh > script.lint-report.txt`
2. Fix issues manually based on report
3. Consider shellcheck as alternative
4. Consider rewriting complex scripts in Rust

## Future Improvements

### Short-term

1. Fix critical shell script errors (10 scripts)
2. Add shellcheck for additional validation
3. Create purification templates for common patterns

### Long-term

1. Migrate complex shell scripts to Rust
2. Standardize error handling patterns
3. Add automated fixes for common issues
4. Enforce stricter quality gates (zero warnings)

## References

- [bashrs GitHub](https://github.com/paiml/bashrs)
- [bashrs Documentation](https://docs.rs/bashrs)
- [GNU Make Manual](https://www.gnu.org/software/make/manual/)
- [ShellCheck](https://www.shellcheck.net/)

## Changelog

- **2025-11-19**: Initial bashrs validation implementation
  - Purified root Makefile (5 errors fixed, 32% size reduction)
  - Purified tdd-book Makefile (22% size reduction)
  - Validated 25 shell scripts (23 lint reports generated)
  - Added CI/CD integration
  - Added pre-commit hooks
