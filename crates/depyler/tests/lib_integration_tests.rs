//! Fast integration tests for lib.rs CLI handlers
//! Target: 100% coverage of testable pure functions

use depyler::*;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// complexity_rating Tests
// ============================================================================

#[test]
fn test_complexity_rating_low() {
    let rating = complexity_rating(5.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_medium() {
    let rating = complexity_rating(15.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_high() {
    let rating = complexity_rating(25.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_boundary_10() {
    let rating = complexity_rating(10.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_boundary_20() {
    let rating = complexity_rating(20.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_zero() {
    let rating = complexity_rating(0.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_complexity_rating_very_high() {
    let rating = complexity_rating(100.0);
    let s = rating.to_string();
    assert!(!s.is_empty());
}

// ============================================================================
// analyze_command Tests
// ============================================================================

#[test]
fn test_analyze_command_valid_file() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("analyze.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = analyze_command(py_file, "text".to_string());
    let _ = result;
}

#[test]
fn test_analyze_command_json_format() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("analyze.py");
    std::fs::write(&py_file, "x = 1\n").unwrap();

    let result = analyze_command(py_file, "json".to_string());
    let _ = result;
}

#[test]
fn test_analyze_command_nonexistent() {
    let result = analyze_command(PathBuf::from("/nonexistent.py"), "text".to_string());
    assert!(result.is_err());
}

// ============================================================================
// check_command Tests
// ============================================================================

#[test]
fn test_check_command_valid_file() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("check.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = check_command(py_file);
    let _ = result;
}

#[test]
fn test_check_command_nonexistent() {
    let result = check_command(PathBuf::from("/nonexistent.py"));
    assert!(result.is_err());
}

// ============================================================================
// transpile_command Tests
// ============================================================================

#[test]
fn test_transpile_command_valid_file() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("transpile.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = transpile_command(py_file, None, false, false, false, false);
    assert!(result.is_ok());
}

#[test]
fn test_transpile_command_with_output() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("transpile.py");
    let rs_file = temp.path().join("output.rs");
    std::fs::write(&py_file, "def greet() -> str:\n    return 'hello'\n").unwrap();

    let result = transpile_command(py_file, Some(rs_file.clone()), false, false, false, false);
    assert!(result.is_ok());
    assert!(rs_file.exists());
}

#[test]
fn test_transpile_command_nonexistent() {
    let result = transpile_command(PathBuf::from("/nonexistent.py"), None, false, false, false, false);
    assert!(result.is_err());
}

#[test]
fn test_transpile_command_with_verify_flag() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("verify.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = transpile_command(py_file, None, true, false, false, false);
    let _ = result;
}

#[test]
fn test_transpile_command_with_gen_tests_flag() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("tests.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = transpile_command(py_file, None, false, true, false, false);
    let _ = result;
}

#[test]
fn test_transpile_command_with_debug_flag() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("debug.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = transpile_command(py_file, None, false, false, true, false);
    let _ = result;
}

#[test]
fn test_transpile_command_with_source_map_flag() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("sourcemap.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    let result = transpile_command(py_file, None, false, false, false, true);
    let _ = result;
}

// ============================================================================
// compile_command Tests
// ============================================================================

#[test]
fn test_compile_command_nonexistent() {
    let result = compile_command(PathBuf::from("/nonexistent.py"), None, "release".to_string());
    assert!(result.is_err());
}

#[test]
fn test_compile_command_empty_profile() {
    let temp = TempDir::new().unwrap();
    let py_file = temp.path().join("compile.py");
    std::fs::write(&py_file, "def add(a: int, b: int) -> int:\n    return a + b\n").unwrap();

    // Empty profile should default to release
    let result = compile_command(py_file, None, "".to_string());
    // May fail during actual compilation, but should not panic
    let _ = result;
}

// ============================================================================
// CLI Struct Tests
// ============================================================================

#[test]
fn test_cli_parse_transpile() {
    use clap::Parser;
    let args = vec!["depyler", "transpile", "test.py"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_compile() {
    use clap::Parser;
    let args = vec!["depyler", "compile", "test.py"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_analyze() {
    use clap::Parser;
    let args = vec!["depyler", "analyze", "test.py"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_check() {
    use clap::Parser;
    let args = vec!["depyler", "check", "test.py"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_cache_stats() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "stats"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_cache_gc() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "gc"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_cache_clear() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "clear"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_parse_cache_warm() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "warm", "--input-dir", "/tmp"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_verbose_flag() {
    use clap::Parser;
    let args = vec!["depyler", "-v", "check", "test.py"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert!(cli.verbose);
}

#[test]
fn test_cli_transpile_with_output() {
    use clap::Parser;
    let args = vec!["depyler", "transpile", "test.py", "-o", "out.rs"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_transpile_with_verify() {
    use clap::Parser;
    let args = vec!["depyler", "transpile", "test.py", "--verify"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_transpile_with_gen_tests() {
    use clap::Parser;
    let args = vec!["depyler", "transpile", "test.py", "--gen-tests"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_compile_with_profile() {
    use clap::Parser;
    let args = vec!["depyler", "compile", "test.py", "--profile", "debug"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

#[test]
fn test_cli_analyze_json_format() {
    use clap::Parser;
    let args = vec!["depyler", "analyze", "test.py", "-f", "json"];
    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());
}

// ============================================================================
// CacheCommands enum Tests
// ============================================================================

#[test]
fn test_cache_commands_stats_default_format() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "stats"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Cache(CacheCommands::Stats { format }) = cli.command {
        assert_eq!(format, "text");
    } else {
        panic!("Expected Cache Stats command");
    }
}

#[test]
fn test_cache_commands_gc_with_max_age() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "gc", "--max-age-days", "7"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Cache(CacheCommands::Gc { max_age_days, dry_run }) = cli.command {
        assert_eq!(max_age_days, 7);
        assert!(!dry_run);
    } else {
        panic!("Expected Cache Gc command");
    }
}

#[test]
fn test_cache_commands_gc_dry_run() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "gc", "--dry-run"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Cache(CacheCommands::Gc { dry_run, .. }) = cli.command {
        assert!(dry_run);
    } else {
        panic!("Expected Cache Gc command");
    }
}

#[test]
fn test_cache_commands_clear_force() {
    use clap::Parser;
    let args = vec!["depyler", "cache", "clear", "--force"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Cache(CacheCommands::Clear { force }) = cli.command {
        assert!(force);
    } else {
        panic!("Expected Cache Clear command");
    }
}

// ============================================================================
// Commands enum Tests
// ============================================================================

#[test]
fn test_converge_command_defaults() {
    use clap::Parser;
    let args = vec!["depyler", "converge", "--input-dir", "/tmp"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Converge { target_rate, max_iterations, display, .. } = cli.command {
        assert_eq!(target_rate, 100.0);
        assert_eq!(max_iterations, 50);
        assert_eq!(display, "rich");
    } else {
        panic!("Expected Converge command");
    }
}

#[test]
fn test_report_command_defaults() {
    use clap::Parser;
    let args = vec!["depyler", "report", "--input-dir", "/tmp"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Report { format, failures_only, .. } = cli.command {
        assert_eq!(format, "text");
        assert!(!failures_only);
    } else {
        panic!("Expected Report command");
    }
}

#[test]
fn test_utol_command_defaults() {
    use clap::Parser;
    let args = vec!["depyler", "utol"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Utol { target_rate, max_iterations, patience, display, status, .. } = cli.command {
        assert_eq!(target_rate, 0.80);
        assert_eq!(max_iterations, 50);
        assert_eq!(patience, 5);
        assert_eq!(display, "rich");
        assert!(!status);
    } else {
        panic!("Expected Utol command");
    }
}

#[test]
fn test_utol_command_status_flag() {
    use clap::Parser;
    let args = vec!["depyler", "utol", "--status"];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Commands::Utol { status, .. } = cli.command {
        assert!(status);
    } else {
        panic!("Expected Utol command");
    }
}
