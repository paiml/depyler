//! Module mapping from Python to Rust equivalents

use crate::hir::{Import, ImportItem};
use std::collections::HashMap;

#[cfg(test)]
#[path = "module_mapper_tests.rs"]
mod tests;

/// Maps Python modules/packages to their Rust equivalents
pub struct ModuleMapper {
    /// Mapping from Python module names to Rust crate/module paths
    module_map: HashMap<String, ModuleMapping>,
}

/// DEPYLER-0493: Constructor pattern for Rust types
#[derive(Debug, Clone, PartialEq)]
pub enum ConstructorPattern {
    /// Call as ::new() - most common pattern (BufReader, NamedTempFile, etc.)
    New,
    /// Call as regular function - not a struct (e.g., tempfile::tempfile())
    Function,
    /// Custom method call (e.g., File::open(), Regex::compile())
    Method(String),
}

#[derive(Debug, Clone)]
pub struct ModuleMapping {
    /// The Rust crate or module path
    pub rust_path: String,
    /// Whether this requires an external crate dependency
    pub is_external: bool,
    /// Optional crate version requirement
    pub version: Option<String>,
    /// Item-specific mappings within the module
    pub item_map: HashMap<String, String>,
    /// DEPYLER-0493: Constructor patterns for items that are types (not functions)
    /// Maps item name to how it should be constructed
    pub constructor_patterns: HashMap<String, ConstructorPattern>,
}

