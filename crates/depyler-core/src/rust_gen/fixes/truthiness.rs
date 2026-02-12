//! Truthiness-related fix functions for Python-to-Rust transpilation.
//!
//! These functions handle converting Python truthiness semantics (where non-empty
//! collections and strings are truthy) into proper Rust boolean expressions using
//! `.is_empty()` and explicit comparisons.

use super::depyler_value::extract_string_typed_vars;

pub(super) fn fix_python_truthiness(code: &str) -> String {
    // Extract identifiers known to be bool from function signatures
    let bool_vars = extract_bool_typed_vars(code);
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());

    for line in &lines {
        let fixed = fix_truthiness_line(line, &bool_vars);
        result.push(fixed);
    }

    result.join("\n") + "\n"
}

/// Extract variable names that are typed as `bool` from function signatures,
/// local variable declarations, and loop variables over Vec<bool>.
pub(super) fn extract_bool_typed_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    // Track parameter names that are Vec<bool> for loop variable inference
    let mut vec_bool_params: Vec<String> = Vec::new();

    for line in code.lines() {
        let trimmed = line.trim();
        // Extract from function signatures: `fn foo(x: bool, values: &Vec<bool>)`
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.find(')') {
                    let params = &trimmed[start + 1..end];
                    for param in params.split(',') {
                        let p = param.trim();
                        if p.ends_with(": bool") {
                            if let Some(name) = p.strip_suffix(": bool") {
                                let name = name.trim();
                                if !name.is_empty() {
                                    vars.push(name.to_string());
                                }
                            }
                        }
                        // DEPYLER-99MODE-S9: Track Vec<bool> params for loop var inference
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
            }
            continue;
        }
        // DEPYLER-99MODE-S9: Extract from local variable declarations
        // Patterns: `let mut result: bool = ...` or `let result: bool = ...`
        if trimmed.starts_with("let ") {
            let rest = trimmed.strip_prefix("let ").unwrap_or("");
            let rest = rest.strip_prefix("mut ").unwrap_or(rest);
            if let Some(colon_pos) = rest.find(": bool") {
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
        // DEPYLER-99MODE-S9: Extract loop variables over Vec<bool> params
        // Pattern: `for VAR in PARAM.iter()` where PARAM is Vec<bool>
        if trimmed.starts_with("for ") {
            if let Some(in_pos) = trimmed.find(" in ") {
                let loop_var = trimmed[4..in_pos].trim();
                let iter_part = &trimmed[in_pos + 4..];
                for param in &vec_bool_params {
                    if iter_part.starts_with(&format!("{param}."))
                        || iter_part.starts_with(&format!("{param} "))
                    {
                        if !loop_var.is_empty()
                            && loop_var
                                .chars()
                                .all(|c| c.is_alphanumeric() || c == '_')
                        {
                            vars.push(loop_var.to_string());
                        }
                    }
                }
            }
        }
    }
    vars
}

/// Fix a single line's Python truthiness negation patterns.
///
/// Converts `if !identifier {` to `if identifier.is_empty() {` for non-boolean
/// identifiers. Skips known boolean prefixes (is_, has_, should_, etc.).
pub(super) fn fix_truthiness_line(line: &str, bool_vars: &[String]) -> String {
    let trimmed = line.trim();
    // Match pattern: `if !IDENT {` where IDENT is a simple variable name
    if let Some(rest) = trimmed.strip_prefix("if !") {
        if let Some(ident) = rest.strip_suffix(" {") {
            if is_likely_non_boolean_ident(ident, bool_vars) {
                let indent = &line[..line.len() - line.trim_start().len()];
                return format!("{}if {}.is_empty() {{", indent, ident);
            }
        }
    }
    line.to_string()
}

/// Check if an identifier is likely NOT a boolean.
pub(super) fn is_likely_non_boolean_ident(ident: &str, bool_vars: &[String]) -> bool {
    // Must be a simple identifier (no dots, parens, brackets, spaces)
    if ident.contains('.') || ident.contains('(') || ident.contains('[') || ident.contains(' ') {
        return false;
    }
    // Skip variables known to be bool from type annotations
    if bool_vars.iter().any(|v| v == ident) {
        return false;
    }
    // Skip known boolean prefixes
    let bool_prefixes = [
        "is_",
        "has_",
        "should_",
        "can_",
        "will_",
        "was_",
        "did_",
        "does_",
        "are_",
        "do_",
        "were_",
        "ok",
        "err",
        "found",
        "done",
        "valid",
        "enabled",
        "disabled",
        "active",
        "ready",
        "empty",
        "full",
        "true",
        "false",
        "success",
        "failed",
        "_cse_temp",
    ];
    for prefix in &bool_prefixes {
        if ident.starts_with(prefix) || ident == *prefix {
            return false;
        }
    }
    // Must be a valid Rust identifier
    ident.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub(super) fn fix_negation_on_non_bool(code: &str) -> String {
    if !code.contains("!") {
        return code.to_string();
    }
    let string_typed_vars = extract_string_typed_vars(code);
    if string_typed_vars.is_empty() {
        return code.to_string();
    }
    // DEPYLER-99MODE-S9: Exclude variables that are also bool-typed in other scopes.
    // A variable like `v` may be `String` in a From impl and `bool` in a loop.
    // When ambiguous, do NOT rewrite - leaving `!v` for a bool is correct.
    let bool_vars = extract_bool_typed_vars(code);
    let string_typed_vars: Vec<String> = string_typed_vars
        .into_iter()
        .filter(|v| !bool_vars.contains(v))
        .collect();
    if string_typed_vars.is_empty() {
        return code.to_string();
    }
    // Sort by length descending to match longer names first (avoid substring matches)
    let mut sorted_vars = string_typed_vars.clone();
    sorted_vars.sort_by_key(|b: &String| std::cmp::Reverse(b.len()));
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let mut fixed = line.to_string();
        for var in &sorted_vars {
            let neg_pattern = format!("!{}", var);
            // Check that the char AFTER the var name is not alphanumeric (word boundary)
            if let Some(pos) = fixed.find(&neg_pattern) {
                let after_pos = pos + neg_pattern.len();
                let next_char = fixed[after_pos..].chars().next();
                // DEPYLER-99MODE-S9: Don't replace `!var.method()` - that's boolean negation
                // of the method result, NOT truthiness of the variable.
                let is_word_boundary = next_char
                    .map(|c| !c.is_alphanumeric() && c != '_' && c != '.')
                    .unwrap_or(true);
                if is_word_boundary {
                    let empty_check = format!("{}.is_empty()", var);
                    fixed = format!("{}{}{}", &fixed[..pos], empty_check, &fixed[after_pos..]);
                }
            }
        }
        result.push(fixed);
    }
    result.join("\n")
}

pub(super) fn fix_field_access_truthiness(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let fixed = fix_field_negation_in_line(line);
        result.push(fixed);
    }
    result.join("\n")
}

