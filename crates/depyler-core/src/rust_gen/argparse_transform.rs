//! ArgumentParser → Clap transformation (DEPYLER-0363)
//!
//! This module handles the structural transformation of Python argparse
//! patterns to Rust clap derive macros.
//!
//! # Transformation Strategy
//!
//! Python argparse uses imperative API:
//! ```python
//! parser = argparse.ArgumentParser(description="...")
//! parser.add_argument("files", nargs="+", type=Path)
//! parser.add_argument("-v", "--verbose", action="store_true")
//! args = parser.parse_args()
//! ```
//!
//! Rust clap uses declarative struct:
//! ```rust
//! #[derive(Parser)]
//! #[command(about = "...")]
//! struct Args {
//!     files: Vec<PathBuf>,
//!     #[arg(short, long)]
//!     verbose: bool,
//! }
//! let args = Args::parse();
//! ```
//!
//! # Detection Algorithm
//!
//! 1. Detect `parser = ArgumentParser(...)` assignment
//! 2. Accumulate all `parser.add_argument(...)` method calls
//! 3. Detect `args = parser.parse_args()` assignment
//! 4. Generate struct definition with clap derives
//! 5. Replace parse_args() call with `Args::parse()`

use crate::hir::{HirExpr, Type};
use std::collections::HashMap;

/// Convert HIR Type to Rust type string for argparse arguments
///
/// # DEPYLER-0364: Type Mapping
/// Maps Python types to idiomatic Rust types for CLI arguments:
/// - int → i32
/// - str → String
/// - Path → PathBuf
/// - bool → bool
/// - float → f64
///
/// # Complexity
/// 3 (pattern match on Type enum)
fn type_to_rust_string(ty: &Type) -> String {
    match ty {
        Type::Int => "i32".to_string(),
        Type::Float => "f64".to_string(),
        Type::String => "String".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Custom(name) if name == "PathBuf" => "PathBuf".to_string(),
        Type::Custom(name) => name.clone(),
        Type::List(inner) => format!("Vec<{}>", type_to_rust_string(inner)),
        Type::Optional(inner) => format!("Option<{}>", type_to_rust_string(inner)),
        _ => "String".to_string(), // Default fallback
    }
}

/// Tracks an ArgumentParser instance being built
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone, PartialEq)]
pub struct ArgParserInfo {
    /// Variable name assigned to ArgumentParser (e.g., "parser")
    pub parser_var: String,

    /// Description from ArgumentParser(description="...")
    pub description: Option<String>,

    /// Epilog from ArgumentParser(epilog="...")
    pub epilog: Option<String>,

    /// All arguments added via add_argument()
    pub arguments: Vec<ArgParserArgument>,

    /// Variable name for parse_args() result (e.g., "args")
    pub args_var: Option<String>,
}

impl ArgParserInfo {
    /// Create new ArgParser tracker
    ///
    /// # Complexity
    /// 1 (struct initialization)
    pub fn new(parser_var: String) -> Self {
        Self {
            parser_var,
            description: None,
            epilog: None,
            arguments: Vec::new(),
            args_var: None,
        }
    }

    /// Add an argument from add_argument() call
    ///
    /// # Complexity
    /// 1 (vec push)
    pub fn add_argument(&mut self, arg: ArgParserArgument) {
        self.arguments.push(arg);
    }

    /// Set the args variable name from parse_args() assignment
    ///
    /// # Complexity
    /// 1 (field assignment)
    pub fn set_args_var(&mut self, var: String) {
        self.args_var = Some(var);
    }
}

/// Represents a single argument definition from add_argument()
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone, PartialEq)]
pub struct ArgParserArgument {
    /// Positional name or short flag (e.g., "files", "-v")
    pub name: String,

    /// Long flag name (e.g., "--verbose")
    pub long: Option<String>,

    /// Number of arguments: "+", "*", "?", or number
    pub nargs: Option<String>,

    /// Type annotation (e.g., Path, int, str)
    pub arg_type: Option<Type>,

    /// Action: "store_true", "store_false", "store", "append"
    pub action: Option<String>,

    /// Default value
    pub default: Option<HirExpr>,

    /// Help text
    pub help: Option<String>,

    /// Whether this is a required positional argument
    pub is_positional: bool,

