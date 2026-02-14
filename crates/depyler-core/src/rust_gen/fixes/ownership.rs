//! Ownership and borrowing-related post-transpilation fixes.
//!
//! These functions perform string-level corrections on generated Rust code
//! to resolve borrow checker errors, missing `mut` annotations, incorrect
//! dereferences, and move/clone issues.

pub(super) fn fix_borrow_into_iter_chain(code: &str) -> String {
    // Only apply when both chain and collect patterns exist
    if !code.contains(".into_iter().chain(") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut new_lines: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        if line.contains(".into_iter().chain(") && line.contains(".into_iter())") {
            let fixed = line
                .replacen(".into_iter().chain(", ".iter().cloned().chain(", 1)
                .replace(".into_iter())", ".iter().cloned())");
            new_lines.push(fixed);
        } else {
            new_lines.push(line.to_string());
        }
    }
    new_lines.join("\n")
}

// ── fix_borrowed_alias_in_new_calls and helpers ──────────────────────────

/// Collect type aliases that resolve to String (e.g. `type Name = String;`).
fn collect_string_type_aliases(code: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        let rest = match trimmed
            .strip_prefix("pub type ")
            .or_else(|| trimmed.strip_prefix("type "))
        {
            Some(r) => r,
            None => continue,
        };
        if !rest.contains("= String;") && !rest.contains("= String ;") {
            continue;
        }
        let name_end = rest.find(|c: char| !c.is_alphanumeric() && c != '_');
        let name = match name_end {
            Some(e) => &rest[..e],
            None => continue,
        };
        if !name.is_empty() {
            aliases.push(name.to_string());
        }
    }
    aliases
}

/// Find function parameter names whose type is `&Alias` where Alias is a String alias.
fn collect_borrowed_params(code: &str, string_aliases: &[String]) -> Vec<String> {
    let mut borrowed_params = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        for alias in string_aliases {
            if let Some(param) = extract_borrowed_param_name(trimmed, alias) {
                borrowed_params.push(param);
            }
        }
    }
    borrowed_params
}

/// Extract a parameter name from a line that contains `: &Alias`.
fn extract_borrowed_param_name(trimmed: &str, alias: &str) -> Option<String> {
    let pattern = format!(": &{}", alias);
    let pos = trimmed.find(&pattern)?;
    let before = trimmed[..pos].trim();
    let param = before
        .rsplit(|c: char| c == '(' || c == ',' || c.is_whitespace())
        .next()
        .unwrap_or("")
        .trim();
    if !param.is_empty() && param.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Some(param.to_string())
    } else {
        None
    }
}