pub(super) fn fix_field_negation_in_line(line: &str) -> String {
    let mut result = line.to_string();
    loop {
        let current = result.clone();
        if let Some(replacement) = find_and_replace_field_negation(&current) {
            result = replacement;
        } else {
            break;
        }
    }
    result
}

pub(super) fn find_and_replace_field_negation(line: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == b'!' {
            // Check if preceded by a valid context (space, `(`, `=`, `{`, start of line)
            let valid_prefix = i == 0
                || matches!(
                    bytes[i - 1],
                    b' ' | b'(' | b'=' | b'{' | b'|' | b'&' | b'\t'
                );
            if !valid_prefix {
                i += 1;
                continue;
            }
            // Parse first identifier after `!`
            let start = i + 1;
            let mut j = start;
            while j < len && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if j == start || j >= len || bytes[j] != b'.' {
                i += 1;
                continue;
            }
            let ident1 = &line[start..j];
            // Parse second identifier after `.`
            j += 1;
            let field_start = j;
            while j < len && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if j == field_start {
                i += 1;
                continue;
            }
            let field = &line[field_start..j];
            // Check what follows: if `(` it's a method call - skip
            if j < len && bytes[j] == b'(' {
                i = j;
                continue;
            }
            // Also skip if it's another `.` (chained access like `!self.field.method()`)
            if j < len && bytes[j] == b'.' {
                i = j;
                continue;
            }
            // DEPYLER-99MODE-S9: Skip numeric tuple indices (e.g., `!bf_result.1`)
            // Tuple field access like `.0`, `.1` returns the element type (often bool),
            // not a collection. Applying `.is_empty()` to a bool is E0599.
            if field.chars().all(|c| c.is_ascii_digit()) {
                i = j;
                continue;
            }
            // Skip known bool-returning or bool-typed fields
            if is_likely_bool_field(field) {
                i = j;
                continue;
            }
            // Replace `!ident.field` with `ident.field.is_empty()`
            let replacement = format!(
                "{}{}.{}.is_empty(){}",
                &line[..i],
                ident1,
                field,
                &line[j..]
            );
            return Some(replacement);
        }
        i += 1;
    }
    None
}

