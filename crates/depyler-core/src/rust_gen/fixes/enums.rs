//! Enum and LazyLock-related post-transpilation fix functions.
//!
//! These functions perform text-level repairs on generated Rust code to fix
//! enum path separators, LazyLock static declarations, enum constructors,
//! Display impls, and orphaned LazyLock bodies.

pub(super) fn fix_enum_path_separator(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if i + 1 < lines.len() {
            let current = lines[i].trim_end();
            let next_trimmed = lines[i + 1].trim();

            // Check if current line ends with a PascalCase identifier
            // and next line starts with .method(
            if next_trimmed.starts_with('.') && is_trailing_pascal_case(current) {
                let indent = &lines[i][..lines[i].len() - current.trim().len()];
                let type_name = current.trim();
                // Join: TypeName::method(...)
                let method_part = &next_trimmed[1..]; // skip the dot
                result.push(format!("{}{}::{}", indent, type_name, method_part));
                i += 2;
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }

    result.join("\n") + "\n"
}

/// Check if a line ends with a PascalCase identifier (UpperCamelCase).
pub(super) fn is_trailing_pascal_case(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }
    // Must start with uppercase letter
    let first = trimmed.chars().next().unwrap_or('a');
    if !first.is_ascii_uppercase() {
        return false;
    }
    // Must be a single identifier (no spaces, no operators)
    trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub(super) fn fix_enum_dot_to_path_separator(code: &str) -> String {
    let mut result = code.to_string();
    // Common transpiler patterns where dot should be :: for Rust path syntax.
    // Pattern: `PascalCaseType.UPPER_CASE_VARIANT` -> `PascalCaseType::UPPER_CASE_VARIANT`
    let enum_types = [
        "Color",
        "Status",
        "StatusCode",
        "ErrorKind",
        "Level",
        "Priority",
        "Direction",
        "Ordering",
        "SeekFrom",
        "Shutdown",
    ];
    for ty in &enum_types {
        let dot_prefix = format!("{}.", ty);
        let path_prefix = format!("{}::", ty);
        result = result.replace(&dot_prefix, &path_prefix);
    }
    result
}

pub(super) fn fix_lazylock_static_as_type(code: &str) -> String {
    if !code.contains("std::sync::LazyLock<") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("pub static ") && trimmed.contains("std::sync::LazyLock<") {
            let name = extract_static_name(trimmed);
            if !name.is_empty() && is_pascal_case(&name) {
                result.push(format!("pub type {} = String;", name));
                i = skip_block(i, &lines);
                continue;
            }
        }
        // Also handle multi-line: `pub static PascalName: LazyLock<...> =\n    LazyLock::new`
        if trimmed.starts_with("pub static ")
            && trimmed.contains("std::sync::LazyLock<")
            && trimmed.ends_with('=')
        {
            let name = extract_static_name(trimmed);
            if !name.is_empty() && is_pascal_case(&name) {
                result.push(format!("pub type {} = String;", name));
                i = skip_block(i, &lines);
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

/// DEPYLER-CONVERGE-MULTI-ITER9: Repair malformed LazyLock initializers.
///
/// SCREAMING_SNAKE LazyLock statics have invalid enum::iter() and Arc.unwrap().
/// Replace body with empty Vec.
pub(super) fn fix_broken_lazylock_initializers(code: &str) -> String {
    if !code.contains("std::sync::LazyLock<") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Case 1: Single-line `pub static NAME: LazyLock<...> = LazyLock::new(...)`
        // Only replace SCREAMING_SNAKE_CASE names (malformed Tier 3 constants)
        if trimmed.starts_with("pub static ")
            && trimmed.contains("std::sync::LazyLock<")
            && trimmed.contains("= std::sync::LazyLock::new")
        {
            let name = extract_static_name(trimmed);
            if !name.is_empty() && is_screaming_snake(&name) {
                result.push(format!(
                    "pub static {}: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());",
                    name
                ));
                i = skip_block(i, &lines);
                continue;
            }
        }
        // Case 2: Multi-line `pub static NAME: LazyLock<...> =\n    LazyLock::new(...)`
        if trimmed.starts_with("pub static ")
            && trimmed.contains("std::sync::LazyLock<")
            && trimmed.ends_with('=')
        {
            let name = extract_static_name(trimmed);
            if !name.is_empty() && is_screaming_snake(&name) {
                result.push(format!(
                    "pub static {}: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());",
                    name
                ));
                i = skip_block(i, &lines);
                continue;
            }
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

/// DEPYLER-CONVERGE-MULTI-ITER9: Fix Literal.clone().py_index(...) blocks.
///
/// Python `typing.Literal["a","b"]` generates an invalid `Literal.clone().py_index(...)` pattern.
/// Replace with empty string since it's typically a default value.
pub(super) fn fix_literal_clone_pattern(code: &str) -> String {
    if !code.contains("Literal.clone()") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.contains("Literal.clone().py_index(") {
            let indent = &lines[i][..lines[i].len() - trimmed.len()];
            result.push(format!("{}String::new()", indent));
            // Skip multi-line Literal block until closing paren
            let mut depth = count_parens_open(trimmed) - count_parens_close(trimmed);
            i += 1;
            while i < lines.len() && depth > 0 {
                depth += count_parens_open(lines[i].trim());
                depth -= count_parens_close(lines[i].trim());
                i += 1;
            }
            continue;
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

pub(super) fn fix_enum_new_constructor(code: &str) -> String {
    if !code.contains("pub fn value(&self) -> &str") {
        return code.to_string();
    }
    let mut result = code.to_string();
    let marker = "pub fn value(&self) -> &str";
    let mut search_from = 0;

    loop {
        let haystack = &result[search_from..];
        let rel_pos = match haystack.find(marker) {
            Some(p) => p,
            None => break,
        };
        let abs_pos = search_from + rel_pos;

        // Already has new()? Skip.
        let impl_start = result[..abs_pos].rfind("impl ").unwrap_or(0);
        let impl_block = &result[impl_start..abs_pos];
        if impl_block.contains("pub fn new(") {
            search_from = abs_pos + marker.len();
            continue;
        }

        // Extract enum name from `impl EnumName {`
        let enum_name = extract_enum_name_from_impl(&result[impl_start..abs_pos]);
        if enum_name.is_empty() {
            search_from = abs_pos + marker.len();
            continue;
        }

        // Find the match block inside value()
        let match_start = match result[abs_pos..].find("match self {") {
            Some(p) => abs_pos + p + "match self {".len(),
            None => {
                search_from = abs_pos + marker.len();
                continue;
            }
        };

        // Find closing brace of match
        let mut depth = 1;
        let mut idx = match_start;
        let bytes = result.as_bytes();
        while idx < bytes.len() && depth > 0 {
            if bytes[idx] == b'{' {
                depth += 1;
            } else if bytes[idx] == b'}' {
                depth -= 1;
            }
            if depth > 0 {
                idx += 1;
            }
        }

        // Parse match arms: `EnumName::VARIANT => "string",`
        let match_body = &result[match_start..idx];
        let arms = parse_enum_value_arms(match_body, &enum_name);
        if arms.is_empty() {
            search_from = abs_pos + marker.len();
            continue;
        }

        // Generate new() method
        let new_method = generate_enum_new_method(&enum_name, &arms);

        // Insert before `pub fn value`
        let insert_pos = abs_pos;
        result.insert_str(insert_pos, &new_method);
        search_from = insert_pos + new_method.len() + marker.len();
    }

    result
}

pub(super) fn extract_enum_name_from_impl(block: &str) -> String {
    // Find `impl NAME {` pattern
    for line in block.lines().rev() {
        let trimmed = line.trim();
        if trimmed.starts_with("impl ") && trimmed.contains('{') {
            let rest = &trimmed["impl ".len()..];
            let name_end = rest.find(|c: char| !c.is_alphanumeric() && c != '_');
            if let Some(end) = name_end {
                return rest[..end].to_string();
            }
        }
    }
    String::new()
}

pub(super) fn parse_enum_value_arms(match_body: &str, enum_name: &str) -> Vec<(String, String)> {
    let mut arms = Vec::new();
    let prefix = format!("{}::", enum_name);
    for line in match_body.lines() {
        let trimmed = line.trim();
        let rest = match trimmed.strip_prefix(&*prefix) {
            Some(r) => r,
            None => continue,
        };
        // Parse: `EnumName::VARIANT => "string",`
        let arrow_pos = match rest.find(" => ") {
            Some(p) => p,
            None => continue,
        };
        let variant = rest[..arrow_pos].trim().to_string();
        let value_part = rest[arrow_pos + " => ".len()..].trim();
        // Extract string value between quotes
        if let Some(after_quote) = value_part.strip_prefix('"') {
            let end_quote = match after_quote.find('"') {
                Some(p) => p,
                None => continue,
            };
            let string_val = after_quote[..end_quote].to_string();
            arms.push((variant, string_val));
        }
    }
    arms
}

pub(super) fn generate_enum_new_method(enum_name: &str, arms: &[(String, String)]) -> String {
    let mut method = String::new();
    method.push_str("    pub fn new(s: impl Into<String>) -> Self {\n");
    method.push_str("        let s = s.into();\n");
    method.push_str("        match s.as_str() {\n");
    for (variant, string_val) in arms {
        method.push_str(&format!(
            "            \"{}\" => {}::{},\n",
            string_val, enum_name, variant
        ));
    }
    // Default to first variant
    if let Some((first_variant, _)) = arms.first() {
        method.push_str(&format!(
            "            _ => {}::{},\n",
            enum_name, first_variant
        ));
    }
    method.push_str("        }\n");
    method.push_str("    }\n");
    method
}

pub(super) fn fix_enum_new_call_args(code: &str) -> String {
    if !code.contains("pub enum ") {
        return code.to_string();
    }
    // Collect enum names
    let enum_names: Vec<String> = code
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let rest = trimmed.strip_prefix("pub enum ")?;
            if !trimmed.ends_with('{') {
                return None;
            }
            let name_end = rest.find(|c: char| !c.is_alphanumeric() && c != '_');
            name_end.map(|end| rest[..end].to_string())
        })
        .collect();

    if enum_names.is_empty() {
        return code.to_string();
    }

    let mut result = code.to_string();
    for enum_name in &enum_names {
        let pattern = format!("{}::new(", enum_name);
        let mut search_from = 0;
        loop {
            let haystack = &result[search_from..];
            let rel_pos = match haystack.find(&pattern) {
                Some(p) => p,
                None => break,
            };
            let abs_pos = search_from + rel_pos;
            // Check this isn't a function definition (fn new)
            let before = &result[..abs_pos];
            if before.ends_with("fn ") || before.ends_with("pub fn ") {
                search_from = abs_pos + pattern.len();
                continue;
            }

            let args_start = abs_pos + pattern.len();
            // Find matching closing paren
            let mut depth = 1;
            let mut idx = args_start;
            let bytes = result.as_bytes();
            while idx < bytes.len() && depth > 0 {
                if bytes[idx] == b'(' {
                    depth += 1;
                } else if bytes[idx] == b')' {
                    depth -= 1;
                }
                if depth > 0 {
                    idx += 1;
                }
            }
            if depth != 0 {
                search_from = abs_pos + pattern.len();
                continue;
            }

            let args_str = &result[args_start..idx].to_string();
            // Count commas at top level to detect multi-arg calls
            let first_comma = find_top_level_comma(args_str);
            if let Some(comma_pos) = first_comma {
                // Keep only first arg
                let first_arg = args_str[..comma_pos].trim();
                let old = format!("{}{})", pattern, args_str);
                let new = format!("{}{})", pattern, first_arg);
                result = result.replacen(&old, &new, 1);
                search_from = abs_pos + new.len();
            } else {
                search_from = idx + 1;
            }
        }
    }
    result
}

pub(super) fn find_top_level_comma(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, ch) in s.char_indices() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// DEPYLER-CONVERGE-MULTI-ITER9: Add Display impl for enums.
///
/// Many generated enums need Display for string formatting.
pub(super) fn fix_enum_display(code: &str) -> String {
    if !code.contains("pub enum ") {
        return code.to_string();
    }
    let mut enum_impls = String::new();
    let lines: Vec<&str> = code.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("pub enum ") && trimmed.ends_with('{') {
            let name = trimmed
                .strip_prefix("pub enum ")
                .unwrap_or("")
                .trim_end_matches(" {")
                .trim()
                .to_string();
            if !name.is_empty() && !name.contains('<') {
                let mut variants: Vec<String> = Vec::new();
                i += 1;
                while i < lines.len() {
                    let vline = lines[i].trim();
                    if vline == "}" {
                        break;
                    }
                    let vname = vline.trim_end_matches(',').trim();
                    if !vname.is_empty() && !vname.starts_with("//") {
                        variants.push(vname.to_string());
                    }
                    i += 1;
                }
                if !variants.is_empty()
                    && !code.contains(&format!("impl std::fmt::Display for {}", name))
                {
                    enum_impls.push_str(&format!(
                        "\nimpl std::fmt::Display for {} {{\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        match self {{\n",
                        name
                    ));
                    for v in &variants {
                        let (vname, has_payload) = if let Some(paren) = v.find('(') {
                            (v[..paren].trim().to_string(), true)
                        } else {
                            (v.clone(), false)
                        };
                        if has_payload {
                            enum_impls.push_str(&format!(
                                "            {}::{}(..) => write!(f, \"{}\"),\n",
                                name, vname, vname
                            ));
                        } else {
                            enum_impls.push_str(&format!(
                                "            {}::{} => write!(f, \"{}\"),\n",
                                name, vname, vname
                            ));
                        }
                    }
                    enum_impls.push_str("        }\n    }\n}\n");
                }
            }
        }
        i += 1;
    }
    if enum_impls.is_empty() {
        code.to_string()
    } else {
        format!("{}{}", code, enum_impls)
    }
}

pub(super) fn fix_add_enum_from_impls(code: &str) -> String {
    let enum_names = collect_non_dv_enum_names(code);
    if enum_names.is_empty() {
        return code.to_string();
    }
    let mut impls_to_add = Vec::new();
    for name in &enum_names {
        let from_marker = format!("impl From<{}> for DepylerValue", name);
        if code.contains(&from_marker) {
            continue;
        }
        // Always add From impl for all enums since they may be used via .into()
        {
            impls_to_add.push(format!(
                "impl From<{name}> for DepylerValue {{\n    \
                 fn from(v: {name}) -> Self {{\n        \
                 DepylerValue::Str(format!(\"{{:?}}\", v))\n    \
                 }}\n}}\n"
            ));
        }
    }
    if impls_to_add.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    if let Some(main_pos) = result.find("\npub fn main()") {
        let insert = impls_to_add.join("\n");
        result.insert_str(main_pos, &format!("\n{}", insert));
    } else {
        result.push_str(&impls_to_add.join("\n"));
    }
    result
}

/// Collect names of all `pub enum X` that are NOT `DepylerValue`.
pub(super) fn collect_non_dv_enum_names(code: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in code.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("pub enum ") {
            let name = rest
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("");
            if !name.is_empty() && name != "DepylerValue" {
                names.push(name.to_string());
            }
        }
    }
    names
}

pub(super) fn extract_static_name(line: &str) -> String {
    let rest = line.trim().strip_prefix("pub static ").unwrap_or("");
    rest.split(':').next().unwrap_or("").trim().to_string()
}

pub(super) fn is_pascal_case(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let first = name.chars().next().unwrap_or('_');
    first.is_uppercase() && !name.chars().all(|c| c.is_uppercase() || c == '_')
}

pub(super) fn is_screaming_snake(name: &str) -> bool {
    !name.is_empty()
        && name.len() > 1
        && name
            .chars()
            .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
}

pub(super) fn skip_block(start: usize, lines: &[&str]) -> usize {
    let mut i = start;
    let trimmed = lines[i].trim();
    if trimmed.ends_with(';') {
        return i + 1;
    }
    let mut depth: i32 = 0;
    let mut found_opening = false;
    loop {
        let line = lines[i].trim();
        for c in line.chars() {
            match c {
                '{' | '(' => {
                    depth += 1;
                    found_opening = true;
                }
                '}' | ')' => depth -= 1,
                _ => {}
            }
        }
        i += 1;
        // Only break on depth <= 0 if we actually found an opening bracket
        if (found_opening && depth <= 0) || i >= lines.len() {
            break;
        }
        // Safety: if we've scanned 500 lines without resolution, bail
        if i - start > 500 {
            break;
        }
    }
    // Skip trailing semicolons or closing lines
    while i < lines.len() && lines[i].trim() == "});" {
        i += 1;
    }
    i
}

/// DEPYLER-CONVERGE-MULTI-ITER9: Remove orphaned LazyLock initializer bodies.
///
/// After type-alias and malformed-init corrections, multi-line LazyLock bodies
/// can remain as top-level code (not inside any `pub static`). Remove them.
pub(super) fn fix_orphaned_lazylock_bodies(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    let mut skip_orphan = false;
    let mut just_consumed_lazylock = false;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        // Detect orphaned LazyLock::new that's NOT part of a pub static assignment
        if trimmed.starts_with("std::sync::LazyLock::new(") && !is_continuation_of_static(i, &lines)
        {
            i = skip_block(i, &lines);
            just_consumed_lazylock = true;
            continue;
        }
        // Remove orphaned `.into_iter()` ONLY right after skip_block consumed a
        // LazyLock block or when skip_orphan is active. Never remove legitimate
        // method chains like `v\n.into_iter()\n.map(...)`.
        if (just_consumed_lazylock || skip_orphan)
            && (trimmed.starts_with(". into_iter()") || trimmed.starts_with(".into_iter()"))
        {
            i += 1;
            continue;
        }
        just_consumed_lazylock = false;
        // After a one-liner `LazyLock::new(|| Vec::new());`, skip orphaned body lines.
        if skip_orphan {
            if is_orphaned_lazylock_body_line(trimmed) {
                i += 1;
                continue;
            }
            skip_orphan = false;
        }
        // Check if this line triggers orphan-skip mode
        if trimmed.contains("LazyLock::new(|| Vec::new());") {
            skip_orphan = true;
        }
        result.push(lines[i].to_string());
        i += 1;
    }
    result.join("\n")
}

/// Check if a line looks like an orphaned LazyLock body statement at the top level.
pub(super) fn is_orphaned_lazylock_body_line(trimmed: &str) -> bool {
    if trimmed.is_empty() {
        return false;
    }
    // Valid top-level items - NOT orphaned
    if trimmed.starts_with("pub ")
        || trimmed.starts_with("fn ")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("enum ")
        || trimmed.starts_with("impl ")
        || trimmed.starts_with("impl<")
        || trimmed.starts_with("use ")
        || trimmed.starts_with("const ")
        || trimmed.starts_with("static ")
        || trimmed.starts_with("#[")
        || trimmed.starts_with("#![")
        || trimmed.starts_with("//")
        || trimmed.starts_with("type ")
        || trimmed.starts_with("mod ")
        || trimmed.starts_with("trait ")
        || trimmed.starts_with("extern ")
    {
        return false;
    }
    // Orphaned body patterns from LazyLock replacements
    trimmed.starts_with("set.insert(")
        || trimmed.starts_with("let mut set")
        || trimmed == "set"
        || trimmed == "}"
        || trimmed == "});"
        || trimmed == "]"
        || trimmed == "]);"
        || trimmed == "]),"
        || trimmed.starts_with(". into_iter()")
        || trimmed.starts_with(".into_iter()")
        || trimmed.starts_with(".unwrap()")
        || trimmed.starts_with(".collect::<")
        // Vec literal items: `"item".to_string(),` or `"item".to_string()`
        || (trimmed.starts_with('"') && trimmed.contains(".to_string()"))
        // Vec literal opening/closing: `vec![` or bare `[`
        || trimmed == "vec!["
}

/// Check if a LazyLock::new line is a continuation of a pub static on the previous line.
pub(super) fn is_continuation_of_static(idx: usize, lines: &[&str]) -> bool {
    if idx == 0 {
        return false;
    }
    let prev = lines[idx - 1].trim();
    prev.contains("pub static ") && prev.ends_with('=')
}

fn count_parens_open(s: &str) -> i32 {
    s.chars().filter(|&c| c == '(').count() as i32
}

fn count_parens_close(s: &str) -> i32 {
    s.chars().filter(|&c| c == ')').count() as i32
}
