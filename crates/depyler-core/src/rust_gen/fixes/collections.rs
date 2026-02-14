//! Collection-related fix functions for post-transpilation Rust code repair.
//!
//! These functions handle fixes for HashMap, Vec, and other collection types
//! in generated Rust code, including type annotation mismatches, method name
//! corrections, and iterator/collect patterns.

/// Extract the value type from a return type like `-> HashMap<String, ValueType>`.
fn extract_hashmap_return_value_type(trimmed: &str) -> Option<String> {
    let start = trimmed.find("-> HashMap<String,")?;
    let after = &trimmed[start + 18..];
    let end = after.rfind('>')?;
    let vtype = after[..end].trim().to_string();
    if vtype != "()" && !vtype.is_empty() {
        Some(vtype)
    } else {
        None
    }
}

pub(super) fn fix_hashmap_empty_value_type(code: &str) -> String {
    if !code.contains("HashMap<String, ()>") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut current_return_value_type: Option<String> = None;

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.contains("-> HashMap<String,") {
            if let Some(vtype) = extract_hashmap_return_value_type(trimmed) {
                current_return_value_type = Some(vtype);
            }
        }
        if trimmed.contains("HashMap<String, ()>") {
            if let Some(ref vtype) = current_return_value_type {
                let fixed = line.replace(
                    "HashMap<String, ()>",
                    &format!("HashMap<String, {}>", vtype),
                );
                result.push(fixed);
                continue;
            }
        }
        if (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "))
            && !trimmed.contains("-> HashMap<String,")
        {
            current_return_value_type = None;
        }
        result.push(line.to_string());
    }
    result.join("\n")
}

pub(super) fn fix_hashmap_contains(code: &str) -> String {
    let hashmap_vars = extract_hashmap_typed_vars(code);
    if hashmap_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &hashmap_vars {
        let pattern = format!("{var}.contains(");
        let replacement = format!("{var}.contains_key(");
        if result.contains(&pattern) {
            eprintln!("[DEBUG] Replacing '{}' with '{}'", pattern, replacement);
            result = result.replace(&pattern, &replacement);
        }
    }
    result
}

/// Check if a name is a valid Rust identifier (alphanumeric + underscore, non-empty).
fn is_valid_ident(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Try to extract a HashMap variable name from a parameter declaration line.
fn extract_hashmap_param_var(trimmed: &str) -> Option<String> {
    if !trimmed.contains("HashMap<") || !trimmed.contains(':') || trimmed.contains("Option<") {
        return None;
    }
    // Skip function return types (contains `->`)
    if trimmed.contains("->") && !trimmed.contains(',') {
        return None;
    }
    let clean = trimmed.trim_end_matches(',').trim();
    let colon_pos = clean.find(':')?;
    let name = clean[..colon_pos].trim();
    if is_valid_ident(name) && !name.starts_with("fn ") && !name.starts_with("pub ") {
        Some(name.to_string())
    } else {
        None
    }
}

/// Try to extract a HashMap variable name from a `let` declaration line.
fn extract_hashmap_let_var(trimmed: &str) -> Option<String> {
    if !trimmed.starts_with("let ") || !trimmed.contains("HashMap<") || trimmed.contains("Option<")
    {
        return None;
    }
    let rest = trimmed.strip_prefix("let ")?.trim_start_matches("mut ");
    let colon_pos = rest.find(':')?;
    let name = rest[..colon_pos].trim();
    if is_valid_ident(name) {
        Some(name.to_string())
    } else {
        None
    }
}

/// Extract variable names typed as HashMap from function signatures and local declarations.
/// Handles multi-line function signatures where params span multiple lines.
pub(super) fn extract_hashmap_typed_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_hashmap_param_var(trimmed) {
            vars.push(name);
        } else if let Some(name) = extract_hashmap_let_var(trimmed) {
            vars.push(name);
        }
    }
    vars
}

/// Extract the identifier following `.contains(&*` at the given position.
/// Returns Some(var_name) if a valid identifier followed by `)` is found.
fn extract_deref_contains_var(result: &str, pos: usize) -> Option<String> {
    let after = pos + ".contains(&*".len();
    let mut end = after;
    let bytes = result.as_bytes();
    while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_') {
        end += 1;
    }
    if end > after && end < bytes.len() && bytes[end] == b')' {
        Some(result[after..end].to_string())
    } else {
        None
    }
}

pub(super) fn fix_vec_contains_deref(code: &str) -> String {
    if !code.contains(".contains(&*") {
        return code.to_string();
    }
    let mut result = code.to_string();
    loop {
        let pos = match result.find(".contains(&*") {
            Some(p) => p,
            None => break,
        };
        if let Some(var) = extract_deref_contains_var(&result, pos) {
            let old = format!(".contains(&*{})", var);
            let new = format!(".iter().any(|s| s == {})", var);
            result = result.replacen(&old, &new, 1);
        } else {
            break;
        }
    }
    result
}

