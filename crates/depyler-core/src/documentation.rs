//! Documentation generation for transpiled Rust code
//!
//! This module provides tools to generate comprehensive documentation
//! for transpiled Python code, including module docs, function signatures,
//! and usage examples.

use crate::hir::{HirClass, HirFunction, HirMethod, HirModule, Type};
use std::fmt::Write;

/// Documentation generator configuration
#[derive(Debug, Clone)]
pub struct DocConfig {
    /// Include Python source as reference
    pub include_python_source: bool,
    /// Generate usage examples
    pub generate_examples: bool,
    /// Include type migration notes
    pub include_migration_notes: bool,
    /// Generate module-level documentation
    pub generate_module_docs: bool,
    /// Include performance notes
    pub include_performance_notes: bool,
}

impl Default for DocConfig {
    fn default() -> Self {
        Self {
            include_python_source: true,
            generate_examples: true,
            include_migration_notes: true,
            generate_module_docs: true,
            include_performance_notes: false,
        }
    }
}

/// Documentation generator
pub struct DocGenerator {
    config: DocConfig,
    python_source: Option<String>,
}

impl DocGenerator {
    pub fn new(config: DocConfig) -> Self {
        Self {
            config,
            python_source: None,
        }
    }

    pub fn with_python_source(mut self, source: String) -> Self {
        self.python_source = Some(source);
        self
    }

    /// Generate documentation for a HIR module
    pub fn generate_docs(&self, module: &HirModule) -> String {
        let mut doc = String::new();

        // Module header
        self.write_module_header(&mut doc, module);

        // Module-level documentation
        if self.config.generate_module_docs {
            self.write_module_docs(&mut doc, module);
        }

        // Function documentation
        if !module.functions.is_empty() {
            doc.push_str("\n## Functions\n\n");
            for func in &module.functions {
                self.write_function_docs(&mut doc, func);
            }
        }

        // Class documentation
        if !module.classes.is_empty() {
            doc.push_str("\n## Classes\n\n");
            for class in &module.classes {
                self.write_class_docs(&mut doc, class);
            }
        }

        // Migration notes
        if self.config.include_migration_notes && !module.functions.is_empty() {
            doc.push_str("\n## Migration Notes\n\n");
            self.write_migration_notes(&mut doc, module);
        }

        doc
    }

    fn write_module_header(&self, doc: &mut String, _module: &HirModule) {
        doc.push_str("# Generated Rust Documentation\n\n");
        doc.push_str("This documentation was automatically generated from Python source code ");
        doc.push_str("by the Depyler transpiler.\n\n");

        if self.config.include_python_source {
            if let Some(python_source) = &self.python_source {
                doc.push_str("<details>\n");
                doc.push_str("<summary>Original Python Source</summary>\n\n");
                doc.push_str("```python\n");
                doc.push_str(python_source);
                doc.push_str("\n```\n\n");
                doc.push_str("</details>\n\n");
            }
        }
    }

    fn write_module_docs(&self, doc: &mut String, module: &HirModule) {
        doc.push_str("## Module Overview\n\n");

        let func_count = module.functions.len();
        let class_count = module.classes.len();
        let import_count = module.imports.len();

        writeln!(doc, "- **Functions**: {}", func_count).unwrap();
        writeln!(doc, "- **Classes**: {}", class_count).unwrap();
        writeln!(doc, "- **Imports**: {}", import_count).unwrap();
        doc.push('\n');

        if !module.imports.is_empty() {
            doc.push_str("### Dependencies\n\n");
            for import in &module.imports {
                writeln!(doc, "- `{}`", import.module).unwrap();
            }
            doc.push('\n');
        }
    }

