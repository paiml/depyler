# Type Inference Hints in Depyler

## Overview

Depyler v2.0.0 introduces an intelligent type inference system that analyzes Python code to suggest appropriate types for parameters, return values, and variables that lack explicit type annotations.

## Features

### Usage Pattern Analysis
The system analyzes how variables are used to infer their likely types:
- **Numeric operations** (`+`, `-`, `*`, `/`) suggest numeric types
- **String methods** (`.upper()`, `.lower()`, `.strip()`) strongly indicate string type
- **List operations** (`.append()`, `.extend()`) suggest list type
- **Iterator usage** (`for x in y`) indicates container types
- **Boolean contexts** suggest bool type

### Confidence Levels
Type hints are assigned confidence levels based on the strength of evidence:
- **Certain**: Multiple strong indicators or explicit type conversions
- **High**: Clear method usage or multiple consistent operations
- **Medium**: Single type indicator or pattern
- **Low**: Minimal evidence or ambiguous usage

### Automatic Application
High-confidence type hints are automatically applied during transpilation:
- Parameter types are inferred from their usage in the function body
- Return types are inferred from explicit return statements
- Only hints with High or Certain confidence are applied automatically

## Example

```python
def process_text(text):
    """String methods suggest str type."""
    result = text.upper()
    return result
```

Transpiles to:
```rust
pub fn process_text<'a>(text: &'a str) -> DynamicType {
    let mut result = text.to_uppercase();
    return result;
}
```

## Implementation Details

### Type Hint Provider
The `TypeHintProvider` analyzes HIR functions and generates hints by:
1. Collecting usage patterns for each variable
2. Analyzing operations and method calls
3. Building type constraints from the evidence
4. Generating hints with appropriate confidence levels

### Integration
Type hints are integrated into the transpilation pipeline:
1. After HIR generation, functions are analyzed for type hints
2. Hints are displayed to stderr for user awareness
3. High-confidence hints are applied to the HIR
4. The modified HIR is then used for code generation

### Extensibility
The system is designed to be extended with:
- Additional usage patterns
- More sophisticated constraint solving
- Integration with external type information
- User-provided hints and overrides

## Benefits

1. **Better Performance**: Inferred concrete types avoid dynamic dispatch overhead
2. **Improved Safety**: Static types catch more errors at compile time
3. **Clearer Code**: Generated Rust code is more idiomatic with proper types
4. **Gradual Typing**: Works alongside explicit annotations, filling in gaps

## Future Enhancements

- Support for more complex type patterns (generics, unions)
- Integration with Python type stubs (.pyi files)
- Machine learning-based type prediction
- Cross-function type propagation
- IDE integration for interactive type hints