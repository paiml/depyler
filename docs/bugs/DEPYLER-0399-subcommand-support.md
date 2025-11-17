# DEPYLER-0399: Clap Subcommand Support Missing

**Status**: 🛑 STOP THE LINE
**Priority**: P0 (STOP ALL WORK) - Blocks 3/11 examples (27% of test suite)
**Created**: 2025-11-17
**Affects**: example_subcommands, example_config, example_io_streams

## Problem Statement

The DEPYLER transpiler does not support `argparse.add_subparsers()` patterns, preventing transpilation of git-like CLIs with subcommands. This blocks 3 out of 11 examples in the validation suite.

### Minimal Reproduction

**Input Python** (`test_subcommand_minimal.py`):
```python
import argparse

def main():
    parser = argparse.ArgumentParser(description="Git-like CLI")

    parser.add_argument("--verbose", action="store_true", help="Verbose output")

    subparsers = parser.add_subparsers(dest="command", required=True)

    # Subcommand: clone
    parser_clone = subparsers.add_parser("clone", help="Clone a repository")
    parser_clone.add_argument("url", help="Repository URL")

    # Subcommand: push
    parser_push = subparsers.add_parser("push", help="Push changes")
    parser_push.add_argument("remote", help="Remote name")

    args = parser.parse_args()

    if args.command == "clone":
        print(f"Cloning {args.url}")
    elif args.command == "push":
        print(f"Pushing to {args.remote}")

if __name__ == "__main__":
    main()
```

**Current Transpiled Output** (WRONG):
```rust
use clap::Parser;

pub fn main() {
    #[derive(clap::Parser)]
    #[command(about = "Git-like CLI")]
    struct Args {
        #[arg(long)]
        #[arg(action = clap::ArgAction::SetTrue)]
        #[doc = "Verbose output"]
        verbose: bool,
    }

    // ERROR: parser variable doesn't exist in scope
    let subparsers = parser.add_subparsers();
    let parser_clone = subparsers.add_parser("clone");
    let parser_push = subparsers.add_parser("push");

    let args = Args::parse();

    // ERROR: Args struct has no 'command' field
    if args.command == "clone" {
        // ERROR: Args struct has no 'url' field
        println!("{}", format!("Cloning {:?}", args.url));
    } else if args.command == "push" {
        // ERROR: Args struct has no 'remote' field
        println!("{}", format!("Pushing to {:?}", args.remote));
    }
}
```

**Expected Transpiled Output**:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Git-like CLI")]
struct Args {
    #[arg(long)]
    #[doc = "Verbose output"]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Clone a repository")]
    Clone {
        #[doc = "Repository URL"]
        url: String,
    },
    #[command(about = "Push changes")]
    Push {
        #[doc = "Remote name"]
        remote: String,
    },
}

pub fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Clone { url } => {
            println!("Cloning {}", url);
        }
        Commands::Push { remote } => {
            println!("Pushing to {}", remote);
        }
    }
}
```

## Compilation Errors

**example_subcommands/git_clone.rs**:
```
error[E0425]: cannot find value `parser` in this scope
  --> git_clone.rs:17:22
   |
17 |     let subparsers = parser.add_subparsers();
   |                      ^^^^^^ not found in this scope

error[E0609]: no field `command` on type `main::Args`
  --> git_clone.rs:25:28
   |
25 |     let _cse_temp_0 = args.command == "clone";
   |                            ^^^^^^^ unknown field
```

## Root Cause Analysis

### Architecture Gap
The transpiler's `ArgParserTracker` only handles flat argument structures. It lacks:
1. Subparser tracking (`add_subparsers()` → Commands enum)
2. Subcommand argument tracking (parser_clone.add_argument() → Clone variant fields)
3. Code generation for Subcommand derive macro
4. Match statement generation for subcommand dispatch

### Current Implementation
**File**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

The `ArgParserTracker` struct:
```rust
pub struct ArgParserTracker {
    /// Currently active ArgumentParser instances
    pub parsers: HashMap<String, ArgParserInfo>,