impl ModuleMapper {
    /// Create a new module mapper with default Python to Rust mappings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::module_mapper::ModuleMapper;
    ///
    /// let mapper = ModuleMapper::new();
    /// assert!(mapper.get_mapping("os").is_some());
    /// assert!(mapper.get_mapping("json").is_some());
    /// ```
    pub fn new() -> Self {
        let mut module_map = HashMap::new();

        // Standard library mappings
        module_map.insert(
            "os".to_string(),
            ModuleMapping {
                rust_path: "std".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("getcwd".to_string(), "env::current_dir".to_string()),
                    ("environ".to_string(), "env::vars".to_string()),
                    ("path".to_string(), "path::Path".to_string()),
                    ("getenv".to_string(), "env::var".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "os.path".to_string(),
            ModuleMapping {
                rust_path: "std::path".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("join".to_string(), "Path::join".to_string()),
                    ("exists".to_string(), "Path::exists".to_string()),
                    ("basename".to_string(), "Path::file_name".to_string()),
                    ("dirname".to_string(), "Path::parent".to_string()),
                    // DEPYLER-0721: splitext is handled inline in expr_gen.rs
                    // Mark as Path to suppress invalid use statement
                    ("splitext".to_string(), "Path".to_string()),
                    ("split".to_string(), "Path".to_string()),
                    ("normpath".to_string(), "Path".to_string()),
                    ("isfile".to_string(), "Path::is_file".to_string()),
                    ("isdir".to_string(), "Path::is_dir".to_string()),
                    ("isabs".to_string(), "Path::is_absolute".to_string()),
                    ("abspath".to_string(), "Path::canonicalize".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "sys".to_string(),
            ModuleMapping {
                rust_path: "std".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("argv".to_string(), "env::args".to_string()),
                    ("exit".to_string(), "process::exit".to_string()),
                    ("stdin".to_string(), "io::stdin".to_string()),
                    ("stdout".to_string(), "io::stdout".to_string()),
                    ("stderr".to_string(), "io::stderr".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // DEPYLER-0493: Python io module → Rust std::io
        module_map.insert(
            "io".to_string(),
            ModuleMapping {
                rust_path: "std::io".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("BufferedReader".to_string(), "BufReader".to_string()),
                    ("BufferedWriter".to_string(), "BufWriter".to_string()),
                    ("BytesIO".to_string(), "Cursor".to_string()),
                    ("StringIO".to_string(), "Cursor".to_string()),
                ]),
                // DEPYLER-0493: Constructor patterns for IO types
                constructor_patterns: HashMap::from([
                    // BufReader and BufWriter use ::new(inner) pattern
                    ("BufReader".to_string(), ConstructorPattern::New),
                    ("BufWriter".to_string(), ConstructorPattern::New),
                    // Cursor also uses ::new()
                    ("Cursor".to_string(), ConstructorPattern::New),
                ]),
            },
        );

        module_map.insert(
            "json".to_string(),
            ModuleMapping {
                rust_path: "serde_json".to_string(),
                is_external: true,
                version: Some("1.0".to_string()),
                item_map: HashMap::from([
                    ("loads".to_string(), "from_str".to_string()),
                    ("dumps".to_string(), "to_string".to_string()),
                    ("load".to_string(), "from_reader".to_string()),
                    ("dump".to_string(), "to_writer".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // DEPYLER-EXTDEPS-001: Enhanced re → regex mapping
        module_map.insert(
            "re".to_string(),
            ModuleMapping {
                rust_path: "regex".to_string(),
                is_external: true,
                version: Some("1.10".to_string()),
                item_map: HashMap::from([
                    // Core functions
                    ("compile".to_string(), "Regex::new".to_string()),
                    ("search".to_string(), "Regex::find".to_string()),
                    ("match".to_string(), "Regex::is_match".to_string()),
                    ("findall".to_string(), "Regex::find_iter".to_string()),
                    ("finditer".to_string(), "Regex::find_iter".to_string()),
                    ("Pattern".to_string(), "Regex".to_string()),
                    // Replacement operations
                    ("sub".to_string(), "Regex::replace_all".to_string()),
                    ("subn".to_string(), "Regex::replace_all".to_string()),
                    // Split operations
                    ("split".to_string(), "Regex::split".to_string()),
                    // Flags - mapped to RegexBuilder methods or inline patterns
                    ("IGNORECASE".to_string(), "(?i)".to_string()),
                    ("I".to_string(), "(?i)".to_string()),
                    ("MULTILINE".to_string(), "(?m)".to_string()),
                    ("M".to_string(), "(?m)".to_string()),
                    ("DOTALL".to_string(), "(?s)".to_string()),
                    ("S".to_string(), "(?s)".to_string()),
                    ("VERBOSE".to_string(), "(?x)".to_string()),
                    ("X".to_string(), "(?x)".to_string()),
                ]),
                // GH-204: Add constructor patterns for regex types
                constructor_patterns: HashMap::from([(
                    "Regex".to_string(),
                    ConstructorPattern::Method("new".to_string()),
                )]),
            },
        );

        module_map.insert(
            "datetime".to_string(),
            ModuleMapping {
                rust_path: "chrono".to_string(),
                is_external: true,
                version: Some("0.4".to_string()),
                item_map: HashMap::from([
                    ("datetime".to_string(), "DateTime".to_string()),
                    ("date".to_string(), "NaiveDate".to_string()),
                    ("time".to_string(), "NaiveTime".to_string()),
                    ("timedelta".to_string(), "Duration".to_string()),
                ]),
                // GH-204: Add constructor patterns for datetime types
                constructor_patterns: HashMap::from([
                    (
                        "DateTime".to_string(),
                        ConstructorPattern::Method("now".to_string()),
                    ),
                    (
                        "NaiveDate".to_string(),
                        ConstructorPattern::Method("from_ymd_opt".to_string()),
                    ),
                    (
                        "NaiveTime".to_string(),
                        ConstructorPattern::Method("from_hms_opt".to_string()),
                    ),
                    (
                        "Duration".to_string(),
                        ConstructorPattern::Method("seconds".to_string()),
                    ),
                ]),
            },
        );

        module_map.insert(
            "typing".to_string(),
            ModuleMapping {
                rust_path: "".to_string(), // No direct mapping, handled by type system
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("List".to_string(), "Vec".to_string()),
                    ("Dict".to_string(), "HashMap".to_string()),
                    ("Set".to_string(), "HashSet".to_string()),
                    ("Tuple".to_string(), "".to_string()), // Tuples are built-in
                    ("Optional".to_string(), "Option".to_string()),
                    ("Union".to_string(), "".to_string()), // Handled specially
                    ("Any".to_string(), "".to_string()),   // No direct mapping
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "collections".to_string(),
            ModuleMapping {
                rust_path: "std::collections".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    // DEPYLER-0170: Map to HashMap type, not HashMap::new
                    // Constructor calls are handled separately in expr_gen.rs
                    ("defaultdict".to_string(), "HashMap".to_string()),
                    ("Counter".to_string(), "HashMap".to_string()),
                    ("deque".to_string(), "VecDeque".to_string()),
                    // DEPYLER-0936: Map OrderedDict to HashMap (not IndexMap which needs external crate)
                    // HashMap in Rust 1.36+ preserves insertion order, so this is semantically correct
                    ("OrderedDict".to_string(), "HashMap".to_string()),
                ]),
                // GH-204: Add constructor patterns to prevent E0423 errors
                constructor_patterns: HashMap::from([
                    ("defaultdict".to_string(), ConstructorPattern::New),
                    ("Counter".to_string(), ConstructorPattern::New),
                    ("deque".to_string(), ConstructorPattern::New),
                    ("OrderedDict".to_string(), ConstructorPattern::New),
                    ("VecDeque".to_string(), ConstructorPattern::New),
                    ("HashMap".to_string(), ConstructorPattern::New),
                    ("HashSet".to_string(), ConstructorPattern::New),
                    ("BTreeMap".to_string(), ConstructorPattern::New),
                    ("BTreeSet".to_string(), ConstructorPattern::New),
                ]),
            },
        );

        module_map.insert(
            "math".to_string(),
            ModuleMapping {
                rust_path: "std::f64".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("sqrt".to_string(), "sqrt".to_string()),
                    ("sin".to_string(), "sin".to_string()),
                    ("cos".to_string(), "cos".to_string()),
                    ("tan".to_string(), "tan".to_string()),
                    ("pi".to_string(), "consts::PI".to_string()),
                    ("e".to_string(), "consts::E".to_string()),
                    // DEPYLER-0771: isqrt is handled inline in expr_gen.rs
                    ("isqrt".to_string(), "isqrt".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // DEPYLER-EXTDEPS-001: Enhanced random → rand mapping (Phase 2)
        module_map.insert(
            "random".to_string(),
            ModuleMapping {
                rust_path: "rand".to_string(),
                is_external: true,
                version: Some("0.8".to_string()),
                item_map: HashMap::from([
                    ("random".to_string(), "random".to_string()),
                    ("randint".to_string(), "gen_range".to_string()),
                    ("choice".to_string(), "choose".to_string()),
                    ("shuffle".to_string(), "shuffle".to_string()),
                    // Phase 2 additions
                    ("uniform".to_string(), "gen_range".to_string()),
                    ("seed".to_string(), "SeedableRng::seed_from_u64".to_string()),
                    ("randrange".to_string(), "gen_range".to_string()),
                    ("sample".to_string(), "choose_multiple".to_string()),
                    ("gauss".to_string(), "Normal::sample".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "itertools".to_string(),
            ModuleMapping {
                rust_path: "itertools".to_string(),
                is_external: true,
                version: Some("0.11".to_string()),
                item_map: HashMap::from([
                    ("chain".to_string(), "chain".to_string()),
                    ("combinations".to_string(), "combinations".to_string()),
                    ("permutations".to_string(), "permutations".to_string()),
                    ("product".to_string(), "iproduct".to_string()),
                    // DEPYLER-0557: groupby uses Itertools trait method, not standalone function
                    ("groupby".to_string(), "Itertools".to_string()),
                    ("accumulate".to_string(), "scan".to_string()),
                    ("takewhile".to_string(), "take_while".to_string()),
                    ("dropwhile".to_string(), "drop_while".to_string()),
                    ("cycle".to_string(), "cycle".to_string()),
                    ("repeat".to_string(), "repeat_n".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "functools".to_string(),
            ModuleMapping {
                rust_path: "std".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("reduce".to_string(), "".to_string()), // fold is a method on Iterator, no import needed
                    ("partial".to_string(), "".to_string()), // Closures in Rust
                    ("lru_cache".to_string(), "".to_string()), // Would need external crate
                    ("wraps".to_string(), "".to_string()),   // Not applicable in Rust
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "hashlib".to_string(),
            ModuleMapping {
                rust_path: "sha2".to_string(),
                is_external: true,
                version: Some("0.10".to_string()),
                item_map: HashMap::from([
                    ("sha256".to_string(), "Sha256".to_string()),
                    ("sha512".to_string(), "Sha512".to_string()),
                    ("sha1".to_string(), "Sha1".to_string()),
                    ("md5".to_string(), "Md5".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "base64".to_string(),
            ModuleMapping {
                rust_path: "base64".to_string(),
                is_external: true,
                version: Some("0.21".to_string()),
                item_map: HashMap::from([
                    ("b64encode".to_string(), "encode".to_string()),
                    ("b64decode".to_string(), "decode".to_string()),
                    ("urlsafe_b64encode".to_string(), "encode_config".to_string()),
                    ("urlsafe_b64decode".to_string(), "decode_config".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "urllib.parse".to_string(),
            ModuleMapping {
                rust_path: "url".to_string(),
                is_external: true,
                version: Some("2.5".to_string()),
                item_map: HashMap::from([
                    ("urlparse".to_string(), "Url::parse".to_string()),
                    ("urljoin".to_string(), "Url::join".to_string()),
                    (
                        "quote".to_string(),
                        "percent_encoding::percent_encode".to_string(),
                    ),
                    (
                        "unquote".to_string(),
                        "percent_encoding::percent_decode".to_string(),
                    ),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        module_map.insert(
            "pathlib".to_string(),
            ModuleMapping {
                rust_path: "std::path".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("Path".to_string(), "PathBuf".to_string()),
                    ("PurePath".to_string(), "Path".to_string()),
                ]),
                // GH-204: Add constructor patterns for path types
                constructor_patterns: HashMap::from([
                    (
                        "PathBuf".to_string(),
                        ConstructorPattern::Method("from".to_string()),
                    ),
                    (
                        "Path".to_string(),
                        ConstructorPattern::Method("new".to_string()),
                    ),
                ]),
            },
        );

        module_map.insert(
            "tempfile".to_string(),
            ModuleMapping {
                rust_path: "tempfile".to_string(),
                is_external: true,
                version: Some("3.0".to_string()),
                item_map: HashMap::from([
                    (
                        "NamedTemporaryFile".to_string(),
                        "NamedTempFile".to_string(),
                    ),
                    ("TemporaryDirectory".to_string(), "TempDir".to_string()),
                    ("mkstemp".to_string(), "tempfile".to_string()),
                    ("mkdtemp".to_string(), "tempdir".to_string()),
                ]),
                // DEPYLER-0493: Specify constructor patterns for tempfile types
                constructor_patterns: HashMap::from([
                    // NamedTempFile is a struct → use ::new() pattern
                    ("NamedTempFile".to_string(), ConstructorPattern::New),
                    // TempDir is a struct → use ::new() pattern
                    ("TempDir".to_string(), ConstructorPattern::New),
                    // tempfile() is a function → call directly (no ::new)
                    ("tempfile".to_string(), ConstructorPattern::Function),
                    // tempdir() is a function → call directly (no ::new)
                    ("tempdir".to_string(), ConstructorPattern::Function),
                ]),
            },
        );

        module_map.insert(
            "csv".to_string(),
            ModuleMapping {
                rust_path: "csv".to_string(),
                is_external: true,
                version: Some("1.0".to_string()),
                item_map: HashMap::from([
                    ("reader".to_string(), "Reader".to_string()),
                    ("writer".to_string(), "Writer".to_string()),
                    ("DictReader".to_string(), "Reader".to_string()),
                    ("DictWriter".to_string(), "Writer".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // =================================================================
        // DEPYLER-EXTDEPS-001: Batuta Stack Mappings (Tier 0 - P0 Priority)
        // =================================================================

        // NumPy → Trueno (Spec Section 2.3)
        // trueno provides SIMD/GPU-accelerated vector and matrix operations
        module_map.insert(
            "numpy".to_string(),
            ModuleMapping {
                rust_path: "trueno".to_string(),
                is_external: true,
                version: Some("0.7".to_string()),
                item_map: HashMap::from([
                    // Array creation
                    ("array".to_string(), "Vector::from_slice".to_string()),
                    ("zeros".to_string(), "Vector::zeros".to_string()),
                    ("ones".to_string(), "Vector::ones".to_string()),
                    ("empty".to_string(), "Vector::zeros".to_string()),
                    ("arange".to_string(), "Vector::arange".to_string()),
                    ("linspace".to_string(), "Vector::linspace".to_string()),
                    // Element-wise operations
                    ("add".to_string(), "Vector::add".to_string()),
                    ("subtract".to_string(), "Vector::sub".to_string()),
                    ("multiply".to_string(), "Vector::mul".to_string()),
                    ("divide".to_string(), "Vector::div".to_string()),
                    ("sqrt".to_string(), "Vector::sqrt".to_string()),
                    ("exp".to_string(), "Vector::exp".to_string()),
                    ("log".to_string(), "Vector::ln".to_string()),
                    ("sin".to_string(), "Vector::sin".to_string()),
                    ("cos".to_string(), "Vector::cos".to_string()),
                    ("abs".to_string(), "Vector::abs".to_string()),
                    // Dot product and matrix operations
                    ("dot".to_string(), "Vector::dot".to_string()),
                    ("matmul".to_string(), "Matrix::matmul".to_string()),
                    // Reductions
                    ("sum".to_string(), "Vector::sum".to_string()),
                    ("mean".to_string(), "Vector::mean".to_string()),
                    ("max".to_string(), "Vector::max".to_string()),
                    ("min".to_string(), "Vector::min".to_string()),
                    ("std".to_string(), "Vector::std".to_string()),
                    ("var".to_string(), "Vector::var".to_string()),
                    ("argmax".to_string(), "Vector::argmax".to_string()),
                    ("argmin".to_string(), "Vector::argmin".to_string()),
                    // Shape operations
                    ("reshape".to_string(), "Matrix::reshape".to_string()),
                    ("transpose".to_string(), "Matrix::transpose".to_string()),
                    ("flatten".to_string(), "Vector::flatten".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // NumPy linalg submodule → Trueno linalg
        module_map.insert(
            "numpy.linalg".to_string(),
            ModuleMapping {
                rust_path: "trueno::linalg".to_string(),
                is_external: true,
                version: Some("0.7".to_string()),
                item_map: HashMap::from([
                    ("norm".to_string(), "norm".to_string()),
                    ("inv".to_string(), "inv".to_string()),
                    ("det".to_string(), "det".to_string()),
                    ("eig".to_string(), "eig".to_string()),
                    ("svd".to_string(), "svd".to_string()),
                    ("solve".to_string(), "solve".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // Sklearn → Aprender (Spec Section 2.4)
        // aprender provides ML algorithms compatible with sklearn API

        // sklearn.linear_model → aprender::linear
        module_map.insert(
            "sklearn.linear_model".to_string(),
            ModuleMapping {
                rust_path: "aprender::linear".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    (
                        "LinearRegression".to_string(),
                        "LinearRegression".to_string(),
                    ),
                    (
                        "LogisticRegression".to_string(),
                        "LogisticRegression".to_string(),
                    ),
                    ("Ridge".to_string(), "Ridge".to_string()),
                    ("Lasso".to_string(), "Lasso".to_string()),
                    ("ElasticNet".to_string(), "ElasticNet".to_string()),
                ]),
                constructor_patterns: HashMap::from([
                    ("LinearRegression".to_string(), ConstructorPattern::New),
                    ("LogisticRegression".to_string(), ConstructorPattern::New),
                ]),
            },
        );

        // sklearn.cluster → aprender::cluster
        module_map.insert(
            "sklearn.cluster".to_string(),
            ModuleMapping {
                rust_path: "aprender::cluster".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    ("KMeans".to_string(), "KMeans".to_string()),
                    ("DBSCAN".to_string(), "DBSCAN".to_string()),
                    (
                        "AgglomerativeClustering".to_string(),
                        "Agglomerative".to_string(),
                    ),
                ]),
                constructor_patterns: HashMap::from([(
                    "KMeans".to_string(),
                    ConstructorPattern::New,
                )]),
            },
        );

        // sklearn.tree → aprender::tree
        module_map.insert(
            "sklearn.tree".to_string(),
            ModuleMapping {
                rust_path: "aprender::tree".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    (
                        "DecisionTreeClassifier".to_string(),
                        "DecisionTree".to_string(),
                    ),
                    (
                        "DecisionTreeRegressor".to_string(),
                        "DecisionTreeRegressor".to_string(),
                    ),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // sklearn.ensemble → aprender::ensemble
        module_map.insert(
            "sklearn.ensemble".to_string(),
            ModuleMapping {
                rust_path: "aprender::ensemble".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    (
                        "RandomForestClassifier".to_string(),
                        "RandomForest".to_string(),
                    ),
                    (
                        "RandomForestRegressor".to_string(),
                        "RandomForestRegressor".to_string(),
                    ),
                    (
                        "GradientBoostingClassifier".to_string(),
                        "GradientBoosting".to_string(),
                    ),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // sklearn.preprocessing → aprender::preprocessing
        module_map.insert(
            "sklearn.preprocessing".to_string(),
            ModuleMapping {
                rust_path: "aprender::preprocessing".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    ("StandardScaler".to_string(), "StandardScaler".to_string()),
                    ("MinMaxScaler".to_string(), "MinMaxScaler".to_string()),
                    ("LabelEncoder".to_string(), "LabelEncoder".to_string()),
                    ("OneHotEncoder".to_string(), "OneHotEncoder".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // sklearn.decomposition → aprender::decomposition
        module_map.insert(
            "sklearn.decomposition".to_string(),
            ModuleMapping {
                rust_path: "aprender::decomposition".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    ("PCA".to_string(), "PCA".to_string()),
                    ("TruncatedSVD".to_string(), "TruncatedSVD".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // sklearn.model_selection → aprender::model_selection
        module_map.insert(
            "sklearn.model_selection".to_string(),
            ModuleMapping {
                rust_path: "aprender::model_selection".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    (
                        "train_test_split".to_string(),
                        "train_test_split".to_string(),
                    ),
                    ("KFold".to_string(), "KFold".to_string()),
                    ("cross_val_score".to_string(), "cross_val_score".to_string()),
                    ("GridSearchCV".to_string(), "GridSearchCV".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // sklearn.metrics → aprender::metrics
        module_map.insert(
            "sklearn.metrics".to_string(),
            ModuleMapping {
                rust_path: "aprender::metrics".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    ("accuracy_score".to_string(), "accuracy".to_string()),
                    ("precision_score".to_string(), "precision".to_string()),
                    ("recall_score".to_string(), "recall".to_string()),
                    ("f1_score".to_string(), "f1".to_string()),
                    (
                        "confusion_matrix".to_string(),
                        "confusion_matrix".to_string(),
                    ),
                    ("mean_squared_error".to_string(), "mse".to_string()),
                    ("r2_score".to_string(), "r2".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // =================================================================
        // DEPYLER-EXTDEPS-001: High-Impact Standard Library Mappings (P0)
        // =================================================================

        // subprocess → std::process (146 occurrences in corpus)
        module_map.insert(
            "subprocess".to_string(),
            ModuleMapping {
                rust_path: "std::process".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    // Process execution
                    ("run".to_string(), "Command".to_string()),
                    ("Popen".to_string(), "Command".to_string()),
                    ("call".to_string(), "Command".to_string()),
                    ("check_call".to_string(), "Command".to_string()),
                    ("check_output".to_string(), "Command".to_string()),
                    // I/O constants
                    ("PIPE".to_string(), "Stdio::piped".to_string()),
                    ("STDOUT".to_string(), "Stdio::inherit".to_string()),
                    ("DEVNULL".to_string(), "Stdio::null".to_string()),
                    // CompletedProcess fields map to Output
                    ("CompletedProcess".to_string(), "Output".to_string()),
                ]),
                constructor_patterns: HashMap::from([(
                    "Command".to_string(),
                    ConstructorPattern::Method("new".to_string()),
                )]),
            },
        );

        // DEPYLER-0363: Map argparse to clap
        // Note: This requires special handling in codegen for structural transformation
        module_map.insert(
            "argparse".to_string(),
            ModuleMapping {
                rust_path: "clap".to_string(),
                is_external: true,
                version: Some("4.5".to_string()),
                item_map: HashMap::from([
                    ("ArgumentParser".to_string(), "Parser".to_string()),
                    // These require special codegen handling:
                    // - ArgumentParser() → #[derive(Parser)] struct
                    // - add_argument() → struct fields with #[arg] attributes
                    // - parse_args() → Args::parse()
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // =================================================================
        // DEPYLER-EXTDEPS-001: Phase 2 Mappings (P1 - Medium Impact)
        // =================================================================

        // threading → std::thread (stdlib mapping)
        // Maps Python threading primitives to Rust std library equivalents
        module_map.insert(
            "threading".to_string(),
            ModuleMapping {
                rust_path: "std::thread".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    // Thread creation and management
                    ("Thread".to_string(), "spawn".to_string()),
                    ("current_thread".to_string(), "current".to_string()),
                    // Synchronization primitives (from std::sync)
                    ("Lock".to_string(), "Mutex".to_string()),
                    ("RLock".to_string(), "Mutex".to_string()),
                    ("Event".to_string(), "Condvar".to_string()),
                    ("Condition".to_string(), "Condvar".to_string()),
                    ("Semaphore".to_string(), "Semaphore".to_string()),
                    ("BoundedSemaphore".to_string(), "Semaphore".to_string()),
                    ("Barrier".to_string(), "Barrier".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // asyncio → tokio (external async runtime)
        // Maps Python asyncio primitives to Tokio equivalents
        module_map.insert(
            "asyncio".to_string(),
            ModuleMapping {
                rust_path: "tokio".to_string(),
                is_external: true,
                version: Some("1.35".to_string()),
                item_map: HashMap::from([
                    // Runtime and execution
                    ("run".to_string(), "runtime::Runtime::block_on".to_string()),
                    ("create_task".to_string(), "spawn".to_string()),
                    // Time operations
                    ("sleep".to_string(), "time::sleep".to_string()),
                    ("wait_for".to_string(), "time::timeout".to_string()),
                    // Concurrency primitives
                    ("gather".to_string(), "join!".to_string()),
                    ("wait".to_string(), "select!".to_string()),
                    // Channel/Queue
                    ("Queue".to_string(), "sync::mpsc::channel".to_string()),
                    // Event loop (conceptually maps to runtime)
                    (
                        "get_event_loop".to_string(),
                        "runtime::Handle::current".to_string(),
                    ),
                    (
                        "new_event_loop".to_string(),
                        "runtime::Runtime::new".to_string(),
                    ),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // struct → byteorder (binary data packing)
        // Maps Python struct module to byteorder crate
        module_map.insert(
            "struct".to_string(),
            ModuleMapping {
                rust_path: "byteorder".to_string(),
                is_external: true,
                version: Some("1.5".to_string()),
                item_map: HashMap::from([
                    // Core pack/unpack operations
                    ("pack".to_string(), "WriteBytesExt".to_string()),
                    ("unpack".to_string(), "ReadBytesExt".to_string()),
                    ("pack_into".to_string(), "WriteBytesExt".to_string()),
                    ("unpack_from".to_string(), "ReadBytesExt".to_string()),
                    // Size calculations
                    ("calcsize".to_string(), "std::mem::size_of".to_string()),
                    // Struct object for repeated use
                    ("Struct".to_string(), "ByteOrder".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // statistics → statrs (statistical functions)
        // Maps Python statistics module to statrs crate
        module_map.insert(
            "statistics".to_string(),
            ModuleMapping {
                rust_path: "statrs".to_string(),
                is_external: true,
                version: Some("0.16".to_string()),
                item_map: HashMap::from([
                    // Central tendency
                    (
                        "mean".to_string(),
                        "statistics::Statistics::mean".to_string(),
                    ),
                    (
                        "median".to_string(),
                        "statistics::Statistics::median".to_string(),
                    ),
                    (
                        "mode".to_string(),
                        "statistics::Statistics::mode".to_string(),
                    ),
                    // Spread measures
                    (
                        "stdev".to_string(),
                        "statistics::Statistics::std_dev".to_string(),
                    ),
                    (
                        "variance".to_string(),
                        "statistics::Statistics::variance".to_string(),
                    ),
                    (
                        "pstdev".to_string(),
                        "statistics::Statistics::population_std_dev".to_string(),
                    ),
                    (
                        "pvariance".to_string(),
                        "statistics::Statistics::population_variance".to_string(),
                    ),
                    // Quantiles
                    (
                        "quantiles".to_string(),
                        "statistics::Statistics::percentile".to_string(),
                    ),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // =================================================================
        // GH-204: E0425/E0433 Resolution - Additional stdlib mappings
        // =================================================================

        // logging → log (standard Rust logging facade)
        // Maps Python logging module to log crate
        module_map.insert(
            "logging".to_string(),
            ModuleMapping {
                rust_path: "log".to_string(),
                is_external: true,
                version: Some("0.4".to_string()),
                item_map: HashMap::from([
                    // Log level functions
                    ("debug".to_string(), "debug!".to_string()),
                    ("info".to_string(), "info!".to_string()),
                    ("warning".to_string(), "warn!".to_string()),
                    ("warn".to_string(), "warn!".to_string()),
                    ("error".to_string(), "error!".to_string()),
                    ("critical".to_string(), "error!".to_string()),
                    // Configuration (often no-ops in Rust - use env_logger)
                    ("basicConfig".to_string(), "env_logger::init".to_string()),
                    ("getLogger".to_string(), "log::logger".to_string()),
                    // Log levels as constants
                    ("DEBUG".to_string(), "log::Level::Debug".to_string()),
                    ("INFO".to_string(), "log::Level::Info".to_string()),
                    ("WARNING".to_string(), "log::Level::Warn".to_string()),
                    ("ERROR".to_string(), "log::Level::Error".to_string()),
                    ("CRITICAL".to_string(), "log::Level::Error".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // configparser → config crate (INI file parsing)
        // Maps Python configparser module to config crate
        module_map.insert(
            "configparser".to_string(),
            ModuleMapping {
                rust_path: "config".to_string(),
                is_external: true,
                version: Some("0.14".to_string()),
                item_map: HashMap::from([
                    ("ConfigParser".to_string(), "Config".to_string()),
                    ("RawConfigParser".to_string(), "Config".to_string()),
                    ("SafeConfigParser".to_string(), "Config".to_string()),
                ]),
                constructor_patterns: HashMap::from([(
                    "Config".to_string(),
                    ConstructorPattern::Method("builder".to_string()),
                )]),
            },
        );

        // unittest → Rust test module (no external crate needed)
        // Maps Python unittest to Rust's built-in test system
        module_map.insert(
            "unittest".to_string(),
            ModuleMapping {
                rust_path: "".to_string(), // Uses #[test] attribute
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    // TestCase is handled via #[test] attribute
                    ("TestCase".to_string(), "".to_string()),
                    // Assertions map to assert! macros
                    ("assertEqual".to_string(), "assert_eq!".to_string()),
                    ("assertNotEqual".to_string(), "assert_ne!".to_string()),
                    ("assertTrue".to_string(), "assert!".to_string()),
                    ("assertFalse".to_string(), "assert!".to_string()),
                    ("assertIsNone".to_string(), "assert!".to_string()),
                    ("assertIsNotNone".to_string(), "assert!".to_string()),
                    ("assertIn".to_string(), "assert!".to_string()),
                    ("assertNotIn".to_string(), "assert!".to_string()),
                    ("assertRaises".to_string(), "assert!".to_string()),
                    ("main".to_string(), "".to_string()), // No-op, tests run via cargo test
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // traceback → backtrace crate (stack trace handling)
        // Maps Python traceback module to backtrace crate
        module_map.insert(
            "traceback".to_string(),
            ModuleMapping {
                rust_path: "backtrace".to_string(),
                is_external: true,
                version: Some("0.3".to_string()),
                item_map: HashMap::from([
                    ("print_exc".to_string(), "Backtrace::capture".to_string()),
                    ("format_exc".to_string(), "Backtrace::capture".to_string()),
                    ("print_tb".to_string(), "Backtrace::capture".to_string()),
                    ("format_tb".to_string(), "Backtrace::capture".to_string()),
                    ("extract_tb".to_string(), "Backtrace::capture".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        // contextlib → No direct equivalent (handled inline)
        // Maps Python contextlib module - mostly handled via special codegen
        module_map.insert(
            "contextlib".to_string(),
            ModuleMapping {
                rust_path: "".to_string(), // Handled via Drop trait
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    // These are largely handled by Rust's RAII pattern
                    ("contextmanager".to_string(), "".to_string()),
                    ("closing".to_string(), "".to_string()),
                    ("suppress".to_string(), "".to_string()),
                    ("nullcontext".to_string(), "".to_string()),
                ]),
                constructor_patterns: HashMap::new(),
            },
        );

        Self { module_map }
    }

    /// Map a Python import to Rust use statements
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::module_mapper::{ModuleMapper, RustImport};
    /// use depyler_core::hir::{Import, ImportItem};
    ///
    /// let mapper = ModuleMapper::new();
    /// let import = Import {
    ///     module: "json".to_string(),
    ///     items: vec![ImportItem::Named("loads".to_string())],
    /// };
    ///
    /// let rust_imports = mapper.map_import(&import);
    /// assert_eq!(rust_imports[0].path, "serde_json::from_str");
    /// assert!(rust_imports[0].is_external);
    /// ```
    pub fn map_import(&self, import: &Import) -> Vec<RustImport> {
        let mut rust_imports = Vec::new();

        if let Some(mapping) = self.module_map.get(&import.module) {
            // If no specific items, it's a whole module import
            if import.items.is_empty() {
                // DEPYLER-0363: For mapped modules, emit the Rust equivalent
                // For argparse, this means `use clap::Parser;`
                if !mapping.rust_path.is_empty() {
                    // For external crates like argparse->clap, import the main trait/type
                    if import.module == "argparse" {
                        // ArgumentParser needs the Parser derive trait
                        rust_imports.push(RustImport {
                            path: format!("{}::Parser", mapping.rust_path),
                            alias: None,
                            is_external: mapping.is_external,
                        });
                    } else {
                        // For other modules, just import the module path
                        rust_imports.push(RustImport {
                            path: mapping.rust_path.clone(),
                            alias: Some(import.module.clone()),
                            is_external: mapping.is_external,
                        });
                    }
                } else {
                    // Empty rust_path means no direct mapping (like typing module)
                    rust_imports.push(RustImport {
                        path: format!("// Python import: {} (no Rust equivalent)", import.module),
                        alias: None,
                        is_external: false,
                    });
                }
            } else {
                // Handle each imported item
                for item in &import.items {
                    match item {
                        ImportItem::Named(name) => {
                            if let Some(rust_name) = mapping.item_map.get(name) {
                                rust_imports.push(RustImport {
                                    path: format!("{}::{}", mapping.rust_path, rust_name),
                                    alias: None,
                                    is_external: mapping.is_external,
                                });
                            } else {
                                // Direct mapping
                                rust_imports.push(RustImport {
                                    path: format!("{}::{}", mapping.rust_path, name),
                                    alias: None,
                                    is_external: mapping.is_external,
                                });
                            }
                        }
                        ImportItem::Aliased { name, alias } => {
                            if let Some(rust_name) = mapping.item_map.get(name) {
                                rust_imports.push(RustImport {
                                    path: format!("{}::{}", mapping.rust_path, rust_name),
                                    alias: Some(alias.clone()),
                                    is_external: mapping.is_external,
                                });
                            } else {
                                rust_imports.push(RustImport {
                                    path: format!("{}::{}", mapping.rust_path, name),
                                    alias: Some(alias.clone()),
                                    is_external: mapping.is_external,
                                });
                            }
                        }
                    }
                }
            }
        } else {
            // Unknown module - create a placeholder or warning
            rust_imports.push(RustImport {
                path: format!(
                    "// NOTE: Map Python module '{}' (tracked in DEPYLER-0424)",
                    import.module
                ),
                alias: None,
                is_external: false,
            });
        }

        rust_imports
    }

    /// Get all external dependencies needed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::module_mapper::ModuleMapper;
    /// use depyler_core::hir::{Import, ImportItem};
    ///
    /// let mapper = ModuleMapper::new();
    /// let imports = vec![
    ///     Import {
    ///         module: "json".to_string(),
    ///         items: vec![ImportItem::Named("loads".to_string())],
    ///     },
    ///     Import {
    ///         module: "os".to_string(),
    ///         items: vec![ImportItem::Named("getcwd".to_string())],
    ///     },
    /// ];
    ///
    /// let deps = mapper.get_dependencies(&imports);
    /// assert_eq!(deps.len(), 1); // Only json is external
    /// assert_eq!(deps[0], ("serde_json".to_string(), "1.0".to_string()));
    /// ```
    pub fn get_dependencies(&self, imports: &[Import]) -> Vec<(String, String)> {
        let mut deps = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for import in imports {
            if let Some(mapping) = self.module_map.get(&import.module) {
                if mapping.is_external && !seen.contains(&mapping.rust_path) {
                    seen.insert(&mapping.rust_path);
                    if let Some(version) = &mapping.version {
                        deps.push((mapping.rust_path.clone(), version.clone()));
                    }
                }
            }
        }

        deps
    }

    /// Get module mapping for a given module name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::module_mapper::ModuleMapper;
    ///
    /// let mapper = ModuleMapper::new();
    ///
    /// if let Some(mapping) = mapper.get_mapping("json") {
    ///     assert_eq!(mapping.rust_path, "serde_json");
    ///     assert!(mapping.is_external);
    ///     assert_eq!(mapping.version.as_ref().unwrap(), "1.0");
    /// }
    /// ```
    pub fn get_mapping(&self, module_name: &str) -> Option<&ModuleMapping> {
        self.module_map.get(module_name)
    }
}

#[derive(Debug, Clone)]
pub struct RustImport {
    pub path: String,
    pub alias: Option<String>,
    pub is_external: bool,
}

impl Default for ModuleMapper {
    fn default() -> Self {
        Self::new()
    }
}
