//! Fix pattern templates with code transformations.
//!
//! Provides structured fix templates that can be applied to specific error patterns.
//! Each template includes:
//! - Pattern matching rules
//! - Code transformation templates
//! - Contextual hints

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::classifier::ErrorCategory;

/// A code transformation template.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeTransform {
    /// Description of the transformation
    pub description: String,
    /// Pattern to match (regex-like)
    pub match_pattern: String,
    /// Replacement template (with capture groups)
    pub replacement: String,
    /// Example before transformation
    pub example_before: String,
    /// Example after transformation
    pub example_after: String,
}

impl CodeTransform {
    /// Create a new code transformation.
    #[must_use]
    pub fn new(
        description: &str,
        match_pattern: &str,
        replacement: &str,
        example_before: &str,
        example_after: &str,
    ) -> Self {
        Self {
            description: description.to_string(),
            match_pattern: match_pattern.to_string(),
            replacement: replacement.to_string(),
            example_before: example_before.to_string(),
            example_after: example_after.to_string(),
        }
    }
}

/// A comprehensive fix template with multiple strategies.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixTemplate {
    /// Unique identifier for the template
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Error category this template addresses
    pub category: ErrorCategory,
    /// Keywords that trigger this template
    pub trigger_keywords: Vec<String>,
    /// Detailed explanation
    pub explanation: String,
    /// Code transformations (if applicable)
    pub transforms: Vec<CodeTransform>,
    /// General fix suggestions
    pub suggestions: Vec<String>,
    /// Links to documentation
    pub doc_links: Vec<String>,
    /// Priority (higher = more relevant)
    pub priority: u32,
}

impl FixTemplate {
    /// Create a new fix template builder.
    #[must_use]
    pub fn builder(id: &str, name: &str, category: ErrorCategory) -> FixTemplateBuilder {
        FixTemplateBuilder::new(id, name, category)
    }

    /// Check if this template matches an error message.
    #[must_use]
    pub fn matches(&self, error_message: &str) -> bool {
        let lower = error_message.to_lowercase();
        self.trigger_keywords
            .iter()
            .any(|kw| lower.contains(&kw.to_lowercase()))
    }

    /// Calculate match score for an error message.
    #[must_use]
    pub fn match_score(&self, error_message: &str) -> f32 {
        let lower = error_message.to_lowercase();
        let matched = self
            .trigger_keywords
            .iter()
            .filter(|kw| lower.contains(&kw.to_lowercase()))
            .count();

        if self.trigger_keywords.is_empty() {
            return 0.0;
        }

        let keyword_score = matched as f32 / self.trigger_keywords.len() as f32;
        keyword_score * (self.priority as f32 / 100.0)
    }
}

/// Builder for `FixTemplate`.
pub struct FixTemplateBuilder {
    template: FixTemplate,
}

impl FixTemplateBuilder {
    fn new(id: &str, name: &str, category: ErrorCategory) -> Self {
        Self {
            template: FixTemplate {
                id: id.to_string(),
                name: name.to_string(),
                category,
                trigger_keywords: Vec::new(),
                explanation: String::new(),
                transforms: Vec::new(),
                suggestions: Vec::new(),
                doc_links: Vec::new(),
                priority: 50,
            },
        }
    }

    /// Add trigger keywords.
    #[must_use]
    pub fn with_keywords(mut self, keywords: &[&str]) -> Self {
        self.template.trigger_keywords = keywords.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Set explanation.
    #[must_use]
    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.template.explanation = explanation.to_string();
        self
    }

    /// Add a code transformation.
    #[must_use]
    pub fn with_transform(mut self, transform: CodeTransform) -> Self {
        self.template.transforms.push(transform);
        self
    }

