//! Typeshed Stub Ingestion for Auto-Generating Module Mappings
//!
//! DEPYLER-O1MAP-001 Section 5: Automation Strategy
//!
//! This module parses Python `.pyi` stub files from typeshed and automatically
//! generates `ModuleMapping` structs for the module mapper.
//!
//! ## Design Principles (Toyota Way)
//!
//! - **Genchi Genbutsu**: Parse actual typeshed stubs, not assumptions
//! - **Jidoka**: Validate mappings against known Rust equivalents
//! - **Kaizen**: Incrementally expand coverage from json → stdlib → ecosystem

use crate::module_mapper::ModuleMapping;
use std::collections::HashMap;

/// Result of parsing a single function signature from a .pyi stub
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFunction {
    /// Python function name
    pub name: String,
    /// Parameter types (name -> type string)
    pub params: Vec<(String, String)>,
    /// Return type string
    pub return_type: String,
}

/// Configuration for mapping Python types to Rust types
#[derive(Debug, Clone)]
pub struct TypeMappingConfig {
    /// Python type -> Rust type
    pub type_map: HashMap<String, String>,
}

impl Default for TypeMappingConfig {
    fn default() -> Self {
        let mut type_map = HashMap::new();
        // Primitive types
        type_map.insert("str".to_string(), "String".to_string());
        type_map.insert("int".to_string(), "i64".to_string());
        type_map.insert("float".to_string(), "f64".to_string());
        type_map.insert("bool".to_string(), "bool".to_string());
        type_map.insert("None".to_string(), "()".to_string());
        type_map.insert("bytes".to_string(), "Vec<u8>".to_string());
        // Generic types
        type_map.insert("Any".to_string(), "serde_json::Value".to_string());
        type_map.insert("object".to_string(), "serde_json::Value".to_string());
        // Container types (simplified - full generic handling is more complex)
        type_map.insert("list".to_string(), "Vec".to_string());
        type_map.insert("dict".to_string(), "HashMap".to_string());
        type_map.insert("set".to_string(), "HashSet".to_string());
        type_map.insert("tuple".to_string(), "tuple".to_string());
        type_map.insert("List".to_string(), "Vec".to_string());
        type_map.insert("Dict".to_string(), "HashMap".to_string());
        type_map.insert("Set".to_string(), "HashSet".to_string());
        type_map.insert("Tuple".to_string(), "tuple".to_string());
        type_map.insert("Optional".to_string(), "Option".to_string());

        Self { type_map }
    }
}

/// Known Rust crate mappings for Python modules
#[derive(Debug, Clone)]
pub struct CrateMappingConfig {
    /// Python module -> (Rust crate path, is_external, version)
    pub crate_map: HashMap<String, (String, bool, Option<String>)>,
}

impl Default for CrateMappingConfig {
    fn default() -> Self {
        let mut crate_map = HashMap::new();
        // Standard library -> Rust stdlib or well-known crates
        crate_map.insert("json".to_string(), ("serde_json".to_string(), true, Some("1.0".to_string())));
        crate_map.insert("os".to_string(), ("std".to_string(), false, None));
        crate_map.insert("sys".to_string(), ("std".to_string(), false, None));
        crate_map.insert("math".to_string(), ("std::f64".to_string(), false, None));
        crate_map.insert("re".to_string(), ("regex".to_string(), true, Some("1.10".to_string())));
        crate_map.insert("random".to_string(), ("rand".to_string(), true, Some("0.8".to_string())));
        crate_map.insert("datetime".to_string(), ("chrono".to_string(), true, Some("0.4".to_string())));
        crate_map.insert("collections".to_string(), ("std::collections".to_string(), false, None));
        crate_map.insert("itertools".to_string(), ("itertools".to_string(), true, Some("0.12".to_string())));
        crate_map.insert("hashlib".to_string(), ("sha2".to_string(), true, Some("0.10".to_string())));
        crate_map.insert("base64".to_string(), ("base64".to_string(), true, Some("0.21".to_string())));
        crate_map.insert("csv".to_string(), ("csv".to_string(), true, Some("1.3".to_string())));
        crate_map.insert("pathlib".to_string(), ("std::path".to_string(), false, None));
        crate_map.insert("tempfile".to_string(), ("tempfile".to_string(), true, Some("3.0".to_string())));

        Self { crate_map }
    }
}

/// Known function-to-Rust mappings for specific modules
/// This is the "semantic bridge" that maps Python function names to Rust equivalents
#[derive(Debug, Clone)]
pub struct FunctionMappingConfig {
    /// (module, python_func) -> rust_func
    pub func_map: HashMap<(String, String), String>,
}

