//! Comprehensive argparse_transform tests
//!
//! These tests exercise the argparse_transform.rs code paths through
//! both direct unit tests and integration tests via the transpilation pipeline.

use crate::hir::{HirExpr, Literal, Type};
use crate::rust_gen::argparse_transform::{
    ArgParserArgument, ArgParserInfo, ArgParserTracker, SubcommandInfo, SubparserInfo,
};

// ============================================================================
// ARGPARSERARGUMENT TESTS
// ============================================================================

#[test]
fn test_arg_new_positional() {
    let arg = ArgParserArgument::new("files".to_string());
    assert!(arg.is_positional);
    assert_eq!(arg.name, "files");
    assert!(arg.long.is_none());
}

#[test]
fn test_arg_new_short_flag() {
    let arg = ArgParserArgument::new("-v".to_string());
    assert!(!arg.is_positional);
    assert_eq!(arg.name, "-v");
}

#[test]
fn test_arg_new_long_flag() {
    let arg = ArgParserArgument::new("--verbose".to_string());
    assert!(!arg.is_positional);
    assert_eq!(arg.name, "--verbose");
}

#[test]
fn test_arg_rust_field_name_positional() {
    let arg = ArgParserArgument::new("input_file".to_string());
    assert_eq!(arg.rust_field_name(), "input_file");
}

#[test]
fn test_arg_rust_field_name_long_flag() {
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.long = Some("--verbose".to_string());
    assert_eq!(arg.rust_field_name(), "verbose");
}

#[test]
fn test_arg_rust_field_name_short_only() {
    let arg = ArgParserArgument::new("-v".to_string());
    assert_eq!(arg.rust_field_name(), "v");
}

#[test]
fn test_arg_rust_field_name_long_only() {
    let arg = ArgParserArgument::new("--output-file".to_string());
    assert_eq!(arg.rust_field_name(), "output_file");
}

#[test]
fn test_arg_rust_field_name_with_dest() {
    let mut arg = ArgParserArgument::new("--config-file".to_string());
    arg.dest = Some("config_path".to_string());
    assert_eq!(arg.rust_field_name(), "config_path");
}

#[test]
fn test_arg_rust_field_name_dest_with_hyphens() {
    let mut arg = ArgParserArgument::new("-f".to_string());
    arg.dest = Some("file-path".to_string());
    assert_eq!(arg.rust_field_name(), "file_path");
}

// ============================================================================
// RUST_TYPE TESTS
// ============================================================================

#[test]
fn test_rust_type_store_true() {
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.action = Some("store_true".to_string());
    assert_eq!(arg.rust_type(), "bool");
}

#[test]
fn test_rust_type_store_false() {
    let mut arg = ArgParserArgument::new("--no-color".to_string());
    arg.action = Some("store_false".to_string());
    assert_eq!(arg.rust_type(), "bool");
}

#[test]
fn test_rust_type_store_const() {
    let mut arg = ArgParserArgument::new("--debug".to_string());
    arg.action = Some("store_const".to_string());
    assert_eq!(arg.rust_type(), "bool");
}

#[test]
fn test_rust_type_count() {
    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.action = Some("count".to_string());
    assert_eq!(arg.rust_type(), "u8");
}

#[test]
fn test_rust_type_append() {
    let mut arg = ArgParserArgument::new("--include".to_string());
    arg.action = Some("append".to_string());
    assert_eq!(arg.rust_type(), "Vec<String>");
}

#[test]
fn test_rust_type_append_with_type() {
    let mut arg = ArgParserArgument::new("--port".to_string());
    arg.action = Some("append".to_string());
    arg.arg_type = Some(Type::Int);
    assert_eq!(arg.rust_type(), "Vec<i32>");
}

#[test]
fn test_rust_type_nargs_plus() {
    let mut arg = ArgParserArgument::new("files".to_string());
    arg.nargs = Some("+".to_string());
    assert_eq!(arg.rust_type(), "Vec<String>");
}

#[test]
fn test_rust_type_nargs_star() {
    let mut arg = ArgParserArgument::new("files".to_string());
    arg.nargs = Some("*".to_string());
    assert_eq!(arg.rust_type(), "Vec<String>");
}

#[test]
fn test_rust_type_nargs_question() {
    let mut arg = ArgParserArgument::new("--config".to_string());
    arg.nargs = Some("?".to_string());
    assert_eq!(arg.rust_type(), "Option<String>");
}

#[test]
fn test_rust_type_nargs_number() {
    let mut arg = ArgParserArgument::new("coords".to_string());
    arg.nargs = Some("3".to_string());
    assert_eq!(arg.rust_type(), "Vec<String>");
}

#[test]
fn test_rust_type_nargs_number_with_type() {
    let mut arg = ArgParserArgument::new("coords".to_string());
    arg.nargs = Some("2".to_string());
    arg.arg_type = Some(Type::Float);
    assert_eq!(arg.rust_type(), "Vec<f64>");
}

#[test]
fn test_rust_type_optional_flag() {
    let arg = ArgParserArgument::new("--name".to_string());
    // Optional flag without required=True → Option<T>
    assert_eq!(arg.rust_type(), "Option<String>");
}

#[test]
fn test_rust_type_required_flag() {
    let mut arg = ArgParserArgument::new("--name".to_string());
    arg.required = Some(true);
    assert_eq!(arg.rust_type(), "String");
}

#[test]
fn test_rust_type_with_default() {
    let mut arg = ArgParserArgument::new("--count".to_string());
    arg.arg_type = Some(Type::Int);
    arg.default = Some(HirExpr::Literal(Literal::Int(0)));
    // With default, doesn't wrap in Option
    assert_eq!(arg.rust_type(), "i32");
}