/// DEPYLER-CONVERGE-MULTI-ITER9: Generalize DepylerValue insert wrapping.
///
/// Extends beyond just `map.insert()` to handle `kwargs.insert()`,
/// `config.insert()`, `params.insert()`, and other common variable names.
pub(super) fn is_likely_bool_field(field: &str) -> bool {
    field.starts_with("is_")
        || field.starts_with("has_")
        || field.starts_with("should_")
        || field.starts_with("can_")
        || field.starts_with("enable")
        || field.starts_with("disable")
        || field.starts_with("use_")
        || field.starts_with("load_in_")
        || field.starts_with("allow_")
        || field.starts_with("do_")
        || field.starts_with("with_")
        || field.starts_with("no_")
        || field.starts_with("skip_")
        || field.starts_with("force_")
        || field.starts_with("apply_")
        || field.starts_with("generate_")
        || field.starts_with("include_")
        || field.starts_with("exclude_")
        || field.ends_with("_enabled")
        || field.ends_with("_flag")
        || field.ends_with("_only")
        || field == "verbose"
        || field == "debug"
        || field == "quiet"
        || field == "overwrite"
        || field == "resume"
        || field == "fp16"
        || field == "bf16"
}

pub(super) fn fix_not_string_truthiness(code: &str) -> String {
    let mut result = code.to_string();
    // Pattern: `(!expr.trim().to_string())` -> `expr.trim().is_empty()`
    result = fix_not_trim_to_string(&result);
    // Pattern: `(!expr.to_string())` -> `expr.is_empty()`
    result = fix_not_to_string(&result);
    result
}

pub(super) fn fix_not_trim_to_string(code: &str) -> String {
    let mut result = code.to_string();
    let pattern = ".trim().to_string())";
    while let Some(end_pos) = result.find(pattern) {
        // Walk backwards to find the `(!` that starts this expression
        let before = &result[..end_pos];
        if let Some(start) = before.rfind("(!") {
            let expr = &result[start + 2..end_pos];
            // Only fix if expr looks like a variable/field access
            if expr
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
            {
                let old = format!("(!{}{})", expr, ".trim().to_string()");
                let new = format!("{}.trim().is_empty()", expr);
                result = result.replacen(&old, &new, 1);
                continue;
            }
        }
        break;
    }
    result
}

pub(super) fn fix_not_to_string(code: &str) -> String {
    let mut result = code.to_string();
    let marker = ".to_string())";
    let mut search_from = 0;
    while search_from < result.len() {
        let haystack = &result[search_from..];
        let Some(rel_pos) = haystack.find(marker) else {
            break;
        };
        let end_pos = search_from + rel_pos;
        let before = &result[..end_pos];
        if let Some(start) = before.rfind("(!") {
            let expr = &result[start + 2..end_pos];
            if expr
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
            {
                let old = format!("(!{}.to_string())", expr);
                if result[start..].starts_with(&old) {
                    let new = format!("{}.is_empty()", expr);
                    result = format!(
                        "{}{}{}",
                        &result[..start],
                        new,
                        &result[start + old.len()..]
                    );
                    search_from = start + new.len();
                    continue;
                }
            }
        }
        search_from = end_pos + marker.len();
    }
    result
}

pub(super) fn fix_bitwise_and_truthiness(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("if ")
            && trimmed.contains(" & ")
            && trimmed.ends_with('{')
            && !trimmed.contains("!=")
            && !trimmed.contains("==")
            && !trimmed.contains("&&")
            && !trimmed.contains("||")
            // DEPYLER-99MODE-S9: Skip lines with borrow patterns (& var)
            // to avoid confusing Rust borrow `& var` with bitwise AND `x & mask`
            && !trimmed.contains("(& ")
            && !trimmed.contains(", & ")
        {
            // Extract: "if EXPR {"
            if let Some(rest) = trimmed.strip_prefix("if ") {
                if let Some(expr) = rest.strip_suffix('{') {
                    let expr = expr.trim();
                    // Only fix if expr contains &  and looks like bitwise
                    if expr.contains(" & ") {
                        let indent = line.len() - line.trim_start().len();
                        let pad: String = " ".repeat(indent);
                        result.push_str(&format!("{}if ({}) != 0 {{\n", pad, expr));
                        continue;
                    }
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    if code.ends_with('\n') {
        result
    } else {
        result.truncate(result.len().saturating_sub(1));
        result
    }
}