/// Count paren depth changes in a string.
fn count_paren_depth(s: &str) -> i32 {
    let mut depth: i32 = 0;
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// Add `.clone()` to borrowed params within a line inside a `::new()` call.
fn clone_borrowed_param_in_line(line: &str, borrowed_params: &[String]) -> String {
    let mut modified = line.to_string();
    for param in borrowed_params {
        let patterns = [
            (format!(", {},", param), format!(", {}.clone(),", param)),
            (format!(", {})", param), format!(", {}.clone())", param)),
            (format!("({},", param), format!("({}.clone(),", param)),
            (format!("({})", param), format!("({}.clone())", param)),
        ];
        for (from, to) in &patterns {
            modified = modified.replace(from, to);
        }
        // Handle standalone arg on its own line: `        param,`
        let arg_trimmed = modified.trim();
        if arg_trimmed == format!("{},", param) || arg_trimmed == format!("{})", param) {
            let indent = &modified[..modified.len() - modified.trim_start().len()];
            let suffix = if arg_trimmed.ends_with(',') { "," } else { ")" };
            modified = format!("{}{}.clone(){}", indent, param, suffix);
        }
    }
    modified
}

pub(super) fn fix_borrowed_alias_in_new_calls(code: &str) -> String {
    let string_aliases = collect_string_type_aliases(code);
    if string_aliases.is_empty() {
        return code.to_string();
    }
    let borrowed_params = collect_borrowed_params(code, &string_aliases);
    if borrowed_params.is_empty() {
        return code.to_string();
    }
    // In ::new() calls (single or multi-line), add .clone() to borrowed params
    let mut result = String::new();
    let mut in_new_call = false;
    let mut paren_depth: i32 = 0;
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.contains("::new(") {
            in_new_call = true;
            paren_depth = count_paren_depth(trimmed);
        }
        if in_new_call {
            let modified = clone_borrowed_param_in_line(line, &borrowed_params);
            result.push_str(&modified);
            if !trimmed.contains("::new(") {
                paren_depth += count_paren_depth(trimmed);
            }
            if paren_depth <= 0 {
                in_new_call = false;
            }
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

// ── fix_deref_string_comparison and helpers ──────────────────────────────

/// Check if a `(*var)` at the given position is followed by `== "` or `!= "`.
fn is_deref_comparison(result: &str, abs_pos: usize, var_end: usize) -> bool {
    let after_close = &result[var_end..];
    let trimmed = after_close.trim_start();
    trimmed.starts_with("== \"") || trimmed.starts_with("!= \"")
}

/// Try to replace a `(*var) == "..."` pattern starting at position `i`.
/// Returns `Some((new_result, next_i))` on success, or `None` if no replacement.
fn try_replace_deref_comparison(result: &str, i: usize) -> Option<(String, usize)> {
    let pos = result[i..].find("(*")?;
    let abs_pos = i + pos;
    let after = &result[abs_pos + 2..];
    let close = after.find(')')?;
    let var_name = &after[..close];
    // Check it's a simple identifier
    if !var_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    {
        return None;
    }
    let var_end = abs_pos + 2 + close + 1;
    if !is_deref_comparison(result, abs_pos, var_end) {
        return None;
    }
    let old = format!("(*{})", var_name);
    let new = var_name.to_string();
    let replaced = format!(
        "{}{}{}",
        &result[..abs_pos],
        new,
        &result[abs_pos + old.len()..]
    );
    Some((replaced, abs_pos + new.len()))
}

/// DEPYLER-CONVERGE-MULTI-ITER11: Fix deref string comparisons.
///
/// The transpiler generates `(*var) == "literal"` which dereferences `&String`
/// to `str`, but `str == &str` has no implementation. Remove the unnecessary `*`.
/// Pattern: `(*identifier) == "` -> `identifier == "`
pub(super) fn fix_deref_string_comparison(code: &str) -> String {
    let mut result = code.to_string();
    let mut i = 0;
    while i < result.len() {
        if let Some((new_result, next_i)) = try_replace_deref_comparison(&result, i) {
            result = new_result;
            i = next_i;
        } else if result[i..].contains("(*") {
            // Advance past the "(*" that didn't match
            i += result[i..].find("(*").map_or(result.len() - i, |p| p + 2);
        } else {
            break;
        }
    }
    result
}

pub(super) fn fix_deref_unwrap_result(code: &str) -> String {
    let mut result = code.to_string();
    for method in &[
        "unwrap_or_default()",
        "unwrap()",
        "unwrap_or(0)",
        "unwrap_or(0.0)",
    ] {
        let search = format!(".{}", method);
        let mut i = 0;
        while i < result.len() {
            let Some(pos) = result[i..].find(&search) else {
                break;
            };
            let abs = i + pos;
            let end = abs + search.len();
            // Check if followed by `)` and preceded by `(*`
            if end < result.len()
                && result.as_bytes()[end] == b')'
                && abs >= 2
                && &result[abs - 2..abs] == "(*"
            {
                // Remove `(*` prefix and `)` suffix, keeping `VAR.method()`
                let inner = &result[abs..end];
                let new = inner.to_string();
                result = format!("{}{}{}", &result[..abs - 2], new, &result[end + 1..]);
                i = abs - 2 + new.len();
                continue;
            }
            i = abs + search.len();
        }
    }
    result
}

pub(super) fn fix_ref_option_in_new(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let option_params = collect_ref_option_params(&lines);
    if option_params.is_empty() {
        return code.to_string();
    }
    apply_deref_in_new_calls(code, &option_params)
}

pub(super) fn collect_ref_option_params(lines: &[&str]) -> Vec<String> {
    let mut params = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.contains("Option<") || !trimmed.ends_with(',') {
            continue;
        }
        let Some(colon) = trimmed.find(':') else {
            continue;
        };
        let type_part = trimmed[colon + 1..].trim();
        let is_ref_option = (type_part.starts_with("&Option<") || type_part.starts_with("&'"))
            && type_part.contains("Option<");
        if !is_ref_option {
            continue;
        }
        let name = trimmed[..colon].trim().trim_start_matches("mut ");
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            params.push(name.to_string());
        }
    }
    params.sort();
    params.dedup();
    params
}

pub(super) fn apply_deref_in_new_calls(code: &str, params: &[String]) -> String {
    let mut result = String::with_capacity(code.len());
    let mut in_new_call = false;
    let mut paren_depth: i32 = 0;
    for line in code.lines() {
        let mut fixed_line = line.to_string();
        if line.contains("::new(") && !in_new_call {
            in_new_call = true;
            paren_depth = 0;
        }
        if in_new_call {
            paren_depth += count_paren_depth(line);
            for param in params {
                fixed_line = deref_param_in_new_call_line(&fixed_line, param);
            }
            if paren_depth <= 0 {
                in_new_call = false;
            }
        }
        result.push_str(&fixed_line);
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.pop();
    }
    result
}

/// Replace a param reference with `*param` when it appears as a standalone arg in a ::new() call.
fn deref_param_in_new_call_line(line: &str, param: &str) -> String {
    let trimmed = line.trim();
    if trimmed == format!("{},", param) {
        line.replace(&format!("{},", param), &format!("*{},", param))
    } else if trimmed == format!("{})", param) {
        line.replace(&format!("{})", param), &format!("*{})", param))
    } else {
        line.to_string()
    }
}

/// Fix `(*ref_option.unwrap_or_default())` where ref_option is `&Option<T>`.
/// Deref the reference first: `(*VAR).unwrap_or_default()` (works for Copy types).
pub(super) fn fix_deref_ref_option_unwrap(code: &str) -> String {
    let mut result = code.to_string();
    let search = ".unwrap_or_default())";
    let mut i = 0;
    while i < result.len() {
        let Some(pos) = result[i..].find(search) else {
            break;
        };
        let abs = i + pos;
        if abs < 2 {
            i = abs + search.len();
            continue;
        }
        let before = &result[..abs];
        if let Some(star_pos) = before.rfind("(*") {
            let var = result[star_pos + 2..abs].trim().to_string();
            if var
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
            {
                let old = format!("(*{}.unwrap_or_default())", var);
                let new = format!("(*{}).unwrap_or_default()", var);
                result = result.replacen(&old, &new, 1);
                i = star_pos + new.len();
                continue;
            }
        }
        i = abs + search.len();
    }
    result
}

// ── fix_immutable_ref_to_mut and helpers ─────────────────────────────────

/// Collect a map of function name -> Vec<param_index> for &mut parameters.
fn collect_mut_param_map(code: &str) -> std::collections::HashMap<String, Vec<usize>> {
    use std::collections::HashMap;
    let mut mut_params: HashMap<String, Vec<usize>> = HashMap::new();
    for line in code.lines() {
        let t = line.trim();
        if !(t.starts_with("fn ") || t.starts_with("pub fn ")) {
            continue;
        }
        if let Some((name, positions)) = extract_mut_param_positions(t) {
            if !positions.is_empty() {
                mut_params.insert(name, positions);
            }
        }
    }
    mut_params
}

/// Check if a call to `fname` at position `pos` in `line_str` is a word-boundary match.
fn is_word_boundary_call(line_str: &str, pos: usize) -> bool {
    if pos == 0 {
        return true;
    }
    let prev_char = line_str.as_bytes()[pos - 1] as char;
    !prev_char.is_alphanumeric() && prev_char != '_'
}

pub(super) fn fix_immutable_ref_to_mut(code: &str) -> String {
    let mut_params = collect_mut_param_map(code);
    if mut_params.is_empty() {
        return code.to_string();
    }
    // Pass 2: Fix call sites
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Skip function definitions
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            result.push_str(line);
            result.push('\n');
            continue;
        }
        let mut line_str = line.to_string();
        for (fname, positions) in &mut_params {
            let pat = format!("{}(", fname);
            if let Some(pos) = line_str.find(&pat) {
                if !is_word_boundary_call(&line_str, pos) {
                    continue;
                }
            } else {
                continue;
            }
            line_str = fix_mut_args_in_call(&line_str, fname, positions);
        }
        result.push_str(&line_str);
        result.push('\n');
    }
    if code.ends_with('\n') {
        result
    } else {
        result.truncate(result.len().saturating_sub(1));
        result
    }
}

