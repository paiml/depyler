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
//! ```rust,ignore
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

use crate::emit_decision;
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

    /// DEPYLER-0371: Custom destination variable name (dest="var_name")
    pub dest: Option<String>,

    /// DEPYLER-0372: Metavar for help display (metavar="FILE")
    pub metavar: Option<String>,

    /// DEPYLER-0373: Restricted value choices (choices=["a", "b", "c"])
    pub choices: Option<Vec<String>>,

    /// DEPYLER-0374: Constant value for action="store_const" or nargs="?" with const
    pub const_value: Option<HirExpr>,
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
            dest: None,
            metavar: None,
            choices: None,
            const_value: None,
        }
    }

    /// Get the Rust field name (convert -v/--verbose → verbose)
    ///
    /// # Complexity
    /// 3 (string operations)
    /// # DEPYLER-0371: Use dest parameter if present
    pub fn rust_field_name(&self) -> String {
        // DEPYLER-0371: If dest is specified, use that as the field name
        if let Some(ref dest) = self.dest {
            return dest.replace('-', "_");
        }

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
        // action="store_true"/"store_false"/"store_const" → bool
        // DEPYLER-0375: action="store_const" also maps to bool
        if self.action.as_deref() == Some("store_true")
            || self.action.as_deref() == Some("store_false")
            || self.action.as_deref() == Some("store_const")
        {
            return "bool".to_string();
        }

        // action="count" → u8 (counts occurrences: -v -v -v → 3)
        if self.action.as_deref() == Some("count") {
            return "u8".to_string();
        }

        // DEPYLER-0368: action="append" → Vec<T> (collects multiple flag uses)
        if self.action.as_deref() == Some("append") {
            let inner_type = self
                .arg_type
                .as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Vec<{}>", inner_type);
        }

        // nargs="+" or nargs="*" → Vec<T>
        if self.nargs.as_deref() == Some("+") || self.nargs.as_deref() == Some("*") {
            let inner_type = self
                .arg_type
                .as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Vec<{}>", inner_type);
        }

        // DEPYLER-0370: nargs=N (specific number) → Vec<T>
        if let Some(nargs_str) = self.nargs.as_deref() {
            if nargs_str.parse::<usize>().is_ok() {
                let inner_type = self
                    .arg_type
                    .as_ref()
                    .map(type_to_rust_string)
                    .unwrap_or_else(|| "String".to_string());
                return format!("Vec<{}>", inner_type);
            }
        }

        // nargs="?" → Option<T>
        // DEPYLER-0374: Handle const parameter with nargs="?" separately in generate_args_struct
        if self.nargs.as_deref() == Some("?") {
            let inner_type = self
                .arg_type
                .as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Option<{}>", inner_type);
        }

        // DEPYLER-0527: Optional flags (--arg without required=True) → Option<T>
        // In argparse, long arguments without required=True default to None
        if !self.is_positional && self.required != Some(true) && self.nargs.is_none() {
            let inner_type = self
                .arg_type
                .as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Option<{}>", inner_type);
        }

        // Use explicit type or default to String
        self.arg_type
            .as_ref()
            .map(type_to_rust_string)
            .unwrap_or_else(|| "String".to_string())
    }
}

/// DEPYLER-0399: Information about a subparser collection (from add_subparsers())
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone, PartialEq)]
pub struct SubparserInfo {
    /// Parent parser variable name
    pub parser_var: String,

    /// Destination field name (from dest= parameter)
    pub dest_field: String,

    /// Whether subcommand is required
    pub required: bool,

    /// Help text for subparsers group
    pub help: Option<String>,
}

/// DEPYLER-0399: Information about a single subcommand
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone, PartialEq)]
pub struct SubcommandInfo {
    /// Subcommand name (e.g., "clone")
    pub name: String,

    /// Help text for this subcommand
    pub help: Option<String>,

    /// Arguments specific to this subcommand
    pub arguments: Vec<ArgParserArgument>,

    /// Parent subparsers variable
    pub subparsers_var: String,
}

/// Container for ArgumentParser tracking in CodeGenContext
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone, Default)]
pub struct ArgParserTracker {
    /// Currently active ArgumentParser instances (keyed by variable name)
    pub parsers: HashMap<String, ArgParserInfo>,

