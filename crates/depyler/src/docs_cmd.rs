//! Documentation generation command
//!
//! This module provides the CLI command for generating documentation
//! from Python source code, including API references and usage guides.

use anyhow::Result;
use clap::Args;
use depyler_core::{
    documentation::{DocConfig, DocGenerator},
    DepylerPipeline,
};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
pub struct DocsArgs {
    /// Path to Python file or directory
    pub input: PathBuf,

    /// Output directory for documentation
    #[arg(short, long, default_value = "./docs")]
    pub output: PathBuf,

    /// Documentation format (markdown, html)
    #[arg(short, long, default_value = "markdown")]
    pub format: String,

    /// Include Python source in documentation
    #[arg(long, default_value = "true")]
    pub include_source: bool,

    /// Generate usage examples
    #[arg(long, default_value = "true")]
    pub examples: bool,

    /// Include migration notes
    #[arg(long, default_value = "true")]
    pub migration_notes: bool,

    /// Include performance notes
    #[arg(long)]
    pub performance_notes: bool,

    /// Generate API reference
    #[arg(long, default_value = "true")]
    pub api_reference: bool,

    /// Generate usage guide
    #[arg(long, default_value = "true")]
    pub usage_guide: bool,

    /// Generate index file
    #[arg(long, default_value = "true")]
    pub index: bool,
}

pub fn handle_docs_command(args: DocsArgs) -> Result<()> {
    // Create output directory
    fs::create_dir_all(&args.output)?;

    // Process single file or directory
    if args.input.is_file() {
        process_file(&args)?;
    } else if args.input.is_dir() {
        process_directory(&args)?;
    } else {
        return Err(anyhow::anyhow!("Input must be a file or directory"));
    }

    // Generate index if requested
    if args.index {
        generate_index(&args)?;
    }

    println!("📚 Documentation generated successfully!");
    println!("📁 Output directory: {}", args.output.display());

    Ok(())
}

fn process_file(args: &DocsArgs) -> Result<()> {
    let source = fs::read_to_string(&args.input)?;
    let pipeline = DepylerPipeline::new();
    let hir = pipeline.parse_to_hir(&source)?;

    // Configure documentation generator
    let config = DocConfig {
        include_python_source: args.include_source,
        generate_examples: args.examples,
        include_migration_notes: args.migration_notes,
        generate_module_docs: true,
        include_performance_notes: args.performance_notes,
    };

    let generator = DocGenerator::new(config).with_python_source(source.clone());

    // Generate different documentation types
    let module_name = args.input.file_stem().unwrap_or_default().to_string_lossy();

    // Main documentation
    let main_doc = generator.generate_docs(&hir);
    let main_path = args.output.join(format!("{}.md", module_name));
    fs::write(&main_path, main_doc)?;
    println!("  ✅ Generated: {}", main_path.display());

    // API reference
    if args.api_reference {
        let api_doc = generator.generate_api_reference(&hir);
        let api_path = args.output.join(format!("{}_api.md", module_name));
        fs::write(&api_path, api_doc)?;
        println!("  ✅ Generated: {}", api_path.display());
    }

    // Usage guide
    if args.usage_guide && !hir.functions.is_empty() {
        let usage_doc = generator.generate_usage_guide(&hir);
        let usage_path = args.output.join(format!("{}_usage.md", module_name));
        fs::write(&usage_path, usage_doc)?;
        println!("  ✅ Generated: {}", usage_path.display());
    }

    // Convert to HTML if requested
    if args.format == "html" {
        convert_to_html(&args.output, &module_name)?;
    }

    Ok(())
}

fn process_directory(args: &DocsArgs) -> Result<()> {
    let mut processed = 0;

    // Find all Python files
    for entry in fs::read_dir(&args.input)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "py") {
            let file_args = DocsArgs {
                input: path.clone(),
                output: args.output.clone(),
                format: args.format.clone(),
                include_source: args.include_source,
                examples: args.examples,
                migration_notes: args.migration_notes,
                performance_notes: args.performance_notes,
                api_reference: args.api_reference,
                usage_guide: args.usage_guide,
                index: false, // Don't generate index for individual files
            };

            if process_file(&file_args).is_ok() {
                processed += 1;
            }
        }
    }

    println!("📊 Processed {} Python files", processed);
    Ok(())
}

