# Tractable External Library Mapping Specification

**Version**: 1.1.0
**Status**: DRAFT - Awaiting Team Review (Updated per Gemini Code Review)
**Ticket**: DEPYLER-0903
**Authors**: Depyler Team
**Date**: 2025-12-10
**Revision**: 2025-12-10 - Added argument reordering, version decoupling, 10 additional citations

## Abstract

This specification defines a deterministic, extensible system for mapping Python external libraries to Rust equivalents. The system requires **zero machine learning** because the problem is fundamentally a finite dictionary lookup with known, enumerable mappings. We provide extension points for enterprise customization (Netflix, Google, Amazon, etc.) while maintaining mathematical guarantees of correctness.

## 1. Problem Classification

### 1.1 Computational Complexity

The external library mapping problem belongs to the complexity class **O(1)** for lookup and **O(n)** for registration, where n is the number of mappings. This is a **solved problem** in computer science, requiring only:

1. A hash table for O(1) amortized lookup [1]
2. A plugin architecture for extensibility [2]
3. A verification oracle for correctness [3]

### 1.2 Why Zero ML is Required

| Criterion | ML Requirement | This Problem | Citation |
|-----------|---------------|--------------|----------|
| Unstructured input | Required | No - AST is structured | [4] |
| Probabilistic output | Required | No - deterministic | [5] |
| Training data | Required | No - finite enumeration | [6] |
| Generalization | Required | No - closed domain | [7] |
| Ambiguity resolution | Required | No - 1:1 mappings | [8] |

Per Knuth's fundamental theorem on algorithm selection: "Premature optimization is the root of all evil" [9]. Applying ML to a lookup table problem is not premature optimization—it is **categorical misapplication**.

### 1.3 Toyota Way Alignment

This specification adheres to Toyota Production System principles [10]:

| Principle | Application |
|-----------|-------------|
| **Jidoka** (Autonomation) | Compile-time verification catches mapping errors |
| **Poka-Yoke** (Error-proofing) | Type system prevents invalid mappings |
| **Genchi Genbutsu** (Go and see) | Mappings derived from actual library documentation |
| **Kaizen** (Continuous improvement) | Plugin system enables incremental enhancement |
| **Heijunka** (Level loading) | Lazy evaluation prevents startup overhead |

## 2. Architecture

### 2.1 Core Data Structure

```rust
/// A deterministic mapping from Python library to Rust equivalent.
///
/// This is a pure function: f(python_module, python_item) → rust_equivalent
/// No randomness, no learning, no approximation.
///
/// Design follows Parnas's information hiding principle [26]: internal
/// representation can change without affecting clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMapping {
    /// Python module path (e.g., "pandas", "numpy.linalg")
    pub python_module: String,

    /// Rust crate and module path (e.g., "polars", "ndarray::linalg")
    pub rust_crate: String,

    /// Python version requirement (e.g., ">=3.8" or "*")
    /// Decoupled from Rust version per API evolution principles [30, 31]
    pub python_version_req: String,

    /// Rust crate version constraint (semver, e.g., "1.0", ">=2.0,<3.0")
    /// Independent of Python version for proper dependency management
    pub rust_crate_version: String,

    /// Item-level mappings: Python name → Rust name
    pub items: HashMap<String, ItemMapping>,

    /// Required Cargo.toml features
    pub features: Vec<String>,

    /// Mapping confidence: Verified, Community, Experimental
    pub confidence: MappingConfidence,

    /// Source of mapping (documentation URL, RFC, etc.)
    pub provenance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMapping {
    /// Rust equivalent name
    pub rust_name: String,

    /// Transformation pattern
    pub pattern: TransformPattern,

    /// Type signature transformation
    pub type_transform: Option<TypeTransform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformPattern {
    /// Direct 1:1 rename: pandas.DataFrame → polars::DataFrame
    Direct,

    /// Method call transformation: df.head() → df.head(None)
    MethodCall { extra_args: Vec<String> },

    /// Property to method: df.shape → df.shape()
    PropertyToMethod,

    /// Constructor pattern: DataFrame() → DataFrame::new()
    Constructor { method: String },

    /// Argument reordering: Python args[i] → Rust args[indices[i]]
    /// Example: Python subprocess.run(cmd, cwd, env) → Rust Command::new(cmd).current_dir(cwd).envs(env)
    /// indices = [0, 2, 1] means: Rust arg 0 = Python arg 0, Rust arg 1 = Python arg 2, etc.
    /// This is a pure permutation - O(n) application, verified at registration time [27]
    ReorderArgs { indices: Vec<usize> },

    /// Typed template with named parameters (Poka-Yoke: type-safe templates [33])
    /// Instead of stringly-typed "{var}.method()", use structured parameters
    /// that can be validated at registration time.
    ///
    /// Example: TypedTemplate for boto3.s3.upload_file:
    /// ```
    /// TypedTemplate {
    ///     pattern: "{client}.put_object().bucket({bucket}).key({key}).body({body}).send().await?",
    ///     params: ["client", "bucket", "key", "body"],
    ///     param_types: [Expr, String, String, Bytes],
    /// }
    /// ```
    TypedTemplate {
        /// Template string with {param_name} placeholders
        pattern: String,
        /// Ordered list of parameter names (must match placeholders)
        params: Vec<String>,
        /// Expected type for each parameter (for validation)
        param_types: Vec<ParamType>,
    },

    /// Legacy: Custom template with {var} placeholders (deprecated, use TypedTemplate)
    #[deprecated(note = "Use TypedTemplate for type-safe templates")]
    Template { template: String },
}

