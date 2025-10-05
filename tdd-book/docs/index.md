# Depyler TDD Book

Welcome to the **Depyler TDD Book** - a comprehensive, test-driven reference for Python's standard library, designed to validate the Depyler transpiler.

## 🎯 Overview

This project provides extensively tested examples of Python's standard library. Each module is:

- ✅ **Verified**: All code is tested with pytest (1350+ tests)
- 📊 **Quality-Controlled**: PMAT quality gates enforce A+ code standards
- 🔍 **Edge-Case Focused**: Property-based testing with Hypothesis
- 📚 **Documentation-Ready**: Tests generate reference documentation
- 🎭 **Mutation Tested**: Ensures test quality through mutation testing

## 📊 Current Status

!!! info "Integration Progress"
    - **Modules Covered**: 27/200 (13.5%)
    - **Test Pass Rate**: 1350/1350 (100%)
    - **Coverage**: 99.8%
    - **SATD**: 0 (zero technical debt)

See the [Integration Status](integration.md) page for detailed progress tracking.

## 🚀 Quick Start

### Running Tests Locally

```bash
# Install dependencies
cd tdd-book
pip install -r requirements.txt

# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov --cov-report=html

# Run specific module
pytest tests/test_json/ -v
```

### Quality Gates

```bash
# Run quality checks (requires pmat)
pmat analyze complexity --fail-on-violation
pmat analyze satd --fail-on-violation
pmat quality-gate --strict
```

## 📚 Module Documentation

Explore comprehensive test-driven documentation for Python standard library modules:

### Phase 1: Core Utilities ✅ Complete

- [os](modules/os.md) - Operating system interfaces
- [sys](modules/sys.md) - System-specific parameters
- [pathlib](modules/pathlib.md) - Object-oriented filesystem paths
- [time](modules/time.md) - Time access and conversions
- [datetime](modules/datetime.md) - Date and time handling
- [calendar](modules/calendar.md) - Calendar operations

### Phase 2: Data Processing ✅ Complete

- [json](modules/json.md) - JSON encoding and decoding
- [csv](modules/csv.md) - CSV file reading and writing
- [collections](modules/collections.md) - Container datatypes
- [itertools](modules/itertools.md) - Iterator building blocks
- [functools](modules/functools.md) - Higher-order functions
- [copy](modules/copy.md) - Shallow and deep copy operations
- [array](modules/array.md) - Efficient arrays
- [base64](modules/base64.md) - Base64 encoding
- [decimal](modules/decimal.md) - Decimal arithmetic
- [fractions](modules/fractions.md) - Rational numbers
- [hashlib](modules/hashlib.md) - Secure hashes
- [io](modules/io.md) - Core I/O tools
- [math](modules/math.md) - Mathematical functions
- [random](modules/random.md) - Random number generation

## 🎯 Quality Standards

All code in this book adheres to strict quality standards:

| Metric | Target | Current |
|--------|--------|---------|
| Cyclomatic Complexity | ≤10 per function | ✅ Passing |
| Test Coverage | ≥80% | ✅ 99.8% |
| SATD Count | 0 | ✅ 0 |
| Type Coverage | 100% for public APIs | ✅ Passing |

## 🐛 Edge Cases

Discover interesting edge cases and stdlib behaviors:

- [Edge Cases Overview](edge-cases/index.md)

## 🛠️ Development Workflow

### TDD Protocol

1. **Write Test First**: Create failing test
2. **Run Test**: Verify it fails
3. **Implement**: Make test pass
4. **Refactor**: Improve code quality
5. **Quality Check**: Run quality gates

### Example

```python
# tests/test_example/test_feature.py
def test_json_encode_unicode():
    """JSON should properly encode Unicode characters."""
    data = {"message": "Hello 世界"}
    result = json.dumps(data, ensure_ascii=False)
    assert result == '{"message": "Hello 世界"}'
```

## 📖 Project Goals

1. **Transpiler Validation**: Provide reference implementations for Depyler
2. **Edge Case Discovery**: Find and document stdlib corner cases
3. **Quality Assurance**: Demonstrate A+ code standards
4. **Educational Resource**: Teach TDD and Python stdlib

## 🤝 Contributing

Contributions are welcome! Please:

1. Follow the TDD protocol
2. Ensure all quality gates pass
3. Add comprehensive edge case tests
4. Document any stdlib bugs discovered

## 📄 License

MIT License - See the [main repository](https://github.com/paiml/depyler) for details.

## 🔗 Resources

- [Python Standard Library Docs](https://docs.python.org/3/library/)
- [Depyler Transpiler](https://github.com/paiml/depyler)
- [Hypothesis Testing Framework](https://hypothesis.readthedocs.io/)
- [PMAT Quality Tools](https://github.com/noahgift/pmat)