impl Default for FunctionMappingConfig {
    fn default() -> Self {
        let mut func_map = HashMap::new();

        // json module
        func_map.insert(("json".to_string(), "loads".to_string()), "from_str".to_string());
        func_map.insert(("json".to_string(), "dumps".to_string()), "to_string".to_string());
        func_map.insert(("json".to_string(), "load".to_string()), "from_reader".to_string());
        func_map.insert(("json".to_string(), "dump".to_string()), "to_writer".to_string());

        // os module
        func_map.insert(("os".to_string(), "getcwd".to_string()), "env::current_dir".to_string());
        func_map.insert(("os".to_string(), "getenv".to_string()), "env::var".to_string());
        func_map.insert(("os".to_string(), "listdir".to_string()), "fs::read_dir".to_string());

        // math module
        func_map.insert(("math".to_string(), "sqrt".to_string()), "sqrt".to_string());
        func_map.insert(("math".to_string(), "sin".to_string()), "sin".to_string());
        func_map.insert(("math".to_string(), "cos".to_string()), "cos".to_string());
        func_map.insert(("math".to_string(), "floor".to_string()), "floor".to_string());
        func_map.insert(("math".to_string(), "ceil".to_string()), "ceil".to_string());
        func_map.insert(("math".to_string(), "abs".to_string()), "abs".to_string());
        func_map.insert(("math".to_string(), "pow".to_string()), "powf".to_string());

        // re module
        func_map.insert(("re".to_string(), "compile".to_string()), "Regex::new".to_string());
        func_map.insert(("re".to_string(), "match".to_string()), "Regex::is_match".to_string());
        func_map.insert(("re".to_string(), "search".to_string()), "Regex::find".to_string());
        func_map.insert(("re".to_string(), "findall".to_string()), "Regex::find_iter".to_string());
        func_map.insert(("re".to_string(), "sub".to_string()), "Regex::replace_all".to_string());

        Self { func_map }
    }
}

/// Parse a .pyi stub file content and extract function signatures
///
/// # Arguments
/// * `content` - The content of the .pyi file
/// * `module_name` - The Python module name (e.g., "json", "os")
///
/// # Returns
/// A `ModuleMapping` ready for use in the module mapper
pub fn parse_pyi(content: &str, module_name: &str) -> ModuleMapping {
    parse_pyi_with_config(
        content,
        module_name,
        &TypeMappingConfig::default(),
        &CrateMappingConfig::default(),
        &FunctionMappingConfig::default(),
    )
}

/// Parse a .pyi stub with custom configuration
pub fn parse_pyi_with_config(
    content: &str,
    module_name: &str,
    _type_config: &TypeMappingConfig,
    crate_config: &CrateMappingConfig,
    func_config: &FunctionMappingConfig,
) -> ModuleMapping {
    let functions = extract_function_signatures(content);

    // Get crate mapping for this module
    let (rust_path, is_external, version) = crate_config
        .crate_map
        .get(module_name)
        .cloned()
        .unwrap_or_else(|| (module_name.to_string(), true, None));

    // Build item map from parsed functions + known mappings
    let mut item_map = HashMap::new();

    for func in &functions {
        // Check if we have a known mapping for this function
        if let Some(rust_func) = func_config.func_map.get(&(module_name.to_string(), func.name.clone())) {
            item_map.insert(func.name.clone(), rust_func.clone());
        } else {
            // Default: use same name (snake_case preserved)
            item_map.insert(func.name.clone(), func.name.clone());
        }
    }

    ModuleMapping {
        rust_path,
        is_external,
        version,
        item_map,
        constructor_patterns: HashMap::new(),
    }
}

/// Extract function signatures from .pyi content
///
/// Parses lines like:
/// - `def loads(s: str) -> Any: ...`
/// - `def dumps(obj: Any, indent: int = None) -> str: ...`
/// - Multiline function definitions (joined before parsing)
fn extract_function_signatures(content: &str) -> Vec<ParsedFunction> {
    let mut functions = Vec::new();

    // First, join multiline function definitions into single lines
    let normalized = normalize_multiline_functions(content);

    // Simple regex-like parsing for function definitions
    // Pattern: def <name>(<params>) -> <return_type>: ...
    for line in normalized.lines() {
        let line = line.trim();

        // Skip non-function lines
        if !line.starts_with("def ") {
            continue;
        }

        // Skip private/dunder methods for now
        if line.starts_with("def _") && !line.starts_with("def __init__") {
            continue;
        }

        if let Some(func) = parse_function_line(line) {
            functions.push(func);
        }
    }

    functions
}

