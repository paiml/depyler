//! Debugging support for Depyler
//!
//! This module provides debugging features including:
//! - Source map generation
//! - Debug symbol preservation
//! - Debugger integration helpers
//! - Runtime debugging utilities

use crate::hir::{HirFunction, Type};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Source mapping information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    /// Original Python source file
    pub source_file: PathBuf,
    /// Generated Rust file
    pub target_file: PathBuf,
    /// Mapping entries
    pub mappings: Vec<SourceMapping>,
    /// Function mappings
    pub function_map: HashMap<String, FunctionMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMapping {
    /// Python source location
    pub python_line: usize,
    pub python_column: usize,
    /// Rust target location
    pub rust_line: usize,
    pub rust_column: usize,
    /// Optional symbol name
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMapping {
    pub python_name: String,
    pub rust_name: String,
    pub python_start_line: usize,
    pub python_end_line: usize,
    pub rust_start_line: usize,
    pub rust_end_line: usize,
}

/// Debug information generator
pub struct DebugInfoGenerator {
    source_map: SourceMap,
    current_rust_line: usize,
    debug_level: DebugLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DebugLevel {
    /// No debug information
    None,
    /// Basic line mappings
    Basic,
    /// Full debug information with symbols
    Full,
}

impl DebugInfoGenerator {
    pub fn new(source_file: PathBuf, target_file: PathBuf, debug_level: DebugLevel) -> Self {
        Self {
            source_map: SourceMap {
                source_file,
                target_file,
                mappings: Vec::new(),
                function_map: HashMap::new(),
            },
            current_rust_line: 1,
            debug_level,
        }
    }

    /// Add a source mapping
    pub fn add_mapping(
        &mut self,
        python_line: usize,
        python_column: usize,
        symbol: Option<String>,
    ) {
        if self.debug_level == DebugLevel::None {
            return;
        }

        self.source_map.mappings.push(SourceMapping {
            python_line,
            python_column,
            rust_line: self.current_rust_line,
            rust_column: 0, // Simplified for now
            symbol,
        });
    }

    /// Add a function mapping
    pub fn add_function_mapping(&mut self, func: &HirFunction, rust_start: usize) {
        if self.debug_level == DebugLevel::None {
            return;
        }

        let rust_end = self.current_rust_line;
        self.source_map.function_map.insert(
            func.name.clone(),
            FunctionMapping {
                python_name: func.name.clone(),
                rust_name: func.name.clone(), // Could be mangled
                python_start_line: 0,         // Would need source location
                python_end_line: 0,
                rust_start_line: rust_start,
                rust_end_line: rust_end,
            },
        );
    }

    /// Increment line counter (for tracking generated code)
    pub fn new_line(&mut self) {
        self.current_rust_line += 1;
    }

    /// Get the source map
    pub fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    /// Generate debug annotations for a function
    pub fn generate_function_debug(&self, func: &HirFunction) -> String {
        match self.debug_level {
            DebugLevel::None => String::new(),
            DebugLevel::Basic => format!("// Function: {}\n", func.name),
            DebugLevel::Full => {
                format!(
                    "// Function: {} (Python source)\n// Parameters: {:?}\n// Returns: {:?}\n",
                    func.name,
                    func.params.iter().map(|p| &p.name).collect::<Vec<_>>(),
                    func.ret_type
                )
            }
        }
    }

    /// Generate debug print for a variable
    pub fn generate_debug_print(&self, var_name: &str, var_type: &Type) -> String {
        match self.debug_level {
            DebugLevel::None => String::new(),
            DebugLevel::Basic | DebugLevel::Full => match var_type {
                Type::Int | Type::Float | Type::Bool => {
                    format!("eprintln!(\"DEBUG: {} = {{}}\", {});", var_name, var_name)
                }
                Type::String => {
                    format!("eprintln!(\"DEBUG: {} = {{}}\", {});", var_name, var_name)
                }
                _ => {
                    format!("eprintln!(\"DEBUG: {} = {{:?}}\", {});", var_name, var_name)
                }
            },
        }
    }
}

/// Runtime debugging utilities
pub struct DebugRuntime;

impl DebugRuntime {
    /// Generate a breakpoint macro
    pub fn breakpoint() -> &'static str {
        "depyler_breakpoint!()"
    }

    /// Generate an assertion with debug info
    pub fn debug_assert(condition: &str, message: &str) -> String {
        format!("debug_assert!({}, \"{}\");", condition, message)
    }

    /// Generate a trace point
    pub fn trace_point(location: &str) -> String {
        format!("depyler_trace!(\"{}\");", location)
    }
}

/// Debugger integration helpers
pub struct DebuggerIntegration {
    debugger_type: DebuggerType,
}

#[derive(Debug, Clone, Copy)]
pub enum DebuggerType {
    Gdb,
    Lldb,
    RustGdb,
}

impl DebuggerIntegration {
    pub fn new(debugger_type: DebuggerType) -> Self {
        Self { debugger_type }
    }

    /// Generate debugger initialization script
    pub fn generate_init_script(&self, source_map: &SourceMap) -> String {
        match self.debugger_type {
            DebuggerType::Gdb | DebuggerType::RustGdb => self.generate_gdb_script(source_map),
            DebuggerType::Lldb => self.generate_lldb_script(source_map),
        }
    }

    fn generate_gdb_script(&self, source_map: &SourceMap) -> String {
        let mut script = String::new();
        script.push_str("# GDB initialization script for Depyler debugging\n");
        script.push_str("# Source: ");
        script.push_str(&source_map.source_file.display().to_string());
        script.push_str("\n\n");

        // Add source path
        script.push_str("directory .\n");

        // Add function breakpoints
        for mapping in source_map.function_map.values() {
            script.push_str(&format!("break {}\n", mapping.rust_name));
        }

        // Pretty printers for Rust types
        if matches!(self.debugger_type, DebuggerType::RustGdb) {
            script.push_str("\n# Load Rust pretty printers\n");
            script.push_str("python\nimport gdb\n");
            script.push_str("gdb.execute('set print pretty on')\n");
            script.push_str("end\n");
        }

        script
    }

    fn generate_lldb_script(&self, source_map: &SourceMap) -> String {
        let mut script = String::new();
        script.push_str("# LLDB initialization script for Depyler debugging\n");
        script.push_str("# Source: ");
        script.push_str(&source_map.source_file.display().to_string());
        script.push_str("\n\n");

        // Add source mapping
        script.push_str("settings set target.source-map . .\n");

        // Add function breakpoints
        for mapping in source_map.function_map.values() {
            script.push_str(&format!("breakpoint set --name {}\n", mapping.rust_name));
        }

        script
    }
}

/// Debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub debug_level: DebugLevel,
    pub generate_source_map: bool,
    pub preserve_symbols: bool,
    pub debug_prints: bool,
    pub breakpoints: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            debug_level: DebugLevel::Basic,
            generate_source_map: true,
            preserve_symbols: true,
            debug_prints: false,
            breakpoints: false,
        }
    }
}

