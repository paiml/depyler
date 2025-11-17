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
            || self.action.as_deref() == Some("store_const") {
            return "bool".to_string();
        }

        // action="count" → u8 (counts occurrences: -v -v -v → 3)
        if self.action.as_deref() == Some("count") {
            return "u8".to_string();
        }

        // DEPYLER-0368: action="append" → Vec<T> (collects multiple flag uses)
        if self.action.as_deref() == Some("append") {
            let inner_type = self.arg_type.as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Vec<{}>", inner_type);
        }

        // nargs="+" or nargs="*" → Vec<T>
        if self.nargs.as_deref() == Some("+") || self.nargs.as_deref() == Some("*") {
            let inner_type = self.arg_type.as_ref()
                .map(type_to_rust_string)
                .unwrap_or_else(|| "String".to_string());
            return format!("Vec<{}>", inner_type);
        }

        // DEPYLER-0370: nargs=N (specific number) → Vec<T>
        if let Some(nargs_str) = self.nargs.as_deref() {
            if nargs_str.parse::<usize>().is_ok() {
                let inner_type = self.arg_type.as_ref()
                    .map(type_to_rust_string)
                    .unwrap_or_else(|| "String".to_string());
                return format!("Vec<{}>", inner_type);
            }
        }

        // nargs="?" → Option<T>
        // DEPYLER-0374: Handle const parameter with nargs="?" separately in generate_args_struct
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
    use quote::{quote, format_ident};

    if tracker.subcommands.is_empty() {
        return quote! {};
    }

    let variants: Vec<proc_macro2::TokenStream> = tracker.subcommands.values().map(|subcommand| {
        // Convert "clone" -> "Clone" (PascalCase)
        let variant_name = format_ident!("{}", to_pascal_case(&subcommand.name));

        // Generate help attribute if present
        let help_attr = if let Some(ref help) = subcommand.help {
            quote! { #[command(about = #help)] }
        } else {
            quote! {}
        };

        // Generate fields from subcommand arguments
        let fields: Vec<proc_macro2::TokenStream> = subcommand.arguments.iter().map(|arg| {
            let field_name = format_ident!("{}", arg.rust_field_name());
            let type_str = arg.rust_type();
            let field_type: syn::Type = syn::parse_str(&type_str).unwrap_or_else(|_| syn::parse_quote! { String });

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
        }).collect();

        quote! {
            #help_attr
            #variant_name {
                #(#fields),*
            }
        }
    }).collect();

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
pub fn generate_args_struct(parser_info: &ArgParserInfo, tracker: &ArgParserTracker) -> proc_macro2::TokenStream {
    use quote::quote;
    use syn::parse_quote;

    // Generate struct fields from arguments
    let mut fields: Vec<proc_macro2::TokenStream> = parser_info
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
            // - DEPYLER-0368: Has action="append" (Vec handles absence as empty)
            // - DEPYLER-0375: Has action="store_const" (bool with implicit default)
            let has_implicit_default = matches!(
                arg.action.as_deref(),
                Some("store_true") | Some("store_false") | Some("count") | Some("append") | Some("store_const")
            );
            // DEPYLER-0370: nargs="+" or nargs=N (specific number) are required
            let is_required_nargs = arg.nargs.as_deref() == Some("+") ||
                arg.nargs.as_deref().map(|s| s.parse::<usize>().is_ok()).unwrap_or(false);

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
                    _ => None,  // Skip complex defaults
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
                if let Some(crate::hir::HirExpr::Literal(crate::hir::Literal::Bool(_val))) = arg.const_value.as_ref() {
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
