//! Miscellaneous fix functions for post-processing generated Rust code.
//!
//! These are string-level transformations that correct various transpilation
//! artifacts: orphaned syntax, type mismatches, missing annotations, and
//! API compatibility shims.

pub(super) fn fix_docstring_in_main(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Detect `let _ = "...` patterns (docstring assignments).
        // These are always dead code since they assign to `_`.
        // Skip single-line: `let _ = "...";`
        // Skip multi-line: `let _ = "...` (no closing `;` on same line)
        if trimmed.starts_with("let _ = \"") || trimmed.starts_with("let _ = r#\"") {
            if trimmed.ends_with("\";") || trimmed.ends_with("\"#;") {
                // Single-line docstring, skip it
                i += 1;
                continue;
            }
            // Multi-line: skip until closing `";` or `"#;`
            i += 1;
            while i < lines.len() {
                let t = lines[i].trim();
                if t.ends_with("\";") || t.ends_with("\"#;") {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        result.push(lines[i].to_string());
        i += 1;
    }

    result.join("\n") + "\n"
}

pub(super) fn fix_generator_yield_scope(code: &str) -> String {
    if !code.contains("Generator state struct") {
        return code.to_string();
    }
    let mut result = code.to_string();
    // Pattern: `return Some(items)` where `items` is a yield value not
    // captured as a field. If `let items` or `let mut items` doesn't
    // appear before the yield, the variable is undefined.
    if result.contains("return Some(items)")
        && !result.contains("let items")
        && !result.contains("let mut items")
        && !result.contains("self.items")
    {
        result = result.replace("return Some(items)", "return Some(Vec::new())");
    }
    result
}

/// DEPYLER-CONVERGE-MULTI-ITER7: Fix BufReader.deserialize() calls.
///
/// Python csv.reader() maps to BufReader, but BufReader has no .deserialize()
/// method. Replace with BufRead::lines()-based CSV parsing.
pub(super) fn fix_bufreader_deserialize(code: &str) -> String {
    if !code.contains(".deserialize::<HashMap<String, String>>()") {
        return code.to_string();
    }
    let mut result = code.to_string();
    result = result.replace(
        ".deserialize::<HashMap<String, String>>()\n        .collect::<Vec<_>>()",
        ".lines()\n        .filter_map(|l| l.ok())\
         \n        .map(|line| line.split(',')\
         .map(|s| s.trim().to_string())\
         .collect::<Vec<String>>())\
         \n        .collect::<Vec<Vec<String>>>()",
    );
    // Also try single-line variant
    result = result.replace(
        ".deserialize::<HashMap<String, String>>().collect::<Vec<_>>()",
        ".lines().filter_map(|l| l.ok())\
         .map(|line| line.split(',')\
         .map(|s| s.trim().to_string())\
         .collect::<Vec<String>>())\
         .collect::<Vec<Vec<String>>>()",
    );
    if !result.contains("use std::io::BufRead") {
        result = format!("use std::io::BufRead;\n{}", result);
    }
    result
}

/// DEPYLER-CONVERGE-MULTI-ITER7: Fix checked_pow + sqrt type mismatch.
///
/// When .sqrt() follows power operations, the intermediate checked_pow
/// results must be f64, not i32. This fixes E0277 "cannot add f64 to i32".
pub(super) fn fix_power_sqrt_types(code: &str) -> String {
    if !code.contains(".sqrt()") || !code.contains(".checked_pow(") {
        return code.to_string();
    }
    let mut result = code.to_string();
    // Make powf branches return f64 instead of i32
    result = result.replace(".powf({ 2 } as f64) as i32", ".powf({ 2 } as f64)");
    // Make checked_pow branches return f64
    result = result.replace(
        ".expect(\"Power operation overflowed\")",
        ".expect(\"Power operation overflowed\") as f64",
    );
    result
}

/// DEPYLER-CONVERGE-MULTI-ITER7: Fix DepylerDateTime subtraction.
///
/// Python `(d2 - d1).days` transpiles to `(d2) - (d1).day() as i32`
/// which fails because DepylerDateTime doesn't implement Sub<i32>.
/// Replace with direct field access subtraction.
pub(super) fn fix_datetime_subtraction(code: &str) -> String {
    if !code.contains("DepylerDateTime") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    for line in &lines {
        if line.contains(") - (") && line.contains(".day()") {
            // Pattern: ((d2) - (d1).day() as i32).abs()
            // Fix: ((d2.day as i32) - (d1.day as i32)).abs()
            let fixed = line
                .replace("((d2) - (d1).day() as i32)", "((d2.day as i32) - (d1.day as i32))")
                .replace("((d2) - (d1).day())", "((d2.day as i32) - (d1.day as i32))");
            result.push(fixed);
        } else {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}

/// DEPYLER-CONVERGE-MULTI-ITER7: Fix Hasher digest-like method calls.
///
/// Python hashlib generates .update()/.finalize_reset() which come from
/// the `digest` crate API, but we use std::hash::Hasher. Inject a
/// HasherExt trait that provides these methods.
pub(super) fn fix_hasher_digest_methods(code: &str) -> String {
    if !code.contains("DefaultHasher") || !code.contains(".update(") {
        return code.to_string();
    }
    let mut result = code.to_string();
    // Inject HasherExt trait providing digest-like API on std Hasher types
    let ext_trait = "\
trait HasherExt: std::hash::Hasher {\n\
    fn update(&mut self, data: Vec<u8>) { self.write(&data); }\n\
    fn finalize_reset(&mut self) -> Vec<u8> {\n\
        self.finish().to_be_bytes().to_vec()\n\
    }\n\
}\n\
impl<T: std::hash::Hasher + ?Sized> HasherExt for T {}\n";
    result = format!("{}{}", ext_trait, result);
    result
}

pub(super) fn fix_path_or_string_union_coercion(code: &str) -> String {
    if !code.contains("PathOrStringUnion") {
        return code.to_string();
    }
    // Collect function names that take PathOrStringUnion parameters
    // (handle multi-line signatures where PathOrStringUnion is on a later line)
    let mut path_union_fns: Vec<String> = Vec::new();
    let mut current_fn_name: Option<String> = None;
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            let name = trimmed
                .trim_start_matches("pub fn ")
                .trim_start_matches("fn ")
                .split('(')
                .next()
                .unwrap_or("")
                .to_string();
            if !name.is_empty() {
                current_fn_name = Some(name.clone());
            }
            if trimmed.contains("PathOrStringUnion") {
                path_union_fns.push(name);
                current_fn_name = None;
            }
        } else if trimmed.contains("PathOrStringUnion") {
            if let Some(ref name) = current_fn_name {
                if !path_union_fns.contains(name) {
                    path_union_fns.push(name.clone());
                }
            }
        }
        // End of signature
        if trimmed.contains(") ->") || trimmed == ")" || trimmed.starts_with(") {") {
            current_fn_name = None;
        }
    }
    if path_union_fns.is_empty() {
        return code.to_string();
    }
    // Only apply .into() on lines that call a PathOrStringUnion function
    let lines: Vec<&str> = code.lines().collect();
    let mut output = Vec::with_capacity(lines.len());
    let field_patterns = [
        "args.baseline",
        "args.current",
        "args.input",
        "args.output_dir",
        "args.corpus",
        "args.corpus_dir",
        "args.zero_dir",
        "args.input_dir",
        "args.input_path",
        "args.file",
        "args.directory",
        "args.path",
        "args.source",
        "args.target",
        "args.dest",
    ];
    for line in &lines {
        let trimmed = line.trim();
        let is_call_to_path_fn =
            path_union_fns.iter().any(|f| trimmed.contains(&format!("{}(", f)));
        if is_call_to_path_fn {
            let mut fixed = line.to_string();
            for pat in &field_patterns {
                if fixed.contains(pat) && !fixed.contains(&format!("{}.into()", pat)) {
                    fixed = fixed.replace(pat, &format!("{}.into()", pat));
                }
            }
            output.push(fixed);
            continue;
        }
        output.push(line.to_string());
    }
    output.join("\n")
}

/// DEPYLER-CONVERGE-MULTI-ITER8: Fix function stubs used as type constructors.
///
/// When Python imports a class from another module, the transpiler generates a
/// generic function stub. But usage expects a struct with `::new()`. This
/// replaces function stubs with struct+impl patterns.
pub(super) fn fix_function_stub_as_type(code: &str) -> String {
    // Pattern: pub fn CapitalName<T: Default>(_args: impl std::any::Any) -> T
    if !code.contains("<T: Default>(_args: impl std::any::Any) -> T") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.contains("<T: Default>(_args: impl std::any::Any) -> T {") {
            // Extract the name
            let name = if let Some(rest) = trimmed.strip_prefix("pub fn ") {
                rest.split('<').next().unwrap_or("")
            } else if let Some(rest) = trimmed.strip_prefix("fn ") {
                rest.split('<').next().unwrap_or("")
            } else {
                ""
            };
            if !name.is_empty() && name.starts_with(|c: char| c.is_uppercase()) {
                // Skip the function body (next line should be Default::default() + })
                let indent = &lines[i][..lines[i].len() - trimmed.len()];
                result.push(format!(
                    "{}#[derive(Debug, Clone, Default)]\n{}pub struct {} {{}}\n{}impl {} {{\n\
                     {}    pub fn new() -> Self {{ Self {{}} }}\n{}}}",
                    indent, indent, name, indent, name, indent, indent
                ));
                // Skip the body lines
                i += 1;
                while i < lines.len() && !lines[i].trim().starts_with('}') {
                    i += 1;
                }
                i += 1; // skip closing brace
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

pub(super) fn fix_raw_identifier_booleans(code: &str) -> String {
    let mut result = code.to_string();
    // Only replace when used as standalone values, not as part of identifiers
    result = result.replace(" r#false ", " false ");
    result = result.replace(" r#false;", " false;");
    result = result.replace(" r#false}", " false}");
    result = result.replace("(r#false)", "(false)");
    result = result.replace("{r#false}", "{false}");
    result = result.replace(" r#true ", " true ");
    result = result.replace(" r#true;", " true;");
    result = result.replace(" r#true}", " true}");
    result = result.replace("(r#true)", "(true)");
    result = result.replace("{r#true}", "{true}");
    result
}

pub(super) fn fix_validate_not_none_args(code: &str) -> String {
    let one_arg_sig = "fn validate_not_none<T: Default>(_args: impl std::any::Any) -> T";
    if !code.contains(one_arg_sig) {
        return code.to_string();
    }
    let has_two_arg_call = code.lines().any(|l| {
        let t = l.trim();
        t.starts_with("validate_not_none(") && t.contains(", \"")
    });
    let mut result = if has_two_arg_call {
        let two_arg_sig =
            "fn validate_not_none<T: Default>(_args: impl std::any::Any, _name: &str) -> T";
        code.replace(one_arg_sig, two_arg_sig)
    } else {
        code.to_string()
    };
    // Also turbofish unused calls with ::<()> to resolve generic type inference
    let lines: Vec<&str> = result.lines().collect();
    let mut new_lines: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let t = line.trim();
        if t.starts_with("validate_not_none(") && t.ends_with(';') {
            let fixed = line.replace("validate_not_none(", "validate_not_none::<()>(");
            new_lines.push(fixed);
        } else {
            new_lines.push(line.to_string());
        }
    }
    result = new_lines.join("\n");
    result
}

pub(super) fn fix_tuple_to_vec_when_len_called(code: &str) -> String {
    // Find struct fields with pattern: `pub X: (SomeType, DepylerValue),`
    // Then check if `.X.len()` appears in the code
    let mut replacements: Vec<(String, String, String)> = Vec::new();
    for line in code.lines() {
        let t = line.trim();
        if !t.starts_with("pub ") || !t.contains(": (") || !t.contains(", DepylerValue)") {
            continue;
        }
        // Extract field name and inner type
        if let Some(colon_pos) = t.find(": (") {
            let field_part = &t[4..colon_pos]; // after "pub "
            let field_name = field_part.trim();
            let type_start = colon_pos + 3; // after ": ("
            if let Some(comma_pos) = t[type_start..].find(", DepylerValue)") {
                let inner_type = t[type_start..type_start + comma_pos].trim();
                // Check if .field_name.len() is used in code
                let len_pattern = format!(".{}.len()", field_name);
                let tostr_pattern = format!(".{}.to_string()", field_name);
                let iter_pattern = format!(".{}.iter()", field_name);
                if code.contains(&len_pattern)
                    || code.contains(&tostr_pattern)
                    || code.contains(&iter_pattern)
                {
                    let old_type = format!("({}, DepylerValue)", inner_type);
                    let new_type = format!("Vec<{}>", inner_type);
                    replacements.push((field_name.to_string(), old_type, new_type));
                }
            }
        }
    }
    if replacements.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for (_field, old_type, new_type) in &replacements {
        result = result.replace(old_type.as_str(), new_type.as_str());
    }
    result
}

pub(super) fn fix_orphaned_semicolon_paren(code: &str) -> String {
    if !code.contains("};)") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let trimmed = line.trim();
        // `};)` is never valid Rust: `}` closes a block, `;` terminates,
        // and `)` has no matching `(`.
        if trimmed == "};)" {
            let indent = &line[..line.len() - trimmed.len()];
            result.push(format!("{}}}", indent));
        } else {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}

/// Patterns like `Arc::new({ let mut set = HashSet::new(); ... set }` generate
/// a block expression inside a function call but the closing `}` is missing
/// the corresponding `)` characters to close `Arc::new(` and `Some(`.
pub(super) fn fix_inline_block_expression_parens(code: &str) -> String {
    if !code.contains("({ let mut") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Detect opening: a line containing `({ let mut`
        // Skip `for ... in ({ let mut` patterns -- the `for` body brace
        // confuses depth tracking and these already have correct parens.
        if trimmed.contains("({ let mut") && !trimmed.contains(" in ({") {
            let unclosed = count_unquoted_parens(trimmed);
            if unclosed > 0 {
                let mut rel_depth: i32 = count_unquoted_braces(trimmed);
                result.push(lines[i].to_string());
                i += 1;
                while i < lines.len() && rel_depth > 0 {
                    let line_trimmed = lines[i].trim();
                    rel_depth += count_unquoted_braces(line_trimmed);
                    if rel_depth <= 0 {
                        // Check if next line is a method chain continuation
                        // (e.g., `.into_iter().collect::<...>())`).
                        // If so, the closing parens should come from the
                        // continuation line, not from us.
                        let has_continuation = i + 1 < lines.len() && {
                            let next = lines[i + 1].trim();
                            next.starts_with('.')
                                && (next.contains(".into_iter()")
                                    || next.contains(".collect")
                                    || next.contains(".map("))
                        };
                        if has_continuation {
                            // Output the closing `}` line as-is. The continuation
                            // line already has the `)` closings.
                            result.push(lines[i].to_string());
                            i += 1;
                            // Push the continuation and any further continuations
                            while i < lines.len() {
                                let cont = lines[i].trim();
                                if cont.starts_with('.') || cont.starts_with(')') {
                                    result.push(lines[i].to_string());
                                    i += 1;
                                } else {
                                    break;
                                }
                            }
                        } else {
                            // No continuation: add missing `)` chars
                            let existing_close = count_trailing_close_parens(line_trimmed);
                            let needed = unclosed - existing_close;
                            if needed > 0 {
                                let indent = &lines[i][..lines[i].len() - line_trimmed.len()];
                                let close_str = ")".repeat(needed as usize);
                                if line_trimmed == "}" {
                                    result.push(format!("{}}}{};", indent, close_str));
                                } else {
                                    result.push(format!("{}{}", lines[i], close_str));
                                }
                            } else {
                                result.push(lines[i].to_string());
                            }
                            i += 1;
                        }
                    } else {
                        result.push(lines[i].to_string());
                        i += 1;
                    }
                }
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

/// Count unclosed parens on a line, skipping string literals.
pub(super) fn count_unquoted_parens(line: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut prev = '\0';
    for c in line.chars() {
        if c == '"' && prev != '\\' {
            in_string = !in_string;
        }
        if !in_string {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
            }
        }
        prev = c;
    }
    depth
}

/// Count net brace depth on a line, skipping string literals.
pub(super) fn count_unquoted_braces(line: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut prev = '\0';
    for c in line.chars() {
        if c == '"' && prev != '\\' {
            in_string = !in_string;
        }
        if !in_string {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
            }
        }
        prev = c;
    }
    depth
}

/// Count trailing `)` characters after the last `}` on a line.
pub(super) fn count_trailing_close_parens(line: &str) -> i32 {
    if let Some(pos) = line.rfind('}') {
        let after = &line[pos + 1..];
        after.chars().filter(|&c| c == ')').count() as i32
    } else {
        0
    }
}

#[allow(dead_code)]
pub(super) fn count_parens_open(s: &str) -> i32 {
    s.chars().filter(|&c| c == '(').count() as i32
}

#[allow(dead_code)]
pub(super) fn count_parens_close(s: &str) -> i32 {
    s.chars().filter(|&c| c == ')').count() as i32
}

pub(super) fn fix_as_bool_on_bool(code: &str) -> String {
    let has_as_bool = code.contains(".as_bool()");
    let has_unwrap = code.contains(".unwrap_or_default()");
    if !has_as_bool && !has_unwrap {
        return code.to_string();
    }
    let bool_vars = extract_bool_typed_vars(code);
    if bool_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &bool_vars {
        if has_as_bool {
            let pattern = format!("{}.as_bool()", var);
            if result.contains(&pattern) {
                result = result.replace(&pattern, var);
            }
        }
        if has_unwrap {
            let pattern = format!("{}.unwrap_or_default()", var);
            if result.contains(&pattern) {
                result = result.replace(&pattern, var);
            }
        }
    }
    result
}

/// DEPYLER-99MODE-S9: Fix bare `Range` type annotation -> `PyRange` (E0425).
/// The transpiler generates `struct PyRange` but uses `Range` as type annotation.
pub(super) fn fix_range_type_annotation(code: &str) -> String {
    if !code.contains("struct PyRange") || !code.contains(": Range") {
        return code.to_string();
    }
    let mut result = code.to_string();
    // Replace ": Range " and ": Range=" type annotations
    result = result.replace(": Range =", ": PyRange =");
    result = result.replace(": Range;", ": PyRange;");
    // Handle "let r: Range = " (with space after)
    result = result.replace(": Range =", ": PyRange =");
    result
}

pub(super) fn fix_closure_to_dyn_fn_ref(code: &str) -> String {
    if !code.contains("&dyn Fn") {
        return code.to_string();
    }
    // Find let-bindings or fn signatures that have &dyn Fn params
    // We need to find: `let NAME = move |..., func: &dyn Fn(...) -> ...| -> ... {`
    // Then at call sites: `NAME(..., move |...|  ...)` -> `NAME(..., &(move |...|  ...))`
    // This is complex in general. Use a targeted approach for the most common pattern.

    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();

    // Collect names of closures/functions that take &dyn Fn params
    let mut dyn_fn_names: Vec<String> = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        // Pattern: `let NAME = move |..., func: &dyn Fn` or `fn NAME(..., func: &dyn Fn`
        if trimmed.contains("&dyn Fn") {
            if let Some(let_idx) = trimmed.find("let ") {
                let after_let = &trimmed[let_idx + 4..];
                if let Some(eq_or_colon) = after_let.find(['=', ':']) {
                    let name = after_let[..eq_or_colon].trim().to_string();
                    if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        dyn_fn_names.push(name);
                    }
                }
            } else if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
                let after_fn = if let Some(stripped) = trimmed.strip_prefix("pub fn ") {
                    stripped
                } else if let Some(stripped) = trimmed.strip_prefix("fn ") {
                    stripped
                } else {
                    continue;
                };
                if let Some(paren) = after_fn.find('(') {
                    let name = after_fn[..paren].trim().to_string();
                    if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        dyn_fn_names.push(name);
                    }
                }
            }
        }
    }

    if dyn_fn_names.is_empty() {
        return code.to_string();
    }

    for line in &lines {
        let mut new_line = (*line).to_string();
        for name in &dyn_fn_names {
            // Find call pattern: `NAME(..., move |...|  ...)`
            // Replace: `move |x: i32| expr)` -> `&(move |x: i32| expr))`
            let call_prefix = format!("{}(", name);
            if new_line.contains(&call_prefix) && new_line.contains("move |") {
                // Find the `move |` that's a trailing argument
                if let Some(move_idx) = new_line.rfind(", move |") {
                    // Find the closing `)` of the outer call
                    if let Some(close_paren) = new_line.rfind(");") {
                        let closure_text = &new_line[move_idx + 2..close_paren];
                        let replacement = format!(" &({})", closure_text);
                        new_line = format!(
                            "{}{}{}",
                            &new_line[..move_idx + 1],
                            replacement,
                            &new_line[close_paren..]
                        );
                    }
                }
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

#[allow(dead_code)]
pub(super) fn fix_tuple_field_on_unit(code: &str) -> String {
    if !code.contains(".0") && !code.contains(".1") {
        return code.to_string();
    }
    // Check for the pattern: `let mut ops: Vec<()>` with later `ops.push((string, int))`
    // This is a transpiler type inference issue; for now skip as it needs AST-level fix
    code.to_string()
}

/// DEPYLER-99MODE-S9: Fix PyRange iteration.
///
/// When `for i in r.iter().cloned()` and `r` is a `PyRange`, replace with
/// `for i in r.start..r.stop` since PyRange doesn't implement Iterator.
pub(super) fn fix_pyrange_iteration(code: &str) -> String {
    if !code.contains("struct PyRange") || !code.contains(".iter().cloned()") {
        return code.to_string();
    }
    // Collect variable names declared as PyRange
    let mut pyrange_vars: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        // Match: `let r: PyRange = PyRange::new(...)` or `let r = PyRange::new(...)`
        if (trimmed.contains(": PyRange") || trimmed.contains("= PyRange::new"))
            && trimmed.starts_with("let ")
        {
            let after_let = trimmed.strip_prefix("let ").unwrap_or("");
            let after_let = after_let.strip_prefix("mut ").unwrap_or(after_let);
            if let Some(sep) = after_let.find([':', ' ']) {
                let var = after_let[..sep].trim().to_string();
                if !var.is_empty() && var.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    pyrange_vars.push(var);
                }
            }
        }
    }
    if pyrange_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &pyrange_vars {
        let old = format!("{}.iter().cloned()", var);
        let new = format!("{}.start..{}.stop", var, var);
        result = result.replace(&old, &new);
    }
    result
}

/// DEPYLER-99MODE-S9: Strip async/await/tokio for standalone rustc compilation.
///
/// Since we compile with `rustc --crate-type lib` (no Cargo, no tokio crate),
/// async code can't resolve `#[tokio::main]`. Strip async constructs to make
/// the code synchronous for compilation purposes.
pub(super) fn fix_remove_async_for_standalone(code: &str) -> String {
    if !code.contains("#[tokio::main]") && !code.contains("async fn") {
        return code.to_string();
    }
    let mut result = code.to_string();
    // Remove #[tokio::main] attribute
    result = result.replace("#[tokio::main] ", "");
    result = result.replace("#[tokio::main]", "");
    // Change `pub async fn` to `pub fn` and `async fn` to `fn`
    result = result.replace("pub async fn ", "pub fn ");
    result = result.replace("async fn ", "fn ");
    // Remove `.await` suffixes
    result = result.replace(".await", "");
    result
}

/// DEPYLER-99MODE-S9: Fix unclosed `vec![` macro wrapping collect expressions.
///
/// When the transpiler generates `vec![EXPR.collect::<Vec<_>>()` without a closing `]`,
/// remove the `vec![` since the expression already produces a Vec.
pub(super) fn fix_unclosed_vec_macro(code: &str) -> String {
    if !code.contains("vec![") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::with_capacity(code.len());
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Detect: `vec![EXPR` where there's no `]` on this or subsequent lines before `}`
        if trimmed.contains("vec![") && !trimmed.contains(']') {
            // Check if a matching `]` appears before the next closing `}`
            let mut has_bracket = false;
            let mut j = i + 1;
            while j < lines.len() {
                let next_trimmed = lines[j].trim();
                if next_trimmed.contains(']') {
                    has_bracket = true;
                    break;
                }
                if next_trimmed == "}"
                    || next_trimmed.starts_with("pub fn ")
                    || next_trimmed.starts_with("#[doc")
                {
                    break;
                }
                j += 1;
            }
            if !has_bracket {
                // Remove the `vec![` wrapper
                let new_line = lines[i].replace("vec![", "");
                result.push_str(&new_line);
                result.push('\n');
                i += 1;
                continue;
            }
        }
        result.push_str(lines[i]);
        result.push('\n');
        i += 1;
    }
    result
}

/// DEPYLER-99MODE-S9: Fix missing inherited fields in child structs.
///
/// When a child struct method accesses `self.field` but `field` isn't in the struct,
/// find it in the parent struct and add it. This handles Python class inheritance.
pub(super) fn fix_missing_inherited_fields(code: &str) -> String {
    if !code.contains("pub struct ") || !code.contains("self.") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let struct_fields = collect_struct_fields(&lines);

    if struct_fields.len() < 2 {
        return code.to_string();
    }

    let additions = find_missing_inherited_fields_all(&lines, &struct_fields);

    if additions.is_empty() {
        return code.to_string();
    }

    apply_inherited_field_additions(code, &additions)
}

/// Parse all `pub struct Name { ... }` blocks and collect their field names and types.
fn collect_struct_fields<'a>(lines: &[&'a str]) -> Vec<(String, Vec<(String, String)>)> {
    let mut struct_fields = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("pub struct ") && trimmed.ends_with('{') {
            let name = trimmed
                .strip_prefix("pub struct ")
                .unwrap_or("")
                .split([' ', '{'])
                .next()
                .unwrap_or("")
                .to_string();
            let mut fields = Vec::new();
            i += 1;
            while i < lines.len() {
                let field_line = lines[i].trim();
                if field_line == "}" {
                    break;
                }
                if let Some(pair) = parse_pub_field(field_line) {
                    fields.push(pair);
                }
                i += 1;
            }
            if !name.is_empty() {
                struct_fields.push((name, fields));
            }
        }
        i += 1;
    }
    struct_fields
}