/// Extract function name and positions of `&mut` parameters.
pub(super) fn extract_mut_param_positions(sig: &str) -> Option<(String, Vec<usize>)> {
    let start = if sig.starts_with("pub fn ") {
        7
    } else if sig.starts_with("fn ") {
        3
    } else {
        return None;
    };
    let rest = &sig[start..];
    let paren = rest.find('(')?;
    let raw_name = rest[..paren].trim();
    let name = if let Some(lt) = raw_name.find('<') {
        raw_name[..lt].trim()
    } else {
        raw_name
    };
    if name.is_empty() {
        return None;
    }
    let after = &rest[paren + 1..];
    let close = after.find(')')?;
    let params = &after[..close];
    let positions = find_mut_param_positions(params);
    Some((name.to_string(), positions))
}

/// Parse a comma-separated parameter list and return indices of `&mut` params.
fn find_mut_param_positions(params: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut idx = 0usize;
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in params.chars() {
        match ch {
            '<' | '(' => {
                depth += 1;
                current.push(ch);
            }
            '>' | ')' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                if current.contains("&mut ") {
                    positions.push(idx);
                }
                idx += 1;
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if current.contains("&mut ") {
        positions.push(idx);
    }
    positions
}

// ── fix_mut_args_in_call and helpers ─────────────────────────────────────

/// Split a call argument string into individual args respecting nesting depth.
fn split_args_at_depth_zero(args_str: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut d = 0i32;
    let mut cur = String::new();
    for ch in args_str.chars() {
        match ch {
            '(' | '<' | '[' => {
                d += 1;
                cur.push(ch);
            }
            ')' | '>' | ']' => {
                d -= 1;
                cur.push(ch);
            }
            ',' if d == 0 => {
                args.push(cur.clone());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        args.push(cur);
    }
    args
}

/// Find the end of the argument list (matching `)`) starting after the opening `(`.
fn find_call_args_end(line: &str, args_start: usize) -> usize {
    let mut depth = 1i32;
    let mut args_end = args_start;
    for (i, c) in line[args_start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    args_end = args_start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    args_end
}

/// Fix a single call site: change `&arg` to `&mut arg` at specified positions.
pub(super) fn fix_mut_args_in_call(line: &str, fname: &str, positions: &[usize]) -> String {
    let pat = format!("{}(", fname);
    let call_pos = match line.find(&pat) {
        Some(p) => {
            if !is_word_boundary_call(line, p) {
                return line.to_string();
            }
            p
        }
        None => return line.to_string(),
    };
    let args_start = call_pos + pat.len();
    let args_end = find_call_args_end(line, args_start);
    let args_str = &line[args_start..args_end];
    let mut args = split_args_at_depth_zero(args_str);
    let mut changed = false;
    for &pos in positions {
        if pos < args.len() {
            if let Some(replacement) = promote_ref_to_mut(&args[pos]) {
                args[pos] = replacement;
                changed = true;
            }
        }
    }
    if !changed {
        return line.to_string();
    }
    format!(
        "{}{}{}",
        &line[..args_start],
        args.join(","),
        &line[args_end..]
    )
}

/// If an argument starts with `&` but not `&mut `, return the `&mut` version.
fn promote_ref_to_mut(arg: &str) -> Option<String> {
    let trimmed = arg.trim();
    if !trimmed.starts_with('&') || trimmed.starts_with("&mut ") {
        return None;
    }
    let ws = arg.len() - arg.trim_start().len();
    let prefix: String = arg.chars().take(ws).collect();
    Some(format!("{}&mut {}", prefix, &trimmed[1..]))
}

// ── fix_deref_expect_on_primitive and helpers ────────────────────────────

/// Try to fix a single `(*var).expect("...")` pattern in a line.
/// Returns `Some(fixed_line)` if a replacement was made, `None` otherwise.
fn try_fix_deref_expect_in_line(line: &str) -> Option<String> {
    let star_pos = line.find("(*")?;
    let after_star = &line[star_pos + 2..];
    let close_paren = after_star.find(')')?;
    let var_name = &after_star[..close_paren];
    if var_name.is_empty()
        || !var_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }
    let after_close = &after_star[close_paren + 1..];
    let expect_content = after_close.strip_prefix(".expect(")?;
    let end = find_matching_paren(expect_content)?;
    let rest = &expect_content[end + 1..];
    Some(format!("{}{}{}", &line[..star_pos], var_name, rest))
}

pub(super) fn fix_deref_expect_on_primitive(code: &str) -> String {
    if !code.contains("(*") || !code.contains(".expect(") {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut fixed_line = line.to_string();
        // Repeatedly apply the fix until no more patterns match
        while let Some(new_line) = try_fix_deref_expect_in_line(&fixed_line) {
            fixed_line = new_line;
        }
        result.push_str(&fixed_line);
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.truncate(result.len().saturating_sub(1));
    }
    result
}

// ── fix_ref_arg_to_fn_string_param and helpers ──────────────────────────

/// Collect parameter names whose type is `impl Fn(String)`.
fn collect_fn_string_param_names(code: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("impl Fn(String)") {
            continue;
        }
        let Some(idx) = trimmed.find(": impl Fn(String)") else {
            continue;
        };
        let before = &trimmed[..idx];
        let Some(name) = before.split(['(', ',']).next_back() else {
            continue;
        };
        let name = name.trim();
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            names.push(name.to_string());
        }
    }
    names
}

/// Replace `name(&var)` calls with `name(var.clone())` for a single fn param name.
fn replace_ref_calls_with_clone(result: &str, name: &str) -> String {
    let re_pattern = format!(r"{}(&", name);
    if !result.contains(&re_pattern) {
        return result.to_string();
    }
    let mut new_result = String::with_capacity(result.len());
    let mut pos = 0;
    while let Some(idx) = result[pos..].find(&re_pattern) {
        let abs_idx = pos + idx;
        new_result.push_str(&result[pos..abs_idx]);
        new_result.push_str(&format!("{}(", name));
        let after = abs_idx + re_pattern.len();
        if let Some(close) = result[after..].find(')') {
            let var = &result[after..after + close];
            new_result.push_str(&format!("{}.clone())", var));
            pos = after + close + 1;
        } else {
            new_result.push_str(&result[after..]);
            pos = result.len();
        }
    }
    new_result.push_str(&result[pos..]);
    new_result
}

pub(super) fn fix_ref_arg_to_fn_string_param(code: &str) -> String {
    if !code.contains("impl Fn(String)") {
        return code.to_string();
    }
    let fn_param_names = collect_fn_string_param_names(code);
    if fn_param_names.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for name in &fn_param_names {
        result = replace_ref_calls_with_clone(&result, name);
    }
    result
}

// ── fix_double_expect_on_option_ref and helpers ─────────────────────────

/// Try to fix a double-expect pattern in a single trimmed line.
/// Returns `Some((prefix_text, first_expect_result, suffix_text))` on success.
fn try_parse_double_expect(trimmed: &str) -> Option<(String, String, String)> {
    if !trimmed.contains("(*") || trimmed.matches(".expect(").count() < 2 {
        return None;
    }
    let star_idx = trimmed.find("(*")?;
    let exp1_offset = trimmed[star_idx + 2..].find(".expect(")?;
    let var_name = &trimmed[star_idx + 2..star_idx + 2 + exp1_offset];
    if !var_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }
    let exp1_start = star_idx + 2 + exp1_offset;
    let quote1_start = trimmed[exp1_start..].find('"')?;
    let abs_q1 = exp1_start + quote1_start;
    let quote1_end = trimmed[abs_q1 + 1..].find('"')?;
    let exp1_close = abs_q1 + 1 + quote1_end + 1;
    if exp1_close >= trimmed.len() || trimmed.as_bytes()[exp1_close] != b')' {
        return None;
    }
    let first_expect_result = trimmed[star_idx + 2..exp1_close + 1].to_string();
    let after = &trimmed[exp1_close + 1..];
    if !after.starts_with(").expect(") {
        return None;
    }
    let exp2_start = exp1_close + 1 + 1; // skip )
    let exp2_close = trimmed[exp2_start..].find("\")")?;
    let prefix = trimmed[..star_idx].to_string();
    let suffix = trimmed[exp2_start + exp2_close + 2..].to_string();
    Some((prefix, first_expect_result, suffix))
}

/// DEPYLER-99MODE-S9: Fix `(*opt.expect(...)).expect(...)` double-unwrap on Option<i32>.
///
/// Pattern: `(*maybe_value.expect("msg1")).expect("msg2")` where maybe_value: &Option<i32>.
/// The first .expect() unwraps Option->i32, then (*i32).expect() is nonsensical.
/// Fix: remove dereference and second .expect(), keep just `maybe_value.expect("msg1")`.
pub(super) fn fix_double_expect_on_option_ref(code: &str) -> String {
    if !code.contains(".expect(") {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        if let Some((prefix, first_expect_result, suffix)) = try_parse_double_expect(trimmed) {
            let indent = &line[..line.len() - line.trim_start().len()];
            let new_trimmed = format!("{}{}{}", prefix, first_expect_result, suffix);
            result.push_str(indent);
            result.push_str(&new_trimmed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

// ── fix_move_closure_capture and helpers ─────────────────────────────────

/// Find the end line index (exclusive) of a closure body starting at `start_line + 1`.
fn find_closure_body_end(lines: &[&str], start_line: usize) -> usize {
    let mut body_depth = 1;
    let mut j = start_line + 1;
    while j < lines.len() && body_depth > 0 {
        for c in lines[j].chars() {
            match c {
                '{' => body_depth += 1,
                '}' => body_depth -= 1,
                _ => {}
            }
        }
        j += 1;
    }
    j
}

/// Find variables declared as `let mut VAR` before `closure_start` that are used both
/// inside the closure body and after it.
fn find_captured_vars(
    lines: &[&str],
    closure_start: usize,
    closure_end: usize,
) -> Vec<String> {
    let mut captured_vars = Vec::new();
    for k in 0..closure_start {
        let prev_trimmed = lines[k].trim();
        if !prev_trimmed.starts_with("let mut ") {
            continue;
        }
        let after_let = prev_trimmed.strip_prefix("let mut ").unwrap_or("");
        let var_end = after_let.find([':', ' ', '=']).unwrap_or(0);
        let var = &after_let[..var_end];
        if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let in_closure = (closure_start + 1..closure_end).any(|m| lines[m].contains(var));
        let after_closure = (closure_end..lines.len()).any(|m| lines[m].contains(var));
        if in_closure && after_closure {
            captured_vars.push(var.to_string());
        }
    }
    captured_vars
}

/// Build the list of (old_pattern, new_pattern) replacements for a captured variable.
fn build_clone_replacement_patterns(var: &str) -> Vec<(String, String)> {
    let clone_name = format!("{}_clone", var);
    vec![
        (format!("{}.get", var), format!("{}.get", clone_name)),
        (
            format!("{}.insert", var),
            format!("{}.insert", clone_name),
        ),
        (
            format!("{}.contains", var),
            format!("{}.contains", clone_name),
        ),
        (format!("{}.entry", var), format!("{}.entry", clone_name)),
        (format!("{}.len", var), format!("{}.len", clone_name)),
        (format!("{}.push", var), format!("{}.push", clone_name)),
        (format!("{}.clone", var), format!("{}.clone", clone_name)),
        (
            format!("{}.remove", var),
            format!("{}.remove", clone_name),
        ),
        (format!(" {} ", var), format!(" {} ", clone_name)),
        (format!("[{} as", var), format!("[{} as", clone_name)),
        (format!("[{}]", var), format!("[{}]", clone_name)),
    ]
}

/// Rewrite a single line in a closure body, replacing captured variable references.
fn rewrite_closure_body_line(line: &str, captured_vars: &[String]) -> String {
    let mut result = line.to_string();
    for var in captured_vars {
        let clone_name = format!("{}_clone", var);
        let patterns = build_clone_replacement_patterns(var);
        for (old, new) in &patterns {
            if result.contains(old.as_str()) && !result.contains(&clone_name) {
                result = result.replace(old.as_str(), new.as_str());
            }
        }
    }
    result
}

#[allow(dead_code)]
pub(super) fn fix_move_closure_capture(code: &str) -> String {
    if !code.contains("move |") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::with_capacity(code.len());
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        if !(trimmed.contains("= move |") && trimmed.ends_with('{')) {
            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
            continue;
        }

        let indent = &lines[i][..lines[i].len() - lines[i].trim_start().len()];
        let closure_end = find_closure_body_end(&lines, i);
        let captured_vars = find_captured_vars(&lines, i, closure_end);

        // Add clone statements before the closure
        for var in &captured_vars {
            result.push_str(&format!(
                "{}let {}_clone = {}.clone();\n",
                indent, var, var
            ));
        }

        // Insert the original closure line
        result.push_str(lines[i]);
        result.push('\n');
        i += 1;

        // Rewrite closure body to use VAR_clone instead of VAR
        let mut body_depth = 1;
        while i < lines.len() && body_depth > 0 {
            for c in lines[i].chars() {
                match c {
                    '{' => body_depth += 1,
                    '}' => body_depth -= 1,
                    _ => {}
                }
            }
            let line = rewrite_closure_body_line(lines[i], &captured_vars);
            result.push_str(&line);
            result.push('\n');
            i += 1;
        }
    }
    result
}

// ── fix_ref_string_to_owned_in_call and helpers ─────────────────────────

/// Find the position after `fn ` keyword in a line, if present.
fn find_fn_keyword_pos(trimmed: &str) -> Option<usize> {
    if let Some(pos) = trimmed.find("pub fn ") {
        return Some(pos + 7);
    }
    if let Some(pos) = trimmed.find("fn ") {
        if pos == 0 || !trimmed.as_bytes()[pos - 1].is_ascii_alphanumeric() {
            return Some(pos + 3);
        }
    }
    None
}

/// Parse a function signature to extract (fn_name, vec_of_string_param_positions).
fn parse_string_param_fn(trimmed: &str) -> Option<(String, Vec<usize>)> {
    let fn_start = find_fn_keyword_pos(trimmed)?;
    if !trimmed[fn_start..].contains('(') || !trimmed.contains("String") {
        return None;
    }
    let after_fn = &trimmed[fn_start..];
    let paren_start = after_fn.find('(')?;
    let fn_name = after_fn[..paren_start].trim().to_string();
    if fn_name.is_empty() || !fn_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let paren_end = after_fn.find(')')?;
    let params_str = &after_fn[paren_start + 1..paren_end];
    let mut string_positions: Vec<usize> = Vec::new();
    for (idx, param) in params_str.split(',').enumerate() {
        let param = param.trim();
        if param.contains(": String") && !param.contains('&') {
            string_positions.push(idx);
        }
    }
    if string_positions.is_empty() {
        return None;
    }
    Some((fn_name, string_positions))
}

/// Collect all functions that take owned String parameters.
fn collect_string_param_fns(code: &str) -> Vec<(String, Vec<usize>)> {
    let mut fns = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if let Some(entry) = parse_string_param_fn(trimmed) {
            fns.push(entry);
        }
    }
    fns
}

/// Try to replace a `fn_name(& var)` or `fn_name(&var)` pattern with `fn_name(var.clone())`.
fn try_replace_ref_arg_with_clone(line: &str, fn_name: &str) -> String {
    let ref_pattern_spaced = format!("{}(& ", fn_name);
    if let Some(new_line) = try_clone_ref_pattern(line, fn_name, &ref_pattern_spaced) {
        return new_line;
    }
    let ref_pattern_tight = format!("{}(&", fn_name);
    if let Some(new_line) = try_clone_ref_pattern(line, fn_name, &ref_pattern_tight) {
        return new_line;
    }
    line.to_string()
}

/// Attempt to find and replace a specific ref pattern with clone.
fn try_clone_ref_pattern(line: &str, fn_name: &str, pattern: &str) -> Option<String> {
    let start = line.find(pattern)?;
    let after = &line[start + pattern.len()..];
    let end = after.find([')', ','])?;
    let var = after[..end].trim();
    if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let delim = if after.as_bytes()[end] == b')' {
        ")"
    } else {
        ","
    };
    let old = format!("{}{}{}", pattern, var, delim);
    let new_str = format!("{}({}.clone(){}", fn_name, var, delim);
    Some(line.replace(&old, &new_str))
}

pub(super) fn fix_ref_string_to_owned_in_call(code: &str) -> String {
    let string_param_fns = collect_string_param_fns(code);
    if string_param_fns.is_empty() {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut new_line = line.to_string();
        let trimmed = line.trim();
        for (fn_name, positions) in &string_param_fns {
            let call_pattern = format!("{}(", fn_name);
            if !trimmed.contains(&call_pattern) {
                continue;
            }
            for &pos in positions {
                if pos == 0 {
                    new_line = try_replace_ref_arg_with_clone(&new_line, fn_name);
                }
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

// ── fix_missing_mut_for_method_calls and helpers ────────────────────────

/// Collect method names that have `&mut self` in their signature.
fn collect_mut_method_names(code: &str) -> Vec<String> {
    let mut methods = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("&mut self") || !trimmed.contains("fn ") {
            continue;
        }
        if let Some(method) = extract_mut_method_name(trimmed) {
            methods.push(method);
        }
    }
    methods
}

/// Extract a method name from a `fn name(&mut self, ...)` signature line.
fn extract_mut_method_name(trimmed: &str) -> Option<String> {
    let fn_idx = trimmed.find("fn ")?;
    let after_fn = &trimmed[fn_idx + 3..];
    let paren = after_fn.find('(')?;
    let method = after_fn[..paren].trim().to_string();
    if !method.is_empty()
        && method.chars().all(|c| c.is_alphanumeric() || c == '_')
        && method != "new"
    {
        Some(method)
    } else {
        None
    }
}

/// Find variable names that call mut methods but are not declared as `mut`.
fn collect_vars_needing_mut(code: &str, mut_methods: &[String]) -> Vec<String> {
    let mut vars = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        for method in mut_methods {
            if let Some(var) = extract_caller_var(trimmed, method) {
                vars.push(var);
            }
        }
    }
    vars
}

/// Extract the variable name that calls `.method(` from a line.
fn extract_caller_var(trimmed: &str, method: &str) -> Option<String> {
    let call_pattern = format!(".{}(", method);
    let dot_idx = trimmed.find(&call_pattern)?;
    let before = trimmed[..dot_idx].trim();
    let var = before
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .next_back()
        .unwrap_or("");
    if !var.is_empty()
        && var != "self"
        && var != "mut"
        && var.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        Some(var.to_string())
    } else {
        None
    }
}

/// Add `mut` to variable declarations for variables that need it.
fn apply_mut_to_declarations(code: &str, vars_needing_mut: &[String]) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut new_line = line.to_string();
        for var in vars_needing_mut {
            new_line = add_mut_to_let_binding(&new_line, var);
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

/// Add `mut` to a `let VAR =` or `let VAR:` binding if not already present.
fn add_mut_to_let_binding(line: &str, var: &str) -> String {
    let mut result = line.to_string();
    let non_mut = format!("let {} =", var);
    let with_mut = format!("let mut {} =", var);
    if result.contains(&non_mut) && !result.contains(&with_mut) {
        result = result.replace(&non_mut, &with_mut);
    }
    let non_mut_typed = format!("let {}:", var);
    let with_mut_typed = format!("let mut {}:", var);
    if result.contains(&non_mut_typed) && !result.contains(&with_mut_typed) {
        result = result.replace(&non_mut_typed, &with_mut_typed);
    }
    result
}

/// DEPYLER-99MODE-S9: Fix `let var` -> `let mut var` when method requires `&mut self`.
///
/// When a method is defined with `&mut self` and called on a non-mut variable,
/// add `mut` to the variable declaration.
pub(super) fn fix_missing_mut_for_method_calls(code: &str) -> String {
    if !code.contains("&mut self") {
        return code.to_string();
    }
    let mut_methods = collect_mut_method_names(code);
    if mut_methods.is_empty() {
        return code.to_string();
    }
    let vars_needing_mut = collect_vars_needing_mut(code, &mut_methods);
    if vars_needing_mut.is_empty() {
        return code.to_string();
    }
    apply_mut_to_declarations(code, &vars_needing_mut)
}

pub(super) fn fix_deref_on_unwrap_or(code: &str) -> String {
    code.replace("(*self.", "(self.")
        .replace("(* self.", "(self.")
}

// ── fix_missing_ref_in_fn_call and helpers ───────────────────────────────

/// Collect function names whose first parameter is a reference type.
fn collect_ref_first_param_fns(lines: &[&str]) -> Vec<String> {
    let mut fns = Vec::new();
    let mut pending_fn: Option<String> = None;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            if let Some(result) = process_fn_signature_for_ref_param(trimmed, &mut pending_fn) {
                fns.push(result);
            }
        } else if let Some(ref fn_name) = pending_fn {
            if trimmed.contains(": &") {
                fns.push(fn_name.clone());
                pending_fn = None;
            } else if trimmed.contains(':') || trimmed.starts_with(')') {
                pending_fn = None;
            }
        }
    }
    fns
}

/// Process a function signature line to determine if its first param is a reference.
/// Returns `Some(fn_name)` if the first param is a reference on this line,
/// or sets `pending_fn` if the first param is on the next line.
fn process_fn_signature_for_ref_param(
    trimmed: &str,
    pending_fn: &mut Option<String>,
) -> Option<String> {
    let name_start = trimmed.find("fn ")?;
    let after = &trimmed[name_start + 3..];
    let name_end = after.find(['(', '<']).unwrap_or(after.len());
    let fn_name = after[..name_end].trim().to_string();
    if let Some(paren) = trimmed.find('(') {
        let after_paren = &trimmed[paren + 1..];
        let first_param = after_paren.split(',').next().unwrap_or("");
        if first_param.contains(": &") {
            return Some(fn_name);
        }
        if first_param.trim().is_empty() {
            *pending_fn = Some(fn_name);
        }
    } else {
        *pending_fn = Some(fn_name);
    }
    None
}

/// Add `&` before the first argument in a call if it's not already a reference.
fn add_ref_to_first_call_arg(line: &str, fn_name: &str) -> String {
    let call_pat = format!("{}(", fn_name);
    let Some(call_pos) = line.find(&call_pat) else {
        return line.to_string();
    };
    let after_paren = call_pos + call_pat.len();
    let rest = &line[after_paren..];
    if rest.starts_with('&') || rest.starts_with("self") {
        return line.to_string();
    }
    let Some(comma) = rest.find(',') else {
        return line.to_string();
    };
    let arg = rest[..comma].trim();
    if arg.is_empty() || arg.starts_with('&') {
        return line.to_string();
    }
    format!(
        "{}&{}{}",
        &line[..after_paren],
        &line[after_paren..after_paren + comma],
        &line[after_paren + comma..]
    )
}

#[allow(dead_code)]
pub(super) fn fix_missing_ref_in_fn_call(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let ref_first_param_fns = collect_ref_first_param_fns(&lines);
    for line in &lines {
        let mut new_line = line.to_string();
        for fn_name in &ref_first_param_fns {
            new_line = add_ref_to_first_call_arg(&new_line, fn_name);
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

/// Helper: find the index of the matching closing parenthesis in a string.
/// Assumes the opening paren has already been consumed; `s` starts after `(`.
fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    let mut in_string = false;
    let mut escape_next = false;
    for (i, c) in s.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }
        match c {
            '\\' => {
                escape_next = true;
            }
            '"' => {
                in_string = !in_string;
            }
            '(' if !in_string => depth += 1,
            ')' if !in_string => {
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
