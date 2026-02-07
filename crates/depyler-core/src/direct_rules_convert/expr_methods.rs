//! Method call conversion for ExprConverter
//!
//! Handles Python method calls (obj.method(args)) → Rust equivalents.
//! This is the largest single conversion function.

use crate::direct_rules::{make_ident, parse_target_pattern, safe_class_name};
use crate::hir::*;
use crate::rust_gen::keywords::safe_ident;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_method_call(
        &self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // Handle classmethod cls.method() → Self::method()
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.is_classmethod {
                let method_ident = make_ident(method);
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { Self::#method_ident(#(#arg_exprs),*) });
            }
        }

        // DEPYLER-0610: Handle Python stdlib module constructor calls
        // threading.Semaphore(n) → std::sync::Mutex::new(n)
        // queue.Queue() → std::collections::VecDeque::new()
        if let HirExpr::Var(module_name) = object {
            if let Some(rust_expr) = self.convert_module_constructor(module_name, method, args)? {
                return Ok(rust_expr);
            }
        }

        // DEPYLER-0200: Handle os module method calls in class methods
        // This was missing - os.unlink() etc. weren't being converted inside class methods
        if let HirExpr::Var(module_name) = object {
            if module_name == "os" {
                if let Some(rust_expr) = self.try_convert_os_method(method, args)? {
                    return Ok(rust_expr);
                }
            }
        }

        // DEPYLER-1097: Handle sys module method calls and attribute access
        // sys.exit(code) → std::process::exit(code)
        // sys.argv → std::env::args().collect::<Vec<_>>()
        if let HirExpr::Var(module_name) = object {
            if module_name == "sys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "exit" => {
                        let code = arg_exprs
                            .first()
                            .map(|e| quote::quote!(#e))
                            .unwrap_or_else(|| quote::quote!(0));
                        return Ok(parse_quote! { std::process::exit(#code as i32) });
                    }
                    "argv" => {
                        // sys.argv is an attribute, but might be called as method via wrapper
                        return Ok(parse_quote! { std::env::args().collect::<Vec<String>>() });
                    }
                    "version" | "version_info" => {
                        // Stub: return Rust version string
                        return Ok(parse_quote! { env!("CARGO_PKG_VERSION").to_string() });
                    }
                    "platform" => {
                        return Ok(parse_quote! { std::env::consts::OS.to_string() });
                    }
                    "path" => {
                        // sys.path → empty vec (no Python module system in Rust)
                        return Ok(parse_quote! { Vec::<String>::new() });
                    }
                    "stdin" | "stdout" | "stderr" => {
                        // Return appropriate std::io handle
                        match method {
                            "stdin" => return Ok(parse_quote! { std::io::stdin() }),
                            "stdout" => return Ok(parse_quote! { std::io::stdout() }),
                            "stderr" => return Ok(parse_quote! { std::io::stderr() }),
                            _ => {}
                        }
                    }
                    "getsizeof" if arg_exprs.len() == 1 => {
                        let obj = &arg_exprs[0];
                        return Ok(parse_quote! { std::mem::size_of_val(&#obj) as i32 });
                    }
                    _ => {} // Fall through for unhandled sys methods
                }
            }
        }

        // DEPYLER-1200: Handle re (regex) module method calls in class methods
        // NASA mode: Uses DepylerRegexMatch helper struct (no external crate)
        // Non-NASA mode: Uses regex crate for full regex support
        if let HirExpr::Var(module_name) = object {
            if module_name == "re" {
                // DEPYLER-1200: For regex methods, extract raw string literals where possible
                let extract_str_literal = |hir: &HirExpr| -> Option<String> {
                    if let HirExpr::Literal(Literal::String(s)) = hir {
                        Some(s.clone())
                    } else {
                        None
                    }
                };

                let nasa_mode = self.type_mapper.nasa_mode;

                match method {
                    "search" if args.len() >= 2 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let text_str = extract_str_literal(&args[1]);

                        if let (Some(pattern), Some(text)) = (pattern_str, text_str) {
                            return if nasa_mode {
                                Ok(parse_quote! { DepylerRegexMatch::search(#pattern, #text) })
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(#pattern).expect("invalid regex").find(#text) },
                                )
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let text_expr = self.convert(&args[1])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::search(&#pattern_expr, &#text_expr) },
                                )
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(&#pattern_expr).expect("invalid regex").find(&#text_expr) },
                                )
                            };
                        }
                    }
                    "match" if args.len() >= 2 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let text_str = extract_str_literal(&args[1]);

                        if let (Some(pattern), Some(text)) = (pattern_str, text_str) {
                            return if nasa_mode {
                                Ok(parse_quote! { DepylerRegexMatch::match_start(#pattern, #text) })
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(#pattern).expect("invalid regex").find(#text) },
                                )
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let text_expr = self.convert(&args[1])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::match_start(&#pattern_expr, &#text_expr) },
                                )
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(&#pattern_expr).expect("invalid regex").find(&#text_expr) },
                                )
                            };
                        }
                    }
                    "fullmatch" if args.len() >= 2 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let text_str = extract_str_literal(&args[1]);

                        if let (Some(pattern), Some(text)) = (pattern_str, text_str) {
                            return if nasa_mode {
                                // NASA mode: use DepylerRegexMatch::match_start for simplicity
                                Ok(parse_quote! { DepylerRegexMatch::match_start(#pattern, #text) })
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(&format!("^(?:{})$", #pattern)).expect("invalid regex").find(#text) },
                                )
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let text_expr = self.convert(&args[1])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::match_start(&#pattern_expr, &#text_expr) },
                                )
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(&format!("^(?:{})$", &#pattern_expr)).expect("invalid regex").find(&#text_expr) },
                                )
                            };
                        }
                    }
                    "findall" if args.len() >= 2 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let text_str = extract_str_literal(&args[1]);

                        if let (Some(pattern), Some(text)) = (pattern_str, text_str) {
                            return if nasa_mode {
                                Ok(parse_quote! { DepylerRegexMatch::findall(#pattern, #text) })
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(#pattern)
                                        .expect("invalid regex")
                                        .find_iter(#text)
                                        .map(|m| m.as_str().to_string())
                                        .collect::<Vec<_>>()
                                })
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let text_expr = self.convert(&args[1])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::findall(&#pattern_expr, &#text_expr) },
                                )
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(&#pattern_expr)
                                        .expect("invalid regex")
                                        .find_iter(&#text_expr)
                                        .map(|m| m.as_str().to_string())
                                        .collect::<Vec<_>>()
                                })
                            };
                        }
                    }
                    "finditer" if args.len() >= 2 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let text_str = extract_str_literal(&args[1]);

                        if let (Some(pattern), Some(text)) = (pattern_str, text_str) {
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::findall(#pattern, #text).into_iter() },
                                )
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(#pattern)
                                        .expect("invalid regex")
                                        .find_iter(#text)
                                        .map(|m| m.as_str().to_string())
                                        .collect::<Vec<_>>()
                                })
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let text_expr = self.convert(&args[1])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::findall(&#pattern_expr, &#text_expr).into_iter() },
                                )
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(&#pattern_expr)
                                        .expect("invalid regex")
                                        .find_iter(&#text_expr)
                                        .map(|m| m.as_str().to_string())
                                        .collect::<Vec<_>>()
                                })
                            };
                        }
                    }
                    "sub" if args.len() >= 3 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let repl_str = extract_str_literal(&args[1]);
                        let text_str = extract_str_literal(&args[2]);

                        if let (Some(pattern), Some(repl), Some(text)) =
                            (pattern_str, repl_str, text_str)
                        {
                            return if nasa_mode {
                                Ok(parse_quote! { DepylerRegexMatch::sub(#pattern, #repl, #text) })
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(#pattern)
                                        .expect("invalid regex")
                                        .replace_all(#text, #repl)
                                        .to_string()
                                })
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let repl_expr = self.convert(&args[1])?;
                            let text_expr = self.convert(&args[2])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::sub(&#pattern_expr, &#repl_expr, &#text_expr) },
                                )
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(&#pattern_expr)
                                        .expect("invalid regex")
                                        .replace_all(&#text_expr, &#repl_expr)
                                        .to_string()
                                })
                            };
                        }
                    }
                    "subn" if args.len() >= 3 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let repl_str = extract_str_literal(&args[1]);
                        let text_str = extract_str_literal(&args[2]);

                        if let (Some(pattern), Some(repl), Some(text)) =
                            (pattern_str, repl_str, text_str)
                        {
                            return if nasa_mode {
                                Ok(parse_quote! {
                                    {
                                        let result = DepylerRegexMatch::sub(#pattern, #repl, #text);
                                        let count = (#text).matches(#pattern).count();
                                        (result, count)
                                    }
                                })
                            } else {
                                Ok(parse_quote! {
                                    {
                                        let re = regex::Regex::new(#pattern).expect("invalid regex");
                                        let count = re.find_iter(#text).count();
                                        let result = re.replace_all(#text, #repl).to_string();
                                        (result, count)
                                    }
                                })
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let repl_expr = self.convert(&args[1])?;
                            let text_expr = self.convert(&args[2])?;
                            return if nasa_mode {
                                Ok(parse_quote! {
                                    {
                                        let result = DepylerRegexMatch::sub(&#pattern_expr, &#repl_expr, &#text_expr);
                                        let count = (&#text_expr).matches(&#pattern_expr).count();
                                        (result, count)
                                    }
                                })
                            } else {
                                Ok(parse_quote! {
                                    {
                                        let re = regex::Regex::new(&#pattern_expr).expect("invalid regex");
                                        let count = re.find_iter(&#text_expr).count();
                                        let result = re.replace_all(&#text_expr, &#repl_expr).to_string();
                                        (result, count)
                                    }
                                })
                            };
                        }
                    }
                    "split" if args.len() >= 2 => {
                        let pattern_str = extract_str_literal(&args[0]);
                        let text_str = extract_str_literal(&args[1]);

                        if let (Some(pattern), Some(text)) = (pattern_str, text_str) {
                            return if nasa_mode {
                                Ok(parse_quote! { DepylerRegexMatch::split(#pattern, #text) })
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(#pattern)
                                        .expect("invalid regex")
                                        .split(#text)
                                        .map(|s| s.to_string())
                                        .collect::<Vec<_>>()
                                })
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            let text_expr = self.convert(&args[1])?;
                            return if nasa_mode {
                                Ok(
                                    parse_quote! { DepylerRegexMatch::split(&#pattern_expr, &#text_expr) },
                                )
                            } else {
                                Ok(parse_quote! {
                                    regex::Regex::new(&#pattern_expr)
                                        .expect("invalid regex")
                                        .split(&#text_expr)
                                        .map(|s| s.to_string())
                                        .collect::<Vec<_>>()
                                })
                            };
                        }
                    }
                    "compile" if !args.is_empty() => {
                        let pattern_str = extract_str_literal(&args[0]);

                        if let Some(pattern) = pattern_str {
                            return if nasa_mode {
                                // NASA mode: compile returns the pattern string
                                Ok(parse_quote! { #pattern.to_string() })
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(#pattern).expect("invalid regex") },
                                )
                            };
                        } else {
                            let pattern_expr = self.convert(&args[0])?;
                            return if nasa_mode {
                                Ok(parse_quote! { (#pattern_expr).to_string() })
                            } else {
                                Ok(
                                    parse_quote! { regex::Regex::new(&#pattern_expr).expect("invalid regex") },
                                )
                            };
                        }
                    }
                    "escape" if !args.is_empty() => {
                        let text_str = extract_str_literal(&args[0]);

                        if let Some(text) = text_str {
                            return if nasa_mode {
                                // NASA mode: escape just returns the string as-is
                                Ok(parse_quote! { #text.to_string() })
                            } else {
                                Ok(parse_quote! { regex::escape(#text).to_string() })
                            };
                        } else {
                            let text_expr = self.convert(&args[0])?;
                            return if nasa_mode {
                                Ok(parse_quote! { (#text_expr).to_string() })
                            } else {
                                Ok(parse_quote! { regex::escape(&#text_expr).to_string() })
                            };
                        }
                    }
                    _ => {} // Fall through for unhandled re methods
                }
            }
        }

        // DEPYLER-0912: Handle colorsys module method calls in class methods
        // colorsys.rgb_to_hsv(r, g, b) → inline color conversion
        if let HirExpr::Var(module_name) = object {
            if module_name == "colorsys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "rgb_to_hsv" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let v = max_c;
                                if min_c == max_c { (0.0, 0.0, v) }
                                else {
                                    let s = (max_c - min_c) / max_c;
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c { bc - gc }
                                        else if g == max_c { 2.0 + rc - bc }
                                        else { 4.0 + gc - rc };
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
                        return Ok(parse_quote! {
                            {
                                let (h, s, v) = (#h as f64, #s as f64, #v as f64);
                                if s == 0.0 { (v, v, v) }
                                else {
                                    let i = (h * 6.0).floor();
                                    let f = (h * 6.0) - i;
                                    let p = v * (1.0 - s);
                                    let q = v * (1.0 - s * f);
                                    let t = v * (1.0 - s * (1.0 - f));
                                    let i = i as i32 % 6;
                                    match i { 0 => (v, t, p), 1 => (q, v, p), 2 => (p, v, t),
                                              3 => (p, q, v), 4 => (t, p, v), _ => (v, p, q) }
                                }
                            }
                        });
                    }
                    "rgb_to_hls" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let l = (min_c + max_c) / 2.0;
                                if min_c == max_c { (0.0, l, 0.0) }
                                else {
                                    let s = if l <= 0.5 { (max_c - min_c) / (max_c + min_c) }
                                        else { (max_c - min_c) / (2.0 - max_c - min_c) };
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c { bc - gc }
                                        else if g == max_c { 2.0 + rc - bc }
                                        else { 4.0 + gc - rc };
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
                        return Ok(parse_quote! {
                            {
                                let (h, l, s) = (#h as f64, #l as f64, #s as f64);
                                if s == 0.0 { (l, l, l) }
                                else {
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

        // DEPYLER-1002: Handle base64 module method calls in class methods
        // DEPYLER-1026: NASA mode uses stub implementations instead of base64 crate
        if let HirExpr::Var(module_name) = object {
            if module_name == "base64" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                let nasa_mode = self.type_mapper.nasa_mode;
                match method {
                    "b64encode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Return hex-encoded bytes as stub
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::STANDARD.encode(#data)
                        });
                    }
                    "b64decode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Return input bytes as stub
                            return Ok(parse_quote! {
                                #data.as_bytes().to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::STANDARD.decode(#data).expect("decode failed")
                        });
                    }
                    "urlsafe_b64encode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::URL_SAFE.encode(#data)
                        });
                    }
                    "urlsafe_b64decode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.as_bytes().to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            base64::engine::general_purpose::URL_SAFE.decode(#data).expect("decode failed")
                        });
                    }
                    "b32encode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>().into_bytes()
                            });
                        }
                        return Ok(parse_quote! {
                            data_encoding::BASE32.encode(#data).into_bytes()
                        });
                    }
                    "b32decode" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            return Ok(parse_quote! {
                                #data.to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            data_encoding::BASE32.decode(#data).expect("decode failed")
                        });
                    }
                    "b16encode" | "hexlify" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Can use std format for hex
                            return Ok(parse_quote! {
                                #data.iter().map(|b| format!("{:02x}", b)).collect::<String>().into_bytes()
                            });
                        }
                        return Ok(parse_quote! {
                            hex::encode(#data).into_bytes()
                        });
                    }
                    "b16decode" | "unhexlify" if arg_exprs.len() == 1 => {
                        let data = &arg_exprs[0];
                        if nasa_mode {
                            // NASA mode: Return input as bytes stub
                            return Ok(parse_quote! {
                                #data.to_vec()
                            });
                        }
                        return Ok(parse_quote! {
                            hex::decode(#data).expect("decode failed")
                        });
                    }
                    _ => {} // Fall through for unhandled base64 methods
                }
            }
        }

        // DEPYLER-1002: Handle hashlib module method calls in class methods
        // hashlib.md5() → Md5::new()
        // hashlib.sha256() → Sha256::new()
        if let HirExpr::Var(module_name) = object {
            if module_name == "hashlib" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "md5" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use md5::Digest;
                                    Box::new(md5::Md5::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use md5::Digest;
                                    let mut h = Box::new(md5::Md5::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha1" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha1::Digest;
                                    Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha1::Digest;
                                    let mut h = Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha256" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha512" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "sha384" => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "blake2b" | "blake2s" => {
                        // For blake2, just use sha256 as fallback since blake2 crate API differs
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            let data = &arg_exprs[0];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    "new" => {
                        // hashlib.new("algorithm", data) factory method
                        // For simplicity, default to sha256 since we can't pattern match strings at compile time
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else if arg_exprs.len() == 1 {
                            // Just algorithm name, no data
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                                }
                            });
                        } else {
                            // Algorithm name + data
                            let data = &arg_exprs[1];
                            return Ok(parse_quote! {
                                {
                                    use digest::DynDigest;
                                    use sha2::Digest;
                                    let mut h = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                                    h.update(#data);
                                    h
                                }
                            });
                        }
                    }
                    _ => {} // Fall through for unhandled hashlib methods
                }
            }
        }

        // DEPYLER-1002: Handle json module method calls in class methods
        // DEPYLER-1022: NASA mode uses std-only stubs
        // json.dumps(obj) → serde_json::to_string(&obj).unwrap() (or format! in NASA mode)
        // json.loads(s) → serde_json::from_str(&s).unwrap() (or empty HashMap in NASA mode)
        if let HirExpr::Var(module_name) = object {
            if module_name == "json" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "dumps" if !arg_exprs.is_empty() => {
                        let obj = &arg_exprs[0];
                        // DEPYLER-1022: NASA mode uses format! instead of serde_json
                        if self.type_mapper.nasa_mode {
                            return Ok(parse_quote! { format!("{:?}", #obj) });
                        }
                        return Ok(
                            parse_quote! { serde_json::to_string(&#obj).expect("JSON serialize failed") },
                        );
                    }
                    "loads" if !arg_exprs.is_empty() => {
                        let _s = &arg_exprs[0];
                        // DEPYLER-1022/1051: NASA mode returns empty HashMap stub with DepylerValue
                        if self.type_mapper.nasa_mode {
                            return Ok(
                                parse_quote! { std::collections::HashMap::<String, DepylerValue>::new() },
                            );
                        }
                        return Ok(
                            parse_quote! { serde_json::from_str::<serde_json::Value>(&#_s).expect("JSON parse failed") },
                        );
                    }
                    _ => {} // Fall through
                }
            }
        }

        // DEPYLER-1002: Handle math module method calls in class methods
        // math.sqrt(x) → x.sqrt()
        if let HirExpr::Var(module_name) = object {
            if module_name == "math" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "sqrt" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).sqrt() });
                    }
                    "sin" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).sin() });
                    }
                    "cos" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).cos() });
                    }
                    "tan" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).tan() });
                    }
                    "floor" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).floor() });
                    }
                    "ceil" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).ceil() });
                    }
                    "abs" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).abs() });
                    }
                    "pow" if arg_exprs.len() >= 2 => {
                        let x = &arg_exprs[0];
                        let y = &arg_exprs[1];
                        return Ok(parse_quote! { (#x as f64).powf(#y as f64) });
                    }
                    "log" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        if arg_exprs.len() >= 2 {
                            let base = &arg_exprs[1];
                            return Ok(parse_quote! { (#x as f64).log(#base as f64) });
                        }
                        return Ok(parse_quote! { (#x as f64).ln() });
                    }
                    "exp" if !arg_exprs.is_empty() => {
                        let x = &arg_exprs[0];
                        return Ok(parse_quote! { (#x as f64).exp() });
                    }
                    _ => {} // Fall through
                }
            }
        }

        // DEPYLER-1002: Handle random module method calls in class methods
        // random.randint(a, b) → rand::thread_rng().gen_range(a..=b)
        if let HirExpr::Var(module_name) = object {
            if module_name == "random" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "randint" if arg_exprs.len() >= 2 => {
                        let a = &arg_exprs[0];
                        let b = &arg_exprs[1];
                        return Ok(parse_quote! {
                            {
                                use rand::Rng;
                                rand::thread_rng().gen_range(#a..=#b)
                            }
                        });
                    }
                    "random" if arg_exprs.is_empty() => {
                        return Ok(parse_quote! {
                            {
                                use rand::Rng;
                                rand::thread_rng().gen::<f64>()
                            }
                        });
                    }
                    "choice" if !arg_exprs.is_empty() => {
                        let seq = &arg_exprs[0];
                        return Ok(parse_quote! {
                            {
                                use rand::seq::SliceRandom;
                                #seq.choose(&mut rand::thread_rng()).cloned().expect("empty collection")
                            }
                        });
                    }
                    "shuffle" if !arg_exprs.is_empty() => {
                        let seq = &arg_exprs[0];
                        return Ok(parse_quote! {
                            {
                                use rand::seq::SliceRandom;
                                #seq.shuffle(&mut rand::thread_rng())
                            }
                        });
                    }
                    _ => {} // Fall through
                }
            }
        }

        // DEPYLER-1049: Handle time module method calls in class methods
        // time.time() → std::time::SystemTime::now().duration_since(UNIX_EPOCH).as_secs_f64()
        // time.sleep(s) → std::thread::sleep(Duration::from_secs_f64(s))
        if let HirExpr::Var(module_name) = object {
            if module_name == "time" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "time" if arg_exprs.is_empty() => {
                        return Ok(parse_quote! {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("time error")
                                .as_secs_f64()
                        });
                    }
                    "sleep" if arg_exprs.len() == 1 => {
                        let secs = &arg_exprs[0];
                        return Ok(parse_quote! {
                            std::thread::sleep(std::time::Duration::from_secs_f64(#secs))
                        });
                    }
                    "monotonic" | "perf_counter" if arg_exprs.is_empty() => {
                        return Ok(parse_quote! { std::time::Instant::now() });
                    }
                    "process_time" | "thread_time" if arg_exprs.is_empty() => {
                        // Approximation using Instant
                        return Ok(parse_quote! { std::time::Instant::now() });
                    }
                    "ctime" => {
                        // NASA mode: return formatted string stub
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! {
                                format!("{:?}", std::time::SystemTime::now())
                            });
                        } else {
                            let timestamp = &arg_exprs[0];
                            return Ok(parse_quote! {
                                format!("{:?}", std::time::UNIX_EPOCH + std::time::Duration::from_secs_f64(#timestamp))
                            });
                        }
                    }
                    "gmtime" | "localtime" => {
                        // NASA mode: return SystemTime stub
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! { std::time::SystemTime::now() });
                        } else {
                            let timestamp = &arg_exprs[0];
                            return Ok(parse_quote! {
                                std::time::UNIX_EPOCH + std::time::Duration::from_secs_f64(#timestamp)
                            });
                        }
                    }
                    "mktime" if !arg_exprs.is_empty() => {
                        let _t = &arg_exprs[0];
                        // NASA mode: return current timestamp as stub
                        return Ok(parse_quote! {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("time error")
                                .as_secs_f64()
                        });
                    }
                    _ => {} // Fall through for unhandled time methods
                }
            }
        }

        // DEPYLER-0200: Handle os.path.* and os.environ.* method calls in class methods
        // Pattern: os.path.exists(path), os.environ.get("KEY") etc.
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module_name) = value.as_ref() {
                if module_name == "os" && attr == "path" {
                    if let Some(rust_expr) = self.try_convert_os_path_method(method, args)? {
                        return Ok(rust_expr);
                    }
                }
                if module_name == "os" && attr == "environ" {
                    if let Some(rust_expr) = self.try_convert_os_environ_method(method, args)? {
                        return Ok(rust_expr);
                    }
                }
            }
        }

        // DEPYLER-0932: Handle dict.fromkeys(keys, default) class method
        // dict.fromkeys(keys, default) → keys.iter().map(|k| (k.clone(), default)).collect()
        if let HirExpr::Var(var_name) = object {
            if var_name == "dict" && method == "fromkeys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
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

        // DEPYLER-0933: Handle int.from_bytes(bytes, byteorder) class method in class methods
        // int.from_bytes(bytes, "big") → i64::from_be_bytes(...)
        // int.from_bytes(bytes, "little") → i64::from_le_bytes(...)
        if let HirExpr::Var(var_name) = object {
            if var_name == "int" && method == "from_bytes" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
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

        // Check if this is a static method call on a class (e.g., Counter.create_with_value)
        if let HirExpr::Var(class_name) = object {
            if class_name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                // This is likely a static method call - convert to ClassName::method(args)
                let class_ident = make_ident(class_name);
                let method_ident = make_ident(method);
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) });
            }
        }

        // DEPYLER-1008: Check if this is a mutating method call on self.field
        // If so, we should NOT add .clone() to the object expression
        // Mutating methods: append, push, insert, pop, clear, extend, remove, add, update, etc.
        let is_mutating_method = matches!(
            method,
            "append"
                | "push"
                | "push_back"
                | "push_front"
                | "appendleft"
                | "popleft"
                | "pop"
                | "insert"
                | "remove"
                | "clear"
                | "extend"
                | "add"
                | "update"
                | "discard"
        );

        // Check if object is self.field pattern
        let is_self_field = matches!(
            object,
            HirExpr::Attribute { value, .. } if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
        );

        let object_expr = if is_mutating_method && is_self_field {
            // DEPYLER-1008: For mutating calls on self.field, don't add .clone()
            // Just generate self.field directly
            if let HirExpr::Attribute { value, attr } = object {
                let attr_ident = make_ident(attr);
                let value_expr = self.convert(value)?;
                parse_quote! { #value_expr.#attr_ident }
            } else {
                self.convert(object)?
            }
        } else {
            self.convert(object)?
        };
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        // Map Python collection methods to Rust equivalents
        match method {
            // List/Deque methods
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-1051: Check if target is Vec<DepylerValue> (e.g., untyped class field)
                // If so, wrap the argument in appropriate DepylerValue variant
                // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
                // NOTE: DirectRulesConverter only has param_types and class_field_types,
                // not local var_types - those are handled by expr_gen_instance_methods
                let is_vec_depyler_value = if let HirExpr::Attribute { attr, .. } = object {
                    self.class_field_types
                        .get(attr)
                        .map(|t| matches!(t, Type::List(elem) if matches!(**elem, Type::Unknown | Type::UnificationVar(_))))
                        .unwrap_or(false)
                } else {
                    false
                };

                if is_vec_depyler_value {
                    // Wrap argument in DepylerValue based on argument type
                    let wrapped_arg: syn::Expr = if !args.is_empty() {
                        match &args[0] {
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
                            HirExpr::Var(name) => {
                                // Check parameter type
                                match self.param_types.get(name) {
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
                                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) },
                                }
                            }
                            _ => parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) },
                        }
                    } else {
                        parse_quote! { DepylerValue::Str(format!("{:?}", #arg)) }
                    };
                    return Ok(parse_quote! { #object_expr.push(#wrapped_arg) });
                }

                // DEPYLER-0742: VecDeque uses push_back, Vec uses push
                if self.is_deque_expr(object) {
                    Ok(parse_quote! { #object_expr.push_back(#arg) })
                } else {
                    Ok(parse_quote! { #object_expr.push(#arg) })
                }
            }
            // DEPYLER-0742: Deque-specific methods
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push_front(#arg) })
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.pop_front() })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // Check if it's a list (using position) or set (using remove)
                // For now, assume set behavior since we're working on sets
                // DEPYLER-E0277-FIX: String literals are already &str, other values need &
                if self.is_set_expr(object) {
                    let is_str_lit =
                        matches!(arg, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)));
                    if is_str_lit {
                        Ok(parse_quote! {
                            if !#object_expr.remove(#arg) {
                                panic!("KeyError: element not in set");
                            }
                        })
                    } else {
                        Ok(parse_quote! {
                            if !#object_expr.remove(&#arg) {
                                panic!("KeyError: element not in set");
                            }
                        })
                    }
                } else {
                    // List remove behavior
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#arg) {
                            #object_expr.remove(pos);
                        } else {
                            panic!("ValueError: list.remove(x): x not in list");
                        }
                    })
                }
            }

            // Set methods
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // DEPYLER-E0277-FIX: String literals are already &str, other values need &
                let is_str_lit =
                    matches!(arg, syn::Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Str(_)));
                if is_str_lit {
                    Ok(parse_quote! { #object_expr.remove(#arg) })
                } else {
                    Ok(parse_quote! { #object_expr.remove(&#arg) })
                }
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "pop" => {
                if self.is_set_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments");
                    }
                    // HashSet doesn't have pop(), simulate with iter().next() and remove
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else if self.is_deque_expr(object) {
                    // DEPYLER-0742: VecDeque uses pop_back
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop_back().unwrap_or_default() })
                    } else {
                        bail!("deque.pop() does not accept an index argument");
                    }
                } else {
                    // List pop
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                    } else {
                        let idx = &arg_exprs[0];
                        Ok(parse_quote! { #object_expr.remove(#idx as usize) })
                    }
                }
            }

            // String methods - DEPYLER-0413
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }
            "strip" => {
                if !arg_exprs.is_empty() {
                    bail!("strip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim().to_string() })
            }
            "lstrip" => {
                if !arg_exprs.is_empty() {
                    bail!("lstrip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim_start().to_string() })
            }
            "rstrip" => {
                if !arg_exprs.is_empty() {
                    bail!("rstrip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim_end().to_string() })
            }
            "startswith" => {
                if args.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                // DEPYLER-0602: For starts_with(), use raw string literal for Pattern trait.
                let prefix: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.starts_with(#prefix) })
            }
            "endswith" => {
                if args.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                // DEPYLER-0602: For ends_with(), use raw string literal for Pattern trait.
                let suffix: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.ends_with(#suffix) })
            }
            "split" => {
                if args.is_empty() {
                    Ok(
                        parse_quote! { #object_expr.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if args.len() == 1 {
                    // DEPYLER-0602: For split(), use raw string literal for Pattern trait.
                    let sep: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => self.convert(&args[0])?,
                    };
                    Ok(
                        parse_quote! { #object_expr.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if args.len() == 2 {
                    // DEPYLER-0188: split(sep, maxsplit) -> splitn(maxsplit+1, sep)
                    // Python maxsplit=N means at most N splits → N+1 parts
                    // Rust splitn(n, pat) returns at most n parts
                    let sep: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => self.convert(&args[0])?,
                    };
                    let maxsplit = self.convert(&args[1])?;
                    Ok(
                        parse_quote! { #object_expr.splitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() requires 0-2 arguments");
                }
            }
            "join" => {
                if arg_exprs.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                Ok(parse_quote! { #iterable.join(#object_expr) })
            }
            "replace" => {
                if args.len() != 2 {
                    bail!("replace() requires exactly two arguments");
                }
                // DEPYLER-0602: For replace(), use raw string literals for Pattern trait.
                let old: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                let new: syn::Expr = match &args[1] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[1])?,
                };
                Ok(parse_quote! { #object_expr.replace(#old, #new) })
            }
            "find" => {
                if args.len() != 1 {
                    bail!("find() requires exactly one argument");
                }
                // DEPYLER-0602: For find(), use raw string literal for Pattern trait.
                // String doesn't implement Pattern, but &str does.
                let substring: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.find(#substring).map(|i| i as i64).unwrap_or(-1) })
            }
            "rfind" => {
                if args.len() != 1 {
                    bail!("rfind() requires exactly one argument");
                }
                // DEPYLER-0602: For rfind(), use raw string literal for Pattern trait.
                let substring: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.rfind(#substring).map(|i| i as i64).unwrap_or(-1) })
            }
            "isdigit" => {
                if !arg_exprs.is_empty() {
                    bail!("isdigit() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_ascii_digit()) },
                )
            }
            "isalpha" => {
                if !arg_exprs.is_empty() {
                    bail!("isalpha() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_alphabetic()) },
                )
            }
            "isalnum" => {
                if !arg_exprs.is_empty() {
                    bail!("isalnum() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_alphanumeric()) },
                )
            }

            // DEPYLER-0200/DEPYLER-0960: String/Dict contains method
            // String: use .contains() with raw string literal for Pattern trait
            // Dict/HashMap: use .contains_key() - E0599 fix
            "__contains__" | "contains" => {
                if args.len() != 1 {
                    bail!("contains() requires exactly one argument");
                }

                // DEPYLER-0960: Detect if object is a dict/HashMap type
                let is_dict_like = match object {
                    HirExpr::Var(name) => {
                        let n = name.as_str();
                        n.contains("dict")
                            || n.contains("map")
                            || n.contains("data")
                            || n == "result"
                            || n == "config"
                            || n == "settings"
                            || n == "params"
                            || n == "options"
                            || n == "env"
                            || n == "d"
                            || n == "m"
                            || n == "cache"
                    }
                    HirExpr::Call { func, .. } => {
                        func.contains("dict")
                            || func.contains("json")
                            || func.contains("config")
                            || func.contains("result")
                            || func.contains("load")
                    }
                    _ => false,
                };

                if is_dict_like {
                    // HashMap uses contains_key(&key)
                    let key = self.convert(&args[0])?;
                    Ok(parse_quote! { #object_expr.contains_key(&#key) })
                } else {
                    // String uses .contains(pattern) with Pattern trait
                    let pattern: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String and &str
                            let arg = self.convert(&args[0])?;
                            parse_quote! { &*#arg }
                        }
                    };
                    Ok(parse_quote! { #object_expr.contains(#pattern) })
                }
            }

            // DEPYLER-0613: Semaphore/Mutex method mappings
            // Python: sem.acquire() → Rust: mutex.lock().unwrap() (returns guard)
            "acquire" => {
                // Mutex.lock() returns a guard - acquire returns bool in Python but we adapt
                Ok(parse_quote! { #object_expr.lock().is_ok() })
            }
            // Python: sem.release() → Rust: drop guard (no-op if guard not held)
            "release" => {
                // In Rust, release happens when guard is dropped
                // For now, just return unit since we can't easily track the guard
                Ok(parse_quote! { () })
            }

            // DEPYLER-0613: List/Dict copy method
            // Python: list.copy() → Rust: vec.clone()
            "copy" => {
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clone() })
            }

            // DEPYLER-0613: Dict contains_key (may be called on wrong type)
            // Python: dict.__contains__(key) sometimes transpiles as contains_key
            "contains_key" => {
                if arg_exprs.len() != 1 {
                    bail!("contains_key() requires exactly one argument");
                }
                let key = &arg_exprs[0];
                // For HashMap this is correct, for Vec use contains
                Ok(parse_quote! { #object_expr.contains(&#key) })
            }

            // DEPYLER-1125: Dict get(key, default) - Python's dict.get with 2 args
            // Python: d.get(key, default) returns value at key or default
            // Rust: HashMap doesn't have 2-arg get, use .get(&key).cloned().unwrap_or(default)
            "get" => {
                if arg_exprs.len() == 1 {
                    // Single arg: dict.get(key) → Option<&V>
                    let key = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.get(&#key).cloned() })
                } else if arg_exprs.len() == 2 {
                    // Two args: dict.get(key, default) → V
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    // GH-226: Check if default is a string literal - needs .to_string()
                    // to match the cloned String type from HashMap<K, String>
                    let is_str_literal = matches!(&args[1], HirExpr::Literal(Literal::String(_)));
                    if is_str_literal {
                        Ok(
                            parse_quote! { #object_expr.get(&#key).cloned().unwrap_or_else(|| (#default).to_string()) },
                        )
                    } else {
                        Ok(
                            parse_quote! { #object_expr.get(&#key).cloned().unwrap_or_else(|| #default) },
                        )
                    }
                } else {
                    bail!("get() requires 1 or 2 arguments");
                }
            }

            // Generic method call fallback
            _ => {
                // DEPYLER-0596: Validate method name before creating identifier
                // Method names must be valid Rust identifiers (no empty, no special chars)
                if method.is_empty() {
                    bail!("Empty method name in method call");
                }
                // Check if method is a valid identifier (starts with letter/underscore, alphanumeric)
                let is_valid_ident = method
                    .starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                    && method
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_');
                if !is_valid_ident {
                    bail!(
                        "Invalid method name '{}' - not a valid Rust identifier",
                        method
                    );
                }

                // DEPYLER-0823: Wrap cast expressions in parentheses before method calls
                // Rust parses `x as i32.method()` as `x as (i32.method())` which is invalid
                // Must be: `(x as i32).method()`
                let safe_object_expr: syn::Expr = if matches!(object_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#object_expr) }
                } else {
                    object_expr.clone()
                };

                // Debug: Check if method is a Rust keyword
                if syn::parse_str::<syn::Ident>(method).is_err() {
                    // Method is a Rust keyword - use raw identifier
                    let method_ident = syn::Ident::new_raw(method, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #safe_object_expr.#method_ident(#(#arg_exprs),*) });
                }
                let method_ident = make_ident(method);
                Ok(parse_quote! { #safe_object_expr.#method_ident(#(#arg_exprs),*) })
            }
        }
    }


}
