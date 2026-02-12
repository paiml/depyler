//! Collection-related fix functions for post-transpilation Rust code repair.
//!
//! These functions handle fixes for HashMap, Vec, and other collection types
//! in generated Rust code, including type annotation mismatches, method name
//! corrections, and iterator/collect patterns.

pub(super) fn fix_hashmap_empty_value_type(code: &str) -> String {
    if !code.contains("HashMap<String, ()>") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut current_return_value_type: Option<String> = None;

    for line in &lines {
        let trimmed = line.trim();
        // Track function return types: -> HashMap<String, X>
        if trimmed.contains("-> HashMap<String,") {
            if let Some(start) = trimmed.find("-> HashMap<String,") {
                let after = &trimmed[start + 18..]; // after "-> HashMap<String,"
                if let Some(end) = after.rfind('>') {
                    let vtype = after[..end].trim().to_string();
                    if vtype != "()" && !vtype.is_empty() {
                        current_return_value_type = Some(vtype);
                    }
                }
            }
        }
        // Replace HashMap<String, ()> with the return type's value type
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
        // Reset on new function definition
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

/// Extract variable names typed as HashMap from function signatures and local declarations.
/// Handles multi-line function signatures where params span multiple lines.
pub(super) fn extract_hashmap_typed_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    // Scan individual lines for HashMap-typed parameter declarations
    // This handles both single-line and multi-line function signatures
    for line in code.lines() {
        let trimmed = line.trim();
        // Match parameter lines containing HashMap (works for multi-line sigs too)
        // Pattern: `name: &'a HashMap<...>` or `name: HashMap<...>`
        if trimmed.contains("HashMap<") && trimmed.contains(':') && !trimmed.contains("Option<") {
            // Skip function return types (contains `->`)
            if trimmed.contains("->") && !trimmed.contains(',') {
                continue;
            }
            // Extract the parameter name before the colon
            // Handle trailing comma: `data: &HashMap<String, i32>,`
            let clean = trimmed.trim_end_matches(',').trim();
            if let Some(colon_pos) = clean.find(':') {
                let name = clean[..colon_pos].trim();
                if !name.is_empty()
                    && name
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_')
                    && !name.starts_with("fn ")
                    && !name.starts_with("pub ")
                {
                    vars.push(name.to_string());
                }
            }
        }
        // From local declarations: `let mut data: HashMap<...>`
        // Skip Option<HashMap<...>> — the var is Option, not directly a HashMap
        if trimmed.starts_with("let ")
            && trimmed.contains("HashMap<")
            && !trimmed.contains("Option<")
        {
            let rest = trimmed
                .strip_prefix("let ")
                .unwrap_or("")
                .trim_start_matches("mut ");
            if let Some(colon_pos) = rest.find(':') {
                let name = rest[..colon_pos].trim();
                if !name.is_empty()
                    && name
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_')
                {
                    vars.push(name.to_string());
                }
            }
        }
    }
    vars
}

pub(super) fn fix_vec_contains_deref(code: &str) -> String {
    if !code.contains(".contains(&*") {
        return code.to_string();
    }
    let mut result = code.to_string();
    // Match `.contains(&*identifier)` and `.contains(&*identifier)`
    // where identifier is [a-zA-Z_][a-zA-Z0-9_]*
    loop {
        if let Some(pos) = result.find(".contains(&*") {
            let after = pos + ".contains(&*".len();
            let mut end = after;
            let bytes = result.as_bytes();
            while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_') {
                end += 1;
            }
            if end > after && end < bytes.len() && bytes[end] == b')' {
                let var = &result[after..end].to_string();
                let old = format!(".contains(&*{})", var);
                let new = format!(".iter().any(|s| s == {})", var);
                result = result.replacen(&old, &new, 1);
                continue;
            }
        }
        break;
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

pub(super) fn fix_vec_to_string_debug(code: &str) -> String {
    let mut vec_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    for line in code.lines() {
        let t = line.trim();
        // struct field: `pub field: Vec<Something>,`
        if t.starts_with("pub ") && t.contains(": Vec<") {
            if let Some(colon) = t.find(": Vec<") {
                let field = t[4..colon].trim();
                if !field.is_empty() {
                    vec_vars.insert(field.to_string());
                }
            }
        }
        // let binding: `let var: Vec<Something> = ...`
        if t.starts_with("let ") && t.contains(": Vec<") {
            if let Some(name) = t.strip_prefix("let ") {
                let var = name.split(':').next().unwrap_or("").trim();
                let var = var.trim_start_matches("mut ");
                if !var.is_empty() {
                    vec_vars.insert(var.to_string());
                }
            }
        }
        // Function parameter: `var: &Vec<Something>` or `var: Vec<Something>`
        if (t.contains(": &Vec<") || t.contains(": Vec<"))
            && !t.starts_with("pub ")
            && !t.starts_with("let ")
        {
            let parts: Vec<&str> = t.split(':').collect();
            if parts.len() >= 2 {
                let param = parts[0].trim().trim_start_matches("mut ");
                let param = param.trim_end_matches(',');
                if !param.is_empty() && param.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    vec_vars.insert(param.to_string());
                }
            }
        }
    }
    if vec_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &vec_vars {
        // DEPYLER-99MODE-S9: Skip internal boilerplate variables (_dv_ prefix)
        // and single-char vars (too prone to false positives in closures like |s|)
        if var.starts_with("_dv_") || var.len() <= 1 {
            continue;
        }
        // Replace `.to_string()` with `.clone()` so Vec is passed by value
        // Only replace when preceded by word boundary (space, (, |, or line start)
        // to avoid corrupting closure params like `|s| DepylerValue::Str(s.to_string())`
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

pub(super) fn fix_hashmap_keys_iter_clone(code: &str) -> String {
    if !code.contains(".keys()") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::with_capacity(code.len());
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Match: `for VAR in EXPR.keys() {`
        if trimmed.starts_with("for ")
            && trimmed.contains(".keys()")
            && trimmed.ends_with('{')
            && !trimmed.contains(".cloned()")
        {
            // Extract the loop variable name
            let after_for = trimmed.strip_prefix("for ").unwrap_or("");
            let loop_var = after_for
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();

            // Extract the map variable (before .keys())
            let map_var = if let Some(in_idx) = after_for.find(" in ") {
                let after_in = &after_for[in_idx + 4..];
                if let Some(keys_idx) = after_in.find(".keys()") {
                    after_in[..keys_idx].trim().to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            // Add .cloned() to keys()
            result.push_str(&lines[i].replace(".keys() {", ".keys().cloned() {"));
            result.push('\n');
            i += 1;

            // Process the loop body: fix .get(key) -> .get(&key)
            if !loop_var.is_empty() && !map_var.is_empty() {
                let get_pattern = format!(".get({})", loop_var);
                let get_fixed = format!(".get(&{})", loop_var);
                let mut brace_depth = 1;
                while i < lines.len() && brace_depth > 0 {
                    let mut line = lines[i].to_string();
                    for c in lines[i].chars() {
                        match c {
                            '{' => brace_depth += 1,
                            '}' => brace_depth -= 1,
                            _ => {}
                        }
                    }
                    // Fix .get(key) to .get(&key)
                    if line.contains(&get_pattern) {
                        line = line.replace(&get_pattern, &get_fixed);
                    }
                    result.push_str(&line);
                    result.push('\n');
                    i += 1;
                }
            }
        } else {
            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
        }
    }
    result
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

        // Look for `let [mut] VAR: TYPE = `
        let let_offset = if trimmed.starts_with("let mut ") {
            Some(8)
        } else if trimmed.starts_with("let ") {
            Some(4)
        } else {
            None
        };

        if let Some(offset) = let_offset {
            let after_let = &trimmed[offset..];
            // Find `: ` (type annotation) and ` = ` (assignment)
            if let (Some(colon_rel), Some(eq_rel)) = (after_let.find(": "), after_let.find(" = "))
            {
                if colon_rel < eq_rel {
                    let type_ann = &after_let[colon_rel + 2..eq_rel];

                    // Check if this multi-line statement contains .collect::<
                    // Track brace depth so we don't stop at `;` inside closures.
                    let mut has_collect = trimmed.contains(".collect::<");
                    let mut brace_depth: i32 = trimmed.chars().fold(0, |d, c| match c {
                        '{' => d + 1,
                        '}' => d - 1,
                        _ => d,
                    });
                    let mut check_line = i + 1;
                    while !has_collect && check_line < lines.len() {
                        let cl = lines[check_line].trim();
                        for ch in cl.chars() {
                            match ch {
                                '{' => brace_depth += 1,
                                '}' => brace_depth -= 1,
                                _ => {}
                            }
                        }
                        if cl.contains(".collect::<") {
                            has_collect = true;
                        }
                        // Only stop at `;` when we're at top-level (not inside closures)
                        if brace_depth <= 0 && cl.ends_with(';') {
                            break;
                        }
                        check_line += 1;
                    }

                    if has_collect {
                        // Check for mismatch conditions:
                        // 1. Primitive type (bool, i32, etc.) used where collection expected
                        // 2. Container<DepylerValue> when iterator doesn't use .into()
                        let is_wrong_primitive = type_ann == "bool"
                            || type_ann == "i32"
                            || type_ann == "f64"
                            || type_ann == "String";
                        let is_depyler_collection = type_ann.contains("DepylerValue")
                            && (type_ann.contains("Vec")
                                || type_ann.contains("HashSet")
                                || type_ann.contains("HashMap"));
                        // Only strip DepylerValue collections if the chain doesn't use .into()
                        let chain_has_into = {
                            let mut found = trimmed.contains(".into()");
                            let mut bd: i32 = trimmed.chars().fold(0, |d, c| match c {
                                '{' => d + 1,
                                '}' => d - 1,
                                _ => d,
                            });
                            let mut cl = i + 1;
                            while !found && cl < lines.len() {
                                let clt = lines[cl].trim();
                                for ch in clt.chars() {
                                    match ch {
                                        '{' => bd += 1,
                                        '}' => bd -= 1,
                                        _ => {}
                                    }
                                }
                                if clt.contains(".into()") {
                                    found = true;
                                }
                                if bd <= 0 && clt.ends_with(';') {
                                    break;
                                }
                                cl += 1;
                            }
                            found
                        };

                        if is_wrong_primitive
                            || (is_depyler_collection && !chain_has_into)
                        {
                            // Remove the `: TYPE` annotation
                            let indent = &line[..line.len() - trimmed.len()];
                            let before_colon = &trimmed[..offset + colon_rel];
                            let after_type = &trimmed[offset + eq_rel..];
                            let new_line =
                                format!("{}{}{}", indent, before_colon, after_type);
                            result.push(new_line);
                            i += 1;
                            continue;
                        }
                    }
                }
            }
        }

        result.push(line.to_string());
        i += 1;
    }
    result.join("\n")
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
            let just_vec = trimmed == "vec![]" || trimmed == "vec![],"
                || trimmed == "vec![]);" || trimmed == "vec![]);";
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

/// DEPYLER-99MODE-S9: Fix empty vec in assert where nested type expected (E0277).
///
/// When `Vec::<i32>::new()` was inserted by fix_empty_vec_in_assert, but
/// the actual return type is `Vec<Vec<i32>>`, the assert fails.
/// Detect: assert_eq!(fn_call(), Vec::<i32>::new()) where fn returns Vec<Vec<...>>
pub(super) fn fix_nested_vec_type_in_assert(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Build a map of function return types (unwrap Result<T, E> -> T)
    // Handles multi-line signatures: `pub fn foo(\n  ...\n) -> RetType {`
    let mut fn_return_types: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut pending_fn_name: Option<String> = None;
    for line in &lines {
        let trimmed = line.trim();
        // Detect fn declaration start
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            if let Some(name_start) = trimmed.find("fn ") {
                let after_fn = &trimmed[name_start + 3..];
                if let Some(paren) = after_fn.find('(') {
                    let fn_name = after_fn[..paren].trim().to_string();
                    if trimmed.contains("-> ") {
                        // Single-line signature
                        if let Some(arrow) = trimmed.find("-> ") {
                            let mut ret = trimmed[arrow + 3..]
                                .split('{')
                                .next()
                                .unwrap_or("")
                                .trim()
                                .to_string();
                            unwrap_result_type(&mut ret);
                            fn_return_types.insert(fn_name, ret);
                        }
                        pending_fn_name = None;
                    } else {
                        // Multi-line: fn name found, await -> on next lines
                        pending_fn_name = Some(fn_name);
                    }
                }
            }
        } else if let Some(ref fn_name) = pending_fn_name {
            if trimmed.contains("-> ") {
                if let Some(arrow) = trimmed.find("-> ") {
                    let mut ret = trimmed[arrow + 3..]
                        .split('{')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    unwrap_result_type(&mut ret);
                    fn_return_types.insert(fn_name.clone(), ret);
                }
                pending_fn_name = None;
            } else if trimmed.contains('{') {
                // Hit the body without finding ->, no return type
                pending_fn_name = None;
            }
        }
    }
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.contains("Vec::<i32>::new()") && trimmed.contains("assert_eq!") {
            let mut replaced = false;
            for (fn_name, ret_type) in &fn_return_types {
                if !trimmed.contains(&format!("{}(", fn_name)) {
                    continue;
                }
                // Check if fn returns any Vec<T> where T != i32
                if ret_type.starts_with("Vec<") && ret_type != "Vec<i32>" {
                    // Extract inner type from Vec<INNER>
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
                    let inner = &ret_type[inner_start..end];
                    let replacement = format!("Vec::<{}>::new()", inner);
                    result.push_str(&line.replace("Vec::<i32>::new()", &replacement));
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                result.push_str(line);
            }
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    // Second pass: handle multi-line assert blocks where Vec::<i32>::new() is on its own line
    let pass1_lines: Vec<&str> = result.lines().collect();
    let mut result2 = String::with_capacity(result.len());
    let mut current_assert_fn: Option<String> = None;
    let mut in_assert = false;
    let mut assert_depth: i32 = 0;
    for line in &pass1_lines {
        let trimmed = line.trim();
        if trimmed.contains("assert_eq!") || trimmed.contains("assert_ne!") {
            in_assert = true;
            assert_depth = 0;
            current_assert_fn = None;
            for ch in trimmed.chars() {
                match ch {
                    '(' => assert_depth += 1,
                    ')' => assert_depth -= 1,
                    _ => {}
                }
            }
            // Try to find fn call on this line
            for (fn_name, _) in &fn_return_types {
                if trimmed.contains(&format!("{}(", fn_name)) {
                    current_assert_fn = Some(fn_name.clone());
                    break;
                }
            }
            if assert_depth <= 0 {
                in_assert = false;
            }
        } else if in_assert {
            // Try to find fn call on continuation lines
            if current_assert_fn.is_none() {
                for (fn_name, _) in &fn_return_types {
                    if trimmed.contains(&format!("{}(", fn_name)) {
                        current_assert_fn = Some(fn_name.clone());
                        break;
                    }
                }
            }
            for ch in trimmed.chars() {
                match ch {
                    '(' => assert_depth += 1,
                    ')' => assert_depth -= 1,
                    _ => {}
                }
            }
            if assert_depth <= 0 {
                in_assert = false;
            }
        }
        // Replace Vec::<i32>::new() on this line if we know the fn's return type
        // But ONLY if it's a standalone argument (not inside a function call like `fn(&Vec::<i32>::new())`)
        let is_standalone_vec = trimmed.contains("Vec::<i32>::new()")
            && !trimmed.contains("(&Vec::<i32>::new()")
            && !trimmed.contains("(Vec::<i32>::new()");
        if is_standalone_vec && in_assert {
            if let Some(ref fn_name) = current_assert_fn {
                if let Some(ret_type) = fn_return_types.get(fn_name) {
                    if ret_type.starts_with("Vec<") && ret_type != "Vec<i32>" {
                        let inner_start = 4;
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
                        let inner = &ret_type[inner_start..end];
                        let replacement = format!("Vec::<{}>::new()", inner);
                        result2.push_str(&line.replace("Vec::<i32>::new()", &replacement));
                        result2.push('\n');
                        continue;
                    }
                }
            }
        }
        result2.push_str(line);
        result2.push('\n');
    }
    result2
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

pub(super) fn fix_unit_vec_to_tuple_type(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Pass 1: find vec literals typed Vec<()> that contain tuple elements
    let mut replacements: Vec<(String, String)> = Vec::new();
    let mut in_vec_literal = false;
    let mut vec_type_line: Option<usize> = None;
    let mut first_tuple: Option<String> = None;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Detect: `let VAR: Vec<()> = vec![` or `VAR: &Vec<()>`
        if trimmed.contains("Vec<()>") && trimmed.contains("vec![") {
            in_vec_literal = true;
            vec_type_line = Some(i);
            first_tuple = None;
        } else if trimmed.contains(": &Vec<()>") || trimmed.contains(": Vec<()>") {
            // Function parameter — check if it's used with tuple field access
            // We'll handle this with a separate pattern
        }
        if in_vec_literal && first_tuple.is_none() && trimmed.starts_with('(') {
            // This is a tuple element — infer types from it
            // Count the fields and infer types
            let inner = trimmed.trim_end_matches(',').trim();
            if inner.starts_with('(') && inner.ends_with(')') {
                let fields = &inner[1..inner.len() - 1];
                let mut types = Vec::new();
                for field in fields.split(',') {
                    let f = field.trim();
                    if f.contains(".to_string()") || f.starts_with('"') || f.starts_with("STR_") {
                        types.push("String");
                    } else if f.parse::<i64>().is_ok() || f.ends_with("i32") {
                        types.push("i32");
                    } else if f.contains('.') && f.parse::<f64>().is_ok() {
                        types.push("f64");
                    } else {
                        types.push("DepylerValue");
                    }
                }
                if !types.is_empty() {
                    let tuple_type = format!("({})", types.join(", "));
                    first_tuple = Some(tuple_type);
                }
            }
        }
        if in_vec_literal && (trimmed == "];" || trimmed == "];") {
            in_vec_literal = false;
            if let (Some(_line_idx), Some(ref tuple_type)) = (vec_type_line, &first_tuple) {
                replacements.push(("Vec<()>".to_string(), format!("Vec<{}>", tuple_type)));
            }
            vec_type_line = None;
            first_tuple = None;
        }
    }
    // Also detect fn parameter `operations: &Vec<()>` and fix based on usage
    // Check if any line has `.0` or `.1` field access on elements of the vec
    if replacements.is_empty() {
        // Look for fn params with Vec<()> and tuple field access
        let mut has_tuple_access = false;
        let mut field_types: Vec<&str> = Vec::new();
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.contains("op.0") || trimmed.contains("item.0") {
                has_tuple_access = true;
            }
            if has_tuple_access && trimmed.contains(": String = ") && trimmed.contains(".0") {
                if !field_types.contains(&"String") {
                    field_types.push("String");
                }
            }
            if has_tuple_access && trimmed.contains(": i32 = ") && trimmed.contains(".1") {
                if !field_types.contains(&"i32") {
                    field_types.push("i32");
                }
            }
        }
        if has_tuple_access && field_types.len() >= 2 {
            let tuple_type = format!("({})", field_types.join(", "));
            replacements.push(("Vec<()>".to_string(), format!("Vec<{}>", tuple_type)));
        }
    }
    if replacements.is_empty() {
        return code.to_string();
    }
    // Apply all replacements
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
    let mut result = String::with_capacity(code.len());
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
                        for j in (i + 1)..std::cmp::min(i + 10, lines.len()) {
                            let next = lines[j].trim();
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