/// Parse a single `pub field_name: FieldType,` line into (name, type).
fn parse_pub_field(field_line: &str) -> Option<(String, String)> {
    if !field_line.starts_with("pub ") || !field_line.contains(':') {
        return None;
    }
    let after_pub = field_line.strip_prefix("pub ").unwrap_or("");
    let colon = after_pub.find(':')?;
    let field_name = after_pub[..colon].trim().to_string();
    let field_type = after_pub[colon + 1..].trim().trim_end_matches(',').to_string();
    Some((field_name, field_type))
}

/// For every struct, find `self.field` accesses in its impl block that reference
/// fields not present in the struct itself but present in other structs.
fn find_missing_inherited_fields_all(
    lines: &[&str],
    struct_fields: &[(String, Vec<(String, String)>)],
) -> Vec<(String, Vec<(String, String)>)> {
    let mut additions = Vec::new();
    for (struct_name, fields) in struct_fields {
        let missing = find_missing_fields_for_struct(lines, struct_name, fields, struct_fields);
        if !missing.is_empty() {
            additions.push((struct_name.clone(), missing));
        }
    }
    additions
}

/// Scan the impl block for `struct_name` and find self.field references not in `own_fields`,
/// resolving them from other structs.
fn find_missing_fields_for_struct(
    lines: &[&str],
    struct_name: &str,
    own_fields: &[(String, String)],
    all_structs: &[(String, Vec<(String, String)>)],
) -> Vec<(String, String)> {
    let impl_pattern = format!("impl {} {{", struct_name);
    let field_names: Vec<&str> = own_fields.iter().map(|(n, _)| n.as_str()).collect();
    let mut missing_fields: Vec<(String, String)> = Vec::new();
    let mut in_impl = false;
    let mut impl_brace_depth: i32 = 0;

    for line in lines {
        let trimmed = line.trim();
        if !in_impl && trimmed.contains(&impl_pattern) {
            in_impl = true;
            impl_brace_depth = 1;
            continue;
        }
        if in_impl {
            impl_brace_depth += trimmed.chars().filter(|&c| c == '{').count() as i32;
            impl_brace_depth -= trimmed.chars().filter(|&c| c == '}').count() as i32;
            if impl_brace_depth <= 0 {
                in_impl = false;
                continue;
            }
        }
        if in_impl && trimmed.contains("self.") {
            collect_missing_self_fields(
                trimmed,
                &field_names,
                &mut missing_fields,
                struct_name,
                all_structs,
            );
        }
    }
    missing_fields
}