    /// Argument group variables → parent parser
    pub group_to_parser: HashMap<String, String>,

    /// Whether we've generated the Args struct
    pub struct_generated: bool,
}
```

**Missing**:
- `subparsers: HashMap<String, SubparserInfo>` - Track subparser collections
- `subcommands: HashMap<String, SubcommandInfo>` - Track individual subcommands
- Commands enum generation logic
- Match statement generation for if/elif chains

### Pattern Analysis

**Python Pattern 1**: Subparser Creation
```python
subparsers = parser.add_subparsers(dest="command", required=True)
```
- `dest="command"` → Field name in Args struct
- `required=True` → Make Commands enum mandatory (not Option<Commands>)
- Need to track: subparsers variable → parent parser mapping

**Python Pattern 2**: Subcommand Definition
```python
parser_clone = subparsers.add_parser("clone", help="Clone a repository")
parser_clone.add_argument("url", help="Repository URL")
```
- "clone" → Commands enum variant name (Clone)
- help text → #[command(about = "...")]
- Arguments added to parser_clone → Fields in Clone variant

**Python Pattern 3**: Subcommand Dispatch
```python
if args.command == "clone":
    handle_clone(args)
elif args.command == "push":
    handle_push(args)
```
- Convert to match statement on args.command
- Extract variant fields for handler parameters

## Solution Design

### Phase 1: Data Structures

**File**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

Add subparser tracking:
```rust
/// Information about a subparser collection (from add_subparsers())
#[derive(Debug, Clone)]
pub struct SubparserInfo {
    /// Parent parser variable name
    pub parser_var: String,
    /// Destination field name (from dest= parameter)
    pub dest_field: String,
    /// Whether subcommand is required
    pub required: bool,
}

/// Information about a single subcommand
#[derive(Debug, Clone)]
pub struct SubcommandInfo {
    /// Subcommand name (e.g., "clone")
    pub name: String,
    /// Help text
    pub help: String,
    /// Arguments specific to this subcommand
    pub arguments: Vec<ArgInfo>,
}

pub struct ArgParserTracker {
    pub parsers: HashMap<String, ArgParserInfo>,
    pub group_to_parser: HashMap<String, String>,

    /// NEW: Subparser collections (variable → info)
    pub subparsers: HashMap<String, SubparserInfo>,

    /// NEW: Subcommands (parser variable → info)
    pub subcommands: HashMap<String, SubcommandInfo>,

