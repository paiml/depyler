# Depyler TDD Book

Test-driven Python standard library reference for validating the Depyler transpiler.

## Overview

This project provides comprehensive, test-driven examples of Python's standard library. Each example is:

- ✅ **Verified**: All code is tested with pytest
- 📊 **Quality-Controlled**: PMAT quality gates enforce A+ code standards
- 🔍 **Edge-Case Focused**: Property-based testing with Hypothesis
- 📚 **Documentation-Ready**: Tests generate reference documentation

## Quick Start

### Installation

```bash
# Install dependencies
pip install -e ".[dev]"

# Or use requirements file
pip install -r requirements.txt
```

### Running Tests

```bash
# Run all tests
pytest tests/

# Run with coverage
pytest tests/ --cov --cov-report=html

# Run specific module tests
pytest tests/test_os/

# Run in parallel
pytest tests/ -n auto
```

### Quality Gates

```bash
# Run all quality checks (if pmat is installed)
pmat analyze complexity --fail-on-violation
pmat analyze satd --fail-on-violation
pmat quality-gate --strict
```

## Project Structure

```
tdd-book/
├── tests/                  # Test files (become documentation)
│   ├── test_os/           # os module tests
│   └── conftest.py        # Pytest configuration
├── scripts/               # Utility scripts
├── docs/                  # Generated documentation
│   ├── modules/           # Per-module docs
│   └── edge-cases/        # Bug discoveries
├── reports/               # Quality reports
├── pyproject.toml         # Project configuration
└── README.md              # This file
```

## Development Workflow

### TDD Protocol

1. **Write Test First**: Create failing test
2. **Run Test**: Verify it fails
3. **Implement**: Make test pass
4. **Refactor**: Improve code quality
5. **Quality Check**: Run quality gates

### Example Workflow

```bash
# 1. Write test
vim tests/test_os/test_new_feature.py

# 2. Run test (expect failure)
pytest tests/test_os/test_new_feature.py -v

# 3. Implement code (tests ARE the implementation for stdlib)

# 4. Run test (expect success)
pytest tests/test_os/test_new_feature.py -v

# 5. Quality check
pmat analyze complexity tests/test_os/test_new_feature.py --max-cyclomatic 10
```

## Quality Standards

- **Cyclomatic Complexity**: ≤10 per function
- **Test Coverage**: ≥80%
- **SATD**: 0 (zero TODO/FIXME comments)
- **Type Coverage**: 100% for public APIs

## Contributing

1. Follow TDD protocol
2. Ensure all quality gates pass
3. Add edge case tests
4. Document any stdlib bugs discovered

## License

MIT

## References

- [Python Standard Library](https://docs.python.org/3/library/)
- [Depyler Transpiler](https://github.com/yourusername/depyler)
- [Hypothesis Testing](https://hypothesis.readthedocs.io/)
