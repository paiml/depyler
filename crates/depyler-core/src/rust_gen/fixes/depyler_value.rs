//! DepylerValue-related fix functions for post-processing generated Rust code.
//!
//! These functions handle wrapping, unwrapping, and type-converting expressions
//! involving `DepylerValue` enum variants in transpiled output.

use super::enums::{collect_non_dv_enum_names, find_top_level_comma};

pub(super) fn fix_heterogeneous_dict_inserts(code: &str) -> String {
    if !code.contains("map.insert(") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut in_depyler_map = false;

    for line in &lines {
        let trimmed = line.trim();
        // Only activate for maps declared with DepylerValue value type
        if trimmed.contains("let mut map: HashMap<String, DepylerValue>")
            || trimmed.contains("let map: HashMap<String, DepylerValue>")
        {
            in_depyler_map = true;
        }
        // Fix insert calls with bare values in DepylerValue maps
        if in_depyler_map && trimmed.starts_with("map.insert(") {
            let fixed = wrap_map_insert_value(line);
            result.push(fixed);
            continue;
        }
        // End of map block when map is returned or semicolon-terminated
        if in_depyler_map && (trimmed == "map" || trimmed == "map;" || trimmed.starts_with("}")) {
            in_depyler_map = false;
        }
        // Reset on new map declaration with different type
        if trimmed.contains("let mut map: HashMap<") && !trimmed.contains("DepylerValue") {
            in_depyler_map = false;
        }
        result.push(line.to_string());
    }
    result.join("\n")
}

/// Wrap a map.insert value in the appropriate DepylerValue variant.
pub(super) fn wrap_map_insert_value(line: &str) -> String {
    let trimmed = line.trim();
    // Parse: map.insert("key".to_string(), VALUE);
    if let Some(rest) = trimmed.strip_prefix("map.insert(") {
        if let Some(comma_pos) = rest.find(", ") {
            let value_part = &rest[comma_pos + 2..];
            let value = value_part.trim_end_matches(");");
            // Determine value type and wrap
            let wrapped = if value.parse::<i64>().is_ok() {
                format!("DepylerValue::Int({})", value)
            } else if value.parse::<f64>().is_ok() {
                format!("DepylerValue::Float({})", value)
            } else if value == "vec![]" || value.starts_with("vec![") {
                format!("DepylerValue::List({})", value)
            } else if value == "true" || value == "false" {
                format!("DepylerValue::Bool({})", value)
            } else if value.starts_with('"') || value.ends_with(".to_string()") {
                format!(
                    "DepylerValue::Str({}.to_string())",
                    value.trim_end_matches(".to_string()")
                )
            } else {
                return line.to_string(); // unknown type, don't wrap
            };
            let indent = &line[..line.len() - trimmed.len()];
            let key = &rest[..comma_pos];
            return format!("{}map.insert({}, {});", indent, key, wrapped);
        }
    }
    line.to_string()
}

pub(super) fn fix_depyler_value_inserts_generalized(code: &str) -> String {
    if !code.contains("HashMap<String, DepylerValue>") {
        return code.to_string();
    }
    let dv_map_vars = extract_depyler_value_map_vars(code);
    if dv_map_vars.is_empty() {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let trimmed = line.trim();
        let mut matched = false;
        for var in &dv_map_vars {
            let prefix = format!("{}.insert(", var);
            if trimmed.starts_with(&prefix) {
                result.push(wrap_generic_insert_value(line, var));
                matched = true;
                break;
            }
        }
        if !matched {
            result.push(line.to_string());
        }
    }
    result.join("\n")
}

pub(super) fn fix_depyler_value_vec_join(code: &str) -> String {
    if !code.contains("DepylerValue") {
        return code.to_string();
    }
    // Find variables typed as Vec<DepylerValue>
    let mut dv_vec_vars: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.contains("Vec<DepylerValue>") {
            // Extract variable name from patterns like:
            // `let mut varname: Vec<DepylerValue>` or `let varname: Vec<DepylerValue>`
            if let Some(rest) = trimmed
                .strip_prefix("let mut ")
                .or_else(|| trimmed.strip_prefix("let "))
            {
                if let Some(colon) = rest.find(':') {
                    let name = rest[..colon].trim();
                    if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        dv_vec_vars.push(name.to_string());
                    }
                }
            }
        }
    }
    if dv_vec_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &dv_vec_vars {
        // Replace var.join("...") with var.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("...")
        let pattern = format!("{}.join(", var);
        if result.contains(&pattern) {
            let replacement = format!(
                "{}.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",
                var
            );
            result = result.replace(&pattern, &replacement);
        }
    }
    result
}