/// DEPYLER-CONVERGE-MULTI-ITER10: Fix `.get(&ref).is_some()` -> `.iter().any()`.
///
/// The transpiler generates `VEC.get(&string_ref).is_some()` for `in` checks,
/// but `Vec::get()` takes `usize`, not `&String`. Convert to `.iter().any()`.
pub(super) fn fix_vec_get_membership(code: &str) -> String {
    if !code.contains(".get(&") || !code.contains(".is_some()") {
        return code.to_string();
    }
    // DEPYLER-99MODE-S9: Collect HashMap-typed variables so we can skip them.
    // Their .get(&key).is_some() is correct and should NOT be rewritten to .contains().
    let hashmap_typed = extract_hashmap_typed_vars(code);
    let mut result = code.to_string();
    let mut search_from = 0;
    loop {
        let haystack = &result[search_from..];
        let rel_pos = match haystack.find(".get(&") {
            Some(p) => p,
            None => break,
        };
        let pos = search_from + rel_pos;

        // DEPYLER-99MODE-S9: Skip HashMap/dict variables — their .get(&key).is_some()
        // is correct. Only Vec needs this fix (Vec::get takes usize, not &T).
        // Look at the variable name before .get() to detect dicts.
        let before = &result[..pos];
        let var_name: String = before
            .chars()
            .rev()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        let is_dict_var = matches!(
            var_name.as_str(),
            "memo" | "seen" | "visited" | "counts" | "freq"
                | "frequency" | "lookup" | "graph" | "adj"
                | "dp" | "cache" | "config" | "settings"
                | "params" | "headers" | "env" | "mapping"
                | "index" | "registry" | "table" | "counter"
                | "scores" | "weights" | "distances"
                | "data" | "info" | "metadata" | "kwargs"
                | "context" | "options" | "result" | "results"
                | "record" | "row" | "entry" | "item"
        ) || var_name.contains("dict")
            || var_name.contains("map")
            || var_name.contains("hash");

        // Also check if the line declares this as HashMap
        let line_start = before.rfind('\n').map_or(0, |p| p + 1);
        let line = &result[line_start..];
        let is_hashmap_line = line.contains("HashMap") || line.contains("BTreeMap");

        // DEPYLER-99MODE-S9: Check against extracted HashMap-typed variables
        let is_hashmap_typed = hashmap_typed.contains(&var_name);

        if is_dict_var || is_hashmap_line || is_hashmap_typed {
            search_from = pos + 1;
            continue;
        }

        let after = pos + ".get(&".len();
        let mut depth = 1;
        let mut end = after;
        let bytes = result.as_bytes();
        while end < bytes.len() && depth > 0 {
            if bytes[end] == b'(' {
                depth += 1;
            } else if bytes[end] == b')' {
                depth -= 1;
            }
            if depth > 0 {
                end += 1;
            }
        }
        if depth == 0 && end < bytes.len() {
            let expr = result[after..end].to_string();
            if expr.starts_with(|c: char| c.is_ascii_digit()) {
                search_from = end + 1;
                continue;
            }
            let suffix_start = end + 1;
            if suffix_start + 10 <= result.len()
                && &result[suffix_start..suffix_start + 10] == ".is_some()"
            {
                let old = format!(".get(&{}).is_some()", expr);
                let new = format!(".contains(&{})", expr);
                result = result.replacen(&old, &new, 1);
                continue;
            }
        }
        search_from = pos + 1;
    }
    result
}

pub(super) fn fix_vec_char_join(code: &str) -> String {
    // Handle single-line pattern
    let result = code.replace(".collect::<Vec<_>>().join(\"\")", ".collect::<String>()");
    // Handle multi-line: `.collect::<Vec<_>>()` on one line, `.join("");` on next
    let lines: Vec<&str> = result.lines().collect();
    let mut output = Vec::with_capacity(lines.len());
    let mut skip_next = false;
    for i in 0..lines.len() {
        if skip_next {
            skip_next = false;
            continue;
        }
        let trimmed = lines[i].trim();
        if trimmed.ends_with(".collect::<Vec<_>>()") && i + 1 < lines.len() {
            let next_trimmed = lines[i + 1].trim();
            if next_trimmed.starts_with(".join(\"\")") {
                // Replace collect with String collection and skip join line
                let fixed = lines[i].replace(".collect::<Vec<_>>()", ".collect::<String>()");
                // If the join line has a trailing semicolon, append it
                let suffix = next_trimmed.strip_prefix(".join(\"\")").unwrap_or("");
                output.push(format!("{}{}", fixed, suffix));
                skip_next = true;
                continue;
            }
        }
        output.push(lines[i].to_string());
    }
    output.join("\n")
}

pub(super) fn fix_hashmap_key_type_mismatch(code: &str) -> String {
    code.to_string()
}

/// Extract a Vec-typed variable name from a struct field declaration.
fn extract_vec_struct_field(t: &str) -> Option<String> {
    if !t.starts_with("pub ") || !t.contains(": Vec<") {
        return None;
    }
    let colon = t.find(": Vec<")?;
    let field = t[4..colon].trim();
    if field.is_empty() { None } else { Some(field.to_string()) }
}

