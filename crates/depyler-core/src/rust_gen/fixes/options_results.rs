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

    // Collect full function signatures (may span multiple lines)
    let lines: Vec<&str> = code.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Look for function signatures: fn name(...) or pub fn name(...)
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            // Collect the full signature until we find the closing )
            let mut signature = String::new();
            let mut j = i;
            let mut paren_depth = 0;
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
                            break;
                        }
                    }
                }
                if paren_depth == 0 && found_open_paren {
                    break;
                }
                signature.push(' ');
                j += 1;
            }

            // Parse parameters between ( and )
            if let Some(paren_start) = signature.find('(') {
                if let Some(paren_end) = signature.rfind(')') {
                    let params_str = &signature[paren_start + 1..paren_end];
                    // Split by comma (simple parsing, may not handle nested generics perfectly)
                    for param in params_str.split(',') {
                        let param = param.trim();
                        // Pattern: name: ... Option<...>
                        if let Some(colon_pos) = param.find(':') {
                            let name = param[..colon_pos].trim();
                            let ty = param[colon_pos + 1..].trim();
                            // Check if type contains Option
                            if ty.contains("Option<") {
                                option_params.insert(name.to_string());
                            }
                        }
                    }
                }
            }

            i = j + 1;
        } else {
            i += 1;
        }
    }

    option_params
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
    // Pass 1: collect function names that return Result
    // Handles both single-line and multi-line signatures
    let mut result_fns: std::collections::HashSet<String> = std::collections::HashSet::new();
    let code_lines: Vec<&str> = code.lines().collect();
    for (idx, line) in code_lines.iter().enumerate() {
        let t = line.trim();
        if !(t.starts_with("pub fn ") || t.starts_with("fn ")) {
            continue;
        }
        // Check this line AND subsequent lines for -> Result<
        let has_result = t.contains("-> Result<") || has_result_return_multiline(&code_lines, idx);
        if !has_result {
            continue;
        }
        let start = if t.starts_with("pub fn ") { 7 } else { 3 };
        let rest = &t[start..];
        if let Some(paren) = rest.find('(') {
            let raw_name = rest[..paren].trim();
            // Strip generic/lifetime parameters like <'a, T>
            let name = if let Some(lt) = raw_name.find('<') {
                raw_name[..lt].trim()
            } else {
                raw_name
            };
            if !name.is_empty() {
                result_fns.insert(name.to_string());
            }
        }
    }
    if result_fns.is_empty() {
        return code.to_string();
    }
    // Pass 2: find Ok(fn_name( and Ok(!fn_name( patterns, add ? before closing )
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        let mut fixed = false;
        for fname in &result_fns {
            let already_q = format!("{}(?", fname);
            // Pattern 1: Ok(fn_name(...)) → Ok(fn_name(...)?)
            // Pattern 2: Ok(!fn_name(...)) → Ok(!fn_name(...)?)
            for prefix in &[format!("Ok({}(", fname), format!("Ok(!{}(", fname)] {
                if !trimmed.contains(prefix.as_str()) || trimmed.contains(&already_q) {
                    continue;
                }
                if let Some(cp) = find_call_close_paren(line, prefix, fname) {
                    let before = &line[..cp + 1];
                    let after = &line[cp + 1..];
                    result.push_str(before);
                    result.push('?');
                    result.push_str(after);
                    result.push('\n');
                    fixed = true;
                    break;
                }
            }
            if fixed {
                break;
            }
        }
        if !fixed {
            result.push_str(line);
            result.push('\n');
        }
    }
    if code.ends_with('\n') {
        result
    } else {
        result.truncate(result.len().saturating_sub(1));
        result
    }
}