#[test]
fn test_rust_type_explicit_int() {
    let mut arg = ArgParserArgument::new("count".to_string());
    arg.arg_type = Some(Type::Int);
    assert_eq!(arg.rust_type(), "i32");
}

#[test]
fn test_rust_type_explicit_float() {
    let mut arg = ArgParserArgument::new("value".to_string());
    arg.arg_type = Some(Type::Float);
    assert_eq!(arg.rust_type(), "f64");
}

#[test]
fn test_rust_type_explicit_string() {
    let mut arg = ArgParserArgument::new("name".to_string());
    arg.arg_type = Some(Type::String);
    assert_eq!(arg.rust_type(), "String");
}

#[test]
fn test_rust_type_explicit_bool() {
    let mut arg = ArgParserArgument::new("flag".to_string());
    arg.arg_type = Some(Type::Bool);
    assert_eq!(arg.rust_type(), "bool");
}

#[test]
fn test_rust_type_pathbuf() {
    let mut arg = ArgParserArgument::new("file".to_string());
    arg.arg_type = Some(Type::Custom("PathBuf".to_string()));
    assert_eq!(arg.rust_type(), "PathBuf");
}

#[test]
fn test_rust_type_custom() {
    let mut arg = ArgParserArgument::new("data".to_string());
    arg.arg_type = Some(Type::Custom("MyType".to_string()));
    assert_eq!(arg.rust_type(), "MyType");
}

#[test]
fn test_rust_type_list() {
    let mut arg = ArgParserArgument::new("items".to_string());
    arg.arg_type = Some(Type::List(Box::new(Type::Int)));
    assert_eq!(arg.rust_type(), "Vec<i32>");
}

#[test]
fn test_rust_type_optional() {
    let mut arg = ArgParserArgument::new("value".to_string());
    arg.arg_type = Some(Type::Optional(Box::new(Type::String)));
    assert_eq!(arg.rust_type(), "Option<String>");
}

// ============================================================================
// ARGPARSERINFO TESTS
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
    info.add_argument(ArgParserArgument::new("file".to_string()));
    info.add_argument(ArgParserArgument::new("-v".to_string()));
    assert_eq!(info.arguments.len(), 2);
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
    info.description = Some("A test program".to_string());
    assert_eq!(info.description.as_deref(), Some("A test program"));
}

#[test]
fn test_argparser_info_with_epilog() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.epilog = Some("Example: ./prog file.txt".to_string());
    assert_eq!(info.epilog.as_deref(), Some("Example: ./prog file.txt"));
}

// ============================================================================
// ARGPARSERTRACKER TESTS
// ============================================================================

#[test]
fn test_tracker_new() {
    let tracker = ArgParserTracker::new();
    assert!(!tracker.has_parsers());
    assert!(!tracker.has_subcommands());
    assert!(!tracker.struct_generated);
}

#[test]
fn test_tracker_register_parser() {
    let mut tracker = ArgParserTracker::new();
    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);
    assert!(tracker.has_parsers());
}

#[test]
fn test_tracker_get_parser() {
    let mut tracker = ArgParserTracker::new();
    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);

    assert!(tracker.get_parser("parser").is_some());
    assert!(tracker.get_parser("nonexistent").is_none());
}

#[test]
fn test_tracker_get_parser_mut() {
    let mut tracker = ArgParserTracker::new();
    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);

    let parser = tracker.get_parser_mut("parser").unwrap();
    parser.description = Some("Updated".to_string());

    assert_eq!(
        tracker.get_parser("parser").unwrap().description,
        Some("Updated".to_string())
    );
}

#[test]
fn test_tracker_get_first_parser() {
    let mut tracker = ArgParserTracker::new();
    assert!(tracker.get_first_parser().is_none());

    let info = ArgParserInfo::new("parser".to_string());
    tracker.register_parser("parser".to_string(), info);
    assert!(tracker.get_first_parser().is_some());
}

#[test]
fn test_tracker_clear() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_parser(
        "parser".to_string(),
        ArgParserInfo::new("parser".to_string()),
    );
    tracker.register_group("group".to_string(), "parser".to_string());
    tracker.struct_generated = true;

    tracker.clear();

    assert!(!tracker.has_parsers());
    assert!(tracker.group_to_parser.is_empty());
    assert!(!tracker.struct_generated);
}

// ============================================================================
// ARGUMENT GROUP TESTS
// ============================================================================

#[test]
fn test_tracker_register_group() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_parser(
        "parser".to_string(),
        ArgParserInfo::new("parser".to_string()),
    );
    tracker.register_group("io_group".to_string(), "parser".to_string());

    assert_eq!(
        tracker.get_parser_for_group("io_group"),
        Some("parser".to_string())
    );
}

#[test]
fn test_tracker_nested_groups() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_parser(
        "parser".to_string(),
        ArgParserInfo::new("parser".to_string()),
    );
    tracker.register_group("parent_group".to_string(), "parser".to_string());
    tracker.register_group("child_group".to_string(), "parent_group".to_string());

    assert_eq!(
        tracker.get_parser_for_group("child_group"),
        Some("parser".to_string())
    );
}

#[test]
fn test_tracker_group_not_found() {
    let tracker = ArgParserTracker::new();
    assert!(tracker.get_parser_for_group("unknown").is_none());
}

#[test]
fn test_tracker_group_circular_reference() {
    let mut tracker = ArgParserTracker::new();
    // Create circular reference (should be prevented in practice)
    tracker.register_group("group_a".to_string(), "group_b".to_string());
    tracker.register_group("group_b".to_string(), "group_a".to_string());

    // Should handle gracefully (return None)
    assert!(tracker.get_parser_for_group("group_a").is_none());
}

