//! Type checking and inference helper methods for ExpressionConverter
//!
//! These methods are pure type-checking/inference helpers that determine
//! expression types, check type properties, and infer types from HIR expressions.
//! They do not perform expression conversion themselves.

use crate::hir::*;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::truthiness_helpers::is_option_var_name;
use anyhow::Result;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Check if the index expression is a string key (for HashMap access)
    /// Returns true if: index is string literal, OR index variable is string type
    /// DEPYLER-1060: Does NOT return true just because base is Dict - that could have non-string keys
    pub(crate) fn is_string_index(&self, base: &HirExpr, index: &HirExpr) -> Result<bool> {
        // Check 1: Is index a string literal?
        if matches!(index, HirExpr::Literal(Literal::String(_))) {
            return Ok(true);
        }

        // DEPYLER-1060: If index is an integer literal, this is NOT string indexing
        // Even for dicts - they might have integer keys like {1: "a"}
        if matches!(index, HirExpr::Literal(Literal::Int(_))) {
            return Ok(false);
        }

        // Check 2: Is base expression a Dict/HashMap type?
        // We need to look at the base's inferred type
        if let HirExpr::Var(sym) = base {
            // DEPYLER-0449: First check actual variable type if known
            if let Some(var_type) = self.ctx.var_types.get(sym) {
                // DEPYLER-1060: For Dict types, only use string indexing if index is a string variable
                // Not just because base is Dict - dict could have non-string keys
                if matches!(var_type, Type::Dict(_, _)) {
                    // Check if index is a string variable
                    return Ok(self.is_string_variable(index));
                }
            }

            // Try to find the variable's type in the current function context
            // For parameters, we can check the function signature
            // For local variables, this is harder without full type inference
            //
            // DEPYLER-0422: Removed "data" from heuristic - too broad, catches sorted_data, dataset, etc.
            // Only use "dict" or "map" which are more specific to HashMap variables
            let name = sym.as_str();
            if (name.contains("dict")
                || name.contains("map")
                || name.contains("config")
                || name.contains("value"))
                && !self.is_numeric_index(index)
            {
                return Ok(true);
            }
        }

        // Check 3: Does the index expression look like a string variable?
        if self.is_string_variable(index) {
            return Ok(true);
        }

        // Default: assume numeric index (Vec/List access)
        Ok(false)
    }

    /// Check if expression is likely a string variable or string-returning expression
    /// DEPYLER-1150: Also recognizes string-returning function calls like chr(), str(), etc.
    pub(crate) fn is_string_variable(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(sym) => {
                // DEPYLER-0449: First check actual variable type if known
                if let Some(var_type) = self.ctx.var_types.get(sym) {
                    // DEPYLER-99MODE-S9: If we have concrete type info, USE IT.
                    // Don't fall through to heuristics when type is known.
                    // e.g., `k: Int` from `for k in range(n)` must NOT match "k" heuristic.
                    return matches!(var_type, Type::String);
                }

                // Fallback to heuristics ONLY when type is truly unknown
                let name = sym.as_str();
                name == "key"
                    || name == "k"
                    || name == "name"
                    || name == "id"
                    || name == "word"
                    || name == "text"
                    || name.ends_with("_key")
                    || name.ends_with("_name")
            }
            // DEPYLER-1150: Recognize string-returning function calls
            HirExpr::Call { func, .. } => {
                // Python built-in functions that always return strings
                matches!(
                    func.as_str(),
                    "chr" | "str" | "repr" | "format" | "input" | "hex" | "oct" | "bin" | "ascii"
                )
            }
            // DEPYLER-1150: String method calls return strings
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "upper"
                        | "lower"
                        | "strip"
                        | "lstrip"
                        | "rstrip"
                        | "replace"
                        | "format"
                        | "join"
                        | "capitalize"
                        | "title"
                        | "swapcase"
                        | "center"
                        | "ljust"
                        | "rjust"
                        | "zfill"
                        | "encode"
                        | "decode"
                )
            }
            _ => false,
        }
    }

    /// Check if expression is likely numeric (heuristic)
    pub(crate) fn is_numeric_index(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => true,
            HirExpr::Var(sym) => {
                // DEPYLER-99MODE-S9: Check var_types first before name heuristic
                if let Some(var_type) = self.ctx.var_types.get(sym) {
                    return matches!(var_type, Type::Int | Type::Float | Type::Bool);
                }
                let name = sym.as_str();
                // Heuristic fallback only when type unknown
                name == "i"
                    || name == "j"
                    || name == "k"
                    || name == "idx"
                    || name == "index"
                    || name.starts_with("idx_")
                    || name.ends_with("_idx")
                    || name.ends_with("_index")
            }
            HirExpr::Binary { .. } => true, // Arithmetic expressions are numeric
            HirExpr::Call { .. } => false,  // Could be anything
            _ => false,
        }
    }

    /// DEPYLER-0299 Pattern #3: Check if base expression is a String type (heuristic)
    /// Returns true if base is likely a String/str type (not Vec/List)
    pub(crate) fn is_string_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(sym) => {
                // DEPYLER-0479: Check type system first (most reliable)
                if let Some(ty) = self.ctx.var_types.get(sym) {
                    // Return true if definitely String, false if definitely NOT string
                    // Fall through to heuristics for Unknown/Any types
                    match ty {
                        Type::String => return true,
                        // DEPYLER-0579: Optional<String> is still string-like
                        Type::Optional(inner) if matches!(**inner, Type::String) => return true,
                        Type::Int | Type::Float | Type::Bool | Type::List(_) | Type::Dict(_, _) => {
                            return false;
                        }
                        _ => {} // Unknown/Any - fall through to heuristics
                    }
                }

                // DEPYLER-0267 FIX: Only match singular string-like names, NOT plurals
                // "words" (plural) is likely list[str], not str!
                // "word" (singular) without 's' ending is likely str
                let name = sym.as_str();
                // Only match if: singular AND string-like name
                let is_singular = !name.ends_with('s');
                name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name == "content"     // DEPYLER-0538: File content is usually String
                    || name == "timestamp"  // GH-70: Common string field (ISO 8601, etc.)
                    || name == "message"     // GH-70: Log messages are strings
                    || name == "level"       // GH-70: Log levels are strings ("INFO", "ERROR")
                    || name == "prefix"      // String prefix for startswith operations
                    || name == "suffix"      // String suffix for endswith operations
                    || name == "pattern"     // String pattern for matching
                    || name == "char"        // Single character string
                    || name == "delimiter"   // String delimiter
                    || name == "separator"   // String separator
                    || (name == "word" && is_singular)
                    || (name.starts_with("text") && is_singular)
                    || (name.starts_with("str") && is_singular)
                    || (name.ends_with("_str") && is_singular)
                    || (name.ends_with("_string") && is_singular)
                    || (name.ends_with("_word") && is_singular)
                    || (name.ends_with("_text") && is_singular)
                    || (name.ends_with("timestamp") && is_singular)  // GH-70: created_timestamp, etc.
                    || (name.ends_with("_message") && is_singular) // GH-70: error_message, etc.
            }
            // DEPYLER-0577: Handle attribute access (e.g., args.text, args.prefix)
            HirExpr::Attribute { attr, .. } => {
                let name = attr.as_str();
                let is_singular = !name.ends_with('s');
                name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name == "content"
                    || name == "message"
                    || name == "prefix"      // String prefix for startswith operations
                    || name == "suffix"      // String suffix for endswith operations
                    || name == "pattern"     // String pattern for matching
                    || name == "char"        // Single character string
                    || name == "delimiter"   // String delimiter
                    || name == "separator"   // String separator
                    || name == "old"         // String replacement old value
                    || name == "new"         // String replacement new value
                    || (name.starts_with("text") && is_singular)
                    || (name.ends_with("_text") && is_singular)
                    || (name.ends_with("_string") && is_singular)
            }
            HirExpr::MethodCall { method, .. }
                if method.as_str().contains("upper")
                    || method.as_str().contains("lower")
                    || method.as_str().contains("strip")
                    || method.as_str().contains("lstrip")
                    || method.as_str().contains("rstrip")
                    || method.as_str().contains("title") =>
            {
                true
            }
            HirExpr::Call { func, .. } if func.as_str() == "str" => true,
            // DEPYLER-0573: Dict value access with string-like keys
            // Pattern: dict["hash"], dict.get("hash")... - these return string values
            HirExpr::Index { base, index } if self.is_dict_expr(base) => {
                // Check if key suggests string value
                if let HirExpr::Literal(Literal::String(key)) = index.as_ref() {
                    let k = key.to_lowercase();
                    k.contains("hash")
                        || k.contains("name")
                        || k.contains("path")
                        || k.contains("text")
                        || k.contains("message")
                        || k.contains("algorithm")
                        || k.contains("filename")
                        || k.contains("modified")
                } else {
                    false
                }
            }
            // DEPYLER-0573: Dict.get() chain with string-like keys
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if (method == "get" || method == "cloned" || method == "unwrap_or_default")
                && self.is_dict_value_access(object) =>
            {
                // If it's a get() call, check the key
                if method == "get" && !args.is_empty() {
                    if let HirExpr::Literal(Literal::String(key)) = &args[0] {
                        let k = key.to_lowercase();
                        return k.contains("hash")
                            || k.contains("name")
                            || k.contains("path")
                            || k.contains("text")
                            || k.contains("message")
                            || k.contains("algorithm")
                            || k.contains("filename")
                            || k.contains("modified");
                    }
                }
                // For cloned/unwrap_or_default, check the chain
                self.is_string_base(object)
            }
            _ => false,
        }
    }

    /// DEPYLER-0701: Check if base expression is a tuple type
    /// Used to detect tuple[idx] patterns that need special handling
    pub(crate) fn is_tuple_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Tuple(_) => true,
            HirExpr::Var(sym) => {
                // Check type system for Tuple type
                if let Some(ty) = self.ctx.var_types.get(sym) {
                    matches!(ty, Type::Tuple(_))
                } else {
                    // Heuristic: common tuple variable names
                    let name = sym.as_str();
                    matches!(
                        name,
                        "pair" | "tuple" | "entry" | "item" | "elem" | "row" | "t"
                    )
                }
            }
            // Method call returning tuple (e.g., dict.items() element)
            HirExpr::MethodCall { object, method, .. } => {
                // Enumerate returns (index, value) tuples
                if method == "enumerate" {
                    return true;
                }
                // Dict.items() returns (key, value) tuples
                if method == "items" && self.is_dict_expr(object) {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0701: Get the size of a tuple type for array conversion
    /// Returns the number of elements in the tuple, or None if unknown
    pub(crate) fn get_tuple_size(&self, expr: &HirExpr) -> Option<usize> {
        match expr {
            HirExpr::Tuple(elements) => Some(elements.len()),
            HirExpr::Var(sym) => {
                if let Some(Type::Tuple(types)) = self.ctx.var_types.get(sym) {
                    Some(types.len())
                } else {
                    None // Default will be used
                }
            }
            _ => None,
        }
    }

    pub(crate) fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_) => {
                // Check type information in context for variables
                self.is_set_var(expr)
            }
            _ => false,
        }
    }

    /// DEPYLER-0575: Check if expression is a numpy array (trueno Vector)
    pub(crate) fn is_numpy_array_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // np.array() call
            HirExpr::Call { func, .. } if func == "array" => true,
            // DEPYLER-1044: np.zeros(), np.ones() always return vectors
            HirExpr::Call { func, .. } if matches!(func.as_str(), "zeros" | "ones") => true,
            // DEPYLER-1044: abs, sqrt, etc. return vector ONLY if argument is vector
            // abs(scalar) -> scalar, abs(array) -> array
            HirExpr::Call { func, args, .. }
                if matches!(
                    func.as_str(),
                    "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" | "clip" | "clamp" | "normalize"
                ) =>
            {
                args.first()
                    .is_some_and(|arg| self.is_numpy_array_expr(arg))
            }
            // DEPYLER-1044: Method calls on numpy arrays return numpy arrays
            // BUT scalar.abs() returns scalar, not vector
            HirExpr::MethodCall { object, method, .. } => {
                // unwrap/abs/sqrt/etc preserve array nature of object
                if matches!(
                    method.as_str(),
                    "unwrap" | "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" | "clamp" | "clip"
                ) {
                    return self.is_numpy_array_expr(object);
                }
                false
            }
            // DEPYLER-0804: Check var_types first to avoid false positives
            // Variables with known scalar types (Float, Int) are NOT numpy arrays
            HirExpr::Var(name) => {
                // DEPYLER-1044: FIRST check CSE temps - they are NEVER numpy arrays
                // This must happen before any other checks to prevent false positives
                let n = name.as_str();
                if n.starts_with("_cse_temp_") {
                    return false;
                }

                // DEPYLER-0932: Check numpy_vars set (most reliable)
                // This tracks variables explicitly assigned from numpy operations
                if self.ctx.numpy_vars.contains(name) {
                    return true;
                }

                // Next check var_types for definitive type info
                if let Some(ty) = self.ctx.var_types.get(name) {
                    // Scalar types are never numpy arrays
                    if matches!(ty, Type::Float | Type::Int | Type::Bool | Type::String) {
                        return false;
                    }
                    // DEPYLER-1044: Check for Rust-specific scalar types stored as Custom
                    // Parameters with explicit type annotations (e.g., a: i32) are stored as
                    // Type::Custom("i32"), not Type::Int. These are also scalars!
                    if let Type::Custom(type_name) = ty {
                        let tn = type_name.as_str();
                        // Rust scalar types are NOT numpy arrays
                        if matches!(
                            tn,
                            "i8" | "i16"
                                | "i32"
                                | "i64"
                                | "i128"
                                | "isize"
                                | "u8"
                                | "u16"
                                | "u32"
                                | "u64"
                                | "u128"
                                | "usize"
                                | "f32"
                                | "f64"
                                | "bool"
                        ) {
                            return false;
                        }
                        // DEPYLER-0836: trueno::Vector<T> types are numpy arrays
                        if tn.starts_with("Vector<") || tn == "Vector" {
                            return true;
                        }
                    }
                    // DEPYLER-0955: Only treat List types as numpy arrays if they contain
                    // numeric primitives (Int, Float). Lists of tuples, strings, etc.
                    // should NOT use .copied() which requires Copy trait.
                    if let Type::List(inner) = ty {
                        // Only numeric inner types are numpy-like
                        if matches!(inner.as_ref(), Type::Int | Type::Float) {
                            return true;
                        }
                        // Non-numeric lists (tuples, strings, etc.) are NOT numpy arrays
                        return false;
                    }
                }
                // Fall back to name heuristics only for unknown types
                // DEPYLER-0804: Removed "x", "y" - too generic, often scalars
                // DEPYLER-1044: Removed "a", "b", "result" - WAY too generic, causes CSE failures
                // Only use truly unambiguous numpy-like names
                matches!(n, "arr" | "array" | "data" | "values" | "vec" | "vector")
                    || n.starts_with("arr_")
                    || n.ends_with("_arr")
                    || n.starts_with("vec_")
                    || n.ends_with("_vec")
            }
            // Recursive: binary op on vector yields vector
            HirExpr::Binary { left, .. } => self.is_numpy_array_expr(left),
            _ => false,
        }
    }

    /// DEPYLER-0188: Check if expression is a pathlib Path (std::path::PathBuf)
    ///
    /// Python's pathlib.Path uses `/` operator (via __truediv__) for path concatenation.
    /// Rust's PathBuf doesn't implement Div, so we convert to .join().
    pub(crate) fn is_path_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Path() or pathlib.Path() call
            HirExpr::Call { func, .. } => {
                matches!(
                    func.as_str(),
                    "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath"
                )
            }
            // Method calls that return paths
            // Note: "resolve" and "absolute" are NOT included because they are converted
            // with .to_string_lossy().to_string() and thus return String, not PathBuf
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "parent"
                        | "expanduser"
                        | "with_name"
                        | "with_suffix"
                        | "with_stem"
                        | "joinpath"
                )
            }
            // Attribute access like Path(__file__).parent
            HirExpr::Attribute { attr, .. } => {
                matches!(attr.as_str(), "parent" | "root" | "anchor")
            }
            // Variable named 'path' or with path-like semantics
            // DEPYLER-0188: Include common module-level path constants (SCRIPT, FILE, etc.)
            // DEPYLER-0930: Also check var_types for PathBuf type (e.g., result = Path(...))
            HirExpr::Var(name) => {
                // First check if variable is typed as PathBuf/Path
                let is_typed_path = self
                    .ctx
                    .var_types
                    .get(name)
                    .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                    .unwrap_or(false);
                if is_typed_path {
                    return true;
                }
                // Fall back to name-based heuristics
                let n = name.as_str();
                let n_lower = n.to_lowercase();
                matches!(
                    n,
                    "path"
                        | "filepath"
                        | "dir_path"
                        | "file_path"
                        | "base_path"
                        | "root_path"
                        | "SCRIPT"
                        | "SCRIPT_PATH"
                        | "SCRIPT_DIR"
                        | "SCRIPT_FILE"
                        | "ROOT"
                        | "ROOT_DIR"
                        | "ROOT_PATH"
                        | "BASE"
                        | "BASE_DIR"
                ) || n.starts_with("path_")
                    || n.ends_with("_path")
                    || n.starts_with("dir_")
                    || n.ends_with("_dir")
                    || n_lower.ends_with("_path")
                    || n_lower.ends_with("_dir")
                    || n_lower.starts_with("script")
            }
            // Recursive: path / segment is still a path
            HirExpr::Binary {
                left,
                op: BinOp::Div,
                ..
            } => self.is_path_expr(left),
            _ => false,
        }
    }

    /// DEPYLER-0607: Check if expression yields serde_json::Value that needs iteration conversion
    ///
    /// serde_json::Value doesn't implement IntoIterator, so we need to detect when
    /// the iteration expression is a JSON Value and wrap it with .as_array().
    ///
    /// Returns true for:
    /// - Variables with dict/JSON Value types in context
    /// - Method chains like data.get("items").cloned().unwrap_or_default()
    /// - Dict index expressions like data["items"]
    pub(crate) fn is_json_value_iteration(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable - check if it has a JSON/dict type in context
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, v) if
                        matches!(v.as_ref(), Type::Unknown) ||
                        matches!(v.as_ref(), Type::Custom(n) if n.contains("Value") || n.contains("json")))
                } else {
                    // Heuristic: if needs_serde_json is set, variables may be JSON Values
                    self.ctx.needs_serde_json
                }
            }
            // Dict index expression - likely yields JSON Value
            HirExpr::Index { base, .. } => {
                match base.as_ref() {
                    HirExpr::Var(var_name) => {
                        if let Some(t) = self.ctx.var_types.get(var_name) {
                            matches!(t, Type::Dict(_, v) if
                                matches!(v.as_ref(), Type::Unknown) ||
                                matches!(v.as_ref(), Type::Custom(n) if n.contains("Value") || n.contains("json")))
                        } else {
                            self.ctx.needs_serde_json
                        }
                    }
                    HirExpr::Dict(_) => true, // Dict literal
                    _ => false,
                }
            }
            // Method chains that yield JSON Value
            HirExpr::MethodCall { object, method, .. } => {
                let is_chain_method = matches!(
                    method.as_str(),
                    "get" | "cloned" | "unwrap_or_default" | "unwrap_or" | "unwrap"
                );
                if is_chain_method {
                    self.is_json_value_iteration(object.as_ref())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Check if a variable has a set type based on type information in context
    pub(crate) fn is_set_var(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types in context to see if this variable is a set
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Set(_))
                // DEPYLER-1060: Check module_constant_types for module-level static sets
                } else if let Some(var_type) = self.ctx.module_constant_types.get(name.as_str()) {
                    matches!(var_type, Type::Set(_))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0321: Check if expression is a string type
    /// Used to distinguish string.contains() from HashMap.contains_key()
    ///
    /// # Complexity
    /// 4 (match + type lookup + variant check + attribute check)
    pub(crate) fn is_string_type(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a string
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::String)
                } else {
                    // Fallback to heuristic for cases without type info
                    self.is_string_base(expr)
                }
            }
            // DEPYLER-0649: Handle attribute access for known string fields
            HirExpr::Attribute { attr, .. } => {
                // Known string attributes from common types:
                // - CompletedProcess.stdout, CompletedProcess.stderr
                // - Exception.args (often treated as string)
                // - argparse Namespace string fields
                matches!(
                    attr.as_str(),
                    "stdout" | "stderr" | "text" | "output" | "message" | "name"
                )
            }
            // DEPYLER-0675: Handle str() function call - returns String
            // Python: list(str(num)) → Rust: num.to_string().chars().collect()
            HirExpr::Call { func, .. } => {
                // str() builtin returns a string
                func == "str"
            }
            // DEPYLER-0676: Handle method calls that return strings
            // Python: list(num.to_string()) → Rust: num.to_string().chars().collect()
            HirExpr::MethodCall { method, .. } => {
                // Methods that return strings
                matches!(
                    method.as_str(),
                    "to_string" | "format" | "upper" | "lower" | "strip" | "replace" | "join"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0498: Check if expression is an Option type
    /// Used to determine if unwrap_or is needed in binary operations
    ///
    /// Returns true if:
    /// - Expression is a variable with Option<T> type
    /// - Expression is an attribute access that returns Option
    ///
    /// # Complexity
    /// 2 (match + type lookup)
    pub(crate) fn expr_is_option(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable: check if type is Optional
            HirExpr::Var(var_name) => {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
                    matches!(var_type, Type::Optional(_))
                } else {
                    false
                }
            }
            // Attribute access: check if field type is Optional
            HirExpr::Attribute { value, attr } => {
                // DEPYLER-0498: Check if self.field is Option in generator context
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    if obj_name == "self" && self.ctx.in_generator {
                        // Check if this field is a generator state variable with Optional type
                        if self.ctx.generator_state_vars.contains(attr) {
                            // Field is a generator state var - check its type in var_types
                            if let Some(field_type) = self.ctx.var_types.get(attr) {
                                return matches!(field_type, Type::Optional(_));
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #7: Check if expression is a dict/HashMap
    /// Used for dict merge operator (|) and other dict-specific operations
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    pub(crate) fn is_dict_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Dict(_) => true,
            HirExpr::Call { func, .. } if func == "dict" => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a dict/HashMap
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    match var_type {
                        Type::Dict(_, _) => true,
                        // DEPYLER-1004: Handle bare Dict from typing import (becomes Type::Custom("Dict"))
                        Type::Custom(s) if s == "Dict" => true,
                        // DEPYLER-1004: json.loads() returns serde_json::Value which is dict-like
                        // When assigned from json.loads(), variables get Type::Custom("serde_json::Value")
                        Type::Custom(s) if s == "serde_json::Value" || s == "Value" => true,
                        _ => false,
                    }
                // DEPYLER-1060: Check module_constant_types for module-level static dicts
                // var_types is cleared per-function, but module_constant_types persists
                } else if let Some(var_type) = self.ctx.module_constant_types.get(name.as_str()) {
                    matches!(var_type, Type::Dict(_, _))
                } else {
                    // DEPYLER-99MODE: Heuristic fallback for common dict variable names
                    let n = name.as_str();
                    n.contains("dict")
                        || n.contains("map")
                        || n.contains("hash")
                        || n == "memo"
                        || n == "seen"
                        || n == "visited"
                        || n == "counts"
                        || n == "freq"
                        || n == "frequency"
                        || n == "lookup"
                        || n == "graph"
                        || n == "adj"
                        || n == "dp"
                        || n == "cache"
                        || n == "config"
                        || n == "settings"
                        || n == "params"
                        || n == "options"
                        || n == "env"
                        || n.ends_with("_map")
                        || n.ends_with("_dict")
                        || n.ends_with("_cache")
                        || n.ends_with("_index")
                        || n.ends_with("_lookup")
                }
            }
            // DEPYLER-1044: Handle attribute access (e.g., self.config)
            HirExpr::Attribute { attr, .. } => {
                if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                    matches!(field_type, Type::Dict(_, _))
                        || matches!(field_type, Type::Custom(s) if s == "Dict")
                } else {
                    // Heuristic: common dict-like attribute names
                    let name = attr.as_str();
                    name == "config"
                        || name == "settings"
                        || name == "options"
                        || name == "data"
                        || name == "metadata"
                        || name == "headers"
                        || name == "params"
                        || name == "kwargs"
                        || name.ends_with("_dict")
                        || name.ends_with("_map")
                }
            }
            // DEPYLER-1189: Handle dict-of-dicts indexing (e.g., distances[current])
            // If base is a dict with dict value type, the result is also a dict
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(name) = base.as_ref() {
                    if let Some(Type::Dict(_, value_type)) = self.ctx.var_types.get(name) {
                        // If the value type is also a Dict, indexing returns a dict
                        matches!(value_type.as_ref(), Type::Dict(_, _))
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            // GH-207-PHASE2: Handle method calls that preserve dict type
            // dict.clone() and dict.copy() return a dict, so items()/keys()/values() should work
            HirExpr::MethodCall { object, method, .. } => {
                // Methods that return the same dict type
                if method == "clone" || method == "copy" {
                    self.is_dict_expr(object)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0572: Check if expression is a dict value access (returns serde_json::Value)
    /// Pattern: dict[key] or dict.get(key).cloned().unwrap_or_default()
    /// These return Value which needs .to_string() when mixed with String in lists
    pub(crate) fn is_dict_value_access(&self, expr: &HirExpr) -> bool {
        match expr {
            // dict[key] index access
            HirExpr::Index { base, .. } => self.is_dict_expr(base),
            // dict.get(key)... chain
            HirExpr::MethodCall { object, method, .. } => {
                if method == "get" {
                    self.is_dict_expr(object)
                } else if method == "cloned" || method == "unwrap_or_default" || method == "unwrap"
                {
                    // Check the chain for dict access
                    self.is_dict_value_access(object)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0540: Check if expression is typed as serde_json::Value
    /// serde_json::Value needs special handling for .keys(), .values(), .items()
    /// because it requires .as_object().unwrap() before iteration methods.
    /// DEPYLER-0969: H₃ Error Cascade Prevention - Type::Unknown maps to serde_json::Value
    /// so ALL Unknown-typed variables should use JSON method translations.
    /// DEPYLER-1017: In NASA mode, skip serde_json - Unknown maps to String
    pub(crate) fn is_serde_json_value(&self, expr: &HirExpr) -> bool {
        // DEPYLER-1017: In NASA mode, never treat anything as serde_json::Value
        if self.ctx.type_mapper.nasa_mode {
            return false;
        }
        if let HirExpr::Var(name) = expr {
            // Check explicit type info first - this is authoritative
            if let Some(var_type) = self.ctx.var_types.get(name) {
                // Check for explicit serde_json::Value type
                if matches!(var_type, Type::Custom(ref s) if s == "serde_json::Value") {
                    return true;
                }
                // DEPYLER-0708: Removed overly aggressive check for Dict(_, Unknown)
                // A plain `dict` annotation creates Dict(Unknown, Unknown) but should NOT
                // trigger serde_json::Value treatment. Only explicit serde_json::Value should.
                // Dict types use regular HashMap.iter(), not .as_object().
                if matches!(var_type, Type::Dict(_, _)) {
                    return false;
                }
                // DEPYLER-0969: H₃ Error Cascade Prevention
                // Type::Unknown maps to serde_json::Value in type_mapper.rs
                // Therefore, ALL Unknown-typed variables need JSON method translations
                // to prevent E0599 "no method named X found" cascading errors
                if matches!(var_type, Type::Unknown) {
                    return true;
                }
                // For other explicitly typed variables, not a JSON value
                return false;
            }

            // DEPYLER-0540: Use name heuristic when NO type info
            // (e.g., in nested closures where parent param types aren't tracked)
            // Be conservative - only match explicitly json-like names
            // Note: "filters", "config" are commonly used for serde_json::Value dicts
            let is_json_by_name = matches!(
                name.as_str(),
                "filters" | "json_data" | "json_obj" | "json_value" | "json_config" | "config"
            );
            if is_json_by_name {
                return true;
            }
        }
        false
    }

    /// DEPYLER-0550: Check if expression could be a serde_json::Value
    /// Used for comparison handling when .get() returns Option<String>
    /// but the other side is a JSON Value from .items() iteration
    pub(crate) fn is_serde_json_value_expr(&self, expr: &HirExpr) -> bool {
        // First check using the existing helper
        if self.is_serde_json_value(expr) {
            return true;
        }

        // DEPYLER-0550: Check for pattern variables from JSON iteration
        // When iterating over filters.items(), we get (col, val) where val is Value
        // The variable "val" in this context is a JSON Value
        if let HirExpr::Var(name) = expr {
            // Variables commonly used for JSON values in iteration patterns
            // "val" is the most common from: for col, val in filters.items()
            if matches!(name.as_str(), "val" | "v" | "value" | "json_val") {
                // Additional context check: if there's no type info, assume JSON in iteration
                if !self.ctx.var_types.contains_key(name) {
                    return true;
                }
            }
        }

        false
    }

    /// DEPYLER-0700: Check if dict expression has serde_json::Value values
    ///
    /// Returns true if the dict maps to HashMap<String, serde_json::Value>,
    /// which happens when:
    /// - Dict has heterogeneous value types (e.g., {"name": "Alice", "age": 42})
    /// - Dict value type is Unknown (untyped dict)
    /// - Dict uses serde_json expressions
    ///
    /// This is used to wrap default values in dict.get(key, default) with json!()
    /// for type compatibility.
    /// DEPYLER-1017: In NASA mode, never use serde_json::Value
    pub(crate) fn dict_has_json_value_values(&self, expr: &HirExpr) -> bool {
        // DEPYLER-1017: In NASA mode, dicts never have JSON values
        if self.ctx.type_mapper.nasa_mode {
            return false;
        }
        match expr {
            // Variable dict - check type info
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Dict with Unknown value type uses serde_json::Value
                    if matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::Unknown)) {
                        return true;
                    }
                    // Custom type that is serde_json::Value or HashMap with Value
                    if matches!(var_type, Type::Custom(ref s) if s.contains("serde_json::Value") || (s.contains("HashMap") && s.contains("Value")))
                    {
                        return true;
                    }
                }
                // If serde_json is already needed, this dict might use Value
                // Conservative: if we're generating serde_json code, assume mixed types
                self.ctx.needs_serde_json
            }
            // Dict literal - check if it has mixed value types
            HirExpr::Dict(items) => {
                if let Ok(has_mixed) = self.dict_has_mixed_types(items) {
                    has_mixed
                } else {
                    // Error checking - assume needs json for safety
                    self.ctx.needs_serde_json
                }
            }
            // Method call on dict - check base object
            HirExpr::MethodCall { object, .. } => self.dict_has_json_value_values(object),
            // Index into another dict
            HirExpr::Index { base, .. } => self.dict_has_json_value_values(base),
            _ => {
                // Fallback: if serde_json is in use, assume might be Value type
                self.ctx.needs_serde_json
            }
        }
    }

    /// DEPYLER-0729: Check if dict value type is String (not &str)
    /// Used to determine if string literal defaults in dict.get() need .to_string()
    /// GH-226: Default to true when type unknown - HashMap<String, String> is most common
    /// and calling .to_string() on String just clones (harmless), while NOT calling it
    /// on String values causes compile errors
    pub(crate) fn dict_value_type_is_string(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // String values or unknown dict value type - need .to_string() for safety
                    matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::String | Type::Unknown))
                        || !matches!(var_type, Type::Dict(_, _))
                } else {
                    // GH-226: Unknown type - assume String (safer default)
                    true
                }
            }
            HirExpr::MethodCall { object, .. } => self.dict_value_type_is_string(object),
            HirExpr::Index { base, .. } => self.dict_value_type_is_string(base),
            // GH-226: Unknown expression type - assume String (safer default)
            _ => true,
        }
    }

    /// DEPYLER-1319: Check if dict has DepylerValue values (requires .into() for type conversion)
    /// In NASA mode, dicts with Unknown/Any value types use DepylerValue as the value type.
    /// When accessing such dicts, we need .into() to convert to the expected primitive type.
    pub(crate) fn dict_has_depyler_value_values(&self, expr: &HirExpr) -> bool {
        // In NASA mode, check if the dict value type is Unknown (maps to DepylerValue)
        if !self.ctx.type_mapper.nasa_mode {
            return false;
        }
        match expr {
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Dict with Unknown value type → DepylerValue in NASA mode
                    matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::Unknown))
                } else {
                    // Unknown variable in NASA mode - assume DepylerValue values
                    true
                }
            }
            HirExpr::MethodCall { object, .. } => self.dict_has_depyler_value_values(object),
            HirExpr::Index { base, .. } => self.dict_has_depyler_value_values(base),
            // For function parameters like `data: dict`, assume DepylerValue in NASA mode
            _ => true,
        }
    }

    pub(crate) fn is_list_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::List(_) => true,
            HirExpr::Call { func, .. } if func == "list" => true,
            // DEPYLER-0811: Function calls that return list types
            HirExpr::Call { func, .. } => {
                if let Some(ret_type) = self.ctx.function_return_types.get(func) {
                    matches!(ret_type, Type::List(_))
                } else {
                    false
                }
            }
            HirExpr::Var(name) => {
                // DEPYLER-169: Check var_types for List type
                // This enables proper `item in list_var` detection
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::List(_))
                // DEPYLER-1060: Check module_constant_types for module-level static lists
                } else if let Some(var_type) = self.ctx.module_constant_types.get(name.as_str()) {
                    matches!(var_type, Type::List(_))
                } else {
                    // Fall back to conservative: only treat explicit list literals as lists
                    false
                }
            }
            // DEPYLER-0811: Binary Add of lists produces a list (for chained concat)
            HirExpr::Binary {
                op: BinOp::Add,
                left,
                right,
            } => self.is_list_expr(left) || self.is_list_expr(right),
            // DEPYLER-1044: Handle attribute access (e.g., self.permissions)
            // Check class_field_types for the attribute's type
            HirExpr::Attribute { attr, .. } => {
                if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                    matches!(field_type, Type::List(_))
                } else {
                    // Heuristic: common list-like attribute names
                    let name = attr.as_str();
                    name.ends_with("s") && !name.ends_with("ss")
                        || name.ends_with("list")
                        || name.ends_with("items")
                        || name.ends_with("elements")
                        || name == "permissions"
                        || name == "values"
                        || name == "keys"
                        || name == "children"
                        || name == "args"
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0742: Check if expression is a deque type (VecDeque)
    /// Used to generate correct VecDeque methods instead of Vec methods.
    pub(crate) fn is_deque_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call to deque() constructor
            HirExpr::Call { func, .. }
                if func == "deque" || func == "collections.deque" || func == "Deque" =>
            {
                true
            }
            HirExpr::Var(name) => {
                // Check var_types for Deque type annotation
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Check if the type string contains "deque" or "VecDeque"
                    let type_str = format!("{:?}", var_type);
                    type_str.contains("deque") || type_str.contains("VecDeque")
                } else {
                    // Fallback: common deque variable names
                    matches!(
                        name.as_str(),
                        "d" | "dq" | "deque" | "queue" | "buffer" | "deck"
                    )
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0521: Check if variable is a borrowed string parameter (&str)
    ///
    /// Function parameters with Python `str` type annotation become `&str` in Rust.
    /// When used as dict keys, they should NOT have `&` added (already borrowed).
    ///
    /// Heuristic: If variable not in var_types and has a string-key-like name,
    /// it's likely a function parameter that's &str.
    ///
    /// # Complexity
    /// 2 (lookup + name check)
    pub(crate) fn is_borrowed_str_param(&self, var_name: &str) -> bool {
        // DEPYLER-0543: Check if variable is a function param with str type
        // These become &str in Rust and should NOT have & added
        if self.ctx.fn_str_params.contains(var_name) {
            // DEPYLER-99MODE-S9: If the param is MUTATED, it was promoted from
            // &str to mut String (takes ownership). In that case it's no longer
            // borrowed, so DO add & when used as dict key.
            if self.ctx.mutable_vars.contains(var_name) {
                return false; // promoted to owned String, needs borrowing
            }
            return true; // already &str, don't add &
        }

        // When we have type info, use it
        if let Some(var_type) = self.ctx.var_types.get(var_name) {
            match var_type {
                Type::String => {
                    // Variable has Type::String but is NOT in fn_str_params
                    // This means it's a local variable (loop var, assignment) → owned String
                    return false; // needs borrowing
                }
                Type::Unknown => {
                    // Unknown type - use name heuristic as fallback
                }
                _ => {
                    // Other types - likely not a string key situation
                    return false;
                }
            }
        }

        // DEPYLER-0550: Removed "col" from heuristic - commonly used as loop variable
        // when iterating over dict items: for col, val in filters.items()
        // In that context, col is owned String from k.clone(), NOT a borrowed param
        // No type info or Unknown type - use name heuristics for function params
        // These are function parameters that typically become &str in Rust
        // Keep list minimal - only include names that are DEFINITELY function params
        let fn_param_names = matches!(var_name, "column" | "field" | "attr" | "property");

        if fn_param_names {
            return true;
        }

        // Variable not in var_types and not a known borrowed name
        // Default: assume needs borrowing (safer)
        false
    }

    /// DEPYLER-0496: Check if expression returns a Result type
    /// Used to determine if ? operator is needed in binary operations
    ///
    /// Returns true if:
    /// - Expression is a function call to a Result-returning function
    /// - Expression is a method call that might return Result
    ///
    /// # Complexity
    /// 2 (match + HashSet lookup)
    pub(crate) fn expr_returns_result(&self, expr: &HirExpr) -> bool {
        match expr {
            // Function calls: check if function is tracked as Result-returning
            HirExpr::Call { func, .. } => self.ctx.result_returning_functions.contains(func),
            // Method calls: Some method calls return Result (e.g., parse(), read_to_string())
            // For now, be conservative and don't assume method calls return Result
            // This can be enhanced later with specific method tracking
            HirExpr::MethodCall { .. } => false,
            // Other expressions don't return Result
            _ => false,
        }
    }

    /// DEPYLER-0575: Check if expression returns a float type
    /// Used to coerce integer literals to floats in comparisons
    pub(crate) fn expr_returns_float(&self, expr: &HirExpr) -> bool {
        match expr {
            // Float literals
            HirExpr::Literal(Literal::Float(_)) => true,
            // Variable with Float type, or variable from numpy float methods
            HirExpr::Var(name) => {
                // DEPYLER-1026: If we have explicit type info, use it exclusively
                // Don't fall through to heuristics when type is known
                if let Some(ty) = self.ctx.var_types.get(name) {
                    return matches!(ty, Type::Float);
                }
                // Common float result variable names from numpy operations
                // ONLY used when no type info is available
                // DEPYLER-0668: Remove "result" - too general, often used for ints/bools
                // DEPYLER-0927: Sync with expr_returns_f32 - include norm_a, norm_b, dot etc.
                // DEPYLER-0928: Added min_val, max_val for Vector-scalar operations
                matches!(
                    name.as_str(),
                    "mean"
                        | "std"
                        | "variance"
                        | "sum"
                        | "norm"
                        | "norm_a"
                        | "norm_b"
                        | "stddev"
                        | "var"
                        | "denom"
                        | "dot"
                        | "min_val"
                        | "max_val"
                )
            }
            // DEPYLER-0577: Attribute access (e.g., args.x) - check if attr is float type
            // DEPYLER-0720: Also check class_field_types for self.X attribute access
            HirExpr::Attribute { attr, value, .. } => {
                // Check var_types first (for non-self attributes)
                if matches!(self.ctx.var_types.get(attr), Some(Type::Float)) {
                    return true;
                }
                // Check class_field_types for self.X patterns
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
                    && matches!(self.ctx.class_field_types.get(attr), Some(Type::Float))
                {
                    return true;
                }
                false
            }
            // NumPy/trueno methods that return f32
            // DEPYLER-0927: Added norm_l2 and dot for trueno compatibility
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "mean"
                        | "sum"
                        | "std"
                        | "stddev"
                        | "var"
                        | "variance"
                        | "min"
                        | "max"
                        | "norm"
                        | "norm_l2"
                        | "dot"
                )
            }
            // DEPYLER-0799: Function calls - check return type from function_return_types
            // This handles cases like f(a) * f(b) > 0 where f returns float
            HirExpr::Call { func, .. } => {
                // Check module-level function return types first
                if let Some(ret_type) = self.ctx.function_return_types.get(func) {
                    if matches!(ret_type, Type::Float) {
                        return true;
                    }
                }
                // DEPYLER-0800: Check if func is a Callable parameter with Float return type
                // Example: f: Callable[[float], float] -> f(x) returns Float
                if let Some(Type::Function { ret, .. }) = self.ctx.var_types.get(func) {
                    if matches!(ret.as_ref(), Type::Float) {
                        return true;
                    }
                }
                // Callable is stored as Generic { base: "Callable", params: [param_types, return_type] }
                if let Some(Type::Generic { base, params }) = self.ctx.var_types.get(func) {
                    if base == "Callable" && params.len() == 2 && matches!(params[1], Type::Float) {
                        return true;
                    }
                }
                // Also check for math builtin functions that return float
                // DEPYLER-0816: Removed "abs" - Python abs() preserves input type (int→int, float→float)
                // The math functions below ALWAYS return float, but abs() is type-preserving
                matches!(
                    func.as_str(),
                    "sqrt"
                        | "sin"
                        | "cos"
                        | "tan"
                        | "exp"
                        | "log"
                        | "log10"
                        | "log2"
                        | "floor"
                        | "ceil"
                        | "pow"
                        | "float"
                )
            }
            // DEPYLER-0694: Binary expression with float operand returns float
            // This handles chained operations like (principal * rate) * years
            HirExpr::Binary { left, right, .. } => {
                self.expr_returns_float(left)
                    || self.expr_returns_float(right)
                    || self.is_float_var(left)
                    || self.is_float_var(right)
            }
            _ => false,
        }
    }

    /// DEPYLER-0920: Check if expression returns f32 specifically (trueno/numpy results)
    /// Used to generate f32 literals instead of f64 in comparisons
    /// DEPYLER-0927: Synced with expr_returns_float for consistent detection
    pub(crate) fn expr_returns_f32(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable names commonly used for trueno f32 results
            HirExpr::Var(name) => {
                // DEPYLER-1026: If we have explicit type info, don't use heuristics
                if self.ctx.var_types.contains_key(name) {
                    return false; // f32 detection is only for trueno contexts without type info
                }
                matches!(
                    name.as_str(),
                    "mean"
                        | "std"
                        | "variance"
                        | "sum"
                        | "norm"
                        | "norm_a"
                        | "norm_b"
                        | "stddev"
                        | "var"
                        | "denom"
                        | "dot"
                )
            }
            // Method calls on trueno Vectors return f32
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "mean"
                        | "sum"
                        | "std"
                        | "stddev"
                        | "var"
                        | "variance"
                        | "min"
                        | "max"
                        | "norm"
                        | "norm_l2"
                        | "dot"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-1085: Check if expression returns DepylerValue type
    /// Used for Value Lifting in if/else branch unification
    pub(crate) fn expr_returns_depyler_value(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variables with Unknown type become DepylerValue
            // DEPYLER-1316: In NASA mode, also return true for variables not in var_types
            // (like loop variables) since they likely came from iterating over DepylerValue
            HirExpr::Var(name) => {
                let var_type = self.ctx.var_types.get(name);
                match var_type {
                    Some(Type::Unknown) | Some(Type::UnificationVar(_)) => true,
                    // DEPYLER-1316: In NASA mode, any untracked or ambiguous variable
                    // likely came from DepylerValue
                    None if self.ctx.type_mapper.nasa_mode => {
                        // Exception: Don't assume system-level vars like `_dv_*` are DepylerValue
                        !name.starts_with("_dv_")
                            && !name.starts_with("_cse_")
                            && !name.starts_with("_range_")
                    }
                    // DEPYLER-1316: Also check for custom types that map to DepylerValue
                    Some(Type::Custom(type_name)) if self.ctx.type_mapper.nasa_mode => {
                        // Custom types in NASA mode are typically DepylerValue wrappers
                        type_name.contains("DepylerValue") || type_name == "Unknown"
                    }
                    _ => false,
                }
            }
            // Index access on collections with Unknown element type
            // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
            // DEPYLER-1209: Also check for UnificationVar
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(name) = base.as_ref() {
                    if let Some(ty) = self.ctx.var_types.get(name) {
                        return match ty {
                            Type::List(elem) => {
                                matches!(**elem, Type::Unknown | Type::UnificationVar(_))
                            }
                            Type::Dict(_, value) => {
                                matches!(**value, Type::Unknown | Type::UnificationVar(_))
                            }
                            _ => false,
                        };
                    }
                }
                false
            }
            // Method calls on Unknown-typed objects
            // DEPYLER-1316: Recursively check if object returns DepylerValue
            // This handles chained calls like: record.get("s3").cloned().unwrap_or_default().get("bucket")
            HirExpr::MethodCall { object, method, .. } => {
                // Check if the object itself returns DepylerValue
                if self.expr_returns_depyler_value(object) {
                    // Methods that preserve DepylerValue type
                    return matches!(
                        method.as_str(),
                        "get" | "get_str" | "cloned" | "clone" | "unwrap_or_default" | "unwrap"
                    );
                }
                // DEPYLER-1316: Recursively check the object via Var case logic
                if let HirExpr::Var(name) = object.as_ref() {
                    let var_type = self.ctx.var_types.get(name);
                    return match var_type {
                        Some(Type::Unknown) | Some(Type::UnificationVar(_)) => true,
                        None if self.ctx.type_mapper.nasa_mode => {
                            !name.starts_with("_dv_")
                                && !name.starts_with("_cse_")
                                && !name.starts_with("_range_")
                        }
                        Some(Type::Custom(type_name)) if self.ctx.type_mapper.nasa_mode => {
                            type_name.contains("DepylerValue") || type_name == "Unknown"
                        }
                        _ => false,
                    };
                }
                false
            }
            // IfExpr where either branch returns DepylerValue
            HirExpr::IfExpr { body, orelse, .. } => {
                self.expr_returns_depyler_value(body) || self.expr_returns_depyler_value(orelse)
            }
            _ => false,
        }
    }

    /// DEPYLER-1085: Check if expression returns a concrete (non-DepylerValue) type
    /// Returns the concrete type if known, None if Unknown/DepylerValue
    #[allow(dead_code)] // May be used for future value lifting optimizations
    pub(crate) fn expr_concrete_type(&self, expr: &HirExpr) -> Option<Type> {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => Some(Type::Int),
            HirExpr::Literal(Literal::Float(_)) => Some(Type::Float),
            HirExpr::Literal(Literal::String(_)) => Some(Type::String),
            HirExpr::Literal(Literal::Bool(_)) => Some(Type::Bool),
            HirExpr::Var(name) => self.ctx.var_types.get(name).and_then(|ty| {
                if matches!(ty, Type::Unknown) {
                    None
                } else {
                    Some(ty.clone())
                }
            }),
            _ => None,
        }
    }

    /// DEPYLER-1085: Wrap a concrete expression in DepylerValue::from()
    /// Used for Value Lifting when branch types don't match
    pub(crate) fn lift_to_depyler_value(&self, expr: &HirExpr, rust_expr: syn::Expr) -> syn::Expr {
        // Use DepylerValue::from() which handles most common types
        // For specific types, use explicit variants for better performance
        match expr {
            HirExpr::Literal(Literal::Int(_)) => {
                parse_quote! { DepylerValue::Int(#rust_expr as i64) }
            }
            HirExpr::Literal(Literal::Float(_)) => {
                parse_quote! { DepylerValue::Float(#rust_expr as f64) }
            }
            HirExpr::Literal(Literal::String(_)) => {
                parse_quote! { DepylerValue::Str(#rust_expr.to_string()) }
            }
            HirExpr::Literal(Literal::Bool(_)) => {
                parse_quote! { DepylerValue::Bool(#rust_expr) }
            }
            HirExpr::Var(name) => {
                // Check the concrete type and wrap appropriately
                if let Some(ty) = self.ctx.var_types.get(name) {
                    match ty {
                        Type::Int => parse_quote! { DepylerValue::Int(#rust_expr as i64) },
                        Type::Float => parse_quote! { DepylerValue::Float(#rust_expr as f64) },
                        Type::String => parse_quote! { DepylerValue::Str(#rust_expr.to_string()) },
                        Type::Bool => parse_quote! { DepylerValue::Bool(#rust_expr) },
                        Type::List(_) => {
                            parse_quote! { DepylerValue::List(#rust_expr.into_iter().map(|x| DepylerValue::from(x)).collect()) }
                        }
                        _ => parse_quote! { DepylerValue::from(#rust_expr) },
                    }
                } else {
                    parse_quote! { DepylerValue::from(#rust_expr) }
                }
            }
            // For method calls that return known types
            _ if self.expr_returns_float(expr) => {
                parse_quote! { DepylerValue::Float(#rust_expr as f64) }
            }
            // Default: use DepylerValue::from() which requires Into trait
            _ => {
                parse_quote! { DepylerValue::Str(format!("{:?}", #rust_expr)) }
            }
        }
    }

    /// DEPYLER-0786: Check if expression is a string type
    /// Used to determine if `or` operator should return string instead of bool
    /// DEPYLER-CI-FIX: Also handles Binary Add expressions for string concatenation chains
    pub(crate) fn expr_is_string_type(&self, expr: &HirExpr) -> bool {
        match expr {
            // String literals
            HirExpr::Literal(Literal::String(_)) => true,
            // Variable with String type
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::String))
            }
            // Attribute access with String type
            HirExpr::Attribute { attr, .. } => {
                matches!(self.ctx.var_types.get(attr), Some(Type::String))
            }
            // Method calls that return strings
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "strip" | "lower" | "upper" | "replace" | "join" | "format"
                )
            }
            // Binary Add with string operands is string concatenation
            HirExpr::Binary { left, right, op } => {
                matches!(op, BinOp::Add)
                    && (self.expr_is_string_type(left) || self.expr_is_string_type(right))
            }
            // Function calls that return strings
            // DEPYLER-STRING-FUNC-FIX: func is Symbol (String), not HirExpr
            HirExpr::Call { func, .. } => {
                matches!(func.as_str(), "str" | "format" | "repr")
            }
            _ => false,
        }
    }

    /// DEPYLER-99MODE-S9: Check if expression is float-typed.
    /// Used to determine chain cast type (as f64 vs as i32) in py_ops chains.
    pub(crate) fn expr_is_float_type(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Float(_)) => true,
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::Float))
            }
            HirExpr::Index { base, .. } => {
                // List[float] element access
                if let HirExpr::Var(name) = &**base {
                    matches!(
                        self.ctx.var_types.get(name),
                        Some(Type::List(inner)) if matches!(&**inner, Type::Float)
                    )
                } else {
                    false
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.expr_is_float_type(left) || self.expr_is_float_type(right)
            }
            HirExpr::Call { func, .. } => {
                matches!(
                    self.ctx.function_return_types.get(func),
                    Some(Type::Float)
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-1127: Check if expression is a boolean-returning expression
    /// Used to determine if `or`/`and` should return bool or value
    ///
    /// Returns true for:
    /// - Boolean literals: True, False
    /// - Comparison expressions: a > b, a == b, etc.
    /// - Logical not: not x
    /// - Type checks: isinstance(), hasattr()
    /// - in/not in expressions
    ///
    /// Returns false for expressions that return non-boolean values
    pub(crate) fn expr_is_boolean_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Boolean literals always return bool
            HirExpr::Literal(Literal::Bool(_)) => true,
            // Comparison operators return bool
            HirExpr::Binary { op, .. } => matches!(
                op,
                BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn
            ),
            // not x returns bool
            HirExpr::Unary {
                op: UnaryOp::Not, ..
            } => true,
            // isinstance, hasattr, callable return bool
            HirExpr::Call { func, .. } => {
                matches!(
                    func.as_str(),
                    "isinstance" | "hasattr" | "callable" | "issubclass" | "all" | "any" | "bool"
                )
            }
            // Method calls that return bool
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "startswith"
                        | "endswith"
                        | "isalpha"
                        | "isdigit"
                        | "isalnum"
                        | "isspace"
                        | "islower"
                        | "isupper"
                        | "istitle"
                        | "isidentifier"
                        | "isnumeric"
                        | "isdecimal"
                        | "isascii"
                        | "is_empty"
                        | "is_some"
                        | "is_none"
                        | "is_ok"
                        | "is_err"
                        | "contains"
                        | "exists"
                )
            }
            // Variables with Bool type
            HirExpr::Var(name) => matches!(self.ctx.var_types.get(name), Some(Type::Bool)),
            _ => false,
        }
    }

    /// DEPYLER-1127: Check if expression is DepylerValue type
    /// Used to determine if `or`/`and` needs DepylerValue wrapping
    pub(crate) fn expr_is_depyler_value(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variables with Unknown type default to DepylerValue
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::Unknown))
            }
            // Dict/List subscript often returns DepylerValue
            HirExpr::Index { base, .. } => {
                if let HirExpr::Var(name) = base.as_ref() {
                    // Dict access returns DepylerValue for heterogeneous dicts
                    matches!(
                        self.ctx.var_types.get(name),
                        Some(Type::Dict(_, _)) | Some(Type::Unknown)
                    )
                } else {
                    false
                }
            }
            // Method calls on collections that return DepylerValue
            HirExpr::MethodCall { object, method, .. } => {
                if method == "get" || method == "pop" || method == "cloned" {
                    // Check if object is a dict with DepylerValue values
                    if let HirExpr::Var(name) = object.as_ref() {
                        matches!(
                            self.ctx.var_types.get(name),
                            Some(Type::Dict(_, _)) | Some(Type::Unknown)
                        )
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-1127: Heuristic check if expression MIGHT return DepylerValue
    /// More permissive than expr_is_depyler_value - used to safely wrap literals
    /// in DepylerValue when there's uncertainty.
    ///
    /// Returns true for:
    /// - All cases from expr_is_depyler_value
    /// - Method chains with .get(), .cloned(), .unwrap_or() patterns
    /// - Variables not in var_types (unknown type)
    pub(crate) fn expr_might_be_depyler_value(&self, expr: &HirExpr) -> bool {
        // First check the strict version
        if self.expr_is_depyler_value(expr) {
            return true;
        }

        match expr {
            // Method chains that commonly return DepylerValue
            HirExpr::MethodCall { method, object, .. } => {
                // Check if this method typically returns uncertain types
                let uncertain_methods = matches!(
                    method.as_str(),
                    "get" | "cloned" | "unwrap_or" | "unwrap_or_default" | "pop" | "to_string"
                );
                if uncertain_methods {
                    return true;
                }
                // Recursively check the object
                self.expr_might_be_depyler_value(object)
            }
            // Variables not tracked in var_types - could be anything
            HirExpr::Var(name) => !self.ctx.var_types.contains_key(name),
            // Any subscript access could return DepylerValue
            HirExpr::Index { .. } => true,
            // Any attribute access on unknown objects
            HirExpr::Attribute { value, .. } => self.expr_might_be_depyler_value(value),
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #6: Check if expression is an owned collection
    /// Used to determine if zip() should use .into_iter() (owned) vs .iter() (borrowed)
    ///
    /// Returns true if:
    /// - Expression is a Var with type List (Vec<T>) - function parameters are owned
    /// - Expression is a list literal - always owned
    /// - Expression is a list() call - creates owned Vec
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    pub(crate) fn is_owned_collection(&self, expr: &HirExpr) -> bool {
        match expr {
            // List literals are always owned
            HirExpr::List(_) => true,
            // list() calls create owned Vec
            HirExpr::Call { func, .. } if func == "list" => true,
            // Check if variable has List type (function parameters of type Vec<T>)
            HirExpr::Var(name) => {
                if let Some(ty) = self.ctx.var_types.get(name) {
                    matches!(ty, Type::List(_))
                } else {
                    // No type info - conservative default is borrowed
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-1153: Check if a type is "concrete" (not Unknown/Any/DepylerValue)
    /// Used to determine if a Dict return type should use type-preserving codegen
    /// instead of DepylerValue wrapping.
    ///
    /// A type is concrete if it's:
    /// - A scalar type (Int, Float, String, Bool)
    /// - A Dict/List/Tuple with concrete element types (recursive)
    /// - NOT Unknown, Any, or DepylerValue
    #[inline]
    pub(super) fn is_concrete_type(&self, ty: &Type) -> bool {
        match ty {
            // Scalar types are concrete
            Type::Int | Type::Float | Type::String | Type::Bool | Type::None => true,

            // Nested collections are concrete if their element types are concrete
            Type::Dict(key_type, val_type) => {
                self.is_concrete_type(key_type) && self.is_concrete_type(val_type)
            }
            Type::List(elem_type) => self.is_concrete_type(elem_type),
            Type::Tuple(types) => types.iter().all(|t| self.is_concrete_type(t)),
            Type::Set(elem_type) => self.is_concrete_type(elem_type),
            Type::Optional(inner) => self.is_concrete_type(inner),

            // Unknown, Any, DepylerValue, serde_json::Value are NOT concrete
            Type::Unknown => false,
            Type::Custom(name)
                if name == "Any" || name == "DepylerValue" || name == "serde_json::Value" =>
            {
                false
            }

            // Other custom types are considered concrete (user-defined classes, etc.)
            Type::Custom(_) => true,

            // Union types are concrete if all variants are concrete
            Type::Union(types) => types.iter().all(|t| self.is_concrete_type(t)),

            // Callable, Generator, etc. - treat as non-concrete for now
            _ => false,
        }
    }

    /// Check if an expression is a user-defined class instance
    pub(crate) fn is_class_instance(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a user-defined class
                if let Some(Type::Custom(class_name)) = self.ctx.var_types.get(name) {
                    // Check if this is a user-defined class (not a builtin)
                    self.ctx.class_names.contains(class_name)
                } else {
                    false
                }
            }
            HirExpr::Call { func, .. } => {
                // Direct constructor call like Calculator(10)
                self.ctx.class_names.contains(func)
            }
            _ => false,
        }
    }

    /// DEPYLER-1211: Infer type from an HirExpr
    ///
    /// Used for recursive type propagation: when `.append(arg)` is called,
    /// we infer the list element type from the argument's type.
    ///
    /// Returns the inferred Type or Type::Unknown if type cannot be determined.
    pub(super) fn infer_type_from_hir_expr(&self, expr: &HirExpr) -> Type {
        match expr {
            // Literal types are directly known
            HirExpr::Literal(Literal::Int(_)) => Type::Int,
            HirExpr::Literal(Literal::Float(_)) => Type::Float,
            HirExpr::Literal(Literal::String(_)) => Type::String,
            HirExpr::Literal(Literal::Bool(_)) => Type::Bool,
            HirExpr::Literal(Literal::None) => Type::Optional(Box::new(Type::Unknown)),

            // Variable - look up in context
            HirExpr::Var(name) => self
                .ctx
                .var_types
                .get(name)
                .cloned()
                .unwrap_or(Type::Unknown),

            // List literal - infer element type from first element
            HirExpr::List(elements) => {
                if elements.is_empty() {
                    Type::List(Box::new(Type::Unknown))
                } else {
                    let elem_type = self.infer_type_from_hir_expr(&elements[0]);
                    Type::List(Box::new(elem_type))
                }
            }

            // Dict literal - infer key/value types from first entry
            HirExpr::Dict(entries) => {
                if entries.is_empty() {
                    Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
                } else {
                    let (key, val) = &entries[0];
                    let key_type = self.infer_type_from_hir_expr(key);
                    let val_type = self.infer_type_from_hir_expr(val);
                    Type::Dict(Box::new(key_type), Box::new(val_type))
                }
            }

            // Tuple - infer element types (for now, default to Unknown)
            HirExpr::Tuple(elements) => {
                if elements.is_empty() {
                    Type::Tuple(vec![])
                } else {
                    let types: Vec<Type> = elements
                        .iter()
                        .map(|e| self.infer_type_from_hir_expr(e))
                        .collect();
                    Type::Tuple(types)
                }
            }

            // Set - infer element type
            HirExpr::Set(elements) => {
                if elements.is_empty() {
                    Type::Set(Box::new(Type::Unknown))
                } else {
                    let elem_type = self.infer_type_from_hir_expr(&elements[0]);
                    Type::Set(Box::new(elem_type))
                }
            }

            // Binary operations - infer result type
            HirExpr::Binary { left, op, right: _ } => {
                match op {
                    // Comparison ops always return bool
                    BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn => Type::Bool,
                    // Arithmetic ops - infer from left operand
                    _ => self.infer_type_from_hir_expr(left),
                }
            }

            // Unary operations
            HirExpr::Unary { op, operand } => match op {
                UnaryOp::Not => Type::Bool,
                UnaryOp::Neg | UnaryOp::Pos => self.infer_type_from_hir_expr(operand),
                UnaryOp::BitNot => Type::Int,
            },

            // Ternary/conditional - infer from body
            HirExpr::IfExpr { body, .. } => self.infer_type_from_hir_expr(body),

            // Call expression - harder to infer, default to Unknown
            HirExpr::Call { func, .. } => {
                // Special case: built-in constructors
                match func.as_str() {
                    "int" => Type::Int,
                    "float" => Type::Float,
                    "str" => Type::String,
                    "bool" => Type::Bool,
                    "list" => Type::List(Box::new(Type::Unknown)),
                    "dict" => Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
                    "set" => Type::Set(Box::new(Type::Unknown)),
                    "len" => Type::Int,
                    _ => Type::Unknown,
                }
            }

            // Attribute access - check class field types
            HirExpr::Attribute { attr, .. } => self
                .ctx
                .class_field_types
                .get(attr)
                .cloned()
                .unwrap_or(Type::Unknown),

            // Index (subscript) - try to infer element type
            HirExpr::Index { base, .. } => {
                let base_type = self.infer_type_from_hir_expr(base);
                match base_type {
                    Type::List(elem) => *elem,
                    Type::Dict(_, val) => *val,
                    Type::Tuple(types) if !types.is_empty() => types[0].clone(),
                    Type::String => Type::String, // s[i] -> str
                    _ => Type::Unknown,
                }
            }

            // Default to Unknown for complex expressions
            _ => Type::Unknown,
        }
    }

    /// DEPYLER-1117: Infer lambda parameter type from body expression
    ///
    /// Analyzes the body to determine what type a parameter should be:
    /// - If parameter.iter() is called -> Vec<i64> (iterable)
    /// - If parameter is used directly in PyOps (py_add, py_mul, etc.) -> i64
    /// - Default -> i64 (most common in Python numeric code)
    pub(super) fn infer_lambda_param_type(&self, param: &str, body: &HirExpr) -> Option<syn::Type> {
        // DEPYLER-1117: Check iterator methods FIRST - if param.iter() is called,
        // it's a collection, not a scalar
        // DEPYLER-1156: Use &Vec<i32> (reference) instead of Vec<i32> (owned)
        // Python passes lists by reference, so lambda calls use &numbers not numbers
        // DEPYLER-1314: Changed from i64 to i32 for consistency with list element inference
        if self.body_uses_iter_on_param(param, body) {
            return Some(parse_quote! { &Vec<i32> });
        }

        // DEPYLER-1130: Check if parameter is used as a boolean condition
        // Pattern: `lambda is_add: (expr) if is_add else (expr)` → is_add: bool
        if self.param_used_as_condition(param, body) {
            return Some(parse_quote! { bool });
        }

        // Check if parameter is used directly in PyOps (not in nested lambdas)
        // DEPYLER-1314: Changed from i64 to i32 for consistency
        if self.param_directly_in_pyops(param, body) {
            return Some(parse_quote! { i32 });
        }

        // Default to i32 for standalone lambdas to resolve E0282
        // DEPYLER-1314: Changed from i64 to i32 to match list element type inference
        // This is safe because PyOps traits are implemented for i32
        Some(parse_quote! { i32 })
    }

    /// DEPYLER-1130: Check if parameter is used as a boolean condition
    /// Detects patterns like `if param:` or `if param else` in lambda body
    pub(super) fn param_used_as_condition(&self, param: &str, body: &HirExpr) -> bool {
        match body {
            // Direct use as condition: `(expr) if param else (expr)`
            HirExpr::IfExpr { test, body, orelse } => {
                // Check if param IS the test condition (not just contained in it)
                if self.is_direct_var(param, test) {
                    return true;
                }
                // Also recurse to check nested conditionals
                self.param_used_as_condition(param, body)
                    || self.param_used_as_condition(param, orelse)
            }
            // Check in nested expressions
            HirExpr::Binary { left, right, .. } => {
                self.param_used_as_condition(param, left)
                    || self.param_used_as_condition(param, right)
            }
            HirExpr::Call { args, .. } => args
                .iter()
                .any(|arg| self.param_used_as_condition(param, arg)),
            HirExpr::MethodCall { object, args, .. } => {
                self.param_used_as_condition(param, object)
                    || args
                        .iter()
                        .any(|arg| self.param_used_as_condition(param, arg))
            }
            _ => false,
        }
    }

    /// Check if parameter is used DIRECTLY in a binary operation (not nested in closures)
    pub(super) fn param_directly_in_pyops(&self, param: &str, body: &HirExpr) -> bool {
        match body {
            // Direct binary operation with the parameter
            HirExpr::Binary { left, right, .. } => {
                self.is_direct_var(param, left) || self.is_direct_var(param, right)
            }
            // Direct method call on the parameter (but not iter/map/filter)
            HirExpr::MethodCall { object, method, .. } => {
                // Skip iterator methods - those indicate collection type
                if matches!(
                    method.as_str(),
                    "iter" | "into_iter" | "map" | "filter" | "cloned"
                ) {
                    return false;
                }
                self.is_direct_var(param, object)
            }
            // Ternary expression - check all branches
            HirExpr::IfExpr {
                test,
                body: if_body,
                orelse,
            } => {
                self.param_directly_in_pyops(param, test)
                    || self.param_directly_in_pyops(param, if_body)
                    || self.param_directly_in_pyops(param, orelse)
            }
            _ => false,
        }
    }

    /// Check if expr is directly the variable (not nested)
    pub(super) fn is_direct_var(&self, var_name: &str, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Var(name) if name == var_name)
    }

    /// Check if body uses the parameter as an iterable (list comprehension, for loop, iter() call)
    pub(super) fn body_uses_iter_on_param(&self, param: &str, body: &HirExpr) -> bool {
        match body {
            // DEPYLER-1117: List comprehension `[x for x in param]` - param is iterable
            HirExpr::ListComp { generators, .. } => generators
                .iter()
                .any(|gen| self.is_direct_var(param, &gen.iter)),
            // Set comprehension `{x for x in param}` - param is iterable
            HirExpr::SetComp { generators, .. } => generators
                .iter()
                .any(|gen| self.is_direct_var(param, &gen.iter)),
            // Dict comprehension `{k: v for k, v in param}` - param is iterable
            HirExpr::DictComp { generators, .. } => generators
                .iter()
                .any(|gen| self.is_direct_var(param, &gen.iter)),
            // Generator expression `(x for x in param)` - param is iterable
            HirExpr::GeneratorExp { generators, .. } => generators
                .iter()
                .any(|gen| self.is_direct_var(param, &gen.iter)),
            // Method call - check for iter() or chained iterator methods
            HirExpr::MethodCall { object, method, .. } => {
                // Check if this is an iterator method call directly on the parameter
                if matches!(method.as_str(), "iter" | "into_iter") {
                    return self.is_direct_var(param, object);
                }
                // Also check if this is a chained call like param.iter().map()...
                if let HirExpr::MethodCall {
                    object: inner_obj,
                    method: inner_method,
                    ..
                } = object.as_ref()
                {
                    if matches!(inner_method.as_str(), "iter" | "into_iter") {
                        return self.is_direct_var(param, inner_obj);
                    }
                }
                false
            }
            // Check in nested expressions
            HirExpr::Call { args, .. } => args
                .iter()
                .any(|arg| self.body_uses_iter_on_param(param, arg)),
            _ => false,
        }
    }

    /// Check if an expression is a len() call
    pub(crate) fn is_len_call(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
    }

    /// DEPYLER-0544: Check if expression creates a File (open() or File::create())
    pub(crate) fn is_file_creating_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call { func: Symbol, .. } - func is a simple function name like "open"
            HirExpr::Call { func, .. } => {
                // Check for open() builtin
                func == "open"
            }
            // MethodCall { object, method, .. } - e.g., File.create()
            HirExpr::MethodCall { object, method, .. } => {
                if method == "create" || method == "open" {
                    if let HirExpr::Var(name) = object.as_ref() {
                        return name == "File";
                    }
                    // std.fs.File.create()
                    if let HirExpr::Attribute { attr, .. } = object.as_ref() {
                        return attr == "File";
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0544: Check if expression is sys.stdout
    pub(crate) fn is_stdout_expr(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Attribute { value, attr } = expr {
            if attr == "stdout" {
                if let HirExpr::Var(name) = value.as_ref() {
                    return name == "sys";
                }
            }
        }
        false
    }

    /// DEPYLER-1071: Check if a variable is an Option type (by type tracking or name heuristic)
    pub(super) fn is_option_variable(&self, var_name: &str) -> bool {
        // First check if we have type info
        if let Some(var_type) = self.ctx.var_types.get(var_name) {
            if matches!(var_type, Type::Optional(_)) {
                return true;
            }
        }
        // Fall back to name heuristic for regex match results
        is_option_var_name(var_name)
    }

    /// DEPYLER-1071: Check if the body expression uses the given variable in a method call
    /// This detects patterns like `m.group(0)` where m is the Option variable
    pub(super) fn body_uses_option_var_method(&self, body: &HirExpr, var_name: &str) -> bool {
        match body {
            // Direct method call on the variable: m.group(0)
            HirExpr::MethodCall { object, .. } => {
                if let HirExpr::Var(obj_name) = object.as_ref() {
                    return obj_name == var_name;
                }
                // Check nested method calls
                self.body_uses_option_var_method(object, var_name)
            }
            // Attribute access on the variable
            HirExpr::Attribute { value, .. } => {
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    return obj_name == var_name;
                }
                self.body_uses_option_var_method(value, var_name)
            }
            // Variable used directly in body
            HirExpr::Var(name) => name == var_name,
            _ => false,
        }
    }

}