    /// DEPYLER-0367: Whether this flag is required (required=True)
    pub required: Option<bool>,
}

impl ArgParserArgument {
    /// Create new argument definition
    ///
    /// # Complexity
    /// 2 (string check + struct initialization)
    pub fn new(name: String) -> Self {
        let is_positional = !name.starts_with('-');
        Self {
            name,
            long: None,
            nargs: None,
            arg_type: None,
            action: None,
            default: None,
            help: None,
            is_positional,
            required: None,
        }
    }

    /// Get the Rust field name (convert -v/--verbose → verbose)
    ///
    /// # Complexity
    /// 3 (string operations)
    pub fn rust_field_name(&self) -> String {
        if self.is_positional {
            // Positional arguments keep their name
            self.name.clone()
        } else if let Some(ref long) = self.long {
            // Use long flag without -- (convert hyphens to underscores)
            long.trim_start_matches("--").replace('-', "_")
        } else {
            // Use flag name without leading hyphens (convert hyphens to underscores)
            // DEPYLER-0366: Handle flags like --no-color → no_color
            self.name.trim_start_matches('-').replace('-', "_")
        }
    }

    /// Get the Rust type for this argument
    ///
    /// # Complexity
    /// 7 (multiple match + string checks)
    pub fn rust_type(&self) -> String {
        // action="store_true" → bool
        if self.action.as_deref() == Some("store_true")
            || self.action.as_deref() == Some("store_false") {
            return "bool".to_string();
        }

        // action="count" → u8 (counts occurrences: -v -v -v → 3)
        if self.action.as_deref() == Some("count") {
            return "u8".to_string();
        }

        // nargs="+" or nargs="*" → Vec<T>
        if self.nargs.as_deref() == Some("+") || self.nargs.as_deref() == Some("*") {
            let inner_type = self.arg_type.as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Vec<{}>", inner_type);
        }

        // nargs="?" → Option<T>
        if self.nargs.as_deref() == Some("?") {
            let inner_type = self.arg_type.as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Option<{}>", inner_type);
        }

        // Use explicit type or default to String
        self.arg_type.as_ref()
            .map(type_to_rust_string)
            .unwrap_or_else(|| "String".to_string())
    }
}

/// Container for ArgumentParser tracking in CodeGenContext
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone, Default)]
pub struct ArgParserTracker {
    /// Currently active ArgumentParser instances (keyed by variable name)
    pub parsers: HashMap<String, ArgParserInfo>,

    /// Whether we've generated the Args struct for current function
    pub struct_generated: bool,
}

impl ArgParserTracker {
    /// Create new tracker
    ///
    /// # Complexity
    /// 1 (struct initialization)
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new ArgumentParser assignment
    ///
    /// # Complexity
    /// 2 (struct creation + hashmap insert)
    pub fn register_parser(&mut self, var_name: String, info: ArgParserInfo) {
        self.parsers.insert(var_name, info);
    }

    /// Get mutable reference to parser info by variable name
    ///
    /// # Complexity
    /// 1 (hashmap lookup)
    pub fn get_parser_mut(&mut self, var_name: &str) -> Option<&mut ArgParserInfo> {
        self.parsers.get_mut(var_name)
    }

    /// Get reference to parser info by variable name
    ///
    /// # Complexity
    /// 1 (hashmap lookup)
    pub fn get_parser(&self, var_name: &str) -> Option<&ArgParserInfo> {
        self.parsers.get(var_name)
    }

    /// Clear all parser tracking (e.g., when entering new function)
    ///
    /// # Complexity
    /// 1 (hashmap clear)
    pub fn clear(&mut self) {
        self.parsers.clear();
        self.struct_generated = false;
    }

    /// Check if any ArgumentParser was detected
    ///
    /// # Complexity
    /// 1 (hashmap empty check)
    pub fn has_parsers(&self) -> bool {
        !self.parsers.is_empty()
    }

    /// Get the first parser (assumes single parser per function for now)
    ///
    /// # Complexity
    /// 2 (iterator + first)
    pub fn get_first_parser(&self) -> Option<&ArgParserInfo> {
        self.parsers.values().next()
    }
}