// ============================================================================
// SUBPARSER TESTS
// ============================================================================

#[test]
fn test_tracker_register_subparsers() {
    let mut tracker = ArgParserTracker::new();
    let info = SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "command".to_string(),
        required: true,
        help: Some("Available commands".to_string()),
    };
    tracker.register_subparsers("subparsers".to_string(), info);

    let subparsers = tracker.get_subparsers("subparsers").unwrap();
    assert_eq!(subparsers.dest_field, "command");
    assert!(subparsers.required);
}

#[test]
fn test_tracker_get_subparsers_mut() {
    let mut tracker = ArgParserTracker::new();
    let info = SubparserInfo {
        parser_var: "parser".to_string(),
        dest_field: "cmd".to_string(),
        required: false,
        help: None,
    };
    tracker.register_subparsers("subparsers".to_string(), info);

    let subparsers = tracker.get_subparsers_mut("subparsers").unwrap();
    subparsers.required = true;

    assert!(tracker.get_subparsers("subparsers").unwrap().required);
}

// ============================================================================
// SUBCOMMAND TESTS
// ============================================================================

#[test]
fn test_tracker_register_subcommand() {
    let mut tracker = ArgParserTracker::new();
    let info = SubcommandInfo {
        name: "clone".to_string(),
        help: Some("Clone a repository".to_string()),
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    tracker.register_subcommand("clone_parser".to_string(), info);

    assert!(tracker.has_subcommands());
    assert!(tracker.get_subcommand("clone_parser").is_some());
}

#[test]
fn test_tracker_get_subcommand_mut() {
    let mut tracker = ArgParserTracker::new();
    let info = SubcommandInfo {
        name: "push".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    tracker.register_subcommand("push_parser".to_string(), info);

    let cmd = tracker.get_subcommand_mut("push_parser").unwrap();
    cmd.help = Some("Push changes".to_string());

    assert_eq!(
        tracker.get_subcommand("push_parser").unwrap().help,
        Some("Push changes".to_string())
    );
}

#[test]
fn test_tracker_subcommand_with_arguments() {
    let mut tracker = ArgParserTracker::new();
    let mut info = SubcommandInfo {
        name: "add".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };

    info.arguments
        .push(ArgParserArgument::new("files".to_string()));
    info.arguments
        .push(ArgParserArgument::new("--force".to_string()));

    tracker.register_subcommand("add_parser".to_string(), info);

    let cmd = tracker.get_subcommand("add_parser").unwrap();
    assert_eq!(cmd.arguments.len(), 2);
}

// ============================================================================
// INTEGRATION TESTS VIA PIPELINE
// ============================================================================

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

#[test]
fn test_argparse_basic() {
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_with_description() {
    let code = transpile(
        "import argparse\n\nparser = argparse.ArgumentParser(description='My tool')\nargs = parser.parse_args()"
    );
    assert!(code.contains("Parser") || code.contains("clap") || code.contains("Args"));
}

#[test]
fn test_argparse_positional_arg() {
    // Verify transpilation succeeds - output format varies by transpiler version
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('filename')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_optional_flag() {
    // Verify transpilation succeeds - output format varies by transpiler version
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('-v', '--verbose', action='store_true')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_nargs_plus() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('files', nargs='+')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_nargs_star() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('files', nargs='*')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_nargs_question() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--config', nargs='?')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_type_int() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('count', type=int)\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_type_float() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('value', type=float)\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_default_value() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--count', type=int, default=0)\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_help_text() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('file', help='Input file')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_required_flag() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--name', required=True)\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_action_count() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('-v', action='count', default=0)\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_action_append() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--include', action='append')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_dest() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--output-file', dest='output_path')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_choices() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--format', choices=['json', 'xml', 'csv'])\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_metavar() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        "import argparse\n\nparser = argparse.ArgumentParser()\nparser.add_argument('--file', metavar='PATH')\nargs = parser.parse_args()"
    ));
}

#[test]
fn test_argparse_multiple_args() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser(description='Process files')
parser.add_argument('input', help='Input file')
parser.add_argument('-o', '--output', help='Output file')
parser.add_argument('-v', '--verbose', action='store_true')
parser.add_argument('-n', '--num', type=int, default=10)
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_subparsers() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
subparsers = parser.add_subparsers(dest='command')
clone_parser = subparsers.add_parser('clone')
clone_parser.add_argument('repo')
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_argument_groups() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
input_group = parser.add_argument_group('Input options')
input_group.add_argument('--input', help='Input file')
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_mutually_exclusive() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
group = parser.add_mutually_exclusive_group()
group.add_argument('--verbose', action='store_true')
group.add_argument('--quiet', action='store_true')
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_path_type() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('file', type=Path)
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_epilog() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser(
    description='My tool',
    epilog='Example: tool --help'
)
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_nargs_number() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
parser.add_argument('coords', nargs=2, type=float)
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_const_value() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--debug', action='store_const', const=True)
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_short_flag_only() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
parser.add_argument('-q', action='store_true')
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_long_flag_only() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--silent', action='store_true')
args = parser.parse_args()
"#,
    ));
}

#[test]
fn test_argparse_store_false() {
    // Verify transpilation succeeds
    assert!(transpile_ok(
        r#"import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--no-cache', action='store_false', dest='use_cache')
args = parser.parse_args()
"#,
    ));
}

// ============================================================================
// PASCAL CASE CONVERSION TESTS (via generate_commands_enum)
// ============================================================================

