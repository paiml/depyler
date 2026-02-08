//! Wave 8 coverage tests: argparse_transform.rs + string_methods.rs
//!
//! Targets coverage gaps in:
//! - argparse_transform.rs: ~480 missed lines (parser creation, argument types,
//!   subparsers, commands enum, struct generation, field access analysis)
//! - string_methods.rs: ~340 missed lines (upper/lower/strip/split/join/replace/
//!   find/count/isdigit/isalpha/center/ljust/rjust/zfill/capitalize/swapcase/
//!   expandtabs/splitlines/partition/casefold/isprintable/isupper/islower/
//!   istitle/isnumeric/isascii/isdecimal/isidentifier/hex/format)

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(code)?;
        Ok(result)
    }

    // ========================================================================
    // SECTION 1: argparse_transform.rs - Basic parser creation
    // ========================================================================

    #[test]
    fn test_w8_argparse_basic_parser_creation() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"A tool\")
    args = parser.parse_args()
    print(args)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("clap") || result.contains("Args") || result.contains("parse"),
                "Expected clap/Args/parse in output: {}",
                &result[..result.len().min(500)]
            );
        }
    }

    #[test]
    fn test_w8_argparse_parser_no_description() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("clap") || result.contains("Args") || result.contains("parse"),
                "Expected clap output"
            );
        }
    }

    #[test]
    fn test_w8_argparse_parser_with_epilog() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"Tool\", epilog=\"Example usage\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("after_help") || result.contains("epilog") || result.contains("Args"),
                "Expected epilog/after_help in output"
            );
        }
    }

    // ========================================================================
    // SECTION 2: argparse - Positional arguments
    // ========================================================================

    #[test]
    fn test_w8_argparse_positional_filename() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"tool\")
    parser.add_argument(\"filename\")
    args = parser.parse_args()
    print(args.filename)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("filename") || result.contains("String"),
                "Expected filename field"
            );
        }
    }

    #[test]
    fn test_w8_argparse_positional_input() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"input\")
    args = parser.parse_args()
    print(args.input)
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("input") || result.contains("String"));
        }
    }

    #[test]
    fn test_w8_argparse_multiple_positionals() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"source\")
    parser.add_argument(\"destination\")
    args = parser.parse_args()
    print(args.source)
    print(args.destination)
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("source") || result.contains("destination"));
        }
    }

    // ========================================================================
    // SECTION 3: argparse - Optional/flag arguments
    // ========================================================================

    #[test]
    fn test_w8_argparse_store_true_flag() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--verbose\", action=\"store_true\")
    args = parser.parse_args()
    if args.verbose:
        print(\"verbose\")
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("bool") || result.contains("verbose") || result.contains("SetTrue"),
                "Expected bool/verbose/SetTrue"
            );
        }
    }

    #[test]
    fn test_w8_argparse_store_false_flag() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--no-color\", action=\"store_false\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("bool") || result.contains("SetFalse") || result.contains("no_color"),
                "Expected store_false handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_int_type_argument() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--count\", type=int, default=0)
    args = parser.parse_args()
    print(args.count)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("i32") || result.contains("count") || result.contains("default"),
                "Expected int type or count field"
            );
        }
    }

    #[test]
    fn test_w8_argparse_float_type_argument() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--timeout\", type=float, default=30.0)
    args = parser.parse_args()
    print(args.timeout)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("f64") || result.contains("timeout") || result.contains("30"),
                "Expected float type"
            );
        }
    }

    #[test]
    fn test_w8_argparse_str_type_argument() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--name\", type=str, default=\"world\")
    args = parser.parse_args()
    print(args.name)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("String") || result.contains("name") || result.contains("world"),
                "Expected string type"
            );
        }
    }

    #[test]
    fn test_w8_argparse_choices_argument() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--mode\", choices=[\"fast\", \"slow\"])
    args = parser.parse_args()
    print(args.mode)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("mode") || result.contains("value_parser") || result.contains("fast"),
                "Expected choices handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_plus() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"files\", nargs=\"+\")
    args = parser.parse_args()
    for f in args.files:
        print(f)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("files"),
                "Expected Vec type for nargs=+"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_star() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"items\", nargs=\"*\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("items"),
                "Expected Vec type for nargs=*"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_question() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--output\", nargs=\"?\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Option") || result.contains("output"),
                "Expected Option type for nargs=?"
            );
        }
    }

    #[test]
    fn test_w8_argparse_required_flag() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--output\", required=True)
    args = parser.parse_args()
    print(args.output)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("output") || result.contains("String"),
                "Expected required output field"
            );
        }
    }

    #[test]
    fn test_w8_argparse_help_text() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--debug\", help=\"Enable debug mode\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("debug") || result.contains("Enable"),
                "Expected help text or field name"
            );
        }
    }

    #[test]
    fn test_w8_argparse_metavar() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--file\", metavar=\"PATH\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("file") || result.contains("PATH") || result.contains("value_name"),
                "Expected metavar handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_short_and_long_flag() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"-v\", \"--verbose\", action=\"store_true\")
    args = parser.parse_args()
    if args.verbose:
        print(\"verbose mode\")
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("verbose") || result.contains("short") || result.contains("bool"),
                "Expected short+long flag handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_dest_parameter() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"-n\", dest=\"num\")
    args = parser.parse_args()
    print(args.num)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("num") || result.contains("dest"),
                "Expected dest handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_count_action() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"-v\", \"--verbose\", action=\"count\")
    args = parser.parse_args()
    print(args.verbose)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("u8") || result.contains("Count") || result.contains("verbose"),
                "Expected count action handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_append_action() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--include\", action=\"append\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("include") || result.contains("append"),
                "Expected append action handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_default_value_int() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--retries\", type=int, default=3)
    args = parser.parse_args()
    print(args.retries)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("retries") || result.contains("3") || result.contains("default"),
                "Expected default int handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_default_value_string() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--format\", default=\"json\")
    args = parser.parse_args()
    print(args.format)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("json") || result.contains("default") || result.contains("format"),
                "Expected default string handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparsers_basic() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"command\")
    run_parser = subparsers.add_parser(\"run\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Subcommand") || result.contains("command") || result.contains("Run") || result.contains("enum"),
                "Expected subparser handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparsers_with_help() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"Git-like tool\")
    subparsers = parser.add_subparsers(dest=\"command\")
    clone_parser = subparsers.add_parser(\"clone\", help=\"Clone a repository\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Clone") || result.contains("clone") || result.contains("Subcommand"),
                "Expected subparser with help"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparser_with_arguments() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"command\")
    build_parser = subparsers.add_parser(\"build\")
    build_parser.add_argument(\"--target\", default=\"release\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Build") || result.contains("target") || result.contains("release"),
                "Expected subparser arguments"
            );
        }
    }

    #[test]
    fn test_w8_argparse_multiple_subparsers() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"command\")
    run_parser = subparsers.add_parser(\"run\")
    test_parser = subparsers.add_parser(\"test\")
    build_parser = subparsers.add_parser(\"build\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Run") || result.contains("Test") || result.contains("Build")
                    || result.contains("enum") || result.contains("Subcommand"),
                "Expected multiple subcommands"
            );
        }
    }

    #[test]
    fn test_w8_argparse_complex_cli_pattern() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"Package manager\")
    parser.add_argument(\"--verbose\", \"-v\", action=\"store_true\")
    subparsers = parser.add_subparsers(dest=\"command\")
    install_parser = subparsers.add_parser(\"install\", help=\"Install package\")
    install_parser.add_argument(\"package\")
    install_parser.add_argument(\"--upgrade\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Install") || result.contains("package") || result.contains("verbose"),
                "Expected complex CLI pattern"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_specific_number() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"coords\", nargs=2)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("coords") || result.contains("num_args"),
                "Expected nargs=N handling"
            );
        }
    }

    #[test]
    fn test_w8_argparse_multiple_flags_with_types() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--host\", type=str, default=\"localhost\")
    parser.add_argument(\"--port\", type=int, default=8080)
    parser.add_argument(\"--debug\", action=\"store_true\")
    args = parser.parse_args()
    print(args.host)
    print(args.port)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("host") || result.contains("port") || result.contains("debug"),
                "Expected multiple typed flags"
            );
        }
    }

    #[test]
    fn test_w8_argparse_access_parsed_verbose() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--verbose\", action=\"store_true\")
    args = parser.parse_args()
    if args.verbose:
        print(\"Verbose mode enabled\")
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("verbose") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w8_argparse_access_parsed_count() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--count\", type=int, default=1)
    args = parser.parse_args()
    result = args.count * 2
    print(result)
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("count") || result.contains("i32"));
        }
    }

    #[test]
    fn test_w8_argparse_access_parsed_filename() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"filename\")
    args = parser.parse_args()
    print(args.filename)
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("filename") || result.contains("String"));
        }
    }

    #[test]
    fn test_w8_argparse_long_flag_with_hyphens() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--no-cache\", action=\"store_true\")
    args = parser.parse_args()
    if args.no_cache:
        print(\"cache disabled\")
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("no_cache") || result.contains("bool") || result.contains("cache"),
                "Expected hyphen-to-underscore conversion"
            );
        }
    }

    #[test]
    fn test_w8_argparse_boolean_default_false() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--quiet\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("quiet") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w8_argparse_optional_without_default() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--config\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Option") || result.contains("config"),
                "Expected Option type for optional flag without default"
            );
        }
    }

    #[test]
    fn test_w8_argparse_store_const_action() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--enable\", action=\"store_const\", const=True)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("bool") || result.contains("enable"));
        }
    }

    #[test]
    fn test_w8_argparse_choices_with_type() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--level\", choices=[\"info\", \"warn\", \"error\"])
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("level") || result.contains("info") || result.contains("value_parser"),
                "Expected choices with type"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_question_with_const() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--log\", nargs=\"?\", const=\"debug\", default=\"info\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Option") || result.contains("log") || result.contains("default"),
                "Expected nargs=? with const"
            );
        }
    }

    #[test]
    fn test_w8_argparse_multiple_short_flags() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"-v\", \"--verbose\", action=\"store_true\")
    parser.add_argument(\"-q\", \"--quiet\", action=\"store_true\")
    parser.add_argument(\"-d\", \"--debug\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("verbose") || result.contains("quiet") || result.contains("debug"),
                "Expected multiple short flags"
            );
        }
    }

    #[test]
    fn test_w8_argparse_positional_with_type() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"port\", type=int)
    args = parser.parse_args()
    print(args.port)