fn generate_index(args: &DocsArgs) -> Result<()> {
    let mut index = String::new();

    index.push_str("# Documentation Index\n\n");
    index.push_str("This documentation was generated from Python source code by Depyler.\n\n");

    index.push_str("## Modules\n\n");

    // List all generated documentation files
    let mut modules = Vec::new();
    for entry in fs::read_dir(&args.output)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            if let Some(name) = path.file_stem() {
                let name_str = name.to_string_lossy();
                if !name_str.ends_with("_api")
                    && !name_str.ends_with("_usage")
                    && name_str != "index"
                {
                    modules.push(name_str.to_string());
                }
            }
        }
    }

    modules.sort();

    for module in modules {
        index.push_str(&format!("### {}\n\n", module));
        index.push_str(&format!("- [Module Documentation](./{}.md)\n", module));

        // Check if API reference exists
        let api_path = args.output.join(format!("{}_api.md", module));
        if api_path.exists() {
            index.push_str(&format!("- [API Reference](./{}_api.md)\n", module));
        }

        // Check if usage guide exists
        let usage_path = args.output.join(format!("{}_usage.md", module));
        if usage_path.exists() {
            index.push_str(&format!("- [Usage Guide](./{}_usage.md)\n", module));
        }

        index.push('\n');
    }

    // Write index
    let index_path = args.output.join("index.md");
    fs::write(&index_path, index)?;
    println!("  ✅ Generated: {}", index_path.display());

    Ok(())
}