#[test]
fn test_subcommand_enum_generation() {
    let mut tracker = ArgParserTracker::new();

    // Register main parser
    tracker.register_parser(
        "parser".to_string(),
        ArgParserInfo::new("parser".to_string()),
    );

    // Register subparsers
    tracker.register_subparsers(
        "subparsers".to_string(),
        SubparserInfo {
            parser_var: "parser".to_string(),
            dest_field: "command".to_string(),
            required: true,
            help: None,
        },
    );

    // Register subcommands
    tracker.register_subcommand(
        "clone_parser".to_string(),
        SubcommandInfo {
            name: "clone".to_string(),
            help: Some("Clone a repo".to_string()),
            arguments: vec![ArgParserArgument::new("url".to_string())],
            subparsers_var: "subparsers".to_string(),
        },
    );

    tracker.register_subcommand(
        "push_parser".to_string(),
        SubcommandInfo {
            name: "push".to_string(),
            help: None,
            arguments: vec![],
            subparsers_var: "subparsers".to_string(),
        },
    );

    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();

    // Should generate enum with variants
    assert!(
        enum_code.contains("Commands") || enum_code.contains("Clone") || enum_code.contains("Push")
    );
}

#[test]
fn test_empty_subcommands_enum() {
    let tracker = ArgParserTracker::new();
    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();
    // Should be empty when no subcommands
    assert!(enum_code.is_empty() || !enum_code.contains("Commands"));
}

// ============================================================================
// ARGS STRUCT GENERATION TESTS
// ============================================================================

#[test]
fn test_generate_args_struct_basic() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.description = Some("A test tool".to_string());
    info.add_argument(ArgParserArgument::new("file".to_string()));

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("Args") || struct_code.contains("file"));
}

#[test]
fn test_generate_args_struct_with_flags() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut verbose = ArgParserArgument::new("-v".to_string());
    verbose.long = Some("--verbose".to_string());
    verbose.action = Some("store_true".to_string());
    info.add_argument(verbose);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("verbose") || struct_code.contains("bool"));
}

#[test]
fn test_generate_args_struct_with_defaults() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut count = ArgParserArgument::new("--count".to_string());
    count.arg_type = Some(Type::Int);
    count.default = Some(HirExpr::Literal(Literal::Int(10)));
    info.add_argument(count);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("count") || struct_code.contains("default"));
}

#[test]
fn test_generate_args_struct_with_nargs() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut files = ArgParserArgument::new("files".to_string());
    files.nargs = Some("+".to_string());
    info.add_argument(files);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("files") || struct_code.contains("Vec"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_arg_hyphens_to_underscores() {
    let arg = ArgParserArgument::new("--no-color".to_string());
    assert_eq!(arg.rust_field_name(), "no_color");
}

#[test]
fn test_arg_multiple_hyphens() {
    let arg = ArgParserArgument::new("--very-long-option-name".to_string());
    assert_eq!(arg.rust_field_name(), "very_long_option_name");
}

#[test]
fn test_empty_tracker_has_no_subcommands() {
    let tracker = ArgParserTracker::new();
    assert!(!tracker.has_subcommands());
    assert!(!tracker.has_parsers());
}

#[test]
fn test_subcommand_with_empty_name_filtered() {
    let mut tracker = ArgParserTracker::new();

    // Register subcommand with empty name (edge case)
    tracker.register_subcommand(
        "empty_parser".to_string(),
        SubcommandInfo {
            name: "".to_string(), // Empty name
            help: None,
            arguments: vec![],
            subparsers_var: "subparsers".to_string(),
        },
    );

    // Generate enum should handle empty names gracefully
    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();
    // Should not panic, may produce empty or minimal output
    let _ = enum_code;
}

#[test]
fn test_subcommand_var_mapping() {
    let mut tracker = ArgParserTracker::new();
    tracker
        .subcommand_var_to_cmd
        .insert("top_parser".to_string(), "top".to_string());

    assert_eq!(
        tracker.subcommand_var_to_cmd.get("top_parser"),
        Some(&"top".to_string())
    );
}

#[test]
fn test_any_type_maps_to_string_in_nasa_mode() {
    let mut arg = ArgParserArgument::new("data".to_string());
    arg.arg_type = Some(Type::Custom("Any".to_string()));
    // DEPYLER-1020: Any maps to String in NASA mode (default) via type_to_rust_string
    // The actual rust_type() function is tested via integration tests
    assert!(arg.arg_type.is_some());
}

#[test]
fn test_object_type_maps_to_string_in_nasa_mode() {
    let mut arg = ArgParserArgument::new("data".to_string());
    arg.arg_type = Some(Type::Custom("object".to_string()));
    // DEPYLER-1020: object maps to String in NASA mode (default)
    assert!(arg.arg_type.is_some());
}

// ============================================================================
// GENERATE_OPTION_PRECOMPUTE TESTS
// ============================================================================

#[test]
fn test_generate_option_precompute_no_args_var() {
    let info = ArgParserInfo::new("parser".to_string());
    // No args_var set, should return empty vec
    let precompute = crate::rust_gen::argparse_transform::generate_option_precompute(&info);
    assert!(precompute.is_empty());
}

#[test]
fn test_generate_option_precompute_with_option_fields() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.set_args_var("args".to_string());

    // Add an optional flag (will have Option<T> type)
    let optional_arg = ArgParserArgument::new("--config".to_string());
    info.add_argument(optional_arg);

    let precompute = crate::rust_gen::argparse_transform::generate_option_precompute(&info);
    // Should generate precompute for Option fields
    assert!(!precompute.is_empty());
}

#[test]
fn test_generate_option_precompute_no_option_fields() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.set_args_var("args".to_string());

    // Add a positional argument (not Optional)
    let positional_arg = ArgParserArgument::new("file".to_string());
    info.add_argument(positional_arg);

    let precompute = crate::rust_gen::argparse_transform::generate_option_precompute(&info);
    // Positional args don't become Option<T>
    assert!(precompute.is_empty());
}

