//! Option/Result-related fix functions for post-processing transpiled Rust code.
//!
//! These functions handle common patterns where Option and Result types
//! need adjustments after initial code generation, such as unwrapping,
//! double-wrap prevention, and type-correct assignments.

use super::numeric::{find_call_close_paren, has_result_return_multiline};

pub(super) fn fix_is_none_on_non_option(code: &str) -> String {
    if !code.contains(".is_none()") {
        return code.to_string();
    }

    // DEPYLER-99MODE-E0308: Extract Option parameter names to skip them
    let option_params = extract_option_params(code);

    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Pattern: `if VAR.is_none() {` or `let ... = VAR.is_none()`
        if (trimmed.starts_with("if ") || trimmed.starts_with("let "))
            && trimmed.contains(".is_none()")
        {
            let fixed = fix_is_none_in_line(line, &option_params);
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.pop();
    }
    result
}

/// DEPYLER-99MODE-E0308: Extract parameter names that are Option types.
///
/// Looks for patterns like `param: &mut Option<T>` or `param: Option<T>` in function signatures.
/// Handles multi-line function signatures by collecting the full signature first.
pub(super) fn extract_option_params(code: &str) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut option_params = HashSet::new();
    let lines: Vec<&str> = code.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            let (signature, end_idx) = collect_fn_signature(&lines, i);
            collect_option_params_from_signature(&signature, &mut option_params);
            i = end_idx + 1;
        } else {
            i += 1;
        }
    }
    option_params
}

/// Collect the full function signature (may span multiple lines) starting at `start`.
/// Returns the collected signature string and the index of the last line consumed.
fn collect_fn_signature(lines: &[&str], start: usize) -> (String, usize) {
    let mut signature = String::new();
    let mut j = start;
    let mut paren_depth: i32 = 0;
    let mut found_open_paren = false;
    while j < lines.len() {
        let line = lines[j];
        for ch in line.chars() {
            signature.push(ch);
            if ch == '(' {
                found_open_paren = true;
                paren_depth += 1;
            } else if ch == ')' {
                paren_depth -= 1;
                if paren_depth == 0 && found_open_paren {
                    return (signature, j);
                }
            }
        }
        signature.push(' ');
        j += 1;
    }
    (signature, j.saturating_sub(1))
}

/// Parse parameters from a function signature and insert Option parameter names into the set.
fn collect_option_params_from_signature(
    signature: &str,
    option_params: &mut std::collections::HashSet<String>,
) {
    let paren_start = match signature.find('(') {
        Some(p) => p,
        None => return,
    };
    let paren_end = match signature.rfind(')') {
        Some(p) => p,
        None => return,
    };
    let params_str = &signature[paren_start + 1..paren_end];
    for param in params_str.split(',') {
        let param = param.trim();
        if let Some(colon_pos) = param.find(':') {
            let name = param[..colon_pos].trim();
            let ty = param[colon_pos + 1..].trim();
            if ty.contains("Option<") {
                option_params.insert(name.to_string());
            }
        }
    }
}

pub(super) fn fix_is_none_in_line(line: &str, option_params: &std::collections::HashSet<String>) -> String {
    let mut result = line.to_string();
    // Find VAR.is_none() patterns where VAR doesn't contain Option-like indicators
    while let Some(pos) = result.find(".is_none()") {
        // Walk back to find the variable name
        let before = &result[..pos];
        let var_start = before
            .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
            .map(|p| p + 1)
            .unwrap_or(0);
        let var = &result[var_start..pos];
        // Skip if the variable name suggests it IS an Option (from .get(), etc.)
        if var.contains("get(") || var.contains("unwrap") || var.is_empty() {
            break;
        }
        // DEPYLER-99MODE-E0308: Skip if the variable is a known Option parameter
        if option_params.contains(var) {
            break;
        }
        // Simple variable or field access: replace .is_none() with == false
        // i.e., `config.is_none()` → `false`
        let old = format!("{}.is_none()", var);
        result = result.replacen(&old, "false", 1);
    }
    result
}

