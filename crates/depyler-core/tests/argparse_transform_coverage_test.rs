//! Targeted coverage tests for argparse_transform module
//!
//! Target: argparse_transform.rs (43.62% coverage → 80%+)
//! Focus: Unit tests for type conversion, struct generation, enum generation
//!
//! Coverage Strategy:
//! - Test all type_to_rust_string variants
//! - Test ArgParserInfo/ArgParserArgument construction
//! - Test generate_args_struct with various configurations
//! - Test generate_commands_enum for subcommands
//! - Test edge cases: dest, metavar, choices, actions

use depyler_core::rust_gen::{
    ArgParserArgument, ArgParserInfo, ArgParserTracker, SubcommandInfo, SubparserInfo,
    generate_args_struct, generate_commands_enum,
};
use depyler_core::hir::{HirExpr, Literal, Type};

// ============================================================================
// ArgParserInfo Tests
// ============================================================================

#[test]
fn test_argparser_info_new() {
    let info = ArgParserInfo::new("parser".to_string());
    assert_eq!(info.parser_var, "parser");
    assert!(info.description.is_none());
    assert!(info.epilog.is_none());
    assert!(info.arguments.is_empty());
    assert!(info.args_var.is_none());
}

#[test]
fn test_argparser_info_add_argument() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let arg = ArgParserArgument::new("--verbose".to_string());
    info.add_argument(arg);
    assert_eq!(info.arguments.len(), 1);
    assert_eq!(info.arguments[0].name, "--verbose");
}

#[test]
fn test_argparser_info_set_args_var() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.set_args_var("args".to_string());
    assert_eq!(info.args_var, Some("args".to_string()));
}

#[test]
fn test_argparser_info_with_description() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.description = Some("Test program".to_string());
    info.epilog = Some("Example usage".to_string());
    assert_eq!(info.description, Some("Test program".to_string()));
    assert_eq!(info.epilog, Some("Example usage".to_string()));
}

// ============================================================================
// ArgParserArgument Tests
// ============================================================================

#[test]
fn test_argparser_argument_positional() {
    let arg = ArgParserArgument::new("filename".to_string());
    assert!(arg.is_positional);
    assert_eq!(arg.name, "filename");
    assert!(arg.long.is_none());
}

#[test]
fn test_argparser_argument_short_flag() {
    let arg = ArgParserArgument::new("-v".to_string());
    assert!(!arg.is_positional);
    assert_eq!(arg.name, "-v");
}

#[test]
fn test_argparser_argument_long_flag() {
    let arg = ArgParserArgument::new("--verbose".to_string());
    assert!(!arg.is_positional);
    assert_eq!(arg.name, "--verbose");
}

#[test]
fn test_argparser_argument_rust_field_name_positional() {
    let arg = ArgParserArgument::new("input_file".to_string());
    assert_eq!(arg.rust_field_name(), "input_file");
}

#[test]
fn test_argparser_argument_rust_field_name_short() {
    let arg = ArgParserArgument::new("-v".to_string());
    assert_eq!(arg.rust_field_name(), "v");
}

#[test]
fn test_argparser_argument_rust_field_name_long() {
    let arg = ArgParserArgument::new("--output-dir".to_string());
    assert_eq!(arg.rust_field_name(), "output_dir");
}

#[test]
fn test_argparser_argument_rust_field_name_with_dest() {
    let mut arg = ArgParserArgument::new("-o".to_string());
    arg.dest = Some("output_path".to_string());
    assert_eq!(arg.rust_field_name(), "output_path");
}

#[test]
fn test_argparser_argument_rust_field_name_with_long() {
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.long = Some("--verbose".to_string());
    // When both short and long, use long for field name
    assert_eq!(arg.rust_field_name(), "verbose");
}

#[test]
fn test_argparser_argument_rust_type_string() {
    let arg = ArgParserArgument::new("name".to_string());
    assert_eq!(arg.rust_type(), "String");
}

#[test]
fn test_argparser_argument_rust_type_int() {
    let mut arg = ArgParserArgument::new("count".to_string());
    arg.arg_type = Some(Type::Int);
    assert_eq!(arg.rust_type(), "i32");
}