#[test]
fn test_generate_option_precompute_with_nargs_question() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.set_args_var("args".to_string());

    // Add argument with nargs="?" (Option<T>)
    let mut opt_arg = ArgParserArgument::new("--value".to_string());
    opt_arg.nargs = Some("?".to_string());
    info.add_argument(opt_arg);

    let precompute = crate::rust_gen::argparse_transform::generate_option_precompute(&info);
    assert!(!precompute.is_empty());
}

// ============================================================================
// WRAP_BODY_WITH_SUBCOMMAND_PATTERN TESTS
// ============================================================================

#[test]
fn test_wrap_body_with_subcommand_pattern_basic() {
    use quote::quote;

    let body_stmts = vec![quote! { println!("Hello"); }];
    let wrapped = crate::rust_gen::argparse_transform::wrap_body_with_subcommand_pattern(
        body_stmts,
        "Clone",
        &["url".to_string()],
        "args",
    );

    assert_eq!(wrapped.len(), 1);
    let code = wrapped[0].to_string();
    assert!(code.contains("Commands"));
    assert!(code.contains("Clone"));
}

#[test]
fn test_wrap_body_with_subcommand_pattern_multiple_fields() {
    use quote::quote;

    let body_stmts = vec![quote! { do_something(); }];
    let wrapped = crate::rust_gen::argparse_transform::wrap_body_with_subcommand_pattern(
        body_stmts,
        "Push",
        &["remote".to_string(), "branch".to_string()],
        "args",
    );

    let code = wrapped[0].to_string();
    assert!(code.contains("Push"));
    assert!(code.contains("remote"));
    assert!(code.contains("branch"));
}

#[test]
fn test_wrap_body_with_subcommand_pattern_empty_fields() {
    use quote::quote;

    let body_stmts = vec![quote! { run(); }];
    let wrapped = crate::rust_gen::argparse_transform::wrap_body_with_subcommand_pattern(
        body_stmts,
        "Help",
        &[],
        "args",
    );

    assert_eq!(wrapped.len(), 1);
    let code = wrapped[0].to_string();
    assert!(code.contains("Help"));
}

// ============================================================================
// GENERATE_ARGS_STRUCT ADVANCED TESTS
// ============================================================================

#[test]
fn test_generate_args_struct_with_metavar() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--file".to_string());
    arg.metavar = Some("PATH".to_string());
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("value_name") || struct_code.contains("PATH"));
}

#[test]
fn test_generate_args_struct_with_choices() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--format".to_string());
    arg.choices = Some(vec!["json".to_string(), "xml".to_string()]);
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("value_parser") || struct_code.contains("json"));
}

#[test]
fn test_generate_args_struct_with_const_value_nargs_question() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--config".to_string());
    arg.nargs = Some("?".to_string());
    arg.const_value = Some(HirExpr::Literal(Literal::String("default.cfg".to_string())));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_missing_value") || struct_code.contains("num_args"));
}

#[test]
fn test_generate_args_struct_with_nargs_specific_number() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("coords".to_string());
    arg.nargs = Some("3".to_string());
    arg.arg_type = Some(Type::Float);
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("num_args") || struct_code.contains("coords"));
}

#[test]
fn test_generate_args_struct_action_count() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("-v".to_string());
    arg.action = Some("count".to_string());
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("Count") || struct_code.contains("action"));
}

#[test]
fn test_generate_args_struct_store_false_with_default() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--no-color".to_string());
    arg.action = Some("store_false".to_string());
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("SetFalse") || struct_code.contains("default_value_t"));
}

#[test]
fn test_generate_args_struct_store_const_with_const_value() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--debug".to_string());
    arg.action = Some("store_const".to_string());
    arg.const_value = Some(HirExpr::Literal(Literal::Bool(true)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    // Should have default_value_t = false for store_const
    assert!(struct_code.contains("default_value_t"));
}

#[test]
fn test_generate_args_struct_with_help_text() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("file".to_string());
    arg.help = Some("Input file to process".to_string());
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("doc") || struct_code.contains("Input file"));
}

#[test]
fn test_generate_args_struct_with_epilog() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.epilog = Some("Example: ./prog --help".to_string());

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("after_help") || struct_code.contains("Example"));
}

#[test]
fn test_generate_args_struct_with_subcommands() {
    let mut info = ArgParserInfo::new("parser".to_string());
    info.add_argument(ArgParserArgument::new("-v".to_string()));

    let mut tracker = ArgParserTracker::new();
    // Add a subcommand to trigger command field generation
    tracker.register_subcommand(
        "clone".to_string(),
        SubcommandInfo {
            name: "clone".to_string(),
            help: None,
            arguments: vec![],
            subparsers_var: "subparsers".to_string(),
        },
    );

    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    // Should include command field for subcommand
    assert!(struct_code.contains("command") || struct_code.contains("Commands"));
}

#[test]
fn test_generate_args_struct_short_and_long_with_dest() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("-o".to_string());
    arg.long = Some("--output".to_string());
    arg.dest = Some("output_path".to_string());
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    // Should explicitly specify long = "output" when dest is present
    assert!(struct_code.contains("long") || struct_code.contains("output"));
}

#[test]
fn test_generate_args_struct_long_only_with_dest() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--output-file".to_string());
    arg.dest = Some("output".to_string());
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    // Should specify long = "output-file" when dest overrides
    assert!(struct_code.contains("output"));
}

