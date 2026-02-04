//! TranspilerPatcher - DEPYLER-1308
//!
//! Self-modifying compiler infrastructure that applies patches to depyler-core
//! source code based on Oracle-identified error patterns.
//!
//! # Architecture
//!
//! ```text
//! Pattern Store → TranspilerPatcher → Modified depyler-core
//!       ↓                  ↓
//!  Error Pattern      syn parser
//!       ↓                  ↓
//!   .apr file         AST modification
//!       ↓                  ↓
//!  Patch Record      quote! codegen
//! ```
//!
//! # APR Format (Automated Patch Record)
//!
//! ```toml
//! [[patch]]
//! id = "E0308-list-literal"
//! target_file = "expr_gen.rs"
//! target_function = "convert_list_expr"
//! error_pattern = "E0308"
//! error_keywords = ["list", "Vec", "type mismatch"]
//! patch_type = "inject_before_return"
//! code_template = """
//! let inferred_type = self.infer_element_type(&elements);
//! """
//! confidence = 0.85
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use syn::{parse_file, File, Item, ItemFn, ItemImpl};

/// Automated Patch Record - defines how to patch the transpiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRecord {
    /// Unique identifier for the patch
    pub id: String,
    /// Target source file (relative to crates/depyler-core/src)
    pub target_file: String,
    /// Target function or method name
    pub target_function: String,
    /// Optional: impl block name (for methods)
    pub impl_block: Option<String>,
    /// Error code this patch addresses (e.g., "E0308")
    pub error_pattern: String,
    /// Keywords that must appear in the error message
    pub error_keywords: Vec<String>,
    /// Type of patch to apply
    pub patch_type: PatchType,
    /// Code template to inject
    pub code_template: String,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Whether this patch is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

/// Types of patches that can be applied
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatchType {
    /// Inject code at the start of a function
    InjectAtStart,
    /// Inject code before the return statement
    InjectBeforeReturn,
    /// Replace a match arm
    ReplaceMatchArm { pattern: String },
    /// Add a new match arm
    AddMatchArm { before_pattern: String },
    /// Wrap the function body
    WrapBody,
    /// Add import statement
    AddImport,
    /// Modify type annotation
    ModifyType { from: String, to: String },
}

/// Patch application result
#[derive(Debug, Clone)]
pub struct PatchResult {
    pub patch_id: String,
    pub file_modified: PathBuf,
    pub success: bool,
    pub description: String,
    pub backup_path: Option<PathBuf>,
}

/// APR file containing multiple patches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AprFile {
    pub version: String,
    pub created: String,
    pub patches: Vec<PatchRecord>,
}

impl Default for AprFile {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            created: chrono::Utc::now().to_rfc3339(),
            patches: Vec::new(),
        }
    }
}

/// TranspilerPatcher - modifies depyler-core source based on Oracle patterns
pub struct TranspilerPatcher {
    /// Path to depyler-core crate
    core_path: PathBuf,
    /// Loaded patches indexed by error pattern
    patches: HashMap<String, Vec<PatchRecord>>,
    /// Applied patches history
    applied: Vec<PatchResult>,
    /// Create backups before patching
    create_backups: bool,
}

impl TranspilerPatcher {
    /// Create a new patcher targeting depyler-core
    pub fn new(core_path: impl AsRef<Path>) -> Self {
        Self {
            core_path: core_path.as_ref().to_path_buf(),
            patches: HashMap::new(),
            applied: Vec::new(),
            create_backups: true,
        }
    }

    /// Load patches from an APR file
    pub fn load_apr(&mut self, apr_path: impl AsRef<Path>) -> Result<usize> {
        let content =
            std::fs::read_to_string(apr_path.as_ref()).context("Failed to read APR file")?;

        let apr: AprFile = toml::from_str(&content).context("Failed to parse APR file")?;

        let count = apr.patches.len();
        for patch in apr.patches {
            if patch.enabled {
                self.patches
                    .entry(patch.error_pattern.clone())
                    .or_default()
                    .push(patch);
            }
        }

        Ok(count)
    }