    fn write_function_docs(&self, doc: &mut String, func: &HirFunction) {
        writeln!(doc, "### `{}`\n", func.name).unwrap();

        // Function signature
        doc.push_str("```rust\n");
        doc.push_str(&self.format_function_signature(func));
        doc.push_str("\n```\n\n");

        // Docstring
        if let Some(ref docstring) = func.docstring {
            doc.push_str(docstring);
            doc.push_str("\n\n");
        }

        // Parameters
        if !func.params.is_empty() {
            doc.push_str("**Parameters:**\n");
            for param in &func.params {
                writeln!(doc, "- `{}`: {}", param.name, self.format_type(&param.ty)).unwrap();
            }
            doc.push('\n');
        }

        // Return type
        if !matches!(func.ret_type, Type::None) {
            writeln!(doc, "**Returns:** {}\n", self.format_type(&func.ret_type)).unwrap();
        }

        // Properties
        doc.push_str("**Properties:**\n");
        if func.properties.is_pure {
            doc.push_str("- Pure function (no side effects)\n");
        }
        if func.properties.always_terminates {
            doc.push_str("- Always terminates\n");
        }
        if func.properties.panic_free {
            doc.push_str("- Panic-free\n");
        }
        if func.properties.is_async {
            doc.push_str("- Async function\n");
        }
        doc.push('\n');

        // Usage example
        if self.config.generate_examples {
            self.write_function_example(doc, func);
        }

        // Performance notes
        if self.config.include_performance_notes {
            self.write_performance_notes(doc, func);
        }

        doc.push_str("---\n\n");
    }

    fn write_class_docs(&self, doc: &mut String, class: &HirClass) {
        writeln!(doc, "### `{}`\n", class.name).unwrap();

        // Class docstring
        if let Some(ref docstring) = class.docstring {
            doc.push_str(docstring);
            doc.push_str("\n\n");
        }

        // Fields
        if !class.fields.is_empty() {
            doc.push_str("**Fields:**\n");
            for field in &class.fields {
                writeln!(
                    doc,
                    "- `{}`: {}",
                    field.name,
                    self.format_type(&field.field_type)
                )
                .unwrap();
            }
            doc.push('\n');
        }

        // Methods
        if !class.methods.is_empty() {
            doc.push_str("**Methods:**\n\n");
            for method in &class.methods {
                self.write_method_docs(doc, method);
            }
        }

        doc.push_str("---\n\n");
    }

    fn write_method_docs(&self, doc: &mut String, method: &HirMethod) {
        writeln!(doc, "#### `{}`", method.name).unwrap();

        // Method signature
        doc.push_str("```rust\n");
        doc.push_str(&self.format_method_signature(method));
        doc.push_str("\n```\n");

        // Docstring
        if let Some(ref docstring) = method.docstring {
            doc.push_str(docstring);
            doc.push('\n');
        }

        // Method type
        if method.is_static {
            doc.push_str("- **Static method**\n");
        } else if method.is_classmethod {
            doc.push_str("- **Class method**\n");
        } else if method.is_property {
            doc.push_str("- **Property getter**\n");
        }

        doc.push('\n');
    }

    fn write_migration_notes(&self, doc: &mut String, module: &HirModule) {
        doc.push_str("### Python to Rust Migration\n\n");

        doc.push_str("When migrating from Python to the generated Rust code, note:\n\n");
        doc.push_str("1. **Type Safety**: All types are now statically checked at compile time\n");
        doc.push_str("2. **Memory Management**: Rust's ownership system ensures memory safety\n");
        doc.push_str(
            "3. **Error Handling**: Python exceptions are converted to Rust `Result` types\n",
        );
        doc.push_str("4. **Performance**: Expect significant performance improvements\n\n");

        // Specific migration notes for functions
        for func in &module.functions {
            if func
                .params
                .iter()
                .any(|param| matches!(param.ty, Type::List(_)))
            {
                writeln!(
                    doc,
                    "- `{}`: List parameters are passed as slices (`&[T]`) for efficiency",
                    func.name
                )
                .unwrap();
            }
            if matches!(func.ret_type, Type::Optional(_)) {
                writeln!(
                    doc,
                    "- `{}`: Returns `Option<T>` instead of potentially `None`",
                    func.name
                )
                .unwrap();
            }
        }
    }