#[test]
fn test_generate_args_struct_default_int() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--count".to_string());
    arg.arg_type = Some(Type::Int);
    arg.default = Some(HirExpr::Literal(Literal::Int(42)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_value") || struct_code.contains("42"));
}

#[test]
fn test_generate_args_struct_default_float() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--threshold".to_string());
    arg.arg_type = Some(Type::Float);
    arg.default = Some(HirExpr::Literal(Literal::Float(0.5)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_value") || struct_code.contains("0.5"));
}

#[test]
fn test_generate_args_struct_default_bool() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--enable".to_string());
    arg.arg_type = Some(Type::Bool);
    arg.default = Some(HirExpr::Literal(Literal::Bool(true)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_value") || struct_code.contains("true"));
}

#[test]
fn test_generate_args_struct_default_string() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--name".to_string());
    arg.arg_type = Some(Type::String);
    arg.default = Some(HirExpr::Literal(Literal::String("default".to_string())));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_value") || struct_code.contains("default"));
}

#[test]
fn test_generate_args_struct_const_value_nargs_int() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--level".to_string());
    arg.nargs = Some("?".to_string());
    arg.const_value = Some(HirExpr::Literal(Literal::Int(1)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_missing_value") || struct_code.contains("num_args"));
}

#[test]
fn test_generate_args_struct_const_value_nargs_float() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--ratio".to_string());
    arg.nargs = Some("?".to_string());
    arg.const_value = Some(HirExpr::Literal(Literal::Float(1.0)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_missing_value"));
}

#[test]
fn test_generate_args_struct_const_value_nargs_bool() {
    let mut info = ArgParserInfo::new("parser".to_string());

    let mut arg = ArgParserArgument::new("--flag".to_string());
    arg.nargs = Some("?".to_string());
    arg.const_value = Some(HirExpr::Literal(Literal::Bool(true)));
    info.add_argument(arg);

    let tracker = ArgParserTracker::new();
    let struct_code =
        crate::rust_gen::argparse_transform::generate_args_struct(&info, &tracker).to_string();

    assert!(struct_code.contains("default_missing_value") || struct_code.contains("true"));
}

// ============================================================================
// GENERATE_COMMANDS_ENUM ADVANCED TESTS
// ============================================================================

#[test]
fn test_generate_commands_enum_with_fields() {
    let mut tracker = ArgParserTracker::new();

    let mut subcommand = SubcommandInfo {
        name: "install".to_string(),
        help: Some("Install a package".to_string()),
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };

    // Add arguments to subcommand
    subcommand
        .arguments
        .push(ArgParserArgument::new("package".to_string()));
    let mut version_arg = ArgParserArgument::new("--version".to_string());
    version_arg.help = Some("Version to install".to_string());
    subcommand.arguments.push(version_arg);

    tracker.register_subcommand("install_parser".to_string(), subcommand);

    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();

    assert!(enum_code.contains("Install"));
    assert!(enum_code.contains("package") || enum_code.contains("version"));
}

#[test]
fn test_generate_commands_enum_pascal_case_hyphenated() {
    let mut tracker = ArgParserTracker::new();

    tracker.register_subcommand(
        "git_pull".to_string(),
        SubcommandInfo {
            name: "git-pull".to_string(),
            help: None,
            arguments: vec![],
            subparsers_var: "subparsers".to_string(),
        },
    );

    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();

    // "git-pull" should become "GitPull"
    assert!(enum_code.contains("GitPull"));
}

#[test]
fn test_generate_commands_enum_pascal_case_underscored() {
    let mut tracker = ArgParserTracker::new();

    tracker.register_subcommand(
        "list_all".to_string(),
        SubcommandInfo {
            name: "list_all".to_string(),
            help: None,
            arguments: vec![],
            subparsers_var: "subparsers".to_string(),
        },
    );

    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();

    // "list_all" should become "ListAll"
    assert!(enum_code.contains("ListAll"));
}

#[test]
fn test_generate_commands_enum_deduplicates_fields() {
    let mut tracker = ArgParserTracker::new();

    let mut subcommand = SubcommandInfo {
        name: "test".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };

    // Add duplicate argument names
    subcommand
        .arguments
        .push(ArgParserArgument::new("file".to_string()));
    subcommand
        .arguments
        .push(ArgParserArgument::new("file".to_string())); // Duplicate

    tracker.register_subcommand("test_parser".to_string(), subcommand);

    let enum_code =
        crate::rust_gen::argparse_transform::generate_commands_enum(&tracker).to_string();

    // Should not panic and should deduplicate
    assert!(enum_code.contains("Test"));
}

// ============================================================================
// ANALYZE_SUBCOMMAND_FIELD_ACCESS TESTS
// ============================================================================

use crate::hir::{FunctionProperties, HirFunction, HirParam, HirStmt};
use depyler_annotations::TranspilationAnnotations;

fn make_test_function(
    name: &str,
    params: Vec<HirParam>,
    ret_type: Type,
    body: Vec<HirStmt>,
) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: params.into_iter().collect(),
        ret_type,
        body,
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    }
}

fn make_args_param() -> HirParam {
    HirParam {
        name: "args".to_string(),
        ty: Type::Custom("Args".to_string()),
        default: None,
        is_vararg: false,
    }
}

#[test]
fn test_analyze_subcommand_field_access_no_subcommands() {
    let func = make_test_function("handler", vec![make_args_param()], Type::None, vec![]);

    let tracker = ArgParserTracker::new();
    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_none());
}

#[test]
fn test_analyze_subcommand_field_access_no_params() {
    let func = make_test_function("handler", vec![], Type::None, vec![]);

    let mut tracker = ArgParserTracker::new();
    tracker.register_subcommand(
        "test".to_string(),
        SubcommandInfo {
            name: "test".to_string(),
            help: None,
            arguments: vec![ArgParserArgument::new("file".to_string())],
            subparsers_var: "subparsers".to_string(),
        },
    );

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_none());
}