    pub struct_generated: bool,
}
```

### Phase 2: Pattern Detection

**Location**: `crates/depyler-core/src/rust_gen/stmt_gen.rs` in `codegen_assign_stmt`

Detect `subparsers = parser.add_subparsers(...)`:
```rust
// Pattern: subparsers = parser.add_subparsers(dest="command", required=True)
if method == "add_subparsers" {
    if let HirExpr::Var(parser_var) = object.as_ref() {
        if ctx.argparser_tracker.get_parser(parser_var).is_some() {
            // Extract dest and required from kwargs
            let dest_field = extract_kwarg(kwargs, "dest").unwrap_or("command".to_string());
            let required = extract_kwarg_bool(kwargs, "required").unwrap_or(false);

            if let AssignTarget::Symbol(subparsers_var) = target {
                ctx.argparser_tracker.register_subparsers(
                    subparsers_var.clone(),
                    SubparserInfo {
                        parser_var: parser_var.clone(),
                        dest_field,
                        required,
                    }
                );
            }
            return Ok(quote! {});  // Skip generating code
        }
    }
}
```

Detect `parser_clone = subparsers.add_parser("clone", help="...")`:
```rust
// Pattern: parser_clone = subparsers.add_parser("clone", help="...")
if method == "add_parser" {
    if let HirExpr::Var(subparsers_var) = object.as_ref() {
        if ctx.argparser_tracker.get_subparsers(subparsers_var).is_some() {
            // Extract command name from first positional arg
            let command_name = extract_string_literal(&args[0]);
            let help = extract_kwarg(kwargs, "help").unwrap_or_default();

            if let AssignTarget::Symbol(parser_var) = target {
                ctx.argparser_tracker.register_subcommand(
                    parser_var.clone(),
                    SubcommandInfo {
                        name: command_name,
                        help,
                        arguments: vec![],
                    }
                );
            }
            return Ok(quote! {});  // Skip generating code
        }
    }
}
```

### Phase 3: Argument Tracking

Track `parser_clone.add_argument("url")`:
```rust
// In existing add_argument handling
if let Some(subcommand_info) = ctx.argparser_tracker.get_subcommand_mut(parser_var) {
    // This is an argument for a subcommand, not main parser
    let arg_info = ArgInfo {
        name: arg_name,
        arg_type,
        default,
        help,
        action,
        required,
        choices,
        is_positional,
    };
    subcommand_info.arguments.push(arg_info);
    return Ok(quote! {});
}
```

### Phase 4: Code Generation

**Location**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

Generate Commands enum:
```rust
pub fn generate_commands_enum(&self, ctx: &RustGenContext) -> TokenStream {
    let subcommands: Vec<_> = self.subcommands.values().collect();

    let variants: Vec<TokenStream> = subcommands.iter().map(|cmd| {
        let variant_name = format_ident!("{}", to_pascal_case(&cmd.name));
        let help = &cmd.help;

        let fields: Vec<TokenStream> = cmd.arguments.iter().map(|arg| {
            let field_name = format_ident!("{}", arg.name.trim_start_matches("--"));
            let field_type = arg_type_to_rust(&arg.arg_type);
            let doc = &arg.help;

            quote! {
                #[doc = #doc]
                #field_name: #field_type
            }
        }).collect();

        quote! {
            #[command(about = #help)]
            #variant_name {
                #(#fields),*
            }
        }
    }).collect();

    quote! {
        #[derive(clap::Subcommand)]
        enum Commands {
            #(#variants),*
        }
    }
}
```

Add command field to Args:
```rust
// In generate_args_struct(), after existing fields:
if !self.subparsers.is_empty() {
    let subparser_info = self.subparsers.values().next().unwrap();

    quote! {
        #[command(subcommand)]
        command: Commands
    }
}
```

### Phase 5: Match Generation

Convert if/elif chains to match:
```rust
// In stmt_gen.rs, detect pattern:
// if args.command == "clone": ...
// elif args.command == "push": ...

// Generate:
match args.command {
    Commands::Clone { url } => {
        handle_clone(Args { verbose: args.verbose, url });
    }
    Commands::Push { remote } => {
        handle_push(Args { verbose: args.verbose, remote });
    }
}
```

## Implementation Plan

### Task Breakdown (3-4 days estimated)

**Day 1: Data Structures & Pattern Detection**
- [ ] Add SubparserInfo, SubcommandInfo structs
- [ ] Add subparsers, subcommands fields to ArgParserTracker
- [ ] Implement register_subparsers(), register_subcommand() methods
- [ ] Detect add_subparsers() pattern in stmt_gen.rs
- [ ] Detect add_parser() pattern in stmt_gen.rs
- [ ] Write unit tests for pattern detection

**Day 2: Argument Tracking**
- [ ] Route subcommand arguments to correct SubcommandInfo
- [ ] Handle positional vs optional arguments in subcommands
- [ ] Handle help text and action attributes
- [ ] Write unit tests for argument routing

**Day 3: Code Generation**
- [ ] Implement generate_commands_enum()
- [ ] Add command field to Args struct
- [ ] Generate proper derives and attributes
- [ ] Write integration tests

**Day 4: Match Statement & Integration**
- [ ] Detect if/elif command dispatch pattern
- [ ] Generate match statement with variant destructuring
- [ ] Update handler function signatures
- [ ] Re-transpile all 3 blocked examples
- [ ] Verify compilation success

## Test Strategy

### Unit Tests
```rust
#[test]
fn test_subparser_registration() {
    let mut tracker = ArgParserTracker::new();
    tracker.register_parser("parser".to_string(), ArgParserInfo::default());
    tracker.register_subparsers(
        "subparsers".to_string(),
        SubparserInfo {
            parser_var: "parser".to_string(),
            dest_field: "command".to_string(),
            required: true,
        }
    );
    assert!(tracker.get_subparsers("subparsers").is_some());
}

