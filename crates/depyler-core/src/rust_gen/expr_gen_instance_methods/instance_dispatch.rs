//! Instance method dispatch for ExpressionConverter
//!
//! Contains convert_instance_method - the main router for Python instance method calls.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
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

}
