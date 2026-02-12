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
    // Scan for lines with `.into_iter().chain(` and replace the first into_iter
    // with `.iter().cloned()`. Also fix the chained argument's `.into_iter()`.
    let result = code.to_string();
    // Replace pattern: `VAR.into_iter().chain(VAR2.into_iter())`
    // with: `VAR.iter().cloned().chain(VAR2.iter().cloned())`
    // We do this by finding `.into_iter().chain(` and replacing in context.
    let lines: Vec<&str> = result.lines().collect();
    let mut new_lines: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        if line.contains(".into_iter().chain(") && line.contains(".into_iter())") {
            // This line has the pattern: X.into_iter().chain(Y.into_iter())
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

pub(super) fn fix_borrowed_alias_in_new_calls(code: &str) -> String {
    // Collect String type aliases
    let mut string_aliases: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        let rest = match trimmed
            .strip_prefix("pub type ")
            .or_else(|| trimmed.strip_prefix("type "))
        {
            Some(r) => r,
            None => continue,
        };
        if rest.contains("= String;") || rest.contains("= String ;") {
            let name_end = rest.find(|c: char| !c.is_alphanumeric() && c != '_');
            let name = match name_end {
                Some(e) => &rest[..e],
                None => continue,
            };
            if !name.is_empty() {
                string_aliases.push(name.to_string());
            }
        }
    }
    if string_aliases.is_empty() {
        return code.to_string();
    }
    // Find function params with `&Alias` types
    let mut borrowed_params: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        for alias in &string_aliases {
            let pattern = format!(": &{}", alias);
            if trimmed.contains(&pattern) {
                // Extract param name before the `: &Alias`
                if let Some(pos) = trimmed.find(&pattern) {
                    let before = trimmed[..pos].trim();
                    let param = before
                        .rsplit(|c: char| c == '(' || c == ',' || c.is_whitespace())
                        .next()
                        .unwrap_or("")
                        .trim();
                    if !param.is_empty() && param.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        borrowed_params.push(param.to_string());
                    }
                }
            }
        }
    }
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
            paren_depth = 0;
            for ch in trimmed.chars() {
                match ch {
                    '(' => paren_depth += 1,
                    ')' => paren_depth -= 1,
                    _ => {}
                }
            }
        }
        if in_new_call {
            let mut modified = line.to_string();
            for param in &borrowed_params {
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
            result.push_str(&modified);
            if !trimmed.contains("::new(") {
                for ch in trimmed.chars() {
                    match ch {
                        '(' => paren_depth += 1,
                        ')' => paren_depth -= 1,
                        _ => {}
                    }
                }
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

/// DEPYLER-CONVERGE-MULTI-ITER11: Fix deref string comparisons.
///
/// The transpiler generates `(*var) == "literal"` which dereferences `&String`
/// to `str`, but `str == &str` has no implementation. Remove the unnecessary `*`.
/// Pattern: `(*identifier) == "` → `identifier == "`
pub(super) fn fix_deref_string_comparison(code: &str) -> String {
    let mut result = code.to_string();
    // Pattern: (*var) == "..." or (*var) != "..."
    let mut i = 0;
    while i < result.len() {
        if let Some(pos) = result[i..].find("(*") {
            let abs_pos = i + pos;
            // Find the matching close paren
            let after = &result[abs_pos + 2..];
            if let Some(close) = after.find(')') {
                let var_name = &after[..close];
                // Check it's a simple identifier
                if var_name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
                {
                    let after_close = &result[abs_pos + 2 + close + 1..];
                    let trimmed = after_close.trim_start();
                    if trimmed.starts_with("== \"") || trimmed.starts_with("!= \"") {
                        // Replace (*var) with var
                        let old = format!("(*{})", var_name);
                        let new = var_name.to_string();
                        result = format!(
                            "{}{}{}",
                            &result[..abs_pos],
                            new,
                            &result[abs_pos + old.len()..]
                        );
                        i = abs_pos + new.len();
                        continue;
                    }
                }
            }
            i = abs_pos + 2;
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
        // Match: `param: &Option<T>,` or `param: &'a Option<T>,`
        if !trimmed.contains("Option<") || !trimmed.ends_with(',') {
            continue;
        }
        let Some(colon) = trimmed.find(':') else {
            continue;
        };
        let type_part = trimmed[colon + 1..].trim();
        let is_ref_option = (type_part.starts_with("&Option<") || type_part.starts_with("&'"))
            && type_part.contains("Option<");
        if is_ref_option {
            let name = trimmed[..colon].trim().trim_start_matches("mut ");
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                params.push(name.to_string());
            }
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
        // Only start tracking when not already inside a ::new() call
        if line.contains("::new(") && !in_new_call {
            in_new_call = true;
            paren_depth = 0;
        }
        if in_new_call {
            for ch in line.chars() {
                match ch {
                    '(' => paren_depth += 1,
                    ')' => paren_depth -= 1,
                    _ => {}
                }
            }
            for param in params {
                if fixed_line.trim() == format!("{},", param) {
                    fixed_line =
                        fixed_line.replace(&format!("{},", param), &format!("*{},", param));
                } else if fixed_line.trim() == format!("{})", param) {
                    fixed_line =
                        fixed_line.replace(&format!("{})", param), &format!("*{})", param));
                }
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

/// Fix `(*ref_option.unwrap_or_default())` where ref_option is `&Option<T>`.
/// Deref the reference first: `(*VAR).unwrap_or_default()` (works for Copy types).
pub(super) fn fix_deref_ref_option_unwrap(code: &str) -> String {
    let mut result = code.to_string();
    // Pattern: `(*VAR.unwrap_or_default())` → `(*VAR).unwrap_or_default()`
    let search = ".unwrap_or_default())";
    let mut i = 0;
    while i < result.len() {
        let Some(pos) = result[i..].find(search) else {
            break;
        };
        let abs = i + pos;
        // Walk back to find `(*`
        if abs >= 2 {
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
        }
        i = abs + search.len();
    }
    result
}

pub(super) fn fix_immutable_ref_to_mut(code: &str) -> String {
    use std::collections::HashMap;
    // Pass 1: Collect fn_name → Vec<param_index> for &mut params
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
            // DEPYLER-99MODE-S9: Use word boundary check to avoid substring matches
            // e.g., "set(" must not match inside "abs_set(" or "reset("
            if let Some(pos) = line_str.find(&pat) {
                // Check that char before the match is not alphanumeric or underscore
                if pos > 0 {
                    let prev_char = line_str.as_bytes()[pos - 1] as char;
                    if prev_char.is_alphanumeric() || prev_char == '_' {
                        continue;
                    }
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
    Some((name.to_string(), positions))
}

/// Fix a single call site: change `&arg` to `&mut arg` at specified positions.
pub(super) fn fix_mut_args_in_call(line: &str, fname: &str, positions: &[usize]) -> String {
    let pat = format!("{}(", fname);
    let call_pos = match line.find(&pat) {
        Some(p) => {
            // DEPYLER-99MODE-S9: Word boundary check — don't match substrings
            if p > 0 {
                let prev = line.as_bytes()[p - 1] as char;
                if prev.is_alphanumeric() || prev == '_' {
                    return line.to_string();
                }
            }
            p
        }
        None => return line.to_string(),
    };
    let args_start = call_pos + pat.len();
    // Find matching )
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
    let args_str = &line[args_start..args_end];
    // Split args at depth 0
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
    let mut changed = false;
    for &pos in positions {
        if pos < args.len() {
            let trimmed = args[pos].trim();
            if trimmed.starts_with('&') && !trimmed.starts_with("&mut ") {
                let ws = args[pos].len() - args[pos].trim_start().len();
                let prefix: String = args[pos].chars().take(ws).collect();
                args[pos] = format!("{}&mut {}", prefix, &trimmed[1..]);
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

pub(super) fn fix_deref_expect_on_primitive(code: &str) -> String {
    if !code.contains("(*") || !code.contains(".expect(") {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut fixed_line = line.to_string();
        // Match pattern: (*var).expect("...")
        // Replace with just: var
        while let Some(star_pos) = fixed_line.find("(*") {
            let after_star = &fixed_line[star_pos + 2..];
            // Find the closing paren of (*var)
            if let Some(close_paren) = after_star.find(')') {
                let var_name = &after_star[..close_paren];
                // Only fix simple variable names (no dots, no complex expressions)
                if var_name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_')
                    && !var_name.is_empty()
                {
                    let after_close = &after_star[close_paren + 1..];
                    if after_close.starts_with(".expect(") {
                        // Find the end of .expect("...")
                        let expect_content = &after_close[8..]; // skip .expect(
                        if let Some(end) = find_matching_paren(expect_content) {
                            let rest = &expect_content[end + 1..];
                            fixed_line = format!(
                                "{}{}{}",
                                &fixed_line[..star_pos],
                                var_name,
                                rest
                            );
                            continue; // Check for more patterns in same line
                        }
                    }
                }
            }
            break; // No more patterns or couldn't fix
        }
        result.push_str(&fixed_line);
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.truncate(result.len().saturating_sub(1));
    }
    result
}

pub(super) fn fix_ref_arg_to_fn_string_param(code: &str) -> String {
    if !code.contains("impl Fn(String)") {
        return code.to_string();
    }
    // Find parameter names with type `impl Fn(String) -> String`
    let mut fn_param_names: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.contains("impl Fn(String)") {
            // Extract param name before `: impl Fn(String)`
            if let Some(idx) = trimmed.find(": impl Fn(String)") {
                let before = &trimmed[..idx];
                if let Some(name) = before.split(|c: char| c == '(' || c == ',').last() {
                    let name = name.trim();
                    if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        fn_param_names.push(name.to_string());
                    }
                }
            }
        }
    }
    if fn_param_names.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for name in &fn_param_names {
        // Replace: `name(&var)` with `name(var.clone())`
        // This is safe because String is Clone
        let re_pattern = format!(r"{}(&", name);
        if result.contains(&re_pattern) {
            // Find all occurrences and fix them
            let mut new_result = String::with_capacity(result.len());
            let mut pos = 0;
            while let Some(idx) = result[pos..].find(&re_pattern) {
                let abs_idx = pos + idx;
                new_result.push_str(&result[pos..abs_idx]);
                new_result.push_str(&format!("{}(", name));
                // Skip past the `&`
                let after = abs_idx + re_pattern.len();
                // Find the closing `)`
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
            result = new_result;
        }
    }
    result
}

/// DEPYLER-99MODE-S9: Fix `(*opt.expect(...)).expect(...)` double-unwrap on Option<i32>.
///
/// Pattern: `(*maybe_value.expect("msg1")).expect("msg2")` where maybe_value: &Option<i32>.
/// The first .expect() unwraps Option→i32, then (*i32).expect() is nonsensical.
/// Fix: remove dereference and second .expect(), keep just `maybe_value.expect("msg1")`.
pub(super) fn fix_double_expect_on_option_ref(code: &str) -> String {
    if !code.contains(".expect(") {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Match pattern: `(*VAR.expect("msg1")).expect("msg2")`
        // The full pattern has: leading `(*`, first `.expect("...")`, closing `)`, second `.expect("...")`
        if trimmed.contains("(*") && {
            // Count .expect( occurrences - need at least 2
            trimmed.matches(".expect(").count() >= 2
        } {
            if let Some(star_idx) = trimmed.find("(*") {
                // Find first .expect(
                if let Some(exp1_offset) = trimmed[star_idx + 2..].find(".expect(") {
                    let var_name = &trimmed[star_idx + 2..star_idx + 2 + exp1_offset];
                    if var_name
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_')
                    {
                        // Find the first .expect("...") end
                        let exp1_start = star_idx + 2 + exp1_offset;
                        // Find matching close paren for first expect
                        if let Some(quote1_start) = trimmed[exp1_start..].find('"') {
                            let abs_q1 = exp1_start + quote1_start;
                            if let Some(quote1_end) = trimmed[abs_q1 + 1..].find('"') {
                                let exp1_close = abs_q1 + 1 + quote1_end + 1; // after closing quote
                                // Skip to closing paren of first expect
                                if exp1_close < trimmed.len()
                                    && trimmed.as_bytes()[exp1_close] == b')'
                                {
                                    let first_expect_result =
                                        &trimmed[star_idx + 2..exp1_close + 1];
                                    // Check for ).expect( after
                                    let after = &trimmed[exp1_close + 1..];
                                    if after.starts_with(").expect(") {
                                        // Replace entire expression with just the first unwrap
                                        let indent =
                                            &line[..line.len() - line.trim_start().len()];
                                        let prefix = &trimmed[..star_idx];
                                        // Find end of second .expect("...")
                                        let exp2_start = exp1_close + 1 + 1; // skip )
                                        if let Some(exp2_close) =
                                            trimmed[exp2_start..].find("\")") // end of second expect
                                        {
                                            let suffix =
                                                &trimmed[exp2_start + exp2_close + 2..];
                                            let new_trimmed = format!(
                                                "{}{}{}",
                                                prefix, first_expect_result, suffix
                                            );
                                            result.push_str(indent);
                                            result.push_str(&new_trimmed);
                                            result.push('\n');
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result.push_str(line);
        result.push('\n');
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

        // Detect: `let mut CLOSURE_NAME = move |PARAMS| -> TYPE {`
        if trimmed.contains("= move |") && trimmed.ends_with('{') {
            // Find variables used inside the closure that were defined before
            // Look for HashMap/Vec variables referenced inside the closure body
            let indent = &lines[i][..lines[i].len() - lines[i].trim_start().len()];

            // Scan the closure body to find captured variables
            let mut captured_vars: Vec<String> = Vec::new();
            let mut body_depth = 1;
            let mut j = i + 1;
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
            let closure_end = j;

            // Look for variables used BOTH in the closure and AFTER it
            // Check for HashMap/Vec variables that are `let mut VAR: Type = ...` before this line
            for k in 0..i {
                let prev_trimmed = lines[k].trim();
                if prev_trimmed.starts_with("let mut ") {
                    let after_let = prev_trimmed.strip_prefix("let mut ").unwrap_or("");
                    let var_end = after_let
                        .find(|c: char| c == ':' || c == ' ' || c == '=')
                        .unwrap_or(0);
                    let var = &after_let[..var_end];
                    if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        continue;
                    }
                    // Check if variable is used in closure body AND after closure
                    let in_closure = (i + 1..closure_end)
                        .any(|m| lines[m].contains(var));
                    let after_closure = (closure_end..lines.len())
                        .any(|m| lines[m].contains(var));
                    if in_closure && after_closure {
                        captured_vars.push(var.to_string());
                    }
                }
            }

            // Add clone statements before the closure
            for var in &captured_vars {
                result.push_str(&format!(
                    "{}let {}_clone = {}.clone();\n",
                    indent, var, var
                ));
            }

            // Rewrite the closure to use the cloned variable
            let mut closure_line = lines[i].to_string();
            for var in &captured_vars {
                // In the closure body, replace references to the original var
                // with the clone. We do this for the body lines.
            }

            // Actually, simpler approach: change `move |` to use cloned vars
            // Insert clone before, and change closure body to use VAR_clone
            result.push_str(&closure_line);
            result.push('\n');
            i += 1;

            // Rewrite closure body to use VAR_clone instead of VAR
            body_depth = 1;
            while i < lines.len() && body_depth > 0 {
                let mut line = lines[i].to_string();
                for c in lines[i].chars() {
                    match c {
                        '{' => body_depth += 1,
                        '}' => body_depth -= 1,
                        _ => {}
                    }
                }
                for var in &captured_vars {
                    let clone_name = format!("{}_clone", var);
                    // Replace standalone variable references
                    // Be careful not to replace substrings (e.g., "parent" in "parent_clone")
                    let patterns = [
                        (format!("{}.get", var), format!("{}.get", clone_name)),
                        (format!("{}.insert", var), format!("{}.insert", clone_name)),
                        (format!("{}.contains", var), format!("{}.contains", clone_name)),
                        (format!("{}.entry", var), format!("{}.entry", clone_name)),
                        (format!("{}.len", var), format!("{}.len", clone_name)),
                        (format!("{}.push", var), format!("{}.push", clone_name)),
                        (format!("{}.clone", var), format!("{}.clone", clone_name)),
                        (format!("{}.remove", var), format!("{}.remove", clone_name)),
                        (format!(" {} ", var), format!(" {} ", clone_name)),
                        (
                            format!("[{} as", var),
                            format!("[{} as", clone_name),
                        ),
                        (
                            format!("[{}]", var),
                            format!("[{}]", clone_name),
                        ),
                    ];
                    for (old, new) in &patterns {
                        if line.contains(old.as_str()) && !line.contains(&clone_name) {
                            line = line.replace(old.as_str(), new.as_str());
                        }
                    }
                }
                result.push_str(&line);
                result.push('\n');
                i += 1;
            }
            continue;
        }

        result.push_str(lines[i]);
        result.push('\n');
        i += 1;
    }
    result
}

pub(super) fn fix_ref_string_to_owned_in_call(code: &str) -> String {
    // Collect functions that take owned String params
    let mut string_param_fns: Vec<(String, Vec<usize>)> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        // Match function signatures - use contains() since doc comments may precede on same line
        let fn_keyword_pos = if let Some(pos) = trimmed.find("pub fn ") {
            Some(pos + 7)
        } else if let Some(pos) = trimmed.find("fn ") {
            // Make sure it's not inside a string or comment
            if pos == 0 || !trimmed.as_bytes()[pos - 1].is_ascii_alphanumeric() {
                Some(pos + 3)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(fn_start) = fn_keyword_pos {
            if !trimmed[fn_start..].contains('(') || !trimmed.contains("String") {
                continue;
            }
            let after_fn = &trimmed[fn_start..];
            if let Some(paren_start) = after_fn.find('(') {
                let fn_name = after_fn[..paren_start].trim().to_string();
                if fn_name.is_empty()
                    || !fn_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    continue;
                }
                // Parse params to find which positions take String
                if let Some(paren_end) = after_fn.find(')') {
                    let params_str = &after_fn[paren_start + 1..paren_end];
                    let mut string_positions: Vec<usize> = Vec::new();
                    for (idx, param) in params_str.split(',').enumerate() {
                        let param = param.trim();
                        // Check if param type is exactly String (not &str, not &String)
                        if param.contains(": String") && !param.contains("&") {
                            string_positions.push(idx);
                        }
                    }
                    if !string_positions.is_empty() {
                        string_param_fns.push((fn_name, string_positions));
                    }
                }
            }
        }
    }

    if string_param_fns.is_empty() {
        return code.to_string();
    }

    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut new_line = line.to_string();
        let trimmed = line.trim();
        for (fn_name, positions) in &string_param_fns {
            // Check if this line calls the function
            let call_pattern = format!("{}(", fn_name);
            if !trimmed.contains(&call_pattern) {
                continue;
            }
            // For each String param position, check if `& ` is used
            for &pos in positions {
                // Simple approach: find `fn_name(` and then the nth argument
                // For position 0, look for `fn_name(& ` or `fn_name(&var)`
                if pos == 0 {
                    let ref_pattern = format!("{}(& ", fn_name);
                    let ref_pattern2 = format!("{}(&", fn_name);
                    if new_line.contains(&ref_pattern) {
                        // Find the var name after &
                        if let Some(start) = new_line.find(&ref_pattern) {
                            let after = &new_line[start + ref_pattern.len()..];
                            if let Some(end) = after.find(|c: char| c == ')' || c == ',') {
                                let var = after[..end].trim();
                                if var.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                    let old = format!("{}(& {}{}",
                                        fn_name, var,
                                        if after.as_bytes()[end] == b')' { ")" } else { "," });
                                    let new_str = format!("{}{}.clone(){}",
                                        format!("{}(", fn_name), var,
                                        if after.as_bytes()[end] == b')' { ")" } else { "," });
                                    new_line = new_line.replace(&old, &new_str);
                                }
                            }
                        }
                    } else if new_line.contains(&ref_pattern2) {
                        if let Some(start) = new_line.find(&ref_pattern2) {
                            let after = &new_line[start + ref_pattern2.len()..];
                            if let Some(end) = after.find(|c: char| c == ')' || c == ',') {
                                let var = after[..end].trim();
                                if !var.is_empty()
                                    && var.chars().all(|c| c.is_alphanumeric() || c == '_')
                                {
                                    let old = format!("{}(&{}{}",
                                        fn_name, var,
                                        if after.as_bytes()[end] == b')' { ")" } else { "," });
                                    let new_str = format!("{}{}.clone(){}",
                                        format!("{}(", fn_name), var,
                                        if after.as_bytes()[end] == b')' { ")" } else { "," });
                                    new_line = new_line.replace(&old, &new_str);
                                }
                            }
                        }
                    }
                }
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

/// DEPYLER-99MODE-S9: Fix `let var` → `let mut var` when method requires `&mut self`.
///
/// When a method is defined with `&mut self` and called on a non-mut variable,
/// add `mut` to the variable declaration.
pub(super) fn fix_missing_mut_for_method_calls(code: &str) -> String {
    if !code.contains("&mut self") {
        return code.to_string();
    }
    // Collect method names that take &mut self
    let mut mut_methods: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.contains("&mut self") && trimmed.contains("fn ") {
            // Extract method name
            if let Some(fn_idx) = trimmed.find("fn ") {
                let after_fn = &trimmed[fn_idx + 3..];
                if let Some(paren) = after_fn.find('(') {
                    let method = after_fn[..paren].trim().to_string();
                    if !method.is_empty()
                        && method.chars().all(|c| c.is_alphanumeric() || c == '_')
                        && method != "new"
                    {
                        mut_methods.push(method);
                    }
                }
            }
        }
    }
    if mut_methods.is_empty() {
        return code.to_string();
    }

    // Find variables that call mut methods but aren't declared as mut
    let mut vars_needing_mut: Vec<String> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        for method in &mut_methods {
            let call_pattern = format!(".{}(", method);
            if trimmed.contains(&call_pattern) {
                // Extract the variable name before .method(
                if let Some(dot_idx) = trimmed.find(&call_pattern) {
                    let before = trimmed[..dot_idx].trim();
                    // Get the last token (the variable name)
                    let var = before
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .last()
                        .unwrap_or("");
                    if !var.is_empty()
                        && var != "self"
                        && var != "mut"
                        && var.chars().all(|c| c.is_alphanumeric() || c == '_')
                    {
                        vars_needing_mut.push(var.to_string());
                    }
                }
            }
        }
    }
    if vars_needing_mut.is_empty() {
        return code.to_string();
    }

    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let mut new_line = line.to_string();
        for var in &vars_needing_mut {
            // Replace `let VAR =` with `let mut VAR =` (only if not already mut)
            let non_mut = format!("let {} =", var);
            let with_mut = format!("let mut {} =", var);
            if new_line.contains(&non_mut) && !new_line.contains(&with_mut) {
                new_line = new_line.replace(&non_mut, &with_mut);
            }
            // Also handle `let VAR:` pattern
            let non_mut_typed = format!("let {}:", var);
            let with_mut_typed = format!("let mut {}:", var);
            if new_line.contains(&non_mut_typed) && !new_line.contains(&with_mut_typed) {
                new_line = new_line.replace(&non_mut_typed, &with_mut_typed);
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_deref_on_unwrap_or(code: &str) -> String {
    code.replace("(*self.", "(self.")
        .replace("(* self.", "(self.")
}

#[allow(dead_code)]
pub(super) fn fix_missing_ref_in_fn_call(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    // Collect function names whose first parameter is a reference type
    // Handle multi-line signatures by scanning ahead for the first param
    let mut ref_first_param_fns: Vec<String> = Vec::new();
    let mut pending_fn: Option<String> = None;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            if let Some(name_start) = trimmed.find("fn ") {
                let after = &trimmed[name_start + 3..];
                // Strip generic params for name extraction
                let name_end = after.find(['(', '<']).unwrap_or(after.len());
                let fn_name = after[..name_end].trim().to_string();
                // Check if first param is on this line
                if let Some(paren) = trimmed.find('(') {
                    let after_paren = &trimmed[paren + 1..];
                    let first_param = after_paren.split(',').next().unwrap_or("");
                    if first_param.contains(": &") {
                        ref_first_param_fns.push(fn_name.clone());
                    } else if first_param.trim().is_empty() || first_param.trim() == "" {
                        // First param is on next line
                        pending_fn = Some(fn_name);
                    }
                } else {
                    pending_fn = Some(fn_name);
                }
            }
        } else if let Some(ref fn_name) = pending_fn {
            if trimmed.contains(": &") {
                ref_first_param_fns.push(fn_name.clone());
                pending_fn = None;
            } else if trimmed.contains(':') || trimmed.starts_with(')') {
                pending_fn = None;
            }
        }
    }
    // Fix calls: fn_name(VAR.clone(), ...) → fn_name(&VAR.clone(), ...)
    for line in &lines {
        let mut new_line = line.to_string();
        for fn_name in &ref_first_param_fns {
            let call_pat = format!("{}(", fn_name);
            if let Some(call_pos) = new_line.find(&call_pat) {
                let after_paren = call_pos + call_pat.len();
                let rest = &new_line[after_paren..];
                // Check if first arg is NOT already a reference
                if !rest.starts_with('&') && !rest.starts_with("self") {
                    // Find the first comma (end of first arg)
                    if let Some(comma) = rest.find(',') {
                        let arg = rest[..comma].trim();
                        if !arg.is_empty() && !arg.starts_with('&') {
                            new_line = format!(
                                "{}&{}{}",
                                &new_line[..after_paren],
                                &new_line[after_paren..after_paren + comma],
                                &new_line[after_paren + comma..]
                            );
                        }
                    }
                }
            }
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
        if c == '\\' {
            escape_next = true;
            continue;
        }
        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match c {
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