/// Extract a Vec-typed variable name from a let binding.
fn extract_vec_let_binding(t: &str) -> Option<String> {
    if !t.starts_with("let ") || !t.contains(": Vec<") {
        return None;
    }
    let name = t.strip_prefix("let ")?;
    let var = name.split(':').next().unwrap_or("").trim();
    let var = var.trim_start_matches("mut ");
    if var.is_empty() { None } else { Some(var.to_string()) }
}

/// Extract a Vec-typed variable name from a function parameter.
fn extract_vec_param(t: &str) -> Option<String> {
    if !(t.contains(": &Vec<") || t.contains(": Vec<"))
        || t.starts_with("pub ")
        || t.starts_with("let ")
    {
        return None;
    }
    let parts: Vec<&str> = t.split(':').collect();
    if parts.len() < 2 {
        return None;
    }
    let param = parts[0].trim().trim_start_matches("mut ").trim_end_matches(',');
    if is_valid_ident(param) {
        Some(param.to_string())
    } else {
        None
    }
}

/// Collect all Vec-typed variable names from source code.
fn collect_vec_typed_vars(code: &str) -> std::collections::HashSet<String> {
    let mut vec_vars = std::collections::HashSet::new();
    for line in code.lines() {
        let t = line.trim();
        if let Some(v) = extract_vec_struct_field(t) {
            vec_vars.insert(v);
        }
        if let Some(v) = extract_vec_let_binding(t) {
            vec_vars.insert(v);
        }
        if let Some(v) = extract_vec_param(t) {
            vec_vars.insert(v);
        }
    }
    vec_vars
}

pub(super) fn fix_vec_to_string_debug(code: &str) -> String {
    let vec_vars = collect_vec_typed_vars(code);
    if vec_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &vec_vars {
        // Skip internal boilerplate variables (_dv_ prefix)
        // and single-char vars (too prone to false positives in closures like |s|)
        if var.starts_with("_dv_") || var.len() <= 1 {
            continue;
        }
        let old = format!("{}.to_string()", var);
        let new = format!("{}.clone()", var);
        result = result.replace(&old, &new);
    }
    result
}

pub(super) fn fix_sorted_vec_reference(code: &str) -> String {
    if !code.contains("let mut sorted_vec = &") {
        return code.to_string();
    }
    code.replace("let mut sorted_vec = &", "let mut sorted_vec = ")
}

/// Check if a line is a `for VAR in EXPR.keys() {` loop that needs `.cloned()`.
fn is_keys_for_loop(trimmed: &str) -> bool {
    trimmed.starts_with("for ")
        && trimmed.contains(".keys()")
        && trimmed.ends_with('{')
        && !trimmed.contains(".cloned()")
}

/// Extract loop variable and map variable from `for VAR in MAP.keys() {`.
fn extract_keys_loop_vars(trimmed: &str) -> (String, String) {
    let after_for = trimmed.strip_prefix("for ").unwrap_or("");
    let loop_var = after_for
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();
    let map_var = after_for
        .find(" in ")
        .and_then(|in_idx| {
            let after_in = &after_for[in_idx + 4..];
            after_in.find(".keys()").map(|keys_idx| after_in[..keys_idx].trim().to_string())
        })
        .unwrap_or_default();
    (loop_var, map_var)
}

/// Process the body of a keys loop, fixing `.get(key)` to `.get(&key)`.
fn process_keys_loop_body(
    lines: &[&str],
    start: usize,
    loop_var: &str,
    result: &mut String,
) -> usize {
    let get_pattern = format!(".get({})", loop_var);
    let get_fixed = format!(".get(&{})", loop_var);
    let mut brace_depth: i32 = 1;
    let mut i = start;
    while i < lines.len() && brace_depth > 0 {
        let mut line = lines[i].to_string();
        brace_depth += count_brace_depth(lines[i]);
        if line.contains(&get_pattern) {
            line = line.replace(&get_pattern, &get_fixed);
        }
        result.push_str(&line);
        result.push('\n');
        i += 1;
    }
    i
}

pub(super) fn fix_hashmap_keys_iter_clone(code: &str) -> String {
    if !code.contains(".keys()") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::with_capacity(code.len());
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if is_keys_for_loop(trimmed) {
            let (loop_var, map_var) = extract_keys_loop_vars(trimmed);
            result.push_str(&lines[i].replace(".keys() {", ".keys().cloned() {"));
            result.push('\n');
            i += 1;
            if !loop_var.is_empty() && !map_var.is_empty() {
                i = process_keys_loop_body(&lines, i, &loop_var, &mut result);
            }
        } else {
            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
        }
    }
    result
}