/// Helper macros for generated code
pub fn generate_debug_macros() -> String {
    r#"
// Depyler debugging macros
#[macro_export]
macro_rules! depyler_breakpoint {
    () => {
        #[cfg(debug_assertions)]
        {
            eprintln!("BREAKPOINT at {}:{}", file!(), line!());
            // Uncomment to actually break in debugger
            // std::intrinsics::breakpoint();
        }
    };
}

#[macro_export]
macro_rules! depyler_trace {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        eprintln!("[TRACE] {} at {}:{}", $msg, file!(), line!());
    };
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{FunctionProperties, HirParam};
    use smallvec::smallvec;

    // === SourceMap tests ===

    #[test]
    fn test_source_map_new() {
        let sm = SourceMap {
            source_file: PathBuf::from("input.py"),
            target_file: PathBuf::from("output.rs"),
            mappings: vec![],
            function_map: HashMap::new(),
        };
        assert_eq!(sm.source_file, PathBuf::from("input.py"));
        assert_eq!(sm.target_file, PathBuf::from("output.rs"));
        assert!(sm.mappings.is_empty());
    }

    #[test]
    fn test_source_map_clone() {
        let sm = SourceMap {
            source_file: PathBuf::from("a.py"),
            target_file: PathBuf::from("a.rs"),
            mappings: vec![SourceMapping {
                python_line: 1,
                python_column: 0,
                rust_line: 1,
                rust_column: 0,
                symbol: None,
            }],
            function_map: HashMap::new(),
        };
        let cloned = sm.clone();
        assert_eq!(cloned.mappings.len(), 1);
    }

    #[test]
    fn test_source_map_serialize() {
        let sm = SourceMap {
            source_file: PathBuf::from("test.py"),
            target_file: PathBuf::from("test.rs"),
            mappings: vec![],
            function_map: HashMap::new(),
        };
        let json = serde_json::to_string(&sm).unwrap();
        assert!(json.contains("test.py"));
        let deserialized: SourceMap = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.source_file, sm.source_file);
    }

    // === SourceMapping tests ===

    #[test]
    fn test_source_mapping_new() {
        let mapping = SourceMapping {
            python_line: 42,
            python_column: 8,
            rust_line: 100,
            rust_column: 4,
            symbol: Some("my_func".to_string()),
        };
        assert_eq!(mapping.python_line, 42);
        assert_eq!(mapping.rust_line, 100);
        assert_eq!(mapping.symbol, Some("my_func".to_string()));
    }

    #[test]
    fn test_source_mapping_clone() {
        let mapping = SourceMapping {
            python_line: 1,
            python_column: 0,
            rust_line: 5,
            rust_column: 0,
            symbol: None,
        };
        let cloned = mapping.clone();
        assert_eq!(cloned.python_line, mapping.python_line);
    }

    #[test]
    fn test_source_mapping_serialize() {
        let mapping = SourceMapping {
            python_line: 10,
            python_column: 2,
            rust_line: 20,
            rust_column: 4,
            symbol: Some("var".to_string()),
        };
        let json = serde_json::to_string(&mapping).unwrap();
        assert!(json.contains("10"));
        assert!(json.contains("var"));
    }

    // === FunctionMapping tests ===

    #[test]
    fn test_function_mapping_new() {
        let fm = FunctionMapping {
            python_name: "py_func".to_string(),
            rust_name: "rust_func".to_string(),
            python_start_line: 5,
            python_end_line: 15,
            rust_start_line: 10,
            rust_end_line: 30,
        };
        assert_eq!(fm.python_name, "py_func");
        assert_eq!(fm.rust_name, "rust_func");
    }

    #[test]
    fn test_function_mapping_clone() {
        let fm = FunctionMapping {
            python_name: "f".to_string(),
            rust_name: "f".to_string(),
            python_start_line: 1,
            python_end_line: 2,
            rust_start_line: 3,
            rust_end_line: 4,
        };
        let cloned = fm.clone();
        assert_eq!(cloned.python_name, fm.python_name);
    }

    // === DebugLevel tests ===

    #[test]
    fn test_debug_level_none() {
        assert_eq!(DebugLevel::None, DebugLevel::None);
        assert_ne!(DebugLevel::None, DebugLevel::Basic);
    }

    #[test]
    fn test_debug_level_basic() {
        assert_eq!(DebugLevel::Basic, DebugLevel::Basic);
    }

    #[test]
    fn test_debug_level_full() {
        let level = DebugLevel::Full;
        let cloned = level;
        assert_eq!(cloned, DebugLevel::Full);
    }

    #[test]
    fn test_debug_level_serialize() {
        let level = DebugLevel::Full;
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: DebugLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, level);
    }

    // === DebugInfoGenerator tests ===

    #[test]
    fn test_debug_info_generator_new() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("src.py"),
            PathBuf::from("src.rs"),
            DebugLevel::Basic,
        );
        assert_eq!(gen.current_rust_line, 1);
        assert_eq!(gen.debug_level, DebugLevel::Basic);
    }

    #[test]
    fn test_debug_info_generator_add_mapping_none_level() {
        let mut gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::None,
        );
        gen.add_mapping(10, 0, Some("test".to_string()));
        // Should not add mappings when debug level is None
        assert!(gen.source_map().mappings.is_empty());
    }

    #[test]
    fn test_debug_info_generator_new_line() {
        let mut gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Basic,
        );
        assert_eq!(gen.current_rust_line, 1);
        gen.new_line();
        assert_eq!(gen.current_rust_line, 2);
        gen.new_line();
        gen.new_line();
        assert_eq!(gen.current_rust_line, 4);
    }

    #[test]
    fn test_debug_info_generator_source_map_getter() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("test.py"),
            PathBuf::from("test.rs"),
            DebugLevel::Full,
        );
        let sm = gen.source_map();
        assert_eq!(sm.source_file, PathBuf::from("test.py"));
    }

    #[test]
    fn test_generate_function_debug_none_level() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::None,
        );
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        let debug = gen.generate_function_debug(&func);
        assert!(debug.is_empty());
    }

    #[test]
    fn test_generate_function_debug_basic_level() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Basic,
        );
        let func = HirFunction {
            name: "my_func".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        let debug = gen.generate_function_debug(&func);
        assert!(debug.contains("// Function: my_func"));
        assert!(!debug.contains("Parameters")); // Basic doesn't include params
    }

    #[test]
    fn test_generate_function_debug_full_level() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Full,
        );
        let func = HirFunction {
            name: "calc".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        let debug = gen.generate_function_debug(&func);
        assert!(debug.contains("// Function: calc"));
        assert!(debug.contains("Parameters"));
        assert!(debug.contains("Returns"));
    }

    #[test]
    fn test_generate_debug_print_none_level() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::None,
        );
        let debug = gen.generate_debug_print("x", &Type::Int);
        assert!(debug.is_empty());
    }

    #[test]
    fn test_generate_debug_print_float() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Basic,
        );
        let debug = gen.generate_debug_print("value", &Type::Float);
        assert!(debug.contains("eprintln!"));
        assert!(debug.contains("value = {}"));
    }

    #[test]
    fn test_generate_debug_print_bool() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Full,
        );
        let debug = gen.generate_debug_print("flag", &Type::Bool);
        assert!(debug.contains("flag = {}"));
    }

    #[test]
    fn test_generate_debug_print_string() {
        let gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Basic,
        );
        let debug = gen.generate_debug_print("name", &Type::String);
        assert!(debug.contains("name = {}"));
    }

    #[test]
    fn test_add_function_mapping_none_level() {
        let mut gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::None,
        );
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        gen.add_function_mapping(&func, 1);
        assert!(gen.source_map().function_map.is_empty());
    }

    #[test]
    fn test_add_function_mapping_full_level() {
        let mut gen = DebugInfoGenerator::new(
            PathBuf::from("a.py"),
            PathBuf::from("a.rs"),
            DebugLevel::Full,
        );
        gen.new_line(); // line 2
        gen.new_line(); // line 3
        let func = HirFunction {
            name: "my_func".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: depyler_annotations::TranspilationAnnotations::default(),
            docstring: None,
        };
        gen.add_function_mapping(&func, 1);
        assert!(gen.source_map().function_map.contains_key("my_func"));
        let fm = gen.source_map().function_map.get("my_func").unwrap();
        assert_eq!(fm.rust_start_line, 1);
        assert_eq!(fm.rust_end_line, 3);
    }

    // === DebugRuntime tests ===

    #[test]
    fn test_debug_runtime_breakpoint() {
        let bp = DebugRuntime::breakpoint();
        assert_eq!(bp, "depyler_breakpoint!()");
    }

    #[test]
    fn test_debug_runtime_debug_assert() {
        let assertion = DebugRuntime::debug_assert("x > 0", "x must be positive");
        assert!(assertion.contains("debug_assert!"));
        assert!(assertion.contains("x > 0"));
        assert!(assertion.contains("x must be positive"));
    }

    #[test]
    fn test_debug_runtime_trace_point() {
        let trace = DebugRuntime::trace_point("entering loop");
        assert!(trace.contains("depyler_trace!"));
        assert!(trace.contains("entering loop"));
    }

    // === DebuggerType tests ===

    #[test]
    fn test_debugger_type_gdb() {
        let dt = DebuggerType::Gdb;
        let debug = format!("{:?}", dt);
        assert!(debug.contains("Gdb"));
    }

    #[test]
    fn test_debugger_type_lldb() {
        let dt = DebuggerType::Lldb;
        let cloned = dt;
        assert!(matches!(cloned, DebuggerType::Lldb));
    }

    #[test]
    fn test_debugger_type_rust_gdb() {
        let dt = DebuggerType::RustGdb;
        let debug = format!("{:?}", dt);
        assert!(debug.contains("RustGdb"));
    }

    // === DebuggerIntegration tests ===

    #[test]
    fn test_debugger_integration_new() {
        let di = DebuggerIntegration::new(DebuggerType::Gdb);
        assert!(matches!(di.debugger_type, DebuggerType::Gdb));
    }

    #[test]
    fn test_generate_rust_gdb_script() {
        let source_map = SourceMap {
            source_file: PathBuf::from("test.py"),
            target_file: PathBuf::from("test.rs"),
            mappings: vec![],
            function_map: HashMap::new(),
        };
        let di = DebuggerIntegration::new(DebuggerType::RustGdb);
        let script = di.generate_init_script(&source_map);
        assert!(script.contains("GDB initialization"));
        assert!(script.contains("Rust pretty printers"));
        assert!(script.contains("python"));
    }

    // === DebugConfig tests ===

    #[test]
    fn test_debug_config_default() {
        let config = DebugConfig::default();
        assert_eq!(config.debug_level, DebugLevel::Basic);
        assert!(config.generate_source_map);
        assert!(config.preserve_symbols);
        assert!(!config.debug_prints);
        assert!(!config.breakpoints);
    }

    #[test]
    fn test_debug_config_clone() {
        let config = DebugConfig {
            debug_level: DebugLevel::Full,
            generate_source_map: false,
            preserve_symbols: false,
            debug_prints: true,
            breakpoints: true,
        };
        let cloned = config.clone();
        assert_eq!(cloned.debug_level, DebugLevel::Full);
        assert!(cloned.debug_prints);
    }

    #[test]
    fn test_debug_config_serialize() {
        let config = DebugConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("Basic"));
        let deserialized: DebugConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.debug_level, config.debug_level);
    }

    // === generate_debug_macros tests ===

    #[test]
    fn test_generate_debug_macros() {
        let macros = generate_debug_macros();
        assert!(macros.contains("depyler_breakpoint"));
        assert!(macros.contains("depyler_trace"));
        assert!(macros.contains("#[macro_export]"));
        assert!(macros.contains("debug_assertions"));
    }

    // === Original tests ===

    #[test]
    fn test_source_mapping() {
        let mut generator = DebugInfoGenerator::new(
            PathBuf::from("test.py"),
            PathBuf::from("test.rs"),
            DebugLevel::Full,
        );

        generator.add_mapping(10, 0, Some("test_func".to_string()));
        generator.new_line();
        generator.add_mapping(11, 4, None);

        assert_eq!(generator.source_map().mappings.len(), 2);
        assert_eq!(generator.source_map().mappings[0].python_line, 10);
        assert_eq!(generator.source_map().mappings[0].rust_line, 1);
        assert_eq!(generator.source_map().mappings[1].rust_line, 2);
    }

    #[test]
    fn test_debug_print_generation() {
        let generator = DebugInfoGenerator::new(
            PathBuf::from("test.py"),
            PathBuf::from("test.rs"),
            DebugLevel::Full,
        );

        let int_debug = generator.generate_debug_print("x", &Type::Int);
        assert!(int_debug.contains("eprintln!"));
        assert!(int_debug.contains("x = {}"));

        let vec_debug = generator.generate_debug_print("items", &Type::List(Box::new(Type::Int)));
        assert!(vec_debug.contains("{:?}"));
    }

    #[test]
    fn test_debugger_scripts() {
        let source_map = SourceMap {
            source_file: PathBuf::from("test.py"),
            target_file: PathBuf::from("test.rs"),
            mappings: vec![],
            function_map: vec![(
                "test_func".to_string(),
                FunctionMapping {
                    python_name: "test_func".to_string(),
                    rust_name: "test_func".to_string(),
                    python_start_line: 1,
                    python_end_line: 5,
                    rust_start_line: 10,
                    rust_end_line: 20,
                },
            )]
            .into_iter()
            .collect(),
        };

        let gdb_integration = DebuggerIntegration::new(DebuggerType::Gdb);
        let gdb_script = gdb_integration.generate_init_script(&source_map);
        assert!(gdb_script.contains("break test_func"));

        let lldb_integration = DebuggerIntegration::new(DebuggerType::Lldb);
        let lldb_script = lldb_integration.generate_init_script(&source_map);
        assert!(lldb_script.contains("breakpoint set --name test_func"));
    }
}