#[test]
fn test_argparser_argument_rust_type_float() {
    let mut arg = ArgParserArgument::new("rate".to_string());
    arg.arg_type = Some(Type::Float);
    assert_eq!(arg.rust_type(), "f64");
}

#[test]
fn test_argparser_argument_rust_type_bool() {
    // Note: Bool type for a non-positional flag wraps in Option unless has action
    let mut arg = ArgParserArgument::new("--flag".to_string());
    arg.arg_type = Some(Type::Bool);
    // Without action, it's Option<bool>
    assert!(arg.rust_type().contains("bool"));
}

#[test]
fn test_argparser_argument_rust_type_pathbuf() {
    let mut arg = ArgParserArgument::new("path".to_string());
    arg.arg_type = Some(Type::Custom("PathBuf".to_string()));
    assert_eq!(arg.rust_type(), "PathBuf");
}

#[test]
fn test_argparser_argument_rust_type_store_true() {
    let mut arg = ArgParserArgument::new("--verbose".to_string());
    arg.action = Some("store_true".to_string());
    assert_eq!(arg.rust_type(), "bool");
}

#[test]
fn test_argparser_argument_rust_type_store_false() {
    let mut arg = ArgParserArgument::new("--no-cache".to_string());
    arg.action = Some("store_false".to_string());
    assert_eq!(arg.rust_type(), "bool");
}

#[test]
fn test_argparser_argument_rust_type_count() {
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.action = Some("count".to_string());
    assert_eq!(arg.rust_type(), "u8");
}

#[test]
fn test_argparser_argument_rust_type_nargs_plus() {
    let mut arg = ArgParserArgument::new("files".to_string());
    arg.nargs = Some("+".to_string());
    assert!(arg.rust_type().starts_with("Vec<"));
}

#[test]
fn test_argparser_argument_rust_type_nargs_star() {
    let mut arg = ArgParserArgument::new("items".to_string());
    arg.nargs = Some("*".to_string());
    assert!(arg.rust_type().starts_with("Vec<"));
}

#[test]
fn test_argparser_argument_rust_type_nargs_optional() {
    let mut arg = ArgParserArgument::new("--config".to_string());
    arg.nargs = Some("?".to_string());
    assert!(arg.rust_type().starts_with("Option<"));
}

#[test]
fn test_argparser_argument_rust_type_append() {
    let mut arg = ArgParserArgument::new("--include".to_string());
    arg.action = Some("append".to_string());
    assert!(arg.rust_type().starts_with("Vec<"));
}

#[test]
fn test_argparser_argument_with_help() {
    let mut arg = ArgParserArgument::new("--output".to_string());
    arg.help = Some("Output file path".to_string());
    assert_eq!(arg.help, Some("Output file path".to_string()));
}

#[test]
fn test_argparser_argument_with_default_int() {
    let mut arg = ArgParserArgument::new("--count".to_string());
    arg.default = Some(HirExpr::Literal(Literal::Int(10)));
    assert!(arg.default.is_some());
}

#[test]
fn test_argparser_argument_with_default_string() {
    let mut arg = ArgParserArgument::new("--name".to_string());
    arg.default = Some(HirExpr::Literal(Literal::String("default".to_string())));
    assert!(arg.default.is_some());
}

#[test]
fn test_argparser_argument_with_metavar() {
    let mut arg = ArgParserArgument::new("--file".to_string());
    arg.metavar = Some("FILE".to_string());
    assert_eq!(arg.metavar, Some("FILE".to_string()));
}

#[test]
fn test_argparser_argument_with_choices() {
    let mut arg = ArgParserArgument::new("--format".to_string());
    arg.choices = Some(vec!["json".to_string(), "yaml".to_string(), "toml".to_string()]);
    assert_eq!(arg.choices.as_ref().unwrap().len(), 3);
}

#[test]
fn test_argparser_argument_required() {
    let mut arg = ArgParserArgument::new("--config".to_string());
    arg.required = Some(true);
    assert_eq!(arg.required, Some(true));
}

#[test]
fn test_argparser_argument_const_value() {
    let mut arg = ArgParserArgument::new("--debug".to_string());
    arg.const_value = Some(HirExpr::Literal(Literal::Bool(true)));
    assert!(arg.const_value.is_some());
}