    /// Add suggestions.
    #[must_use]
    pub fn with_suggestions(mut self, suggestions: &[&str]) -> Self {
        self.template.suggestions = suggestions.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Add documentation links.
    #[must_use]
    pub fn with_docs(mut self, links: &[&str]) -> Self {
        self.template.doc_links = links.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Set priority.
    #[must_use]
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.template.priority = priority;
        self
    }

    /// Build the template.
    #[must_use]
    pub fn build(self) -> FixTemplate {
        self.template
    }
}

/// Registry of fix templates.
pub struct FixTemplateRegistry {
    /// Templates indexed by category
    templates: HashMap<ErrorCategory, Vec<FixTemplate>>,
}

impl FixTemplateRegistry {
    /// Create a new empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Create a registry with default Rust error templates.
    #[must_use]
    pub fn with_rust_defaults() -> Self {
        let mut registry = Self::new();
        register_type_mismatch_templates(&mut registry);
        register_borrow_checker_templates(&mut registry);
        register_lifetime_templates(&mut registry);
        register_trait_bound_templates(&mut registry);
        register_import_templates(&mut registry);
        register_syntax_templates(&mut registry);
        registry
    }

    /// Register a template.
    pub fn register(&mut self, template: FixTemplate) {
        self.templates
            .entry(template.category)
            .or_default()
            .push(template);
    }

    /// Get templates for a category.
    #[must_use]
    pub fn get_templates(&self, category: ErrorCategory) -> &[FixTemplate] {
        self.templates.get(&category).map_or(&[], |v| v.as_slice())
    }