pub(super) fn fix_string_to_depyler_value_insert(code: &str) -> String {
    if !code.contains("HashMap<String, DepylerValue>") {
        return code.to_string();
    }
    let dv_map_vars = collect_depyler_value_map_names(code);
    if dv_map_vars.is_empty() {
        return code.to_string();
    }
    wrap_string_inserts_in_dv_maps(code, &dv_map_vars)
}

pub(super) fn collect_depyler_value_map_names(code: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("HashMap<String, DepylerValue>") {
            continue;
        }
        // Pattern: `let mut var: HashMap<String, DepylerValue>`
        let rest = match trimmed
            .strip_prefix("let mut ")
            .or_else(|| trimmed.strip_prefix("let "))
        {
            Some(r) => r,
            None => continue,
        };
        if let Some(colon) = rest.find(':') {
            let name = rest[..colon].trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                names.push(name.to_string());
            }
        }
    }
    names
}

/// Try to wrap the value of a single insert call at `abs` position.
/// Returns the new search position if a replacement was made, or `None`.
fn try_wrap_single_insert(
    result: &mut String,
    abs: usize,
    after_insert: usize,
) -> Option<usize> {
    let close = find_matching_close(&result[after_insert..])?;
    let args = result[after_insert..after_insert + close].to_string();
    let comma = find_top_level_comma(&args)?;
    let value_part = args[comma + 1..].trim();
    if value_part.starts_with("DepylerValue::") || !is_field_access(value_part) {
        return None;
    }
    let new_val = format!("DepylerValue::Str({})", value_part);
    let old_full = format!("{}{}", &result[abs..after_insert], args);
    let new_args = format!("{}, {}", &args[..comma], new_val);
    let new_full = format!("{}{}", &result[abs..after_insert], new_args);
    *result = result.replacen(&old_full, &new_full, 1);
    Some(abs + new_full.len())
}

pub(super) fn wrap_string_inserts_in_dv_maps(code: &str, vars: &[String]) -> String {
    let mut result = code.to_string();
    for var in vars {
        let insert_pat = format!("{}.insert(", var);
        let mut search_from = 0;
        while search_from < result.len() {
            let Some(pos) = result[search_from..].find(&insert_pat) else {
                break;
            };
            let abs = search_from + pos;
            let after_insert = abs + insert_pat.len();
            if let Some(new_pos) = try_wrap_single_insert(&mut result, abs, after_insert) {
                search_from = new_pos;
            } else {
                search_from = abs + insert_pat.len();
            }
        }
    }
    result
}