pub(super) fn fix_negate_result_fn_call(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Pattern: `return Ok(-FUNC(ARGS));` where FUNC is called recursively
        // and returns Result, so needs `?` operator
        if trimmed.contains("Ok(-") && trimmed.ends_with(");") {
            // Find the pattern Ok(-func_name(args))
            if let Some(ok_neg_start) = trimmed.find("Ok(-") {
                let after = &trimmed[ok_neg_start + 4..]; // after "Ok(-"
                // Find matching closing paren for the function call
                let mut depth = 0;
                let mut call_end = None;
                for (j, ch) in after.char_indices() {
                    match ch {
                        '(' => depth += 1,
                        ')' => {
                            if depth == 0 {
                                // This closes the Ok(...)
                                call_end = Some(j);
                                break;
                            }
                            depth -= 1;
                        }
                        _ => {}
                    }
                }
                if let Some(end) = call_end {
                    let fn_call = &after[..end]; // e.g., "reverse_number(-n)"
                    // Check if this looks like a function call (has parens)
                    if fn_call.contains('(') && fn_call.contains(')') {
                        // Add ? to unwrap the Result: Ok(-fn(args)?)
                        let indent = &line[..line.len() - trimmed.len()];
                        let before_ok = &trimmed[..ok_neg_start];
                        let after_close = &trimmed[ok_neg_start + 4 + end..];
                        let new_line = format!(
                            "{}{}Ok(-{}?{}",
                            indent, before_ok, fn_call, after_close
                        );
                        result.push_str(&new_line);
                        result.push('\n');
                        continue;
                    }
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
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
        let mut modified = false;
        for method in &option_methods {
            if trimmed.contains(method) {
                // Case 1: `let var: T = expr.dequeue();` → add .unwrap()
                if trimmed.starts_with("let ") && trimmed.contains(':') && trimmed.ends_with(';') {
                    let target = format!("{};", method);
                    if trimmed.ends_with(&target) {
                        let unwrapped = format!("{}.unwrap();", method);
                        result.push_str(&line.replace(&target, &unwrapped));
                        modified = true;
                        break;
                    }
                }
                // Case 2: `let value = expr.dequeue();` → track as Option var
                if trimmed.starts_with("let ") && !trimmed.contains(':') && trimmed.ends_with(';')
                {
                    if let Some(eq) = trimmed.find(" = ") {
                        let var = trimmed[4..eq].trim().trim_start_matches("mut ").to_string();
                        option_vars.push(var);
                    }
                }
            }
        }
        if !modified {
            // Fix uses of option vars: add .unwrap()
            let mut new_line = line.to_string();
            for var in &option_vars {
                // Pattern: `result.push(var)` → `result.push(var.unwrap())`
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
            result.push_str(&new_line);
        }
        result.push('\n');
    }
    result
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
        // Track Option<HashMap> vars
        if trimmed.contains("Option<") && trimmed.contains("HashMap<") {
            if let Some(colon_pos) = trimmed.find(':') {
                let before = trimmed[..colon_pos].trim();
                let name = before
                    .strip_prefix("let ")
                    .unwrap_or(before)
                    .trim()
                    .trim_start_matches("mut ")
                    .trim();
                if !name.is_empty()
                    && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && !name.starts_with("pub ")
                    && !name.starts_with("fn ")
                {
                    option_hashmap_vars.push(name.to_string());
                }
            }
        }
        // Fix: memo.contains_key(&key) → memo.as_ref().map_or(false, |m| m.contains_key(&key))
        let mut new_line = line.to_string();
        let mut modified = false;
        for var in &option_hashmap_vars {
            let pat = format!("{}.contains_key(", var);
            if new_line.contains(&pat) {
                if let Some(start) = new_line.find(&pat) {
                    let after = &new_line[start + pat.len()..];
                    if let Some(close) = after.find(')') {
                        let key_arg = &after[..close]; // e.g., "&n"
                        let replacement = format!(
                            "{}.as_ref().map_or(false, |_ohm| _ohm.contains_key({}))",
                            var, key_arg
                        );
                        let end_pos = start + pat.len() + close + 1;
                        new_line = format!(
                            "{}{}{}",
                            &new_line[..start],
                            replacement,
                            &new_line[end_pos..]
                        );
                        modified = true;
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
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Pass 1: collect struct fields that are Option<T>
    let mut option_fields: Vec<String> = Vec::new();
    let mut in_struct = false;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub struct ") {
            in_struct = true;
        } else if in_struct {
            if trimmed == "}" {
                in_struct = false;
            } else if trimmed.contains(": Option<") {
                // Extract field name: `pub field: Option<i32>,`
                let field_part = trimmed.trim_start_matches("pub ");
                if let Some(colon) = field_part.find(':') {
                    let field_name = field_part[..colon].trim().to_string();
                    option_fields.push(field_name);
                }
            }
        }
    }
    if option_fields.is_empty() {
        return code.to_string();
    }
    // Pass 2: fix assignments to option fields
    for line in &lines {
        let trimmed = line.trim();
        let mut replaced = false;
        for field in &option_fields {
            // Pattern: self.field = EXPR;
            let assign_pat = format!("self.{} = ", field);
            if trimmed.starts_with(&assign_pat) || trimmed.contains(&format!("self.{} = ", field)) {
                // Check if RHS already wrapped in Some() or is None
                let after_assign = if let Some(pos) = trimmed.find(&assign_pat) {
                    &trimmed[pos + assign_pat.len()..]
                } else {
                    continue;
                };
                let rhs = after_assign.trim_end_matches(';').trim();
                if !rhs.starts_with("Some(") && rhs != "None" && !rhs.starts_with("None") {
                    // Wrap in Some()
                    let indent = &line[..line.len() - line.trim_start().len()];
                    let new_rhs = format!("Some({})", rhs);
                    result.push_str(&format!(
                        "{}{}{};",
                        indent, assign_pat, new_rhs
                    ));
                    replaced = true;
                    break;
                }
            }
        }
        if !replaced {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// DEPYLER-99MODE-S9: Fix `option_var.clone().to_string()` in is_some() guard → `unwrap().to_string()`.
///
/// When inside an `is_some()` guard, `.clone().to_string()` on an Option var should
/// be `.unwrap().to_string()` since the value is guaranteed to be Some.
/// Handles `self.field.clone().is_some()` and `var.is_some()` patterns.
pub(super) fn fix_option_to_string_in_is_some_guard(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Track variables that have is_some() guards
    let mut is_some_vars: Vec<String> = Vec::new();
    let mut guard_depth: i32 = 0;
    let mut in_guard = false;
    for line in &lines {
        let trimmed = line.trim();
        // Detect is_some() guard: `if EXPR.is_some() {` or `if EXPR.clone().is_some() {`
        if trimmed.starts_with("if ") && trimmed.contains(".is_some()") {
            // Extract variable expression before .is_some() or .clone().is_some()
            if let Some(is_some_pos) = trimmed.find(".is_some()") {
                let before = &trimmed[3..is_some_pos]; // skip "if "
                // Strip trailing .clone() if present
                let var = if before.ends_with(".clone()") {
                    &before[..before.len() - 8]
                } else {
                    before
                };
                let var = var.trim().to_string();
                if !var.is_empty() {
                    is_some_vars.push(var);
                    in_guard = true;
                    guard_depth = 0;
                    for ch in trimmed.chars() {
                        match ch {
                            '{' => guard_depth += 1,
                            '}' => guard_depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
        } else if in_guard {
            for ch in trimmed.chars() {
                match ch {
                    '{' => guard_depth += 1,
                    '}' => guard_depth -= 1,
                    _ => {}
                }
            }
            if guard_depth <= 0 {
                in_guard = false;
                is_some_vars.clear();
            }
        }
        // Fix: var.clone().to_string() → var.unwrap().to_string() in guard
        if in_guard {
            let mut new_line = line.to_string();
            for var in &is_some_vars {
                let pat = format!("{}.clone().to_string()", var);
                let rep = format!("{}.unwrap().to_string()", var);
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

/// DEPYLER-99MODE-S9: Fix `return value;` in is_some() guard when value is `&Option<T>` param.
///
/// When function takes `value: &Option<T>` and has `if value.is_some() { return value; }`,
/// the return should be `return *value.as_ref().unwrap();`.
pub(super) fn fix_return_option_param_in_is_some(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Track &Option<T> parameters
    let mut option_params: Vec<String> = Vec::new();
    let mut is_some_param: Option<String> = None;
    let mut guard_depth: i32 = 0;
    let mut in_guard = false;
    for line in &lines {
        let trimmed = line.trim();
        // Detect fn with &Option<T> params
        if (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "))
            && trimmed.contains("&Option<")
        {
            option_params.clear();
            // Extract param names that are &Option<T>
            if let Some(paren_start) = trimmed.find('(') {
                if let Some(paren_end) = trimmed.rfind(')') {
                    let params = &trimmed[paren_start + 1..paren_end];
                    for param in params.split(',') {
                        let param = param.trim();
                        if param.contains(": &Option<") {
                            if let Some(colon) = param.find(':') {
                                let name = param[..colon].trim().to_string();
                                option_params.push(name);
                            }
                        }
                    }
                }
            }
        }
        // Detect is_some() guard for option params
        if !option_params.is_empty()
            && trimmed.starts_with("if ")
            && trimmed.contains(".is_some()")
        {
            for param in &option_params {
                if trimmed.contains(&format!("{}.is_some()", param)) {
                    is_some_param = Some(param.clone());
                    in_guard = true;
                    guard_depth = 0;
                    for ch in trimmed.chars() {
                        match ch {
                            '{' => guard_depth += 1,
                            '}' => guard_depth -= 1,
                            _ => {}
                        }
                    }
                    break;
                }
            }
        } else if in_guard {
            for ch in trimmed.chars() {
                match ch {
                    '{' => guard_depth += 1,
                    '}' => guard_depth -= 1,
                    _ => {}
                }
            }
            if guard_depth <= 0 {
                in_guard = false;
                is_some_param = None;
            }
        }
        // Fix: `return param;` → `return *param.as_ref().unwrap();`
        if in_guard {
            if let Some(ref param) = is_some_param {
                let pat = format!("return {};", param);
                if trimmed == pat {
                    let indent = &line[..line.len() - line.trim_start().len()];
                    result.push_str(&format!(
                        "{}return *{}.as_ref().unwrap();",
                        indent, param
                    ));
                    result.push('\n');
                    continue;
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// DEPYLER-99MODE-S9: Fix `let _ = Ok(EXPR);` → `Ok(EXPR)` as tail expression.
///
/// When a function's last expression is `let _ = Ok(expr);`, the Result is discarded.
/// It should be `Ok(expr)` as a tail expression (or `return Ok(expr);`).
pub(super) fn fix_let_discard_ok_return(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let len = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        // Pattern: `let _ = Ok(EXPR);` followed by `}` (end of function)
        if trimmed.starts_with("let _ = Ok(") && trimmed.ends_with(");") {
            // Check if next non-empty line is `}`
            let mut next_is_close = false;
            for j in (i + 1)..len {
                let next = lines[j].trim();
                if next.is_empty() {
                    continue;
                }
                if next == "}" {
                    next_is_close = true;
                }
                break;
            }
            if next_is_close {
                // Convert `let _ = Ok(EXPR);` to `Ok(EXPR)`
                let indent = &line[..line.len() - line.trim_start().len()];
                let ok_expr = &trimmed[8..trimmed.len() - 1]; // strip "let _ = " and ";"
                result.push_str(&format!("{}{}", indent, ok_expr));
                result.push('\n');
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_bare_return_in_result_fn(code: &str) -> String {
    use std::collections::HashSet;
    if !code.contains("-> Result<") {
        return code.to_string();
    }
    // Build set of functions that return Result (to avoid double-wrapping)
    // Handle multi-line signatures: track last fn name, check continuation lines
    let mut result_fns: HashSet<String> = HashSet::new();
    {
        let mut last_fn_name: Option<String> = None;
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
                let after_fn = if trimmed.starts_with("pub fn ") { &trimmed[7..] } else { &trimmed[3..] };
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
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut in_result_fn = false;
    let mut brace_depth: i32 = 0;
    let mut fn_brace_depth: i32 = 0;
    let mut in_bare_return = false;
    let mut return_paren_depth: i32 = 0;
    let mut pending_fn_name: Option<String> = None;
    // Track closure scopes: stack of brace depths where non-Result closures start
    let mut closure_scope_stack: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        // Detect Result-returning function (single or multi-line signature)
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            if trimmed.contains("-> Result<") {
                in_result_fn = true;
                fn_brace_depth = brace_depth;
                pending_fn_name = None;
            } else if !trimmed.contains('{') {
                // Multi-line signature (no { yet, so return type is on next line)
                pending_fn_name = Some("pending".to_string());
            } else {
                pending_fn_name = None;
            }
        } else if pending_fn_name.is_some() && trimmed.contains("-> Result<") {
            in_result_fn = true;
            fn_brace_depth = brace_depth;
            pending_fn_name = None;
        } else if pending_fn_name.is_some() && trimmed.contains('{') {
            pending_fn_name = None;
        }
        // Detect closure starts: lines with |...|...{ that are not fn declarations
        // Only track closures with non-Result return types (those should skip Ok wrapping)
        if in_result_fn
            && !trimmed.starts_with("pub fn ")
            && !trimmed.starts_with("fn ")
            && trimmed.contains('{')
        {
            // Look for closure parameter pattern: paired | characters (not ||)
            let bytes = trimmed.as_bytes();
            let mut pipe_positions = Vec::new();
            let mut j = 0;
            while j < bytes.len() {
                if bytes[j] == b'|' {
                    if j + 1 < bytes.len() && bytes[j + 1] == b'|' {
                        j += 2; // skip ||
                        continue;
                    }
                    pipe_positions.push(j);
                }
                j += 1;
            }
            if pipe_positions.len() >= 2 {
                // Found closure params. Check if return type is Result
                let last_pipe = pipe_positions[pipe_positions.len() - 1];
                let after_pipes = &trimmed[last_pipe + 1..];
                let is_result_closure = after_pipes.contains("-> Result<");
                if !is_result_closure {
                    // Record depth BEFORE this line's braces are counted
                    closure_scope_stack.push(brace_depth);
                }
            }
        }
        for ch in trimmed.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                _ => {}
            }
        }
        // Pop closure scopes that have ended
        while let Some(&scope_depth) = closure_scope_stack.last() {
            if brace_depth <= scope_depth {
                closure_scope_stack.pop();
            } else {
                break;
            }
        }
        if in_result_fn && brace_depth <= fn_brace_depth {
            in_result_fn = false;
            closure_scope_stack.clear();
        }
        // Skip Ok() wrapping inside non-Result closures
        let in_non_result_closure = !closure_scope_stack.is_empty();
        if in_result_fn && !in_bare_return && !in_non_result_closure {
            if trimmed.starts_with("return ") && trimmed.ends_with(';') {
                let expr = trimmed[7..trimmed.len() - 1].trim();
                if !expr.starts_with("Ok(") && !expr.starts_with("Err(") {
                    // Check if expression calls a Result-returning function
                    let calls_result_fn = if let Some(paren) = expr.find('(') {
                        let called = expr[..paren].trim();
                        result_fns.contains(called)
                    } else {
                        false
                    };
                    if !calls_result_fn {
                        let indent = &line[..line.len() - trimmed.len()];
                        result.push(format!("{}return Ok({});", indent, expr));
                        i += 1;
                        continue;
                    }
                }
            } else if trimmed.starts_with("return (") && !trimmed.ends_with(';') {
                let after_return = &trimmed[7..];
                if !after_return.starts_with("Ok(") && !after_return.starts_with("Err(") {
                    in_bare_return = true;
                    return_paren_depth = 0;
                    for ch in after_return.chars() {
                        match ch { '(' => return_paren_depth += 1, ')' => return_paren_depth -= 1, _ => {} }
                    }
                    let indent = &line[..line.len() - trimmed.len()];
                    result.push(format!("{}return Ok({}", indent, after_return));
                    i += 1;
                    continue;
                }
            }
        }
        if in_bare_return {
            for ch in trimmed.chars() {
                match ch { '(' => return_paren_depth += 1, ')' => return_paren_depth -= 1, _ => {} }
            }
            if return_paren_depth <= 0 && trimmed.ends_with(';') {
                let indent = &line[..line.len() - trimmed.len()];
                let content = trimmed.trim_end_matches(';');
                result.push(format!("{}{});", indent, content));
                in_bare_return = false;
            } else {
                result.push(line.to_string());
            }
            i += 1;
            continue;
        }
        result.push(line.to_string());
        i += 1;
    }
    result.join("\n")
}