/// Normalize multiline function definitions into single lines
fn normalize_multiline_functions(content: &str) -> String {
    let mut result = String::new();
    let mut current_def = String::new();
    let mut in_def = false;
    let mut paren_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and non-function content when not in a def
        if trimmed.is_empty() && !in_def {
            continue;
        }

        if trimmed.starts_with("def ") {
            // If we were in a def, flush it first (shouldn't happen normally)
            if in_def && !current_def.is_empty() {
                result.push_str(&current_def);
                result.push('\n');
            }
            // Start of a new function definition
            in_def = true;
            current_def = trimmed.to_string();
            paren_depth = count_parens(trimmed);
        } else if in_def {
            // Continuation of function definition
            current_def.push(' ');
            current_def.push_str(trimmed);
            paren_depth += count_parens(trimmed);
        }

        // Check if function definition is complete (parens balanced and has closing pattern)
        if in_def && paren_depth == 0 && (current_def.ends_with(": ...") || current_def.ends_with("):")) {
            result.push_str(&current_def);
            result.push('\n');
            current_def.clear();
            in_def = false;
        }
    }

    // Flush any remaining definition
    if !current_def.is_empty() {
        result.push_str(&current_def);
        result.push('\n');
    }

    result
}

/// Count net paren depth change in a line
fn count_parens(s: &str) -> i32 {
    let mut depth = 0;
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// Parse a single function definition line
fn parse_function_line(line: &str) -> Option<ParsedFunction> {
    // Remove "def " prefix
    let line = line.strip_prefix("def ")?;

    // Find function name (up to open paren)
    let paren_idx = line.find('(')?;
    let name = line[..paren_idx].trim().to_string();

    // Find the closing paren and return type
    let close_paren_idx = line.rfind(')')?;
    let params_str = &line[paren_idx + 1..close_paren_idx];

    // Parse return type (after "->")
    let return_type = if let Some(arrow_idx) = line.find("->") {
        let ret_part = &line[arrow_idx + 2..];
        // Remove trailing ": ..." if present
        let ret_type = ret_part.trim().trim_end_matches(": ...");
        ret_type.trim().to_string()
    } else {
        "None".to_string()
    };

    // Parse parameters
    let params = parse_params(params_str);

    Some(ParsedFunction {
        name,
        params,
        return_type,
    })
}

/// Parse parameter list from function signature
fn parse_params(params_str: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();

    if params_str.trim().is_empty() {
        return params;
    }

    // Simple splitting (doesn't handle nested generics perfectly)
    // For production, use a proper parser
    let mut depth = 0;
    let mut current = String::new();
    let mut parts = Vec::new();

    for ch in params_str.chars() {
        match ch {
            '[' | '(' => {
                depth += 1;
                current.push(ch);
            }
            ']' | ')' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }

    // Parse each parameter
    for part in parts {
        // Skip *args, **kwargs, self
        if part.starts_with('*') || part == "self" {
            continue;
        }

        // Handle: name: type or name: type = default
        if let Some(colon_idx) = part.find(':') {
            let param_name = part[..colon_idx].trim().to_string();
            let type_part = &part[colon_idx + 1..];

            // Remove default value if present
            let param_type = if let Some(eq_idx) = type_part.find('=') {
                type_part[..eq_idx].trim().to_string()
            } else {
                type_part.trim().to_string()
            };

            params.push((param_name, param_type));
        } else {
            // Untyped parameter
            params.push((part.clone(), "Any".to_string()));
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TDD RED: Test ingestion of json.pyi stub
    /// This is the primary acceptance test for the typeshed ingestion system
    #[test]
    fn test_ingest_json_stub() {
        // Mock json.pyi content (simplified from actual typeshed)
        let json_pyi = r#"
from typing import Any, IO, Optional

def loads(
    s: str,
    *,
    cls: Optional[type] = None,
    object_hook: Optional[Any] = None,
    parse_float: Optional[Any] = None,
    parse_int: Optional[Any] = None,
    parse_constant: Optional[Any] = None,
    object_pairs_hook: Optional[Any] = None,
) -> Any: ...

def dumps(
    obj: Any,
    *,
    skipkeys: bool = False,
    ensure_ascii: bool = True,
    check_circular: bool = True,
    allow_nan: bool = True,
    cls: Optional[type] = None,
    indent: Optional[int] = None,
    separators: Optional[tuple[str, str]] = None,
    default: Optional[Any] = None,
    sort_keys: bool = False,
) -> str: ...

def load(
    fp: IO[str],
    *,
    cls: Optional[type] = None,
    object_hook: Optional[Any] = None,
    parse_float: Optional[Any] = None,
    parse_int: Optional[Any] = None,
    parse_constant: Optional[Any] = None,
    object_pairs_hook: Optional[Any] = None,
) -> Any: ...

def dump(
    obj: Any,
    fp: IO[str],
    *,
    skipkeys: bool = False,
    ensure_ascii: bool = True,
    check_circular: bool = True,
    allow_nan: bool = True,
    cls: Optional[type] = None,
    indent: Optional[int] = None,
    separators: Optional[tuple[str, str]] = None,
    default: Optional[Any] = None,
    sort_keys: bool = False,
) -> None: ...
"#;

        let mapping = parse_pyi(json_pyi, "json");

        // Verify crate mapping
        assert_eq!(mapping.rust_path, "serde_json");
        assert!(mapping.is_external);
        assert_eq!(mapping.version, Some("1.0".to_string()));

        // Verify function mappings
        assert_eq!(mapping.item_map.get("loads"), Some(&"from_str".to_string()));
        assert_eq!(mapping.item_map.get("dumps"), Some(&"to_string".to_string()));
        assert_eq!(mapping.item_map.get("load"), Some(&"from_reader".to_string()));
        assert_eq!(mapping.item_map.get("dump"), Some(&"to_writer".to_string()));
    }

    #[test]
    fn test_parse_simple_function() {
        let line = "def sqrt(x: float) -> float: ...";
        let func = parse_function_line(line).unwrap();

        assert_eq!(func.name, "sqrt");
        assert_eq!(func.params.len(), 1);
        assert_eq!(func.params[0], ("x".to_string(), "float".to_string()));
        assert_eq!(func.return_type, "float");
    }

    #[test]
    fn test_parse_function_with_defaults() {
        let line = "def round(number: float, ndigits: int = None) -> float: ...";
        let func = parse_function_line(line).unwrap();

        assert_eq!(func.name, "round");
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.params[0], ("number".to_string(), "float".to_string()));
        assert_eq!(func.params[1], ("ndigits".to_string(), "int".to_string()));
        assert_eq!(func.return_type, "float");
    }

    #[test]
    fn test_parse_function_with_generic_return() {
        let line = "def keys(self) -> list[str]: ...";
        let func = parse_function_line(line).unwrap();

        assert_eq!(func.name, "keys");
        assert_eq!(func.return_type, "list[str]");
    }

    #[test]
    fn test_extract_multiple_functions() {
        let content = r#"
def func_a(x: int) -> int: ...
def func_b(s: str) -> str: ...
def _private() -> None: ...
"#;

        let funcs = extract_function_signatures(content);

        // Should have 2 functions (private one excluded)
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].name, "func_a");
        assert_eq!(funcs[1].name, "func_b");
    }

    #[test]
    fn test_ingest_math_stub() {
        let math_pyi = r#"
def sqrt(x: float) -> float: ...
def sin(x: float) -> float: ...
def cos(x: float) -> float: ...
def floor(x: float) -> int: ...
def ceil(x: float) -> int: ...
def pow(x: float, y: float) -> float: ...
pi: float
e: float
"#;

        let mapping = parse_pyi(math_pyi, "math");

        assert_eq!(mapping.rust_path, "std::f64");
        assert!(!mapping.is_external);

        // Verify known mappings applied
        assert_eq!(mapping.item_map.get("sqrt"), Some(&"sqrt".to_string()));
        assert_eq!(mapping.item_map.get("sin"), Some(&"sin".to_string()));
        assert_eq!(mapping.item_map.get("cos"), Some(&"cos".to_string()));
        assert_eq!(mapping.item_map.get("pow"), Some(&"powf".to_string()));
    }

    #[test]
    fn test_ingest_os_stub() {
        let os_pyi = r#"
def getcwd() -> str: ...
def getenv(key: str, default: str = None) -> str: ...
def listdir(path: str = ".") -> list[str]: ...
"#;

        let mapping = parse_pyi(os_pyi, "os");

        assert_eq!(mapping.rust_path, "std");
        assert!(!mapping.is_external);

        assert_eq!(mapping.item_map.get("getcwd"), Some(&"env::current_dir".to_string()));
        assert_eq!(mapping.item_map.get("getenv"), Some(&"env::var".to_string()));
        assert_eq!(mapping.item_map.get("listdir"), Some(&"fs::read_dir".to_string()));
    }

    #[test]
    fn test_unknown_module_fallback() {
        let unknown_pyi = r#"
def custom_func(x: int) -> int: ...
"#;

        let mapping = parse_pyi(unknown_pyi, "unknown_module");

        // Should fallback to module name as crate
        assert_eq!(mapping.rust_path, "unknown_module");
        assert!(mapping.is_external);

        // Unknown function should map to itself
        assert_eq!(mapping.item_map.get("custom_func"), Some(&"custom_func".to_string()));
    }
}