/// Parameter types for TypedTemplate validation (Poka-Yoke principle [33])
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParamType {
    /// Any expression
    Expr,
    /// String literal or variable
    String,
    /// Numeric literal or variable
    Number,
    /// Bytes/binary data
    Bytes,
    /// Boolean
    Bool,
    /// Path (filesystem)
    Path,
    /// List/Vec
    List,
    /// Dict/HashMap
    Dict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MappingConfidence {
    /// Verified against official documentation and tests
    Verified,
    /// Community-contributed, reviewed but not exhaustively tested
    Community,
    /// Experimental, may have edge cases
    Experimental,
}
```

### 2.2 Registry Architecture

```rust
/// The mapping registry uses a two-level hash table for O(1) lookup.
///
/// Level 1: Module → ModuleMappings
/// Level 2: Item → ItemMapping
///
/// Total lookup: O(1) amortized [1]
pub struct MappingRegistry {
    /// Core mappings (shipped with depyler)
    core: HashMap<String, LibraryMapping>,

    /// Enterprise extensions (loaded from plugins)
    extensions: HashMap<String, LibraryMapping>,

    /// User overrides (highest priority)
    overrides: HashMap<String, LibraryMapping>,
}

impl MappingRegistry {
    /// Lookup with priority: overrides > extensions > core
    ///
    /// This is a pure function with no side effects.
    /// Complexity: O(1) amortized
    pub fn lookup(&self, module: &str, item: &str) -> Option<&ItemMapping> {
        // Priority chain: user > enterprise > core
        self.overrides.get(module)
            .or_else(|| self.extensions.get(module))
            .or_else(|| self.core.get(module))
            .and_then(|m| m.items.get(item))
    }
}
```

### 2.3 Plugin Architecture for Enterprise Extension

```rust
/// Enterprise plugin interface for custom library mappings.
///
/// Example: Netflix might map their internal Python libraries
/// to their internal Rust equivalents.
pub trait MappingPlugin: Send + Sync {
    /// Plugin identifier (e.g., "netflix-internal", "google-cloud")
    fn id(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Register mappings into the registry
    fn register(&self, registry: &mut MappingRegistry);

    /// Optional: Validate that mappings are correct
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// File-based plugin loading from TOML configuration
///
/// Example: ~/.depyler/plugins/netflix-mappings.toml
///
/// ```toml
/// [plugin]
/// id = "netflix-internal"
/// version = "1.0.0"
///
/// [[mappings]]
/// python_module = "netflix.featurestore"
/// rust_crate = "netflix_featurestore_rs"
/// python_version_req = ">=3.9"      # Requires Python 3.9+
/// rust_crate_version = "2.0"        # Rust crate version (independent)
///
/// [mappings.items]
/// FeatureStore = { rust_name = "FeatureStore", pattern = "Direct" }
/// get_features = { rust_name = "get_features", pattern = "MethodCall", extra_args = [] }
/// ```
pub struct TomlPlugin {
    config: TomlPluginConfig,
}
```

## 3. Standard Library Mappings (Built-in)

### 3.1 Tier 1: Python Standard Library (100% Coverage Target)

| Python Module | Rust Equivalent | Confidence | Citation |
|---------------|-----------------|------------|----------|
| `json` | `serde_json` | Verified | [11] |
| `os` | `std::{env, fs}` | Verified | [12] |
| `os.path` | `std::path` | Verified | [12] |
| `sys` | `std::{env, process}` | Verified | [12] |
| `re` | `regex` | Verified | [13] |
| `datetime` | `chrono` | Verified | [14] |
| `collections` | `std::collections` | Verified | [12] |
| `itertools` | `itertools` | Verified | [15] |
| `typing` | (type system) | Verified | [16] |
| `argparse` | `clap` | Verified | [17] |
| `subprocess` | `std::process` | Verified | [12] |
| `pathlib` | `std::path::PathBuf` | Verified | [12] |
| `math` | `std::f64` + `libm` | Verified | [12] |
| `random` | `rand` | Verified | [18] |
| `hashlib` | `sha2`, `md5`, etc. | Verified | [19] |
| `uuid` | `uuid` | Verified | [20] |
| `csv` | `csv` | Verified | [21] |
| `io` | `std::io` | Verified | [12] |
| `threading` | `std::thread` | Verified | [12] |
| `asyncio` | `tokio` | Verified | [22] |

### 3.2 Tier 2: Popular Third-Party Libraries

| Python Library | Rust Equivalent | Confidence | Citation |
|----------------|-----------------|------------|----------|
| `numpy` | `ndarray` | Community | [23] |
| `pandas` | `polars` | Community | [24] |
| `requests` | `reqwest` | Verified | [25] |
| `sklearn` | `linfa` / `smartcore` | Experimental | - |

## 4. Configuration Format

### 4.1 User-Level Configuration

Location: `~/.depyler/mappings.toml` or `$DEPYLER_MAPPINGS`

```toml
# User-specific mapping overrides
# These take highest priority

[[mappings]]
python_module = "my_company.utils"
rust_crate = "my_company_utils"
python_version_req = "*"           # Any Python version
rust_crate_version = "1.0"         # Rust crate version
confidence = "Verified"
provenance = "https://internal-docs.mycompany.com/rust-migration"

[mappings.items]
parse_config = { rust_name = "parse_config", pattern = "Direct" }
validate = { rust_name = "validate", pattern = "MethodCall", extra_args = ["true"] }
# Example with argument reordering: Python func(a, b, c) → Rust func(c, a, b)
reordered_func = { rust_name = "reordered_func", pattern = "ReorderArgs", indices = [2, 0, 1] }
```

### 4.2 Project-Level Configuration

Location: `depyler.toml` in project root

```toml
[mappings]
# Include enterprise plugins
plugins = ["netflix-internal", "google-cloud"]

# Project-specific overrides
[[mappings.overrides]]
python_module = "legacy_module"
rust_crate = "new_module"
python_version_req = ">=3.7,<3.12"  # Python 3.7-3.11 only
rust_crate_version = "2.0"
```

### 4.3 Enterprise Plugin Distribution

Enterprises can distribute mapping plugins via:

1. **Cargo crates**: `cargo install depyler-plugin-netflix`
2. **TOML files**: Shared via internal package managers
3. **Git repositories**: `depyler plugin add https://github.com/netflix/depyler-mappings`

## 5. Verification Oracle

### 5.1 Compile-Time Verification

Every mapping is verified by attempting compilation:

```rust
/// Verification oracle: Does the mapping produce compiling Rust?
///
/// This is a decidable problem [3] - we get a definitive yes/no answer.
pub fn verify_mapping(mapping: &LibraryMapping) -> VerificationResult {
    // 1. Generate minimal Rust code using the mapping
    let test_code = generate_verification_code(mapping);

    // 2. Attempt compilation with rustc
    let result = compile_check(&test_code, &mapping.rust_crate, &mapping.version);

    // 3. Return deterministic result
    match result {
        Ok(_) => VerificationResult::Valid,
        Err(e) => VerificationResult::Invalid { error: e },
    }
}
```

### 5.2 Semantic Equivalence Testing (Optional)

For high-confidence mappings, we can verify semantic equivalence:

```rust
/// Golden test: Does Python output == Rust output?
///
/// This uses the same approach as compiler validation [3].
pub fn verify_semantic_equivalence(
    python_code: &str,
    rust_code: &str,
    test_inputs: &[TestInput],
) -> EquivalenceResult {
    for input in test_inputs {
        let python_output = run_python(python_code, input);
        let rust_output = run_rust(rust_code, input);

        if python_output != rust_output {
            return EquivalenceResult::Divergent { input, python_output, rust_output };
        }
    }
    EquivalenceResult::Equivalent
}
```

## 6. Extension Points for Enterprise

### 6.1 Netflix Example

```toml
# netflix-mappings.toml
[plugin]
id = "netflix-internal"
version = "1.0.0"
maintainer = "platform-team@netflix.com"

# Netflix's internal feature store
[[mappings]]
python_module = "netflix.featurestore"
rust_crate = "nf_featurestore"
python_version_req = ">=3.9"       # Requires Python 3.9+
rust_crate_version = "3.0"         # Internal Rust crate version
features = ["async"]
confidence = "Verified"
provenance = "https://docs.netflix.internal/featurestore/rust"

[mappings.items]
FeatureStore = { rust_name = "FeatureStore", pattern = "Direct" }
get_features = { rust_name = "fetch_features", pattern = "MethodCall", extra_args = [] }
# TypedTemplate example: type-safe template with validation
batch_get = { rust_name = "batch_fetch", pattern = "TypedTemplate", pattern_str = "{client}.batch_fetch(&{ids}).await?", params = ["client", "ids"], param_types = ["Expr", "List"] }

# Netflix's A/B testing framework
[[mappings]]
python_module = "netflix.abtest"
rust_crate = "nf_experimentation"
python_version_req = ">=3.8"       # Supports older Python
rust_crate_version = "2.5"
```

### 6.2 Google Example

```toml
# google-cloud-mappings.toml
[plugin]
id = "google-cloud"
version = "1.0.0"
maintainer = "cloud-sdk@google.com"

[[mappings]]
python_module = "google.cloud.bigquery"
rust_crate = "google_cloud_bigquery"
python_version_req = ">=3.7"       # GCP SDK Python requirement
rust_crate_version = "0.5"         # Rust SDK version
features = ["rustls"]
confidence = "Community"
provenance = "https://github.com/googleapis/google-cloud-rust"

[mappings.items]
Client = { rust_name = "Client", pattern = "Constructor", method = "new" }
QueryJob = { rust_name = "QueryJob", pattern = "Direct" }
# ReorderArgs example: Python query(sql, params, job_config) → Rust query(job_config, sql, params)
query = { rust_name = "query", pattern = "ReorderArgs", indices = [2, 0, 1] }
```

### 6.3 Amazon Example

```toml
# aws-mappings.toml
[plugin]
id = "aws-sdk"
version = "1.0.0"
maintainer = "sdk-team@amazon.com"

[[mappings]]
python_module = "boto3"
rust_crate = "aws_sdk"
python_version_req = ">=3.8"       # AWS SDK Python requirement
rust_crate_version = "1.0"         # AWS SDK for Rust version
confidence = "Verified"
provenance = "https://docs.aws.amazon.com/sdk-for-rust/"

# S3 client
[[mappings]]
python_module = "boto3.s3"
rust_crate = "aws_sdk_s3"
python_version_req = ">=3.8"
rust_crate_version = "1.0"

[mappings.items]
# TypedTemplate with full type validation (Poka-Yoke)
upload_file = { rust_name = "put_object", pattern = "TypedTemplate", pattern_str = "{client}.put_object().bucket({bucket}).key({key}).body({body}).send().await?", params = ["client", "bucket", "key", "body"], param_types = ["Expr", "String", "String", "Bytes"] }
download_file = { rust_name = "get_object", pattern = "TypedTemplate", pattern_str = "{client}.get_object().bucket({bucket}).key({key}).send().await?", params = ["client", "bucket", "key"], param_types = ["Expr", "String", "String"] }
```

## 7. CLI Interface

```bash
# List all registered mappings
depyler mappings list

# Show mapping for a specific module
depyler mappings show pandas

# Add a plugin
depyler mappings plugin add netflix-internal

# Verify all mappings compile
depyler mappings verify

# Export mappings to TOML
depyler mappings export > my-mappings.toml

# Import custom mappings
depyler mappings import ./company-mappings.toml
```

## 8. Implementation Complexity

| Component | Lines of Code | Complexity | Time to Implement |
|-----------|---------------|------------|-------------------|
| Core data structures | ~200 | O(1) lookup | 2 hours |
| TOML parser | ~150 | O(n) parse | 2 hours |
| Plugin loader | ~100 | O(k) plugins | 2 hours |
| Verification oracle | ~100 | O(1) per mapping | 2 hours |
| CLI commands | ~200 | O(1) per command | 2 hours |
| **Total** | **~750** | **O(1) runtime** | **10 hours** |

This is a **weekend project**, not a research initiative.

## 9. Non-Goals

This specification explicitly does **NOT** include:

1. **Machine learning** - The problem is deterministic
2. **Fuzzy matching** - Mappings are exact or absent
3. **Automatic inference** - Mappings are human-curated
4. **Natural language processing** - Input is structured AST
5. **Neural networks** - A hash table suffices
6. **Training data** - Finite enumeration replaces learning
7. **Approximate solutions** - Either it compiles or it doesn't

## 10. References

### Core Computer Science (Algorithms & Data Structures)

[1] Cormen, T.H., Leiserson, C.E., Rivest, R.L., & Stein, C. (2009). *Introduction to Algorithms* (3rd ed.). MIT Press. Chapter 11: Hash Tables. ISBN: 978-0-262-03384-8.

[27] Fredman, M.L., Komlós, J., & Szemerédi, E. (1984). "Storing a Sparse Table with O(1) Worst Case Access Time". *Journal of the ACM*, 31(3), 538-544. doi:10.1145/828.1884. **[Proves O(1) lookup is achievable for static dictionaries - foundational for our hash table claims]**

### Software Architecture & Design

[2] Gamma, E., Helm, R., Johnson, R., & Vlissides, J. (1994). *Design Patterns: Elements of Reusable Object-Oriented Software*. Addison-Wesley. Plugin Pattern. ISBN: 0-201-63361-2.

[26] Parnas, D.L. (1972). "On the Criteria To Be Used in Decomposing Systems into Modules". *Communications of the ACM*, 15(12), 1053-1058. doi:10.1145/361598.361623. **[Information hiding principle - foundational for our modular architecture]**

[35] Brooks, F.P. (1987). "No Silver Bullet: Essence and Accidents of Software Engineering". *Computer*, 20(4), 10-19. doi:10.1109/MC.1987.1663532. **[Why deterministic solutions beat heuristics for essential complexity]**

### Compilers & Program Analysis

[3] Aho, A.V., Lam, M.S., Sethi, R., & Ullman, J.D. (2006). *Compilers: Principles, Techniques, and Tools* (2nd ed.). Pearson. Chapter 4: Syntax Analysis. ISBN: 0-321-48681-1.

[28] Cousot, P., & Cousot, R. (1977). "Abstract Interpretation: A Unified Lattice Model for Static Analysis of Programs by Construction or Approximation of Fixpoints". *POPL '77*, 238-252. doi:10.1145/512950.512973. **[Formal verification basis for compile-time mapping validation]**

### Type Systems & Formal Methods

[32] Cardelli, L., & Wegner, P. (1985). "On Understanding Types, Data Abstraction, and Polymorphism". *Computing Surveys*, 17(4), 471-523. doi:10.1145/6041.6042. **[Type system foundations for Poka-Yoke error prevention]**

[33] Hoare, C.A.R. (1969). "An Axiomatic Basis for Computer Programming". *Communications of the ACM*, 12(10), 576-580. doi:10.1145/363235.363259. **[Preconditions/postconditions basis for mapping contracts]**

[34] Liskov, B.H., & Wing, J.M. (1994). "A Behavioral Notion of Subtyping". *ACM Transactions on Programming Languages and Systems*, 16(6), 1811-1841. doi:10.1145/197320.197383. **[Behavioral subtyping - semantic equivalence guarantees]**

### API Evolution & Migration

[30] Dig, D., & Johnson, R. (2005). "How do APIs Evolve? A Story of Refactoring". *Journal of Software Maintenance and Evolution*, 18(2), 83-107. doi:10.1002/smr.328. **[API evolution patterns - justifies version decoupling]**

[31] Henkel, J., & Diwan, A. (2005). "CatchUp! Capturing and Replaying Refactorings to Support API Evolution". *ICSE '05*, 274-283. doi:10.1145/1062455.1062512. **[Automated API migration - ReorderArgs pattern basis]**

### Software Quality & Inspection

[36] Fagan, M.E. (1976). "Design and Code Inspections to Reduce Errors in Program Development". *IBM Systems Journal*, 15(3), 182-211. doi:10.1147/sj.153.0182. **[Code inspection methodology - verification oracle justification]**

### Machine Learning (Why Not Applicable)

[4] Mitchell, T.M. (1997). *Machine Learning*. McGraw-Hill. Chapter 1: Introduction. ISBN: 0-07-042807-7.

[5] Bishop, C.M. (2006). *Pattern Recognition and Machine Learning*. Springer. Chapter 1.2: Probability Theory. ISBN: 978-0-387-31073-2.

[6] Russell, S., & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.). Pearson. Chapter 19: Learning from Examples. ISBN: 978-0-13-461099-3.

[7] Vapnik, V.N. (1998). *Statistical Learning Theory*. Wiley. Chapter 1: Setting of the Learning Problem. ISBN: 0-471-03003-1.

[8] Jurafsky, D., & Martin, J.H. (2023). *Speech and Language Processing* (3rd ed.). Chapter 8: Sequence Labeling. https://web.stanford.edu/~jurafsky/slp3/

### Toyota Production System

[9] Knuth, D.E. (1974). "Structured Programming with go to Statements". *ACM Computing Surveys*, 6(4), 261-301. doi:10.1145/356635.356640.

[10] Liker, J.K. (2004). *The Toyota Way: 14 Management Principles from the World's Greatest Manufacturer*. McGraw-Hill. ISBN: 0-07-139231-9.

### Rust Crate Documentation

[11] Serde Contributors. (2023). "serde_json - JSON Serialization". https://docs.rs/serde_json/

[12] Rust Team. (2023). *The Rust Standard Library*. https://doc.rust-lang.org/std/

[13] Gallant, A. (2023). "regex - Regular Expressions for Rust". https://docs.rs/regex/

[14] Kang, B. (2023). "chrono - Date and Time Library". https://docs.rs/chrono/

[15] Bluss. (2023). "itertools - Extra Iterator Adaptors". https://docs.rs/itertools/

[16] van Rossum, G., et al. (2014). "PEP 484 - Type Hints". https://peps.python.org/pep-0484/

[17] Clap Contributors. (2023). "clap - Command Line Argument Parser". https://docs.rs/clap/

[18] Rust Random Contributors. (2023). "rand - Random Number Generation". https://docs.rs/rand/

[19] RustCrypto Contributors. (2023). "sha2 - SHA-2 Hash Functions". https://docs.rs/sha2/

[20] UUID Contributors. (2023). "uuid - UUID Generation". https://docs.rs/uuid/

[21] Gallant, A. (2023). "csv - CSV Parsing and Writing". https://docs.rs/csv/

[22] Tokio Contributors. (2023). "tokio - Asynchronous Runtime". https://docs.rs/tokio/

[23] Bluss, et al. (2023). "ndarray - N-dimensional Array". https://docs.rs/ndarray/

[24] Polars Contributors. (2023). "polars - DataFrame Library". https://docs.rs/polars/

[25] Crichton, S. (2023). "reqwest - HTTP Client". https://docs.rs/reqwest/

## 11. Technical Gaps Addressed (Per Code Review)

This section documents technical improvements made based on architectural review feedback.

### 11.1 Argument Reordering Problem

**Problem**: Python and Rust APIs often have different argument orders. Example:
- Python: `subprocess.run(cmd, cwd=None, env=None)`
- Rust: `Command::new(cmd).current_dir(cwd).envs(env)`

**Solution**: Added `ReorderArgs { indices: Vec<usize> }` pattern [31]:
```rust
// indices = [0, 2, 1] means:
// - Rust arg 0 = Python arg 0
// - Rust arg 1 = Python arg 2
// - Rust arg 2 = Python arg 1
ReorderArgs { indices: vec![0, 2, 1] }
```

**Verification**: Indices are validated at registration time (O(n) permutation check).

### 11.2 Version Coupling Problem

**Problem**: Single `version` field conflated Python package version with Rust crate version.
- Python `pandas==1.5.0` might map to Rust `polars==0.35.0`
- These version numbers are independent

**Solution**: Decoupled into two fields [30]:
```rust
pub python_version_req: String,    // e.g., ">=3.8"
pub rust_crate_version: String,    // e.g., "1.0"
```

**Benefit**: Enables proper semantic versioning for both ecosystems independently.

### 11.3 Stringly-Typed Templates (Poka-Yoke)

**Problem**: Original `Template { template: String }` was error-prone:
- No validation of placeholder names
- No type checking of parameters
- Errors only discovered at runtime

**Solution**: Added `TypedTemplate` with compile-time validation [32, 33]:
```rust
TypedTemplate {
    pattern: "{client}.put_object().bucket({bucket})...",
    params: vec!["client", "bucket", "key", "body"],
    param_types: vec![ParamType::Expr, ParamType::String, ParamType::String, ParamType::Bytes],
}
```

**Poka-Yoke Guarantees**:
1. Parameter count validated at registration
2. Placeholder names must match `params` list
3. Type annotations enable downstream validation
4. Legacy `Template` marked `#[deprecated]`

## 12. Appendix: Formal Proof of Tractability

**Theorem**: The external library mapping problem is in complexity class P.

**Proof**:

1. Let M be the set of all Python modules (finite, enumerable)
2. Let R be the set of all Rust crates (finite, enumerable)
3. A mapping f: M → R is a function from a finite domain to a finite codomain
4. Storage of f requires O(|M|) space
5. Lookup of f(m) requires O(1) time (hash table)
6. Verification of f(m) requires O(1) compilation attempts
7. Therefore, the problem is solvable in polynomial time ∎

**Corollary**: Machine learning is not required for problems in P when the domain is finite and enumerable.

---

## Approval Checklist

- [ ] Architecture review by platform team
- [ ] Security review for plugin loading
- [ ] API design review
- [ ] Documentation review
- [ ] Performance benchmarks defined
- [ ] Test coverage requirements defined

**Awaiting team review before implementation.**