pub(super) fn fix_result_double_wrap(code: &str) -> String {
    let result_fns = collect_result_fn_names(code);
    if result_fns.is_empty() {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        let fixed = try_fix_double_wrap_line(line, trimmed, &result_fns);
        if let Some(fixed_line) = fixed {
            result.push_str(&fixed_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.truncate(result.len().saturating_sub(1));
    }
    result
}

/// Collect function names that return Result from both single-line and multi-line signatures.
fn collect_result_fn_names(code: &str) -> std::collections::HashSet<String> {
    let mut result_fns = std::collections::HashSet::new();
    let code_lines: Vec<&str> = code.lines().collect();
    for (idx, line) in code_lines.iter().enumerate() {
        let t = line.trim();
        if !(t.starts_with("pub fn ") || t.starts_with("fn ")) {
            continue;
        }
        let has_result = t.contains("-> Result<") || has_result_return_multiline(&code_lines, idx);
        if !has_result {
            continue;
        }
        if let Some(name) = extract_fn_name_from_sig(t) {
            result_fns.insert(name);
        }
    }
    result_fns
}

/// Extract the function name from a signature line like `pub fn foo(...)` or `fn bar<'a>(...)`.
fn extract_fn_name_from_sig(trimmed: &str) -> Option<String> {
    let start = if trimmed.starts_with("pub fn ") { 7 } else { 3 };
    let rest = &trimmed[start..];
    let paren = rest.find('(')?;
    let raw_name = rest[..paren].trim();
    let name = if let Some(lt) = raw_name.find('<') {
        raw_name[..lt].trim()
    } else {
        raw_name
    };
    if name.is_empty() { None } else { Some(name.to_string()) }
}

/// Try to fix a double-wrapped Ok(result_fn(...)) line. Returns Some(fixed) if fixed.
fn try_fix_double_wrap_line(
    line: &str,
    trimmed: &str,
    result_fns: &std::collections::HashSet<String>,
) -> Option<String> {
    for fname in result_fns {
        let already_q = format!("{}(?", fname);
        for prefix in &[format!("Ok({}(", fname), format!("Ok(!{}(", fname)] {
            if !trimmed.contains(prefix.as_str()) || trimmed.contains(&already_q) {
                continue;
            }
            if let Some(cp) = find_call_close_paren(line, prefix, fname) {
                let mut fixed = String::with_capacity(line.len() + 1);
                fixed.push_str(&line[..cp + 1]);
                fixed.push('?');
                fixed.push_str(&line[cp + 1..]);
                return Some(fixed);
            }
        }
    }
    None
}

pub(super) fn fix_negate_result_fn_call(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        if let Some(fixed) = try_fix_negate_result_line(line, trimmed) {
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Try to fix `Ok(-FUNC(ARGS))` → `Ok(-FUNC(ARGS)?)` on a single line.
fn try_fix_negate_result_line(line: &str, trimmed: &str) -> Option<String> {
    if !trimmed.contains("Ok(-") || !trimmed.ends_with(");") {
        return None;
    }
    let ok_neg_start = trimmed.find("Ok(-")?;
    let after = &trimmed[ok_neg_start + 4..];
    let call_end = find_matching_close_paren_depth(after)?;
    let fn_call = &after[..call_end];
    if !fn_call.contains('(') || !fn_call.contains(')') {
        return None;
    }
    let indent = &line[..line.len() - trimmed.len()];
    let before_ok = &trimmed[..ok_neg_start];
    let after_close = &trimmed[ok_neg_start + 4 + call_end..];
    Some(format!("{}{}Ok(-{}?{}", indent, before_ok, fn_call, after_close))
}

/// Find the position of the closing paren that matches depth 0 (closes the outer call).
fn find_matching_close_paren_depth(s: &str) -> Option<usize> {
    let mut depth: i32 = 0;
    for (j, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return Some(j);
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    None
}

pub(super) fn fix_option_as_cast(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut option_vars: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        // Track: `let var = expr.pop();` — var is Option<T>
        if trimmed.starts_with("let ") && trimmed.contains(".pop()") && trimmed.ends_with(';') {
            if let Some(eq) = trimmed.find(" = ") {
                let var = trimmed[4..eq].trim().trim_start_matches("mut ");
                option_vars.push(var.to_string());
            }
        }
        // Fix .pop()) as u32 on same line
        if trimmed.contains(".pop()") && trimmed.contains("as u32") {
            let new_line = line.replace(".pop())", ".pop().unwrap())");
            result.push_str(&new_line);
        }
        // Fix (option_var) as u32 — add .unwrap()
        else if trimmed.contains("as u32") {
            let mut new_line = line.to_string();
            for var in &option_vars {
                let pat = format!("({}) as u32", var);
                let rep = format!("({}.unwrap()) as u32", var);
                if new_line.contains(&pat) {
                    new_line = new_line.replace(&pat, &rep);
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

pub(super) fn fix_option_dequeue_unwrap(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let option_methods = [".dequeue()", ".pop_front()"];
    let mut option_vars: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        let modified = try_fix_dequeue_typed_let(line, trimmed, &option_methods);
        if let Some(fixed) = modified {
            result.push_str(&fixed);
        } else {
            track_dequeue_option_vars(trimmed, &option_methods, &mut option_vars);
            let new_line = fix_option_var_push_unwrap(line, &option_vars);
            result.push_str(&new_line);
        }
        result.push('\n');
    }
    result
}

/// Case 1: `let var: T = expr.dequeue();` → add .unwrap()
fn try_fix_dequeue_typed_let(line: &str, trimmed: &str, methods: &[&str]) -> Option<String> {
    if !trimmed.starts_with("let ") || !trimmed.contains(':') || !trimmed.ends_with(';') {
        return None;
    }
    for method in methods {
        if !trimmed.contains(method) {
            continue;
        }
        let target = format!("{};", method);
        if trimmed.ends_with(&target) {
            let unwrapped = format!("{}.unwrap();", method);
            return Some(line.replace(&target, &unwrapped));
        }
    }
    None
}

/// Case 2: `let value = expr.dequeue();` → track as Option var
fn track_dequeue_option_vars(trimmed: &str, methods: &[&str], option_vars: &mut Vec<String>) {
    if !trimmed.starts_with("let ") || trimmed.contains(':') || !trimmed.ends_with(';') {
        return;
    }
    for method in methods {
        if !trimmed.contains(method) {
            continue;
        }
        if let Some(eq) = trimmed.find(" = ") {
            let var = trimmed[4..eq].trim().trim_start_matches("mut ").to_string();
            option_vars.push(var);
        }
    }
}

/// Fix uses of option vars in push calls: add .unwrap()
fn fix_option_var_push_unwrap(line: &str, option_vars: &[String]) -> String {
    let mut new_line = line.to_string();
    for var in option_vars {
        let pat1 = format!(".push({});", var);
        let rep1 = format!(".push({}.unwrap());", var);
        if new_line.contains(&pat1) {
            new_line = new_line.replace(&pat1, &rep1);
        }
        let pat2 = format!(".push({})", var);
        let rep2 = format!(".push({}.unwrap())", var);
        if new_line.contains(&pat2) && !new_line.contains(".unwrap())") {
            new_line = new_line.replace(&pat2, &rep2);
        }
    }
    new_line
}

/// DEPYLER-99MODE-S9: Fix `Option<HashMap>.contains_key()` → unwrap first (E0599).
///
/// When a variable is `Option<HashMap>` and `.contains_key()` is called on it,
/// we need to unwrap: `memo.as_ref().map_or(false, |m| m.contains_key(&key))`
pub(super) fn fix_option_hashmap_contains_key(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut option_hashmap_vars: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if let Some(name) = try_extract_option_hashmap_var(trimmed) {
            option_hashmap_vars.push(name);
        }
        let new_line = fix_contains_key_on_option(line, &option_hashmap_vars);
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

/// Try to extract a variable name from a line declaring `Option<HashMap<...>>`.
fn try_extract_option_hashmap_var(trimmed: &str) -> Option<String> {
    if !trimmed.contains("Option<") || !trimmed.contains("HashMap<") {
        return None;
    }
    let colon_pos = trimmed.find(':')?;
    let before = trimmed[..colon_pos].trim();
    let name = before
        .strip_prefix("let ")
        .unwrap_or(before)
        .trim()
        .trim_start_matches("mut ")
        .trim();
    if name.is_empty()
        || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        || name.starts_with("pub ")
        || name.starts_with("fn ")
    {
        return None;
    }
    Some(name.to_string())
}

/// Fix `var.contains_key(&key)` → `var.as_ref().map_or(false, |_ohm| _ohm.contains_key(&key))`.
fn fix_contains_key_on_option(line: &str, option_hashmap_vars: &[String]) -> String {
    let mut new_line = line.to_string();
    for var in option_hashmap_vars {
        let pat = format!("{}.contains_key(", var);
        if !new_line.contains(&pat) {
            continue;
        }
        let Some(start) = new_line.find(&pat) else { continue };
        let after = &new_line[start + pat.len()..];
        let Some(close) = after.find(')') else { continue };
        let key_arg = &after[..close];
        let replacement = format!(
            "{}.as_ref().map_or(false, |_ohm| _ohm.contains_key({}))",
            var, key_arg
        );
        let end_pos = start + pat.len() + close + 1;
        new_line = format!("{}{}{}", &new_line[..start], replacement, &new_line[end_pos..]);
    }
    new_line
}

pub(super) fn fix_option_push_after_is_some(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut option_vars: Vec<String> = Vec::new();
    let mut in_is_some_guard: Vec<(String, i32)> = Vec::new(); // (var_name, brace_depth)
    let mut brace_depth: i32 = 0;
    for line in code.lines() {
        let trimmed = line.trim();
        // Track brace depth
        brace_depth += trimmed.matches('{').count() as i32;
        brace_depth -= trimmed.matches('}').count() as i32;
        // Track variables that have .is_some() called
        if trimmed.contains(".is_some()") {
            // Extract var name: "if VAR.is_some()" or "VAR.is_some()"
            if let Some(pos) = trimmed.find(".is_some()") {
                let before = trimmed[..pos].trim();
                let var = before
                    .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap_or("")
                    .trim();
                if !var.is_empty() {
                    option_vars.push(var.to_string());
                    in_is_some_guard.push((var.to_string(), brace_depth));
                }
            }
        }
        // Clean up guards that are out of scope
        in_is_some_guard.retain(|(_, depth)| *depth <= brace_depth);
        // Fix: result.push(value) → result.push(value.unwrap())
        let mut new_line = line.to_string();
        for (var, _) in &in_is_some_guard {
            let push_pat = format!(".push({var})");
            let push_replace = format!(".push({var}.unwrap())");
            if new_line.contains(&push_pat) && !new_line.contains(&push_replace) {
                new_line = new_line.replace(&push_pat, &push_replace);
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_option_field_assignment(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let option_fields = collect_option_struct_fields(&lines);
    if option_fields.is_empty() {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in &lines {
        let trimmed = line.trim();
        if let Some(fixed) = try_wrap_field_assignment_in_some(line, trimmed, &option_fields) {
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Collect struct fields that are `Option<T>` from struct definitions.
fn collect_option_struct_fields(lines: &[&str]) -> Vec<String> {
    let mut option_fields = Vec::new();
    let mut in_struct = false;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub struct ") {
            in_struct = true;
        } else if in_struct {
            if trimmed == "}" {
                in_struct = false;
            } else if trimmed.contains(": Option<") {
                let field_part = trimmed.trim_start_matches("pub ");
                if let Some(colon) = field_part.find(':') {
                    option_fields.push(field_part[..colon].trim().to_string());
                }
            }
        }
    }
    option_fields
}

/// Try to wrap `self.field = EXPR;` in `Some()` if the field is `Option<T>`.
fn try_wrap_field_assignment_in_some(
    line: &str,
    trimmed: &str,
    option_fields: &[String],
) -> Option<String> {
    for field in option_fields {
        let assign_pat = format!("self.{} = ", field);
        if !trimmed.contains(&assign_pat) {
            continue;
        }
        let pos = trimmed.find(&assign_pat)?;
        let after_assign = &trimmed[pos + assign_pat.len()..];
        let rhs = after_assign.trim_end_matches(';').trim();
        if rhs.starts_with("Some(") || rhs == "None" || rhs.starts_with("None") {
            continue;
        }
        let indent = &line[..line.len() - line.trim_start().len()];
        return Some(format!("{}{}Some({});", indent, assign_pat, rhs));
    }
    None
}

/// DEPYLER-99MODE-S9: Fix `option_var.clone().to_string()` in is_some() guard → `unwrap().to_string()`.
///
/// When inside an `is_some()` guard, `.clone().to_string()` on an Option var should
/// be `.unwrap().to_string()` since the value is guaranteed to be Some.
/// Handles `self.field.clone().is_some()` and `var.is_some()` patterns.
pub(super) fn fix_option_to_string_in_is_some_guard(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut is_some_vars: Vec<String> = Vec::new();
    let mut guard_depth: i32 = 0;
    let mut in_guard = false;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("if ") && trimmed.contains(".is_some()") {
            if let Some(var) = extract_is_some_guard_var(trimmed) {
                is_some_vars.push(var);
                in_guard = true;
                guard_depth = count_brace_delta(trimmed);
            }
        } else if in_guard {
            guard_depth += count_brace_delta(trimmed);
            if guard_depth <= 0 {
                in_guard = false;
                is_some_vars.clear();
            }
        }
        if in_guard {
            result.push_str(&replace_clone_with_unwrap(line, &is_some_vars));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Extract the variable from `if VAR.is_some()` or `if VAR.clone().is_some()`.
fn extract_is_some_guard_var(trimmed: &str) -> Option<String> {
    let is_some_pos = trimmed.find(".is_some()")?;
    let before = &trimmed[3..is_some_pos]; // skip "if "
    let var = before.strip_suffix(".clone()").unwrap_or(before).trim();
    if var.is_empty() { None } else { Some(var.to_string()) }
}

/// Count brace delta (`{` = +1, `}` = -1) for a trimmed line.
fn count_brace_delta(trimmed: &str) -> i32 {
    let mut delta: i32 = 0;
    for ch in trimmed.chars() {
        match ch {
            '{' => delta += 1,
            '}' => delta -= 1,
            _ => {}
        }
    }
    delta
}

/// Replace `var.clone().to_string()` with `var.unwrap().to_string()` for tracked vars.
fn replace_clone_with_unwrap(line: &str, is_some_vars: &[String]) -> String {
    let mut new_line = line.to_string();
    for var in is_some_vars {
        let pat = format!("{}.clone().to_string()", var);
        let rep = format!("{}.unwrap().to_string()", var);
        if new_line.contains(&pat) {
            new_line = new_line.replace(&pat, &rep);
        }
    }
    new_line
}

/// DEPYLER-99MODE-S9: Fix `return value;` in is_some() guard when value is `&Option<T>` param.
///
/// When function takes `value: &Option<T>` and has `if value.is_some() { return value; }`,
/// the return should be `return *value.as_ref().unwrap();`.
pub(super) fn fix_return_option_param_in_is_some(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut option_params: Vec<String> = Vec::new();
    let mut is_some_param: Option<String> = None;
    let mut guard_depth: i32 = 0;
    let mut in_guard = false;
    for line in &lines {
        let trimmed = line.trim();
        if is_fn_with_ref_option_params(trimmed) {
            option_params = extract_ref_option_param_names(trimmed);
        }
        update_is_some_guard_state(
            trimmed,
            &option_params,
            &mut is_some_param,
            &mut guard_depth,
            &mut in_guard,
        );
        if let Some(fixed) = try_fix_return_in_guard(line, trimmed, in_guard, &is_some_param) {
            result.push_str(&fixed);
            result.push('\n');
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// Check if a line is a function signature with `&Option<T>` parameters.
fn is_fn_with_ref_option_params(trimmed: &str) -> bool {
    (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ")) && trimmed.contains("&Option<")
}

/// Extract parameter names that are `&Option<T>` from a function signature line.
fn extract_ref_option_param_names(trimmed: &str) -> Vec<String> {
    let mut names = Vec::new();
    let Some(paren_start) = trimmed.find('(') else { return names };
    let Some(paren_end) = trimmed.rfind(')') else { return names };
    let params = &trimmed[paren_start + 1..paren_end];
    for param in params.split(',') {
        let param = param.trim();
        if param.contains(": &Option<") {
            if let Some(colon) = param.find(':') {
                names.push(param[..colon].trim().to_string());
            }
        }
    }
    names
}

/// Update the is_some guard tracking state for the current line.
fn update_is_some_guard_state(
    trimmed: &str,
    option_params: &[String],
    is_some_param: &mut Option<String>,
    guard_depth: &mut i32,
    in_guard: &mut bool,
) {
    if !option_params.is_empty() && !*in_guard && trimmed.starts_with("if ") && trimmed.contains(".is_some()") {
        for param in option_params {
            if trimmed.contains(&format!("{}.is_some()", param)) {
                *is_some_param = Some(param.clone());
                *in_guard = true;
                *guard_depth = count_brace_delta(trimmed);
                break;
            }
        }
    } else if *in_guard {
        *guard_depth += count_brace_delta(trimmed);
        if *guard_depth <= 0 {
            *in_guard = false;
            *is_some_param = None;
        }
    }
}

/// Try to fix `return param;` → `return *param.as_ref().unwrap();` when inside a guard.
fn try_fix_return_in_guard(
    line: &str,
    trimmed: &str,
    in_guard: bool,
    is_some_param: &Option<String>,
) -> Option<String> {
    if !in_guard {
        return None;
    }
    let param = is_some_param.as_ref()?;
    let pat = format!("return {};", param);
    if trimmed != pat {
        return None;
    }
    let indent = &line[..line.len() - line.trim_start().len()];
    Some(format!("{}return *{}.as_ref().unwrap();", indent, param))
}

/// DEPYLER-99MODE-S9: Fix `let _ = Ok(EXPR);` → `Ok(EXPR)` as tail expression.
///
/// When a function's last expression is `let _ = Ok(expr);`, the Result is discarded.
/// It should be `Ok(expr)` as a tail expression (or `return Ok(expr);`).
pub(super) fn fix_let_discard_ok_return(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(fixed) = try_fix_let_discard_ok(line, trimmed, &lines, i) {
            result.push_str(&fixed);
            result.push('\n');
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// Try to convert `let _ = Ok(EXPR);` to `Ok(EXPR)` if it's followed by `}`.
fn try_fix_let_discard_ok(
    line: &str,
    trimmed: &str,
    lines: &[&str],
    idx: usize,
) -> Option<String> {
    if !trimmed.starts_with("let _ = Ok(") || !trimmed.ends_with(");") {
        return None;
    }
    if !next_nonblank_is_close_brace(lines, idx + 1) {
        return None;
    }
    let indent = &line[..line.len() - line.trim_start().len()];
    let ok_expr = &trimmed[8..trimmed.len() - 1]; // strip "let _ = " and ";"
    Some(format!("{}{}", indent, ok_expr))
}

/// Check if the next non-blank line after `start` is `}`.
fn next_nonblank_is_close_brace(lines: &[&str], start: usize) -> bool {
    for line in lines.iter().skip(start) {
        let next = line.trim();
        if next.is_empty() {
            continue;
        }
        return next == "}";
    }
    false
}

pub(super) fn fix_bare_return_in_result_fn(code: &str) -> String {
    if !code.contains("-> Result<") {
        return code.to_string();
    }
    let result_fns = build_result_fn_set(code);
    let lines: Vec<&str> = code.lines().collect();
    let mut output = Vec::with_capacity(lines.len());
    let mut state = BareReturnState::default();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        update_fn_detection(trimmed, &mut state);
        detect_closure_scope(trimmed, &mut state);
        state.brace_depth += count_brace_delta(trimmed);
        pop_ended_closure_scopes(&mut state);
        check_fn_scope_end(&mut state);
        if let Some(action) = process_bare_return_line(line, trimmed, &result_fns, &mut state) {
            output.push(action);
            i += 1;
            continue;
        }
        output.push(line.to_string());
        i += 1;
    }
    output.join("\n")
}

/// Mutable state for the bare-return fix pass.
#[derive(Default)]
struct BareReturnState {
    in_result_fn: bool,
    brace_depth: i32,
    fn_brace_depth: i32,
    in_bare_return: bool,
    return_paren_depth: i32,
    pending_fn: bool,
    closure_scope_stack: Vec<i32>,
}

/// Build a set of function names that return `Result` (handles multi-line signatures).
fn build_result_fn_set(code: &str) -> std::collections::HashSet<String> {
    let mut result_fns = std::collections::HashSet::new();
    let mut last_fn_name: Option<String> = None;
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            let after_fn = trimmed
                .strip_prefix("pub fn ")
                .unwrap_or(&trimmed[3..]);
            if let Some(paren_pos) = after_fn.find('(') {
                let name = after_fn[..paren_pos].trim().to_string();
                if trimmed.contains("-> Result<") {
                    result_fns.insert(name);
                    last_fn_name = None;
                } else {
                    last_fn_name = Some(name);
                }
            }
        } else if trimmed.contains("-> Result<") {
            if let Some(name) = last_fn_name.take() {
                result_fns.insert(name);
            }
        } else if trimmed.contains('{') || trimmed.is_empty() {
            last_fn_name = None;
        }
    }
    result_fns
}

/// Detect whether the current line starts a Result-returning function or a pending multi-line sig.
fn update_fn_detection(trimmed: &str, state: &mut BareReturnState) {
    if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
        if trimmed.contains("-> Result<") {
            state.in_result_fn = true;
            state.fn_brace_depth = state.brace_depth;
            state.pending_fn = false;
        } else if !trimmed.contains('{') {
            state.pending_fn = true;
        } else {
            state.pending_fn = false;
        }
    } else if state.pending_fn && trimmed.contains("-> Result<") {
        state.in_result_fn = true;
        state.fn_brace_depth = state.brace_depth;
        state.pending_fn = false;
    } else if state.pending_fn && trimmed.contains('{') {
        state.pending_fn = false;
    }
}

/// Detect non-Result closure scopes that should suppress Ok() wrapping.
fn detect_closure_scope(trimmed: &str, state: &mut BareReturnState) {
    if !state.in_result_fn || !trimmed.contains('{') {
        return;
    }
    if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
        return;
    }
    if let Some(last_pipe) = find_closure_last_pipe(trimmed) {
        let after_pipes = &trimmed[last_pipe + 1..];
        if !after_pipes.contains("-> Result<") {
            state.closure_scope_stack.push(state.brace_depth);
        }
    }
}

/// Find the position of the last single `|` in a closure parameter list. Returns None if not a closure.
fn find_closure_last_pipe(trimmed: &str) -> Option<usize> {
    let bytes = trimmed.as_bytes();
    let mut pipe_positions = Vec::new();
    let mut j = 0;
    while j < bytes.len() {
        if bytes[j] == b'|' {
            if j + 1 < bytes.len() && bytes[j + 1] == b'|' {
                j += 2;
                continue;
            }
            pipe_positions.push(j);
        }
        j += 1;
    }
    if pipe_positions.len() >= 2 {
        Some(pipe_positions[pipe_positions.len() - 1])
    } else {
        None
    }
}

/// Pop closure scopes from the stack that have ended (brace depth fell back).
fn pop_ended_closure_scopes(state: &mut BareReturnState) {
    while let Some(&scope_depth) = state.closure_scope_stack.last() {
        if state.brace_depth <= scope_depth {
            state.closure_scope_stack.pop();
        } else {
            break;
        }
    }
}

/// Check if we've exited the current Result function scope.
fn check_fn_scope_end(state: &mut BareReturnState) {
    if state.in_result_fn && state.brace_depth <= state.fn_brace_depth {
        state.in_result_fn = false;
        state.closure_scope_stack.clear();
    }
}

/// Process a line for bare-return wrapping. Returns Some(fixed_line) if handled, None otherwise.
fn process_bare_return_line(
    line: &str,
    trimmed: &str,
    result_fns: &std::collections::HashSet<String>,
    state: &mut BareReturnState,
) -> Option<String> {
    if state.in_bare_return {
        return Some(handle_multiline_bare_return(line, trimmed, state));
    }
    let in_non_result_closure = !state.closure_scope_stack.is_empty();
    if !state.in_result_fn || in_non_result_closure {
        return None;
    }
    try_wrap_single_line_return(line, trimmed, result_fns)
        .or_else(|| try_start_multiline_return(line, trimmed, state))
}

/// Handle continuation of a multi-line bare return being wrapped in Ok().
fn handle_multiline_bare_return(line: &str, trimmed: &str, state: &mut BareReturnState) -> String {
    state.return_paren_depth += count_paren_delta(trimmed);
    if state.return_paren_depth <= 0 && trimmed.ends_with(';') {
        let indent = &line[..line.len() - trimmed.len()];
        let content = trimmed.trim_end_matches(';');
        state.in_bare_return = false;
        format!("{}{});", indent, content)
    } else {
        line.to_string()
    }
}

/// Try to wrap a single-line `return EXPR;` in `Ok()`.
fn try_wrap_single_line_return(
    line: &str,
    trimmed: &str,
    result_fns: &std::collections::HashSet<String>,
) -> Option<String> {
    if !trimmed.starts_with("return ") || !trimmed.ends_with(';') {
        return None;
    }
    let expr = trimmed[7..trimmed.len() - 1].trim();
    if expr.starts_with("Ok(") || expr.starts_with("Err(") {
        return None;
    }
    if calls_result_fn(expr, result_fns) {
        return None;
    }
    let indent = &line[..line.len() - trimmed.len()];
    Some(format!("{}return Ok({});", indent, expr))
}

/// Try to start a multi-line `return (EXPR\n...);` wrapped in `Ok()`.
fn try_start_multiline_return(
    line: &str,
    trimmed: &str,
    state: &mut BareReturnState,
) -> Option<String> {
    if !trimmed.starts_with("return (") || trimmed.ends_with(';') {
        return None;
    }
    let after_return = &trimmed[7..];
    if after_return.starts_with("Ok(") || after_return.starts_with("Err(") {
        return None;
    }
    state.in_bare_return = true;
    state.return_paren_depth = count_paren_delta(after_return);
    let indent = &line[..line.len() - trimmed.len()];
    Some(format!("{}return Ok({}", indent, after_return))
}

/// Check if an expression calls a function that returns Result.
fn calls_result_fn(expr: &str, result_fns: &std::collections::HashSet<String>) -> bool {
    if let Some(paren) = expr.find('(') {
        let called = expr[..paren].trim();
        result_fns.contains(called)
    } else {
        false
    }
}

/// Count parenthesis delta (`(` = +1, `)` = -1).
fn count_paren_delta(s: &str) -> i32 {
    let mut delta: i32 = 0;
    for ch in s.chars() {
        match ch {
            '(' => delta += 1,
            ')' => delta -= 1,
            _ => {}
        }
    }
    delta
}