    /// DEPYLER-0396: Map argument group variables to their parent parser
    /// e.g., "input_group" → "parser"
    /// This allows tracking add_argument() calls on groups
    pub group_to_parser: HashMap<String, String>,

    /// DEPYLER-0399: Subparser collections (variable → info)
    /// Maps subparsers variable name to parent parser info
    pub subparsers: HashMap<String, SubparserInfo>,

    /// DEPYLER-0399: Subcommands (parser variable → info)
    /// Maps subcommand parser variable (e.g., "parser_clone") to subcommand details
    pub subcommands: HashMap<String, SubcommandInfo>,

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
    /// 2 (hashmap clears)
    pub fn clear(&mut self) {
        self.parsers.clear();
        self.group_to_parser.clear(); // DEPYLER-0396
        self.subparsers.clear(); // DEPYLER-0399
        self.subcommands.clear(); // DEPYLER-0399
        self.struct_generated = false;
    }

    /// DEPYLER-0396: Register an argument group variable
    /// Maps group variable name to its parent parser
    ///
    /// # Complexity
    /// 1 (hashmap insert)
    pub fn register_group(&mut self, group_var: String, parser_var: String) {
        self.group_to_parser.insert(group_var, parser_var);
    }

    /// DEPYLER-0396: Get parser variable name for a group variable
    /// Returns the parent parser if this variable is an argument group
    /// Recursively resolves nested groups (e.g., format_group → output_group → parser)
    ///
    /// # Complexity
    /// O(depth) where depth is the nesting level of groups (typically 1-3)
    pub fn get_parser_for_group(&self, group_var: &str) -> Option<String> {
        let mut current = group_var;
        let mut visited = std::collections::HashSet::new();

        // Follow the chain until we find a parser or hit a cycle
        loop {
            // Prevent infinite loops from circular references
            if !visited.insert(current) {
                return None;
            }

            // Check if current is a group that maps to something
            if let Some(parent) = self.group_to_parser.get(current) {
                // Check if parent is a parser (ultimate target)
                if self.parsers.contains_key(parent) {
                    return Some(parent.clone());
                }
                // Parent is another group, continue following the chain
                current = parent;
            } else {
                // Not found in group mapping
                return None;
            }
        }
    }

    /// DEPYLER-0399: Register a subparser collection
    /// Pattern: subparsers = parser.add_subparsers(dest="command", required=True)
    ///
    /// # Complexity
    /// 1 (hashmap insert)
    pub fn register_subparsers(&mut self, subparsers_var: String, info: SubparserInfo) {
        self.subparsers.insert(subparsers_var, info);
    }

    /// DEPYLER-0399: Get subparser collection info
    ///
    /// # Complexity
    /// 1 (hashmap lookup)
    pub fn get_subparsers(&self, subparsers_var: &str) -> Option<&SubparserInfo> {
        self.subparsers.get(subparsers_var)
    }

    /// DEPYLER-0399: Get mutable subparser collection info
    ///
    /// # Complexity
    /// 1 (hashmap lookup)
    pub fn get_subparsers_mut(&mut self, subparsers_var: &str) -> Option<&mut SubparserInfo> {
        self.subparsers.get_mut(subparsers_var)
    }

    /// DEPYLER-0399: Register a subcommand
    /// Pattern: parser_clone = subparsers.add_parser("clone", help="...")
    ///
    /// # Complexity
    /// 1 (hashmap insert)
    pub fn register_subcommand(&mut self, subcommand_var: String, info: SubcommandInfo) {
        self.subcommands.insert(subcommand_var, info);
    }

    /// DEPYLER-0399: Get subcommand info
    ///
    /// # Complexity
    /// 1 (hashmap lookup)
    pub fn get_subcommand(&self, subcommand_var: &str) -> Option<&SubcommandInfo> {
        self.subcommands.get(subcommand_var)
    }

    /// DEPYLER-0399: Get mutable subcommand info
    ///
    /// # Complexity
    /// 1 (hashmap lookup)
    pub fn get_subcommand_mut(&mut self, subcommand_var: &str) -> Option<&mut SubcommandInfo> {
        self.subcommands.get_mut(subcommand_var)
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

    /// DEPYLER-0399: Check if any subcommands are defined
    ///
    /// # Complexity
    /// 1 (hashmap empty check)
    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
    }
}