    /// Find matching templates for an error message.
    #[must_use]
    pub fn find_matches(&self, error_message: &str) -> Vec<&FixTemplate> {
        let mut matches: Vec<(&FixTemplate, f32)> = self
            .templates
            .values()
            .flatten()
            .filter_map(|t| {
                let score = t.match_score(error_message);
                if score > 0.0 {
                    Some((t, score))
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches.into_iter().map(|(t, _)| t).collect()
    }

    /// Find best matching template.
    #[must_use]
    pub fn find_best_match(&self, error_message: &str) -> Option<&FixTemplate> {
        self.find_matches(error_message).into_iter().next()
    }

    /// Get all templates.
    #[must_use]
    pub fn all_templates(&self) -> Vec<&FixTemplate> {
        self.templates.values().flatten().collect()
    }

    /// Total template count.
    #[must_use]
    pub fn template_count(&self) -> usize {
        self.templates.values().map(|v| v.len()).sum()
    }
}

impl Default for FixTemplateRegistry {
    fn default() -> Self {
        Self::with_rust_defaults()
    }
}

// ============================================
// Default Rust Error Templates
// ============================================

fn register_type_mismatch_templates(registry: &mut FixTemplateRegistry) {
    // Integer type conversion
    registry.register(
        FixTemplate::builder("type-int-convert", "Integer Type Conversion", ErrorCategory::TypeMismatch)
            .with_keywords(&["expected", "found", "i32", "i64", "u32", "u64", "isize", "usize"])
            .with_explanation(
                "Rust has strict type checking for integers. Different integer types \
                 cannot be implicitly converted. Use explicit conversion with `as` or `.into()`."
            )
            .with_transform(CodeTransform::new(
                "Convert using `as`",
                r"(\w+)",
                "$1 as TARGET_TYPE",
                "let x: i32 = some_u64;",
                "let x: i32 = some_u64 as i32;",
            ))
            .with_suggestions(&[
                "Use `as` for explicit numeric conversion",
                "Consider using `.try_into()` for fallible conversion",
                "Check if the source value fits in the target type",
            ])
            .with_docs(&["https://doc.rust-lang.org/std/convert/trait.Into.html"])
            .with_priority(80)
            .build(),
    );

    // String type conversion
    registry.register(
        FixTemplate::builder("type-string-convert", "String Type Conversion", ErrorCategory::TypeMismatch)
            .with_keywords(&["expected", "found", "String", "&str", "string", "str"])
            .with_explanation(
                "Rust distinguishes between owned strings (String) and string slices (&str). \
                 Use `.to_string()` to convert &str to String, or `&` / `.as_str()` for the reverse."
            )
            .with_transform(CodeTransform::new(
                "Convert &str to String",
                r#"(".*")"#,
                r#"$1.to_string()"#,
                r#"let s: String = "hello";"#,
                r#"let s: String = "hello".to_string();"#,
            ))
            .with_transform(CodeTransform::new(
                "Convert String to &str",
                r"(\w+)",
                "&$1",
                "let s: &str = my_string;",
                "let s: &str = &my_string;",
            ))
            .with_suggestions(&[
                "Use `.to_string()` to create an owned String from &str",
                "Use `&` or `.as_str()` to borrow a String as &str",
                "Consider if you really need String or if &str would work",
            ])
            .with_priority(85)
            .build(),
    );

    // Option/Result unwrapping
    registry.register(
        FixTemplate::builder("type-option-result", "Option/Result Type Handling", ErrorCategory::TypeMismatch)
            .with_keywords(&["Option", "Result", "Some", "None", "Ok", "Err", "expected", "found"])
            .with_explanation(
                "Option and Result types wrap values. You need to unwrap them to access the inner value. \
                 Prefer `?` operator, `map`, `and_then`, or pattern matching over `unwrap()`."
            )
            .with_transform(CodeTransform::new(
                "Use ? operator",
                r"(\w+)\.unwrap\(\)",
                "$1?",
                "let value = some_option.unwrap();",
                "let value = some_option?;",
            ))
            .with_suggestions(&[
                "Use `?` operator to propagate errors",
                "Use `if let Some(x) = ...` for optional unwrapping",
                "Use `.unwrap_or_default()` for safe defaults",
                "Use `.expect(\"message\")` for better error messages",
            ])
            .with_docs(&[
                "https://doc.rust-lang.org/std/option/",
                "https://doc.rust-lang.org/std/result/",
            ])
            .with_priority(90)
            .build(),
    );
}

fn register_borrow_checker_templates(registry: &mut FixTemplateRegistry) {
    // Cannot move out of borrowed
    registry.register(
        FixTemplate::builder("borrow-move", "Cannot Move Out of Borrowed", ErrorCategory::BorrowChecker)
            .with_keywords(&["cannot move", "borrowed", "move out of"])
            .with_explanation(
                "You're trying to take ownership of a value that is only borrowed. \
                 You need to either clone the value or restructure your code."
            )
            .with_transform(CodeTransform::new(
                "Clone the value",
                r"(\w+)",
                "$1.clone()",
                "let x = borrowed_value;",
                "let x = borrowed_value.clone();",
            ))
            .with_suggestions(&[
                "Clone the value if it implements Clone",
                "Take ownership of the original instead of borrowing",
                "Use a reference instead of owned value",
                "Restructure to avoid needing ownership",
            ])
            .with_priority(85)
            .build(),
    );

    // Value used after move
    registry.register(
        FixTemplate::builder("borrow-use-after-move", "Value Used After Move", ErrorCategory::BorrowChecker)
            .with_keywords(&["value used", "after move", "moved", "borrowed"])
            .with_explanation(
                "Once a value is moved, it can no longer be used. Clone the value before moving, \
                 or restructure your code to avoid the second use."
            )
            .with_suggestions(&[
                "Clone the value before the first use if you need it twice",
                "Pass by reference instead of by value",
                "Use Rc/Arc for shared ownership",
                "Restructure to use the value only once",
            ])
            .with_docs(&["https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"])
            .with_priority(80)
            .build(),
    );

    // Mutable borrow conflict
    registry.register(
        FixTemplate::builder("borrow-mut-conflict", "Mutable Borrow Conflict", ErrorCategory::BorrowChecker)
            .with_keywords(&["mutable", "immutable", "borrow", "cannot borrow", "already borrowed"])
            .with_explanation(
                "Rust prevents having mutable and immutable borrows at the same time. \
                 Restructure your code to separate the borrows or use interior mutability."
            )
            .with_suggestions(&[
                "Separate the mutable and immutable operations",
                "Use interior mutability (Cell, RefCell, Mutex)",
                "Clone data instead of borrowing",
                "Restructure to avoid overlapping borrows",
            ])
            .with_docs(&["https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html"])
            .with_priority(85)
            .build(),
    );
}

fn register_lifetime_templates(registry: &mut FixTemplateRegistry) {
    // Does not live long enough
    registry.register(
        FixTemplate::builder("lifetime-short", "Value Does Not Live Long Enough", ErrorCategory::LifetimeError)
            .with_keywords(&["does not live long enough", "lifetime", "dropped", "borrowed value"])
            .with_explanation(
                "The borrowed value is dropped before the borrow ends. \
                 You need to ensure the value lives as long as the reference."
            )
            .with_suggestions(&[
                "Move the value to a longer-lived scope",
                "Return an owned value instead of a reference",
                "Use 'static lifetime for truly long-lived data",
                "Clone the data to create owned value",
            ])
            .with_priority(80)
            .build(),
    );

    // Explicit lifetime needed
    registry.register(
        FixTemplate::builder("lifetime-explicit", "Explicit Lifetime Annotation Needed", ErrorCategory::LifetimeError)
            .with_keywords(&["lifetime", "annotation", "explicit", "'a", "parameter"])
            .with_explanation(
                "The compiler cannot infer the lifetime relationship. \
                 Add explicit lifetime parameters to clarify."
            )
            .with_transform(CodeTransform::new(
                "Add lifetime parameter",
                r"fn (\w+)\((.*)\) -> &(\w+)",
                "fn $1<'a>($2) -> &'a $3",
                "fn get_str(s: &String) -> &str",
                "fn get_str<'a>(s: &'a String) -> &'a str",
            ))
            .with_suggestions(&[
                "Add lifetime parameter <'a> to function signature",
                "Annotate references with the same lifetime",
                "Consider returning owned data instead",
            ])
            .with_docs(&["https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"])
            .with_priority(75)
            .build(),
    );
}

fn register_trait_bound_templates(registry: &mut FixTemplateRegistry) {
    // Trait not implemented
    registry.register(
        FixTemplate::builder("trait-not-impl", "Trait Not Implemented", ErrorCategory::TraitBound)
            .with_keywords(&["trait", "not implemented", "bound", "doesn't implement"])
            .with_explanation(
                "The type doesn't implement a required trait. \
                 Either implement the trait or use a different approach."
            )
            .with_suggestions(&[
                "Derive the trait if possible: #[derive(Clone, Debug, ...)]",
                "Implement the trait manually",
                "Use a wrapper type that implements the trait",
                "Change the function to not require the trait",
            ])
            .with_priority(80)
            .build(),
    );

    // Missing derive
    registry.register(
        FixTemplate::builder("trait-derive", "Missing Derive Attribute", ErrorCategory::TraitBound)
            .with_keywords(&["Clone", "Copy", "Debug", "Default", "derive", "cannot"])
            .with_explanation(
                "Common traits like Clone, Copy, Debug can be automatically derived. \
                 Add #[derive(...)] to your struct or enum."
            )
            .with_transform(CodeTransform::new(
                "Add derive attribute",
                r"struct (\w+)",
                "#[derive(Clone, Debug)]\nstruct $1",
                "struct MyStruct { ... }",
                "#[derive(Clone, Debug)]\nstruct MyStruct { ... }",
            ))
            .with_suggestions(&[
                "Add #[derive(Clone)] for Clone trait",
                "Add #[derive(Debug)] for Debug trait",
                "Add #[derive(Default)] for Default trait",
                "Combine derives: #[derive(Clone, Debug, Default)]",
            ])
            .with_priority(85)
            .build(),
    );
}

fn register_import_templates(registry: &mut FixTemplateRegistry) {
    // Not found in scope
    registry.register(
        FixTemplate::builder("import-not-found", "Item Not Found in Scope", ErrorCategory::MissingImport)
            .with_keywords(&["not found", "cannot find", "unresolved", "use of undeclared"])
            .with_explanation(
                "The item is not in scope. You need to import it with `use` \
                 or use the fully qualified path."
            )
            .with_suggestions(&[
                "Add `use` statement at the top of the file",
                "Use fully qualified path: std::collections::HashMap",
                "Check if the crate is in Cargo.toml dependencies",
                "Verify the item exists in that module",
            ])
            .with_priority(80)
            .build(),
    );

    // Common std imports
    registry.register(
        FixTemplate::builder("import-std-common", "Common Standard Library Imports", ErrorCategory::MissingImport)
            .with_keywords(&["HashMap", "HashSet", "Vec", "String", "Box", "Rc", "Arc", "Cell", "RefCell"])
            .with_explanation(
                "Common standard library types need to be imported."
            )
            .with_transform(CodeTransform::new(
                "Import HashMap",
                "HashMap",
                "use std::collections::HashMap;\n\nHashMap",
                "let map: HashMap<K, V>",
                "use std::collections::HashMap;\n\nlet map: HashMap<K, V>",
            ))
            .with_suggestions(&[
                "use std::collections::{HashMap, HashSet};",
                "use std::rc::Rc;",
                "use std::sync::Arc;",
                "use std::cell::{Cell, RefCell};",
            ])
            .with_priority(85)
            .build(),
    );
}

fn register_syntax_templates(registry: &mut FixTemplateRegistry) {
    // Missing semicolon
    registry.register(
        FixTemplate::builder("syntax-semicolon", "Missing Semicolon", ErrorCategory::SyntaxError)
            .with_keywords(&["expected", ";", "semicolon", "statement"])
            .with_explanation(
                "Statements in Rust must end with a semicolon (;). \
                 Expressions that are returned don't need semicolons."
            )
            .with_suggestions(&[
                "Add semicolon at end of statement",
                "If this is a return expression, remove the semicolon",
                "Check for unmatched brackets or braces above",
            ])
            .with_priority(90)
            .build(),
    );

    // Unmatched brackets
    registry.register(
        FixTemplate::builder("syntax-brackets", "Unmatched Brackets", ErrorCategory::SyntaxError)
            .with_keywords(&["expected", "}", ")", "]", "unmatched", "unclosed"])
            .with_explanation(
                "Opening and closing brackets must be balanced. \
                 Check for missing or extra brackets."
            )
            .with_suggestions(&[
                "Count opening and closing brackets",
                "Use editor bracket matching feature",
                "Check recent changes for missing brackets",
            ])
            .with_priority(85)
            .build(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===================
    // CodeTransform Tests
    // ===================

    #[test]
    fn test_code_transform_creation() {
        let transform = CodeTransform::new(
            "Test transform",
            r"(\w+)",
            "$1.clone()",
            "let x = value;",
            "let x = value.clone();",
        );

        assert_eq!(transform.description, "Test transform");
        assert!(!transform.match_pattern.is_empty());
        assert!(!transform.example_before.is_empty());
        assert!(!transform.example_after.is_empty());
    }

    // ===================
    // FixTemplate Tests
    // ===================

    #[test]
    fn test_fix_template_builder() {
        let template = FixTemplate::builder("test-id", "Test Template", ErrorCategory::TypeMismatch)
            .with_keywords(&["expected", "found"])
            .with_explanation("Test explanation")
            .with_suggestions(&["Suggestion 1", "Suggestion 2"])
            .with_priority(75)
            .build();

        assert_eq!(template.id, "test-id");
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.category, ErrorCategory::TypeMismatch);
        assert_eq!(template.trigger_keywords.len(), 2);
        assert_eq!(template.suggestions.len(), 2);
        assert_eq!(template.priority, 75);
    }

    #[test]
    fn test_fix_template_matches() {
        let template = FixTemplate::builder("test", "Test", ErrorCategory::TypeMismatch)
            .with_keywords(&["expected", "found"])
            .build();

        assert!(template.matches("error: expected i32, found str"));
        assert!(template.matches("EXPECTED TYPE"));
        assert!(!template.matches("no keywords here"));
    }

    #[test]
    fn test_fix_template_match_score() {
        let template = FixTemplate::builder("test", "Test", ErrorCategory::TypeMismatch)
            .with_keywords(&["expected", "found", "type"])
            .with_priority(100)
            .build();

        let score_all = template.match_score("expected type, found other type");
        let score_some = template.match_score("expected something");
        let score_none = template.match_score("no match");

        assert!(score_all > score_some);
        assert!(score_some > score_none);
        assert!((score_none - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_fix_template_empty_keywords() {
        let template = FixTemplate::builder("test", "Test", ErrorCategory::TypeMismatch)
            .build();

        let score = template.match_score("anything");
        assert!((score - 0.0).abs() < 1e-6);
    }

    // ===================
    // FixTemplateRegistry Tests
    // ===================

    #[test]
    fn test_registry_creation() {
        let registry = FixTemplateRegistry::new();
        assert_eq!(registry.template_count(), 0);
    }

    #[test]
    fn test_registry_with_defaults() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        assert!(registry.template_count() > 0);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = FixTemplateRegistry::new();

        registry.register(
            FixTemplate::builder("test", "Test", ErrorCategory::TypeMismatch)
                .build(),
        );

        assert_eq!(registry.template_count(), 1);
    }

    #[test]
    fn test_registry_get_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let type_templates = registry.get_templates(ErrorCategory::TypeMismatch);
        assert!(!type_templates.is_empty());

        let borrow_templates = registry.get_templates(ErrorCategory::BorrowChecker);
        assert!(!borrow_templates.is_empty());
    }

    #[test]
    fn test_registry_find_matches() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let matches = registry.find_matches("expected i32, found &str");
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_registry_find_best_match() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let best = registry.find_best_match("expected String, found &str");
        assert!(best.is_some());
    }

    #[test]
    fn test_registry_no_match() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let _matches = registry.find_matches("completely unrelated error xyz abc 123");
        // Might find some matches, but with low scores
        // The important thing is it doesn't panic
    }

    #[test]
    fn test_registry_all_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let all = registry.all_templates();
        assert_eq!(all.len(), registry.template_count());
    }

    // ===================
    // Default Template Tests
    // ===================

    #[test]
    fn test_type_mismatch_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        let templates = registry.get_templates(ErrorCategory::TypeMismatch);

        // Should have multiple type mismatch templates
        assert!(templates.len() >= 2);

        // Check we have common ones
        let ids: Vec<&str> = templates.iter().map(|t| t.id.as_str()).collect();
        assert!(ids.contains(&"type-int-convert"));
        assert!(ids.contains(&"type-string-convert"));
    }

    #[test]
    fn test_borrow_checker_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        let templates = registry.get_templates(ErrorCategory::BorrowChecker);

        assert!(!templates.is_empty());
    }