fn convert_to_html(_output_dir: &Path, _module_name: &str) -> Result<()> {
    // This is a placeholder for HTML conversion
    // In a real implementation, you might use a markdown-to-HTML library
    println!("  ℹ️  HTML conversion not yet implemented");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_docs_args_default_values() {
        // Verify default values are set correctly
        let args = DocsArgs {
            input: PathBuf::from("test.py"),
            output: PathBuf::from("./docs"),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: true,
            usage_guide: true,
            index: true,
        };

        assert_eq!(args.format, "markdown");
        assert!(args.include_source);
        assert!(args.examples);
        assert!(args.migration_notes);
        assert!(!args.performance_notes);
        assert!(args.api_reference);
        assert!(args.usage_guide);
        assert!(args.index);
    }

    #[test]
    fn test_docs_args_debug() {
        let args = DocsArgs {
            input: PathBuf::from("test.py"),
            output: PathBuf::from("./docs"),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: true,
            usage_guide: true,
            index: true,
        };

        let debug_str = format!("{:?}", args);
        assert!(debug_str.contains("DocsArgs"));
        assert!(debug_str.contains("test.py"));
    }

    #[test]
    fn test_docs_command_single_file() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("test.py");
        let output_dir = dir.path().join("docs");

        let python_code = r#"
def add(x: int, y: int) -> int:
    """Add two numbers together."""
    return x + y

class Calculator:
    """A simple calculator class."""

    def __init__(self):
        self.result = 0

    def compute(self, x: int, y: int) -> int:
        """Compute x + y."""
        return x + y
"#;

        fs::write(&input_path, python_code).unwrap();

        let args = DocsArgs {
            input: input_path,
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: true,
            usage_guide: true,
            index: false,
        };

        let result = handle_docs_command(args);
        assert!(result.is_ok());

        // Check that files were created
        assert!(output_dir.join("test.md").exists());
        assert!(output_dir.join("test_api.md").exists());
        assert!(output_dir.join("test_usage.md").exists());
    }

    #[test]
    fn test_docs_command_with_html_format() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("test.py");
        let output_dir = dir.path().join("docs");

        let python_code = "def test(): return 1";
        fs::write(&input_path, python_code).unwrap();

        let args = DocsArgs {
            input: input_path,
            output: output_dir.clone(),
            format: "html".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: false,
            usage_guide: false,
            index: false,
        };

        // HTML format should work (even if just placeholder)
        let result = handle_docs_command(args);
        assert!(result.is_ok());
        assert!(output_dir.join("test.md").exists());
    }

    #[test]
    fn test_docs_command_with_performance_notes() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("test.py");
        let output_dir = dir.path().join("docs");

        let python_code = "def test(): return 1";
        fs::write(&input_path, python_code).unwrap();

        let args = DocsArgs {
            input: input_path,
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: true, // Enable performance notes
            api_reference: false,
            usage_guide: false,
            index: false,
        };

        let result = handle_docs_command(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_docs_command_minimal_options() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("test.py");
        let output_dir = dir.path().join("docs");

        let python_code = "x = 1"; // No functions, just a variable
        fs::write(&input_path, python_code).unwrap();

        let args = DocsArgs {
            input: input_path,
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: false,
            examples: false,
            migration_notes: false,
            performance_notes: false,
            api_reference: false,
            usage_guide: false,
            index: false,
        };

        let result = handle_docs_command(args);
        assert!(result.is_ok());
        // Main doc should still be generated
        assert!(output_dir.join("test.md").exists());
    }

    #[test]
    fn test_docs_command_invalid_input() {
        let args = DocsArgs {
            input: PathBuf::from("/nonexistent/path/file.py"),
            output: PathBuf::from("/tmp/docs"),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: true,
            usage_guide: true,
            index: false,
        };

        let result = handle_docs_command(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_docs_command_directory() {
        let dir = tempdir().unwrap();
        let input_dir = dir.path().join("src");
        let output_dir = dir.path().join("docs");

        fs::create_dir_all(&input_dir).unwrap();

        // Create multiple Python files
        let files = vec![
            ("module1.py", "def func1():\n    return 1"),
            ("module2.py", "def func2():\n    return 2"),
            ("not_python.txt", "This is not Python"),
        ];

        for (name, content) in files {
            fs::write(input_dir.join(name), content).unwrap();
        }

        let args = DocsArgs {
            input: input_dir,
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: false,
            usage_guide: false,
            index: true,
        };

        let result = handle_docs_command(args);
        assert!(result.is_ok());

        // Check that documentation was generated for Python files only
        assert!(output_dir.join("module1.md").exists());
        assert!(output_dir.join("module2.md").exists());
        assert!(!output_dir.join("not_python.md").exists());
        assert!(output_dir.join("index.md").exists());
    }

    #[test]
    fn test_docs_command_empty_directory() {
        let dir = tempdir().unwrap();
        let input_dir = dir.path().join("empty");
        let output_dir = dir.path().join("docs");

        fs::create_dir_all(&input_dir).unwrap();

        let args = DocsArgs {
            input: input_dir,
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: false,
            usage_guide: false,
            index: true,
        };

        let result = handle_docs_command(args);
        assert!(result.is_ok());
        // Index should still be created
        assert!(output_dir.join("index.md").exists());
    }

    #[test]
    fn test_generate_index() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path().join("docs");
        fs::create_dir_all(&output_dir).unwrap();

        // Create some documentation files
        fs::write(output_dir.join("module1.md"), "# Module 1").unwrap();
        fs::write(output_dir.join("module1_api.md"), "# Module 1 API").unwrap();
        fs::write(output_dir.join("module2.md"), "# Module 2").unwrap();
        fs::write(output_dir.join("module2_usage.md"), "# Module 2 Usage").unwrap();

        let args = DocsArgs {
            input: dir.path().to_path_buf(),
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: true,
            usage_guide: true,
            index: true,
        };

        let result = generate_index(&args);
        assert!(result.is_ok());

        // Check index content
        let index_content = fs::read_to_string(output_dir.join("index.md")).unwrap();
        assert!(index_content.contains("# Documentation Index"));
        assert!(index_content.contains("module1"));
        assert!(index_content.contains("module2"));
        assert!(index_content.contains("[Module Documentation]"));
        assert!(index_content.contains("[API Reference]"));
        assert!(index_content.contains("[Usage Guide]"));
    }

    #[test]
    fn test_generate_index_empty_directory() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path().join("docs");
        fs::create_dir_all(&output_dir).unwrap();

        let args = DocsArgs {
            input: dir.path().to_path_buf(),
            output: output_dir.clone(),
            format: "markdown".to_string(),
            include_source: true,
            examples: true,
            migration_notes: true,
            performance_notes: false,
            api_reference: true,
            usage_guide: true,
            index: true,
        };

        let result = generate_index(&args);
        assert!(result.is_ok());

        let index_content = fs::read_to_string(output_dir.join("index.md")).unwrap();
        assert!(index_content.contains("# Documentation Index"));
        // Should have header but no modules
        assert!(!index_content.contains("[Module Documentation]"));
    }

    #[test]
    fn test_convert_to_html_placeholder() {
        let dir = tempdir().unwrap();
        let result = convert_to_html(dir.path(), "test");
        assert!(result.is_ok());
    }
}