// ============================================================================
// ArgParserTracker Tests
// ============================================================================

#[test]
fn test_argparser_tracker_new() {
    let tracker = ArgParserTracker::new();
    assert!(!tracker.has_parsers());
}

#[test]
fn test_argparser_tracker_register_parser() {
    let mut tracker = ArgParserTracker::new();
    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);
    assert!(tracker.has_parsers());
    assert!(tracker.get_parser("parser").is_some());
}

#[test]
fn test_argparser_tracker_get_parser_mut() {
    let mut tracker = ArgParserTracker::new();
    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);

    let parser = tracker.get_parser_mut("parser").unwrap();
    parser.description = Some("Modified".to_string());

    assert_eq!(tracker.get_parser("parser").unwrap().description, Some("Modified".to_string()));
}

#[test]
fn test_argparser_tracker_register_group() {
    let mut tracker = ArgParserTracker::new();
    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);
    tracker.register_group("input_group".to_string(), "parser".to_string());

    assert!(tracker.get_parser_for_group("input_group").is_some());
}

#[test]
fn test_argparser_tracker_has_subcommands() {
    let mut tracker = ArgParserTracker::new();
    assert!(!tracker.has_subcommands());

    // Register subparsers first
    tracker.register_subparsers("subparsers".to_string(), SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: None,
    });

    // Then register a subcommand (has_subcommands checks subcommands map)
    tracker.register_subcommand("clone_parser".to_string(), SubcommandInfo {
        name: "clone".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    });

    assert!(tracker.has_subcommands());
}

#[test]
fn test_argparser_tracker_register_subcommand() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_subparsers("subparsers".to_string(), SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: None,
    });

    tracker.register_subcommand("clone_parser".to_string(), SubcommandInfo {
        name: "clone".to_string(),
        help: Some("Clone a repository".to_string()),
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    });

    assert!(tracker.get_subcommand("clone_parser").is_some());
}

#[test]
fn test_argparser_tracker_get_subcommand_mut() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_subparsers("subparsers".to_string(), SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: None,
    });

    tracker.register_subcommand("push_parser".to_string(), SubcommandInfo {
        name: "push".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    });

    let subcmd = tracker.get_subcommand_mut("push_parser").unwrap();
    subcmd.help = Some("Push changes".to_string());

    assert_eq!(tracker.get_subcommand("push_parser").unwrap().help, Some("Push changes".to_string()));
}

// ============================================================================
// generate_args_struct Tests
// ============================================================================

#[test]
fn test_generate_args_struct_empty() {
    let info = ArgParserInfo::new("parser".to_string());
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    // Verify struct generation - check for actual output
    assert!(code.contains("Args") || code.contains("struct"), "Should contain Args struct: {}", code);
}

#[test]
fn test_generate_args_struct_with_description() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.description = Some("Test CLI tool".to_string());
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("about"));
    assert!(code.contains("Test CLI tool"));
}

#[test]
fn test_generate_args_struct_positional_arg() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.add_argument(ArgParserArgument::new("filename".to_string()));
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("filename"));
    assert!(code.contains("String"));
}

#[test]
fn test_generate_args_struct_short_flag() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.action = Some("store_true".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("short"));
    assert!(code.contains("bool"));
}

#[test]
fn test_generate_args_struct_long_flag() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("--verbose".to_string());
    arg.action = Some("store_true".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("long"));
}

#[test]
fn test_generate_args_struct_short_and_long() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.long = Some("--verbose".to_string());
    arg.action = Some("store_true".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("short"));
    assert!(code.contains("long"));
}

#[test]
fn test_generate_args_struct_with_default() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("--count".to_string());
    arg.arg_type = Some(Type::Int);
    arg.default = Some(HirExpr::Literal(Literal::Int(5)));
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("default_value"));
}

#[test]
fn test_generate_args_struct_count_action() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.action = Some("count".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("Count"));
    assert!(code.contains("u8"));
}

#[test]
fn test_generate_args_struct_store_false() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("--no-cache".to_string());
    arg.action = Some("store_false".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("SetFalse"));
    assert!(code.contains("default_value_t = true"));
}