    #[test]
    fn test_lifetime_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        let templates = registry.get_templates(ErrorCategory::LifetimeError);

        assert!(!templates.is_empty());
    }

    #[test]
    fn test_trait_bound_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        let templates = registry.get_templates(ErrorCategory::TraitBound);

        assert!(!templates.is_empty());
    }

    #[test]
    fn test_import_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        let templates = registry.get_templates(ErrorCategory::MissingImport);

        assert!(!templates.is_empty());
    }

    #[test]
    fn test_syntax_templates() {
        let registry = FixTemplateRegistry::with_rust_defaults();
        let templates = registry.get_templates(ErrorCategory::SyntaxError);

        assert!(!templates.is_empty());
    }

    // ===================
    // Integration Tests
    // ===================

    #[test]
    fn test_string_conversion_match() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let matches = registry.find_matches("error: expected `String`, found `&str`");

        // Should find string conversion template
        let has_string_template = matches.iter().any(|t| t.id == "type-string-convert");
        assert!(has_string_template);
    }

    #[test]
    fn test_borrow_move_match() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        let matches = registry.find_matches("cannot move out of borrowed content");

        // Should find borrow-move template
        let has_borrow_template = matches.iter().any(|t| t.id == "borrow-move");
        assert!(has_borrow_template);
    }

    #[test]
    fn test_template_has_suggestions() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        for template in registry.all_templates() {
            // Every template should have at least one suggestion
            assert!(
                !template.suggestions.is_empty(),
                "Template {} has no suggestions",
                template.id
            );
        }
    }

    #[test]
    fn test_template_has_explanation() {
        let registry = FixTemplateRegistry::with_rust_defaults();

        for template in registry.all_templates() {
            // Every template should have an explanation
            assert!(
                !template.explanation.is_empty(),
                "Template {} has no explanation",
                template.id
            );
        }
    }
}