/// Scan forward from a starting line to find a pattern, tracking brace depth.
/// Returns true if `needle` is found before the statement ends.
fn scan_multiline_for_pattern(lines: &[&str], start_line: &str, from: usize, needle: &str) -> bool {
    if start_line.contains(needle) {
        return true;
    }
    let mut brace_depth: i32 = count_brace_depth(start_line);
    let mut check_line = from;
    while check_line < lines.len() {
        let cl = lines[check_line].trim();
        brace_depth += count_brace_depth(cl);
        if cl.contains(needle) {
            return true;
        }
        if brace_depth <= 0 && cl.ends_with(';') {
            break;
        }
        check_line += 1;
    }
    false
}

/// Count brace depth changes in a string.
fn count_brace_depth(s: &str) -> i32 {
    s.chars().fold(0, |d, c| match c {
        '{' => d + 1,
        '}' => d - 1,
        _ => d,
    })
}

/// Extract the `let` keyword offset (4 for `let `, 8 for `let mut `).
fn let_binding_offset(trimmed: &str) -> Option<usize> {
    if trimmed.starts_with("let mut ") {
        Some(8)
    } else if trimmed.starts_with("let ") {
        Some(4)
    } else {
        None
    }
}

/// Check if a type annotation is a wrong primitive for a collect expression.
fn is_wrong_primitive_type(type_ann: &str) -> bool {
    matches!(type_ann, "bool" | "i32" | "f64" | "String")
}

/// Check if a type annotation is a DepylerValue collection type.
fn is_depyler_collection_type(type_ann: &str) -> bool {
    type_ann.contains("DepylerValue")
        && (type_ann.contains("Vec") || type_ann.contains("HashSet") || type_ann.contains("HashMap"))
}

/// Determine if the type annotation on a collect line should be removed.
fn should_strip_type_annotation(
    lines: &[&str],
    trimmed: &str,
    line_idx: usize,
    type_ann: &str,
) -> bool {
    let has_collect = scan_multiline_for_pattern(lines, trimmed, line_idx + 1, ".collect::<");
    if !has_collect {
        return false;
    }
    if is_wrong_primitive_type(type_ann) {
        return true;
    }
    if is_depyler_collection_type(type_ann) {
        let chain_has_into = scan_multiline_for_pattern(lines, trimmed, line_idx + 1, ".into()");
        return !chain_has_into;
    }
    false
}

pub(super) fn fix_collect_type_annotation_mismatch(code: &str) -> String {
    if !code.contains(".collect::<") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if let Some(offset) = let_binding_offset(trimmed) {
            if let Some(new_line) = try_strip_collect_type(line, trimmed, offset, &lines, i) {
                result.push(new_line);
                i += 1;
                continue;
            }
        }

        result.push(line.to_string());
        i += 1;
    }
    result.join("\n")
}

/// Try to strip the type annotation from a let binding with a collect mismatch.
/// Returns Some(new_line) if stripping should happen.
fn try_strip_collect_type(
    line: &str,
    trimmed: &str,
    offset: usize,
    lines: &[&str],
    line_idx: usize,
) -> Option<String> {
    let after_let = &trimmed[offset..];
    let colon_rel = after_let.find(": ")?;
    let eq_rel = after_let.find(" = ")?;
    if colon_rel >= eq_rel {
        return None;
    }
    let type_ann = &after_let[colon_rel + 2..eq_rel];
    if !should_strip_type_annotation(lines, trimmed, line_idx, type_ann) {
        return None;
    }
    let indent = &line[..line.len() - trimmed.len()];
    let before_colon = &trimmed[..offset + colon_rel];
    let after_type = &trimmed[offset + eq_rel..];
    Some(format!("{}{}{}", indent, before_colon, after_type))
}

