//! Module mapping from Python to Rust equivalents

use crate::hir::{Import, ImportItem};
use std::collections::HashMap;

/// Maps Python modules/packages to their Rust equivalents
pub struct ModuleMapper {
    /// Mapping from Python module names to Rust crate/module paths
    module_map: HashMap<String, ModuleMapping>,
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
}

impl ModuleMapper {
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
                ]),
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
            },
        );

        module_map.insert(
            "re".to_string(),
            ModuleMapping {
                rust_path: "regex".to_string(),
                is_external: true,
                version: Some("1.0".to_string()),
                item_map: HashMap::from([
                    ("compile".to_string(), "Regex::new".to_string()),
                    ("search".to_string(), "Regex::find".to_string()),
                    ("match".to_string(), "Regex::is_match".to_string()),
                    ("findall".to_string(), "Regex::find_iter".to_string()),
                    ("Pattern".to_string(), "Regex".to_string()),
                ]),
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
            },
        );

        module_map.insert(
            "collections".to_string(),
            ModuleMapping {
                rust_path: "std::collections".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("defaultdict".to_string(), "HashMap::new".to_string()),
                    ("Counter".to_string(), "HashMap::new".to_string()),
                    ("deque".to_string(), "VecDeque".to_string()),
                    ("OrderedDict".to_string(), "IndexMap".to_string()), // requires indexmap crate
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
                ]),
            },
        );

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
                ]),
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
                    ("groupby".to_string(), "group_by".to_string()),
                    ("accumulate".to_string(), "scan".to_string()),
                    ("takewhile".to_string(), "take_while".to_string()),
                    ("dropwhile".to_string(), "drop_while".to_string()),
                    ("cycle".to_string(), "cycle".to_string()),
                    ("repeat".to_string(), "repeat_n".to_string()),
                ]),
            },
        );

        module_map.insert(
            "functools".to_string(),
            ModuleMapping {
                rust_path: "std".to_string(),
                is_external: false,
                version: None,
                item_map: HashMap::from([
                    ("reduce".to_string(), "iter::Iterator::fold".to_string()),
                    ("partial".to_string(), "".to_string()), // Closures in Rust
                    ("lru_cache".to_string(), "".to_string()), // Would need external crate
                    ("wraps".to_string(), "".to_string()),   // Not applicable in Rust
                ]),
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
            },
        );

        Self { module_map }
    }

    /// Map a Python import to Rust use statements
    pub fn map_import(&self, import: &Import) -> Vec<RustImport> {
        let mut rust_imports = Vec::new();

        if let Some(mapping) = self.module_map.get(&import.module) {
            // If no specific items, it's a whole module import
            if import.items.is_empty() {
                // For whole module imports, we need to map to appropriate Rust module
                // For now, add a comment indicating module-level import
                rust_imports.push(RustImport {
                    path: format!("// Python import: {}", import.module),
                    alias: None,
                    is_external: false,
                });
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
                path: format!("// TODO: Map Python module '{}'", import.module),
                alias: None,
                is_external: false,
            });
        }

        rust_imports
    }

    /// Get all external dependencies needed
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
