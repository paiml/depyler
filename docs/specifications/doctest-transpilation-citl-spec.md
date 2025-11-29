# Doctest Transpilation for CITL Training

**Specification Version:** 1.0.0
**Status:** Draft
**Authors:** Depyler Team
**Created:** 2025-11-29
**References:** GH-172, metaheuristic-oracle-spec.md

## Abstract

This specification defines the transpilation of Python doctests (`>>>`) to Rust doc tests (`/// ````) as a high-fidelity training signal for Compiler-in-the-Loop (CITL) training. Unlike synthetic data or compile-only validation, doctest transpilation provides **semantic equivalence proof**—a passing doc test proves the transpiled code produces identical outputs to the original Python.

## 1. Motivation

### 1.1 Training Signal Hierarchy

| Signal | Validates | Fidelity | Cost |
|--------|-----------|----------|------|
| `rustc` exit code | Compiles | Low | Free |
| `rustc` error message | Type/syntax correct | Medium | Free |
| `cargo test --doc` compile | Doc test syntax | High | Free |
| **`cargo test --doc` pass** | **Semantic equivalence** | **Highest** | **Free** |
| Runtime comparison | Behavioral equivalence | Highest | Expensive |

### 1.2 Why Doctests?

1. **Already written**: Python stdlib, numpy, pandas have millions of doctest examples
2. **Human-verified**: Documentation is reviewed; synthetic data is not
3. **Micro-granular**: One function, one I/O pair, one ground truth
4. **Type oracle**: `fibonacci(10) → 55` implies return type is integer
5. **Free corpus**: No generation cost—mine existing docstrings

### 1.3 Academic Foundation

> **Plastic Surgery Hypothesis** (Barr et al., FSE 2014): "The raw materials for a patch are often already present in the codebase."

Doctest transpilation extends this hypothesis: the raw materials for **verifying** a transpilation are already present in the docstring.

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                   Doctest CITL Pipeline                          │
└─────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Python     │    │   Doctest    │    │   Depyler    │
│   Source     │ →  │  Extractor   │ →  │  Transpile   │
│  + Docstring │    │  (>>> lines) │    │  (fn + docs) │
└──────────────┘    └──────────────┘    └──────────────┘
                           │                   │
                           ▼                   ▼
                    ┌──────────────┐    ┌──────────────┐
                    │   I/O Pairs  │    │  Rust Code   │
                    │  (input,out) │    │  + Doc Tests │
                    └──────────────┘    └──────────────┘
                           │                   │
                           └─────────┬─────────┘
                                     ▼
                           ┌──────────────────┐
                           │ cargo test --doc │
                           └────────┬─────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
              ┌─────────┐    ┌─────────┐    ┌─────────┐
              │  PASS   │    │ COMPILE │    │ RUNTIME │
              │(correct)│    │  FAIL   │    │  FAIL   │
              └─────────┘    └─────────┘    └─────────┘
                    │               │               │
                    ▼               ▼               ▼
              ┌─────────┐    ┌─────────┐    ┌─────────┐
              │ +1 for  │    │ Type or │    │ Semantic│
              │ Oracle  │    │ syntax  │    │ bug in  │
              │ success │    │ error   │    │transpile│
              └─────────┘    └─────────┘    └─────────┘
```

## 3. Implementation

### 3.1 Doctest Extraction

```rust
use rustpython_parser::ast;

/// Represents a single Python doctest example
#[derive(Debug, Clone)]
pub struct PythonDoctest {
    /// The expression to evaluate (e.g., "fibonacci(10)")
    pub input: String,
    /// The expected output (e.g., "55")
    pub expected: String,
    /// Line number in original docstring
    pub line: usize,
    /// Whether this is a continuation (...)
    pub is_continuation: bool,
}

/// Extract doctests from a Python docstring
pub fn extract_doctests(docstring: &str) -> Vec<PythonDoctest> {
    let mut doctests = Vec::new();
    let mut current = None;

    for (line_num, line) in docstring.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with(">>>") {
            // Save previous doctest if any
            if let Some(dt) = current.take() {
                doctests.push(dt);
            }
            // Start new doctest
            current = Some(PythonDoctest {
                input: trimmed[4..].trim().to_string(),
                expected: String::new(),
                line: line_num,
                is_continuation: false,
            });
        } else if trimmed.starts_with("...") {
            // Continuation line
            if let Some(ref mut dt) = current {
                dt.input.push('\n');
                dt.input.push_str(trimmed[4..].trim());
                dt.is_continuation = true;
            }
        } else if let Some(ref mut dt) = current {
            // Output line
            if !trimmed.is_empty() {
                if !dt.expected.is_empty() {
                    dt.expected.push('\n');
                }
                dt.expected.push_str(trimmed);
            }
        }
    }

    if let Some(dt) = current {
        doctests.push(dt);
    }

    doctests
}
```

### 3.2 Rust Doc Test Generation

```rust
/// Transpile Python doctest to Rust doc test
pub fn transpile_doctest(
    doctest: &PythonDoctest,
    fn_name: &str,
    module_path: &str,
) -> Result<String, TranspileError> {
    // Transpile the input expression
    let rust_input = transpile_expression(&doctest.input)?;

    // Transpile the expected output
    let rust_expected = transpile_literal(&doctest.expected)?;

    Ok(format!(
        "/// ```\n\
         /// use {}::{};\n\
         /// assert_eq!({}, {});\n\
         /// ```",
        module_path, fn_name, rust_input, rust_expected
    ))
}

