//! Attribute access and borrow conversion for ExpressionConverter
//!
//! Contains convert_attribute, convert_borrow, wrap_range_in_parens.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // DEPYLER-1402: Handle type(x).__name__ pattern
        // In Python, type(x).__name__ returns the type name as a string.
        // Since type(x) already maps to std::any::type_name_of_val(&x) which returns &str,
        // we just return the type() call without the .__name__ suffix.
        if attr == "__name__" {
            if let HirExpr::Call { func, args, .. } = value {
                if func == "type" && args.len() == 1 {
                    // type(x).__name__ → std::any::type_name_of_val(&x)
                    let arg_expr = args[0].to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { std::any::type_name_of_val(&#arg_expr) });
                }
            }
        }

        // DEPYLER-0608: In cmd_* handlers, args.X → X (field is now a direct parameter)
        // This is because subcommand fields live in Commands::Variant, not on Args
        // The handler function now takes individual field parameters instead of &Args
        if self.ctx.in_cmd_handler {
            if let HirExpr::Var(var_name) = value {
                if var_name == "args"
                    && self.ctx.cmd_handler_args_fields.contains(&attr.to_string())
                {
                    // Transform args.field → field (the field is now a direct parameter)
                    // DEPYLER-0941: Handle Rust keywords like "type" with raw identifier syntax
                    let attr_ident = if keywords::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // DEPYLER-0627: subprocess.run() now returns CompletedProcess struct
        // with .returncode, .stdout, .stderr fields - no conversion needed,
        // struct field access works directly

        // DEPYLER-0200: Handle os.environ direct access
        // os.environ → std::env::vars() as a HashMap-like collection
        if let HirExpr::Var(var_name) = value {
            if var_name == "os" && attr == "environ" {
                // os.environ returns an environment dict-like object
                // Convert to HashMap<String, String> for dict-like operations
                return Ok(parse_quote! {
                    std::env::vars().collect::<std::collections::HashMap<String, String>>()
                });
            }
        }

        // DEPYLER-1069: Handle datetime class constants (min, max, resolution)
        // date.min → DepylerDate::new(1, 1, 1)
        // datetime.min → DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0)
        // time.min → (0, 0, 0, 0)
        // timedelta.min → DepylerTimeDelta::new(-999999999, 0, 0)
        if let HirExpr::Var(var_name) = value {
            let nasa_mode = self.ctx.type_mapper.nasa_mode;
            if (var_name == "date"
                || var_name == "datetime"
                || var_name == "time"
                || var_name == "timedelta")
                && (attr == "min" || attr == "max" || attr == "resolution")
            {
                if var_name == "date" {
                    self.ctx.needs_depyler_date = true;
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { DepylerDate::new(1, 1, 1) }
                        } else {
                            parse_quote! { DepylerDate::new(9999, 12, 31) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveDate::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDate::MAX }
                        });
                    }
                } else if var_name == "datetime" {
                    self.ctx.needs_depyler_datetime = true;
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0) }
                        } else {
                            parse_quote! { DepylerDateTime::new(9999, 12, 31, 23, 59, 59, 999999) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveDateTime::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDateTime::MAX }
                        });
                    }
                } else if var_name == "time" {
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { (0u32, 0u32, 0u32, 0u32) }
                        } else if attr == "max" {
                            parse_quote! { (23u32, 59u32, 59u32, 999999u32) }
                        } else {
                            // resolution
                            parse_quote! { (0u32, 0u32, 0u32, 1u32) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveTime::MIN }
                        } else if attr == "max" {
                            parse_quote! { chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).expect("operation failed") }
                        } else {
                            // resolution
                            parse_quote! { chrono::Duration::microseconds(1) }
                        });
                    }
                } else if var_name == "timedelta" {
                    self.ctx.needs_depyler_timedelta = true;
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            // timedelta.min = timedelta(-999999999)
                            parse_quote! { DepylerTimeDelta::new(-999999999, 0, 0) }
                        } else if attr == "max" {
                            // timedelta.max = timedelta(days=999999999, hours=23, minutes=59, seconds=59, microseconds=999999)
                            parse_quote! { DepylerTimeDelta::new(999999999, 86399, 999999) }
                        } else {
                            // resolution = timedelta(microseconds=1)
                            parse_quote! { DepylerTimeDelta::new(0, 0, 1) }
                        });
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::Duration::min_value() }
                        } else if attr == "max" {
                            parse_quote! { chrono::Duration::max_value() }
                        } else {
                            // resolution
                            parse_quote! { chrono::Duration::microseconds(1) }
                        });
                    }
                }
            }
        }

        if let HirExpr::Var(var_name) = value {
            // DEPYLER-0517: Handle exception variable attributes
            // Python: `except CalledProcessError as e: e.returncode`
            // Rust: Box<dyn Error> doesn't have returncode, use fallback
            // Common exception variable names: e, err, error, exc, exception
            let is_likely_exception = var_name == "e"
                || var_name == "err"
                || var_name == "error"
                || var_name == "exc"
                || var_name == "exception";

            if is_likely_exception && attr == "returncode" {
                // Use 1 as a generic non-zero exit code for errors
                return Ok(parse_quote! { 1 });
            }

            // DEPYLER-0535: Handle tempfile file handle attributes
            // Python: f.name → Rust: f.path().to_string_lossy().to_string()
            // Common tempfile variable names: f, temp, temp_file, tmpfile
            let is_likely_tempfile = var_name == "f"
                || var_name == "temp"
                || var_name == "tmp"
                || var_name.contains("temp")
                || var_name.contains("tmp");

            if is_likely_tempfile && attr == "name" {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #var_ident.path().to_string_lossy().to_string() });
            }

            // DEPYLER-0551: Handle os.stat_result attributes (from path.stat() / std::fs::metadata)
            // Python: stats.st_size → Rust: stats.len()
            // Python: stats.st_mtime → Rust: stats.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
            let is_likely_stats =
                var_name == "stats" || var_name == "stat" || var_name.ends_with("_stats");

            if is_likely_stats {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match attr {
                    "st_size" => {
                        // DEPYLER-0693: Cast file size to i64 (Python int can be large)
                        return Ok(parse_quote! { #var_ident.len() as i64 });
                    }
                    "st_mtime" => {
                        return Ok(parse_quote! {
                            #var_ident.modified().expect("operation failed").duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs_f64()
                        });
                    }
                    "st_ctime" => {
                        // Creation time (use modified as fallback on Unix)
                        return Ok(parse_quote! {
                            #var_ident.created().unwrap_or_else(|_| #var_ident.modified().expect("operation failed"))
                                .duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs_f64()
                        });
                    }
                    "st_atime" => {
                        return Ok(parse_quote! {
                            #var_ident.accessed().expect("operation failed").duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs_f64()
                        });
                    }
                    "st_mode" => {
                        // File permissions
                        return Ok(parse_quote! { #var_ident.permissions().mode() });
                    }
                    _ => {} // Fall through
                }
            }

            // DEPYLER-0551: Handle pathlib.Path attributes
            // Python: path.name → Rust: path.file_name().and_then(|n| n.to_str()).unwrap_or("")
            // Python: path.suffix → Rust: path.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
            // DEPYLER-0706: Removed `var_name == "p"` - too many false positives (e.g., Person p, Point p)
            // Only use explicit path naming patterns to avoid confusing struct field access with path operations
            // DEPYLER-0942: Also check var_types for PathBuf/Path type assignment
            let is_named_path = var_name == "path" || var_name.ends_with("_path");
            let is_typed_path = self
                .ctx
                .var_types
                .get(var_name)
                .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                .unwrap_or(false);
            let is_likely_path = is_named_path || is_typed_path;

            if is_likely_path {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match attr {
                    "name" => {
                        return Ok(parse_quote! {
                            #var_ident.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
                        });
                    }
                    "suffix" => {
                        return Ok(parse_quote! {
                            #var_ident.extension().map(|e| format!(".{}", e.to_str().expect("operation failed"))).unwrap_or_default()
                        });
                    }
                    "stem" => {
                        return Ok(parse_quote! {
                            #var_ident.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()
                        });
                    }
                    "parent" => {
                        return Ok(parse_quote! {
                            #var_ident.parent().map(|p| p.to_path_buf()).unwrap_or_default()
                        });
                    }
                    _ => {} // Fall through to regular attribute handling
                }
            }
        }

        // DEPYLER-0425: Handle subcommand field access (args.url → url)
        // If this is accessing a subcommand-specific field on args parameter,
        // generate just the field name (it's extracted via pattern matching)
        if let HirExpr::Var(var_name) = value {
            // Check if var_name is an args parameter
            // (heuristic: variable ending in "args" or exactly "args")
            if (var_name == "args" || var_name.ends_with("args"))
                && self.ctx.argparser_tracker.has_subcommands()
            {
                // Check if this field belongs to any subcommand
                let mut is_subcommand_field = false;
                for subcommand in self.ctx.argparser_tracker.subcommands.values() {
                    for arg in &subcommand.arguments {
                        if arg.rust_field_name() == attr {
                            is_subcommand_field = true;
                            break;
                        }
                    }
                    if is_subcommand_field {
                        break;
                    }
                }

                if is_subcommand_field {
                    // Generate just the field name (extracted via pattern matching in func wrapper)
                    let attr_ident = if keywords::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.ctx.is_classmethod {
                // DEPYLER-0306 FIX: Use raw identifiers for attributes that are Rust keywords
                let attr_ident = if keywords::is_rust_keyword(attr) {
                    syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(attr, proc_macro2::Span::call_site())
                };
                return Ok(parse_quote! { Self::#attr_ident });
            }

            // DEPYLER-0422 Fix #11: Detect enum constant access patterns
            // TypeName.CONSTANT → TypeName::CONSTANT
            // Five-Whys Root Cause:
            // 1. Why: E0423 - expected value, found struct 'Color'
            // 2. Why: Code generates Color.RED (field access) instead of Color::RED
            // 3. Why: Default attribute access uses dot syntax
            // 4. Why: No detection for type constant access vs field access
            // 5. ROOT CAUSE: Need to use :: for type-level constants
            //
            // Heuristic: If name starts with uppercase and attr is ALL_CAPS, it's likely an enum constant
            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            // DEPYLER-CONVERGE-MULTI: Allow digits in constant names (e.g. FP8_E4M3)
            let is_constant = attr
                .chars()
                .all(|c| c.is_uppercase() || c == '_' || c.is_ascii_digit());

            if is_type_name && is_constant {
                let type_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        // Check if this is a module attribute access
        if let HirExpr::Var(module_name) = value {
            // DEPYLER-STDLIB-MATH: Handle math module constants
            // math.pi → std::f64::consts::PI
            // math.e → std::f64::consts::E
            // math.inf → f64::INFINITY
            // math.nan → f64::NAN
            if module_name == "math" {
                let result = match attr {
                    "pi" => parse_quote! { std::f64::consts::PI },
                    "e" => parse_quote! { std::f64::consts::E },
                    "tau" => parse_quote! { std::f64::consts::TAU },
                    "inf" => parse_quote! { f64::INFINITY },
                    "nan" => parse_quote! { f64::NAN },
                    // DEPYLER-0595: Math functions as first-class values
                    "sin" => parse_quote! { f64::sin },
                    "cos" => parse_quote! { f64::cos },
                    "tan" => parse_quote! { f64::tan },
                    "asin" => parse_quote! { f64::asin },
                    "acos" => parse_quote! { f64::acos },
                    "atan" => parse_quote! { f64::atan },
                    "sqrt" => parse_quote! { f64::sqrt },
                    "exp" => parse_quote! { f64::exp },
                    "log" => parse_quote! { f64::ln },
                    "log10" => parse_quote! { f64::log10 },
                    "floor" => parse_quote! { f64::floor },
                    "ceil" => parse_quote! { f64::ceil },
                    "abs" => parse_quote! { f64::abs },
                    _ => {
                        // If it's not a recognized constant/function, it might be a typo
                        bail!("math.{} is not a recognized constant or method", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-STDLIB-STRING: Handle string module constants
            // string.ascii_letters → "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            // string.digits → "0123456789"
            // string.punctuation → "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
            if module_name == "string" {
                let result = match attr {
                    "ascii_lowercase" => parse_quote! { "abcdefghijklmnopqrstuvwxyz" },
                    "ascii_uppercase" => parse_quote! { "ABCDEFGHIJKLMNOPQRSTUVWXYZ" },
                    "ascii_letters" => {
                        parse_quote! { "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" }
                    }
                    "digits" => parse_quote! { "0123456789" },
                    "hexdigits" => parse_quote! { "0123456789abcdefABCDEF" },
                    "octdigits" => parse_quote! { "01234567" },
                    "punctuation" => parse_quote! { "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~" },
                    "whitespace" => parse_quote! { " \t\n\r\x0b\x0c" },
                    "printable" => {
                        parse_quote! { "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c" }
                    }
                    _ => {
                        // Not a string constant - might be a method like capwords
                        bail!("string.{} is not a recognized constant", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-0518: Handle re module constants
            // Python regex flags: re.IGNORECASE, re.MULTILINE, etc.
            // These are integer flags in Python but Rust regex uses builder methods.
            // For now, map them to constants that can be used in conditional checks.
            if module_name == "re" {
                let result = match attr {
                    // Map to integer constants (matching Python values for compatibility)
                    "IGNORECASE" | "I" => parse_quote! { 2i32 },
                    "MULTILINE" | "M" => parse_quote! { 8i32 },
                    "DOTALL" | "S" => parse_quote! { 16i32 },
                    "VERBOSE" | "X" => parse_quote! { 64i32 },
                    "ASCII" | "A" => parse_quote! { 256i32 },
                    "LOCALE" | "L" => parse_quote! { 4i32 },
                    "UNICODE" | "U" => parse_quote! { 32i32 },
                    _ => {
                        // Not a recognized constant - fall through to default handling
                        let module_ident =
                            syn::Ident::new(module_name, proc_macro2::Span::call_site());
                        let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                        return Ok(parse_quote! { #module_ident.#attr_ident });
                    }
                };
                return Ok(result);
            }

            // DEPYLER-STDLIB-SYS: Handle sys module attributes
            // sys.argv → std::env::args().collect()
            // sys.platform → compile-time platform string
            // DEPYLER-0381: sys.stdin/stdout/stderr → std::io::stdin()/stdout()/stderr()
            if module_name == "sys" {
                let result = match attr {
                    "argv" => parse_quote! { std::env::args().collect::<Vec<String>>() },
                    "platform" => {
                        // Return platform name based on target OS as String
                        #[cfg(target_os = "linux")]
                        let platform = "linux";
                        #[cfg(target_os = "macos")]
                        let platform = "darwin";
                        #[cfg(target_os = "windows")]
                        let platform = "win32";
                        #[cfg(not(any(
                            target_os = "linux",
                            target_os = "macos",
                            target_os = "windows"
                        )))]
                        let platform = "unknown";
                        parse_quote! { #platform.to_string() }
                    }
                    // DEPYLER-0381: I/O stream attributes (functions in Rust, not objects)
                    "stdin" => parse_quote! { std::io::stdin() },
                    "stdout" => parse_quote! { std::io::stdout() },
                    "stderr" => parse_quote! { std::io::stderr() },
                    // DEPYLER-0381: version_info as a tuple (major, minor)
                    // Note: Python's sys.version_info is a 5-tuple (major, minor, micro, releaselevel, serial)
                    // but most comparisons use only (major, minor), so we return a 2-tuple for compatibility
                    "version_info" => {
                        // Rust doesn't have runtime version info by default
                        // Return a compile-time constant tuple matching Python 3.11
                        parse_quote! { (3, 11) }
                    }
                    _ => {
                        bail!("sys.{} is not a recognized attribute", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-0335 FIX #2: Get rust_path and rust_name (clone to avoid borrow issues)
            let module_info = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| {
                    mapping
                        .item_map
                        .get(attr)
                        .map(|rust_name| (mapping.rust_path.clone(), rust_name.clone()))
                });

            if let Some((rust_path, rust_name)) = module_info {
                // Map to the Rust equivalent
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                if path_parts.len() > 1 {
                    // DEPYLER-0335 FIX #2: Use rust_path from mapping instead of hardcoding "std"
                    let base_path: syn::Path =
                        syn::parse_str(&rust_path).unwrap_or_else(|_| parse_quote! { std });
                    let mut path = quote! { #base_path };
                    for part in path_parts {
                        let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                        path = quote! { #path::#part_ident };
                    }
                    return Ok(parse_quote! { #path });
                } else {
                    // Simple identifier
                    let ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #ident });
                }
            }
        }

        // DEPYLER-STDLIB-DATETIME: Handle datetime/date/time/timedelta properties
        // In chrono, properties are accessed as methods: dt.year → dt.year()
        // This handles properties for fractions, pathlib, datetime, date, time, and timedelta instances
        let value_expr = value.to_rust_expr(self.ctx)?;
        match attr {
            // DEPYLER-STDLIB-FRACTIONS: Fraction properties
            "numerator" => {
                // f.numerator → *f.numer()
                return Ok(parse_quote! { *#value_expr.numer() });
            }

            "denominator" => {
                // f.denominator → *f.denom()
                return Ok(parse_quote! { *#value_expr.denom() });
            }

            // DEPYLER-STDLIB-PATHLIB: Path properties
            // DEPYLER-0357: Removed overly-aggressive "name" special case
            // The .name attribute should only map to .file_name() for Path types
            // For generic objects (like in sorted(people, key=lambda p: p.name)),
            // .name should be preserved as-is and fall through to default handling
            "stem" => {
                // p.stem → p.file_stem().unwrap().to_str().unwrap().to_string()
                return Ok(parse_quote! {
                    #value_expr.file_stem().expect("operation failed").to_str().expect("operation failed").to_string()
                });
            }

            "suffix" => {
                // p.suffix → p.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
                return Ok(parse_quote! {
                    #value_expr.extension()
                        .map(|e| format!(".{}", e.to_str().expect("operation failed")))
                        .unwrap_or_default()
                });
            }

            "parent" => {
                // p.parent → p.parent().unwrap().to_path_buf()
                return Ok(parse_quote! {
                    #value_expr.parent().expect("operation failed").to_path_buf()
                });
            }

            "parts" => {
                // p.parts → p.components().map(|c| c.as_os_str().to_str().unwrap().to_string()).collect()
                return Ok(parse_quote! {
                    #value_expr.components()
                        .map(|c| c.as_os_str().to_str().expect("operation failed").to_string())
                        .collect::<Vec<_>>()
                });
            }

            // datetime/date properties (require method calls in chrono)
            "year" | "month" | "day" | "hour" | "minute" | "second" | "microsecond" => {
                // DEPYLER-99MODE-E0308-P1: Check if value is an &mut Option<T> parameter
                // If so, we need to unwrap it first: as_of.year → as_of.as_ref().unwrap().year()
                if let HirExpr::Var(var_name) = value {
                    if self.ctx.mut_option_params.contains(var_name) {
                        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                        let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident() as i32 },
                        );
                    }
                }
                // Check if this might be a datetime/date/time object
                // We convert: dt.year → dt.year()
                let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #value_expr.#method_ident() as i32 });
            }

            // DEPYLER-1068: timedelta properties with DepylerTimeDelta
            // These now work correctly with the DepylerTimeDelta wrapper struct
            // which provides proper .days(), .seconds(), .microseconds() methods
            "days" => {
                // td.days → td.days() as i32 (DepylerTimeDelta returns i64)
                return Ok(parse_quote! { #value_expr.days() as i32 });
            }

            "seconds" => {
                // td.seconds → td.seconds() as i32
                return Ok(parse_quote! { #value_expr.seconds() as i32 });
            }

            "microseconds" => {
                // td.microseconds → td.microseconds() as i32
                return Ok(parse_quote! { #value_expr.microseconds() as i32 });
            }

            _ => {
                // Not a datetime property, continue with default handling
            }
        }

        // DEPYLER-0452: Check stdlib API mappings before default fallback
        // Try common CSV patterns (heuristic-based for now)
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "DictReader", attr) {
            // Found a CSV DictReader mapping - apply it
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Also try generic Reader patterns
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "Reader", attr) {
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Default behavior for non-module attributes
        // DEPYLER-0306 FIX: Use raw identifiers for attributes that are Rust keywords
        let attr_ident = if keywords::is_rust_keyword(attr) {
            syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(attr, proc_macro2::Span::call_site())
        };

        // DEPYLER-1138: DepylerValue proxy methods require () for property-like access
        // When accessing .tag, .text, .find, .findall on module alias values (e.g., ET.Element()),
        // these are methods not fields, so we need method call syntax
        // This promotes Python property access (root.tag) to Rust method call (root.tag())
        let depyler_value_properties = ["tag", "text", "find", "findall", "set"];
        if depyler_value_properties.contains(&attr) && !self.ctx.module_aliases.is_empty() {
            return Ok(parse_quote! { #value_expr.#attr_ident() });
        }

        // DEPYLER-0737: Check if this is a @property method access
        // In Python, @property allows method access without (), but Rust requires ()
        if self.ctx.property_methods.contains(attr) {
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            Ok(parse_quote! { #value_expr.#attr_ident })
        }
    }

    pub(crate) fn convert_borrow(&mut self, expr: &HirExpr, mutable: bool) -> Result<syn::Expr> {
        // CITL: Trace borrowing strategy decision
        #[cfg(feature = "decision-tracing")]
        let borrow_type = if mutable { "&mut" } else { "&" };
        trace_decision!(
            category = DecisionCategory::BorrowStrategy,
            name = "explicit_borrow",
            chosen = borrow_type,
            alternatives = ["&ref", "&mut_ref", "move", "clone"],
            confidence = 0.92
        );

        let expr_tokens = expr.to_rust_expr(self.ctx)?;
        if mutable {
            Ok(parse_quote! { &mut #expr_tokens })
        } else {
            Ok(parse_quote! { &#expr_tokens })
        }
    }

    /// DEPYLER-0511: Wrap range expressions in parentheses before method calls
    ///
    /// Ranges need parentheses when followed by method calls due to operator precedence.
    /// Without parens: `0..5.into_iter()` parses as `0..(5.into_iter())` ❌
    /// With parens: `(0..5).into_iter()` parses correctly ✅
    ///
    /// Detects syn::Expr::Range and wraps in syn::Expr::Paren.
    pub(crate) fn wrap_range_in_parens(&self, expr: syn::Expr) -> syn::Expr {
        match &expr {
            syn::Expr::Range(_) => {
                // Wrap range in parentheses
                parse_quote! { (#expr) }
            }
            _ => expr, // No wrapping needed for other expressions
        }
    }


}