/// DEPYLER-0399: Generate Commands enum from subcommands
///
/// # Complexity
/// 8 (iteration + quote operations)
pub fn generate_commands_enum(tracker: &ArgParserTracker) -> proc_macro2::TokenStream {
    use quote::{format_ident, quote};

    if tracker.subcommands.is_empty() {
        return quote! {};
    }

    let variants: Vec<proc_macro2::TokenStream> = tracker
        .subcommands
        .values()
        .map(|subcommand| {
            emit_decision!("argparse.enum.variant.added", &subcommand.name);
            // Convert "clone" -> "Clone" (PascalCase)
            let variant_name = format_ident!("{}", to_pascal_case(&subcommand.name));

            // Generate help attribute if present
            let help_attr = if let Some(ref help) = subcommand.help {
                quote! { #[command(about = #help)] }
            } else {
                quote! {}
            };

            // Generate fields from subcommand arguments
            let fields: Vec<proc_macro2::TokenStream> = subcommand
                .arguments
                .iter()
                .map(|arg| {
                    let field_name = format_ident!("{}", arg.rust_field_name());
                    let type_str = arg.rust_type();
                    let field_type: syn::Type =
                        syn::parse_str(&type_str).unwrap_or_else(|_| syn::parse_quote! { String });

                    // Generate help attribute
                    let help_attr = if let Some(ref help) = arg.help {
                        quote! { #[doc = #help] }
                    } else {
                        quote! {}
                    };

                    // Generate positional vs flag attributes
                    if arg.is_positional {
                        quote! {
                            #help_attr
                            #field_name: #field_type
                        }
                    } else {
                        quote! {
                            #[arg(long)]
                            #help_attr
                            #field_name: #field_type
                        }
                    }
                })
                .collect();

            quote! {
                #help_attr
                #variant_name {
                    #(#fields),*
                }
            }
        })
        .collect();

    quote! {
        #[derive(clap::Subcommand)]
        enum Commands {
            #(#variants),*
        }
    }
}

/// Convert string to PascalCase (e.g., "clone" -> "Clone", "git-pull" -> "GitPull")
///
/// # Complexity
/// 5 (string operations)
fn to_pascal_case(s: &str) -> String {
    s.split(&['-', '_'][..])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Generate clap Args struct definition from ArgumentParser info
///
/// # Complexity
/// 8 (multiple loops and quote operations)
pub fn generate_args_struct(
    parser_info: &ArgParserInfo,
    tracker: &ArgParserTracker,
) -> proc_macro2::TokenStream {
    use quote::quote;
    use syn::parse_quote;

    // Generate struct fields from arguments
    let mut fields: Vec<proc_macro2::TokenStream> = parser_info
        .arguments
        .iter()
        .map(|arg| {
            let field_name =
                syn::Ident::new(&arg.rust_field_name(), proc_macro2::Span::call_site());

            // DEPYLER-0367: Determine if field should be Option<T>
            let base_type_str = arg.rust_type();

            // Don't wrap in Option if:
            // - Already Option (nargs="?")
            // - Has a default value (will use default_value attribute)
            // - Is required=True
            // - Is positional
            // - Has action with implicit default (store_true/false/count → bool/u8)
            // - Has nargs="+" (required, 1 or more)
            // - DEPYLER-0368: Has action="append" (Vec handles absence as empty)
            // - DEPYLER-0375: Has action="store_const" (bool with implicit default)
            let has_implicit_default = matches!(
                arg.action.as_deref(),
                Some("store_true")
                    | Some("store_false")
                    | Some("count")
                    | Some("append")
                    | Some("store_const")
            );
            // DEPYLER-0370: nargs="+" or nargs=N (specific number) are required
            let is_required_nargs = arg.nargs.as_deref() == Some("+")
                || arg
                    .nargs
                    .as_deref()
                    .map(|s| s.parse::<usize>().is_ok())
                    .unwrap_or(false);

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
                syn::parse_str(&base_type_str).unwrap_or_else(|_| parse_quote! { String })
            };

            // Generate clap attributes
            let mut attrs = vec![];

            if !arg.is_positional {
                // DEPYLER-0365 Phase 5 + DEPYLER-0371: Proper flag detection with dest support
                // Three cases:
                // 1. Both short and long: "-o" + "--output" → #[arg(short = 'o', long)]
                // 2. Long only: "--debug" → #[arg(long)]
                // 3. Short only: "-v" → #[arg(short = 'v')]
                // DEPYLER-0371: If dest is present, use long = "flag_name"

                if arg.long.is_some() {
                    // Case 1: Both short and long flags
                    let short_str = arg.name.trim_start_matches('-');
                    if let Some(short) = short_str.chars().next() {
                        // DEPYLER-0371: If dest is present, specify long name explicitly
                        if arg.dest.is_some() {
                            let long_name = arg.long.as_ref().unwrap().trim_start_matches("--");
                            attrs.push(quote! {
                                #[arg(short = #short, long = #long_name)]
                            });
                        } else {
                            attrs.push(quote! {
                                #[arg(short = #short, long)]
                            });
                        }
                    }
                } else if arg.name.starts_with("--") {
                    // Case 2: Long flag only (--debug)
                    // DEPYLER-0371: If dest is present, specify long name explicitly
                    if arg.dest.is_some() {
                        let long_name = arg.name.trim_start_matches("--");
                        attrs.push(quote! {
                            #[arg(long = #long_name)]
                        });
                    } else {
                        attrs.push(quote! {
                            #[arg(long)]
                        });
                    }
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
            if let Some(crate::hir::HirExpr::Literal(lit)) = arg.default.as_ref() {
                // Convert HIR literal to string for default_value attribute
                let default_str_opt = match lit {
                    crate::hir::Literal::Int(n) => Some(n.to_string()),
                    crate::hir::Literal::Float(f) => Some(f.to_string()),
                    crate::hir::Literal::String(s) => Some(s.clone()),
                    crate::hir::Literal::Bool(b) => Some(b.to_string()),
                    _ => None, // Skip complex defaults
                };
                if let Some(default_str) = default_str_opt {
                    attrs.push(quote! {
                        #[arg(default_value = #default_str)]
                    });
                }
            }

            // DEPYLER-0374: Add default_missing_value for const + nargs="?"
            if arg.nargs.as_deref() == Some("?") && arg.const_value.is_some() {
                if let Some(crate::hir::HirExpr::Literal(lit)) = arg.const_value.as_ref() {
                    let const_str_opt = match lit {
                        crate::hir::Literal::Int(n) => Some(n.to_string()),
                        crate::hir::Literal::Float(f) => Some(f.to_string()),
                        crate::hir::Literal::String(s) => Some(s.clone()),
                        crate::hir::Literal::Bool(b) => Some(b.to_string()),
                        _ => None,
                    };
                    if let Some(const_str) = const_str_opt {
                        attrs.push(quote! {
                            #[arg(default_missing_value = #const_str, num_args = 0..=1)]
                        });
                    }
                }
            }

            // DEPYLER-0370: Add num_args for nargs=N (specific number)
            if let Some(nargs_str) = arg.nargs.as_deref() {
                if let Ok(n) = nargs_str.parse::<usize>() {
                    // Create a literal integer token
                    let n_lit = syn::LitInt::new(&format!("{}", n), proc_macro2::Span::call_site());
                    attrs.push(quote! {
                        #[arg(num_args = #n_lit)]
                    });
                }
            }

            // DEPYLER-0372: Add value_name for metavar
            if let Some(ref metavar) = arg.metavar {
                attrs.push(quote! {
                    #[arg(value_name = #metavar)]
                });
            }

            // DEPYLER-0373: Add value_parser for choices
            if let Some(ref choices) = arg.choices {
                let choice_strs: Vec<_> = choices.iter().collect();
                attrs.push(quote! {
                    #[arg(value_parser = [#(#choice_strs),*])]
                });
            }

            // DEPYLER-0378: Add action attributes for special actions
            match arg.action.as_deref() {
                Some("count") => {
                    attrs.push(quote! {
                        #[arg(action = clap::ArgAction::Count)]
                    });
                }
                Some("store_true") => {
                    attrs.push(quote! {
                        #[arg(action = clap::ArgAction::SetTrue)]
                    });
                }
                Some("store_false") => {
                    attrs.push(quote! {
                        #[arg(action = clap::ArgAction::SetFalse)]
                    });
                }
                _ => {}
            }

            // DEPYLER-0369/0375: Add default_value_t for store_false/store_const
            if arg.action.as_deref() == Some("store_false") {
                // store_false means default is true, becomes false when present
                attrs.push(quote! {
                    #[arg(default_value_t = true)]
                });
            } else if arg.action.as_deref() == Some("store_const") && arg.const_value.is_some() {
                // store_const: default is false, becomes const value when present
                if let Some(crate::hir::HirExpr::Literal(crate::hir::Literal::Bool(_val))) =
                    arg.const_value.as_ref()
                {
                    attrs.push(quote! {
                        #[arg(default_value_t = false)]
                    });
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

    // DEPYLER-0399: Add command field if subcommands exist
    if tracker.has_subcommands() {
        fields.push(quote! {
            #[command(subcommand)]
            command: Commands
        });
    }

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

/// DEPYLER-0425: Analyze which subcommand fields are accessed in a function
///
/// Returns: Option<(variant_name, Vec<field_names>)>
///
/// # Complexity
/// 7 (recursive walk of HIR expressions)
pub fn analyze_subcommand_field_access(
    func: &crate::hir::HirFunction,
    tracker: &ArgParserTracker,
) -> Option<(String, Vec<String>)> {
    use crate::hir::{HirExpr, HirStmt};
    use std::collections::HashSet;

    if !tracker.has_subcommands() {
        return None;
    }

    // Get the args parameter name (should be first parameter if this is a handler)
    let args_param = func.params.first()?.name.as_ref();

    // Build mapping: field_name -> (variant_name, SubcommandInfo)
    let mut field_to_variant: HashMap<String, (String, &SubcommandInfo)> = HashMap::new();
    for subcommand in tracker.subcommands.values() {
        let variant_name = to_pascal_case(&subcommand.name);
        for arg in &subcommand.arguments {
            let field_name = arg.rust_field_name();
            field_to_variant.insert(field_name, (variant_name.clone(), subcommand));
        }
    }

    // Track which subcommand fields are accessed
    let mut accessed_fields: HashSet<String> = HashSet::new();
    let mut detected_variant: Option<String> = None;

    // Recursive function to walk expressions
    fn walk_expr(
        expr: &HirExpr,
        args_param: &str,
        field_to_variant: &HashMap<String, (String, &SubcommandInfo)>,
        accessed_fields: &mut HashSet<String>,
        detected_variant: &mut Option<String>,
    ) {
        match expr {
            HirExpr::Attribute { value, attr } => {
                // Check if this is args.field_name
                if let HirExpr::Var(id) = &**value {
                    if id == args_param {
                        // This is an attribute access on args
                        if let Some((variant_name, _)) = field_to_variant.get(attr.as_str()) {
                            // This field belongs to a subcommand variant
                            accessed_fields.insert(attr.clone());
                            if detected_variant.is_none() {
                                *detected_variant = Some(variant_name.clone());
                            }
                        }
                    }
                }
                // Recurse into value
                walk_expr(
                    value,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::Binary { left, right, .. } => {
                walk_expr(
                    left,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                walk_expr(
                    right,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::Unary { operand, .. } => {
                walk_expr(
                    operand,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::Call { args, .. } => {
                // Note: func is a Symbol, not an HirExpr
                for arg in args {
                    walk_expr(
                        arg,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                walk_expr(
                    object,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for arg in args {
                    walk_expr(
                        arg,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirExpr::List(elements)
            | HirExpr::Tuple(elements)
            | HirExpr::Set(elements)
            | HirExpr::FrozenSet(elements) => {
                for elem in elements {
                    walk_expr(
                        elem,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirExpr::Dict(items) => {
                for (key, value) in items {
                    walk_expr(
                        key,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                    walk_expr(
                        value,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirExpr::Index { base, index } => {
                walk_expr(
                    base,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                walk_expr(
                    index,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                walk_expr(
                    base,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                if let Some(s) = start {
                    walk_expr(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
                if let Some(s) = stop {
                    walk_expr(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
                if let Some(s) = step {
                    walk_expr(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirExpr::Borrow { expr, .. } => {
                walk_expr(
                    expr,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::ListComp {
                element,
                generators,
            }
            | HirExpr::SetComp {
                element,
                generators,
            } => {
                // DEPYLER-0504: Support multiple generators
                walk_expr(
                    element,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for gen in generators {
                    walk_expr(
                        &gen.iter,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                    for cond in &gen.conditions {
                        walk_expr(
                            cond,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
            }
            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                // DEPYLER-0504: Support multiple generators
                walk_expr(
                    key,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                walk_expr(
                    value,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for gen in generators {
                    walk_expr(
                        &gen.iter,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                    for cond in &gen.conditions {
                        walk_expr(
                            cond,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
            }
            HirExpr::Lambda { body, .. } => {
                walk_expr(
                    body,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::FString { parts } => {
                // DEPYLER-0425: Walk f-string interpolated expressions
                for part in parts {
                    if let crate::hir::FStringPart::Expr(expr) = part {
                        walk_expr(
                            expr,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
            }
            HirExpr::IfExpr { test, body, orelse } => {
                walk_expr(
                    test,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                walk_expr(
                    body,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                walk_expr(
                    orelse,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::SortByKey {
                iterable, key_body, ..
            } => {
                walk_expr(
                    iterable,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                walk_expr(
                    key_body,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::GeneratorExp {
                element,
                generators,
            } => {
                walk_expr(
                    element,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for gen in generators {
                    walk_expr(
                        &gen.iter,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                    for cond in &gen.conditions {
                        walk_expr(
                            cond,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
            }
            HirExpr::Await { value } => {
                walk_expr(
                    value,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::Yield { value: Some(v) } => {
                walk_expr(
                    v,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
            }
            HirExpr::Yield { value: None } => {}
            _ => {}
        }
    }

    // Walk all statements in function body
    fn walk_stmt(
        stmt: &HirStmt,
        args_param: &str,
        field_to_variant: &HashMap<String, (String, &SubcommandInfo)>,
        accessed_fields: &mut HashSet<String>,
        detected_variant: &mut Option<String>,
    ) {
        match stmt {
            HirStmt::Expr(expr) => walk_expr(
                expr,
                args_param,
                field_to_variant,
                accessed_fields,
                detected_variant,
            ),
            HirStmt::Assign { value, .. } => walk_expr(
                value,
                args_param,
                field_to_variant,
                accessed_fields,
                detected_variant,
            ),
            HirStmt::Return(Some(expr)) => walk_expr(
                expr,
                args_param,
                field_to_variant,
                accessed_fields,
                detected_variant,
            ),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                walk_expr(
                    condition,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for s in then_body {
                    walk_stmt(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        walk_stmt(
                            s,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
            }
            HirStmt::While { condition, body } => {
                walk_expr(
                    condition,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for s in body {
                    walk_stmt(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirStmt::For { body, .. } => {
                for s in body {
                    walk_stmt(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirStmt::With { context, body, .. } => {
                walk_expr(
                    context,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                for s in body {
                    walk_stmt(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for s in body {
                    walk_stmt(
                        s,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
                for handler in handlers {
                    for s in &handler.body {
                        walk_stmt(
                            s,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
                if let Some(orelse_stmts) = orelse {
                    for s in orelse_stmts {
                        walk_stmt(
                            s,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for s in final_stmts {
                        walk_stmt(
                            s,
                            args_param,
                            field_to_variant,
                            accessed_fields,
                            detected_variant,
                        );
                    }
                }
            }
            HirStmt::Assert { test, msg } => {
                walk_expr(
                    test,
                    args_param,
                    field_to_variant,
                    accessed_fields,
                    detected_variant,
                );
                if let Some(msg_expr) = msg {
                    walk_expr(
                        msg_expr,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            HirStmt::Raise { exception, cause } => {
                if let Some(exc) = exception {
                    walk_expr(
                        exc,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
                if let Some(cause_expr) = cause {
                    walk_expr(
                        cause_expr,
                        args_param,
                        field_to_variant,
                        accessed_fields,
                        detected_variant,
                    );
                }
            }
            _ => {}
        }
    }

    for stmt in &func.body {
        walk_stmt(
            stmt,
            args_param,
            &field_to_variant,
            &mut accessed_fields,
            &mut detected_variant,
        );
    }

    // If we found a variant and accessed fields, return them
    if let Some(variant) = detected_variant {
        let mut fields: Vec<String> = accessed_fields.into_iter().collect();
        fields.sort(); // Deterministic order
        Some((variant, fields))
    } else {
        None
    }
}

/// DEPYLER-0425: Wrap function body statements in pattern matching for subcommand field extraction
///
/// # Complexity
/// 5 (quote operations + iteration)
pub fn wrap_body_with_subcommand_pattern(
    body_stmts: Vec<proc_macro2::TokenStream>,
    variant_name: &str,
    fields: &[String],
    args_param: &str,
) -> Vec<proc_macro2::TokenStream> {
    use quote::{format_ident, quote};

    let variant_ident = format_ident!("{}", variant_name);
    let args_ident = format_ident!("{}", args_param);
    let field_idents: Vec<syn::Ident> = fields.iter().map(|f| format_ident!("{}", f)).collect();

    vec![quote! {
        if let Commands::#variant_ident { #(#field_idents),* } = &#args_ident.command {
            #(#body_stmts)*
        }
    }]
}

/// DEPYLER-0456 Bug #1: Pre-scan HIR function body to register all add_parser() calls
/// This must run BEFORE body codegen so Commands enum includes expression statement subcommands
///
/// # Complexity
/// 8 (recursive HIR walk)
pub fn preregister_subcommands_from_hir(
    func: &crate::hir::HirFunction,
    tracker: &mut ArgParserTracker,
) {
    use crate::hir::{HirExpr, HirStmt};

    // Helper to extract string literal value from HIR expression
    fn extract_string_from_hir(expr: &HirExpr) -> String {
        match expr {
            HirExpr::Literal(crate::hir::Literal::String(s)) => s.clone(),
            _ => String::new(),
        }
    }

    // Helper to extract keyword argument string value
    fn extract_kwarg_string_from_hir(kwargs: &[(String, HirExpr)], key: &str) -> Option<String> {
        kwargs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| extract_string_from_hir(v))
    }

    // Recursive walker for expressions
    fn walk_expr(expr: &HirExpr, tracker: &mut ArgParserTracker) {
        match expr {
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } if method == "add_parser" => {
                // Check if this is a call on a subparsers variable
                if let HirExpr::Var(subparsers_var) = object.as_ref() {
                    if tracker.get_subparsers(subparsers_var).is_some() {
                        // Extract command name and help text
                        if !args.is_empty() {
                            let command_name = extract_string_from_hir(&args[0]);
                            emit_decision!("argparse.subcommand.detected", &command_name);
                            let help = extract_kwarg_string_from_hir(kwargs, "help");

                            // Register subcommand (use command name as key for expression statements)
                            let subcommand_info = SubcommandInfo {
                                name: command_name.clone(),
                                help,
                                arguments: vec![],
                                subparsers_var: subparsers_var.clone(),
                            };

                            tracker.register_subcommand(command_name.clone(), subcommand_info);
                            emit_decision!("argparse.subcommand.registered", &command_name);
                        }
                    }
                }
                // Recurse into method call arguments
                walk_expr(object, tracker);
                for arg in args {
                    walk_expr(arg, tracker);
                }
                for (_, val) in kwargs {
                    walk_expr(val, tracker);
                }
            }
            // Recurse into all other expression types
            HirExpr::Binary { left, right, .. } => {
                walk_expr(left, tracker);
                walk_expr(right, tracker);
            }
            HirExpr::Unary { operand, .. } => {
                walk_expr(operand, tracker);
            }
            HirExpr::Call { args, kwargs, .. } => {
                for arg in args {
                    walk_expr(arg, tracker);
                }
                for (_, val) in kwargs {
                    walk_expr(val, tracker);
                }
            }
            HirExpr::MethodCall {
                object,
                args,
                kwargs,
                ..
            } => {
                walk_expr(object, tracker);
                for arg in args {
                    walk_expr(arg, tracker);
                }
                for (_, val) in kwargs {
                    walk_expr(val, tracker);
                }
            }
            HirExpr::Attribute { value, .. } => {
                walk_expr(value, tracker);
            }
            HirExpr::List(items)
            | HirExpr::Tuple(items)
            | HirExpr::Set(items)
            | HirExpr::FrozenSet(items) => {
                for item in items {
                    walk_expr(item, tracker);
                }
            }
            HirExpr::Dict(items) => {
                for (k, v) in items {
                    walk_expr(k, tracker);
                    walk_expr(v, tracker);
                }
            }
            HirExpr::Index { base, index } => {
                walk_expr(base, tracker);
                walk_expr(index, tracker);
            }
            HirExpr::IfExpr { test, body, orelse } => {
                walk_expr(test, tracker);
                walk_expr(body, tracker);
                walk_expr(orelse, tracker);
            }
            _ => {} // Literals, vars, etc. - no recursion needed
        }
    }

    // Recursive walker for statements
    fn walk_stmt(stmt: &HirStmt, tracker: &mut ArgParserTracker) {
        match stmt {
            HirStmt::Expr(expr) => walk_expr(expr, tracker),
            HirStmt::Assign {
                target,
                value,
                type_annotation: _,
            } => {
                // Special handling for ArgumentParser() assignments
                // Pattern: parser = argparse.ArgumentParser(...)
                if let HirExpr::Call { func, kwargs, .. } = value {
                    if func == "ArgumentParser" {
                        if let crate::hir::AssignTarget::Symbol(parser_var) = target {
                            // Register parser
                            let description = extract_kwarg_string_from_hir(kwargs, "description");
                            let epilog = extract_kwarg_string_from_hir(kwargs, "epilog");

                            let parser_info = ArgParserInfo {
                                parser_var: parser_var.clone(),
                                description,
                                epilog,
                                arguments: vec![],
                                args_var: None,
                            };

                            tracker.register_parser(parser_var.clone(), parser_info);
                        }
                    }
                }

                // Special handling for add_subparsers() assignments
                // Pattern: subparsers = parser.add_subparsers(...)
                if let HirExpr::MethodCall {
                    object,
                    method,
                    kwargs,
                    ..
                } = value
                {
                    if method == "add_subparsers" {
                        if let HirExpr::Var(parser_var) = object.as_ref() {
                            if tracker.get_parser(parser_var).is_some() {
                                if let crate::hir::AssignTarget::Symbol(subparsers_var) = target {
                                    let dest_field = extract_kwarg_string_from_hir(kwargs, "dest")
                                        .unwrap_or_else(|| "command".to_string());
                                    let required =
                                        extract_kwarg_string_from_hir(kwargs, "required")
                                            .map(|s| s == "true" || s == "True")
                                            .unwrap_or(false);
                                    let help = extract_kwarg_string_from_hir(kwargs, "help");

                                    let subparser_info = SubparserInfo {
                                        parser_var: parser_var.clone(),
                                        dest_field,
                                        required,
                                        help,
                                    };

                                    // Use actual variable name from assignment
                                    tracker.register_subparsers(
                                        subparsers_var.clone(),
                                        subparser_info,
                                    );
                                }
                            }
                        }
                    }
                }
                // Also walk value for other method calls (e.g., nested add_parser() calls)
                walk_expr(value, tracker);
            }
            HirStmt::Return(Some(expr)) => walk_expr(expr, tracker),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                walk_expr(condition, tracker);
                for s in then_body {
                    walk_stmt(s, tracker);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        walk_stmt(s, tracker);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                walk_expr(condition, tracker);
                for s in body {
                    walk_stmt(s, tracker);
                }
            }
            HirStmt::For { body, .. } => {
                for s in body {
                    walk_stmt(s, tracker);
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for s in body {
                    walk_stmt(s, tracker);
                }
                for handler in handlers {
                    for s in &handler.body {
                        walk_stmt(s, tracker);
                    }
                }
                if let Some(orelse_stmts) = orelse {
                    for s in orelse_stmts {
                        walk_stmt(s, tracker);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for s in final_stmts {
                        walk_stmt(s, tracker);
                    }
                }
            }
            _ => {} // Other statement types don't contain add_parser() calls
        }
    }

    // Walk all statements in function body
    for stmt in &func.body {
        walk_stmt(stmt, tracker);
    }
}