    fn write_function_example(&self, doc: &mut String, func: &HirFunction) {
        doc.push_str("**Example:**\n\n```rust\n");

        // Generate a simple example
        let args: Vec<String> = func
            .params
            .iter()
            .map(|param| self.example_value_for_type(&param.name, &param.ty))
            .collect();

        if matches!(func.ret_type, Type::None) {
            writeln!(doc, "{}({});", func.name, args.join(", ")).unwrap();
        } else {
            writeln!(doc, "let result = {}({});", func.name, args.join(", ")).unwrap();
        }

        doc.push_str("```\n\n");
    }

    fn write_performance_notes(&self, doc: &mut String, func: &HirFunction) {
        doc.push_str("**Performance Notes:**\n");

        if func.properties.max_stack_depth.is_some() {
            doc.push_str("- May have deep recursion, consider iterative implementation\n");
        }

        if func
            .params
            .iter()
            .any(|param| matches!(param.ty, Type::String))
        {
            doc.push_str("- String parameters use `&str` for zero-copy performance\n");
        }

        doc.push('\n');
    }

    fn format_function_signature(&self, func: &HirFunction) -> String {
        let params: Vec<String> = func
            .params
            .iter()
            .map(|param| format!("{}: {}", param.name, self.format_type(&param.ty)))
            .collect();

        if matches!(func.ret_type, Type::None) {
            format!("fn {}({})", func.name, params.join(", "))
        } else {
            format!(
                "fn {}({}) -> {}",
                func.name,
                params.join(", "),
                self.format_type(&func.ret_type)
            )
        }
    }

    fn format_method_signature(&self, method: &HirMethod) -> String {
        let self_param = if method.is_static { "" } else { "&self, " };

        let params: Vec<String> = method
            .params
            .iter()
            .map(|param| format!("{}: {}", param.name, self.format_type(&param.ty)))
            .collect();

        let all_params = if params.is_empty() {
            self_param.trim_end_matches(", ").to_string()
        } else {
            format!("{}{}", self_param, params.join(", "))
        };

        if matches!(method.ret_type, Type::None) {
            format!("fn {}({})", method.name, all_params)
        } else {
            format!(
                "fn {}({}) -> {}",
                method.name,
                all_params,
                self.format_type(&method.ret_type)
            )
        }
    }

    fn format_type(&self, ty: &Type) -> String {
        format_type_inner(ty)
    }
}

fn format_type_inner(ty: &Type) -> String {
    match ty {
        Type::Unknown => "?".to_string(),
        Type::None => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Int => "i32".to_string(),
        Type::Float => "f64".to_string(),
        Type::String => "&str".to_string(),
        Type::List(inner) => format!("&[{}]", format_type_inner(inner)),
        Type::Dict(key, val) => format!(
            "HashMap<{}, {}>",
            format_type_inner(key),
            format_type_inner(val)
        ),
        Type::Tuple(types) => {
            let inner: Vec<String> = types.iter().map(format_type_inner).collect();
            format!("({})", inner.join(", "))
        }
        Type::Set(inner) => format!("HashSet<{}>", format_type_inner(inner)),
        Type::Optional(inner) => format!("Option<{}>", format_type_inner(inner)),
        Type::Final(inner) => format_type_inner(inner), // Unwrap Final to get the actual type
        Type::Custom(name) => name.clone(),
        Type::Union(types) => {
            let variants: Vec<String> = types.iter().map(format_type_inner).collect();
            format!("Union<{}>", variants.join(", "))
        }
        Type::Generic { base, params } => {
            if params.is_empty() {
                base.clone()
            } else {
                let args_str: Vec<String> = params.iter().map(format_type_inner).collect();
                format!("{}<{}>", base, args_str.join(", "))
            }
        }
        Type::Function { params, ret } => {
            let param_types: Vec<String> = params.iter().map(format_type_inner).collect();
            format!(
                "fn({}) -> {}",
                param_types.join(", "),
                format_type_inner(ret)
            )
        }
        Type::TypeVar(name) => name.clone(),
        Type::Array {
            element_type,
            size: _,
        } => format!("&[{}]", format_type_inner(element_type)),
        Type::UnificationVar(id) => {
            // UnificationVar should never appear in documentation generation
            panic!("BUG: UnificationVar({}) encountered during documentation. Type inference incomplete.", id)
        }
    }
}