    /// Load patches from embedded defaults
    /// DEPYLER-1311: Updated with contextual keywords from source line extraction
    /// DEPYLER-1312: Corrected function names and impl block names to match codebase
    pub fn load_defaults(&mut self) {
        // DEPYLER-1312: Patches with correct function names matching depyler-core
        let defaults = vec![
            // E0308 patches - Type mismatches
            PatchRecord {
                id: "E0308-list-element-inference".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_list".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0308".to_string(),
                // DEPYLER-1311: Use contextual keywords from source lines
                error_keywords: vec![
                    "vec".to_string(),
                    "list".to_string(),
                    "push".to_string(),
                    "collect".to_string(),
                    "iterator".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1308: Hint - list element type from first element
        let _element_count = elts.len();
"#
                .to_string(),
                confidence: 0.85,
                enabled: true,
            },
            PatchRecord {
                id: "E0308-dict-value-inference".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_dict".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0308".to_string(),
                // DEPYLER-1311: Use contextual keywords
                error_keywords: vec![
                    "dict".to_string(),
                    "hashmap".to_string(),
                    "insert".to_string(),
                    "get".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1308: Hint - dict value type from first entry
        let _entry_count = items.len();
"#
                .to_string(),
                confidence: 0.80,
                enabled: true,
            },
            PatchRecord {
                id: "E0308-function-return-propagation".to_string(),
                target_file: "rust_gen/func_gen.rs".to_string(),
                target_function: "codegen_function_body".to_string(),
                impl_block: None, // Free function, not in impl block
                error_pattern: "E0308".to_string(),
                // DEPYLER-1311: Broader keywords for return type mismatches
                error_keywords: vec![
                    "return".to_string(),
                    "result".to_string(),
                    "option".to_string(),
                    "string".to_string(),
                    "integer".to_string(),
                    "float".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
    // DEPYLER-1308: Hint - propagate return type to body expressions
    let _return_type_hint = func.ret_type.clone();
"#
                .to_string(),
                confidence: 0.75,
                enabled: true,
            },
            PatchRecord {
                id: "E0308-tuple-element-types".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_tuple".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0308".to_string(),
                // DEPYLER-1311: Tuple contextual keywords
                error_keywords: vec![
                    "tuple".to_string(),
                    "integer".to_string(),
                    "string".to_string(),
                    "float".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1308: Preserve tuple element types during conversion
        let _element_count = elts.len();
"#
                .to_string(),
                confidence: 0.75,
                enabled: true,
            },
            PatchRecord {
                id: "E0308-string-conversion".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_string_method".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0308".to_string(),
                // DEPYLER-1311: String type mismatches
                error_keywords: vec!["string".to_string()],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1311: Ensure consistent string type handling
        let _expected_str = "String";
"#
                .to_string(),
                confidence: 0.73,
                enabled: true,
            },
            // E0599 patches - Missing methods
            PatchRecord {
                id: "E0599-datetime-tuple-methods".to_string(),
                target_file: "type_mapper.rs".to_string(),
                target_function: "map_type".to_string(),
                impl_block: Some("TypeMapper".to_string()),
                error_pattern: "E0599".to_string(),
                // DEPYLER-1311: Datetime contextual keywords
                error_keywords: vec![
                    "datetime".to_string(),
                    "time".to_string(),
                    "date".to_string(),
                    "tuple".to_string(),
                ],
                patch_type: PatchType::AddMatchArm {
                    before_pattern: "\"time\" | \"datetime.time\"".to_string(),
                },
                code_template: r#"
    // DEPYLER-1308: Map time to DepylerTime struct with methods
    "time" | "datetime.time" => {
        RustType::Custom("DepylerTime".to_string())
    }
"#
                .to_string(),
                confidence: 0.80,
                enabled: true,
            },
            PatchRecord {
                id: "E0599-option-unwrap-methods".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_method_call".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0599".to_string(),
                // DEPYLER-1311: Option method calls
                error_keywords: vec![
                    "option".to_string(),
                    "get".to_string(),
                    "constructor".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1311: Handle Option method calls
        let _is_option_type = false; // Placeholder for Option detection
"#
                .to_string(),
                confidence: 0.78,
                enabled: true,
            },
            PatchRecord {
                id: "E0599-iterator-collect".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_method_call".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0599".to_string(),
                // DEPYLER-1311: Iterator/collect chains
                error_keywords: vec![
                    "iterator".to_string(),
                    "collect".to_string(),
                    "vec".to_string(),
                    "list".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1311: Handle iterator method chains
        let _is_iterator_chain = false; // Placeholder for iterator detection
"#
                .to_string(),
                confidence: 0.77,
                enabled: true,
            },
            // E0277 patches - Trait bound errors
            PatchRecord {
                id: "E0277-subprocess-vec-string".to_string(),
                target_file: "rust_gen/expr_gen.rs".to_string(),
                target_function: "convert_call".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0277".to_string(),
                // DEPYLER-1311: Subprocess/command contextual keywords
                error_keywords: vec![
                    "subprocess".to_string(),
                    "command".to_string(),
                    "path".to_string(),
                    "file".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1308: Ensure subprocess args are Vec<String> not Vec<Value>
        let _args_hint = "Vec<String>";
"#
                .to_string(),
                confidence: 0.75,
                enabled: true,
            },
            PatchRecord {
                id: "E0277-hashmap-trait-bounds".to_string(),
                target_file: "rust_gen/expr_gen_instance_methods.rs".to_string(),
                target_function: "convert_dict".to_string(),
                impl_block: Some("ExpressionConverter".to_string()),
                error_pattern: "E0277".to_string(),
                // DEPYLER-1311: HashMap trait bound errors
                error_keywords: vec![
                    "hashmap".to_string(),
                    "dict".to_string(),
                    "get".to_string(),
                    "insert".to_string(),
                ],
                patch_type: PatchType::InjectAtStart,
                code_template: r#"
        // DEPYLER-1311: Ensure HashMap keys implement Hash + Eq
        let _key_hashable = true; // Placeholder for hashability check
"#
                .to_string(),
                confidence: 0.72,
                enabled: true,
            },
        ];

        for patch in defaults {
            self.patches
                .entry(patch.error_pattern.clone())
                .or_default()
                .push(patch);
        }
    }

    /// Find patches matching an error
    /// DEPYLER-1310: Enhanced to check context_keywords from source lines
    pub fn find_patches(
        &self,
        error_code: &str,
        error_message: &str,
        context_keywords: &[String],
    ) -> Vec<&PatchRecord> {
        let message_lower = error_message.to_lowercase();
        let context_lower: Vec<String> = context_keywords
            .iter()
            .map(|kw| kw.to_lowercase())
            .collect();

        self.patches
            .get(error_code)
            .map(|patches| {
                patches
                    .iter()
                    .filter(|p| {
                        // Check if any error_keyword matches either:
                        // 1. The error message (original behavior)
                        // 2. The context keywords from source line (DEPYLER-1310)
                        p.error_keywords.iter().any(|kw| {
                            let kw_lower = kw.to_lowercase();
                            message_lower.contains(&kw_lower) || context_lower.contains(&kw_lower)
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Apply a patch to the transpiler source
    pub fn apply_patch(&mut self, patch: &PatchRecord) -> Result<PatchResult> {
        let target_path = self.core_path.join("src").join(&patch.target_file);

        if !target_path.exists() {
            return Ok(PatchResult {
                patch_id: patch.id.clone(),
                file_modified: target_path.clone(),
                success: false,
                description: format!("Target file not found: {}", target_path.display()),
                backup_path: None,
            });
        }

        // Create backup
        let backup_path = if self.create_backups {
            let backup = target_path.with_extension("rs.bak");
            std::fs::copy(&target_path, &backup)?;
            Some(backup)
        } else {
            None
        };

        // Read and parse the source file
        let source = std::fs::read_to_string(&target_path)?;
        let syntax = parse_file(&source)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", target_path.display(), e))?;

        // Apply the patch based on type
        let modified = match &patch.patch_type {
            PatchType::InjectAtStart => self.inject_at_function_start(&syntax, &source, patch)?,
            PatchType::InjectBeforeReturn => self.inject_before_return(&syntax, &source, patch)?,
            PatchType::AddMatchArm { before_pattern } => {
                self.add_match_arm(&source, patch, before_pattern)?
            }
            PatchType::ReplaceMatchArm { pattern } => {
                self.replace_match_arm(&source, patch, pattern)?
            }
            PatchType::WrapBody => self.wrap_function_body(&syntax, &source, patch)?,
            PatchType::AddImport => self.add_import(&source, patch)?,
            PatchType::ModifyType { from, to } => self.modify_type(&source, from, to)?,
        };

        // Write the modified source
        std::fs::write(&target_path, &modified)?;

        let result = PatchResult {
            patch_id: patch.id.clone(),
            file_modified: target_path,
            success: true,
            description: format!("Applied patch {} ({:?})", patch.id, patch.patch_type),
            backup_path,
        };

        self.applied.push(result.clone());
        Ok(result)
    }

    /// Find a function in the parsed AST
    /// Reserved for future AST-based patching (currently using text-based approach)
    #[allow(dead_code)]
    fn find_function<'a>(
        &self,
        syntax: &'a File,
        name: &str,
        impl_block: Option<&str>,
    ) -> Option<&'a ItemFn> {
        for item in &syntax.items {
            match item {
                Item::Fn(func) if func.sig.ident == name && impl_block.is_none() => {
                    return Some(func);
                }
                Item::Impl(impl_item) if impl_block.is_some() => {
                    if self.impl_matches(impl_item, impl_block.expect("checked is_some")) {
                        for item in &impl_item.items {
                            if let syn::ImplItem::Fn(method) = item {
                                if method.sig.ident == name {
                                    // Can't return method directly, would need different approach
                                    // For now, we'll use text-based patching
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Check if an impl block matches the expected name
    /// Reserved for future AST-based patching (currently using text-based approach)
    #[allow(dead_code)]
    fn impl_matches(&self, impl_item: &ItemImpl, expected: &str) -> bool {
        if let syn::Type::Path(type_path) = &*impl_item.self_ty {
            if let Some(segment) = type_path.path.segments.last() {
                return segment.ident == expected;
            }
        }
        false
    }

    /// Inject code at the start of a function (text-based)
    fn inject_at_function_start(
        &self,
        _syntax: &File,
        source: &str,
        patch: &PatchRecord,
    ) -> Result<String> {
        // DEPYLER-1312: Use text-based patching with improved regex for generics
        let fn_pattern = if let Some(impl_name) = &patch.impl_block {
            // Handle impl blocks with generic parameters like impl<'a, 'b> Type<'a, 'b>
            // Match: impl<...>? TypeName<...>? { ... fn func_name(...)
            format!(
                r"(impl\s*(?:<[^>]*>)?\s+(?:\w+\s+for\s+)?{name}(?:<[^>]*>)?\s*(?:where[^{{]*)?\s*\{{[\s\S]*?(?:pub(?:\s*\([^)]*\))?\s+)?fn\s+{func}\s*(?:<[^>]*>)?\s*\([^)]*\)[^{{]*\{{)",
                name = impl_name,
                func = patch.target_function
            )
        } else {
            format!(
                r"((?:pub(?:\s*\([^)]*\))?\s+)?fn\s+{}\s*(?:<[^>]*>)?\s*\([^)]*\)[^{{]*\{{)",
                patch.target_function
            )
        };

        let re = regex::Regex::new(&fn_pattern)?;

        if let Some(caps) = re.captures(source) {
            let matched = caps.get(0).expect("capture group 0 exists");
            let insert_point = matched.end();

            let mut result = source.to_string();
            result.insert_str(insert_point, &format!("\n{}\n", patch.code_template));
            return Ok(result);
        }

        Err(anyhow::anyhow!(
            "Could not find function {} in {}",
            patch.target_function,
            patch.target_file
        ))
    }

    /// Inject code before return statements
    fn inject_before_return(
        &self,
        _syntax: &File,
        source: &str,
        patch: &PatchRecord,
    ) -> Result<String> {
        // Find the function and inject before each return
        let fn_pattern = format!(r"fn\s+{}\s*\([^)]*\)[^{{]*\{{", patch.target_function);
        let re = regex::Regex::new(&fn_pattern)?;

        if let Some(m) = re.find(source) {
            // Find the function body boundaries
            let start = m.end();
            let mut brace_count = 1;
            let mut end = start;

            for (i, c) in source[start..].char_indices() {
                match c {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            end = start + i;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            // Find return statements in the function body
            let body = &source[start..end];
            let return_re = regex::Regex::new(r"\breturn\b")?;

            let mut result = source.to_string();
            let mut offset = 0i64;

            for m in return_re.find_iter(body) {
                let insert_pos = (start + m.start()) as i64 + offset;
                let injection = format!("{}\n        ", patch.code_template.trim());
                result.insert_str(insert_pos as usize, &injection);
                offset += injection.len() as i64;
            }

            return Ok(result);
        }

        Err(anyhow::anyhow!(
            "Could not find function {} for return injection",
            patch.target_function
        ))
    }

    /// Add a new match arm
    fn add_match_arm(
        &self,
        source: &str,
        patch: &PatchRecord,
        before_pattern: &str,
    ) -> Result<String> {
        // Find the pattern and insert the new arm before it
        let pattern =
            regex::Regex::new(&format!(r"(\s*)({}\s*=>)", regex::escape(before_pattern)))?;

        if let Some(caps) = pattern.captures(source) {
            let indent = caps.get(1).map_or("            ", |m| m.as_str());
            let insert_point = caps.get(0).expect("capture group 0 exists").start();

            let new_arm = format!("{}{}\n{}", indent, patch.code_template.trim(), indent);

            let mut result = source.to_string();
            result.insert_str(insert_point, &new_arm);
            return Ok(result);
        }

        Err(anyhow::anyhow!(
            "Could not find pattern '{}' to insert match arm",
            before_pattern
        ))
    }

    /// Replace a match arm
    fn replace_match_arm(
        &self,
        source: &str,
        patch: &PatchRecord,
        pattern: &str,
    ) -> Result<String> {
        // Find the pattern => body and replace
        let arm_pattern = regex::Regex::new(&format!(
            r"({})\s*=>\s*\{{[^}}]*\}}|({})s*=>\s*[^,]+,",
            regex::escape(pattern),
            regex::escape(pattern)
        ))?;

        let replacement = format!("{} => {}", pattern, patch.code_template.trim());
        let result = arm_pattern.replace(source, replacement.as_str());

        Ok(result.to_string())
    }

    /// Wrap function body
    fn wrap_function_body(
        &self,
        _syntax: &File,
        source: &str,
        patch: &PatchRecord,
    ) -> Result<String> {
        // This would wrap the entire function body in the template
        // Template should have {BODY} placeholder
        let fn_pattern = format!(r"fn\s+{}\s*\([^)]*\)[^{{]*\{{", patch.target_function);
        let re = regex::Regex::new(&fn_pattern)?;

        if let Some(m) = re.find(source) {
            let start = m.end();
            let mut brace_count = 1;
            let mut end = start;

            for (i, c) in source[start..].char_indices() {
                match c {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            end = start + i;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let body = &source[start..end];
            let wrapped = patch.code_template.replace("{BODY}", body);

            let mut result = String::new();
            result.push_str(&source[..start]);
            result.push_str(&wrapped);
            result.push_str(&source[end..]);

            return Ok(result);
        }

        Err(anyhow::anyhow!(
            "Could not find function {} to wrap body",
            patch.target_function
        ))
    }

    /// Add an import statement
    fn add_import(&self, source: &str, patch: &PatchRecord) -> Result<String> {
        // Find the last use statement or module doc comment
        let use_pattern = regex::Regex::new(r"(use [^;]+;)\n")?;

        if let Some(last_use) = use_pattern.find_iter(source).last() {
            let insert_point = last_use.end();
            let mut result = source.to_string();
            result.insert_str(insert_point, &format!("{}\n", patch.code_template.trim()));
            return Ok(result);
        }

        // If no use statements, add after module doc
        let doc_pattern = regex::Regex::new(r"(//![^\n]*\n)+")?;
        if let Some(docs) = doc_pattern.find(source) {
            let insert_point = docs.end();
            let mut result = source.to_string();
            result.insert_str(insert_point, &format!("\n{}\n", patch.code_template.trim()));
            return Ok(result);
        }

        // Add at the very beginning
        Ok(format!("{}\n\n{}", patch.code_template.trim(), source))
    }

    /// Modify type annotation
    fn modify_type(&self, source: &str, from: &str, to: &str) -> Result<String> {
        let result = source.replace(from, to);
        Ok(result)
    }

    /// Get applied patches
    pub fn applied_patches(&self) -> &[PatchResult] {
        &self.applied
    }

    /// Rollback a patch using backup
    pub fn rollback(&self, result: &PatchResult) -> Result<()> {
        if let Some(backup) = &result.backup_path {
            std::fs::copy(backup, &result.file_modified)?;
            std::fs::remove_file(backup)?;
        }
        Ok(())
    }

    /// Get total loaded patches
    pub fn patch_count(&self) -> usize {
        self.patches.values().map(|v| v.len()).sum()
    }
}

/// Save an APR file with patches
pub fn save_apr(patches: &[PatchRecord], path: impl AsRef<Path>) -> Result<()> {
    let apr = AprFile {
        version: "1.0".to_string(),
        created: chrono::Utc::now().to_rfc3339(),
        patches: patches.to_vec(),
    };

    let content = toml::to_string_pretty(&apr)?;
    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_record_creation() {
        let patch = PatchRecord {
            id: "test-patch".to_string(),
            target_file: "test.rs".to_string(),
            target_function: "test_fn".to_string(),
            impl_block: None,
            error_pattern: "E0308".to_string(),
            error_keywords: vec!["test".to_string()],
            patch_type: PatchType::InjectAtStart,
            code_template: "// test".to_string(),
            confidence: 0.9,
            enabled: true,
        };

        assert_eq!(patch.id, "test-patch");
        assert!(patch.enabled);
    }

    #[test]
    fn test_transpiler_patcher_load_defaults() {
        let mut patcher = TranspilerPatcher::new("/tmp/fake");
        patcher.load_defaults();

        assert!(patcher.patch_count() > 0);
        assert!(patcher.patches.contains_key("E0308"));
    }

    #[test]
    fn test_find_patches() {
        let mut patcher = TranspilerPatcher::new("/tmp/fake");
        patcher.load_defaults();

        // Test matching via error message
        let patches = patcher.find_patches("E0308", "list element type mismatch", &[]);
        assert!(!patches.is_empty());
    }

    #[test]
    fn test_find_patches_via_context_keywords() {
        let mut patcher = TranspilerPatcher::new("/tmp/fake");
        patcher.load_defaults();

        // Test matching via context keywords (DEPYLER-1310)
        // The error message says "mismatched types" (no keywords)
        // But context_keywords from source line has "vec"
        let context_keywords = vec!["vec".to_string(), "list".to_string()];
        let patches = patcher.find_patches("E0308", "mismatched types", &context_keywords);
        assert!(
            !patches.is_empty(),
            "Should find patches via context_keywords"
        );
    }

    #[test]
    fn test_find_patches_no_match() {
        let mut patcher = TranspilerPatcher::new("/tmp/fake");
        patcher.load_defaults();

        // Test no match when neither message nor context has keywords
        let patches = patcher.find_patches("E0308", "mismatched types", &[]);
        assert!(
            patches.is_empty(),
            "Should not find patches without relevant keywords"
        );
    }

    #[test]
    fn test_apr_serialization() {
        let patch = PatchRecord {
            id: "test".to_string(),
            target_file: "test.rs".to_string(),
            target_function: "test_fn".to_string(),
            impl_block: None,
            error_pattern: "E0308".to_string(),
            error_keywords: vec!["test".to_string()],
            patch_type: PatchType::InjectAtStart,
            code_template: "// code".to_string(),
            confidence: 0.8,
            enabled: true,
        };

        let apr = AprFile {
            version: "1.0".to_string(),
            created: "2025-01-01".to_string(),
            patches: vec![patch],
        };

        let toml_str = toml::to_string_pretty(&apr).unwrap();
        assert!(toml_str.contains("test"));
        assert!(toml_str.contains("E0308"));
    }

    #[test]
    fn test_patch_type_variants() {
        let inject = PatchType::InjectAtStart;
        let before_ret = PatchType::InjectBeforeReturn;
        let add_arm = PatchType::AddMatchArm {
            before_pattern: "test".to_string(),
        };

        assert_ne!(inject, before_ret);
        assert_ne!(before_ret, add_arm);
    }
}