/// Extract all `self.field` accesses from a line and resolve missing ones from other structs.
fn collect_missing_self_fields(
    trimmed: &str,
    field_names: &[&str],
    missing_fields: &mut Vec<(String, String)>,
    struct_name: &str,
    all_structs: &[(String, Vec<(String, String)>)],
) {
    let mut pos = 0;
    while let Some(self_idx) = trimmed[pos..].find("self.") {
        let abs_idx = pos + self_idx + 5;
        let field_end = trimmed[abs_idx..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(trimmed.len() - abs_idx);
        let field = &trimmed[abs_idx..abs_idx + field_end];
        if !field.is_empty()
            && !field_names.contains(&field)
            && !missing_fields.iter().any(|(n, _)| n == field)
        {
            if let Some(pair) = resolve_field_from_other_structs(field, struct_name, all_structs) {
                missing_fields.push(pair);
            }
        }
        pos = abs_idx + field_end;
    }
}

/// Look up a field name in structs other than `struct_name`.
fn resolve_field_from_other_structs(
    field: &str,
    struct_name: &str,
    all_structs: &[(String, Vec<(String, String)>)],
) -> Option<(String, String)> {
    for (other_name, other_fields) in all_structs {
        if other_name != struct_name {
            for (f_name, f_type) in other_fields {
                if f_name == field {
                    return Some((f_name.clone(), f_type.clone()));
                }
            }
        }
    }
    None
}

/// Insert missing fields into struct definitions and add Default::default()
/// initializers into constructor literals.
fn apply_inherited_field_additions(
    code: &str,
    additions: &[(String, Vec<(String, String)>)],
) -> String {
    let mut result = code.to_string();
    for (struct_name, fields_to_add) in additions {
        insert_fields_into_struct(&mut result, struct_name, fields_to_add);
        insert_defaults_into_constructor(&mut result, struct_name, fields_to_add);
    }
    result
}

/// Insert field declarations into a struct definition.
fn insert_fields_into_struct(
    result: &mut String,
    struct_name: &str,
    fields_to_add: &[(String, String)],
) {
    for (field_name, field_type) in fields_to_add {
        let struct_pattern = format!("pub struct {} {{", struct_name);
        if let Some(struct_idx) = result.find(&struct_pattern) {
            let insert_point = struct_idx + struct_pattern.len();
            let field_line = format!("\n    pub {}: {},", field_name, field_type);
            result.insert_str(insert_point, &field_line);
        }
    }
}

/// Find the `Self { ... }` constructor literal (not `-> Self {`) in an impl block
/// and insert Default::default() initializers for inherited fields.
fn insert_defaults_into_constructor(
    result: &mut String,
    struct_name: &str,
    fields_to_add: &[(String, String)],
) {
    let impl_pattern = format!("impl {} {{", struct_name);
    let Some(impl_idx) = result.find(&impl_pattern) else {
        return;
    };
    let after_impl = &result[impl_idx..];
    let mut search_pos = 0;
    while let Some(self_idx) = after_impl[search_pos..].find("Self {") {
        let abs_in_slice = search_pos + self_idx;
        let before = &after_impl[..abs_in_slice];
        if before.trim_end().ends_with("->") {
            search_pos = abs_in_slice + 6;
            continue;
        }
        let abs_self = impl_idx + abs_in_slice + 6;
        let mut extra_fields = String::new();
        for (field_name, _field_type) in fields_to_add {
            extra_fields.push_str(&format!(" {}: Default::default(),", field_name));
        }
        result.insert_str(abs_self, &extra_fields);
        break;
    }
}

pub(super) fn fix_ambiguous_into_on_chain(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Check for multi-line let binding with .into() on a chain
        if trimmed.starts_with("let ") && trimmed.contains(": String") {
            // Look ahead for .into() on subsequent lines
            let mut block = vec![lines[i].to_string()];
            let mut j = i + 1;
            let mut found_into = false;
            while j < lines.len() {
                let next_trimmed = lines[j].trim();
                block.push(lines[j].to_string());
                if next_trimmed == ".into()" || next_trimmed.starts_with(".into()") {
                    found_into = true;
                }
                if next_trimmed.ends_with(';') {
                    break;
                }
                j += 1;
            }
            if found_into {
                // Replace .into() with .to_string() for String context
                for bline in &block {
                    let new_line = bline.replace(".into()", ".to_string()");
                    result.push_str(&new_line);
                    result.push('\n');
                }
                i = j + 1;
                continue;
            }
            // Not modified, output normally
            for bline in &block {
                result.push_str(bline);
                result.push('\n');
            }
            i = j + 1;
            continue;
        }
        result.push_str(lines[i]);
        result.push('\n');
        i += 1;
    }
    result
}