/// Generate clap Args struct definition from ArgumentParser info
///
/// # Complexity
/// 8 (multiple loops and quote operations)
pub fn generate_args_struct(parser_info: &ArgParserInfo) -> proc_macro2::TokenStream {
    use quote::quote;
    use syn::parse_quote;

    // Generate struct fields from arguments
    let fields: Vec<proc_macro2::TokenStream> = parser_info
        .arguments
        .iter()
        .map(|arg| {
            let field_name = syn::Ident::new(&arg.rust_field_name(), proc_macro2::Span::call_site());

            // DEPYLER-0367: Determine if field should be Option<T>
            let base_type_str = arg.rust_type();

            // Don't wrap in Option if:
            // - Already Option (nargs="?")
            // - Has a default value (will use default_value attribute)
            // - Is required=True
            // - Is positional
            // - Has action with implicit default (store_true/false/count → bool/u8)
            // - Has nargs="+" (required, 1 or more)
            let has_implicit_default = matches!(
                arg.action.as_deref(),
                Some("store_true") | Some("store_false") | Some("count")
            );
            let is_required_nargs = arg.nargs.as_deref() == Some("+");

            let field_type: syn::Type = if !arg.is_positional
                && arg.required != Some(true)
                && arg.default.is_none()
                && !base_type_str.starts_with("Option<")
                && !has_implicit_default
                && !is_required_nargs
            {
                // Wrap in Option for optional flags
                syn::parse_str(&format!("Option<{}>", base_type_str))
                    .unwrap_or_else(|_| parse_quote! { Option<String> })
            } else {
                syn::parse_str(&base_type_str)
                    .unwrap_or_else(|_| parse_quote! { String })
            };

            // Generate clap attributes
            let mut attrs = vec![];

            if !arg.is_positional {
                // DEPYLER-0365 Phase 5: Proper flag detection
                // Three cases:
                // 1. Both short and long: "-o" + "--output" → #[arg(short = 'o', long)]
                // 2. Long only: "--debug" → #[arg(long)]
                // 3. Short only: "-v" → #[arg(short = 'v')]

                if arg.long.is_some() {
                    // Case 1: Both short and long flags
                    let short_str = arg.name.trim_start_matches('-');
                    if let Some(short) = short_str.chars().next() {
                        attrs.push(quote! {
                            #[arg(short = #short, long)]
                        });
                    }
                } else if arg.name.starts_with("--") {
                    // Case 2: Long flag only (--debug)
                    attrs.push(quote! {
                        #[arg(long)]
                    });
                } else {
                    // Case 3: Short flag only (-v)
                    let short_str = arg.name.trim_start_matches('-');
                    if let Some(short) = short_str.chars().next() {
                        attrs.push(quote! {
                            #[arg(short = #short)]
                        });
                    }
                }
            }

            // DEPYLER-0367: Add default value if present
            if let Some(ref default_val) = arg.default {
                // Convert HIR literal to string for default_value attribute
                if let crate::hir::HirExpr::Literal(lit) = default_val {
                    let default_str_opt = match lit {
                        crate::hir::Literal::Int(n) => Some(n.to_string()),
                        crate::hir::Literal::Float(f) => Some(f.to_string()),
                        crate::hir::Literal::String(s) => Some(s.clone()),
                        crate::hir::Literal::Bool(b) => Some(b.to_string()),
                        _ => None,  // Skip complex defaults
                    };
                    if let Some(default_str) = default_str_opt {
                        attrs.push(quote! {
                            #[arg(default_value = #default_str)]
                        });
                    }
                }
            }

            // Add help text if present
            if let Some(ref help_text) = arg.help {
                attrs.push(quote! {
                    #[doc = #help_text]
                });
            }

            quote! {
                #(#attrs)*
                #field_name: #field_type
            }
        })
        .collect();

    // Generate command-level attributes
    let mut command_attrs = vec![];
    if let Some(ref desc) = parser_info.description {
        command_attrs.push(quote! {
            #[command(about = #desc)]
        });
    }
    if let Some(ref epilog) = parser_info.epilog {
        command_attrs.push(quote! {
            #[command(after_help = #epilog)]
        });
    }

    // Generate the struct
    quote! {
        #[derive(clap::Parser)]
        #(#command_attrs)*
        struct Args {
            #(#fields),*
        }
    }
}