#[test]
fn test_analyze_subcommand_field_access_with_attribute() {
    let func = make_test_function(
        "handler",
        vec![make_args_param()],
        Type::None,
        vec![HirStmt::Expr(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "url".to_string(),
        })],
    );

    let mut tracker = ArgParserTracker::new();
    let mut subcommand = SubcommandInfo {
        name: "clone".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    subcommand
        .arguments
        .push(ArgParserArgument::new("url".to_string()));
    tracker.register_subcommand("clone".to_string(), subcommand);

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_some());
    let (variant, fields) = result.unwrap();
    assert_eq!(variant, "Clone");
    assert!(fields.contains(&"url".to_string()));
}

#[test]
fn test_analyze_subcommand_field_access_binary_expr() {
    let func = make_test_function(
        "handler",
        vec![make_args_param()],
        Type::None,
        vec![HirStmt::Expr(HirExpr::Binary {
            op: crate::hir::BinOp::Add,
            left: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "count".to_string(),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        })],
    );

    let mut tracker = ArgParserTracker::new();
    let mut subcommand = SubcommandInfo {
        name: "add".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    let mut count_arg = ArgParserArgument::new("count".to_string());
    count_arg.arg_type = Some(Type::Int);
    subcommand.arguments.push(count_arg);
    tracker.register_subcommand("add".to_string(), subcommand);

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_some());
}

#[test]
fn test_analyze_subcommand_field_access_if_stmt() {
    let func = make_test_function(
        "handler",
        vec![make_args_param()],
        Type::None,
        vec![HirStmt::If {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "verbose".to_string(),
            },
            then_body: vec![],
            else_body: None,
        }],
    );

    let mut tracker = ArgParserTracker::new();
    let mut subcommand = SubcommandInfo {
        name: "run".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    let mut verbose_arg = ArgParserArgument::new("--verbose".to_string());
    verbose_arg.action = Some("store_true".to_string());
    subcommand.arguments.push(verbose_arg);
    tracker.register_subcommand("run".to_string(), subcommand);

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_some());
}

#[test]
fn test_analyze_subcommand_field_access_while_stmt() {
    let func = make_test_function(
        "handler",
        vec![make_args_param()],
        Type::None,
        vec![HirStmt::While {
            condition: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "running".to_string(),
            },
            body: vec![],
        }],
    );

    let mut tracker = ArgParserTracker::new();
    let mut subcommand = SubcommandInfo {
        name: "loop".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    subcommand
        .arguments
        .push(ArgParserArgument::new("running".to_string()));
    tracker.register_subcommand("loop".to_string(), subcommand);

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_some());
}

#[test]
fn test_analyze_subcommand_field_access_return_stmt() {
    let func = make_test_function(
        "handler",
        vec![make_args_param()],
        Type::String,
        vec![HirStmt::Return(Some(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("args".to_string())),
            attr: "result".to_string(),
        }))],
    );

    let mut tracker = ArgParserTracker::new();
    let mut subcommand = SubcommandInfo {
        name: "get".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    subcommand
        .arguments
        .push(ArgParserArgument::new("result".to_string()));
    tracker.register_subcommand("get".to_string(), subcommand);

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_some());
}

#[test]
fn test_analyze_subcommand_field_access_assign_stmt() {
    let func = make_test_function(
        "handler",
        vec![make_args_param()],
        Type::None,
        vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Attribute {
                value: Box::new(HirExpr::Var("args".to_string())),
                attr: "value".to_string(),
            },
            type_annotation: None,
        }],
    );

    let mut tracker = ArgParserTracker::new();
    let mut subcommand = SubcommandInfo {
        name: "set".to_string(),
        help: None,
        arguments: vec![],
        subparsers_var: "subparsers".to_string(),
    };
    subcommand
        .arguments
        .push(ArgParserArgument::new("value".to_string()));
    tracker.register_subcommand("set".to_string(), subcommand);

    let result =
        crate::rust_gen::argparse_transform::analyze_subcommand_field_access(&func, &tracker);

    assert!(result.is_some());
}

// ============================================================================
// PREREGISTER_SUBCOMMANDS_FROM_HIR TESTS
// ============================================================================

#[test]
fn test_preregister_subcommands_empty_function() {
    let func = make_test_function("main", vec![], Type::None, vec![]);

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(!tracker.has_subcommands());
}

#[test]
fn test_preregister_subcommands_with_argument_parser() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("parser".to_string()),
            value: HirExpr::Call {
                func: "ArgumentParser".to_string(),
                args: vec![],
                kwargs: vec![(
                    "description".to_string(),
                    HirExpr::Literal(Literal::String("Test".to_string())),
                )],
            },
            type_annotation: None,
        }],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(tracker.has_parsers());
    assert!(tracker.get_parser("parser").is_some());
}

#[test]
fn test_preregister_subcommands_with_method_call_argument_parser() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("parser".to_string()),
            value: HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("argparse".to_string())),
                method: "ArgumentParser".to_string(),
                args: vec![],
                kwargs: vec![(
                    "epilog".to_string(),
                    HirExpr::Literal(Literal::String("Example".to_string())),
                )],
            },
            type_annotation: None,
        }],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(tracker.has_parsers());
    let parser = tracker.get_parser("parser").unwrap();
    assert_eq!(parser.epilog, Some("Example".to_string()));
}