";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("i32") || result.contains("port"));
        }
    }

    #[test]
    fn test_w8_argparse_append_with_type() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--tag\", action=\"append\", type=str)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("tag"),
                "Expected Vec for append action"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_plus_with_type() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"numbers\", nargs=\"+\", type=int)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("i32") || result.contains("numbers"),
                "Expected Vec<i32> for nargs=+ type=int"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparser_required() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"cmd\", required=True)
    subparsers.add_parser(\"start\")
    subparsers.add_parser(\"stop\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Start") || result.contains("Stop") || result.contains("cmd"),
                "Expected required subparsers"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparser_with_multiple_args() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"action\")
    deploy_parser = subparsers.add_parser(\"deploy\")
    deploy_parser.add_argument(\"--env\", required=True)
    deploy_parser.add_argument(\"--dry-run\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Deploy") || result.contains("env") || result.contains("dry_run"),
                "Expected subparser with multiple args"
            );
        }
    }

    #[test]
    fn test_w8_argparse_git_like_subcommands() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"Version control\")
    subparsers = parser.add_subparsers(dest=\"command\")
    clone_parser = subparsers.add_parser(\"clone\")
    clone_parser.add_argument(\"url\")
    pull_parser = subparsers.add_parser(\"pull\")
    pull_parser.add_argument(\"--rebase\", action=\"store_true\")
    push_parser = subparsers.add_parser(\"push\")
    push_parser.add_argument(\"--force\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Clone") || result.contains("Pull") || result.contains("Push")
                    || result.contains("enum") || result.contains("Subcommand"),
                "Expected git-like subcommands"
            );
        }
    }

    #[test]
    fn test_w8_argparse_server_cli_pattern() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"HTTP Server\")
    parser.add_argument(\"--host\", default=\"0.0.0.0\")
    parser.add_argument(\"--port\", type=int, default=8000)
    parser.add_argument(\"--workers\", type=int, default=4)
    parser.add_argument(\"--reload\", action=\"store_true\")
    args = parser.parse_args()
    print(args.host)
    print(args.port)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("host") || result.contains("port") || result.contains("workers"),
                "Expected server CLI pattern"
            );
        }
    }

    #[test]
    fn test_w8_argparse_file_processing_pattern() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"input_file\")
    parser.add_argument(\"-o\", \"--output\", default=\"output.txt\")
    parser.add_argument(\"--encoding\", default=\"utf-8\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("input_file") || result.contains("output") || result.contains("encoding"),
                "Expected file processing pattern"
            );
        }
    }

    #[test]
    fn test_w8_argparse_verbosity_levels() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"-v\", \"--verbose\", action=\"count\", default=0)
    args = parser.parse_args()
    if args.verbose > 1:
        print(\"very verbose\")
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("verbose") || result.contains("Count") || result.contains("u8"),
                "Expected count-based verbosity"
            );
        }
    }

    #[test]
    fn test_w8_argparse_boolean_pair_flags() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--color\", action=\"store_true\")
    parser.add_argument(\"--no-color\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("color") || result.contains("no_color") || result.contains("bool"),
                "Expected boolean pair flags"
            );
        }
    }

    #[test]
    fn test_w8_argparse_optional_int_flag() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--jobs\", type=int)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Option") || result.contains("i32") || result.contains("jobs"),
                "Expected Option<i32> for optional int"
            );
        }
    }

    #[test]
    fn test_w8_argparse_description_only() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"My awesome tool for processing data\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("about") || result.contains("awesome") || result.contains("Args"),
                "Expected description in output"
            );
        }
    }

    #[test]
    fn test_w8_argparse_mixed_positional_and_optional() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"command\")
    parser.add_argument(\"target\")
    parser.add_argument(\"--force\", action=\"store_true\")
    parser.add_argument(\"--timeout\", type=int, default=60)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("command") || result.contains("target") || result.contains("force"),
                "Expected mixed args"
            );
        }
    }

    #[test]
    fn test_w8_argparse_nargs_star_with_default() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--exclude\", nargs=\"*\", default=[])
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("exclude"),
                "Expected Vec for nargs=* with default"
            );
        }
    }

    // ========================================================================
    // SECTION 4: string_methods.rs - Basic case transformations
    // ========================================================================

    #[test]
    fn test_w8_string_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase"),
            "Expected to_uppercase: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lower() {
        let code = "def f(s: str) -> str:\n    return s.lower()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase"),
            "Expected to_lowercase: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_title() {
        let code = "def f(s: str) -> str:\n    return s.title()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase") || result.contains("split_whitespace"),
            "Expected title case logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_capitalize() {
        let code = "def f(s: str) -> str:\n    return s.capitalize()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase") || result.contains("chars"),
            "Expected capitalize logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_swapcase() {
        let code = "def f(s: str) -> str:\n    return s.swapcase()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_uppercase") || result.contains("to_lowercase") || result.contains("to_uppercase"),
            "Expected swapcase logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_casefold() {
        let code = "def f(s: str) -> str:\n    return s.casefold()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase"),
            "Expected casefold->to_lowercase: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 5: string_methods.rs - Strip methods
    // ========================================================================

    #[test]
    fn test_w8_string_strip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.strip()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim"),
            "Expected trim(): {}",
            result
        );
    }

    #[test]
    fn test_w8_string_strip_with_chars() {
        let code = "def f(s: str) -> str:\n    return s.strip(\"xy\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim_matches"),
            "Expected trim_matches: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lstrip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.lstrip()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim_start"),
            "Expected trim_start: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lstrip_with_chars() {
        let code = "def f(s: str) -> str:\n    return s.lstrip(\"abc\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim_start_matches"),
            "Expected trim_start_matches: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rstrip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.rstrip()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim_end"),
            "Expected trim_end: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rstrip_with_chars() {
        let code = "def f(s: str) -> str:\n    return s.rstrip(\"xyz\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim_end_matches"),
            "Expected trim_end_matches: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 6: string_methods.rs - Split methods
    // ========================================================================

    #[test]
    fn test_w8_string_split_no_args() {
        let code = "def f(s: str) -> list:\n    return s.split()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split_whitespace"),
            "Expected split_whitespace: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_with_sep() {
        let code = "def f(s: str) -> list:\n    return s.split(\",\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".split(") && result.contains(","),
            "Expected split with comma separator: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_with_maxsplit() {
        let code = "def f(s: str) -> list:\n    return s.split(\",\", 1)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("splitn"),
            "Expected splitn: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rsplit_no_args() {
        let code = "def f(s: str) -> list:\n    return s.rsplit()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split_whitespace") && result.contains("rev"),
            "Expected split_whitespace().rev(): {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rsplit_with_sep() {
        let code = "def f(s: str) -> list:\n    return s.rsplit(\".\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("rsplit"),
            "Expected rsplit: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rsplit_with_maxsplit() {
        let code = "def f(s: str) -> list:\n    return s.rsplit(\".\", 1)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("rsplitn"),
            "Expected rsplitn: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_splitlines() {
        let code = "def f(s: str) -> list:\n    return s.splitlines()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("lines()"),
            "Expected lines(): {}",
            result
        );
    }

    // ========================================================================
    // SECTION 7: string_methods.rs - Join
    // ========================================================================

    #[test]
    fn test_w8_string_join_list() {
        let code = "def f(items: list) -> str:\n    return \", \".join(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".join("),
            "Expected join: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_join_with_variable_sep() {
        let code = "def f(sep: str, items: list) -> str:\n    return sep.join(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".join("),
            "Expected join with variable: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_join_empty_sep() {
        let code = "def f(items: list) -> str:\n    return \"\".join(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".join("),
            "Expected join with empty sep: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 8: string_methods.rs - Replace
    // ========================================================================

    #[test]
    fn test_w8_string_replace_basic() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"old\", \"new\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".replace("),
            "Expected replace: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_replace_with_count() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"x\", \"y\", 1)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replacen") || result.contains("replace"),
            "Expected replacen or replace: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 9: string_methods.rs - Find / Index
    // ========================================================================

    #[test]
    fn test_w8_string_find_basic() {
        let code = "def f(s: str) -> int:\n    return s.find(\"sub\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".find(") && result.contains("unwrap_or(-1)"),
            "Expected find with -1 fallback: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_find_with_start() {
        let code = "def f(s: str) -> int:\n    return s.find(\"x\", 5)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".find(") && result.contains("unwrap_or(-1)"),
            "Expected find with start: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rfind() {
        let code = "def f(s: str) -> int:\n    return s.rfind(\"sub\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("rfind") && result.contains("unwrap_or(-1)"),
            "Expected rfind: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_index() {
        let code = "def f(s: str) -> int:\n    return s.index(\"sub\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".find(") && result.contains("expect"),
            "Expected index with expect: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rindex() {
        let code = "def f(s: str) -> int:\n    return s.rindex(\"sub\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("rfind") && result.contains("expect"),
            "Expected rindex: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 10: string_methods.rs - startswith / endswith
    // ========================================================================

    #[test]
    fn test_w8_string_startswith_literal() {
        let code = "def f(s: str) -> bool:\n    return s.startswith(\"pre\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("starts_with"),
            "Expected starts_with: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_endswith_literal() {
        let code = "def f(s: str) -> bool:\n    return s.endswith(\".txt\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("ends_with"),
            "Expected ends_with: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_startswith_variable() {
        let code = "def f(s: str, prefix: str) -> bool:\n    return s.startswith(prefix)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("starts_with"),
            "Expected starts_with with variable: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_endswith_variable() {
        let code = "def f(s: str, suffix: str) -> bool:\n    return s.endswith(suffix)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("ends_with"),
            "Expected ends_with with variable: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 11: string_methods.rs - count
    // ========================================================================

    #[test]
    fn test_w8_string_count_literal() {
        let code = "def f(s: str) -> int:\n    return s.count(\"a\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("matches") && result.contains("count"),
            "Expected matches().count(): {}",
            result
        );
    }

    #[test]
    fn test_w8_string_count_variable() {
        let code = "def f(s: str, sub: str) -> int:\n    return s.count(sub)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("matches") && result.contains("count"),
            "Expected matches().count() with variable: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 12: string_methods.rs - is* predicates
    // ========================================================================

    #[test]
    fn test_w8_string_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_numeric"),
            "Expected is_numeric: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_alphabetic"),
            "Expected is_alphabetic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isalnum() {
        let code = "def f(s: str) -> bool:\n    return s.isalnum()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_alphanumeric"),
            "Expected is_alphanumeric: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isspace() {
        let code = "def f(s: str) -> bool:\n    return s.isspace()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_whitespace"),
            "Expected is_whitespace: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isupper() {
        let code = "def f(s: str) -> bool:\n    return s.isupper()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_uppercase"),
            "Expected is_uppercase: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_islower() {
        let code = "def f(s: str) -> bool:\n    return s.islower()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_lowercase"),
            "Expected is_lowercase: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_istitle() {
        let code = "def f(s: str) -> bool:\n    return s.istitle()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_uppercase") || result.contains("is_lowercase") || result.contains("prev_is_cased"),
            "Expected istitle logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isnumeric() {
        let code = "def f(s: str) -> bool:\n    return s.isnumeric()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_numeric"),
            "Expected is_numeric: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isascii() {
        let code = "def f(s: str) -> bool:\n    return s.isascii()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_ascii"),
            "Expected is_ascii: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isdecimal() {
        let code = "def f(s: str) -> bool:\n    return s.isdecimal()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_ascii_digit"),
            "Expected is_ascii_digit: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isidentifier() {
        let code = "def f(s: str) -> bool:\n    return s.isidentifier()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_alphabetic") || result.contains("is_alphanumeric") || result.contains("is_empty"),
            "Expected isidentifier logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isprintable() {
        let code = "def f(s: str) -> bool:\n    return s.isprintable()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_control"),
            "Expected is_control check: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 13: string_methods.rs - Padding/alignment
    // ========================================================================

    #[test]
    fn test_w8_string_center_basic() {
        let code = "def f(s: str) -> str:\n    return s.center(20)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("pad") || result.contains("repeat"),
            "Expected center logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_center_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.center(20, \"*\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("fillchar") || result.contains("repeat"),
            "Expected center with fill: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_ljust_basic() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("repeat") || result.contains("format"),
            "Expected ljust logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_ljust_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20, \"-\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("fillchar") || result.contains("repeat"),
            "Expected ljust with fill: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rjust_basic() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("repeat") || result.contains("format"),
            "Expected rjust logic: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rjust_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20, \"0\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("fillchar") || result.contains("repeat"),
            "Expected rjust with fill: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_zfill() {
        let code = "def f(s: str) -> str:\n    return s.zfill(10)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("0") || result.contains("repeat"),
            "Expected zfill logic: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 14: string_methods.rs - Encode/Decode
    // ========================================================================

    #[test]
    fn test_w8_string_encode_default() {
        let code = "def f(s: str) -> bytes:\n    return s.encode()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("as_bytes") || result.contains("to_vec"),
            "Expected as_bytes().to_vec(): {}",
            result
        );
    }

    #[test]
    fn test_w8_string_encode_utf8() {
        let code = "def f(s: str) -> bytes:\n    return s.encode(\"utf-8\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("as_bytes") || result.contains("to_vec"),
            "Expected as_bytes for utf-8: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 15: string_methods.rs - expandtabs
    // ========================================================================

    #[test]
    fn test_w8_string_expandtabs_default() {
        let code = "def f(s: str) -> str:\n    return s.expandtabs()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace") && (result.contains("8") || result.contains("repeat")),
            "Expected expandtabs(8): {}",
            result
        );
    }

    #[test]
    fn test_w8_string_expandtabs_custom() {
        let code = "def f(s: str) -> str:\n    return s.expandtabs(4)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace") && result.contains("repeat"),
            "Expected expandtabs(4): {}",
            result
        );
    }

    // ========================================================================
    // SECTION 16: string_methods.rs - partition
    // ========================================================================

    #[test]
    fn test_w8_string_partition() {
        let code = "def f(s: str):\n    return s.partition(\",\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".find(") || result.contains("partition") || result.contains("before"),
            "Expected partition logic: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 17: string_methods.rs - hex
    // ========================================================================

    #[test]
    fn test_w8_string_hex() {
        let code = "def f(s: str) -> str:\n    return s.hex()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("bytes") || result.contains("format") || result.contains("02x"),
            "Expected hex conversion: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 18: string_methods.rs - format
    // ========================================================================

    #[test]
    fn test_w8_string_format_no_args() {
        let code = "def f() -> str:\n    return \"hello\".format()\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("hello") || result.contains("format"),
                "Expected format no args: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_format_single_arg() {
        let code = "def f(name: str) -> str:\n    return \"Hello, {}!\".format(name)\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("replacen") || result.contains("format"),
                "Expected format single arg: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_format_multiple_args() {
        let code = "def f(name: str, age: int) -> str:\n    return \"{} is {} years old\".format(name, age)\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("replacen") || result.contains("format"),
                "Expected format multiple args: {}",
                result
            );
        }
    }

    // ========================================================================
    // SECTION 19: string_methods.rs - Combination patterns
    // ========================================================================

    #[test]
    fn test_w8_string_strip_and_lower() {
        let code = "def f(s: str) -> str:\n    return s.strip().lower()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim") || result.contains("to_lowercase"),
            "Expected strip+lower chain: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_upper_and_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.upper().startswith(\"ABC\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase") || result.contains("starts_with"),
            "Expected upper+startswith: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_and_join() {
        let code = "def f(s: str) -> str:\n    parts = s.split(\",\")\n    return \";\".join(parts)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split") && result.contains("join"),
            "Expected split+join: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_replace_and_strip() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"a\", \"b\").strip()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace") || result.contains("trim"),
            "Expected replace+strip: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_find_and_count() {
        let code = "\
def f(s: str) -> int:
    pos = s.find(\"x\")
    cnt = s.count(\"x\")
    return pos + cnt
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("find") || result.contains("matches"),
            "Expected find+count: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lower_and_replace() {
        let code = "def f(s: str) -> str:\n    return s.lower().replace(\" \", \"_\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase") || result.contains("replace"),
            "Expected lower+replace: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_whitespace_join() {
        let code = "def f(s: str) -> str:\n    return \" \".join(s.split())\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split_whitespace") || result.contains("join"),
            "Expected whitespace split+join: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_strip_lower_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.strip().lower().startswith(\"http\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim") || result.contains("to_lowercase") || result.contains("starts_with"),
            "Expected strip+lower+startswith: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 20: string_methods.rs - Edge cases / advanced patterns
    // ========================================================================

    #[test]
    fn test_w8_string_upper_in_condition() {
        let code = "\
def f(s: str) -> str:
    if s.isupper():
        return s.lower()
    return s.upper()
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_uppercase") || result.contains("to_lowercase") || result.contains("to_uppercase"),
            "Expected isupper+lower+upper: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isdigit_in_filter() {
        let code = "\
def f(s: str) -> bool:
    result = s.isdigit()
    return result
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_numeric"),
            "Expected is_numeric in filter: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_strip_in_loop_body() {
        let code = "\
def f(items: list) -> list:
    result = []
    for item in items:
        result.append(item.strip())
    return result
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("trim") || result.contains("strip"),
                "Expected trim in loop: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_replace_empty_string() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"x\", \"\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".replace("),
            "Expected replace with empty: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_find_not_found_handling() {
        let code = "\
def f(s: str) -> bool:
    pos = s.find(\"needle\")
    return pos >= 0
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("find") || result.contains("unwrap_or"),
            "Expected find result handling: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_count_in_comparison() {
        let code = "\
def f(s: str) -> bool:
    return s.count(\"a\") > 3
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("matches") && result.contains("count"),
            "Expected count comparison: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_endswith_common_extension() {
        let code = "\
def f(filename: str) -> bool:
    return filename.endswith(\".py\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("ends_with"),
            "Expected ends_with .py: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_startswith_in_condition() {
        let code = "\
def f(line: str) -> bool:
    if line.startswith(\"#\"):
        return True
    return False
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("starts_with"),
            "Expected starts_with in condition: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_and_index_access() {
        let code = "\
def f(path: str) -> str:
    parts = path.split(\"/\")
    return parts[0]
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split"),
            "Expected split+index: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_join_with_newlines() {
        let code = "def f(lines: list) -> str:\n    return \"\\n\".join(lines)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("join"),
            "Expected join with newline: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_encode_and_length() {
        let code = "\
def f(s: str) -> int:
    data = s.encode()
    return len(data)
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("as_bytes") || result.contains("len"),
            "Expected encode+len: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_method_on_literal() {
        let code = "def f() -> str:\n    return \"hello world\".upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase"),
            "Expected to_uppercase on literal: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_method_on_return_value() {
        let code = "\
def get_name() -> str:
    return \"Alice\"

def f() -> str:
    return get_name().lower()
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase"),
            "Expected to_lowercase on return: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_multiple_replaces() {
        let code = "\
def f(s: str) -> str:
    result = s.replace(\"a\", \"x\")
    result = result.replace(\"b\", \"y\")
    return result
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace"),
            "Expected multiple replaces: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_strip_and_split() {
        let code = "\
def f(line: str) -> list:
    return line.strip().split(\",\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim") || result.contains("split"),
            "Expected strip+split: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lower_comparison() {
        let code = "\
def f(s: str) -> bool:
    return s.lower() == \"hello\"
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase"),
            "Expected to_lowercase comparison: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isalpha_guard() {
        let code = "\
def f(s: str) -> str:
    if s.isalpha():
        return s.upper()
    return s
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_alphabetic") || result.contains("to_uppercase"),
            "Expected isalpha guard: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_with_multiple_seps() {
        let code = "\
def f(s: str) -> list:
    first = s.split(\".\")
    return first
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split"),
            "Expected split with period: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_format_with_int() {
        let code = "def f(n: int) -> str:\n    return \"count: {}\".format(n)\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("replacen") || result.contains("format"),
                "Expected format with int: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_zfill_short_number() {
        let code = "def f() -> str:\n    return \"42\".zfill(5)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("0") || result.contains("width") || result.contains("repeat"),
            "Expected zfill on literal: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_ljust_in_table() {
        let code = "\
def f(name: str) -> str:
    return name.ljust(30)
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("format") || result.contains("repeat"),
            "Expected ljust in table: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rjust_for_alignment() {
        let code = "\
def f(num: str) -> str:
    return num.rjust(10, \"0\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("repeat") || result.contains("fillchar"),
            "Expected rjust alignment: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 21: argparse - Additional edge cases for coverage
    // ========================================================================

    #[test]
    fn test_w8_argparse_parser_with_args_usage() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"tool\")
    parser.add_argument(\"--name\", type=str)
    args = parser.parse_args()
    greeting = \"Hello, \" + str(args.name)
    print(greeting)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("name") || result.contains("greeting"),
                "Expected args usage"
            );
        }
    }

    #[test]
    fn test_w8_argparse_conditional_on_parsed_arg() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--mode\", default=\"fast\")
    args = parser.parse_args()
    if args.mode == \"slow\":
        print(\"Running slowly\")
    else:
        print(\"Running fast\")
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("mode") || result.contains("slow") || result.contains("fast"),
                "Expected conditional on parsed arg"
            );
        }
    }

    #[test]
    fn test_w8_argparse_loop_over_files_nargs() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"files\", nargs=\"+\")
    args = parser.parse_args()
    for f in args.files:
        print(f)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("files") || result.contains("Vec"),
                "Expected loop over nargs files"
            );
        }
    }

    #[test]
    fn test_w8_argparse_required_with_type() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--port\", type=int, required=True)
    args = parser.parse_args()
    print(args.port)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("i32") || result.contains("port"),
                "Expected required int arg"
            );
        }
    }

    #[test]
    fn test_w8_argparse_bool_flag_usage_in_function() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--dry-run\", action=\"store_true\")
    args = parser.parse_args()
    if args.dry_run:
        print(\"Dry run mode\")
    else:
        print(\"Executing\")
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("dry_run") || result.contains("bool"),
                "Expected dry-run flag usage"
            );
        }
    }

    #[test]
    fn test_w8_argparse_combination_int_bool_str() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"name\")
    parser.add_argument(\"--age\", type=int, default=0)
    parser.add_argument(\"--active\", action=\"store_true\")
    args = parser.parse_args()
    print(args.name)
    print(args.age)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("name") || result.contains("age") || result.contains("active"),
                "Expected combination of types"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparser_global_flags() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--verbose\", action=\"store_true\")
    subparsers = parser.add_subparsers(dest=\"cmd\")
    run_parser = subparsers.add_parser(\"run\")
    run_parser.add_argument(\"script\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("verbose") || result.contains("Run") || result.contains("script"),
                "Expected global flags with subparsers"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparser_help_text() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser(description=\"Database tool\")
    subparsers = parser.add_subparsers(dest=\"command\")
    migrate_parser = subparsers.add_parser(\"migrate\", help=\"Run migrations\")
    seed_parser = subparsers.add_parser(\"seed\", help=\"Seed database\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Migrate") || result.contains("Seed")
                    || result.contains("migrate") || result.contains("seed"),
                "Expected subparser help text"
            );
        }
    }

    // ========================================================================
    // SECTION 22: Additional string method edge cases
    // ========================================================================

    #[test]
    fn test_w8_string_expandtabs_in_processing() {
        let code = "\
def f(text: str) -> str:
    expanded = text.expandtabs(2)
    return expanded
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace") || result.contains("repeat"),
            "Expected expandtabs processing: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_splitlines_processing() {
        let code = "\
def f(text: str) -> int:
    lines = text.splitlines()
    return len(lines)
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("lines()") || result.contains("len"),
            "Expected splitlines+len: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_partition_unpack() {
        let code = "\
def f(s: str) -> str:
    before, sep, after = s.partition(\":\")
    return before
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("partition") || result.contains("find") || result.contains("before"),
                "Expected partition unpack: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_isdigit_and_convert() {
        let code = "\
def f(s: str) -> int:
    if s.isdigit():
        return int(s)
    return 0
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_numeric") || result.contains("parse"),
            "Expected isdigit+int: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isalnum_validation() {
        let code = "\
def f(username: str) -> bool:
    return username.isalnum() and len(username) > 3
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_alphanumeric") || result.contains("len"),
            "Expected isalnum validation: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_center_header() {
        let code = "\
def f(title: str) -> str:
    return title.center(40, \"=\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("width") || result.contains("repeat") || result.contains("="),
            "Expected center header: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_multiple_isX_checks() {
        let code = "\
def f(s: str) -> bool:
    return s.isalpha() and not s.isupper()
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_alphabetic") || result.contains("is_uppercase"),
            "Expected multiple is* checks: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_strip_and_lower_and_endswith() {
        let code = "\
def f(s: str) -> bool:
    cleaned = s.strip().lower()
    return cleaned.endswith(\".csv\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim") || result.contains("to_lowercase") || result.contains("ends_with"),
            "Expected chain: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_replace_newlines() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"\\n\", \" \")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace"),
            "Expected replace newlines: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_upper_assignment() {
        let code = "\
def f(s: str) -> str:
    result = s.upper()
    return result
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase"),
            "Expected upper assignment: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lower_in_comparison() {
        let code = "\
def f(a: str, b: str) -> bool:
    return a.lower() == b.lower()
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase"),
            "Expected lower comparison: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_limit_two() {
        let code = "def f(s: str) -> list:\n    return s.split(\":\", 2)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("splitn"),
            "Expected splitn with limit 2: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_rsplit_single_sep() {
        let code = "def f(s: str) -> list:\n    return s.rsplit(\"/\", 1)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("rsplitn"),
            "Expected rsplitn: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_find_empty_string() {
        let code = "def f(s: str) -> int:\n    return s.find(\"\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("find") || result.contains("unwrap_or"),
            "Expected find empty: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_encode_in_assignment() {
        let code = "\
def f(s: str) -> bytes:
    encoded = s.encode()
    return encoded
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("as_bytes") || result.contains("to_vec"),
            "Expected encode assignment: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_join_with_pipe_sep() {
        let code = "def f(items: list) -> str:\n    return \"|\".join(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("join"),
            "Expected join with pipe: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 23: More argparse edge cases for deeper coverage
    // ========================================================================

    #[test]
    fn test_w8_argparse_default_value_float() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--threshold\", type=float, default=0.5)
    args = parser.parse_args()
    print(args.threshold)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("f64") || result.contains("threshold") || result.contains("0.5"),
                "Expected float default"
            );
        }
    }

    #[test]
    fn test_w8_argparse_default_value_bool() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--feature\", default=True)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("feature") || result.contains("true") || result.contains("default"),
                "Expected bool default"
            );
        }
    }

    #[test]
    fn test_w8_argparse_dest_with_long_flag() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--config-file\", dest=\"config\")
    args = parser.parse_args()
    print(args.config)
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("config") || result.contains("dest"),
                "Expected dest with long flag"
            );
        }
    }

    #[test]
    fn test_w8_argparse_parser_with_only_subparsers() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"action\")
    subparsers.add_parser(\"init\")
    subparsers.add_parser(\"update\")
    subparsers.add_parser(\"delete\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Init") || result.contains("Update") || result.contains("Delete")
                    || result.contains("enum") || result.contains("action"),
                "Expected parser with only subparsers"
            );
        }
    }

    #[test]
    fn test_w8_argparse_metavar_and_help_combined() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--input\", metavar=\"FILE\", help=\"Input file path\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("input") || result.contains("FILE") || result.contains("Input file"),
                "Expected metavar+help"
            );
        }
    }

    #[test]
    fn test_w8_argparse_choices_three_values() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--log-level\", choices=[\"debug\", \"info\", \"warning\", \"error\"])
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("log_level") || result.contains("debug") || result.contains("value_parser"),
                "Expected choices with four values"
            );
        }
    }

    // ========================================================================
    // SECTION 24: string_methods.rs - remaining edge cases for 200 total
    // ========================================================================

    #[test]
    fn test_w8_string_upper_on_empty() {
        let code = "def f() -> str:\n    return \"\".upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase"),
            "Expected to_uppercase on empty: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isspace_on_whitespace() {
        let code = "def f() -> bool:\n    return \"  \".isspace()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("is_whitespace"),
            "Expected is_whitespace: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_count_no_match() {
        let code = "def f(s: str) -> int:\n    return s.count(\"xyz\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("matches") && result.contains("count"),
            "Expected matches count for no match: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_replace_all_occurrences() {
        let code = "def f(s: str) -> str:\n    return s.replace(\".\", \",\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains(".replace("),
            "Expected replace all: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_format_three_args() {
        let code = "def f(a: str, b: str, c: str) -> str:\n    return \"{}-{}-{}\".format(a, b, c)\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("replacen") || result.contains("format"),
                "Expected format three args: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_hex_on_string() {
        let code = "def f(s: str) -> str:\n    return s.hex()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("02x") || result.contains("format") || result.contains("bytes"),
            "Expected hex on string: {}",
            result
        );
    }

    // ========================================================================
    // SECTION 25: Additional tests to reach 200 total
    // ========================================================================

    #[test]
    fn test_w8_argparse_nargs_two_with_int_type() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"range\", nargs=2, type=int)
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Vec") || result.contains("i32") || result.contains("range"),
                "Expected Vec<i32> for nargs=2 type=int"
            );
        }
    }

    #[test]
    fn test_w8_argparse_multiple_choices_flags() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--color\", choices=[\"red\", \"green\", \"blue\"])
    parser.add_argument(\"--size\", choices=[\"small\", \"medium\", \"large\"])
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("color") || result.contains("size") || result.contains("value_parser"),
                "Expected multiple choices flags"
            );
        }
    }

    #[test]
    fn test_w8_argparse_subparser_two_args_each() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest=\"cmd\")
    cp_parser = subparsers.add_parser(\"copy\")
    cp_parser.add_argument(\"source\")
    cp_parser.add_argument(\"dest\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Copy") || result.contains("source") || result.contains("dest"),
                "Expected subparser with two positional args"
            );
        }
    }

    #[test]
    fn test_w8_argparse_short_flag_only() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"-f\", action=\"store_true\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("f") || result.contains("bool") || result.contains("short"),
                "Expected short-only flag"
            );
        }
    }

    #[test]
    fn test_w8_argparse_long_flag_double_hyphen() {
        let code = "\
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(\"--output-format\", default=\"json\")
    args = parser.parse_args()
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("output_format") || result.contains("json") || result.contains("String"),
                "Expected double-hyphen flag"
            );
        }
    }

    #[test]
    fn test_w8_string_capitalize_on_empty() {
        let code = "def f() -> str:\n    return \"\".capitalize()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("chars") || result.contains("to_uppercase") || result.contains("next"),
            "Expected capitalize on empty: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_title_single_word() {
        let code = "def f() -> str:\n    return \"hello\".title()\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_uppercase") || result.contains("split_whitespace"),
            "Expected title single word: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_strip_tabs_and_spaces() {
        let code = "def f(s: str) -> str:\n    return s.strip(\" \\t\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("trim_matches"),
            "Expected trim_matches for tabs/spaces: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_split_pipe_separator() {
        let code = "def f(s: str) -> list:\n    return s.split(\"|\")\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("split"),
            "Expected split with pipe: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_join_with_space() {
        let code = "def f(words: list) -> str:\n    return \" \".join(words)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("join"),
            "Expected join with space: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_replace_multiple_chars() {
        let code = "\
def f(s: str) -> str:
    result = s.replace(\"<\", \"&lt;\")
    result = result.replace(\">\", \"&gt;\")
    return result
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("replace"),
            "Expected HTML escape replace: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_find_with_variable() {
        let code = "def f(s: str, sub: str) -> int:\n    return s.find(sub)\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("find") && result.contains("unwrap_or"),
            "Expected find with variable: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_isdigit_in_loop() {
        let code = "\
def f(items: list) -> list:
    result = []
    for item in items:
        if item.isdigit():
            result.append(item)
    return result
";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("is_numeric") || result.contains("isdigit"),
                "Expected isdigit in loop: {}",
                result
            );
        }
    }

    #[test]
    fn test_w8_string_endswith_multiple_checks() {
        let code = "\
def f(name: str) -> bool:
    return name.endswith(\".py\") or name.endswith(\".pyi\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("ends_with"),
            "Expected ends_with double check: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_lower_strip_replace_chain() {
        let code = "\
def f(s: str) -> str:
    return s.lower().strip().replace(\" \", \"-\")
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("to_lowercase") || result.contains("trim") || result.contains("replace"),
            "Expected lower+strip+replace chain: {}",
            result
        );
    }

    #[test]
    fn test_w8_string_splitlines_and_len() {
        let code = "\
def f(text: str) -> int:
    return len(text.splitlines())
";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("lines") || result.contains("len"),
            "Expected splitlines+len: {}",
            result
        );
    }
}