impl DocGenerator {
    fn example_value_for_type(&self, name: &str, ty: &Type) -> String {
        match ty {
            Type::Bool => "true".to_string(),
            Type::Int => "42".to_string(),
            Type::Float => "3.14".to_string(),
            Type::String => "\"example\"".to_string(),
            Type::List(_) => "&vec![1, 2, 3]".to_string(),
            Type::Dict(_, _) => "&HashMap::new()".to_string(),
            Type::Optional(_) => "Some(value)".to_string(),
            _ => name.to_string(),
        }
    }

    /// Generate API reference documentation
    pub fn generate_api_reference(&self, module: &HirModule) -> String {
        let mut doc = String::new();

        doc.push_str("# API Reference\n\n");
        doc.push_str("## Table of Contents\n\n");

        // Generate TOC
        if !module.functions.is_empty() {
            doc.push_str("### Functions\n");
            for func in &module.functions {
                writeln!(doc, "- [`{}`](#{})", func.name, func.name.to_lowercase()).unwrap();
            }
            doc.push('\n');
        }

        if !module.classes.is_empty() {
            doc.push_str("### Classes\n");
            for class in &module.classes {
                writeln!(doc, "- [`{}`](#{})", class.name, class.name.to_lowercase()).unwrap();
            }
            doc.push('\n');
        }

        doc.push_str("\n---\n\n");

        // Generate detailed docs
        doc.push_str(&self.generate_docs(module));

        doc
    }