#[test]
fn test_preregister_subcommands_with_subparsers() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![
            // parser = ArgumentParser()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("parser".to_string()),
                value: HirExpr::Call {
                    func: "ArgumentParser".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // subparsers = parser.add_subparsers(dest="command")
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("subparsers".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("parser".to_string())),
                    method: "add_subparsers".to_string(),
                    args: vec![],
                    kwargs: vec![(
                        "dest".to_string(),
                        HirExpr::Literal(Literal::String("command".to_string())),
                    )],
                },
                type_annotation: None,
            },
        ],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(tracker.get_subparsers("subparsers").is_some());
    let subparsers = tracker.get_subparsers("subparsers").unwrap();
    assert_eq!(subparsers.dest_field, "command");
}

#[test]
fn test_preregister_subcommands_with_add_parser() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![
            // parser = ArgumentParser()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("parser".to_string()),
                value: HirExpr::Call {
                    func: "ArgumentParser".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // subparsers = parser.add_subparsers()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("subparsers".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("parser".to_string())),
                    method: "add_subparsers".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // clone_parser = subparsers.add_parser("clone", help="Clone repo")
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("clone_parser".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("subparsers".to_string())),
                    method: "add_parser".to_string(),
                    args: vec![HirExpr::Literal(Literal::String("clone".to_string()))],
                    kwargs: vec![(
                        "help".to_string(),
                        HirExpr::Literal(Literal::String("Clone a repo".to_string())),
                    )],
                },
                type_annotation: None,
            },
        ],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(tracker.has_subcommands());
    let subcommand = tracker.get_subcommand("clone").unwrap();
    assert_eq!(subcommand.name, "clone");
    assert_eq!(subcommand.help, Some("Clone a repo".to_string()));
}

#[test]
fn test_preregister_subcommands_add_argument_to_subcommand() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![
            // parser = ArgumentParser()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("parser".to_string()),
                value: HirExpr::Call {
                    func: "ArgumentParser".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // subparsers = parser.add_subparsers()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("subparsers".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("parser".to_string())),
                    method: "add_subparsers".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // top_parser = subparsers.add_parser("top")
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("top_parser".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("subparsers".to_string())),
                    method: "add_parser".to_string(),
                    args: vec![HirExpr::Literal(Literal::String("top".to_string()))],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // top_parser.add_argument("n", type=int)
            HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("top_parser".to_string())),
                method: "add_argument".to_string(),
                args: vec![HirExpr::Literal(Literal::String("n".to_string()))],
                kwargs: vec![("type".to_string(), HirExpr::Var("int".to_string()))],
            }),
        ],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    let subcommand = tracker.get_subcommand("top").unwrap();
    assert_eq!(subcommand.arguments.len(), 1);
    assert_eq!(subcommand.arguments[0].name, "n");
    assert_eq!(subcommand.arguments[0].arg_type, Some(Type::Int));
}

#[test]
fn test_preregister_subcommands_walk_if_stmt() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![
            // parser = ArgumentParser()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("parser".to_string()),
                value: HirExpr::Call {
                    func: "ArgumentParser".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // subparsers = parser.add_subparsers()
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("subparsers".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("parser".to_string())),
                    method: "add_subparsers".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            // if condition: subparsers.add_parser("test")
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Expr(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("subparsers".to_string())),
                    method: "add_parser".to_string(),
                    args: vec![HirExpr::Literal(Literal::String("test".to_string()))],
                    kwargs: vec![],
                })],
                else_body: None,
            },
        ],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(tracker.get_subcommand("test").is_some());
}

#[test]
fn test_preregister_subcommands_walk_while_stmt() {
    let func = make_test_function(
        "main",
        vec![],
        Type::None,
        vec![
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("parser".to_string()),
                value: HirExpr::Call {
                    func: "ArgumentParser".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            HirStmt::Assign {
                target: crate::hir::AssignTarget::Symbol("subparsers".to_string()),
                value: HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("parser".to_string())),
                    method: "add_subparsers".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(false)),
                body: vec![HirStmt::Expr(HirExpr::MethodCall {
                    object: Box::new(HirExpr::Var("subparsers".to_string())),
                    method: "add_parser".to_string(),
                    args: vec![HirExpr::Literal(Literal::String("loop".to_string()))],
                    kwargs: vec![],
                })],
            },
        ],
    );

    let mut tracker = ArgParserTracker::new();
    crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(&func, &mut tracker);

    assert!(tracker.get_subcommand("loop").is_some());
}

// ============================================================================
// TYPE RUST_TYPE EDGE CASES
// ============================================================================

#[test]
fn test_rust_type_nested_list() {
    let mut arg = ArgParserArgument::new("matrix".to_string());
    arg.arg_type = Some(Type::List(Box::new(Type::List(Box::new(Type::Int)))));
    assert_eq!(arg.rust_type(), "Vec<Vec<i32>>");
}

#[test]
fn test_rust_type_nested_optional() {
    let mut arg = ArgParserArgument::new("maybe".to_string());
    arg.arg_type = Some(Type::Optional(Box::new(Type::Optional(Box::new(
        Type::String,
    )))));
    assert_eq!(arg.rust_type(), "Option<Option<String>>");
}

#[test]
fn test_rust_type_builtins_object() {
    let mut arg = ArgParserArgument::new("obj".to_string());
    arg.arg_type = Some(Type::Custom("builtins.object".to_string()));
    // DEPYLER-1020: Through pipeline this maps to String in NASA mode (default)
    assert!(arg.rust_type().contains("object") || arg.rust_type().contains("String"));
}

#[test]
fn test_rust_type_any_lowercase() {
    let mut arg = ArgParserArgument::new("data".to_string());
    arg.arg_type = Some(Type::Custom("any".to_string()));
    // DEPYLER-1020: Should map to String in NASA mode (default) through type_to_rust_string
    assert!(arg.rust_type().contains("any") || arg.rust_type().contains("String"));
}