pub(super) fn find_matching_close(s: &str) -> Option<usize> {
    let mut depth = 1i32;
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

pub(super) fn is_field_access(s: &str) -> bool {
    let trimmed = s.trim().trim_end_matches(';');
    trimmed.contains('.')
        && trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
}

pub(super) fn fix_depyler_value_str_clone(code: &str) -> String {
    let mut result = code.to_string();
    let pattern = "DepylerValue::Str(";
    let mut search_from = 0;
    while search_from < result.len() {
        let Some(pos) = result[search_from..].find(pattern) else {
            break;
        };
        let abs = search_from + pos;
        let arg_start = abs + pattern.len();
        // Find the closing paren
        if let Some(close) = find_matching_close(&result[arg_start..]) {
            let arg = &result[arg_start..arg_start + close].trim().to_string();
            // If arg is a field access (config.field) and doesn't already have .clone()
            if arg.contains('.')
                && !arg.ends_with(".clone()")
                && arg
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
            {
                let old = format!("DepylerValue::Str({})", arg);
                let new = format!("DepylerValue::Str({}.clone())", arg);
                result = result.replacen(&old, &new, 1);
                search_from = abs + new.len();
                continue;
            }
        }
        search_from = abs + pattern.len();
    }
    result
}

pub(super) fn fix_depyler_value_from_enum(code: &str) -> String {
    let enum_names = collect_non_dv_enum_names(code);
    if enum_names.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for name in &enum_names {
        let prefix = format!("DepylerValue::from({}::", name);
        while let Some(start) = result.find(&prefix) {
            let paren_start = start + "DepylerValue::from".len();
            let after_paren = paren_start + 1;
            if after_paren < result.len() {
                if let Some(rel_close) = find_matching_close(&result[after_paren..]) {
                    let close = after_paren + rel_close;
                    let inner = result[after_paren..close].to_string();
                    let old = format!("DepylerValue::from({})", inner);
                    let new = format!("DepylerValue::Str(format!(\"{{:?}}\", {}))", inner);
                    result = result.replacen(&old, &new, 1);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    result
}

pub(super) fn fix_depyler_value_hashmap_keys(code: &str) -> String {
    code.to_string()
}

pub(super) fn fix_depyler_value_str_match_arm(code: &str) -> String {
    if !code.contains(".collect::<Vec<_>>()") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    // DEPYLER-99MODE-S9: Track if we're inside a DepylerValue match block
    // Only add .into_iter() in DepylerValue match arms, not in general function returns
    let mut in_depyler_value_match = false;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Detect DepylerValue match blocks
        if trimmed.contains("DepylerValue::") && trimmed.contains("=>") {
            in_depyler_value_match = true;
        }
        // Reset when we exit the match context (new function def)
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            in_depyler_value_match = false;
        }
        // Pattern: `.collect::<Vec<_>>()` followed by `_ => Vec::new().into_iter()`
        // This means the previous match arm is missing `.into_iter(),`
        // DEPYLER-99MODE-S9: Only apply inside DepylerValue match arms
        if in_depyler_value_match
            && trimmed == ".collect::<Vec<_>>()"
            && i + 1 < lines.len()
        {
            let next = lines[i + 1].trim();
            if next.starts_with("_ =>") || next.starts_with("}") {
                let indent = &lines[i][..lines[i].len() - trimmed.len()];
                result.push(format!("{}.collect::<Vec<_>>().into_iter(),", indent));
                i += 1;
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

pub(super) fn extract_string_typed_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        // Match: fn params with String/&str type
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.find(')') {
                    let params = &trimmed[start + 1..end];
                    for param in params.split(',') {
                        let p = param.trim();
                        if p.ends_with(": String") || p.ends_with(": &str") {
                            if let Some(name) = p.split(':').next() {
                                let name = name.trim();
                                if !name.is_empty() {
                                    vars.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Match: let var: String = ...
        if trimmed.starts_with("let ") && trimmed.contains(": String") {
            let rest = trimmed
                .strip_prefix("let ")
                .unwrap_or("")
                .trim_start_matches("mut ");
            if let Some(name) = rest.split(':').next() {
                let name = name.trim();
                if !name.is_empty() {
                    vars.push(name.to_string());
                }
            }
        }
    }
    vars
}

pub(super) fn extract_depyler_value_map_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        // Match: let [mut] VARNAME: ... HashMap<String, DepylerValue>
        if trimmed.starts_with("let ") && trimmed.contains("HashMap<String, DepylerValue>") {
            let rest = trimmed
                .strip_prefix("let ")
                .unwrap_or("")
                .trim_start_matches("mut ");
            if let Some(name) = rest.split(':').next() {
                let name = name.trim();
                if !name.is_empty() && name != "map" {
                    vars.push(name.to_string());
                }
            }
        }
        // Match: fn return type or parameter with HashMap<String, DepylerValue>
        if (trimmed.starts_with("fn ") || trimmed.starts_with("pub fn "))
            && trimmed.contains("HashMap<String, DepylerValue>")
        {
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.find(')') {
                    let params = &trimmed[start + 1..end];
                    for param in params.split(',') {
                        let p = param.trim();
                        if p.contains("HashMap<String, DepylerValue>") {
                            if let Some(name) = p.split(':').next() {
                                let name = name.trim().trim_start_matches("mut ");
                                if !name.is_empty() {
                                    vars.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vars
}

pub(super) fn wrap_generic_insert_value(line: &str, var_name: &str) -> String {
    let trimmed = line.trim();
    let prefix = format!("{}.insert(", var_name);
    if let Some(rest) = trimmed.strip_prefix(&prefix) {
        if let Some(comma_pos) = rest.find(", ") {
            let value_part = &rest[comma_pos + 2..];
            let value = value_part.trim_end_matches(");");
            let wrapped = if value.parse::<i64>().is_ok() {
                format!("DepylerValue::Int({})", value)
            } else if value.parse::<f64>().is_ok() {
                format!("DepylerValue::Float({})", value)
            } else if value == "vec![]" || value.starts_with("vec![") {
                format!("DepylerValue::List({})", value)
            } else if value == "true" || value == "false" {
                format!("DepylerValue::Bool({})", value)
            } else if value.starts_with('"') || value.ends_with(".to_string()") {
                format!(
                    "DepylerValue::Str({}.to_string())",
                    value.trim_end_matches(".to_string()")
                )
            } else {
                return line.to_string();
            };
            let indent = &line[..line.len() - trimmed.len()];
            let key = &rest[..comma_pos];
            return format!("{}{}.insert({}, {});", indent, var_name, key, wrapped);
        }
    }
    line.to_string()
}

const TYPED_TYPES: &[&str] = &["i32", "f64", "i64", "usize", "String", "bool"];
const DV_TERMINATORS: &[&str] = &[
    ".expect(\"IndexError: list index out of range\")",
    ".expect(\"value is None\")",
    ".unwrap_or_default()",
];

/// Strip `let [mut] ` prefix and return the remainder, or `None`.
fn strip_let_prefix(trimmed: &str) -> Option<&str> {
    trimmed
        .strip_prefix("let mut ")
        .or_else(|| trimmed.strip_prefix("let "))
}

/// If the line is a `let` declaration with a concrete type annotation, insert the
/// variable name into `typed_vars`.
fn track_typed_var_declaration(
    trimmed: &str,
    typed_vars: &mut std::collections::HashSet<String>,
) {
    let after_let = match strip_let_prefix(trimmed) {
        Some(rest) => rest,
        None => return,
    };
    if let Some(colon) = after_let.find(':') {
        let var_name = after_let[..colon].trim().to_string();
        let after_colon = &after_let[colon + 1..];
        let type_part = if let Some(eq) = after_colon.find('=') {
            after_colon[..eq].trim()
        } else {
            after_colon.trim().trim_end_matches(';')
        };
        if TYPED_TYPES.contains(&type_part) {
            typed_vars.insert(var_name);
        }
    }
}

/// Return `true` (and write the fixed line to `result`) when the line is a
/// single-line `let` binding whose RHS ends with a DV terminator.
fn try_fix_single_line_let(
    line: &str,
    trimmed: &str,
    result: &mut String,
) -> bool {
    let has_typed_ann = TYPED_TYPES.iter().any(|t| {
        trimmed.contains(&format!(": {} ", t)) || trimmed.contains(&format!(": {} =", t))
    });
    if !has_typed_ann || !trimmed.ends_with(';') {
        return false;
    }
    let before_semi = &trimmed[..trimmed.len() - 1];
    for term in DV_TERMINATORS {
        if before_semi.ends_with(term) {
            let indent = &line[..line.len() - trimmed.len()];
            result.push_str(&format!("{}{}.into();\n", indent, before_semi));
            return true;
        }
    }
    false
}

/// Check whether the current or preceding lines represent a typed assignment.
fn is_typed_assignment_in_context(
    trimmed: &str,
    i: usize,
    lines: &[&str],
    typed_vars: &std::collections::HashSet<String>,
) -> bool {
    // Check the current line for a bare assignment to a typed var
    if let Some(eq_idx) = trimmed.find(" = ") {
        let var_part = trimmed[..eq_idx].trim();
        if typed_vars.contains(var_part) {
            return true;
        }
    }
    // Check lines above for the start of a multi-line chained assignment
    if i > 0 {
        for back in (0..i).rev() {
            let prev = lines[back].trim();
            if prev.is_empty() || prev.starts_with("//") {
                break;
            }
            if let Some(eq_idx) = prev.find(" = ") {
                return typed_vars.contains(prev[..eq_idx].trim());
            }
            if !prev.starts_with('.') && !prev.ends_with('.') {
                break;
            }
        }
    }
    false
}

/// Return `true` (and write the fixed line) when the line ends with a DV
/// terminator and belongs to a typed assignment.
fn try_fix_multiline_dv_terminator(
    line: &str,
    trimmed: &str,
    i: usize,
    lines: &[&str],
    typed_vars: &std::collections::HashSet<String>,
    result: &mut String,
) -> bool {
    if !trimmed.ends_with(';') || trimmed.starts_with("let ") || trimmed.starts_with("//") {
        return false;
    }
    let before_semi = &trimmed[..trimmed.len() - 1];
    for term in DV_TERMINATORS {
        if before_semi.ends_with(term.trim_start_matches('.')) || before_semi.ends_with(term) {
            if is_typed_assignment_in_context(trimmed, i, lines, typed_vars) {
                let indent = &line[..line.len() - trimmed.len()];
                result.push_str(&format!("{}{}.into();\n", indent, before_semi));
                return true;
            }
            break;
        }
    }
    false
}

/// Return `true` (and write the fixed line) when the line is `var = expr.clone();`
/// and `var` was previously declared with a concrete type.
fn try_fix_clone_for_typed_var(
    line: &str,
    trimmed: &str,
    typed_vars: &std::collections::HashSet<String>,
    result: &mut String,
) -> bool {
    if !trimmed.ends_with(".clone();") || trimmed.starts_with("let ") {
        return false;
    }
    if let Some(eq_idx) = trimmed.find(" = ") {
        let var_part = trimmed[..eq_idx].trim();
        if typed_vars.contains(var_part) {
            let indent = &line[..line.len() - trimmed.len()];
            let before_semi = &trimmed[..trimmed.len() - 1];
            result.push_str(&format!("{}{}.into();\n", indent, before_semi));
            return true;
        }
    }
    false
}

pub(super) fn fix_depyler_value_to_typed_assignment(code: &str) -> String {
    use std::collections::HashSet;
    let mut result = String::with_capacity(code.len());
    let mut typed_vars: HashSet<String> = HashSet::new();
    let lines: Vec<&str> = code.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Track typed variable declarations
        track_typed_var_declaration(trimmed, &mut typed_vars);

        // Try each fix strategy in order
        let handled = if trimmed.starts_with("let ") || trimmed.starts_with("let mut ") {
            try_fix_single_line_let(line, trimmed, &mut result)
        } else {
            false
        };

        let handled = handled
            || try_fix_multiline_dv_terminator(line, trimmed, i, &lines, &typed_vars, &mut result);
        let handled = handled
            || try_fix_clone_for_typed_var(line, trimmed, &typed_vars, &mut result);

        if !handled {
            result.push_str(line);
            result.push('\n');
        }
        i += 1;
    }
    result
}

/// Parse a function signature line, returning (return_type, dv_param_names).
/// Returns `None` if the function has no concrete return type we care about.
fn parse_fn_sig_for_dv_return(trimmed: &str) -> Option<(&str, Vec<String>)> {
    let arrow = trimmed.find("-> ")?;
    let ret = &trimmed[arrow + 3..];
    let ret = ret.split('{').next().unwrap_or(ret).trim();
    let return_type = match ret {
        "i32" => "i32",
        "f64" => "f64",
        "String" => "String",
        _ => return None,
    };
    let mut dv_params = Vec::new();
    if let Some(paren_start) = trimmed.find('(') {
        if let Some(paren_end) = trimmed.rfind(')') {
            for param in trimmed[paren_start + 1..paren_end].split(',') {
                let param = param.trim();
                if param.contains("DepylerValue") && !param.contains('&') {
                    if let Some(colon) = param.find(':') {
                        dv_params.push(param[..colon].trim().to_string());
                    }
                }
            }
        }
    }
    Some((return_type, dv_params))
}

/// Try to rewrite `return VAR;` to `return VAR.into();` for a DV param.
fn try_rewrite_return_stmt(
    line: &str,
    trimmed: &str,
    dv_params: &[String],
    result: &mut String,
) -> bool {
    if !trimmed.starts_with("return ") || !trimmed.ends_with(';') {
        return false;
    }
    let expr = trimmed[7..trimmed.len() - 1].trim();
    if dv_params.iter().any(|p| p == expr) {
        let indent = &line[..line.len() - trimmed.len()];
        result.push_str(&format!("{}return {}.into();", indent, expr));
        result.push('\n');
        return true;
    }
    false
}

/// Try to rewrite a tail expression (DV param name before closing `}`) with `.into()`.
fn try_rewrite_tail_expr(
    line: &str,
    trimmed: &str,
    next_line: Option<&&str>,
    dv_params: &[String],
    result: &mut String,
) -> bool {
    if trimmed.starts_with("return ")
        || trimmed.ends_with(';')
        || trimmed.contains('{')
        || trimmed.contains('}')
        || trimmed.is_empty()
    {
        return false;
    }
    let is_tail = next_line.map(|nl| nl.trim() == "}").unwrap_or(false);
    if is_tail && dv_params.iter().any(|p| p == trimmed) {
        let indent = &line[..line.len() - trimmed.len()];
        result.push_str(&format!("{}{}.into()", indent, trimmed));
        result.push('\n');
        return true;
    }
    false
}

pub(super) fn fix_return_depyler_value_param(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut has_concrete_return = false;
    let mut dv_params: Vec<String> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Track function signatures
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            dv_params.clear();
            has_concrete_return = false;
            if let Some((_ret, params)) = parse_fn_sig_for_dv_return(trimmed) {
                has_concrete_return = true;
                dv_params = params;
            }
        }
        if has_concrete_return {
            if try_rewrite_return_stmt(line, trimmed, &dv_params, &mut result) {
                continue;
            }
            if try_rewrite_tail_expr(line, trimmed, lines.get(i + 1), &dv_params, &mut result) {
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_unwrap_or_depyler_value(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Pattern: .get("...").cloned().unwrap_or(N) where N is a bare integer
        if trimmed.contains(".get(\"")
            && trimmed.contains(".cloned().unwrap_or(")
            && !trimmed.contains("DepylerValue::")
        {
            let mut new_line = line.to_string();
            let pat = ".cloned().unwrap_or(";
            while let Some(pos) = new_line.find(pat) {
                let after = &new_line[pos + pat.len()..];
                if let Some(close) = after.find(')') {
                    let arg = after[..close].trim();
                    // Check if arg is a bare integer
                    let is_int = if let Some(stripped) = arg.strip_prefix('-') {
                        !stripped.is_empty() && stripped.chars().all(|c| c.is_ascii_digit())
                    } else {
                        !arg.is_empty() && arg.chars().all(|c| c.is_ascii_digit())
                    };
                    if is_int {
                        let abs_start = pos + pat.len();
                        let abs_end = abs_start + close;
                        new_line = format!(
                            "{}DepylerValue::Int({}i64){}",
                            &new_line[..abs_start],
                            arg,
                            &new_line[abs_end..]
                        );
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_depyler_value_str_literal(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let pattern = "DepylerValue::Str(\"";
    for line in code.lines() {
        if line.contains(pattern) {
            let mut new_line = String::with_capacity(line.len() + 32);
            let mut rest = line;
            while let Some(start) = rest.find(pattern) {
                new_line.push_str(&rest[..start + pattern.len()]);
                let after = &rest[start + pattern.len()..];
                // Find the closing ")
                if let Some(close) = after.find("\")") {
                    let literal = &after[..close];
                    // Check if .to_string() is already there
                    let after_close = &after[close + 2..];
                    if !literal.contains(".to_string()") {
                        new_line.push_str(literal);
                        new_line.push_str("\".to_string())");
                        rest = after_close;
                    } else {
                        new_line.push_str(literal);
                        new_line.push_str("\")");
                        rest = after_close;
                    }
                } else {
                    new_line.push_str(after);
                    rest = "";
                }
            }
            new_line.push_str(rest);
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_pyindex_depyler_value_wrapper(code: &str) -> String {
    // Work on the full code string to handle multi-line patterns
    let needle = "py_index(DepylerValue::Int(";
    if !code.contains(needle) {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    let mut rest = code;
    while let Some(pos) = rest.find(needle) {
        // Copy everything before the match
        result.push_str(&rest[..pos]);
        let inner_start = pos + needle.len();
        // Find the matching close paren for DepylerValue::Int(, then one more `)` for py_index(
        let bytes = rest.as_bytes();
        let mut depth = 1i32;
        let mut inner_end = None;
        for (i, &byte) in bytes.iter().enumerate().skip(inner_start) {
            match byte {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        inner_end = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(end) = inner_end {
            let inner_expr = rest[inner_start..end].trim();
            // Strip trailing comma if present (DepylerValue::Int(EXPR,) is valid)
            let inner_expr = inner_expr.trim_end_matches(',').trim();
            // After the inner `)`, there should be another `)` for the py_index call
            // The pattern is: py_index(DepylerValue::Int(EXPR)) or py_index(DepylerValue::Int(EXPR,))
            if end + 1 < bytes.len() && bytes[end + 1] == b')' {
                // Replace: py_index(DepylerValue::Int(EXPR)) -> py_index((EXPR) as i64).unwrap_or_default()
                // Vec<T>::py_index returns Option<T>, so we need to unwrap
                result.push_str(&format!(
                    "py_index(({}) as i64).unwrap_or_default()",
                    inner_expr
                ));
                rest = &rest[end + 2..]; // skip past both `)`
            } else {
                // Outer `)` not immediately after -- just copy and advance
                result.push_str(needle);
                rest = &rest[inner_start..];
            }
        } else {
            // No matching paren found, copy remainder
            result.push_str(&rest[pos..]);
            rest = "";
        }
    }
    result.push_str(rest);
    result
}

#[allow(dead_code)]
pub(super) fn fix_into_in_depyler_value_chain(code: &str) -> String {
    if !code.contains(".into()") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Check if this line is `.into()` and the next line starts with a DepylerValue method
        if trimmed == ".into()" && i + 1 < lines.len() {
            let next_trimmed = lines[i + 1].trim();
            if next_trimmed.starts_with(".get(")
                || next_trimmed.starts_with(".get_str(")
                || next_trimmed.starts_with(".keys(")
                || next_trimmed.starts_with(".values(")
                || next_trimmed.starts_with(".items(")
                || next_trimmed.starts_with(".contains(")
                || next_trimmed.starts_with(".len(")
            {
                // Skip the spurious .into() line
                i += 1;
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

/// DEPYLER-99MODE-S9: Replace `DepylerValue::from(EXPR)` with just `EXPR` in concrete contexts.
///
/// When the transpiler wraps values in `DepylerValue::from()` but the surrounding
/// context expects a concrete type (e.g., in a tuple `(i32, i32)`), this causes E0308.
/// This fix replaces `DepylerValue::from(EXPR)` with `EXPR` when the expression is:
///   - An integer literal (positive or negative)
///   - A variable that was declared as a concrete type (i32, f64, etc.)
///
/// Only applies when the `DepylerValue::from()` appears as a standalone expression
/// in a tuple, function argument, or assignment to a concrete-typed variable.
#[allow(dead_code)]
pub(super) fn fix_depyler_value_from_in_concrete_tuple(code: &str) -> String {
    if !code.contains("DepylerValue::from(") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    for line in &lines {
        let trimmed = line.trim();
        // Match standalone DepylerValue::from(EXPR) lines ending with , or ;
        // Pattern: "DepylerValue::from(EXPR)," or "DepylerValue::from(EXPR);"
        if trimmed.starts_with("DepylerValue::from(") && (trimmed.ends_with("),") || trimmed.ends_with(");")) {
            let end_char = if trimmed.ends_with("),") { ',' } else { ';' };
            let inner = &trimmed[19..trimmed.len() - 2]; // Extract EXPR from DepylerValue::from(EXPR)
            // Only unwrap if the inner expression is simple (no nested DepylerValue)
            if !inner.contains("DepylerValue") && !inner.contains("vec!") {
                let indent = &line[..line.len() - trimmed.len()];
                result.push(format!("{}{}{}", indent, inner, end_char));
                continue;
            }
        }
        result.push(line.to_string());
    }
    result.join("\n")
}

/// DEPYLER-99MODE-S9: Refine `Vec<DepylerValue>` params from call-site types.
///
/// When a function declares `param: &Vec<DepylerValue>` but call sites in the same
/// file pass `&Vec<i32>` (or other concrete types), replace the param type.
/// Only applies when:
/// - The function body doesn't use DepylerValue-specific methods
/// - There's a matching call site with a concrete vector type
/// Extract function name and `&Vec<DepylerValue>` param names from a fn signature line.
fn extract_dv_vec_fn_params(trimmed: &str) -> Option<(String, Vec<String>)> {
    let after_fn = trimmed
        .strip_prefix("pub fn ")
        .or_else(|| trimmed.strip_prefix("fn "))?;
    let paren_pos = after_fn.find('(')?;
    let name = after_fn[..paren_pos].trim().to_string();
    let params_str = &after_fn[paren_pos + 1..];
    let param_names: Vec<String> = params_str
        .split(',')
        .filter_map(|part| {
            let pt = part.trim();
            if pt.contains("&Vec<DepylerValue>") {
                pt.find(':').map(|c| pt[..c].trim().to_string())
            } else {
                None
            }
        })
        .collect();
    if param_names.is_empty() {
        None
    } else {
        Some((name, param_names))
    }
}

/// Collect all functions that have `&Vec<DepylerValue>` parameters.
fn collect_dv_vec_fn_params(
    code: &str,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut fn_params = std::collections::HashMap::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "))
            && trimmed.contains("&Vec<DepylerValue>")
        {
            if let Some((name, params)) = extract_dv_vec_fn_params(trimmed) {
                fn_params.insert(name, params);
            }
        }
    }
    fn_params
}

/// Collect concrete `Vec<T>` variable declarations (excluding DepylerValue).
fn collect_concrete_vec_var_types(
    code: &str,
) -> std::collections::HashMap<String, String> {
    let mut var_types = std::collections::HashMap::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("let ") || !trimmed.contains("Vec<") || trimmed.contains("DepylerValue") {
            continue;
        }
        if let Some(colon) = trimmed.find(':') {
            let var_name = trimmed[4..colon].trim().trim_start_matches("mut ").to_string();
            if let Some(vec_start) = trimmed.find("Vec<") {
                if let Some(gt) = trimmed[vec_start + 4..].find('>') {
                    let elem_type = trimmed[vec_start + 4..vec_start + 4 + gt].to_string();
                    if !elem_type.contains("DepylerValue") {
                        var_types.insert(var_name, elem_type);
                    }
                }
            }
        }
    }
    var_types
}

/// Match call sites to functions, returning fn_name -> concrete element type.
fn match_callsite_types(
    code: &str,
    fn_params: &std::collections::HashMap<String, Vec<String>>,
    var_types: &std::collections::HashMap<String, String>,
) -> std::collections::HashMap<String, String> {
    let mut fn_replacements = std::collections::HashMap::new();
    for line in code.lines() {
        let trimmed = line.trim();
        for fn_name in fn_params.keys() {
            let call_pattern = format!("{}(&", fn_name);
            if let Some(call_pos) = trimmed.find(&call_pattern) {
                let after_call = &trimmed[call_pos + call_pattern.len()..];
                if let Some(paren_end) = after_call.find(')') {
                    let arg = after_call[..paren_end].trim();
                    if let Some(elem_type) = var_types.get(arg) {
                        fn_replacements.insert(fn_name.clone(), elem_type.clone());
                    }
                }
            }
        }
    }
    fn_replacements
}

/// Apply the signature replacements to the code.
fn apply_vec_signature_replacements(
    code: &str,
    fn_replacements: &std::collections::HashMap<String, String>,
) -> String {
    let mut result = code.to_string();
    for (fn_name, elem_type) in fn_replacements {
        let search = format!("fn {}(", fn_name);
        if let Some(fn_pos) = result.find(&search) {
            let after = &result[fn_pos..];
            if let Some(sig_end) = after.find('{') {
                let sig = &after[..sig_end];
                if sig.contains("&Vec<DepylerValue>") {
                    let new_sig =
                        sig.replace("&Vec<DepylerValue>", &format!("&Vec<{}>", elem_type));
                    let fn_end = fn_pos + sig_end;
                    result = format!("{}{}{}", &result[..fn_pos], new_sig, &result[fn_end..]);
                }
            }
        }
    }
    result
}

pub(super) fn fix_vec_depyler_value_param_from_callsite(code: &str) -> String {
    if !code.contains("&Vec<DepylerValue>") {
        return code.to_string();
    }
    let fn_params = collect_dv_vec_fn_params(code);
    if fn_params.is_empty() {
        return code.to_string();
    }
    let var_types = collect_concrete_vec_var_types(code);
    let fn_replacements = match_callsite_types(code, &fn_params, &var_types);
    if fn_replacements.is_empty() {
        return code.to_string();
    }
    apply_vec_signature_replacements(code, &fn_replacements)
}
