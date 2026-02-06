//! Instance method handlers for ExpressionConverter
//!
//! DEPYLER-COVERAGE-95: Extracted from expr_gen.rs to reduce file size
//! and improve testability. Contains collection and instance method handlers.

mod string_methods;
#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use crate::rust_gen::truthiness_helpers::{
    is_collection_generic_base, is_collection_type_name, is_collection_var_name,
    is_option_var_name, is_string_var_name,
};
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

mod list_methods;
mod dict_methods;
mod set_methods;
mod regex_methods;
mod sys_io_methods;
mod comprehensions;
mod constructors;
mod dict_constructors;
mod indexing;
mod slicing;
mod type_helpers;
impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Convert instance method calls (main dispatcher)
    #[inline]
    pub(crate) fn convert_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // DEPYLER-0363: Handle parse_args() → Skip for now, will be replaced with Args::parse()
        // ArgumentParser.parse_args() requires full struct transformation
        // For now, return unit to allow compilation
        if method == "parse_args" {
            // NOTE: Full argparse implementation requires Args::parse() call (tracked in DEPYLER-0363)
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0363: Handle add_argument() → Skip for now, will be accumulated for struct generation
        if method == "add_argument" {
            // NOTE: Accumulate add_argument calls to generate struct fields (tracked in DEPYLER-0363)
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0109: Handle parser.print_help() → Args::command().print_help()
        // Python: parser.print_help() prints help and continues
        // Rust/clap: Args::command().print_help()? with CommandFactory trait
        if method == "print_help" {
            // Generate clap help printing using CommandFactory
            return Ok(parse_quote! {
                {
                    use clap::CommandFactory;
                    Args::command().print_help().expect("print help failed")
                }
            });
        }

        // DEPYLER-0381: Handle sys I/O stream method calls
        // Check if object is a sys I/O stream (sys.stdin(), sys.stdout(), sys.stderr())
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module) = &**value {
                if module == "sys" && matches!(attr.as_str(), "stdin" | "stdout" | "stderr") {
                    return self.convert_sys_io_method(attr, method, arg_exprs);
                }
            }
        }

        // DEPYLER-0432: Handle file I/O .read() method
        // Python: f.read() → Rust: read_to_string() or read_to_end()
        if method == "read" && arg_exprs.is_empty() {
            // f.read() with no arguments → read entire file
            // Need to determine if text or binary mode
            // For now, default to text mode (read_to_string)
            // Note: Track file open mode to distinguish text vs binary
            return Ok(parse_quote! {
                {
                    let mut content = String::new();
                    #object_expr.read_to_string(&mut content)?;
                    content
                }
            });
        }

        // DEPYLER-0558: Handle file I/O .read(size) method for chunked reading
        // Python: chunk = f.read(8192) → reads up to 8192 bytes, returns bytes (empty = EOF)
        // Rust: f.read(&mut buf) → reads into buffer, returns count (0 = EOF)
        if method == "read" && arg_exprs.len() == 1 {
            let size = &arg_exprs[0];
            return Ok(parse_quote! {
                {
                    let mut _read_buf = vec![0u8; #size];
                    let _n = #object_expr.read(&mut _read_buf).unwrap_or(0);
                    _read_buf.truncate(_n);
                    _read_buf
                }
            });
        }

        // DEPYLER-0305: Handle file I/O .readlines() method
        // Python: lines = f.readlines() → Rust: BufReader::new(f).lines().collect()
        if method == "readlines" && arg_exprs.is_empty() {
            self.ctx.needs_bufread = true;
            return Ok(parse_quote! {
                std::io::BufReader::new(#object_expr)
                    .lines()
                    .map(|l| l.unwrap_or_default())
                    .collect::<Vec<_>>()
            });
        }

        // DEPYLER-0305: Handle file I/O .readline() method
        // Python: line = f.readline() → Rust: read one line
        if method == "readline" && arg_exprs.is_empty() {
            self.ctx.needs_bufread = true;
            return Ok(parse_quote! {
                {
                    let mut _line = String::new();
                    std::io::BufReader::new(&mut #object_expr).read_line(&mut _line).unwrap_or(0);
                    _line
                }
            });
        }

        // DEPYLER-0458: Handle file I/O .write() method
        // DEPYLER-0537: Use .unwrap() instead of ? for functions without explicit error handling
        // DEPYLER-0536: Handle Option<String> arguments by unwrapping
        // Python: f.write(string) → Rust: f.write_all(bytes).unwrap()
        if method == "write" && arg_exprs.len() == 1 {
            // DEPYLER-0605: Set needs_io_write flag for Write trait import
            self.ctx.needs_io_write = true;
            let content = &arg_exprs[0];
            // Check if content might be an Option type based on HIR expression
            // If it's a variable that's known to be Option, unwrap it first
            // DEPYLER-0536: Detect Option type for write() content argument
            // Priority: type system > name heuristics (only use heuristics when no type info)
            // DEPYLER-0647: Check option_unwrap_map first - if already unwrapped, not Option
            // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
            let is_option_content = if let HirExpr::Var(var_name) = &hir_args[0] {
                // Check if variable is already unwrapped (inside if-let body)
                let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name)
                    || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
                if is_unwrapped {
                    false // Already unwrapped, not Option
                } else {
                    match self.ctx.var_types.get(var_name) {
                        Some(Type::Optional(_)) => true,
                        Some(_) => false, // Known non-Option type - don't use name heuristic
                        None => {
                            // No type info - fall back to name heuristic
                            var_name == "content"
                                || var_name.ends_with("_content")
                                || var_name.ends_with("_text")
                        }
                    }
                }
            } else {
                false
            };

            // Convert string to bytes and use write_all()
            // Python's write() returns bytes written, but we simplify to just the operation
            // Use unwrap() since Python would raise exception on failure (matches behavior)
            if is_option_content {
                return Ok(parse_quote! {
                    #object_expr.write_all(#content.as_ref().expect("value is None").as_bytes()).expect("write failed")
                });
            } else {
                return Ok(parse_quote! {
                    #object_expr.write_all(#content.as_bytes()).expect("write failed")
                });
            }
        }

        // DEPYLER-0529: Handle file .close() method
        // Python: f.close() → Rust: no-op (files auto-close on drop via RAII)
        // DEPYLER-0550: Generate () instead of drop() because the file may have been
        // moved into a writer (e.g., csv::Writer::from_writer(output)), and we can't
        // drop a moved value. Rust's RAII handles cleanup automatically.
        if method == "close" && arg_exprs.is_empty() {
            // In Rust, files are automatically closed when dropped
            // No explicit close needed - RAII handles it
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0551: Handle pathlib.Path instance methods
        // Python Path methods that need mapping to Rust std::path/std::fs equivalents
        // Check if object is a path variable (named "path" or known PathBuf type)
        let is_path_object = if let HirExpr::Var(var_name) = object {
            var_name == "path" || var_name.ends_with("_path") || var_name == "p"
        } else {
            false
        };

        if is_path_object {
            match method {
                // path.stat() → std::fs::metadata(&path).unwrap()
                "stat" if arg_exprs.is_empty() => {
                    return Ok(
                        parse_quote! { std::fs::metadata(&#object_expr).expect("operation failed") },
                    );
                }
                // path.absolute() or path.resolve() → path.canonicalize().unwrap()
                "absolute" | "resolve" if arg_exprs.is_empty() => {
                    return Ok(
                        parse_quote! { #object_expr.canonicalize().expect("operation failed").to_string_lossy().to_string() },
                    );
                }
                _ => {} // Fall through to default handling
            }
        }

        // DEPYLER-0553: Handle datetime instance methods
        // Python datetime methods that need mapping to chrono equivalents
        // Check if object is likely a datetime variable
        // DEPYLER-0620: Expanded heuristics to catch common date variable names
        let is_datetime_object = if let HirExpr::Var(var_name) = object {
            var_name == "dt"
                || var_name == "d"  // DEPYLER-0620: Common date variable name
                || var_name == "t"  // DEPYLER-0620: Common time variable name
                || var_name == "datetime"
                || var_name == "date"  // DEPYLER-0620: Common date variable name
                || var_name == "time"  // DEPYLER-0620: Common time variable name
                || var_name.ends_with("_dt")
                || var_name.ends_with("_datetime")
                || var_name.ends_with("_date")
                || var_name.ends_with("_time")
                || var_name.starts_with("date_")  // DEPYLER-0620: date_xyz pattern
                || var_name.starts_with("time_") // DEPYLER-0620: time_xyz pattern
        } else {
            // DEPYLER-0620: Also detect datetime methods being called regardless of variable name
            // If the method is datetime-specific (strftime, isoformat), assume datetime object
            matches!(
                method,
                "strftime" | "isoformat" | "timestamp" | "weekday" | "isoweekday"
            )
        };

        if is_datetime_object {
            // DEPYLER-1025: In NASA mode, use std::time stubs instead of chrono
            let nasa_mode = self.ctx.type_mapper.nasa_mode;
            if !nasa_mode {
                self.ctx.needs_chrono = true;
            }
            match method {
                // dt.isoformat() → format for ISO string representation
                "isoformat" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { format!("{:?}", #object_expr) });
                    } else {
                        return Ok(parse_quote! { #object_expr.to_string() });
                    }
                }
                // dt.strftime(fmt) → format string
                "strftime" if arg_exprs.len() == 1 => {
                    if nasa_mode {
                        return Ok(parse_quote! { format!("{:?}", #object_expr) });
                    } else {
                        // DEPYLER-0555: chrono's format() takes &str, not String
                        let fmt = match hir_args.first() {
                            Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                            _ => arg_exprs[0].clone(),
                        };
                        return Ok(parse_quote! { #object_expr.format(#fmt).to_string() });
                    }
                }
                // dt.timestamp() → Unix timestamp
                "timestamp" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(
                            parse_quote! { #object_expr.duration_since(std::time::UNIX_EPOCH).expect("operation failed").as_secs() as f64 },
                        );
                    } else {
                        return Ok(parse_quote! { #object_expr.and_utc().timestamp() as f64 });
                    }
                }
                // dt.date() → date component
                "date" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { #object_expr });
                    } else {
                        return Ok(parse_quote! { #object_expr.date() });
                    }
                }
                // dt.time() → time component
                "time" if arg_exprs.is_empty() => {
                    if nasa_mode {
                        return Ok(parse_quote! { #object_expr });
                    } else {
                        return Ok(parse_quote! { #object_expr.time() });
                    }
                }
                _ => {} // Fall through to default handling
            }
        }

        // DEPYLER-0548: Handle csv.DictWriter methods
        // Python csv module methods need mapping to Rust csv crate equivalents
        if method == "writeheader" && arg_exprs.is_empty() {
            // writeheader() → no-op in Rust csv crate
            // Headers are typically written automatically or need explicit handling
            // Note: Track fieldnames from DictWriter constructor to write proper header
            return Ok(parse_quote! { () });
        }

        if method == "writerow" && arg_exprs.len() == 1 {
            // writerow(row) → writer.serialize(&row).unwrap()
            // Python's DictWriter.writerow expects a dict
            // Rust's csv::Writer.serialize can handle HashMap
            let row = &arg_exprs[0];
            return Ok(parse_quote! {
                #object_expr.serialize(&#row).expect("operation failed")
            });
        }

        // DEPYLER-0519: Handle regex Match.group() method
        // DEPYLER-0961: Return String instead of &str for type compatibility
        // DEPYLER-1070: Support DepylerRegexMatch in NASA mode
        // Python: match.group(0) or match.group(n)
        // NASA mode: DepylerRegexMatch.group(n) → String
        // Regex crate: match.as_str().to_string() for group(0), or handle numbered groups
        if method == "group" {
            let nasa_mode = self.ctx.type_mapper.nasa_mode;

            if nasa_mode {
                // DEPYLER-1070: DepylerRegexMatch has direct .group(n) method
                if arg_exprs.is_empty() || hir_args.is_empty() {
                    // match.group() → match.group(0)
                    return Ok(parse_quote! { #object_expr.group(0) });
                }
                let idx = &arg_exprs[0];
                return Ok(parse_quote! { #object_expr.group(#idx as usize) });
            }

            // Regex crate mode
            if arg_exprs.is_empty() || hir_args.is_empty() {
                // match.group() with no args defaults to group(0) in Python
                return Ok(parse_quote! { #object_expr.as_str().to_string() });
            }

            // Check if argument is literal 0
            if let HirExpr::Literal(Literal::Int(n)) = &hir_args[0] {
                if *n == 0 {
                    // match.group(0) → match.as_str().to_string()
                    return Ok(parse_quote! { #object_expr.as_str().to_string() });
                } else {
                    // match.group(n) → match.get(n).map(|m| m.as_str().to_string()).unwrap_or_default()
                    let idx = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default()
                    });
                }
            }

            // Non-literal argument - use runtime check
            let idx = &arg_exprs[0];
            return Ok(parse_quote! {
                if #idx == 0 {
                    #object_expr.as_str().to_string()
                } else {
                    #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default()
                }
            });
        }

        // DEPYLER-0413: Handle string methods FIRST before class instance check
        // String methods like upper/lower should be converted even for method parameters
        // that might be typed as class instances (due to how we track types)
        // DEPYLER-0621: Added encode/decode to ensure bytes conversion works on any string
        // DEPYLER-99MODE: Handle "replace" specially - only route to string handler if we have
        // 2+ positional args (str.replace(old, new)). datetime.replace() uses kwargs instead.
        let is_string_replace = method == "replace" && hir_args.len() >= 2;

        if is_string_replace
            || matches!(
                method,
                "upper"
                    | "lower"
                    | "strip"
                    | "lstrip"
                    | "rstrip"
                    | "startswith"
                    | "endswith"
                    | "split"
                    | "splitlines"
                    | "join"
                    | "find"
                    | "rfind"
                    | "rindex"
                    | "isdigit"
                    | "isalpha"
                    | "isalnum"
                    | "title"
                    | "center"
                    | "ljust"
                    | "rjust"
                    | "zfill"
                    | "hex"
                    | "format"
                    | "encode"  // DEPYLER-0621: str.encode() → .as_bytes().to_vec()
                    | "decode" // DEPYLER-0621: bytes.decode() → String::from_utf8_lossy()
            )
        {
            // DEPYLER-1064: Check if object is a DepylerValue variable
            // If so, extract string before calling string method
            let is_depyler_var = if let HirExpr::Var(var_name) = object {
                self.ctx.type_mapper.nasa_mode
                    && self.ctx.var_types.get(var_name).is_some_and(|t| {
                        matches!(t, Type::Unknown)
                            || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                    })
            } else {
                false
            };

            let adjusted_object_expr = if is_depyler_var {
                parse_quote! { #object_expr.to_string() }
            } else {
                object_expr.clone()
            };

            return self.convert_string_method(
                object,
                &adjusted_object_expr,
                method,
                arg_exprs,
                hir_args,
            );
        }

        // DEPYLER-GH207: Handle dict methods on class attributes (self.field.items/keys/values)
        // When object is HirExpr::Attribute { value: self, attr: field_name } and method is a dict method,
        // route to convert_dict_method BEFORE the is_class_instance check.
        // This fixes E0599 "no method named 'items' found for struct 'HashMap'" errors.
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(var_name) = value.as_ref() {
                if var_name == "self" {
                    // Check if this is a dict method
                    let is_dict_method = matches!(
                        method,
                        "items" | "keys" | "values" | "get" | "update" | "setdefault" | "popitem"
                    );
                    // Check if field is dict-like (via class_field_types or heuristic)
                    let field_type_opt = self.ctx.class_field_types.get(attr);
                    let is_dict_field = field_type_opt.map_or_else(
                        || {
                            // Heuristic fallback for common dict field names
                            matches!(
                                attr.as_str(),
                                "config"
                                    | "settings"
                                    | "options"
                                    | "data"
                                    | "metadata"
                                    | "headers"
                                    | "params"
                                    | "kwargs"
                            ) || attr.ends_with("_dict")
                                || attr.ends_with("_map")
                        },
                        |field_type| {
                            matches!(field_type, Type::Dict(_, _))
                                || matches!(field_type, Type::Custom(s) if s == "Dict")
                        },
                    );
                    if is_dict_method && is_dict_field {
                        return self.convert_dict_method(
                            object_expr,
                            object,
                            method,
                            arg_exprs,
                            hir_args,
                        );
                    }
                }
            }
        }

        // DEPYLER-0232 FIX: Check for user-defined class instances
        // User-defined classes can have methods with names like "add" that conflict with
        // built-in collection methods. We must prioritize user-defined methods.
        if self.is_class_instance(object) {
            // DEPYLER-DUNDER-CLASS-FIX: Translate dunder methods to Rust equivalents
            // This handles cases like counter.__next__() → counter.next()
            let method = match method {
                "__next__" => "next",
                "__iter__" => "iter",
                "__len__" => "len",
                "__str__" => "to_string",
                "__repr__" => "fmt",
                "__contains__" => "contains",
                "__hash__" => "hash",
                "__eq__" => "eq",
                "__ne__" => "ne",
                "__lt__" => "lt",
                "__le__" => "le",
                "__gt__" => "gt",
                "__ge__" => "ge",
                _ => method,
            };

            // This is a user-defined class instance - use generic method call
            // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
            let method_ident = if keywords::is_rust_keyword(method) {
                syn::Ident::new_raw(method, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(method, proc_macro2::Span::call_site())
            };

            // DEPYLER-0712: Auto-borrow class instance arguments when calling user-defined methods
            // When calling obj.method(other) where both are class instances,
            // the method signature likely expects &Self, so we borrow the argument.
            // DEPYLER-METHOD-STR-FIX: Also convert string literals to .to_string() for String params
            let processed_args: Vec<syn::Expr> = hir_args
                .iter()
                .zip(arg_exprs.iter())
                .map(|(hir_arg, arg_expr)| {
                    // If argument is also a class instance, borrow it
                    if self.is_class_instance(hir_arg) {
                        parse_quote! { &#arg_expr }
                    } else if let HirExpr::Literal(Literal::String(s)) = hir_arg {
                        // DEPYLER-METHOD-STR-FIX: Convert string literals to owned Strings
                        // Python methods with str param receive literals, Rust expects String
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit.to_string() }
                    } else {
                        arg_expr.clone()
                    }
                })
                .collect();

            return Ok(parse_quote! { #object_expr.#method_ident(#(#processed_args),*) });
        }

        // DEPYLER-0211 FIX: Check object type first for ambiguous methods like update()
        // Both sets and dicts have update(), so we need to disambiguate

        // Check for set-specific context first
        if self.is_set_expr(object) {
            match method {
                "add"
                | "remove"
                | "discard"
                | "update"
                | "intersection_update"
                | "difference_update"
                | "union"
                | "intersection"
                | "difference"
                | "symmetric_difference"
                | "issubset"
                | "issuperset"
                | "isdisjoint" => {
                    return self.convert_set_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                    );
                }
                _ => {}
            }
        }

        // Check for dict-specific context
        if self.is_dict_expr(object) {
            match method {
                "get" | "keys" | "values" | "items" | "update" => {
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    return self.convert_dict_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                    );
                }
                _ => {}
            }
        }

        // DEPYLER-DUNDER-CALL-FIX: Translate Python dunder methods to Rust equivalents at call sites
        // This handles cases like obj.__next__() → obj.next(), obj.__len__() → obj.len()
        // Must be done BEFORE the fallback match to affect the method name used in codegen
        let method = match method {
            "__next__" => "next",
            "__iter__" => "iter",
            "__len__" => "len",
            "__str__" => "to_string",
            "__repr__" => "fmt",
            "__contains__" => "contains",
            "__hash__" => "hash",
            "__eq__" => "eq",
            "__ne__" => "ne",
            "__lt__" => "lt",
            "__le__" => "le",
            "__gt__" => "gt",
            "__ge__" => "ge",
            _ => method,
        };

        // Fallback to method name dispatch
        match method {
            // DEPYLER-0742: Deque-specific methods (must come before list methods)
            // DEPYLER-1165: Auto-box for VecDeque<DepylerValue>
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-1165: Check if deque has DepylerValue element type
                let is_deque_depyler_value = if let HirExpr::Var(var_name) = object {
                    if let Some(Type::Custom(type_str)) = self.ctx.var_types.get(var_name) {
                        type_str.contains("VecDeque") && type_str.contains("DepylerValue")
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_deque_depyler_value && self.ctx.type_mapper.nasa_mode {
                    // Wrap argument in DepylerValue based on argument type
                    let wrapped_arg: syn::Expr = if !hir_args.is_empty() {
                        match &hir_args[0] {
                            HirExpr::Literal(Literal::Int(_)) => {
                                parse_quote! { DepylerValue::Int(#arg as i64) }
                            }
                            HirExpr::Literal(Literal::Float(_)) => {
                                parse_quote! { DepylerValue::Float(#arg as f64) }
                            }
                            HirExpr::Literal(Literal::String(_)) => {
                                parse_quote! { DepylerValue::Str(#arg.to_string()) }
                            }
                            HirExpr::Literal(Literal::Bool(_)) => {
                                parse_quote! { DepylerValue::Bool(#arg) }
                            }
                            HirExpr::Var(name) => match self.ctx.var_types.get(name) {
                                Some(Type::Int) => parse_quote! { DepylerValue::Int(#arg as i64) },
                                Some(Type::Float) => {
                                    parse_quote! { DepylerValue::Float(#arg as f64) }
                                }
                                Some(Type::String) => {
                                    parse_quote! { DepylerValue::Str(#arg.to_string()) }
                                }
                                Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                                _ => parse_quote! { DepylerValue::from(#arg) },
                            },
                            _ => parse_quote! { DepylerValue::from(#arg) },
                        }
                    } else {
                        parse_quote! { DepylerValue::from(#arg) }
                    };
                    self.ctx.needs_depyler_value_enum = true;
                    Ok(parse_quote! { #object_expr.push_front(#wrapped_arg) })
                } else {
                    Ok(parse_quote! { #object_expr.push_front(#arg) })
                }
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                // DEPYLER-1186: Add .expect() to match Python's raise on empty deque
                // Python raises IndexError if deque is empty, so we panic with expect()
                Ok(parse_quote! { #object_expr.pop_front().expect("popleft from empty deque") })
            }
            // DEPYLER-1187: Handle extendleft for deque
            // Python's deque.extendleft(iterable) adds each element to the front
            // The final order is reversed from the input (first element ends up deepest)
            "extendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("extendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // Implement as: for each element in reversed input, push_front
                // Using into_iter().rev() to reverse the input first
                Ok(parse_quote! {
                    for __item in #arg.into_iter().rev() {
                        #object_expr.push_front(__item);
                    }
                })
            }

            // DEPYLER-0742: Handle append/pop for deque vs list
            // DEPYLER-1165: Auto-box for VecDeque<DepylerValue>
            "append" => {
                if self.is_deque_expr(object) {
                    if arg_exprs.len() != 1 {
                        bail!("append() requires exactly one argument");
                    }
                    let arg = &arg_exprs[0];

                    // DEPYLER-1165: Check if deque has DepylerValue element type
                    let is_deque_depyler_value = if let HirExpr::Var(var_name) = object {
                        if let Some(Type::Custom(type_str)) = self.ctx.var_types.get(var_name) {
                            type_str.contains("VecDeque") && type_str.contains("DepylerValue")
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if is_deque_depyler_value && self.ctx.type_mapper.nasa_mode {
                        // Wrap argument in DepylerValue based on argument type
                        let wrapped_arg: syn::Expr = if !hir_args.is_empty() {
                            match &hir_args[0] {
                                HirExpr::Literal(Literal::Int(_)) => {
                                    parse_quote! { DepylerValue::Int(#arg as i64) }
                                }
                                HirExpr::Literal(Literal::Float(_)) => {
                                    parse_quote! { DepylerValue::Float(#arg as f64) }
                                }
                                HirExpr::Literal(Literal::String(_)) => {
                                    parse_quote! { DepylerValue::Str(#arg.to_string()) }
                                }
                                HirExpr::Literal(Literal::Bool(_)) => {
                                    parse_quote! { DepylerValue::Bool(#arg) }
                                }
                                HirExpr::Var(name) => match self.ctx.var_types.get(name) {
                                    Some(Type::Int) => {
                                        parse_quote! { DepylerValue::Int(#arg as i64) }
                                    }
                                    Some(Type::Float) => {
                                        parse_quote! { DepylerValue::Float(#arg as f64) }
                                    }
                                    Some(Type::String) => {
                                        parse_quote! { DepylerValue::Str(#arg.to_string()) }
                                    }
                                    Some(Type::Bool) => parse_quote! { DepylerValue::Bool(#arg) },
                                    _ => parse_quote! { DepylerValue::from(#arg) },
                                },
                                _ => parse_quote! { DepylerValue::from(#arg) },
                            }
                        } else {
                            parse_quote! { DepylerValue::from(#arg) }
                        };
                        self.ctx.needs_depyler_value_enum = true;
                        Ok(parse_quote! { #object_expr.push_back(#wrapped_arg) })
                    } else {
                        Ok(parse_quote! { #object_expr.push_back(#arg) })
                    }
                } else {
                    self.convert_list_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                        kwargs,
                    )
                }
            }
            "pop" => {
                if self.is_deque_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("deque.pop() does not accept an index argument");
                    }
                    Ok(parse_quote! { #object_expr.pop_back().unwrap_or_default() })
                } else {
                    self.convert_list_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                        kwargs,
                    )
                }
            }

            // List methods (remaining)
            "extend" | "insert" | "remove" | "index" | "copy" | "clear" | "reverse" | "sort" => {
                self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
            }

            // DEPYLER-0226: Disambiguate count() for list vs string
            // DEPYLER-0302: Improved heuristic using is_string_base()
            "count" => {
                // Heuristic: Check if object is string-typed using is_string_base()
                // This covers string literals, variables with str type annotations, and string method results
                if self.is_string_base(object) {
                    // String: use str.count() → .matches().count()
                    self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
                } else {
                    // List: use list.count() → .iter().filter().count()
                    self.convert_list_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                        kwargs,
                    )
                }
            }

            // DEPYLER-0223: Disambiguate update() for dict vs set
            "update" => {
                // Check if argument is a set or dict literal
                if !hir_args.is_empty() && self.is_set_expr(&hir_args[0]) {
                    // numbers.update({3, 4}) - set update
                    self.convert_set_method(object_expr, object, method, arg_exprs, hir_args)
                } else {
                    // data.update({"b": 2}) - dict update (default for variables)
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // DEPYLER-0422: Disambiguate .get() for list vs dict
            // List/Vec .get() takes usize by value, Dict .get() takes &K by reference
            "get" => {
                // Only use list handler when we're CERTAIN it's a list (not dict)
                // Default to dict handler for uncertain types (dict.get() supports 1 or 2 args)
                if self.is_list_expr(object) && !self.is_dict_expr(object) {
                    // List/Vec .get() - cast index to usize (must be exactly 1 arg)
                    if arg_exprs.len() != 1 {
                        bail!("list.get() requires exactly one argument");
                    }
                    let index = &arg_exprs[0];
                    // Cast integer index to usize (Vec/slice .get() requires usize, not &i32)
                    Ok(parse_quote! { #object_expr.get(#index as usize).cloned() })
                } else {
                    // Dict .get() - use existing dict handler (supports 1 or 2 args)
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // Dict methods (for variables without type info)
            "keys" | "values" | "items" | "setdefault" | "popitem" => {
                // DEPYLER-0540: Pass object for serde_json::Value detection
                self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
            }

            // String methods
            // Note: "count" handled separately above with disambiguation logic
            // Note: "index" handled in list methods above (lists take precedence)
            "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "startswith" | "endswith"
            | "split" | "rsplit" | "splitlines" | "join" | "replace" | "find" | "rfind"
            | "rindex" | "isdigit" | "isalpha" | "isalnum" | "isspace" | "isupper" | "islower"
            | "istitle" | "isnumeric" | "isascii" | "isdecimal" | "isidentifier"
            | "isprintable" | "title" | "capitalize" | "swapcase" | "casefold" | "center"
            | "ljust" | "rjust" | "zfill" | "hex" | "encode" | "decode" => {
                self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
            }

            // Set methods (for variables without type info)
            // Note: "update" handled separately above with disambiguation logic
            // Note: "remove" is ambiguous (list vs set) - keep in list fallback for now
            "add"
            | "discard"
            | "intersection_update"
            | "difference_update"
            | "symmetric_difference_update"
            | "union"
            | "intersection"
            | "difference"
            | "symmetric_difference"
            | "issubset"
            | "issuperset"
            | "isdisjoint" => {
                self.convert_set_method(object_expr, object, method, arg_exprs, hir_args)
            }

            // DEPYLER-0431: Regex methods (compiled Regex + Match object)
            // Compiled Regex: findall, match, search (note: "find" conflicts with string.find())
            // Match object: group, groups, start, end, span, as_str
            "findall" | "match" | "search" | "group" | "groups" | "start" | "end" | "span"
            | "as_str" => self.convert_regex_method(object_expr, method, arg_exprs),

            // Path instance methods (DEPYLER-0363)
            "read_text" => {
                // filepath.read_text() → std::fs::read_to_string(filepath).unwrap()
                if !arg_exprs.is_empty() {
                    bail!("Path.read_text() takes no arguments");
                }
                Ok(parse_quote! { std::fs::read_to_string(#object_expr).expect("read failed") })
            }

            // DEPYLER-0960: contains/__contains__ method - dict uses contains_key
            "contains" | "__contains__" => {
                if arg_exprs.len() != 1 {
                    bail!("contains() requires exactly one argument");
                }
                let key = &arg_exprs[0];
                // Check if object is a dict/HashMap - use contains_key
                if self.is_dict_expr(object) {
                    Ok(parse_quote! { #object_expr.contains_key(&#key) })
                } else {
                    // String/Set/List uses .contains()
                    Ok(parse_quote! { #object_expr.contains(&#key) })
                }
            }

            // Default: generic method call
            _ => {
                // DEPYLER-1202: Detect Python-specific methods that need trait bridge
                // These methods don't exist on Rust types, so we inject the traits
                match method {
                    // Python string methods that might not be translated
                    "lower" | "upper" | "strip" | "lstrip" | "rstrip" | "split_py"
                    | "startswith" | "endswith" | "find" | "isalpha" | "isdigit" | "isalnum"
                    | "isspace" | "islower" | "isupper" | "capitalize" | "title" | "swapcase"
                    | "center" | "ljust" | "rjust" | "zfill" => {
                        self.ctx.needs_python_string_ops = true;
                    }
                    // Python int methods
                    "bit_length" | "bit_count" => {
                        self.ctx.needs_python_int_ops = true;
                    }
                    _ => {}
                }

                // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
                let method_ident = if keywords::is_rust_keyword(method) {
                    syn::Ident::new_raw(method, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(method, proc_macro2::Span::call_site())
                };

                // DEPYLER-0712: Auto-borrow class instance arguments when calling user-defined methods
                // When calling obj.method(other) where both obj and other are class instances,
                // the method signature likely expects &Self, so we borrow the argument.
                // Use is_class_instance helper which checks both var_types and class_names.
                let receiver_is_class = self.is_class_instance(object);

                // Process arguments, adding & when receiver and argument are both class instances
                let processed_args: Vec<syn::Expr> = hir_args
                    .iter()
                    .zip(arg_exprs.iter())
                    .map(|(hir_arg, arg_expr)| {
                        // If receiver is a class instance and argument is also a class instance,
                        // the method likely expects &Self for the argument
                        if receiver_is_class && self.is_class_instance(hir_arg) {
                            return parse_quote! { &#arg_expr };
                        }
                        arg_expr.clone()
                    })
                    .collect();

                // DEPYLER-0823: Wrap cast expressions in parentheses before method calls
                // Rust parses `x as i32.method()` as `x as (i32.method())` which is invalid
                // Must be: `(x as i32).method()`
                let safe_object_expr: syn::Expr = if matches!(object_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#object_expr) }
                } else {
                    object_expr.clone()
                };

                Ok(parse_quote! { #safe_object_expr.#method_ident(#(#processed_args),*) })
            }
        }
    }

    /// DEPYLER-0188: Convert dynamic/subscript function call
    /// E.g., `handlers[name](args)` → `(handlers[&name])(args)`
    pub(crate) fn convert_dynamic_call(
        &mut self,
        callee: &HirExpr,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let callee_expr = callee.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        if arg_exprs.is_empty() {
            Ok(parse_quote! { (#callee_expr)() })
        } else {
            Ok(parse_quote! { (#callee_expr)(#(#arg_exprs),*) })
        }
    }

    pub(crate) fn convert_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // CITL: Trace method dispatch decision
        trace_decision!(
            category = DecisionCategory::MethodDispatch,
            name = "method_call",
            chosen = method,
            alternatives = ["trait_method", "inherent_method", "extension", "ufcs"],
            confidence = 0.88
        );

        // DEPYLER-1205: Usage-Based Type Inference
        // If a variable is Unknown/DepylerValue and calls a method that implies a type, infer it.
        // This is "Inference by Usage" - we help the compiler by telling it what type the variable must be.
        if let HirExpr::Var(var_name) = object {
            let current_type = self.ctx.var_types.get(var_name).cloned();
            let is_unknown = matches!(current_type, None | Some(Type::Unknown));

            if is_unknown {
                // List-indicator methods
                match method {
                    // DEPYLER-1211: Recursive Type Propagation for append
                    // If .append(arg) is called, infer the element type from arg
                    "append" => {
                        let element_type = if !args.is_empty() {
                            self.infer_type_from_hir_expr(&args[0])
                        } else {
                            Type::Unknown
                        };
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(element_type.clone())));
                        tracing::debug!(
                            "DEPYLER-1211: Inferred {} as List<{:?}> (via append())",
                            var_name,
                            element_type
                        );
                    }
                    // DEPYLER-1211: For insert(idx, arg), infer element type from second arg
                    "insert" => {
                        let element_type = if args.len() >= 2 {
                            self.infer_type_from_hir_expr(&args[1])
                        } else {
                            Type::Unknown
                        };
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(element_type.clone())));
                        tracing::debug!(
                            "DEPYLER-1211: Inferred {} as List<{:?}> (via insert())",
                            var_name,
                            element_type
                        );
                    }
                    // Other list methods - element type remains unknown
                    "extend" | "pop" | "remove" | "sort" | "reverse" | "clear" | "copy"
                    | "index" | "count" => {
                        // This variable must be a list
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(Type::Unknown)));
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as List (via {}())",
                            var_name,
                            method
                        );
                    }
                    // String-indicator methods
                    "lower" | "upper" | "strip" | "lstrip" | "rstrip" | "split" | "join"
                    | "replace" | "startswith" | "endswith" | "find" | "rfind" | "isdigit"
                    | "isalpha" | "isalnum" | "isupper" | "islower" | "title" | "capitalize"
                    | "swapcase" | "center" | "ljust" | "rjust" | "zfill" | "encode" => {
                        // This variable must be a string
                        self.ctx.var_types.insert(var_name.clone(), Type::String);
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as String (via {}())",
                            var_name,
                            method
                        );
                    }
                    // Dict-indicator methods
                    "keys" | "values" | "items" | "get" | "setdefault" | "update" | "popitem" => {
                        // This variable must be a dict
                        self.ctx.var_types.insert(
                            var_name.clone(),
                            Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
                        );
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as Dict (via {}())",
                            var_name,
                            method
                        );
                    }
                    // Iterator-indicator methods (could be list, set, or dict)
                    "iter" => {
                        // Default to list for iter()
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::List(Box::new(Type::Unknown)));
                        tracing::debug!("DEPYLER-1205: Inferred {} as List (via iter())", var_name);
                    }
                    // Set-indicator methods
                    "add"
                    | "discard"
                    | "difference"
                    | "intersection"
                    | "union"
                    | "symmetric_difference"
                    | "issubset"
                    | "issuperset"
                    | "isdisjoint" => {
                        // This variable must be a set
                        self.ctx
                            .var_types
                            .insert(var_name.clone(), Type::Set(Box::new(Type::Unknown)));
                        tracing::debug!(
                            "DEPYLER-1205: Inferred {} as Set (via {}())",
                            var_name,
                            method
                        );
                    }
                    _ => {}
                }
            }
        }

        // DEPYLER-GH208: Handle is_none/is_some on &mut Option<T> parameters
        // When a parameter with Optional type is mutated, it becomes &mut Option<T>.
        // is_none()/is_some() work via auto-deref, but we generate explicit code for clarity.
        // Also handle method calls on the Option's inner value (e.g., datetime methods).
        if let HirExpr::Var(var_name) = object {
            if self.ctx.mut_option_params.contains(var_name) {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match method {
                    "is_none" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.is_none() });
                    }
                    "is_some" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.is_some() });
                    }
                    // For other methods on Option inner value, unwrap first
                    _ => {
                        // Check if this is a method that should be called on the inner value
                        // Common datetime methods that need unwrapping
                        let needs_unwrap = matches!(
                            method,
                            "year"
                                | "month"
                                | "day"
                                | "hour"
                                | "minute"
                                | "second"
                                | "weekday"
                                | "isoweekday"
                                | "timestamp"
                                | "date"
                                | "time"
                                | "replace"
                                | "strftime"
                                | "isoformat"
                        );
                        if needs_unwrap {
                            let method_ident =
                                syn::Ident::new(method, proc_macro2::Span::call_site());
                            let arg_exprs: Vec<syn::Expr> = args
                                .iter()
                                .map(|arg| arg.to_rust_expr(self.ctx))
                                .collect::<Result<Vec<_>>>()?;
                            if arg_exprs.is_empty() {
                                return Ok(
                                    parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident() },
                                );
                            } else {
                                return Ok(
                                    parse_quote! { #var_ident.as_ref().expect("value is None").#method_ident(#(#arg_exprs),*) },
                                );
                            }
                        }
                    }
                }
            }
        }

        // DEPYLER-0964: Handle method calls on &mut Option<HashMap<K, V>> parameters
        // When a parameter is Dict[K,V] with default None, it becomes &mut Option<HashMap>
        // Method calls need to unwrap the Option first:
        // - memo.get(k) → memo.as_ref().unwrap().get(&k)
        // - memo.insert(k, v) → memo.as_mut().unwrap().insert(k, v)
        // - memo.contains_key(k) → memo.as_ref().unwrap().contains_key(&k)
        if let HirExpr::Var(var_name) = object {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match method {
                    "get" => {
                        if args.is_empty() {
                            // dict.get() with no args - shouldn't happen for dict but handle gracefully
                            return Ok(
                                parse_quote! { #var_ident.as_ref().expect("value is None").get() },
                            );
                        }
                        let key_expr = args[0].to_rust_expr(self.ctx)?;
                        // Check if we need default value (2-arg form)
                        if args.len() > 1 {
                            let default_expr = args[1].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().expect("value is None").get(&#key_expr).cloned().unwrap_or(#default_expr)
                            });
                        } else {
                            // Single arg form - return Option<&V>
                            return Ok(parse_quote! {
                                #var_ident.as_ref().expect("value is None").get(&#key_expr).cloned()
                            });
                        }
                    }
                    "contains_key" | "__contains__" => {
                        if !args.is_empty() {
                            let key_expr = args[0].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().expect("value is None").contains_key(&#key_expr)
                            });
                        }
                    }
                    "keys" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").keys() },
                        );
                    }
                    "values" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").values() },
                        );
                    }
                    "items" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").iter() },
                        );
                    }
                    "len" if args.is_empty() => {
                        return Ok(
                            parse_quote! { #var_ident.as_ref().expect("value is None").len() as i32 },
                        );
                    }
                    _ => {} // Fall through to other handlers
                }
            }
        }

        // DEPYLER-0108: Handle is_some/is_none on precomputed argparse Option fields
        // This prevents borrow-after-move when Option field is passed to a function then checked
        if (method == "is_some" || method == "is_none") && args.is_empty() {
            if let HirExpr::Attribute { value, attr } = object {
                if let HirExpr::Var(_) = value.as_ref() {
                    // Check if this field has been precomputed
                    if self.ctx.precomputed_option_fields.contains(attr) {
                        let has_var_name = format!("has_{}", attr);
                        let has_ident =
                            syn::Ident::new(&has_var_name, proc_macro2::Span::call_site());
                        if method == "is_some" {
                            return Ok(parse_quote! { #has_ident });
                        } else {
                            return Ok(parse_quote! { !#has_ident });
                        }
                    }
                }
            }
        }

        // DEPYLER-0931: Handle subprocess.Child methods (.wait(), .kill(), etc.)
        // proc.wait() → proc.as_mut().unwrap().wait().ok().and_then(|s| s.code()).unwrap_or(-1)
        // When proc is Option<Child>, we need to unwrap and extract exit code
        if method == "wait" && args.is_empty() {
            if let HirExpr::Var(var_name) = object {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
                    let is_subprocess_child = matches!(
                        var_type,
                        Type::Custom(s) if s == "std::process::Child" || s == "Child"
                    ) || matches!(
                        var_type,
                        Type::Optional(inner) if matches!(
                            inner.as_ref(),
                            Type::Custom(s) if s == "std::process::Child" || s == "Child"
                        )
                    );
                    if is_subprocess_child {
                        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                        // Handle Option<Child> - unwrap, call wait, extract exit code
                        if matches!(var_type, Type::Optional(_)) {
                            return Ok(parse_quote! {
                                #var_ident.as_mut().expect("value is None").wait().ok().and_then(|s| s.code()).unwrap_or(-1)
                            });
                        } else {
                            return Ok(parse_quote! {
                                #var_ident.wait().ok().and_then(|s| s.code()).unwrap_or(-1)
                            });
                        }
                    }
                }
            }
        }

        // DEPYLER-0663: Handle serde_json::Value method calls
        // serde_json::Value doesn't have direct .len(), .iter(), .is_none(), .is_some() methods
        // We need to convert them to the appropriate serde_json::Value method chains
        // DEPYLER-0969: H₃ Error Cascade Prevention - comprehensive method coverage
        // This prevents E0599 cascades when Type::Unknown maps to serde_json::Value
        if self.is_serde_json_value_expr(object) || self.is_serde_json_value(object) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            match method {
                // value.len() → value.as_array().map(|a| a.len()).unwrap_or_else(|| value.as_object().map(|o| o.len()).unwrap_or(0))
                "len" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.len()).unwrap_or_else(||
                            #object_expr.as_object().map(|o| o.len()).unwrap_or(0)
                        ) as i32
                    });
                }
                // value.iter() → value.as_array().into_iter().flatten()
                "iter" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().into_iter().flatten()
                    });
                }
                // value.is_none() → value.is_null()
                "is_none" if args.is_empty() => {
                    return Ok(parse_quote! { #object_expr.is_null() });
                }
                // value.is_some() → !value.is_null()
                "is_some" if args.is_empty() => {
                    return Ok(parse_quote! { !#object_expr.is_null() });
                }
                // DEPYLER-0969: H₃ - List-like methods for serde_json::Value arrays
                // value.append(x) → value.as_array_mut().unwrap().push(x.into())
                "append" | "push" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.push(serde_json::json!(#arg)))
                    });
                }
                // value.pop() → value.as_array_mut().and_then(|a| a.pop())
                "pop" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().and_then(|a| a.pop()).unwrap_or(serde_json::Value::Null)
                    });
                }
                // value.pop_front/popleft() → value.as_array_mut().and_then(|a| if a.is_empty() { None } else { Some(a.remove(0)) })
                "pop_front" | "popleft" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().and_then(|a| if a.is_empty() { None } else { Some(a.remove(0)) }).unwrap_or(serde_json::Value::Null)
                    });
                }
                // value.push_back(x) → value.as_array_mut().map(|a| a.push(x.into()))
                "push_back" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.push(serde_json::json!(#arg)))
                    });
                }
                // value.push_front(x) → value.as_array_mut().map(|a| a.insert(0, x.into()))
                "push_front" | "appendleft" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.insert(0, serde_json::json!(#arg)))
                    });
                }
                // value.is_empty() → value.as_array().map(|a| a.is_empty()).unwrap_or(true)
                "is_empty" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.is_empty()).unwrap_or_else(||
                            #object_expr.as_object().map(|o| o.is_empty()).unwrap_or(true)
                        )
                    });
                }
                // DEPYLER-0969: H₃ - Dict-like methods for serde_json::Value objects
                // value.get(key) → value.get(key)
                "get" if !args.is_empty() => {
                    let key = &arg_exprs[0];
                    if args.len() > 1 {
                        let default = &arg_exprs[1];
                        return Ok(parse_quote! {
                            #object_expr.get(#key).cloned().unwrap_or(serde_json::json!(#default))
                        });
                    }
                    return Ok(parse_quote! { #object_expr.get(#key).cloned() });
                }
                // value.keys() → value.as_object().into_iter().flat_map(|o| o.keys())
                "keys" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_object().into_iter().flat_map(|o| o.keys().cloned()).collect::<Vec<_>>()
                    });
                }
                // value.values() → value.as_object().into_iter().flat_map(|o| o.values())
                "values" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_object().into_iter().flat_map(|o| o.values().cloned()).collect::<Vec<_>>()
                    });
                }
                // value.items() → value.as_object().into_iter().flat_map(|o| o.iter())
                "items" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_object().into_iter().flat_map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone()))).collect::<Vec<_>>()
                    });
                }
                // value.contains(x) → value.as_array().map(|a| a.contains(&x.into())).unwrap_or(false)
                "contains" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.iter().any(|v| v == &serde_json::json!(#arg))).unwrap_or(false)
                    });
                }
                // value.contains_key(k) → value.as_object().map(|o| o.contains_key(k)).unwrap_or(false)
                "contains_key" | "__contains__" if args.len() == 1 => {
                    let key = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_object().map(|o| o.contains_key(#key)).unwrap_or(false)
                    });
                }
                // value.insert(k, v) → value.as_object_mut().map(|o| o.insert(k, v.into()))
                "insert" if args.len() == 2 => {
                    let key = &arg_exprs[0];
                    let val = &arg_exprs[1];
                    return Ok(parse_quote! {
                        #object_expr.as_object_mut().map(|o| o.insert(#key.to_string(), serde_json::json!(#val)))
                    });
                }
                // value.remove(k) → value.as_object_mut().and_then(|o| o.remove(k))
                "remove" if args.len() == 1 => {
                    let key = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_object_mut().and_then(|o| o.remove(#key))
                    });
                }
                // value.clear() → value.as_array_mut().map(|a| a.clear())
                "clear" if args.is_empty() => {
                    return Ok(parse_quote! {
                        { if let Some(a) = #object_expr.as_array_mut() { a.clear() }
                          else if let Some(o) = #object_expr.as_object_mut() { o.clear() } }
                    });
                }
                // value.copy() / value.clone() → value.clone()
                "copy" | "clone" if args.is_empty() => {
                    return Ok(parse_quote! { #object_expr.clone() });
                }
                // value.extend(other) → merge JSON values
                "extend" if args.len() == 1 => {
                    let other = &arg_exprs[0];
                    return Ok(parse_quote! {
                        { if let (Some(a1), Some(a2)) = (#object_expr.as_array_mut(), #other.as_array()) {
                            a1.extend(a2.iter().cloned());
                        } else if let (Some(o1), Some(o2)) = (#object_expr.as_object_mut(), #other.as_object()) {
                            for (k, v) in o2 { o1.insert(k.clone(), v.clone()); }
                        } }
                    });
                }
                // value.add(x) → for sets, use array push
                "add" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| if !a.iter().any(|v| v == &serde_json::json!(#arg)) { a.push(serde_json::json!(#arg)) })
                    });
                }
                // value.discard(x) → for sets, remove if present
                "discard" if args.len() == 1 => {
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.as_array_mut().map(|a| a.retain(|v| v != &serde_json::json!(#arg)))
                    });
                }
                _ => {} // Fall through to other handlers
            }
        }

        // DEPYLER-0747: Handle asyncio module method calls
        // asyncio.sleep(secs) → tokio::time::sleep(Duration) or std::thread::sleep in NASA mode
        // asyncio.run(coro) → tokio runtime block_on or direct call in NASA mode
        if let HirExpr::Var(module) = object {
            if module == "asyncio" {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_tokio = true;
                }
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "sleep" => {
                        if nasa_mode {
                            // DEPYLER-1024: Use std::thread::sleep in NASA mode
                            if let Some(arg) = arg_exprs.first() {
                                return Ok(parse_quote! {
                                    std::thread::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                                });
                            }
                            return Ok(parse_quote! {
                                std::thread::sleep(std::time::Duration::from_secs(0))
                            });
                        } else {
                            if let Some(arg) = arg_exprs.first() {
                                return Ok(parse_quote! {
                                    tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                                });
                            }
                            return Ok(parse_quote! {
                                tokio::time::sleep(std::time::Duration::from_secs(0))
                            });
                        }
                    }
                    "run" => {
                        if nasa_mode {
                            // DEPYLER-1024: Just call the function directly in NASA mode
                            if let Some(arg) = arg_exprs.first() {
                                return Ok(parse_quote! { #arg });
                            }
                        } else if let Some(arg) = arg_exprs.first() {
                            return Ok(parse_quote! {
                                tokio::runtime::Runtime::new().expect("operation failed").block_on(#arg)
                            });
                        }
                    }
                    _ => {} // Fall through for other asyncio methods
                }
            }
        }

        // DEPYLER-0912: Handle colorsys module method calls
        // colorsys.rgb_to_hsv(r, g, b) → inline HSV conversion
        // colorsys.hsv_to_rgb(h, s, v) → inline RGB conversion
        if let HirExpr::Var(module) = object {
            if module == "colorsys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "rgb_to_hsv" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        // Python colorsys.rgb_to_hsv formula
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let v = max_c;
                                if min_c == max_c {
                                    (0.0, 0.0, v)
                                } else {
                                    let s = (max_c - min_c) / max_c;
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c {
                                        bc - gc
                                    } else if g == max_c {
                                        2.0 + rc - bc
                                    } else {
                                        4.0 + gc - rc
                                    };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, s, v)
                                }
                            }
                        });
                    }
                    "hsv_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let s = &arg_exprs[1];
                        let v = &arg_exprs[2];
                        // Python colorsys.hsv_to_rgb formula
                        return Ok(parse_quote! {
                            {
                                let (h, s, v) = (#h as f64, #s as f64, #v as f64);
                                if s == 0.0 {
                                    (v, v, v)
                                } else {
                                    let i = (h * 6.0).floor();
                                    let f = (h * 6.0) - i;
                                    let p = v * (1.0 - s);
                                    let q = v * (1.0 - s * f);
                                    let t = v * (1.0 - s * (1.0 - f));
                                    let i = i as i32 % 6;
                                    match i {
                                        0 => (v, t, p),
                                        1 => (q, v, p),
                                        2 => (p, v, t),
                                        3 => (p, q, v),
                                        4 => (t, p, v),
                                        _ => (v, p, q),
                                    }
                                }
                            }
                        });
                    }
                    "rgb_to_hls" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        // Python colorsys.rgb_to_hls formula
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let l = (min_c + max_c) / 2.0;
                                if min_c == max_c {
                                    (0.0, l, 0.0)
                                } else {
                                    let s = if l <= 0.5 {
                                        (max_c - min_c) / (max_c + min_c)
                                    } else {
                                        (max_c - min_c) / (2.0 - max_c - min_c)
                                    };
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c {
                                        bc - gc
                                    } else if g == max_c {
                                        2.0 + rc - bc
                                    } else {
                                        4.0 + gc - rc
                                    };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, l, s)
                                }
                            }
                        });
                    }
                    "hls_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let l = &arg_exprs[1];
                        let s = &arg_exprs[2];
                        // Python colorsys.hls_to_rgb formula
                        return Ok(parse_quote! {
                            {
                                let (h, l, s) = (#h as f64, #l as f64, #s as f64);
                                if s == 0.0 {
                                    (l, l, l)
                                } else {
                                    let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - (l * s) };
                                    let m1 = 2.0 * l - m2;
                                    let _v = |hue: f64| {
                                        let hue = hue % 1.0;
                                        let hue = if hue < 0.0 { hue + 1.0 } else { hue };
                                        if hue < 1.0/6.0 { m1 + (m2 - m1) * hue * 6.0 }
                                        else if hue < 0.5 { m2 }
                                        else if hue < 2.0/3.0 { m1 + (m2 - m1) * (2.0/3.0 - hue) * 6.0 }
                                        else { m1 }
                                    };
                                    (_v(h + 1.0/3.0), _v(h), _v(h - 1.0/3.0))
                                }
                            }
                        });
                    }
                    _ => {} // Fall through for other colorsys methods
                }
            }
        }

        // DEPYLER-0778: Handle dict.fromkeys(keys, default) class method
        // dict.fromkeys(keys, default) → keys.iter().map(|k| (k.clone(), default)).collect()
        if let HirExpr::Var(var_name) = object {
            if var_name == "dict" && method == "fromkeys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let keys_expr = &arg_exprs[0];
                    let default_expr = &arg_exprs[1];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), #default_expr)).collect()
                    });
                } else if arg_exprs.len() == 1 {
                    // dict.fromkeys(keys) with implicit None default
                    let keys_expr = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), ())).collect()
                    });
                }
            }
        }

        // DEPYLER-0933: Handle int.from_bytes(bytes, byteorder) class method
        // int.from_bytes(bytes, "big") → i64::from_be_bytes(bytes.try_into().unwrap())
        // int.from_bytes(bytes, "little") → i64::from_le_bytes(bytes.try_into().unwrap())
        if let HirExpr::Var(var_name) = object {
            if var_name == "int" && method == "from_bytes" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let bytes_expr = &arg_exprs[0];
                    // Check if second arg is "big" or "little" string literal
                    let is_big_endian = if let HirExpr::Literal(Literal::String(s)) = &args[1] {
                        s == "big"
                    } else {
                        true // Default to big endian
                    };

                    if is_big_endian {
                        return Ok(parse_quote! {
                            i64::from_be_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                let start = 8usize.saturating_sub(bytes.len());
                                arr[start..].copy_from_slice(bytes);
                                arr
                            })
                        });
                    } else {
                        return Ok(parse_quote! {
                            i64::from_le_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                arr[..bytes.len().min(8)].copy_from_slice(&bytes[..bytes.len().min(8)]);
                                arr
                            })
                        });
                    }
                }
            }
        }

        // DEPYLER-0558: Handle hasher methods (hexdigest, update) for incremental hashing
        // DEPYLER-1002: Use finalize_reset() for Box<dyn DynDigest> compatibility
        if method == "hexdigest" {
            self.ctx.needs_hex = true;
            self.ctx.needs_digest = true;
            let object_expr = object.to_rust_expr(self.ctx)?;
            // hexdigest() on hasher → hex::encode(hasher.finalize_reset())
            // finalize_reset() works with Box<dyn DynDigest>
            return Ok(parse_quote! {
                {
                    use digest::DynDigest;
                    hex::encode(#object_expr.finalize_reset())
                }
            });
        }

        // DEPYLER-0750: Handle Counter.most_common(n)
        // counter.most_common(n) → sort HashMap by value descending, take n
        if method == "most_common" {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;

            if let Some(n_arg) = arg_exprs.first() {
                // With n argument: take top n
                return Ok(parse_quote! {
                    {
                        let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                        entries.sort_by(|a, b| b.1.cmp(&a.1));
                        entries.into_iter().take(#n_arg as usize).collect::<Vec<_>>()
                    }
                });
            } else {
                // No argument: return all sorted
                return Ok(parse_quote! {
                    {
                        let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                        entries.sort_by(|a, b| b.1.cmp(&a.1));
                        entries
                    }
                });
            }
        }

        // DEPYLER-0728: hasher.update() handler should NOT intercept dict/set.update()
        // Only apply to hash objects (Sha256, Md5, etc.), not collections
        if method == "update"
            && !args.is_empty()
            && !self.is_dict_expr(object)
            && !self.is_set_expr(object)
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            let data = &arg_exprs[0];
            // DEPYLER-0558: hasher.update(data) needs borrow for DynDigest trait
            // DynDigest::update takes &[u8], so always add borrow
            return Ok(parse_quote! {
                #object_expr.update(&#data)
            });
        }

        // DEPYLER-0413: Handle string methods FIRST before any other checks
        // This ensures string methods like upper/lower are converted even when
        // inside class methods where parameters might be mistyped as class instances
        // DEPYLER-1070: Skip if object is a known stdlib module (re, json, etc.) to allow
        // module method handling later (e.g., re.split() should be regex split, not str.split())
        let is_stdlib_module = if let HirExpr::Var(name) = object {
            matches!(
                name.as_str(),
                "re" | "json"
                    | "math"
                    | "random"
                    | "os"
                    | "sys"
                    | "time"
                    | "datetime"
                    | "pathlib"
                    | "struct"
                    | "statistics"
                    | "fractions"
                    | "decimal"
                    | "collections"
                    | "itertools"
                    | "functools"
                    | "shutil"
                    | "csv"
                    | "base64"
                    | "hashlib"
                    | "subprocess"
                    | "string"
                    | "tempfile"
            )
        } else {
            false
        };

        if !is_stdlib_module
            && matches!(
                method,
                "upper"
                    | "lower"
                    | "strip"
                    | "lstrip"
                    | "rstrip"
                    | "startswith"
                    | "endswith"
                    | "split"
                    | "splitlines"
                    | "join"
                    // DEPYLER-99MODE-FIX: "replace" removed from here - handled earlier with arg count check
                    // to allow datetime.replace(kwargs) to fall through to datetime handler
                    | "find"
                    | "rfind"
                    | "rindex"
                    | "isdigit"
                    | "isalpha"
                    | "isalnum"
                    | "title"
                    | "center"
                    | "ljust"
                    | "rjust"
                    | "zfill"
                    | "hex"
                    | "format"
            )
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_string_method(object, &object_expr, method, &arg_exprs, args);
        }

        // DEPYLER-0829: Handle pathlib methods on Path variables (not just module calls)
        // Python: p = Path("/foo"); p.write_text(content)
        // Rust: PathBuf doesn't have write_text, must use std::fs::write
        // This catches path methods when object is a variable, not the pathlib module
        // DEPYLER-0956: Exclude "os" module - os.mkdir/os.rmdir are os module functions, not Path methods
        let is_os_module = matches!(object, HirExpr::Var(name) if name == "os");
        if !is_os_module
            && matches!(
                method,
                "write_text"
                    | "read_text"
                    | "read_bytes"
                    | "write_bytes"
                    | "exists"
                    | "is_file"
                    | "is_dir"
                    | "mkdir"
                    | "rmdir"
                    | "unlink"
                    | "iterdir"
                    | "glob"
                    | "rglob"
                    | "with_name"
                    | "with_suffix"
                    | "with_stem"
                    | "resolve"
                    | "absolute"
                    | "relative_to"
            )
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_pathlib_instance_method(&object_expr, method, &arg_exprs);
        }

        // DEPYLER-0830: Handle datetime/timedelta instance methods on variables
        // Python: td = datetime.timedelta(seconds=100); td.total_seconds()
        // Rust: TimeDelta.num_seconds() as f64
        // This catches datetime methods when object is a variable, not the datetime module
        if matches!(
            method,
            "total_seconds"
                | "fromisoformat"
                | "isoformat"
                | "strftime"
                | "timestamp"
                | "timetuple"
                | "weekday"
                | "isoweekday"
                | "isocalendar"
                | "replace"
        ) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_datetime_instance_method(&object_expr, method, args, &arg_exprs);
        }

        // DEPYLER-0416: Check if this is a static method call on a class (e.g., Point.origin())
        // Convert to ClassName::method(args)
        // DEPYLER-0458 FIX: Exclude CONST_NAMES (all uppercase) from static method conversion
        // Constants like DEFAULT_CONFIG should use instance methods (.clone()) not static (::copy())
        if let HirExpr::Var(class_name) = object {
            let is_const = class_name.chars().all(|c| c.is_uppercase() || c == '_');
            let starts_uppercase = class_name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            if starts_uppercase && !is_const {
                // DEPYLER-0900: Rename class if it shadows stdlib type (e.g., Box -> PyBox)
                // This is likely a static method call - convert to ClassName::method(args)
                let safe_name = crate::direct_rules::safe_class_name(class_name);
                let class_ident = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) });
            }
        }

        // Try classmethod handling first
        if let Some(result) = self.try_convert_classmethod(object, method, args)? {
            return Ok(result);
        }

        // Try module method handling
        // DEPYLER-0426: Pass kwargs to module method converter
        if let Some(result) = self.try_convert_module_method(object, method, args, kwargs)? {
            return Ok(result);
        }

        // DEPYLER-0200: Handle .decode() on base64 encode calls
        // base64.b64encode() in Rust returns String, not bytes - no decode needed
        if method == "decode" {
            if let HirExpr::MethodCall {
                object: inner_obj,
                method: inner_method,
                ..
            } = object
            {
                if let HirExpr::Var(module) = inner_obj.as_ref() {
                    if module == "base64"
                        && (inner_method.contains("b64encode")
                            || inner_method.contains("urlsafe_b64encode"))
                    {
                        // base64::encode() returns String - just return the object expression
                        return object.to_rust_expr(self.ctx);
                    }
                }
            }
        }

        // DEPYLER-1115: Handle external module function calls with Rust path syntax
        // When calling module functions like requests.get(url), use requests::get(url)
        // This enables phantom binding generation to provide type-safe stubs
        // GH-204: Extended to properly use module mappings for stdlib modules like logging
        if let HirExpr::Var(module_name) = object {
            // GH-204: Handle collections module constructor patterns FIRST
            // Route collections.Counter, collections.deque, collections.defaultdict
            // to proper builtin converter functions instead of generic module::function()
            // This MUST be checked before the generic module mapping handling below
            if module_name == "collections" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                match method {
                    "Counter" => {
                        return crate::rust_gen::collection_constructors::convert_counter_builtin(
                            self.ctx, &arg_exprs,
                        );
                    }
                    "deque" => {
                        return crate::rust_gen::collection_constructors::convert_deque_builtin(
                            self.ctx, &arg_exprs,
                        );
                    }
                    "defaultdict" => {
                        return crate::rust_gen::collection_constructors::convert_defaultdict_builtin(
                            self.ctx,
                            &arg_exprs,
                        );
                    }
                    _ => {} // Fall through to generic handling for other collections methods
                }
            }

            // Check if this is an imported module (not a local variable)
            // Use all_imported_modules which includes external unmapped modules like requests
            if self.ctx.all_imported_modules.contains(module_name) {
                // GH-204: Check if this module has a mapping with a Rust equivalent
                if let Some(mapping) = self.ctx.imported_modules.get(module_name) {
                    // Get the rust path and item mapping
                    let rust_path = &mapping.rust_path;
                    let rust_method = mapping.item_map.get(method);

                    // GH-204: If we have a mapped method and non-empty rust_path
                    if let Some(rust_name) = rust_method {
                        // Check if this is a macro call (ends with !)
                        if rust_name.ends_with('!') {
                            // Macro call - generate macro_name!(args)
                            let macro_name_str = rust_name.trim_end_matches('!');
                            let macro_ident =
                                syn::Ident::new(macro_name_str, proc_macro2::Span::call_site());

                            let arg_exprs: Vec<syn::Expr> = args
                                .iter()
                                .map(|arg| arg.to_rust_expr(self.ctx))
                                .collect::<Result<Vec<_>>>()?;

                            return Ok(parse_quote! { #macro_ident!(#(#arg_exprs),*) });
                        } else if !rust_path.is_empty() {
                            // Regular function call with full path
                            let full_path: syn::Path =
                                syn::parse_str(&format!("{}::{}", rust_path, rust_name))?;

                            let arg_exprs: Vec<syn::Expr> = args
                                .iter()
                                .map(|arg| arg.to_rust_expr(self.ctx))
                                .collect::<Result<Vec<_>>>()?;

                            return Ok(parse_quote! { #full_path(#(#arg_exprs),*) });
                        }
                    }
                }

                // Fallback: Generate module::function() syntax for unmapped modules
                let module_ident = crate::rust_gen::keywords::safe_ident(module_name);
                let method_ident = crate::rust_gen::keywords::safe_ident(method);

                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                return Ok(parse_quote! { #module_ident::#method_ident(#(#arg_exprs),*) });
            }
        }

        // DEPYLER-1064: Handle method calls on DepylerValue variables
        // When calling string methods on a DepylerValue, extract the string first
        let is_depyler_value_var = if let HirExpr::Var(var_name) = object {
            self.ctx.type_mapper.nasa_mode
                && self.ctx.var_types.get(var_name).is_some_and(|t| {
                    matches!(t, Type::Unknown)
                        || matches!(t, Type::Custom(name) if name == "Any" || name == "object")
                })
        } else {
            false
        };

        let is_string_method = matches!(
            method,
            "upper"
                | "lower"
                | "strip"
                | "lstrip"
                | "rstrip"
                | "startswith"
                | "endswith"
                | "split"
                | "rsplit"
                | "splitlines"
                | "join"
                | "replace"
                | "find"
                | "rfind"
                | "rindex"
                | "isdigit"
                | "isalpha"
                | "isalnum"
                | "isspace"
                | "isupper"
                | "islower"
                | "istitle"
                | "title"
                | "capitalize"
                | "swapcase"
                | "casefold"
                | "center"
                | "ljust"
                | "rjust"
                | "zfill"
                | "format"
                | "encode"
                | "decode"
        );

        let object_expr = if is_depyler_value_var && is_string_method {
            // Extract string from DepylerValue before calling string method
            let base_expr = object.to_rust_expr(self.ctx)?;
            parse_quote! { #base_expr.to_string() }
        } else {
            object.to_rust_expr(self.ctx)?
        };

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0445: Pass original args and kwargs separately to convert_instance_method
        // Some methods like sort(key=func) need to preserve keyword argument names
        // For other methods, they can merge kwargs as positional if needed
        self.convert_instance_method(object, &object_expr, method, &arg_exprs, args, kwargs)
    }

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

    pub(crate) fn convert_lambda(
        &mut self,
        params: &[String],
        body: &HirExpr,
    ) -> Result<syn::Expr> {
        // CITL: Trace lambda/closure conversion decision
        trace_decision!(
            category = DecisionCategory::Ownership,
            name = "lambda_closure",
            chosen = "closure",
            alternatives = ["fn_pointer", "closure_move", "closure_ref", "boxed_fn"],
            confidence = 0.87
        );

        // DEPYLER-1202: Variable Capture Pass - Identify captured variables from outer scope
        // Python lambdas freely capture outer scope variables. Rust move closures need
        // to have non-Copy types cloned before capture to avoid use-after-move errors.
        let param_set: std::collections::HashSet<String> = params.iter().cloned().collect();
        let body_vars = crate::rust_gen::var_analysis::collect_vars_in_expr(body);
        let captured_vars: Vec<String> = body_vars
            .into_iter()
            .filter(|v| !param_set.contains(v))
            .collect();

        // DEPYLER-1202: Generate clone statements for non-Copy captured variables
        let mut clone_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut clone_mappings: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for var_name in &captured_vars {
            // Skip 'self' - it's handled differently
            if var_name == "self" {
                continue;
            }

            // DEPYLER-1202 FIX: Only capture variables that EXIST in the outer scope
            // Variables not in ctx.var_types are likely:
            // - Loop variables from comprehensions (x in [x*2 for x in lst])
            // - Builtins (True, False, None)
            // - Function/class names
            // These don't need to be captured since they're not outer scope variables
            let var_type = match self.ctx.var_types.get(var_name) {
                Some(ty) => ty.clone(),
                None => continue, // Not in outer scope - skip
            };

            // Only clone non-Copy types
            if !var_type.is_copy() {
                let safe_var = crate::rust_gen::keywords::safe_ident(var_name);
                let clone_var_name = format!("{}_capture", var_name);
                let clone_var = crate::rust_gen::keywords::safe_ident(&clone_var_name);

                // Generate: let prefix_capture = prefix.clone();
                clone_stmts.push(quote::quote! {
                    let #clone_var = #safe_var.clone();
                });

                clone_mappings.insert(var_name.clone(), clone_var_name);
            }
        }

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
        // DEPYLER-1117: Add type annotations to fix E0282 errors
        // Parameters are typed based on body expression analysis
        let param_tokens: Vec<proc_macro2::TokenStream> = params
            .iter()
            .map(|p| {
                let ident = crate::rust_gen::keywords::safe_ident(p);
                // DEPYLER-1117: Infer type from body usage
                if let Some(ty) = self.infer_lambda_param_type(p, body) {
                    quote::quote! { #ident: #ty }
                } else {
                    quote::quote! { #ident }
                }
            })
            .collect();

        // DEPYLER-1202: Convert body expression with captured variable substitution
        // For captured variables that we cloned, we need to reference the clone
        let body_expr = if clone_mappings.is_empty() {
            body.to_rust_expr(self.ctx)?
        } else {
            // Substitute captured variables with their cloned versions in the body
            let substituted_body = self.substitute_captured_vars(body, &clone_mappings);
            substituted_body.to_rust_expr(self.ctx)?
        };

        // Generate closure
        // DEPYLER-0837: Use `move` closures to match Python's closure semantics
        // Python closures capture variables by reference but extend their lifetime
        // Rust requires `move` when returning closures that capture local variables
        let closure: syn::Expr = if params.is_empty() {
            // No parameters
            parse_quote! { move || #body_expr }
        } else if params.len() == 1 {
            // Single parameter with type annotation
            let param = &param_tokens[0];
            parse_quote! { move |#param| #body_expr }
        } else {
            // Multiple parameters with type annotations
            parse_quote! { move |#(#param_tokens),*| #body_expr }
        };

        // DEPYLER-1202: Wrap closure in a block with clone statements if needed
        if clone_stmts.is_empty() {
            Ok(closure)
        } else {
            Ok(parse_quote! {
                {
                    #(#clone_stmts)*
                    #closure
                }
            })
        }
    }

    /// DEPYLER-1202: Substitute captured variable references with their cloned versions
    /// This creates a modified copy of the body expression with renamed variables
    fn substitute_captured_vars(
        &self,
        expr: &HirExpr,
        mappings: &std::collections::HashMap<String, String>,
    ) -> HirExpr {
        match expr {
            HirExpr::Var(name) => {
                if let Some(new_name) = mappings.get(name) {
                    HirExpr::Var(new_name.clone())
                } else {
                    expr.clone()
                }
            }
            HirExpr::Binary { left, op, right } => HirExpr::Binary {
                left: Box::new(self.substitute_captured_vars(left, mappings)),
                op: *op,
                right: Box::new(self.substitute_captured_vars(right, mappings)),
            },
            HirExpr::Unary { op, operand } => HirExpr::Unary {
                op: *op,
                operand: Box::new(self.substitute_captured_vars(operand, mappings)),
            },
            HirExpr::Call { func, args, kwargs } => HirExpr::Call {
                func: func.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_captured_vars(a, mappings))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.substitute_captured_vars(v, mappings)))
                    .collect(),
            },
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => HirExpr::MethodCall {
                object: Box::new(self.substitute_captured_vars(object, mappings)),
                method: method.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_captured_vars(a, mappings))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.substitute_captured_vars(v, mappings)))
                    .collect(),
            },
            HirExpr::Index { base, index } => HirExpr::Index {
                base: Box::new(self.substitute_captured_vars(base, mappings)),
                index: Box::new(self.substitute_captured_vars(index, mappings)),
            },
            HirExpr::Attribute { value, attr } => HirExpr::Attribute {
                value: Box::new(self.substitute_captured_vars(value, mappings)),
                attr: attr.clone(),
            },
            HirExpr::IfExpr { test, body, orelse } => HirExpr::IfExpr {
                test: Box::new(self.substitute_captured_vars(test, mappings)),
                body: Box::new(self.substitute_captured_vars(body, mappings)),
                orelse: Box::new(self.substitute_captured_vars(orelse, mappings)),
            },
            HirExpr::List(elements) => HirExpr::List(
                elements
                    .iter()
                    .map(|e| self.substitute_captured_vars(e, mappings))
                    .collect(),
            ),
            HirExpr::Tuple(elements) => HirExpr::Tuple(
                elements
                    .iter()
                    .map(|e| self.substitute_captured_vars(e, mappings))
                    .collect(),
            ),
            HirExpr::Set(elements) => HirExpr::Set(
                elements
                    .iter()
                    .map(|e| self.substitute_captured_vars(e, mappings))
                    .collect(),
            ),
            HirExpr::Dict(pairs) => HirExpr::Dict(
                pairs
                    .iter()
                    .map(|(k, v)| {
                        (
                            self.substitute_captured_vars(k, mappings),
                            self.substitute_captured_vars(v, mappings),
                        )
                    })
                    .collect(),
            ),
            // Lambda within lambda - recursively substitute, but inner lambda params shadow
            HirExpr::Lambda { params, body } => {
                // Create new mappings excluding shadowed params
                let mut inner_mappings = mappings.clone();
                for p in params {
                    inner_mappings.remove(p);
                }
                HirExpr::Lambda {
                    params: params.clone(),
                    body: Box::new(self.substitute_captured_vars(body, &inner_mappings)),
                }
            }
            // Other expression types - clone as-is (they don't contain variable references
            // or are handled through other means)
            _ => expr.clone(),
        }
    }


    pub(crate) fn convert_await(&mut self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = value.to_rust_expr(self.ctx)?;
        // DEPYLER-1024: In NASA mode, strip .await since async is converted to sync
        if self.ctx.type_mapper.nasa_mode {
            Ok(value_expr)
        } else {
            Ok(parse_quote! { #value_expr.await })
        }
    }

    pub(crate) fn convert_yield(&mut self, value: &Option<Box<HirExpr>>) -> Result<syn::Expr> {
        if self.ctx.in_generator {
            // Inside Iterator::next() - convert to return Some(value)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { return Some(#value_expr) })
            } else {
                Ok(parse_quote! { return None })
            }
        } else {
            // Outside generator context - keep as yield (placeholder for future)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { yield #value_expr })
            } else {
                Ok(parse_quote! { yield })
            }
        }
    }

    pub(crate) fn convert_fstring(&mut self, parts: &[FStringPart]) -> Result<syn::Expr> {
        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    // DEPYLER-0438/0441/0446: Smart formatting based on expression type
                    // - Collections (Vec, HashMap, HashSet): Use {:?} debug formatting
                    // - Scalars (String, i32, f64, bool): Use {} Display formatting
                    // - Option types: Unwrap with .unwrap_or_default() or display "None"
                    // This matches Python semantics where lists/dicts have their own repr
                    let arg_expr = expr.to_rust_expr(self.ctx)?;

                    // DEPYLER-0446: Check if this is an argparse Option<T> field (should be wrapped to String)
                    let is_argparse_option = match expr.as_ref() {
                        HirExpr::Attribute { value, attr } => {
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Check if this argument is optional (Option<T> type, not boolean)
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                let field_name = arg.rust_field_name();
                                                if field_name != *attr {
                                                    return false;
                                                }

                                                // Argument is NOT an Option if it has action="store_true" or "store_false"
                                                if matches!(
                                                    arg.action.as_deref(),
                                                    Some("store_true") | Some("store_false")
                                                ) {
                                                    return false;
                                                }

                                                // Argument is an Option<T> if: not required AND no default value AND not positional
                                                !arg.is_positional
                                                    && !arg.required.unwrap_or(false)
                                                    && arg.default.is_none()
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0497: Determine if expression needs {:?} Debug formatting
                    // Required for: collections, Result, Option, Vec, and any non-Display type
                    let needs_debug_fmt = match expr.as_ref() {
                        // Case 1: Simple variable (e.g., targets)
                        HirExpr::Var(var_name) => {
                            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                                // Known type: check if it needs Debug formatting
                                // DEPYLER-0712: Added Tuple - tuples don't implement Display
                                matches!(
                                    var_type,
                                    Type::List(_)
                                        | Type::Dict(_, _)
                                        | Type::Set(_)
                                        | Type::Tuple(_)     // DEPYLER-0712: Tuples need {:?}
                                        | Type::Optional(_) // DEPYLER-0497: Options need {:?}
                                )
                            } else {
                                // DEPYLER-0497 WORKAROUND: Unknown type - default to {:?} (defensive)
                                // This is safer because Debug is more universally implemented than Display
                                // Most types implement Debug: Option<T>, Result<T,E>, Vec<T>, primitives
                                // Only a few types need Display: i32, String, etc (which also have Debug)
                                // This prevents E0277 errors for Option/Result/Vec variables
                                true
                            }
                        }
                        // DEPYLER-0497: Function calls that return Result<T> OR Option<T> need {:?}
                        HirExpr::Call { func, .. } => {
                            self.ctx.result_returning_functions.contains(func)
                                || self.ctx.option_returning_functions.contains(func)
                        }
                        // DEPYLER-0519: Method calls that return Vec types need {:?}
                        HirExpr::MethodCall { method, .. } => {
                            let vec_returning_methods = [
                                "groups",
                                "split",
                                "split_whitespace",
                                "splitlines",
                                "findall",
                                "keys",
                                "values",
                                "items",
                            ];
                            vec_returning_methods.contains(&method.as_str())
                        }
                        // Case 2: Attribute access (e.g., args.targets)
                        HirExpr::Attribute { value, attr } => {
                            // Check if this is accessing a field from argparse Args struct
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                // Check if obj_name is the args variable from ArgumentParser
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Look up the field type in argparse arguments
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                // Match field name (normalized from Python argument name)
                                                let field_name = arg.rust_field_name();
                                                if field_name == *attr {
                                                    // Check if this field is a collection type
                                                    // Either explicit type annotation OR inferred from nargs
                                                    let is_vec_from_nargs = matches!(
                                                        arg.nargs.as_deref(),
                                                        Some("+") | Some("*")
                                                    );
                                                    let is_collection_type =
                                                        if let Some(ref arg_type) = arg.arg_type {
                                                            matches!(
                                                                arg_type,
                                                                Type::List(_)
                                                                    | Type::Dict(_, _)
                                                                    | Type::Set(_)
                                                            )
                                                        } else {
                                                            false
                                                        };
                                                    is_vec_from_nargs || is_collection_type
                                                } else {
                                                    false
                                                }
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0446: Wrap argparse Option types to handle Display trait
                    // Only wrap argparse Optional fields, not regular Option variables
                    // DEPYLER-0930: Check if expression is a PathBuf type that needs .display()
                    // PathBuf doesn't implement Display, so we need to call .display() to format it
                    let is_pathbuf = match expr.as_ref() {
                        HirExpr::Var(var_name) => self
                            .ctx
                            .var_types
                            .get(var_name)
                            .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                            .unwrap_or(false),
                        HirExpr::MethodCall { method, .. } => {
                            // Methods that return PathBuf
                            matches!(
                                method.as_str(),
                                "parent" | "with_name" | "with_suffix" | "with_stem" | "join"
                            )
                        }
                        HirExpr::Attribute { value, attr } => {
                            // path.parent returns PathBuf
                            if matches!(attr.as_str(), "parent") {
                                if let HirExpr::Var(var_name) = value.as_ref() {
                                    self.ctx
                                        .var_types
                                        .get(var_name)
                                        .map(|t| {
                                            matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                                        })
                                        .unwrap_or(false)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    let final_arg = if is_argparse_option {
                        // Argparse Option<T> should display as value or "None" string
                        parse_quote! {
                            {
                                match &#arg_expr {
                                    Some(v) => format!("{}", v),
                                    None => "None".to_string(),
                                }
                            }
                        }
                    } else if is_pathbuf {
                        // DEPYLER-0930: PathBuf needs .display() to implement Display
                        parse_quote! { #arg_expr.display() }
                    } else {
                        arg_expr
                    };

                    // DEPYLER-0497: Use {:?} for non-Display types (Result, Vec, collections, Option)
                    // Use {} for Display types (primitives, String, wrapped argparse Options)
                    // DEPYLER-0930: PathBuf with .display() can use {} (Display trait)
                    if is_argparse_option || is_pathbuf {
                        // Argparse Option was wrapped to String, PathBuf has .display(), use {}
                        template.push_str("{}");
                    } else if needs_debug_fmt {
                        // Non-Display types (Vec, Result, Option, collections) need {:?}
                        template.push_str("{:?}");
                    } else {
                        // Regular Display types (i32, String, etc.)
                        template.push_str("{}");
                    }

                    args.push(final_arg);
                }
            }
        }

        // Generate format!() macro call
        if args.is_empty() {
            // No arguments (shouldn't happen but be safe)
            Ok(parse_quote! { #template.to_string() })
        } else {
            // Build the format! call with template and arguments
            Ok(parse_quote! { format!(#template, #(#args),*) })
        }
    }

    pub(crate) fn convert_ifexpr(
        &mut self,
        test: &HirExpr,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0377: Optimize `x if x else default` pattern
        // Python: `args.include if args.include else []` (check if list is non-empty)
        // Rust: Just `args.include` (clap initializes Vec to empty, so redundant check)
        // This pattern is common with argparse + Vec/Option fields
        if test == body {
            // Pattern: `x if x else y` → just use `x` (the condition is redundant)
            // This avoids type errors where Vec/Option can't be used as bool
            return body.to_rust_expr(self.ctx);
        }

        // DEPYLER-1071: Handle Option variable ternary with method call on Option
        // Pattern: `option_var.method() if option_var else default`
        // Python: `m.group(0) if m else None` where m = re.search(...)
        // Rust: `if let Some(ref m_val) = m { Some(m_val.group(0)) } else { None }`
        if let HirExpr::Var(var_name) = test {
            let is_option_var = self.is_option_variable(var_name);
            if is_option_var {
                // Check if body uses this variable in a method call
                if self.body_uses_option_var_method(body, var_name) {
                    return self.generate_option_if_let_expr(var_name, body, orelse);
                }
            }
        }

        let mut test_expr = test.to_rust_expr(self.ctx)?;
        let body_expr = body.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // DEPYLER-0377: Apply Python truthiness conversion to ternary expressions
        // Python: `val if val else default` where val is String/List/Dict/Set/Optional/Int/Float
        // Without conversion: `if val` fails (expected bool, found Vec/String/etc)
        // With conversion: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
        test_expr = Self::apply_truthiness_conversion(test, test_expr, self.ctx);

        // DEPYLER-0544: Detect File vs Stdout type mismatch
        // Python: `open(path, "w") if path else sys.stdout`
        // Rust: Needs Box<dyn Write> to unify File and Stdout types
        let body_is_file = self.is_file_creating_expr(body);
        let orelse_is_stdout = self.is_stdout_expr(orelse);
        let orelse_is_file = self.is_file_creating_expr(orelse);
        let body_is_stdout = self.is_stdout_expr(body);

        if (body_is_file && orelse_is_stdout) || (body_is_stdout && orelse_is_file) {
            // Wrap both sides in Box::new() for trait object unification
            return Ok(parse_quote! {
                if #test_expr { Box::new(#body_expr) as Box<dyn std::io::Write> } else { Box::new(#orelse_expr) }
            });
        }

        // DEPYLER-0927: Type unification for numeric IfExpr branches
        // When body returns float and orelse is integer literal, coerce orelse to float
        // Example: `dot / (norm_a * norm_b) if cond else 0` → `... else 0.0`
        let body_is_float = self.expr_returns_float(body);
        let body_is_f32 = self.expr_returns_f32(body);
        let orelse_is_int_literal = matches!(orelse, HirExpr::Literal(Literal::Int(_)));

        if body_is_float && orelse_is_int_literal {
            if let HirExpr::Literal(Literal::Int(n)) = orelse {
                let coerced_orelse: syn::Expr = if body_is_f32 {
                    let float_val = *n as f32;
                    parse_quote! { #float_val }
                } else {
                    let float_val = *n as f64;
                    parse_quote! { #float_val }
                };
                return Ok(parse_quote! {
                    if #test_expr { #body_expr } else { #coerced_orelse }
                });
            }
        }

        // DEPYLER-1085: Value Lifting for DepylerValue/concrete type mismatches
        // When one branch yields DepylerValue and the other a concrete type,
        // wrap the concrete branch in DepylerValue to unify types
        let body_is_depyler_value = self.expr_returns_depyler_value(body);
        let orelse_is_depyler_value = self.expr_returns_depyler_value(orelse);

        if body_is_depyler_value && !orelse_is_depyler_value {
            // Body is DepylerValue, orelse is concrete - lift orelse
            let lifted_orelse = self.lift_to_depyler_value(orelse, orelse_expr);
            return Ok(parse_quote! {
                if #test_expr { #body_expr } else { #lifted_orelse }
            });
        }

        if !body_is_depyler_value && orelse_is_depyler_value {
            // Orelse is DepylerValue, body is concrete - lift body
            let lifted_body = self.lift_to_depyler_value(body, body_expr);
            return Ok(parse_quote! {
                if #test_expr { #lifted_body } else { #orelse_expr }
            });
        }

        Ok(parse_quote! {
            if #test_expr { #body_expr } else { #orelse_expr }
        })
    }
    /// DEPYLER-1071: Generate `if let Some(ref val) = option_var { body } else { orelse }`
    /// with the option variable replaced by the unwrapped val in the body
    fn generate_option_if_let_expr(
        &mut self,
        var_name: &str,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        let var_ident = keywords::safe_ident(var_name);
        let val_name = format!("{}_val", var_name);
        let val_ident = keywords::safe_ident(&val_name);

        // Create a temporary context with the unwrapped variable name
        // We'll transform the body to use the unwrapped value
        let body_with_substitution = self.substitute_var_in_expr(body, var_name, &val_name);
        let body_expr = body_with_substitution.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // Check if orelse is None - if so, use Option::map pattern
        if matches!(orelse, HirExpr::Literal(Literal::None)) {
            // Pattern: `x.method() if x else None` → `x.map(|x_val| x_val.method())`
            Ok(parse_quote! {
                #var_ident.as_ref().map(|#val_ident| #body_expr)
            })
        } else {
            // Pattern: `x.method() if x else default` → `if let Some(ref x_val) = x { body } else { orelse }`
            Ok(parse_quote! {
                if let Some(ref #val_ident) = #var_ident { #body_expr } else { #orelse_expr }
            })
        }
    }

    /// DEPYLER-1071: Recursively substitute a variable name in an expression
    fn substitute_var_in_expr(&self, expr: &HirExpr, old_name: &str, new_name: &str) -> HirExpr {
        match expr {
            HirExpr::Var(name) if name == old_name => HirExpr::Var(new_name.to_string()),
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => HirExpr::MethodCall {
                object: Box::new(self.substitute_var_in_expr(object, old_name, new_name)),
                method: method.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_var_in_expr(a, old_name, new_name))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            self.substitute_var_in_expr(v, old_name, new_name),
                        )
                    })
                    .collect(),
            },
            HirExpr::Attribute { value, attr } => HirExpr::Attribute {
                value: Box::new(self.substitute_var_in_expr(value, old_name, new_name)),
                attr: attr.clone(),
            },
            HirExpr::Call { func, args, kwargs } => HirExpr::Call {
                func: func.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_var_in_expr(a, old_name, new_name))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            self.substitute_var_in_expr(v, old_name, new_name),
                        )
                    })
                    .collect(),
            },
            // For other expression types, return as-is (could be extended if needed)
            _ => expr.clone(),
        }
    }

    /// Apply Python truthiness conversion to non-boolean conditions
    /// Python: `if val:` where val is String/List/Dict/Set/Optional/Int/Float
    /// Rust: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
    pub(crate) fn apply_truthiness_conversion(
        condition: &HirExpr,
        cond_expr: syn::Expr,
        ctx: &CodeGenContext,
    ) -> syn::Expr {
        // Check if this is a variable reference that needs truthiness conversion
        if let HirExpr::Var(var_name) = condition {
            if let Some(var_type) = ctx.var_types.get(var_name) {
                match var_type {
                    // Already boolean - no conversion needed
                    Type::Bool => return cond_expr,

                    // String/List/Dict/Set - check if empty
                    Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }

                    // Optional - check if Some
                    Type::Optional(_) => {
                        return parse_quote! { #cond_expr.is_some() };
                    }

                    // Numeric types - check if non-zero
                    Type::Int => {
                        return parse_quote! { #cond_expr != 0 };
                    }
                    Type::Float => {
                        return parse_quote! { #cond_expr != 0.0 };
                    }

                    // DEPYLER-1071: Custom types that are collections
                    Type::Custom(type_name) => {
                        if is_collection_type_name(type_name) {
                            return parse_quote! { !#cond_expr.is_empty() };
                        }
                        // Fall through to heuristics
                    }

                    // DEPYLER-1071: Generic types that are collections
                    Type::Generic { base, .. } => {
                        if is_collection_generic_base(base) {
                            return parse_quote! { !#cond_expr.is_empty() };
                        }
                        // Fall through to heuristics
                    }

                    // Unknown - fall through to heuristics
                    Type::Unknown => {}

                    // Other types - fall through to heuristics
                    _ => {}
                }
            }

            // DEPYLER-1071: Heuristic fallback for common string variable names
            if is_string_var_name(var_name) {
                return parse_quote! { !#cond_expr.is_empty() };
            }

            // DEPYLER-1071: Heuristic fallback for common collection variable names
            if is_collection_var_name(var_name) {
                return parse_quote! { !#cond_expr.is_empty() };
            }

            // DEPYLER-1071: Heuristic fallback for common Option variable names
            // This handles regex match results and other optional values
            // Pattern: `if m:` where m is a regex match result (Option<Match>)
            if is_option_var_name(var_name) {
                return parse_quote! { #cond_expr.is_some() };
            }
        }

        // Not a variable or no type info - use as-is
        cond_expr
    }

    pub(crate) fn convert_sort_by_key(
        &mut self,
        iterable: &HirExpr,
        key_params: &[String],
        key_body: &HirExpr,
        reverse_expr: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let iter_expr = iterable.to_rust_expr(self.ctx)?;

        // DEPYLER-0502: Convert reverse_expr to Rust expression (supports variables and expressions)
        // If None, default to false (no reversal)
        let reverse_rust_expr = if let Some(expr) = reverse_expr {
            expr.to_rust_expr(self.ctx)?
        } else {
            parse_quote! { false }
        };

        // DEPYLER-0307: Check if this is an identity function (lambda x: x)
        // If so, use simple .sort() instead of .sort_by_key()
        let is_identity =
            key_params.len() == 1 && matches!(key_body, HirExpr::Var(v) if v == &key_params[0]);

        if is_identity {
            // Identity function: just sort() + conditional reverse()
            return Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort();
                    if #reverse_rust_expr {
                        __sorted_result.reverse();
                    }
                    __sorted_result
                }
            });
        }

        // Non-identity key function: use sort_by_key
        let body_expr = key_body.to_rust_expr(self.ctx)?;

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in sorted key lambda parameters
        let param_pat: syn::Pat = if key_params.len() == 1 {
            let param = crate::rust_gen::keywords::safe_ident(&key_params[0]);
            parse_quote! { #param }
        } else {
            bail!("sorted() key lambda must have exactly one parameter");
        };

        // DEPYLER-0502: Generate code with runtime conditional reverse
        // { let mut result = iterable.clone(); result.sort_by_key(|param| body); if reverse_expr { result.reverse(); } result }
        Ok(parse_quote! {
            {
                let mut __sorted_result = #iter_expr.clone();
                __sorted_result.sort_by_key(|#param_pat| #body_expr);
                if #reverse_rust_expr {
                    __sorted_result.reverse();
                }
                __sorted_result
            }
        })
    }

    pub(crate) fn convert_generator_expression(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Strategy: Simple cases use iterator chains, nested use flat_map

        if generators.is_empty() {
            bail!("Generator expression must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-1077: Pre-register char_iter_vars BEFORE converting element expression
            // This ensures ord(c) knows c is a char when iterating over a string
            let is_string_iter_precheck = if let HirExpr::Var(var_name) = &*gen.iter {
                self.ctx
                    .var_types
                    .get(var_name)
                    .map(|ty| matches!(ty, crate::hir::Type::String))
                    .unwrap_or(false)
            } else {
                false
            };
            if is_string_iter_precheck {
                self.ctx.char_iter_vars.insert(gen.target.clone());
            }

            // Now convert element expression (with char_iter_vars populated)
            let element_expr = element.to_rust_expr(self.ctx)?;

            // DEPYLER-0454: Detect CSV reader variables in generator expressions
            let is_csv_reader = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "reader"
                    || var_name.contains("csv")
                    || var_name.ends_with("_reader")
                    || var_name.starts_with("reader_")
            } else {
                false
            };

            // DEPYLER-0523: Detect file variables for BufReader wrapping
            // Same heuristics as stmt_gen.rs for loop file iteration
            let is_file_iter = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "f"
                    || var_name == "file"
                    || var_name == "input"
                    || var_name == "output"
                    || var_name.ends_with("_file")
                    || var_name.starts_with("file_")
            } else {
                false
            };

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr =
                if !is_csv_reader && !is_file_iter && !matches!(&*gen.iter, HirExpr::Var(_)) {
                    self.wrap_range_in_parens(iter_expr)
                } else {
                    iter_expr
                };

            // DEPYLER-0307 Fix #10: Use .iter().copied() for borrowed collections
            // DEPYLER-0454 Extension: Use .deserialize() for CSV readers
            // DEPYLER-0523: Use BufReader for file iteration
            // When the iterator is a variable (likely a borrowed parameter like &Vec<i32>),
            // use .iter().copied() to get owned values instead of references
            // This prevents type mismatches like `&i32` vs `i32` in generator expressions
            let mut chain: syn::Expr = if is_csv_reader {
                // DEPYLER-0454: CSV reader - use deserialize pattern
                self.ctx.needs_csv = true;
                parse_quote! { #iter_expr.deserialize::<std::collections::HashMap<String, String>>().filter_map(|result| result.ok()) }
            } else if is_file_iter {
                // DEPYLER-0523: File variable - use BufReader for line iteration
                self.ctx.needs_bufread = true;
                parse_quote! { std::io::BufReader::new(#iter_expr).lines().map(|l| l.unwrap_or_default()) }
            } else if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                // DEPYLER-0909: Use .cloned() instead of .copied() for compatibility with non-Copy types
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else if self.is_json_value_iteration(&gen.iter) {
                // DEPYLER-0607: JSON Value iteration in generator expression
                // serde_json::Value doesn't implement IntoIterator, must convert first
                parse_quote! { #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-1077: Check if variable is a string type - strings use .chars()
                let is_string_type = if let HirExpr::Var(var_name) = &*gen.iter {
                    self.ctx
                        .var_types
                        .get(var_name)
                        .map(|ty| matches!(ty, crate::hir::Type::String))
                        .unwrap_or(false)
                } else {
                    false
                };
                if is_string_type {
                    // DEPYLER-1077: String iteration uses .chars() not .iter()
                    // Also register target as a char iteration variable for ord() handling
                    self.ctx.char_iter_vars.insert(gen.target.clone());
                    parse_quote! { #iter_expr.chars() }
                } else {
                    // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                    parse_quote! { #iter_expr.iter().cloned() }
                }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-1079: Check if iterator is a zip() call on reference types
            // zip() on &Vec produces (&T, &T) tuples that need dereferencing for owned returns
            // Pattern: (a, b) for (a, b) in zip(list1, list2) where list1/list2 are &Vec
            let is_zip_call = matches!(&*gen.iter, HirExpr::Call { func, .. } if func == "zip");

            if is_zip_call && gen.target.contains(',') {
                // Parse target pattern to extract tuple variable names
                // Target is like "(a, b)" or "a, b" - strip parens and split
                let target_clean = gen.target.trim_start_matches('(').trim_end_matches(')');
                let vars: Vec<&str> = target_clean.split(',').map(|s| s.trim()).collect();
                if vars.len() == 2 && !vars[0].is_empty() && !vars[1].is_empty() {
                    let a = syn::Ident::new(vars[0], proc_macro2::Span::call_site());
                    let b = syn::Ident::new(vars[1], proc_macro2::Span::call_site());
                    // Add map to clone/dereference tuple elements
                    chain = parse_quote! { #chain.map(|(#a, #b)| (#a.clone(), #b.clone())) };
                }
            }

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820/1074: filter() receives &Item (even after .cloned())
            // Use |&#target_pat| to destructure the reference, getting owned value
            // This allows comparisons like x > 0 to work without type errors
            //
            // DEPYLER-1074: Register target variable's element type so numeric coercion works
            // When iterating over List[float], target x is float, so x > 0 should coerce to x > 0.0
            let element_type = if let HirExpr::Var(iter_var) = &*gen.iter {
                self.ctx.var_types.get(iter_var).and_then(|ty| match ty {
                    crate::hir::Type::List(elem) => Some(elem.as_ref().clone()),
                    crate::hir::Type::Set(elem) => Some(elem.as_ref().clone()),
                    _ => None,
                })
            } else {
                None
            };

            // Temporarily register target variable with element type for condition conversion
            let target_var_name = gen.target.clone();
            if let Some(ref elem_ty) = element_type {
                self.ctx
                    .var_types
                    .insert(target_var_name.clone(), elem_ty.clone());
            }

            // DEPYLER-1076: When function returns impl Iterator, closures need `move`
            // to take ownership of captured local variables (like min_val, factor, etc.)
            let needs_move = self.ctx.returns_impl_iterator;

            // DEPYLER-1081: Check if target is a tuple pattern
            // For tuples like (i, v), using |&(i, v)| causes E0507 for non-Copy elements
            // Instead, use |(i, v)| which receives references without trying to move
            let is_tuple_pattern = gen.target.contains(',');

            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                if is_tuple_pattern {
                    // DEPYLER-1081: Tuple patterns - use |(a, b)| to avoid move out of shared ref
                    // Rust's match ergonomics will handle &(A, B) with |(a, b)| pattern
                    if needs_move {
                        chain = parse_quote! { #chain.filter(move |#target_pat| #cond_expr) };
                    } else {
                        chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
                    }
                } else if needs_move {
                    chain = parse_quote! { #chain.filter(move |&#target_pat| #cond_expr) };
                } else {
                    chain = parse_quote! { #chain.filter(|&#target_pat| #cond_expr) };
                }
            }

            // Clean up: remove the temporary target variable
            if element_type.is_some() {
                self.ctx.var_types.remove(&target_var_name);
            }

            // Add the map transformation
            // DEPYLER-1076: Use move when returning impl Iterator
            if needs_move {
                chain = parse_quote! { #chain.map(move |#target_pat| #element_expr) };
            } else {
                chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };
            }

            return Ok(chain);
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: (x + y for x in range(3) for y in range(3))
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y))

        self.convert_nested_generators(element, generators)
    }

    pub(crate) fn convert_nested_generators(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build the nested expression recursively
        let inner_expr = self.build_nested_chain(element, generators, 1)?;

        // Start the chain with the first generator
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-1076: When function returns impl Iterator, closures need `move`
        let needs_move = self.ctx.returns_impl_iterator;

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if needs_move {
                chain = parse_quote! { #chain.filter(move |#first_pat| #cond_expr) };
            } else {
                chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
            }
        }

        // Use flat_map for the first generator
        // DEPYLER-1076: Use move when returning impl Iterator
        if needs_move {
            chain = parse_quote! { #chain.flat_map(move |#first_pat| #inner_expr) };
        } else {
            chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };
        }

        Ok(chain)
    }

    pub(crate) fn build_nested_chain(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
        depth: usize,
    ) -> Result<syn::Expr> {
        if depth >= generators.len() {
            // Base case: no more generators, return the element expression
            let element_expr = element.to_rust_expr(self.ctx)?;
            return Ok(element_expr);
        }

        let gen = &generators[depth];
        let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
        let target_pat = self.parse_target_pattern(&gen.target)?;

        // Build the inner expression (recursive)
        let inner_expr = self.build_nested_chain(element, generators, depth + 1)?;

        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let iter_expr = self.wrap_range_in_parens(iter_expr);

        // Build the chain for this level
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // DEPYLER-0691: Add filters for this generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        // DEPYLER-1076: Use move when returning impl Iterator (for captured locals)
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if self.ctx.returns_impl_iterator {
                chain = parse_quote! { #chain.filter(move |#target_pat| #cond_expr) };
            } else {
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }
        }

        // Use flat_map for intermediate generators, map for the last
        // Note: These already use `move` for capturing outer loop variables
        if depth < generators.len() - 1 {
            // Intermediate generator: use flat_map
            chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_expr) };
        } else {
            // Last generator: use map
            // DEPYLER-1082: Check if element is just the target variable (identity pattern)
            // In this case, use .copied() instead of .map(|x| x) to dereference
            // This handles (x for lst in lists for x in lst) where lst is &Vec<i32>
            let is_identity = matches!(element, HirExpr::Var(v) if v == &gen.target);
            if is_identity {
                // DEPYLER-1082: Use .copied() for primitive types to dereference
                // This converts Iterator<Item=&T> to Iterator<Item=T> for Copy types
                chain = parse_quote! { #chain.copied() };
            } else {
                chain = parse_quote! { #chain.map(move |#target_pat| #inner_expr) };
            }
        }

        Ok(chain)
    }

    pub(crate) fn parse_target_pattern(&self, target: &str) -> Result<syn::Pat> {
        // Handle simple variable: x
        // Handle tuple: (x, y)
        if target.starts_with('(') && target.ends_with(')') {
            // Tuple pattern
            let inner = &target[1..target.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            let idents: Vec<syn::Ident> = parts
                .iter()
                .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                .collect();
            Ok(parse_quote! { ( #(#idents),* ) })
        } else {
            // Simple variable
            let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            Ok(parse_quote! { #ident })
        }
    }

    /// DEPYLER-0188: Convert walrus operator (assignment expression)
    /// Python: (x := expr) assigns expr to x and evaluates to expr
    /// Rust: { let x = expr; x } - block expression that assigns and returns
    pub(crate) fn convert_named_expr(
        &mut self,
        target: &str,
        value: &HirExpr,
    ) -> Result<syn::Expr> {
        let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let value_expr = value.to_rust_expr(self.ctx)?;

        // Generate: { let target = value; target }
        // This assigns the value and returns it, matching Python's walrus semantics
        Ok(parse_quote! {
            {
                let #ident = #value_expr;
                #ident
            }
        })
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