#[test]
fn test_subcommand_registration() {
    let mut tracker = ArgParserTracker::new();
    // ... setup ...
    tracker.register_subcommand(
        "parser_clone".to_string(),
        SubcommandInfo {
            name: "clone".to_string(),
            help: "Clone a repository".to_string(),
            arguments: vec![],
        }
    );
    assert_eq!(tracker.subcommands.len(), 1);
}
```

### Integration Tests
```rust
#[test]
fn test_depyler_0399_subcommand_minimal() {
    let python_code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    parser_clone = subparsers.add_parser("clone")
    parser_clone.add_argument("url")
    args = parser.parse_args()
    if args.command == "clone":
        print(args.url)
"#;

    let rust_code = transpile_and_format(python_code).unwrap();

    // Verify Commands enum exists
    assert!(rust_code.contains("enum Commands"));
    assert!(rust_code.contains("Clone { url: String }"));

    // Verify Args has command field
    assert!(rust_code.contains("command: Commands"));

    // Verify match statement
    assert!(rust_code.contains("match args.command"));

    // Verify compilation
    assert!(compile_rust_code(&rust_code).is_ok());
}
```

### Property Tests
```rust
#[quickcheck]
fn prop_subcommand_count_preserved(commands: Vec<String>) -> bool {
    let python = generate_subcommand_python(&commands);
    let rust = transpile(&python).unwrap();
    count_enum_variants(&rust) == commands.len()
}
```

## Complexity Analysis

### Cyclomatic Complexity
- `register_subparsers()`: 2 (simple insert)
- `register_subcommand()`: 2 (simple insert)
- `generate_commands_enum()`: 5 (iteration + conditionals)
- `detect_subparser_pattern()`: 8 (nested if checks)
- **Total**: ≤10 per function ✅

### Cognitive Complexity
- Pattern matching logic: 7 (acceptable)
- Enum generation: 5 (acceptable)
- Match generation: 6 (acceptable)

## Risk Assessment

### High Risk
- **Handler function signatures**: Need to destructure variant fields and pass to handlers
- **Nested subcommands**: Not handling multi-level subparsers yet (defer to future)

### Medium Risk
- **Argument type inference**: Subcommand arguments might have different types than inferred
- **Optional vs required subcommands**: Need to handle Option<Commands> correctly

### Low Risk
- **Basic pattern detection**: Well-defined patterns in Python
- **Enum generation**: Straightforward code generation

## Success Criteria

1. ✅ example_subcommands compiles without errors
2. ✅ example_config compiles without errors
3. ✅ example_io_streams compiles without errors
4. ✅ All 3 examples pass `cargo build --release`
5. ✅ All 3 examples pass `cargo clippy -- -D warnings`
6. ✅ Unit tests achieve 85%+ coverage
7. ✅ Integration tests pass
8. ✅ Compilation success rate: 7/11 (63.6%) → significant progress
9. ✅ Zero cyclomatic complexity violations (≤10)
10. ✅ Zero SATD (no TODOs/FIXMEs)

## References

- **Clap Subcommands Docs**: https://docs.rs/clap/latest/clap/_derive/index.html#subcommands
- **Python argparse.add_subparsers**: https://docs.python.org/3/library/argparse.html#argparse.ArgumentParser.add_subparsers
- **GitHub Issue #3**: Validation of 11 Python CLI examples
- **Related Bugs**: DEPYLER-0394 (argparse tracking), DEPYLER-0396 (nested groups)

## Impact

**Compilation Success Rate**:
- Current: 4/11 (36.4%)
- After fix: 7/11 (63.6%)
- **Improvement**: +27.2 percentage points

**User Impact**:
- Enables git-like CLI patterns
- Unblocks real-world tools (config managers, I/O processors)
- Demonstrates production-ready argparse→clap transformation

---

**STOP THE LINE Protocol**: This bug is now documented. Proceeding to implementation phase with EXTREME TDD methodology.