#[test]
fn test_generate_args_struct_nargs_plus() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("files".to_string());
    arg.nargs = Some("+".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("Vec"));
}

#[test]
fn test_generate_args_struct_with_metavar() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("--output".to_string());
    arg.metavar = Some("FILE".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("value_name"));
    assert!(code.contains("FILE"));
}

#[test]
fn test_generate_args_struct_with_choices() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("--format".to_string());
    arg.choices = Some(vec!["json".to_string(), "yaml".to_string()]);
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    assert!(code.contains("value_parser"));
}

#[test]
fn test_generate_args_struct_required_flag() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let mut arg = ArgParserArgument::new("--config".to_string());
    arg.required = Some(true);
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    // Required flags should NOT be wrapped in Option
    assert!(code.contains("config"));
}

#[test]
fn test_generate_args_struct_optional_flag() {
    let mut info = ArgParserInfo::new("parser".to_string());
    let arg = ArgParserArgument::new("--config".to_string());
    info.add_argument(arg);
    let tracker = ArgParserTracker::new();

    let tokens = generate_args_struct(&info, &tracker);
    let code = tokens.to_string();

    // Optional flags should be wrapped in Option
    assert!(code.contains("Option"));
}

// ============================================================================
// generate_commands_enum Tests
// ============================================================================

#[test]
fn test_generate_commands_enum_empty() {
    let tracker = ArgParserTracker::new();
    let tokens = generate_commands_enum(&tracker);
    let code = tokens.to_string();

    // Empty tracker should produce empty output
    assert!(code.is_empty() || code.trim().is_empty());
}

#[test]
fn test_generate_commands_enum_single_subcommand() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_subparsers("subparsers".to_string(), SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: None,
    });

    tracker.register_subcommand("clone_parser".to_string(), SubcommandInfo {
        name: "clone".to_string(),
        help: Some("Clone a repository".to_string()),
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    });

    let tokens = generate_commands_enum(&tracker);
    let code = tokens.to_string();

    assert!(code.contains("enum Commands"));
    assert!(code.contains("Clone"));
}

#[test]
fn test_generate_commands_enum_multiple_subcommands() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_subparsers("subparsers".to_string(), SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: None,
    });

    tracker.register_subcommand("clone_parser".to_string(), SubcommandInfo {
        name: "clone".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    });

    tracker.register_subcommand("push_parser".to_string(), SubcommandInfo {
        name: "push".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    });

    let tokens = generate_commands_enum(&tracker);
    let code = tokens.to_string();

    assert!(code.contains("Clone"));
    assert!(code.contains("Push"));
}

#[test]
fn test_generate_commands_enum_with_args() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_subparsers("subparsers".to_string(), SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: None,
    });

    let arg = ArgParserArgument::new("url".to_string());
    tracker.register_subcommand("clone_parser".to_string(), SubcommandInfo {
        name: "clone".to_string(),
        help: None,
        arguments: vec![arg],
        subparsers_var: "subparsers".to_string(),
    });

    let tokens = generate_commands_enum(&tracker);
    let code = tokens.to_string();

    assert!(code.contains("Clone"));
    assert!(code.contains("url"));
}

// ============================================================================
// Integration Tests via Pipeline
// ============================================================================

#[test]
fn test_argparse_basic_via_pipeline() {
    use depyler_core::DepylerPipeline;

    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Test CLI")
    parser.add_argument("filename", help="Input file")
    args = parser.parse_args()
    print(args.filename)
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    assert!(rust_code.contains("struct Args"));
    assert!(rust_code.contains("filename"));
}

#[test]
fn test_argparse_flags_via_pipeline() {
    use depyler_core::DepylerPipeline;

    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-c", "--count", type=int, default=1)
    args = parser.parse_args()
    return 0
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    assert!(rust_code.contains("verbose"));
    assert!(rust_code.contains("count"));
}

#[test]
fn test_argparse_nargs_via_pipeline() {
    use depyler_core::DepylerPipeline;

    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()
    for f in args.files:
        print(f)
"#;

    let result = pipeline.transpile(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    assert!(rust_code.contains("Vec<"));
}