pub(super) fn fix_empty_vec_in_assert(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut in_assert_block = false;
    let mut assert_paren_depth: i32 = 0;
    for line in code.lines() {
        let trimmed = line.trim();
        let is_assert = trimmed.contains("assert_eq!") || trimmed.contains("assert_ne!");
        // Track multi-line assert blocks
        if is_assert {
            in_assert_block = true;
            assert_paren_depth = 0;
            // Count parens on this line (starting from the assert macro's opening paren)
            for ch in trimmed.chars() {
                match ch {
                    '(' => assert_paren_depth += 1,
                    ')' => assert_paren_depth -= 1,
                    _ => {}
                }
            }
            if assert_paren_depth <= 0 {
                in_assert_block = false;
            }
        } else if in_assert_block {
            for ch in trimmed.chars() {
                match ch {
                    '(' => assert_paren_depth += 1,
                    ')' => assert_paren_depth -= 1,
                    _ => {}
                }
            }
            if assert_paren_depth <= 0 {
                in_assert_block = false;
            }
        }
        // Replace vec![] with Vec::<i32>::new() when inside or on an assert line
        // Only replace standalone `vec![]` (not inside function call args)
        if (is_assert || in_assert_block) && trimmed.contains("vec![]") {
            // Simple case: line is just `vec![]` or `vec![],`
            let just_vec =
                trimmed == "vec![]" || trimmed == "vec![]," || trimmed == "vec![]);";
            if just_vec {
                result.push_str(&line.replace("vec![]", "Vec::<i32>::new()"));
            } else if is_assert {
                // On assert line: use depth-aware replacement
                let macro_start = if let Some(pos) = trimmed.find("assert_eq!(") {
                    pos + 11
                } else if let Some(pos) = trimmed.find("assert_ne!(") {
                    pos + 11
                } else {
                    result.push_str(line);
                    result.push('\n');
                    continue;
                };
                let macro_body = &trimmed[macro_start..];
                let mut new_line = line.to_string();
                let mut depth: i32 = 0;
                let bytes = macro_body.as_bytes();
                let mut i = 0;
                while i < bytes.len() {
                    match bytes[i] {
                        b'(' => depth += 1,
                        b')' => {
                            if depth == 0 {
                                break;
                            }
                            depth -= 1;
                        }
                        b'v' if depth == 0 => {
                            if macro_body[i..].starts_with("vec![]") {
                                let abs_pos = macro_start + i;
                                let before = &trimmed[..abs_pos];
                                let after = &trimmed[abs_pos + 6..];
                                let indent = &line[..line.len() - trimmed.len()];
                                new_line = format!(
                                    "{}{}Vec::<i32>::new(){}",
                                    indent, before, after
                                );
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                result.push_str(&new_line);
            } else {
                // Multi-line assert: replace vec![] if it looks like a top-level arg
                result.push_str(&line.replace("vec![]", "Vec::<i32>::new()"));
            }
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_hashmap_contains_to_contains_key(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Pattern: expr.contains(&key) where expr involves HashMap or .expect("value is None")
        if trimmed.contains(".contains(&")
            && (trimmed.contains(".expect(\"value is None\").")
                || trimmed.contains("HashMap"))
        {
            result.push_str(&line.replace(".contains(&", ".contains_key(&"));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Helper: unwrap Result<T, E> to just T.
pub(super) fn unwrap_result_type(ret: &mut String) {
    if ret.starts_with("Result<") {
        let inner = &ret[7..];
        let mut depth = 0;
        for (i, ch) in inner.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => depth -= 1,
                ',' if depth == 0 => {
                    *ret = inner[..i].trim().to_string();
                    return;
                }
                _ => {}
            }
        }
    }
}

/// Extract the return type string from a line containing `-> RetType {`.
fn extract_return_type_from_arrow(trimmed: &str) -> Option<String> {
    let arrow = trimmed.find("-> ")?;
    let mut ret = trimmed[arrow + 3..]
        .split('{')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    unwrap_result_type(&mut ret);
    Some(ret)
}

/// Build a map of function name -> return type from source code.
/// Handles both single-line and multi-line function signatures.
fn build_fn_return_type_map(lines: &[&str]) -> std::collections::HashMap<String, String> {
    let mut fn_return_types = std::collections::HashMap::new();
    let mut pending_fn_name: Option<String> = None;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            pending_fn_name = None;
            if let Some(fn_name) = extract_fn_name(trimmed) {
                if trimmed.contains("-> ") {
                    if let Some(ret) = extract_return_type_from_arrow(trimmed) {
                        fn_return_types.insert(fn_name, ret);
                    }
                } else {
                    pending_fn_name = Some(fn_name);
                }
            }
        } else if let Some(ref fn_name) = pending_fn_name {
            if trimmed.contains("-> ") {
                if let Some(ret) = extract_return_type_from_arrow(trimmed) {
                    fn_return_types.insert(fn_name.clone(), ret);
                }
                pending_fn_name = None;
            } else if trimmed.contains('{') {
                pending_fn_name = None;
            }
        }
    }
    fn_return_types
}

/// Extract function name from a line starting with `[pub] fn name(`.
fn extract_fn_name(trimmed: &str) -> Option<String> {
    let name_start = trimmed.find("fn ")?;
    let after_fn = &trimmed[name_start + 3..];
    let paren = after_fn.find('(')?;
    Some(after_fn[..paren].trim().to_string())
}

/// Extract the inner type from `Vec<INNER>`, returning `None` if the type
/// is not a Vec or is `Vec<i32>`.
fn extract_non_i32_vec_inner(ret_type: &str) -> Option<&str> {
    if !ret_type.starts_with("Vec<") || ret_type == "Vec<i32>" {
        return None;
    }
    let inner_start = 4; // skip "Vec<"
    let mut depth = 1;
    let mut end = ret_type.len();
    for (i, ch) in ret_type[inner_start..].char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    end = inner_start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    Some(&ret_type[inner_start..end])
}

/// Count paren depth changes in a string.
fn count_paren_depth(s: &str) -> i32 {
    s.chars().fold(0, |d, c| match c {
        '(' => d + 1,
        ')' => d - 1,
        _ => d,
    })
}

/// Find the function name being called on a line, from a set of known functions.
fn find_fn_call_on_line<'a>(
    trimmed: &str,
    fn_names: impl Iterator<Item = &'a String>,
) -> Option<String> {
    for fn_name in fn_names {
        if trimmed.contains(&format!("{}(", fn_name)) {
            return Some(fn_name.clone());
        }
    }
    None
}

/// Replace `Vec::<i32>::new()` with the correctly typed version for a given return type.
fn replace_vec_i32_with_typed(line: &str, ret_type: &str) -> Option<String> {
    let inner = extract_non_i32_vec_inner(ret_type)?;
    let replacement = format!("Vec::<{}>::new()", inner);
    Some(line.replace("Vec::<i32>::new()", &replacement))
}

/// First pass of fix_nested_vec_type_in_assert: fix single-line assert_eq! with fn call.
fn nested_vec_pass1(
    lines: &[&str],
    fn_return_types: &std::collections::HashMap<String, String>,
) -> String {
    let mut result = String::with_capacity(lines.iter().map(|l| l.len() + 1).sum());
    for line in lines {
        let trimmed = line.trim();
        let pushed = trimmed.contains("Vec::<i32>::new()")
            && trimmed.contains("assert_eq!")
            && try_replace_on_assert_line(line, trimmed, fn_return_types, &mut result);
        if !pushed {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Try to replace Vec::<i32>::new() on a single assert line. Returns true if replaced.
fn try_replace_on_assert_line(
    line: &str,
    trimmed: &str,
    fn_return_types: &std::collections::HashMap<String, String>,
    result: &mut String,
) -> bool {
    for (fn_name, ret_type) in fn_return_types {
        if !trimmed.contains(&format!("{}(", fn_name)) {
            continue;
        }
        if let Some(replaced) = replace_vec_i32_with_typed(line, ret_type) {
            result.push_str(&replaced);
            return true;
        }
    }
    false
}

/// Second pass: handle multi-line assert blocks where Vec::<i32>::new() is on its own line.
fn nested_vec_pass2(
    pass1: &str,
    fn_return_types: &std::collections::HashMap<String, String>,
) -> String {
    let pass1_lines: Vec<&str> = pass1.lines().collect();
    let mut result2 = String::with_capacity(pass1.len());
    let mut state = MultiLineAssertState::default();

    for line in &pass1_lines {
        let trimmed = line.trim();
        update_assert_tracking(trimmed, fn_return_types, &mut state);

        if try_replace_multiline_vec(line, trimmed, fn_return_types, &state, &mut result2) {
            continue;
        }
        result2.push_str(line);
        result2.push('\n');
    }
    result2
}

/// State for tracking multi-line assert blocks.
#[derive(Default)]
struct MultiLineAssertState {
    in_assert: bool,
    assert_depth: i32,
    current_assert_fn: Option<String>,
}

/// Update assert tracking state for a line.
fn update_assert_tracking(
    trimmed: &str,
    fn_return_types: &std::collections::HashMap<String, String>,
    state: &mut MultiLineAssertState,
) {
    if trimmed.contains("assert_eq!") || trimmed.contains("assert_ne!") {
        state.in_assert = true;
        state.assert_depth = 0;
        state.current_assert_fn = find_fn_call_on_line(trimmed, fn_return_types.keys());
        state.assert_depth += count_paren_depth(trimmed);
        if state.assert_depth <= 0 {
            state.in_assert = false;
        }
    } else if state.in_assert {
        if state.current_assert_fn.is_none() {
            state.current_assert_fn = find_fn_call_on_line(trimmed, fn_return_types.keys());
        }
        state.assert_depth += count_paren_depth(trimmed);
        if state.assert_depth <= 0 {
            state.in_assert = false;
        }
    }
}

/// Try to replace Vec::<i32>::new() in a multi-line assert context. Returns true if replaced.
fn try_replace_multiline_vec(
    line: &str,
    trimmed: &str,
    fn_return_types: &std::collections::HashMap<String, String>,
    state: &MultiLineAssertState,
    result: &mut String,
) -> bool {
    let is_standalone_vec = trimmed.contains("Vec::<i32>::new()")
        && !trimmed.contains("(&Vec::<i32>::new()")
        && !trimmed.contains("(Vec::<i32>::new()");

    if !is_standalone_vec || !state.in_assert {
        return false;
    }
    let fn_name = match &state.current_assert_fn {
        Some(n) => n,
        None => return false,
    };
    let ret_type = match fn_return_types.get(fn_name) {
        Some(t) => t,
        None => return false,
    };
    if let Some(replaced) = replace_vec_i32_with_typed(line, ret_type) {
        result.push_str(&replaced);
        result.push('\n');
        return true;
    }
    false
}

/// DEPYLER-99MODE-S9: Fix empty vec in assert where nested type expected (E0277).
///
/// When `Vec::<i32>::new()` was inserted by fix_empty_vec_in_assert, but
/// the actual return type is `Vec<Vec<i32>>`, the assert fails.
/// Detect: assert_eq!(fn_call(), Vec::<i32>::new()) where fn returns Vec<Vec<...>>
pub(super) fn fix_nested_vec_type_in_assert(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let fn_return_types = build_fn_return_type_map(&lines);
    let pass1 = nested_vec_pass1(&lines, &fn_return_types);
    nested_vec_pass2(&pass1, &fn_return_types)
}

pub(super) fn fix_vec_arg_type_in_assert(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    // Pass 1: collect function parameter types for Vec<Vec<T>> parameters
    let mut fn_param_types: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let lines: Vec<&str> = code.lines().collect();
    for line in &lines {
        let trimmed = line.trim();
        if !(trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ")) {
            continue;
        }
        if let Some(name_start) = trimmed.find("fn ") {
            let after_fn = &trimmed[name_start + 3..];
            if let Some(paren_open) = after_fn.find('(') {
                let fn_name = after_fn[..paren_open].trim();
                // Strip generic parameters from name
                let fn_name = if let Some(gen) = fn_name.find('<') {
                    &fn_name[..gen]
                } else {
                    fn_name
                };
                // Extract just the parameter section (between parens), not return type
                let after_paren = &after_fn[paren_open + 1..];
                // Find matching close paren, accounting for nested parens
                let mut depth = 1i32;
                let mut params_end = after_paren.len();
                for (i, ch) in after_paren.char_indices() {
                    match ch {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                params_end = i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                let params_section = &after_paren[..params_end];
                // Only check parameters for Vec<Vec<
                if params_section.contains("Vec<Vec<") {
                    let mut param_types = Vec::new();
                    for param in params_section.split(',') {
                        if param.contains("Vec<Vec<") {
                            if let Some(vv_pos) = param.find("Vec<Vec<") {
                                let inner = &param[vv_pos + 8..];
                                if let Some(close) = inner.find('>') {
                                    param_types.push(inner[..close].trim().to_string());
                                }
                            }
                        }
                    }
                    if !param_types.is_empty() {
                        fn_param_types.insert(fn_name.to_string(), param_types);
                    }
                }
            }
        }
    }
    // Pass 2: fix Vec::<T>::new() in function call arguments
    for line in &lines {
        let mut new_line = line.to_string();
        for (fn_name, param_vec_types) in &fn_param_types {
            let pat = format!("{fn_name}(&Vec::<i32>::new())");
            if new_line.contains(&pat) {
                if let Some(inner_type) = param_vec_types.first() {
                    let replacement = format!("{fn_name}(&Vec::<Vec<{inner_type}>>::new())");
                    new_line = new_line.replace(&pat, &replacement);
                }
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

/// Infer the type of a single tuple field from its expression.
fn infer_tuple_field_type(field: &str) -> &'static str {
    let f = field.trim();
    if f.contains(".to_string()") || f.starts_with('"') || f.starts_with("STR_") {
        "String"
    } else if f.parse::<i64>().is_ok() || f.ends_with("i32") {
        "i32"
    } else if f.contains('.') && f.parse::<f64>().is_ok() {
        "f64"
    } else {
        "DepylerValue"
    }
}

/// Try to infer a tuple type string from a tuple expression like `(expr1, expr2)`.
fn infer_tuple_type_from_element(trimmed: &str) -> Option<String> {
    let inner = trimmed.trim_end_matches(',').trim();
    if !inner.starts_with('(') || !inner.ends_with(')') {
        return None;
    }
    let fields = &inner[1..inner.len() - 1];
    let types: Vec<&str> = fields.split(',').map(infer_tuple_field_type).collect();
    if types.is_empty() {
        return None;
    }
    Some(format!("({})", types.join(", ")))
}

/// Scan vec literals typed as Vec<()> and infer replacement tuple types.
fn find_vec_literal_tuple_replacements(lines: &[&str]) -> Vec<(String, String)> {
    let mut replacements = Vec::new();
    let mut in_vec_literal = false;
    let mut vec_type_line: Option<usize> = None;
    let mut first_tuple: Option<String> = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("Vec<()>") && trimmed.contains("vec![") {
            in_vec_literal = true;
            vec_type_line = Some(i);
            first_tuple = None;
        }
        if in_vec_literal && first_tuple.is_none() && trimmed.starts_with('(') {
            first_tuple = infer_tuple_type_from_element(trimmed);
        }
        if in_vec_literal && trimmed == "];" {
            in_vec_literal = false;
            if let (Some(_), Some(ref tuple_type)) = (vec_type_line, &first_tuple) {
                replacements.push(("Vec<()>".to_string(), format!("Vec<{}>", tuple_type)));
            }
            vec_type_line = None;
            first_tuple = None;
        }
    }
    replacements
}

/// Infer tuple type from fn parameter usage patterns (e.g. `.0`, `.1` field access).
fn find_param_tuple_replacements(lines: &[&str]) -> Vec<(String, String)> {
    let mut has_tuple_access = false;
    let mut field_types: Vec<&str> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("op.0") || trimmed.contains("item.0") {
            has_tuple_access = true;
        }
        if has_tuple_access && trimmed.contains(": String = ") && trimmed.contains(".0")
            && !field_types.contains(&"String")
        {
            field_types.push("String");
        }
        if has_tuple_access && trimmed.contains(": i32 = ") && trimmed.contains(".1")
            && !field_types.contains(&"i32")
        {
            field_types.push("i32");
        }
    }
    if has_tuple_access && field_types.len() >= 2 {
        let tuple_type = format!("({})", field_types.join(", "));
        vec![("Vec<()>".to_string(), format!("Vec<{}>", tuple_type))]
    } else {
        Vec::new()
    }
}

pub(super) fn fix_unit_vec_to_tuple_type(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut replacements = find_vec_literal_tuple_replacements(&lines);
    if replacements.is_empty() {
        replacements = find_param_tuple_replacements(&lines);
    }
    if replacements.is_empty() {
        return code.to_string();
    }
    let mut code_str = code.to_string();
    for (old, new) in &replacements {
        code_str = code_str.replace(old.as_str(), new.as_str());
    }
    code_str
}

/// DEPYLER-99MODE-S9: Fix HashMap type annotation mismatch with insert arg order (E0308).
///
/// When `HashMap<K, V>` type annotation has K/V swapped relative to the actual insert calls,
/// fix the type annotation to match the inserts.
#[allow(dead_code)]
pub(super) fn fix_hashmap_type_annotation_mismatch(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    // Find HashMap declarations followed by inserts with mismatched types
    let mut fixes: Vec<(usize, String, String)> = Vec::new(); // (line_idx, old_type, new_type)
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Pattern: `let mut map: HashMap<TYPE_A, TYPE_B> = HashMap::new();`
        if trimmed.contains("let mut map: HashMap<") && trimmed.contains("= HashMap::new()") {
            // Extract declared types
            if let Some(start) = trimmed.find("HashMap<") {
                let after = &trimmed[start + 8..];
                if let Some(gt) = after.find('>') {
                    let types_str = &after[..gt];
                    let parts: Vec<&str> = types_str.splitn(2, ',').collect();
                    if parts.len() == 2 {
                        let declared_k = parts[0].trim();
                        let declared_v = parts[1].trim();
                        // Check if next few lines have inserts with swapped types
                        let mut inserts_suggest_swap = false;
                        for next_line in lines.iter().take(std::cmp::min(i + 10, lines.len())).skip(i + 1) {
                            let next = next_line.trim();
                            if next.starts_with("map.insert(") {
                                // Check if first arg is String-like and second is int-like
                                // when declared types suggest opposite
                                if declared_k == "i32"
                                    && declared_v == "String"
                                    && next.contains(".to_string()")
                                    && next.contains(", ")
                                {
                                    // First arg looks like string, declared key is i32 -> swap needed
                                    if let Some(comma) = next.find(", ") {
                                        let first_arg = &next[12..comma];
                                        if first_arg.contains(".to_string()") {
                                            inserts_suggest_swap = true;
                                        }
                                    }
                                }
                                break;
                            }
                        }
                        if inserts_suggest_swap {
                            let old_type =
                                format!("HashMap<{}, {}>", declared_k, declared_v);
                            let new_type =
                                format!("HashMap<{}, {}>", declared_v, declared_k);
                            fixes.push((i, old_type, new_type));
                        }
                    }
                }
            }
        }
    }
    if fixes.is_empty() {
        return code.to_string();
    }
    // Apply: replace on the specific lines
    let mut code_str = code.to_string();
    for (_, old_type, new_type) in &fixes {
        code_str = code_str.replace(old_type.as_str(), new_type.as_str());
    }
    code_str
}

#[allow(dead_code)]
pub(super) fn fix_push_back_to_push(code: &str) -> String {
    if !code.contains(".push_back(") {
        return code.to_string();
    }
    code.replace(".push_back(", ".push(")
}

/// DEPYLER-99MODE-S9: Replace `Vec::<T>::new()` with `vec![]` inside `assert_eq!`.
///
/// ONLY replaces when the Vec is the ENTIRE content of the line (standalone
/// second argument), not when embedded inside a function call. This prevents
/// breaking type inference for function arguments like `rotate_left(&Vec::<i32>::new(), 5)`.
#[allow(dead_code)]
pub(super) fn fix_empty_vec_new_in_assert(code: &str) -> String {
    if !code.contains("Vec::<") || !code.contains("assert") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut in_assert = false;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("assert_eq!(") || trimmed.starts_with("assert_ne!(") {
            in_assert = true;
        }
        if in_assert {
            // Only replace when Vec::<T>::new() is the ENTIRE line content (with optional comma)
            let content = trimmed.trim_end_matches(',');
            if content.starts_with("Vec::<") && content.ends_with(">::new()") {
                let indent = &line[..line.len() - trimmed.len()];
                let trailing = if trimmed.ends_with(',') { "," } else { "" };
                result.push(format!("{}vec![]{}", indent, trailing));
            } else {
                result.push(line.to_string());
            }
            if trimmed.ends_with(");") {
                in_assert = false;
            }
        } else {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}