    /// Generate usage guide with examples
    pub fn generate_usage_guide(&self, module: &HirModule) -> String {
        let mut doc = String::new();

        doc.push_str("# Usage Guide\n\n");
        doc.push_str("This guide provides examples of how to use the generated Rust code.\n\n");

        doc.push_str("## Quick Start\n\n");
        doc.push_str("```rust\n");
        doc.push_str("// Import the generated module\n");
        doc.push_str("use generated_module::*;\n\n");

        // Show example usage of main functions
        for func in module.functions.iter().take(3) {
            let args: Vec<String> = func
                .params
                .iter()
                .map(|param| self.example_value_for_type(&param.name, &param.ty))
                .collect();

            writeln!(doc, "// Using {}", func.name).unwrap();
            writeln!(doc, "let result = {}({});", func.name, args.join(", ")).unwrap();
            doc.push('\n');
        }

        doc.push_str("```\n\n");

        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use smallvec::smallvec;

    fn create_test_function(name: &str) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: smallvec![
                HirParam::new("x".to_string(), Type::Int),
                HirParam::new("y".to_string(), Type::Int),
            ],
            ret_type: Type::Int,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: Some("Test function that adds two numbers.".to_string()),
        }
    }

    fn create_test_module() -> HirModule {
        HirModule {
            functions: vec![
                create_test_function("add"),
                create_test_function("multiply"),
            ],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        }
    }

    #[test]
    fn test_basic_doc_generation() {
        let config = DocConfig::default();
        let generator = DocGenerator::new(config);
        let module = create_test_module();

        let docs = generator.generate_docs(&module);

        assert!(docs.contains("# Generated Rust Documentation"));
        assert!(docs.contains("## Functions"));
        assert!(docs.contains("### `add`"));
        assert!(docs.contains("### `multiply`"));
        assert!(docs.contains("Test function that adds two numbers."));
    }

    #[test]
    fn test_with_python_source() {
        let config = DocConfig::default();
        let python_source = "def add(x: int, y: int) -> int:\n    return x + y";
        let generator = DocGenerator::new(config).with_python_source(python_source.to_string());
        let module = create_test_module();

        let docs = generator.generate_docs(&module);

        assert!(docs.contains("Original Python Source"));
        assert!(docs.contains(python_source));
    }

    #[test]
    fn test_function_signature_formatting() {
        let generator = DocGenerator::new(DocConfig::default());
        let func = create_test_function("test");

        let sig = generator.format_function_signature(&func);
        assert_eq!(sig, "fn test(x: i32, y: i32) -> i32");
    }

    #[test]
    fn test_type_formatting() {
        let generator = DocGenerator::new(DocConfig::default());

        assert_eq!(generator.format_type(&Type::Int), "i32");
        assert_eq!(generator.format_type(&Type::String), "&str");
        assert_eq!(
            generator.format_type(&Type::List(Box::new(Type::Int))),
            "&[i32]"
        );
        assert_eq!(
            generator.format_type(&Type::Optional(Box::new(Type::String))),
            "Option<&str>"
        );
    }

    #[test]
    fn test_api_reference_generation() {
        let config = DocConfig::default();
        let generator = DocGenerator::new(config);
        let module = create_test_module();

        let api_ref = generator.generate_api_reference(&module);

        assert!(api_ref.contains("# API Reference"));
        assert!(api_ref.contains("## Table of Contents"));
        assert!(api_ref.contains("### Functions"));
        assert!(api_ref.contains("- [`add`](#add)"));
        assert!(api_ref.contains("- [`multiply`](#multiply)"));
    }

    #[test]
    fn test_usage_guide_generation() {
        let config = DocConfig::default();
        let generator = DocGenerator::new(config);
        let module = create_test_module();

        let guide = generator.generate_usage_guide(&module);

        assert!(guide.contains("# Usage Guide"));
        assert!(guide.contains("## Quick Start"));
        assert!(guide.contains("// Using add"));
        assert!(guide.contains("let result = add(42, 42);"));
    }

    #[test]
    fn test_class_documentation() {
        let config = DocConfig::default();
        let generator = DocGenerator::new(config);

        let class = HirClass {
            name: "TestClass".to_string(),
            fields: vec![HirField {
                name: "value".to_string(),
                field_type: Type::Int,
                default_value: None,
                is_class_var: false,
            }],
            methods: vec![HirMethod {
                name: "get_value".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![],
                is_static: false,
                is_classmethod: false,
                is_property: false,
                is_async: false,
                docstring: Some("Get the value.".to_string()),
            }],
            base_classes: vec![],
            is_dataclass: false,
            docstring: Some("A test class.".to_string()),
            type_params: vec![], // DEPYLER-0739
        };

        let module = HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![class],
            constants: vec![],
        };

        let docs = generator.generate_docs(&module);

        assert!(docs.contains("## Classes"));
        assert!(docs.contains("### `TestClass`"));
        assert!(docs.contains("A test class."));
        assert!(docs.contains("**Fields:**"));
        assert!(docs.contains("- `value`: i32"));
        assert!(docs.contains("**Methods:**"));
        assert!(docs.contains("#### `get_value`"));
        assert!(docs.contains("Get the value."));
    }

    #[test]
    fn test_migration_notes() {
        let config = DocConfig {
            include_migration_notes: true,
            ..Default::default()
        };
        let generator = DocGenerator::new(config);

        let func = HirFunction {
            name: "process_list".to_string(),
            params: smallvec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int))
            ),],
            ret_type: Type::Optional(Box::new(Type::Int)),
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };

        let docs = generator.generate_docs(&module);

        assert!(docs.contains("## Migration Notes"));
        assert!(docs.contains("Python to Rust Migration"));
        assert!(docs.contains("process_list`: List parameters are passed as slices"));
        assert!(docs.contains("process_list`: Returns `Option<T>` instead of potentially `None`"));
    }
}
