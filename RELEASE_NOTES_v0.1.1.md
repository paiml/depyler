# Depyler v0.1.1 Release Notes

## 🎉 Major Achievement: 100% V1.0 Transpilation Success Rate

This release marks a significant milestone with all V1.0 showcase examples now
transpiling successfully!

## ✨ New Features

### Enhanced Operator Support

- **Augmented Assignment Operators**: Full support for `+=`, `-=`, `*=`, `/=`,
  `%=`, etc.
- **Membership Operators**: Implemented `in` and `not in` operators for
  dictionary membership checks
- **Comprehensive Operator Coverage**: All arithmetic, comparison, logical, and
  bitwise operators now fully supported

### Testing Infrastructure

- **QuickCheck Integration**: Property-based testing framework for transpilation
  correctness
- **Operator Test Suite**: Comprehensive tests covering all supported operators
- **Property Tests**: Verification of type preservation, purity, and
  panic-freedom properties

### Code Quality Improvements

- **Reduced Complexity**: Refactored HirExpr::to_rust_expr from cyclomatic
  complexity 42 to <20
- **Cleaner AST Bridge**: Modularized expression and statement conversion with
  dedicated converters
- **Better Error Messages**: More informative error reporting for unsupported
  constructs

## 📊 Metrics

### Transpilation Success Rate

- **V1.0 Examples**: 4/4 (100%) ✅
  - binary_search.py ✅
  - calculate_sum.py ✅
  - classify_number.py ✅
  - process_config.py ✅

### Code Quality

- **Quality Score**: 75.0/100 (baseline for future improvements)
- **Test Coverage**: Comprehensive unit and property tests added
- **Complexity Reduction**: Major hotspots refactored

## 🔧 Technical Details

### Supported Python Constructs

- Functions with type annotations
- Basic control flow (if/elif/else, while, for)
- All arithmetic operators (+, -, *, /, %, //)
- All comparison operators (==, !=, <, <=, >, >=)
- Logical operators (and, or, not)
- Bitwise operators (&, |, ^, <<, >>)
- Augmented assignments (+=, -=, etc.)
- Membership tests (in, not in)
- Lists, dictionaries, tuples
- String operations

### Generated Rust Features

- Safe indexing with bounds checking
- HashMap for dictionary operations
- Idiomatic Rust patterns
- Zero unsafe code
- Clippy-clean output

## 🚀 Getting Started

```bash
# Install depyler
cargo install depyler

# Transpile Python to Rust
depyler transpile my_code.py -o my_code.rs

# Analyze complexity
depyler analyze my_code.py

# Verify transpilation
depyler check my_code.py --verify
```

## 📝 Examples

### Augmented Assignment

```python
def calculate_sum(numbers: List[int]) -> int:
    total: int = 0
    for n in numbers:
        total += n  # Now supported!
    return total
```

### Dictionary Membership

```python
def process_config(config: Dict[str, str]) -> Optional[str]:
    if "debug" in config:  # Now supported!
        return config["debug"]
    return None
```

## 🛠️ Breaking Changes

None - this release maintains backward compatibility with v0.1.0

## 🐛 Bug Fixes

- Fixed transpilation of augmented assignment operators
- Fixed dictionary membership test operators
- Improved handling of string literals in generated code

## 📚 Documentation

- Comprehensive MCP integration guide
- QA checklist for development
- Updated examples with new operators

## 🏗️ Infrastructure

- GitHub Actions CI achieving 100% pass rate
- Property-based testing with QuickCheck
- Automated code formatting and linting

## 🙏 Acknowledgments

Thanks to all contributors and testers who helped achieve this milestone!

## 📮 Feedback

Please report issues at: https://github.com/paiml/depyler/issues

---

_Depyler - Making Python and Rust work together, one function at a time._