/// Generate complete Rust function with doc tests
pub fn generate_with_doctests(
    python_fn: &PythonFunction,
    module_path: &str,
) -> Result<String, TranspileError> {
    let mut doc_lines = Vec::new();

    // Original docstring description
    if let Some(desc) = &python_fn.description {
        doc_lines.push(format!("/// {}", desc));
        doc_lines.push("///".to_string());
    }

    // Extract and transpile doctests
    if let Some(docstring) = &python_fn.docstring {
        let doctests = extract_doctests(docstring);

        if !doctests.is_empty() {
            doc_lines.push("/// # Examples".to_string());
            doc_lines.push("///".to_string());
            doc_lines.push("/// ```".to_string());
            doc_lines.push(format!("/// use {}::{};", module_path, python_fn.name));

            for dt in doctests {
                let rust_input = transpile_expression(&dt.input)?;
                let rust_expected = transpile_literal(&dt.expected)?;
                doc_lines.push(format!(
                    "/// assert_eq!({}, {});",
                    rust_input, rust_expected
                ));
            }

            doc_lines.push("/// ```".to_string());
        }
    }

    // Generate function body
    let rust_body = transpile_function_body(python_fn)?;

    Ok(format!(
        "{}\n{}",
        doc_lines.join("\n"),
        rust_body
    ))
}
```

### 3.3 CITL Training Integration

```rust
/// Result of doctest transpilation attempt
#[derive(Debug)]
pub enum DoctestResult {
    /// Doc test passed - semantic equivalence proven
    Pass {
        python_input: String,
        rust_input: String,
        output: String,
    },
    /// Doc test compiled but failed at runtime
    RuntimeFail {
        python_input: String,
        rust_input: String,
        expected: String,
        actual: String,
    },
    /// Doc test failed to compile
    CompileFail {
        error_code: String,
        error_message: String,
        rust_code: String,
    },
    /// Could not transpile doctest expression
    TranspileFail {
        python_input: String,
        error: String,
    },
}

