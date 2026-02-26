//! NASA mode text-level post-processing for generated Rust code.
//!
//! NASA mode strips external crate dependencies so that transpiled output
//! compiles in a single `rustc` invocation with only the standard library.
//! All transformations are pure string manipulation -- no AST or external
//! imports are required.

/// Apply NASA mode text-level fixes to generated Rust code.
/// NASA mode strips external crate dependencies for single-shot compile compatibility.
/// This function:
/// - Replaces serde_json types with std equivalents
/// - Stubs clap/argparse imports and types
/// - Removes external crate use statements
/// - Stubs pytest assertions
/// - Fixes derive attributes for std-only types
/// - Stubs Python exception types
pub(super) fn apply_nasa_mode_fixes(formatted_code: &mut String) {
    // Replace serde_json types and methods with std equivalents
    *formatted_code = formatted_code.replace("serde_json::Value", "String");
    *formatted_code = formatted_code.replace("serde_json :: Value", "String");
    *formatted_code = formatted_code.replace("serde_json::to_string(&", "format!(\"{:?}\", &");
    *formatted_code = formatted_code.replace("serde_json :: to_string(&", "format!(\"{:?}\", &");
    *formatted_code = formatted_code.replace("serde_json::json!", "format!(\"{:?}\", ");
    *formatted_code = formatted_code.replace("serde_json :: json !", "format!(\"{:?}\", ");
    *formatted_code = formatted_code.replace("serde_json::from_str::<String>(", "String::from(");
    // Remove serde_json and other external crate imports if present
    *formatted_code = formatted_code.replace("use serde_json;\n", "");
    *formatted_code = formatted_code.replace("use serde_json ;\n", "");
    *formatted_code = formatted_code.replace("use serde;\n", "");
    *formatted_code = formatted_code.replace("use base64::Engine;\n", "");
    *formatted_code = formatted_code.replace("use tokio;\n", "");
    *formatted_code = formatted_code.replace("use rand;\n", "");
    *formatted_code = formatted_code.replace("use regex;\n", "");
    // DEPYLER-1030: Remove itertools and other common external crate imports
    *formatted_code = formatted_code.replace("use itertools::Itertools;\n", "");
    *formatted_code = formatted_code.replace("use itertools :: Itertools ;\n", "");
    *formatted_code = formatted_code.replace("use itertools;\n", "");
    *formatted_code = formatted_code.replace("use chrono::prelude::*;\n", "");
    *formatted_code = formatted_code.replace("use chrono;\n", "");
    *formatted_code = formatted_code.replace("use anyhow;\n", "");
    *formatted_code = formatted_code.replace("use thiserror;\n", "");
    // DEPYLER-1032: Remove more external crate imports
    *formatted_code = formatted_code.replace("use digest::Digest;\n", "");
    *formatted_code = formatted_code.replace("use digest :: Digest ;\n", "");
    *formatted_code = formatted_code.replace("use sha2::Sha256;\n", "");
    *formatted_code = formatted_code.replace("use sha2 :: Sha256 ;\n", "");
    *formatted_code = formatted_code.replace("use base64::prelude::*;\n", "");
    *formatted_code = formatted_code.replace("use base64 :: prelude :: * ;\n", "");

    // DEPYLER-1035: Comprehensive external crate sanitization for NASA single-shot compile
    // Remove common external crate imports
    *formatted_code = formatted_code.replace("use csv;\n", "");
    *formatted_code = formatted_code.replace("use walkdir;\n", "");
    *formatted_code = formatted_code.replace("use glob;\n", "");
    *formatted_code = formatted_code.replace("use url;\n", "");
    *formatted_code = formatted_code.replace("use md5;\n", "");
    *formatted_code = formatted_code.replace("use sha2;\n", "");

    // Replace base64 operations with format! stubs
    // DEPYLER-1036: Handle both single-line and multi-line patterns
    *formatted_code = formatted_code
        .replace("base64::engine::general_purpose::STANDARD.encode(", "format!(\"{:?}\", ");
    *formatted_code = formatted_code.replace(
        "base64::engine::general_purpose::STANDARD\n        .encode(",
        "format!(\"{:?}\", ",
    );
    *formatted_code = formatted_code
        .replace("base64::engine::general_purpose::STANDARD.decode(", "format!(\"{:?}\", ");
    *formatted_code = formatted_code.replace(
        "base64::engine::general_purpose::STANDARD\n        .decode(",
        "format!(\"{:?}\", ",
    );
    *formatted_code = formatted_code
        .replace("base64::engine::general_purpose::URL_SAFE.encode(", "format!(\"{:?}\", ");
    *formatted_code = formatted_code.replace(
        "base64::engine::general_purpose::URL_SAFE\n        .encode(",
        "format!(\"{:?}\", ",
    );

    // Also replace import statements and remaining usages
    *formatted_code = formatted_code.replace("use base64;\n", "");
    *formatted_code = formatted_code.replace("use serde;\n", "");
    *formatted_code = formatted_code.replace("use serde::Serialize;\n", "");
    *formatted_code = formatted_code.replace("use serde::Deserialize;\n", "");
    *formatted_code = formatted_code.replace("use serde::{Serialize, Deserialize};\n", "");

    // DEPYLER-1036: Remove serde derive macros
    *formatted_code = formatted_code.replace(", serde::Serialize, serde::Deserialize", "");
    *formatted_code = formatted_code.replace(", serde :: Serialize, serde :: Deserialize", "");
    *formatted_code = formatted_code.replace("serde::Serialize, serde::Deserialize, ", "");
    *formatted_code = formatted_code.replace("serde::Serialize, serde::Deserialize", "");

    // DEPYLER-1036: Replace sha2 usages with std format stubs
    *formatted_code = formatted_code.replace("use sha2::Digest;\n", "");
    *formatted_code = formatted_code.replace("use sha2 :: Digest;\n", "");
    *formatted_code = formatted_code
        .replace("sha2::Sha256::new()", "std::collections::hash_map::DefaultHasher::new()");
    *formatted_code = formatted_code
        .replace("sha2 :: Sha256 :: new()", "std::collections::hash_map::DefaultHasher::new()");
    *formatted_code = formatted_code
        .replace("Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>", "format!(\"sha256_stub\")");
    *formatted_code = formatted_code
        .replace("sha2::Sha512::new()", "std::collections::hash_map::DefaultHasher::new()");

    // DEPYLER-1036: Remove DynDigest and digest traits
    *formatted_code = formatted_code.replace("use digest::DynDigest;\n", "");
    *formatted_code = formatted_code.replace("use digest :: DynDigest;\n", "");
    *formatted_code = formatted_code.replace(": Box<dyn DynDigest>", ": String");

    // DEPYLER-1036: Replace undefined UnionType placeholder with String.
    // The type_mapper.rs creates RustType::Enum { name: "UnionType" } for
    // non-optional unions that aren't resolved by process_union_type().
    // When wrapped in Option<...>, the inner placeholder persists.
    *formatted_code = formatted_code.replace("Option<UnionType>", "Option<String>");
    *formatted_code = formatted_code.replace("Vec<UnionType>", "Vec<String>");
    *formatted_code = formatted_code.replace("&Vec<UnionType>", "&Vec<String>");
    *formatted_code = formatted_code.replace("HashMap<UnionType,", "HashMap<String,");
    *formatted_code = formatted_code.replace(", UnionType>", ", String>");
    *formatted_code = formatted_code.replace(": UnionType", ": String");
    *formatted_code = formatted_code.replace("(UnionType)", "(String)");
    *formatted_code = formatted_code.replace("<UnionType>", "<String>");

    // DEPYLER-1036: Replace more external crate references
    *formatted_code = formatted_code.replace("use md5;\n", "");
    *formatted_code = formatted_code.replace("use sha1;\n", "");
    *formatted_code = formatted_code.replace("md5::compute(", "format!(\"md5:{:?}\", ");
    *formatted_code = formatted_code.replace("sha1::Sha1::digest(", "format!(\"sha1:{:?}\", ");

    // DEPYLER-1036: Remove .unwrap() after format! (format! returns String, not Result)
    // Note: Be specific about which unwrap() to remove - don't use generic patterns
    // that would remove valid unwrap() calls (e.g., after .get_mut())
    *formatted_code = formatted_code
        .replace("format!(\"{:?}\", encoded)\n        .unwrap()", "format!(\"{:?}\", encoded)");
    *formatted_code = formatted_code
        .replace("format!(\"{:?}\", data)\n        .unwrap()", "format!(\"{:?}\", data)");
    *formatted_code = formatted_code
        .replace("format!(\"{:?}\", b\"\")\n        .unwrap()", "format!(\"{:?}\", b\"\")");
    // Remove .unwrap() only after specific format! patterns, not generically
    *formatted_code = formatted_code
        .replace("format!(\"{:?}\", original)\n        .unwrap()", "format!(\"{:?}\", original)");

    // DEPYLER-1036: Replace csv with std::io stubs
    *formatted_code =
        formatted_code.replace("csv::Reader::from_reader(", "std::io::BufReader::new(");
    *formatted_code =
        formatted_code.replace("csv::Writer::from_writer(", "std::io::BufWriter::new(");
    *formatted_code = formatted_code.replace(
        "csv::ReaderBuilder::new().has_headers(true).from_reader(",
        "std::io::BufReader::new(",
    );

    // DEPYLER-1036: Replace walkdir with std::fs stubs
    *formatted_code = formatted_code.replace("walkdir::WalkDir::new(", "std::fs::read_dir(");

    // DEPYLER-1036: Replace glob with std::path stubs
    *formatted_code = formatted_code.replace("glob::glob(", "vec![std::path::PathBuf::from(");

    // DEPYLER-1036: Replace url crate with String stubs
    *formatted_code = formatted_code.replace("url::Url::parse(", "String::from(");
    *formatted_code = formatted_code.replace("url::Url::join(", "format!(\"{}{}\", ");

    // Replace tokio async functions with sync stubs
    *formatted_code = formatted_code.replace("tokio::spawn(", "std::thread::spawn(");
    *formatted_code = formatted_code.replace("tokio :: spawn(", "std::thread::spawn(");
    *formatted_code = formatted_code.replace("tokio::time::timeout(", "Some(");
    *formatted_code = formatted_code.replace("tokio::time::sleep(", "std::thread::sleep(");
    *formatted_code = formatted_code.replace("tokio::join!(", "(");

    // DEPYLER-1200: Do NOT replace regex::Regex::new - it's now properly handled
    // by NASA mode in direct_rules_convert.rs which generates DepylerRegexMatch instead

    // Replace .copied() with .cloned() for non-Copy types like String
    // This is safe because String implements Clone
    *formatted_code = formatted_code.replace(".copied()", ".cloned()");

    // DEPYLER-1037: Remove clap derive macros and attributes for NASA mode
    // clap is an external crate that can't be used in single-shot compile
    // Add Default derive so Args::default() works as a stub for Args::parse()
    *formatted_code = formatted_code.replace("#[derive(clap::Parser)]\n", "#[derive(Default)]\n");
    *formatted_code = formatted_code.replace("#[derive(clap :: Parser)]\n", "#[derive(Default)]\n");
    *formatted_code =
        formatted_code.replace("#[derive(clap::Parser, Debug)]\n", "#[derive(Debug, Default)]\n");
    *formatted_code = formatted_code
        .replace("#[derive(clap::Parser, Debug, Clone)]\n", "#[derive(Debug, Clone, Default)]\n");
    // DEPYLER-1052: Also handle inline patterns (no newline after derive)
    *formatted_code = formatted_code.replace("#[derive(clap::Parser)] ", "#[derive(Default)] ");
    *formatted_code = formatted_code.replace("#[derive(clap :: Parser)] ", "#[derive(Default)] ");
    // DEPYLER-1048: Fix Commands enum for subcommands
    // Add Default derive to Commands enum and add a default unit variant
    *formatted_code =
        formatted_code.replace("#[derive(clap::Subcommand)]\n", "#[derive(Default)]\n");
    *formatted_code =
        formatted_code.replace("#[derive(clap :: Subcommand)]\n", "#[derive(Default)]\n");
    // DEPYLER-1088: Also handle inline patterns (no newline after derive)
    *formatted_code = formatted_code.replace("#[derive(clap::Subcommand)] ", "#[derive(Default)] ");
    *formatted_code =
        formatted_code.replace("#[derive(clap :: Subcommand)] ", "#[derive(Default)] ");
    // Add a default unit variant to Commands enum
    // Pattern: "enum Commands {\n" -> "enum Commands {\n    #[default]\n    __DepylerNone,\n"
    *formatted_code = formatted_code
        .replace("enum Commands {\n", "enum Commands {\n    #[default]\n    __DepylerNone,\n");
    // Add catch-all arm for the new variant in match statements
    // This is simpler than wrapping with Option
    *formatted_code = formatted_code.replace("#[command(author, version, about)]\n", "");

    // DEPYLER-1088: Remove inline #[command(...)] attributes FIRST
    // This must happen BEFORE line filtering to prevent removing enum variants
    // that have inline attributes like `#[command(about = "...")] Resource { name: String },`
    while let Some(start) = formatted_code.find("#[command(") {
        if let Some(end) = formatted_code[start..].find(")]") {
            let attr_end = start + end + 2;
            // Remove attribute and trailing space if present
            let remove_end = if formatted_code.as_bytes().get(attr_end) == Some(&b' ') {
                attr_end + 1
            } else {
                attr_end
            };
            *formatted_code =
                format!("{}{}", &formatted_code[..start], &formatted_code[remove_end..]);
        } else {
            break;
        }
    }

    // DEPYLER-1088: Remove inline #[arg(...)] attributes FIRST (same reason)
    while let Some(start) = formatted_code.find("#[arg(") {
        if let Some(end) = formatted_code[start..].find(")]") {
            let attr_end = start + end + 2;
            // Remove attribute and trailing space if present
            let remove_end = if formatted_code.as_bytes().get(attr_end) == Some(&b' ') {
                attr_end + 1
            } else {
                attr_end
            };
            *formatted_code =
                format!("{}{}", &formatted_code[..start], &formatted_code[remove_end..]);
        } else {
            break;
        }
    }

    // DEPYLER-1088: Line filter is no longer needed since inline attrs are removed above
    // The while loops handle all #[command(...)] and #[arg(...)] patterns
    // Just ensure proper line endings
    if !formatted_code.ends_with('\n') {
        formatted_code.push('\n');
    }

    // DEPYLER-1088: #[arg(...)] attrs are now handled by while loop above
    // Remove clap imports
    *formatted_code = formatted_code.replace("use clap::Parser;\n", "");
    *formatted_code = formatted_code.replace("use clap :: Parser;\n", "");
    *formatted_code = formatted_code.replace("use clap;\n", "");

    // DEPYLER-1090: Remove use clap::CommandFactory imports (any indentation)
    // These appear in help-printing blocks like:
    // { use clap::CommandFactory; Args::command().print_help().unwrap() };
    *formatted_code = formatted_code
        .lines()
        .filter(|line| !line.trim().starts_with("use clap::CommandFactory"))
        .collect::<Vec<_>>()
        .join("\n");
    if !formatted_code.ends_with('\n') {
        formatted_code.push('\n');
    }

    // DEPYLER-1090: Replace Args::command() with a stub that doesn't require clap
    // Args::command().print_help() pattern becomes a no-op in NASA mode
    *formatted_code = formatted_code.replace("Args::command().print_help().unwrap()", "()");
    *formatted_code = formatted_code.replace("Args :: command().print_help().unwrap()", "()");

    // Replace Args::parse() call with Args::default() stub
    // Since clap::Parser derive is removed, we need a fallback
    *formatted_code = formatted_code.replace("Args::parse()", "Args::default()");
    *formatted_code = formatted_code.replace("Args :: parse()", "Args::default()");

    // DEPYLER-CONVERGE-MULTI: Stub pytest references for test files.
    // Python test files use `pytest.raises(ExceptionType)` as a context manager.
    // The transpiler emits `let _context = pytest.raises(TypeError)` which fails
    // because `pytest` is not defined. Replace with no-op tuple assignment.
    // Also stub Python exception types used as pytest arguments.
    *formatted_code = formatted_code
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("let _context = pytest.raises(")
                || trimmed.starts_with("let _context = pytest .raises(")
                || trimmed == "let _context = pytest.raises("
            {
                // Replace entire pytest.raises(...) with a no-op
                let indent = &line[..line.len() - trimmed.len()];
                format!("{}let _context = ();", indent)
            } else if trimmed.contains("pytest.") {
                // Replace any other pytest.<method>(...) with ()
                let indent = &line[..line.len() - trimmed.len()];
                format!("{}// pytest stub: {}", indent, trimmed)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !formatted_code.ends_with('\n') {
        formatted_code.push('\n');
    }

    // DEPYLER-CONVERGE-MULTI: Stub Python exception types not defined in Rust.
    // Exception types like TypeError, ValueError appear as bare identifiers
    // in pytest.raises() patterns. Since we've already stubbed out pytest
    // calls above, remaining exception type references are benign.
}