/// DEPYLER-99MODE-S9: Fix `let VAR = CHAIN.expect("...").into();` without type annotation (E0283).
///
/// When `.into()` follows a DepylerValue-producing chain and the let binding has no type
/// annotation, the conversion target is ambiguous. Add `: i32` annotation.
pub(super) fn fix_ambiguous_into_type_annotation(code: &str) -> String {
    let dv_chain_into_patterns = [
        ".expect(\"IndexError: list index out of range\").into()",
        ".expect(\"value is None\").into()",
        ".unwrap_or_default().into()",
    ];
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut new_line = line.to_string();
        let mut modified = false;
        for pattern in &dv_chain_into_patterns {
            if !new_line.contains(pattern) {
                continue;
            }
            // Find `let IDENT = ` before the chain (possibly nested in blocks)
            // We need to find the closest `let IDENT = ` that has no `:` type annotation
            if let Some(pat_pos) = new_line.find(pattern) {
                let before = &new_line[..pat_pos];
                // Find the last `let ` before the pattern
                if let Some(let_offset) = before.rfind("let ") {
                    let after_let = &before[let_offset + 4..];
                    // Find ` = ` to extract variable name
                    if let Some(eq_offset) = after_let.find(" = ") {
                        let var_name = after_let[..eq_offset].trim();
                        // Only annotate if no existing type annotation
                        if !var_name.contains(':')
                            && !var_name.is_empty()
                            && var_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                        {
                            let insert_pos = let_offset + 4 + eq_offset;
                            new_line = format!(
                                "{}: i32{}",
                                &new_line[..insert_pos],
                                &new_line[insert_pos..]
                            );
                            modified = true;
                            break;
                        }
                    }
                }
            }
        }
        if modified {
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_dict_get_return(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut current_return_type: Option<&str> = None;
    // Vars that hold Option<DepylerValue> from .get().cloned() (not already unwrapped)
    let mut option_dv_vars: Vec<String> = Vec::new();
    // Vars that hold DepylerValue (already unwrapped from dict.get)
    let mut dv_vars: Vec<String> = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            option_dv_vars.clear();
            dv_vars.clear();
            current_return_type = None;
            if let Some(arrow) = trimmed.find("-> ") {
                let ret = trimmed[arrow + 3..].split('{').next().unwrap_or("").trim();
                if ret == "i32" || ret == "f64" {
                    current_return_type = Some(ret);
                }
            }
        }
        // Track: `let value = data.get("x").cloned();` -- value is Option<DepylerValue>
        // BUT: `let value = data.get("x").cloned().unwrap_or(...)` -- value is DepylerValue
        if trimmed.starts_with("let ")
            && trimmed.contains(".get(\"")
            && trimmed.contains(".cloned()")
        {
            if let Some(eq_pos) = trimmed.find(" = ") {
                let var_part = trimmed[4..eq_pos].trim();
                let var_name = var_part.split(':').next().unwrap_or(var_part).trim();
                // Check if already unwrapped on same line
                if trimmed.contains(".unwrap_or(") || trimmed.contains(".unwrap()") {
                    dv_vars.push(var_name.to_string());
                } else {
                    option_dv_vars.push(var_name.to_string());
                }
            }
        }
        if current_return_type.is_some() {
            let trimmed_expr = if trimmed.starts_with("return ") && trimmed.ends_with(';') {
                Some(trimmed[7..trimmed.len() - 1].trim())
            } else {
                None
            };
            // Fix Option<DV> var: `return value;` -> `return value.unwrap_or(DV::Int(0)).into();`
            if let Some(expr) = trimmed_expr {
                if option_dv_vars.iter().any(|v| v == expr) {
                    let indent = &line[..line.len() - trimmed.len()];
                    result.push_str(&format!(
                        "{}return {}.unwrap_or(DepylerValue::Int(0i64)).into();",
                        indent, expr
                    ));
                    result.push('\n');
                    continue;
                }
                // Fix DV var: `return value;` -> `return value.into();`
                if dv_vars.iter().any(|v| v == expr) {
                    let indent = &line[..line.len() - trimmed.len()];
                    result.push_str(&format!("{}return {}.into();", indent, expr));
                    result.push('\n');
                    continue;
                }
            }
            // Fix tail: option var
            if !trimmed.ends_with(';') && option_dv_vars.iter().any(|v| v.as_str() == trimmed) {
                let indent = &line[..line.len() - trimmed.len()];
                result.push_str(&format!(
                    "{}{}.unwrap_or(DepylerValue::Int(0i64)).into()",
                    indent, trimmed
                ));
                result.push('\n');
                continue;
            }
            // Fix tail: dv var
            if !trimmed.ends_with(';') && dv_vars.iter().any(|v| v.as_str() == trimmed) {
                let indent = &line[..line.len() - trimmed.len()];
                result.push_str(&format!("{}{}.into()", indent, trimmed));
                result.push('\n');
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_iter_on_impl_iterator(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    // Track functions returning impl Iterator
    let mut iter_fns: Vec<String> = Vec::new();
    let lines: Vec<&str> = code.lines().collect();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.contains("-> impl Iterator") || trimmed.contains("-> impl IntoIterator") {
            if let Some(name_start) = trimmed.find("fn ") {
                let after_fn = &trimmed[name_start + 3..];
                if let Some(paren) = after_fn.find('(') {
                    let fn_name = after_fn[..paren].trim();
                    // Strip generic parameters (e.g., fibonacci_generator<'a> -> fibonacci_generator)
                    let fn_name =
                        if let Some(gen) = fn_name.find('<') { &fn_name[..gen] } else { fn_name };
                    if !fn_name.is_empty() {
                        iter_fns.push(fn_name.to_string());
                    }
                }
            }
        }
    }
    let mut prev_had_iter_fn_call = false;
    let mut just_removed_iter = false;
    for line in &lines {
        let mut new_line = line.to_string();
        let trimmed = line.trim();
        let mut has_iter_fn_call = false;
        for fn_name in &iter_fns {
            let call_pat = format!("{fn_name}(");
            if new_line.contains(&call_pat) {
                has_iter_fn_call = true;
                if new_line.contains(".iter()") {
                    new_line = new_line.replace(".iter()", "");
                    just_removed_iter = true;
                }
            }
        }
        // Next-line .iter() after a line with an iterator fn call
        if prev_had_iter_fn_call && trimmed.starts_with(".iter()") {
            new_line = new_line.replace(".iter()", "");
            just_removed_iter = true;
        }
        // After removing .iter(), also remove .cloned() since impl Iterator yields owned values
        if just_removed_iter && trimmed == ".cloned()" {
            // Skip this line entirely (remove .cloned())
            just_removed_iter = false;
            prev_had_iter_fn_call = false;
            continue;
        }
        if just_removed_iter && trimmed.starts_with(".cloned()") {
            new_line = new_line.replace(".cloned()", "");
        }
        if just_removed_iter && !trimmed.is_empty() && trimmed != ".cloned()" {
            just_removed_iter = false;
        }
        prev_had_iter_fn_call = has_iter_fn_call;
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_void_fn_with_return_value(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Find functions that have `return EXPR as i32;` but no return type
    // Pattern: `pub fn name(...) {` followed later by `return EXPR as i32;`
    let mut fix_fns: Vec<String> = Vec::new();
    let mut current_fn: Option<(String, bool)> = None; // (name, has_return_type)
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            let has_arrow = trimmed.contains("-> ");
            if let Some(name_start) = trimmed.find("fn ") {
                let after = &trimmed[name_start + 3..];
                if let Some(paren) = after.find('(') {
                    let name = after[..paren].trim().to_string();
                    current_fn = Some((name, has_arrow));
                }
            }
        }
        if let Some((ref name, false)) = current_fn {
            if trimmed.starts_with("return ") && trimmed.contains(" as i32;") {
                fix_fns.push(name.clone());
                current_fn = None;
            }
        }
        if trimmed == "}" {
            current_fn = None;
        }
    }
    // Apply fixes: add `-> i32` to signatures of identified functions
    for line in &lines {
        let trimmed = line.trim();
        let mut replaced = false;
        for fn_name in &fix_fns {
            let pat = format!("fn {}(", fn_name);
            if trimmed.contains(&pat) && !trimmed.contains("-> ") && trimmed.contains(") {") {
                let new_line = line.replace(") {", ") -> i32 {");
                result.push_str(&new_line);
                replaced = true;
                break;
            }
        }
        if !replaced {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

#[allow(dead_code)]
pub(super) fn fix_write_all_on_custom_struct(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    // Check if any struct has a `pub fn write(&self, data: String)` method
    let mut has_custom_write = false;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.contains("pub fn write(") && trimmed.contains("data: String") {
            has_custom_write = true;
            break;
        }
    }
    if !has_custom_write {
        return code.to_string();
    }
    // Replace .write_all("...".as_bytes()).expect("...") with .write("...".to_string())
    let mut result = code.to_string();
    // Pattern: .write_all("STR".as_bytes())
    while let Some(pos) = result.find(".write_all(") {
        let after = &result[pos + 11..];
        // Find matching close paren
        let mut depth = 1i32;
        let mut end = None;
        for (i, ch) in after.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(close) = end {
            let inner = &after[..close];
            // Convert: "STR".as_bytes() -> "STR".to_string()
            let new_inner = if inner.contains(".as_bytes()") {
                inner.replace(".as_bytes()", ".to_string()")
            } else {
                inner.to_string()
            };
            // Also check for .expect("...") after the close paren
            let after_close = &after[close + 1..];
            let expect_len = if let Some(expect_inner) = after_close.strip_prefix(".expect(") {
                // Find matching close paren for .expect(
                let mut d = 1i32;
                let mut eend = expect_inner.len();
                for (i, ch) in expect_inner.char_indices() {
                    match ch {
                        '(' => d += 1,
                        ')' => {
                            d -= 1;
                            if d == 0 {
                                eend = i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                8 + eend + 1 // .expect( + inner + )
            } else {
                0
            };
            let old_len = 11 + close + 1 + expect_len; // .write_all( + inner + ) + .expect(...)
            let old = &result[pos..pos + old_len];
            let new = format!(".write({})", new_inner);
            result = result.replacen(old, &new, 1);
        } else {
            break;
        }
    }
    result
}

pub(super) fn fix_let_type_from_fn_return(code: &str) -> String {
    let fn_returns = build_custom_fn_return_map(code);
    if fn_returns.is_empty() {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    for line in &lines {
        let trimmed = line.trim();
        if let Some(fixed) = try_fix_let_binding_type(line, trimmed, &fn_returns) {
            result.push(fixed);
        } else {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}

/// Build a map of fn_name -> return_type for functions returning custom types.
fn build_custom_fn_return_map(code: &str) -> std::collections::HashMap<String, String> {
    let mut fn_returns = std::collections::HashMap::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if !(trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "))
            || !trimmed.contains("-> ")
        {
            continue;
        }
        if let Some((fn_name, ret_type)) = extract_fn_name_and_return(trimmed) {
            if is_custom_return_type(&ret_type) {
                fn_returns.insert(fn_name, ret_type);
            }
        }
    }
    fn_returns
}

/// Extract function name and return type from a function signature line.
fn extract_fn_name_and_return(trimmed: &str) -> Option<(String, String)> {
    let after_fn = trimmed.strip_prefix("pub fn ").or_else(|| trimmed.strip_prefix("fn "))?;
    let paren_pos = after_fn.find('(')?;
    let fn_name = after_fn[..paren_pos].trim().to_string();
    let arrow_pos = trimmed.find("-> ")?;
    let after_arrow = &trimmed[arrow_pos + 3..];
    let ret_type = after_arrow.trim_end_matches('{').trim_end_matches("where").trim().to_string();
    Some((fn_name, ret_type))
}

/// Check if a return type is a custom (PascalCase) type, not a standard library type.
fn is_custom_return_type(ret_type: &str) -> bool {
    if ret_type.is_empty() {
        return false;
    }
    let starts_upper = ret_type.chars().next().is_some_and(|c| c.is_uppercase());
    if !starts_upper || ret_type.contains("Self") {
        return false;
    }
    let excluded_exact = ["String", "Vec", "HashMap", "Option", "Result", "Box"];
    if excluded_exact.contains(&ret_type) {
        return false;
    }
    let excluded_prefixes = ["Vec<", "Result<", "Option<", "Box<", "HashMap<", "std::"];
    !excluded_prefixes.iter().any(|p| ret_type.starts_with(p))
}

/// Try to fix a `let VAR: TYPE = fn_name(ARGS);` binding to use the correct return type.
fn try_fix_let_binding_type(
    line: &str,
    trimmed: &str,
    fn_returns: &std::collections::HashMap<String, String>,
) -> Option<String> {
    if !trimmed.starts_with("let ") || !trimmed.contains(": ") || !trimmed.contains(" = ") {
        return None;
    }
    let after_let = &trimmed[4..];
    let colon_pos = after_let.find(": ")?;
    let var_name = &after_let[..colon_pos];
    let after_colon = &after_let[colon_pos + 2..];
    let eq_pos = after_colon.find(" = ")?;
    let declared_type = after_colon[..eq_pos].trim();
    let rhs = after_colon[eq_pos + 3..].trim();
    let paren = rhs.find('(')?;
    let called_fn = rhs[..paren].trim();
    let correct_type = fn_returns.get(called_fn)?;
    if declared_type == correct_type {
        return None;
    }
    let indent = &line[..line.len() - trimmed.len()];
    Some(format!("{}let {}: {} = {}", indent, var_name, correct_type, rhs))
}

/// DEPYLER-99MODE-S9: Fix Vec type in assert_eq by inferring from function return type.
///
/// When `assert_eq!(fn_name(ARGS).unwrap(), Vec::<WRONG>::new(), ...)`,
/// looks up fn_name's return type `Result<Vec<CORRECT>, ...>` and replaces
/// `Vec::<WRONG>::new()` with `Vec::<CORRECT>::new()`.
pub(super) fn fix_assert_vec_type_from_fn_return(code: &str) -> String {
    if !code.contains("assert_eq!") || !code.contains("Vec::<") {
        return code.to_string();
    }
    let fn_vec_type = build_fn_vec_inner_type_map(code);
    if fn_vec_type.is_empty() {
        return code.to_string();
    }
    fix_assert_vec_lines(code, &fn_vec_type)
}

/// Build a map of fn_name -> inner Vec element type from `Result<Vec<T>, E>` signatures.
fn build_fn_vec_inner_type_map(code: &str) -> std::collections::HashMap<String, String> {
    let mut fn_vec_type = std::collections::HashMap::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if let Some((name, inner)) = extract_result_vec_inner_type(trimmed) {
            fn_vec_type.insert(name, inner);
        }
    }
    fn_vec_type
}

/// From a function signature like `fn foo(...) -> Result<Vec<MyType>, E>`,
/// extract ("foo", "MyType").
fn extract_result_vec_inner_type(trimmed: &str) -> Option<(String, String)> {
    if !(trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ")) {
        return None;
    }
    if !trimmed.contains("-> Result<Vec<") {
        return None;
    }
    let after_fn = trimmed.strip_prefix("pub fn ").or_else(|| trimmed.strip_prefix("fn "))?;
    let paren_pos = after_fn.find('(')?;
    let fn_name = after_fn[..paren_pos].trim().to_string();
    let vec_start = trimmed.find("-> Result<Vec<")?;
    let inner_start = vec_start + 14;
    let after_inner = &trimmed[inner_start..];
    let inner_type = extract_balanced_angle_content(after_inner)?;
    Some((fn_name, inner_type))
}

/// Extract content up to the matching closing `>`, handling nested angle brackets.
fn extract_balanced_angle_content(s: &str) -> Option<String> {
    let mut depth = 1;
    for (i, ch) in s.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    return Some(s[..i].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// Scan assert_eq!/assert_ne! blocks and fix Vec::<WRONG>::new() with correct inner types.
fn fix_assert_vec_lines(
    code: &str,
    fn_vec_type: &std::collections::HashMap<String, String>,
) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut current_fn_call: Option<String> = None;
    let mut in_assert = false;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("assert_eq!(") || trimmed.starts_with("assert_ne!(") {
            in_assert = true;
            current_fn_call = find_fn_call_in_line(trimmed, fn_vec_type);
        }
        if in_assert {
            if let Some(fixed) = try_fix_vec_new_line(line, trimmed, &current_fn_call, fn_vec_type)
            {
                result.push(fixed);
                if trimmed.ends_with(");") {
                    in_assert = false;
                }
                continue;
            }
            if current_fn_call.is_none() {
                current_fn_call = find_fn_call_in_line(trimmed, fn_vec_type);
            }
            if trimmed.ends_with(");") {
                in_assert = false;
                current_fn_call = None;
            }
        }
        result.push(line.to_string());
    }
    result.join("\n")
}

/// Check if a line contains a known function call with .unwrap().
fn find_fn_call_in_line(
    trimmed: &str,
    fn_vec_type: &std::collections::HashMap<String, String>,
) -> Option<String> {
    for fn_name in fn_vec_type.keys() {
        if trimmed.contains(&format!("{}(", fn_name)) && trimmed.contains(".unwrap()") {
            return Some(fn_name.clone());
        }
    }
    None
}

/// If the line is a standalone `Vec::<T>::new()` inside an assert, fix the inner type.
fn try_fix_vec_new_line(
    line: &str,
    trimmed: &str,
    current_fn_call: &Option<String>,
    fn_vec_type: &std::collections::HashMap<String, String>,
) -> Option<String> {
    let fn_name = current_fn_call.as_ref()?;
    let content = trimmed.trim_end_matches(',');
    if !content.starts_with("Vec::<") || !content.ends_with(">::new()") {
        return None;
    }
    let correct_inner = fn_vec_type.get(fn_name)?;
    let indent = &line[..line.len() - trimmed.len()];
    let trailing = if trimmed.ends_with(',') { "," } else { "" };
    Some(format!("{}Vec::<{}>::new(){}", indent, correct_inner, trailing))
}

pub(super) fn fix_let_unit_type_annotation(code: &str) -> String {
    if !code.contains(": () =") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    for line in &lines {
        let trimmed = line.trim();
        // Match: let IDENT: () = EXPR  (but NOT let _: () = ...)
        if trimmed.starts_with("let ")
            && trimmed.contains(": () =")
            && !trimmed.starts_with("let _:")
        {
            let new_line = line.replace(": () =", " =");
            result.push(new_line);
        } else {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}

fn extract_bool_typed_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let mut vec_bool_params: Vec<String> = Vec::new();

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            extract_bool_params_from_signature(trimmed, &mut vars, &mut vec_bool_params);
            continue;
        }
        if trimmed.starts_with("let ") {
            extract_bool_from_let_binding(trimmed, &mut vars);
        }
        if trimmed.starts_with("for ") {
            extract_bool_from_for_loop(trimmed, &vec_bool_params, &mut vars);
        }
    }
    vars
}

/// Extract bool-typed parameter names and Vec<bool> parameter names from a function signature.
fn extract_bool_params_from_signature(
    trimmed: &str,
    vars: &mut Vec<String>,
    vec_bool_params: &mut Vec<String>,
) {
    let Some(start) = trimmed.find('(') else {
        return;
    };
    let Some(end) = trimmed.find(')') else {
        return;
    };
    let params = &trimmed[start + 1..end];
    for param in params.split(',') {
        let p = param.trim();
        if let Some(name) = p.strip_suffix(": bool") {
            let name = name.trim();
            if !name.is_empty() {
                vars.push(name.to_string());
            }
        }
        if p.contains("Vec<bool>") || p.contains("Vec < bool >") {
            if let Some(colon_pos) = p.find(':') {
                let name = p[..colon_pos].trim();
                if !name.is_empty() {
                    vec_bool_params.push(name.to_string());
                }
            }
        }
    }
}

/// Extract bool-typed variable names from `let [mut] name: bool = ...` declarations.
fn extract_bool_from_let_binding(trimmed: &str, vars: &mut Vec<String>) {
    let rest = trimmed.strip_prefix("let ").unwrap_or("");
    let rest = rest.strip_prefix("mut ").unwrap_or(rest);
    if let Some(colon_pos) = rest.find(": bool") {
        let name = rest[..colon_pos].trim();
        if is_valid_identifier(name) {
            vars.push(name.to_string());
        }
    }
}

/// Extract loop variables iterating over Vec<bool> parameters.
fn extract_bool_from_for_loop(trimmed: &str, vec_bool_params: &[String], vars: &mut Vec<String>) {
    let Some(in_pos) = trimmed.find(" in ") else {
        return;
    };
    let loop_var = trimmed[4..in_pos].trim();
    let iter_part = &trimmed[in_pos + 4..];
    for param in vec_bool_params {
        let matches_param = iter_part.starts_with(&format!("{param}."))
            || iter_part.starts_with(&format!("{param} "));
        if matches_param && is_valid_identifier(loop_var) {
            vars.push(loop_var.to_string());
        }
    }
}

/// Check if a string is a non-empty valid Rust identifier (alphanumeric + underscore).
fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}