/// Training signal from doctest result
impl DoctestResult {
    pub fn to_training_signal(&self) -> TrainingSignal {
        match self {
            DoctestResult::Pass { .. } => TrainingSignal {
                category: SignalCategory::SemanticEquivalence,
                confidence: 1.0,
                label: Label::Correct,
            },
            DoctestResult::RuntimeFail { expected, actual, .. } => TrainingSignal {
                category: SignalCategory::SemanticDivergence,
                confidence: 0.95,
                label: Label::SemanticBug {
                    expected: expected.clone(),
                    actual: actual.clone(),
                },
            },
            DoctestResult::CompileFail { error_code, error_message, .. } => TrainingSignal {
                category: SignalCategory::CompileError,
                confidence: 0.9,
                label: Label::from_rustc_error(error_code, error_message),
            },
            DoctestResult::TranspileFail { .. } => TrainingSignal {
                category: SignalCategory::TranspileGap,
                confidence: 0.8,
                label: Label::UnsupportedConstruct,
            },
        }
    }
}
```

## 4. Training Value Analysis

### 4.1 Signal Strength Comparison

| Source | Samples | Verified | Signal Strength |
|--------|---------|----------|-----------------|
| Synthetic templates | 30,000 | No | Low |
| Manual error fixes | 200 | Yes | High |
| rustc compile errors | 10,000+ | Partial | Medium |
| **Doctest pass** | **10,000+** | **Yes** | **Highest** |

### 4.2 Type Inference Value

Doctests provide ground-truth type information:

```python
>>> len("hello")
5
>>> len([1, 2, 3])
3
>>> 3.14 * 2
6.28
```

Implies:
- `len(&str) -> usize`
- `len(&[T]) -> usize`
- `f64 * i32 -> f64`

This directly trains the type mapper without synthetic examples.

### 4.3 Corpus Size Estimates

| Source | Est. Doctests | Coverage |
|--------|---------------|----------|
| Python stdlib | 5,000+ | Core types, I/O |
| NumPy | 3,000+ | Array ops |
| Pandas | 5,000+ | DataFrame ops |
| Requests | 500+ | HTTP patterns |
| **Total** | **13,500+** | **Broad** |

## 5. Failure Modes and Mitigations

### 5.1 Expression Transpilation Failure

**Problem**: Python doctest uses syntax we can't transpile.

```python
>>> {x: x**2 for x in range(5)}
{0: 0, 1: 1, 2: 4, 3: 9, 4: 16}
```

**Mitigation**: Skip and log for future support. Track unsupported constructs.

### 5.2 Output Format Mismatch

**Problem**: Python repr differs from Rust Debug.

```python
>>> repr([1, 2, 3])
'[1, 2, 3]'
```

```rust
format!("{:?}", vec![1, 2, 3])  // "[1, 2, 3]"
```

**Mitigation**: Normalize output comparison (strip quotes, whitespace).

### 5.3 Side Effects

**Problem**: Doctest has side effects we can't replicate.

```python
>>> import os
>>> os.getcwd()
'/home/user'
```

**Mitigation**: Detect impure functions, skip or mock.

## 6. Metrics

### 6.1 Doctest CITL Dashboard

| Metric | Target | Current |
|--------|--------|---------|
| Doctests extracted | 10,000+ | - |
| Transpile success rate | 70% | - |
| Compile success rate | 50% | - |
| **Doc test pass rate** | **30%** | - |
| Semantic equivalence | 90%+ of passing | - |

### 6.2 Oracle Improvement Targets

| Metric | Before Doctest | After Doctest |
|--------|----------------|---------------|
| K-fold CV Accuracy | 91% | 95%+ |
| E0308 Classification | 85% | 92%+ |
| Novel Error Discovery | - | +500 patterns |

## 7. Roadmap

### Phase 1: Extraction (Week 1)
- [ ] Implement `extract_doctests()` parser
- [ ] Handle multi-line doctests (`...`)
- [ ] Test on Python stdlib

### Phase 2: Transpilation (Week 2)
- [ ] Transpile simple expressions
- [ ] Transpile literal outputs
- [ ] Generate Rust doc test syntax

### Phase 3: Validation (Week 3)
- [ ] Run `cargo test --doc` on transpiled code
- [ ] Collect pass/fail/error signals
- [ ] Feed into Oracle training

### Phase 4: Integration (Week 4)
- [ ] Add to CITL pipeline
- [ ] Dashboard metrics
- [ ] A/B test Oracle with/without doctest signals

## 8. References

1. Barr, E. T., et al. (2014). The Plastic Surgery Hypothesis. *FSE '14*.
2. Gupta, R., et al. (2017). DeepFix: Fixing Common C Language Errors by Deep Learning. *AAAI*.
3. Python Documentation. doctest — Test interactive Python examples.
4. Rust Documentation. Documentation tests.

---

*This specification extends metaheuristic-oracle-spec.md Section 3.2.1.*
