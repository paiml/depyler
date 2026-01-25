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
        // DEPYLER-1309: Bootstrap top 50 transpiler-specific patterns
        register_transpiler_patterns(&mut registry);
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

        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

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
        FixTemplate::builder(
            "type-int-convert",
            "Integer Type Conversion",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&[
            "expected", "found", "i32", "i64", "u32", "u64", "isize", "usize",
        ])
        .with_explanation(
            "Rust has strict type checking for integers. Different integer types \
                 cannot be implicitly converted. Use explicit conversion with `as` or `.into()`.",
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
        FixTemplate::builder(
            "borrow-move",
            "Cannot Move Out of Borrowed",
            ErrorCategory::BorrowChecker,
        )
        .with_keywords(&["cannot move", "borrowed", "move out of"])
        .with_explanation(
            "You're trying to take ownership of a value that is only borrowed. \
                 You need to either clone the value or restructure your code.",
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
        FixTemplate::builder(
            "borrow-use-after-move",
            "Value Used After Move",
            ErrorCategory::BorrowChecker,
        )
        .with_keywords(&["value used", "after move", "moved", "borrowed"])
        .with_explanation(
            "Once a value is moved, it can no longer be used. Clone the value before moving, \
                 or restructure your code to avoid the second use.",
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
        FixTemplate::builder(
            "borrow-mut-conflict",
            "Mutable Borrow Conflict",
            ErrorCategory::BorrowChecker,
        )
        .with_keywords(&[
            "mutable",
            "immutable",
            "borrow",
            "cannot borrow",
            "already borrowed",
        ])
        .with_explanation(
            "Rust prevents having mutable and immutable borrows at the same time. \
                 Restructure your code to separate the borrows or use interior mutability.",
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
        FixTemplate::builder(
            "lifetime-short",
            "Value Does Not Live Long Enough",
            ErrorCategory::LifetimeError,
        )
        .with_keywords(&[
            "does not live long enough",
            "lifetime",
            "dropped",
            "borrowed value",
        ])
        .with_explanation(
            "The borrowed value is dropped before the borrow ends. \
                 You need to ensure the value lives as long as the reference.",
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
        FixTemplate::builder(
            "lifetime-explicit",
            "Explicit Lifetime Annotation Needed",
            ErrorCategory::LifetimeError,
        )
        .with_keywords(&["lifetime", "annotation", "explicit", "'a", "parameter"])
        .with_explanation(
            "The compiler cannot infer the lifetime relationship. \
                 Add explicit lifetime parameters to clarify.",
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
        FixTemplate::builder(
            "trait-not-impl",
            "Trait Not Implemented",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["trait", "not implemented", "bound", "doesn't implement"])
        .with_explanation(
            "The type doesn't implement a required trait. \
                 Either implement the trait or use a different approach.",
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
        FixTemplate::builder(
            "trait-derive",
            "Missing Derive Attribute",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["Clone", "Copy", "Debug", "Default", "derive", "cannot"])
        .with_explanation(
            "Common traits like Clone, Copy, Debug can be automatically derived. \
                 Add #[derive(...)] to your struct or enum.",
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
        FixTemplate::builder(
            "import-not-found",
            "Item Not Found in Scope",
            ErrorCategory::MissingImport,
        )
        .with_keywords(&[
            "not found",
            "cannot find",
            "unresolved",
            "use of undeclared",
        ])
        .with_explanation(
            "The item is not in scope. You need to import it with `use` \
                 or use the fully qualified path.",
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
        FixTemplate::builder(
            "import-std-common",
            "Common Standard Library Imports",
            ErrorCategory::MissingImport,
        )
        .with_keywords(&[
            "HashMap", "HashSet", "Vec", "String", "Box", "Rc", "Arc", "Cell", "RefCell",
        ])
        .with_explanation("Common standard library types need to be imported.")
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
        FixTemplate::builder(
            "syntax-semicolon",
            "Missing Semicolon",
            ErrorCategory::SyntaxError,
        )
        .with_keywords(&["expected", ";", "semicolon", "statement"])
        .with_explanation(
            "Statements in Rust must end with a semicolon (;). \
                 Expressions that are returned don't need semicolons.",
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
        FixTemplate::builder(
            "syntax-brackets",
            "Unmatched Brackets",
            ErrorCategory::SyntaxError,
        )
        .with_keywords(&["expected", "}", ")", "]", "unmatched", "unclosed"])
        .with_explanation(
            "Opening and closing brackets must be balanced. \
                 Check for missing or extra brackets.",
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

// ============================================
// DEPYLER-1309: Transpiler-Specific Patterns
// Bootstrap the top 50 error patterns from corpus analysis
// ============================================

fn register_transpiler_patterns(registry: &mut FixTemplateRegistry) {
    // === E0599: No method found patterns ===

    // Pattern 1: datetime methods on tuples (hour, minute, second)
    registry.register(
        FixTemplate::builder(
            "e0599-datetime-tuple",
            "Datetime Methods on Tuple",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "hour", "minute", "second", "tuple"])
        .with_explanation(
            "Python datetime.time objects have hour/minute/second attributes. \
             The transpiler incorrectly inferred a tuple type instead of a time type.",
        )
        .with_suggestions(&[
            "Change type annotation from tuple to chrono::NaiveTime",
            "Use tuple destructuring: let (hour, minute, second) = time;",
            "Access tuple elements: time.0, time.1, time.2",
        ])
        .with_priority(90)
        .build(),
    );

    // Pattern 2: year/month/day on tuples
    registry.register(
        FixTemplate::builder(
            "e0599-date-tuple",
            "Date Methods on Tuple",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "year", "month", "day", "tuple"])
        .with_explanation(
            "Python datetime.date objects have year/month/day attributes. \
             The transpiler incorrectly inferred a tuple type instead of a date type.",
        )
        .with_suggestions(&[
            "Change type annotation from tuple to chrono::NaiveDate",
            "Use tuple destructuring: let (year, month, day) = date;",
            "Access tuple elements: date.0, date.1, date.2",
        ])
        .with_priority(90)
        .build(),
    );

    // Pattern 3: as_i64 doesn't exist (should be cast)
    registry.register(
        FixTemplate::builder(
            "e0599-as-i64-cast",
            "as_i64 Method Missing",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "as_i64", "i32", "i64"])
        .with_explanation(
            "Rust doesn't have an as_i64() method. Use the `as` keyword for numeric casts.",
        )
        .with_transform(CodeTransform::new(
            "Replace .as_i64() with cast",
            r"\.as_i64\(\)",
            " as i64",
            "value.as_i64()",
            "value as i64",
        ))
        .with_suggestions(&[
            "Use `value as i64` instead of `value.as_i64()`",
            "For Option<i64>, use `.map(|v| v as i64)`",
        ])
        .with_priority(95)
        .build(),
    );

    // Pattern 4: is_some on Vec (should be Option)
    registry.register(
        FixTemplate::builder(
            "e0599-is-some-vec",
            "is_some on Vec",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "is_some", "Vec"])
        .with_explanation(
            "is_some() is an Option method, not a Vec method. \
             The transpiler incorrectly inferred Vec instead of Option.",
        )
        .with_suggestions(&[
            "Change Vec<T> to Option<T> if checking for presence",
            "Use !vec.is_empty() to check if Vec has elements",
            "Wrap the Vec in Option if it's optional: Option<Vec<T>>",
        ])
        .with_priority(90)
        .build(),
    );

    // Pattern 5: display on String (should be Path)
    registry.register(
        FixTemplate::builder(
            "e0599-display-string",
            "display on String",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "display", "String"])
        .with_explanation(
            "display() is a Path method, not a String method. \
             For String formatting, just use the String directly or &str.",
        )
        .with_suggestions(&[
            "Remove .display() - String implements Display directly",
            "If this should be a Path, change the type to PathBuf",
            "Use std::path::Path::new(&string).display()",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 6: ok on ReadDir
    registry.register(
        FixTemplate::builder(
            "e0599-ok-readdir",
            "ok() on ReadDir",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "ok", "ReadDir"])
        .with_explanation(
            "ReadDir doesn't have an ok() method. It's already a Result that was unwrapped. \
             The ok() call is redundant.",
        )
        .with_suggestions(&[
            "Remove the .ok() call - the iterator is already available",
            "If the fs::read_dir() result needs error handling, use ? or match",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 7: duration_since on datetime types
    registry.register(
        FixTemplate::builder(
            "e0599-duration-since",
            "duration_since Missing",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "duration_since", "DateTime"])
        .with_explanation(
            "Python's timedelta arithmetic uses the - operator. \
             In Rust, use signed_duration_since() or subtract with chrono types.",
        )
        .with_suggestions(&[
            "Use .signed_duration_since(other) for chrono types",
            "Use datetime1 - datetime2 for Duration",
            "Check if the type is std::time::Instant vs chrono::DateTime",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 8: toordinal on date types
    registry.register(
        FixTemplate::builder(
            "e0599-toordinal",
            "toordinal Missing",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "toordinal"])
        .with_explanation(
            "Python's date.toordinal() returns days since year 1. \
             In Rust/chrono, use .num_days_from_ce() for similar functionality.",
        )
        .with_suggestions(&[
            "Use .num_days_from_ce() for chrono NaiveDate",
            "Calculate manually: (date - NaiveDate::from_ymd(1, 1, 1)).num_days()",
        ])
        .with_priority(80)
        .build(),
    );

    // Pattern 9: replace on date types
    registry.register(
        FixTemplate::builder(
            "e0599-date-replace",
            "date replace Missing",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "replace", "Date"])
        .with_explanation(
            "Python's date.replace() creates a new date with some fields changed. \
             In Rust/chrono, use .with_year(), .with_month(), .with_day().",
        )
        .with_suggestions(&[
            "Use .with_year(2024) to change year",
            "Use .with_month(6) to change month",
            "Use .with_day(15) to change day",
            "Chain multiple: date.with_year(2024).with_month(6)",
        ])
        .with_priority(80)
        .build(),
    );

    // === E0308: Type mismatch patterns ===

    // Pattern 10: Vec to &[T] conversion
    registry.register(
        FixTemplate::builder(
            "e0308-vec-slice",
            "Vec to Slice Conversion",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["expected", "&[", "found", "Vec"])
        .with_explanation(
            "Function expects a slice &[T] but received Vec<T>. \
             Use &vec or vec.as_slice() to convert.",
        )
        .with_transform(CodeTransform::new(
            "Convert Vec to slice",
            r"(\w+)",
            "&$1",
            "function(my_vec)",
            "function(&my_vec)",
        ))
        .with_suggestions(&[
            "Use &vec to borrow as slice",
            "Use vec.as_slice() for explicit conversion",
        ])
        .with_priority(90)
        .build(),
    );

    // Pattern 11: Wrong argument count
    registry.register(
        FixTemplate::builder(
            "e0308-arg-count",
            "Arguments to Function Incorrect",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["arguments to this function are incorrect"])
        .with_explanation(
            "The function is being called with wrong number or types of arguments. \
             Check the function signature and adjust the call.",
        )
        .with_suggestions(&[
            "Check the function signature for expected argument types",
            "Verify argument order matches the function definition",
            "Add missing arguments or remove extra ones",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 12: DepylerValue type inference
    registry.register(
        FixTemplate::builder(
            "e0308-depyler-value",
            "DepylerValue Type Inference",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["DepylerValue", "expected", "found"])
        .with_explanation(
            "DepylerValue is a dynamic type wrapper. The transpiler needs better type \
             inference to determine the concrete type at compile time.",
        )
        .with_suggestions(&[
            "Add type annotation to help inference",
            "Use .to_i64(), .to_f64(), .to_string() to extract concrete type",
            "Check Python source for type hints",
        ])
        .with_priority(85)
        .build(),
    );

    // === E0277: Trait not implemented patterns ===

    // Pattern 13: AsRef<OsStr> not implemented
    registry.register(
        FixTemplate::builder(
            "e0277-asref-osstr",
            "AsRef<OsStr> Not Implemented",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["AsRef<OsStr>", "not implemented", "Value"])
        .with_explanation(
            "subprocess/Command functions expect string-like types that implement AsRef<OsStr>. \
             serde_json::Value doesn't implement this.",
        )
        .with_suggestions(&[
            "Convert to String first: value.as_str().unwrap().to_string()",
            "Change type inference to Vec<String> instead of Vec<Value>",
            "Use explicit type annotation: let args: Vec<String>",
        ])
        .with_priority(90)
        .build(),
    );

    // Pattern 14: Iterator trait missing
    registry.register(
        FixTemplate::builder(
            "e0277-iterator",
            "Iterator Trait Not Implemented",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["Iterator", "not implemented", "for loop"])
        .with_explanation(
            "The for loop requires an Iterator. The type doesn't implement IntoIterator.",
        )
        .with_suggestions(&[
            "Use .iter() for borrowed iteration",
            "Use .into_iter() for consuming iteration",
            "Check if the type should be a collection",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 15: Display trait missing
    registry.register(
        FixTemplate::builder(
            "e0277-display",
            "Display Trait Not Implemented",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["Display", "not implemented", "format", "println"])
        .with_explanation(
            "The type doesn't implement Display for formatting with {} in format strings.",
        )
        .with_suggestions(&[
            "Use {:?} for Debug formatting instead",
            "Implement Display trait for the type",
            "Convert to String first",
        ])
        .with_priority(80)
        .build(),
    );

    // === E0609: No field on type patterns ===

    // Pattern 16: No field on tuple
    registry.register(
        FixTemplate::builder(
            "e0609-tuple-field",
            "No Named Field on Tuple",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no field", "tuple", "did you mean"])
        .with_explanation(
            "Tuples use numeric indices (0, 1, 2), not named fields. \
             For named fields, use a struct.",
        )
        .with_suggestions(&[
            "Use tuple.0, tuple.1, etc. for positional access",
            "Destructure: let (a, b, c) = tuple;",
            "Consider using a struct with named fields",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 17: No field on reference
    registry.register(
        FixTemplate::builder(
            "e0609-ref-field",
            "Field Access on Reference",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no field", "&", "reference"])
        .with_explanation(
            "Dereferencing happens automatically for method calls but not for field access. \
             The type might be a reference to something without that field.",
        )
        .with_suggestions(&[
            "Dereference explicitly: (*ref).field",
            "Check if the base type has the field",
            "The reference might be to a different type than expected",
        ])
        .with_priority(80)
        .build(),
    );

    // === E0282: Type annotation needed patterns ===

    // Pattern 18: Cannot infer type
    registry.register(
        FixTemplate::builder(
            "e0282-cannot-infer",
            "Cannot Infer Type",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["cannot infer type", "type annotation needed"])
        .with_explanation(
            "Rust's type inference couldn't determine the type. Add explicit annotations.",
        )
        .with_suggestions(&[
            "Add type annotation: let x: i32 = ...;",
            "Use turbofish syntax: collect::<Vec<_>>()",
            "Provide more context with explicit types",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 19: Collection type ambiguous
    registry.register(
        FixTemplate::builder(
            "e0282-collect",
            "Collect Type Ambiguous",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["collect", "cannot infer", "type"])
        .with_explanation(
            "The collect() method can produce many container types. Specify which one.",
        )
        .with_transform(CodeTransform::new(
            "Add turbofish to collect",
            r"\.collect\(\)",
            ".collect::<Vec<_>>()",
            "iter.map(f).collect()",
            "iter.map(f).collect::<Vec<_>>()",
        ))
        .with_suggestions(&[
            "Use .collect::<Vec<_>>() for vectors",
            "Use .collect::<HashMap<_, _>>() for maps",
            "Use .collect::<HashSet<_>>() for sets",
        ])
        .with_priority(90)
        .build(),
    );

    // === E0061: Wrong argument count patterns ===

    // Pattern 20: This function takes N arguments
    registry.register(
        FixTemplate::builder(
            "e0061-arg-count",
            "Wrong Number of Arguments",
            ErrorCategory::SyntaxError,
        )
        .with_keywords(&["this function takes", "arguments", "supplied"])
        .with_explanation("The function was called with the wrong number of arguments.")
        .with_suggestions(&[
            "Check the function definition for required arguments",
            "Some Python default arguments may need explicit values in Rust",
            "Optional parameters might need Option<T> wrapping",
        ])
        .with_priority(85)
        .build(),
    );

    // === E0605: Invalid cast patterns ===

    // Pattern 21: Non-primitive cast
    registry.register(
        FixTemplate::builder(
            "e0605-non-primitive",
            "Non-Primitive Cast",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["non-primitive cast", "as", "cannot cast"])
        .with_explanation(
            "The `as` keyword only works for primitive types. Use From/Into traits \
             or constructor methods for complex types.",
        )
        .with_suggestions(&[
            "Use .into() for types implementing From/Into",
            "Use Type::from(value) explicitly",
            "Implement From trait for custom conversions",
        ])
        .with_priority(80)
        .build(),
    );

    // === E0369: Binary operation not applicable patterns ===

    // Pattern 22: Cannot apply binary operation
    registry.register(
        FixTemplate::builder(
            "e0369-binary-op",
            "Binary Operation Not Applicable",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["cannot be applied to type", "binary operation"])
        .with_explanation(
            "The binary operator (+, -, *, /, etc.) isn't defined for these types. \
             Implement the corresponding trait or convert types.",
        )
        .with_suggestions(&[
            "Convert types to match (both i32, both f64, etc.)",
            "Use PyOps traits for Python-style operations",
            "Implement Add/Sub/Mul/Div traits for custom types",
        ])
        .with_priority(80)
        .build(),
    );

    // === E0600: Unary operation not applicable patterns ===

    // Pattern 23: Cannot apply unary operator
    registry.register(
        FixTemplate::builder(
            "e0600-unary-op",
            "Unary Operation Not Applicable",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["cannot apply unary operator", "-", "!"])
        .with_explanation(
            "The unary operator (-, !) isn't defined for this type. \
             Check the type and implement Neg or Not traits if needed.",
        )
        .with_suggestions(&[
            "Check if the type is correct for negation",
            "Use explicit conversion before applying operator",
            "Implement Neg/Not traits for custom types",
        ])
        .with_priority(75)
        .build(),
    );

    // === E0425: Unresolved name patterns ===

    // Pattern 24: Not found in this scope
    registry.register(
        FixTemplate::builder(
            "e0425-not-found",
            "Name Not Found in Scope",
            ErrorCategory::MissingImport,
        )
        .with_keywords(&["cannot find value", "in this scope"])
        .with_explanation("The variable or function is not defined or not in scope.")
        .with_suggestions(&[
            "Check for typos in the name",
            "Import the item with `use`",
            "Ensure the variable is defined before use",
        ])
        .with_priority(85)
        .build(),
    );

    // === Additional common transpiler patterns ===

    // Pattern 25: PyOps trait missing
    registry.register(
        FixTemplate::builder(
            "e0599-pyops-missing",
            "PyOps Trait Method Missing",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["no method named", "py_add", "py_sub", "py_mul", "py_div"])
        .with_explanation(
            "Python-style operations require PyOps traits. The transpiler should \
             generate these trait implementations inline.",
        )
        .with_suggestions(&[
            "Check if PyOps traits are included in generated code",
            "Verify the types implement the required PyOps trait",
            "For Vec operations, ensure element-wise impls exist",
        ])
        .with_priority(95)
        .build(),
    );

    // Pattern 26: serde_json::Value method missing
    registry.register(
        FixTemplate::builder(
            "e0599-json-value",
            "serde_json::Value Method Missing",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["no method named", "Value", "serde_json"])
        .with_explanation(
            "serde_json::Value is a dynamic JSON type. Python dict methods don't \
             map directly. Use as_object(), as_array(), etc.",
        )
        .with_suggestions(&[
            "Use .as_object() to get Option<&Map>",
            "Use .as_array() to get Option<&Vec>",
            "Use .as_str(), .as_i64(), .as_f64() for primitives",
            "Use .get(key) for dictionary-like access",
        ])
        .with_priority(90)
        .build(),
    );

    // Pattern 27: std::time vs chrono confusion
    registry.register(
        FixTemplate::builder(
            "e0308-time-types",
            "Time Type Mismatch",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["std::time", "chrono", "Duration", "Instant"])
        .with_explanation(
            "Rust has two time libraries: std::time and chrono. They're not interchangeable. \
             Python datetime maps better to chrono types.",
        )
        .with_suggestions(&[
            "Use chrono for date/time: NaiveDate, NaiveTime, DateTime",
            "Use std::time for durations: Duration, Instant",
            "Convert: chrono::Duration::to_std() / from_std()",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 28: Optional argument handling
    registry.register(
        FixTemplate::builder(
            "e0308-optional-arg",
            "Optional Argument Type Mismatch",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["Option", "expected", "found", "argument"])
        .with_explanation(
            "Python's optional parameters with defaults need special handling. \
             Use Option<T> and provide defaults with .unwrap_or().",
        )
        .with_suggestions(&[
            "Wrap optional params in Option<T>",
            "Use .unwrap_or(default) for defaults",
            "Consider builder pattern for many optional args",
        ])
        .with_priority(80)
        .build(),
    );

    // Pattern 29: Closure type inference
    registry.register(
        FixTemplate::builder(
            "e0282-closure",
            "Closure Type Cannot Be Inferred",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["closure", "cannot infer", "type"])
        .with_explanation(
            "Rust closures need type annotations when the type can't be inferred \
             from usage context.",
        )
        .with_suggestions(&[
            "Add parameter types: |x: i32| x + 1",
            "Add return type: |x| -> i32 { x + 1 }",
            "Use the closure immediately to provide context",
        ])
        .with_priority(75)
        .build(),
    );

    // Pattern 30: String vs &str in HashMap keys
    registry.register(
        FixTemplate::builder(
            "e0308-hashmap-key",
            "HashMap Key Type Mismatch",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["HashMap", "expected", "&str", "String", "key"])
        .with_explanation(
            "HashMap<String, V> needs owned String keys, not &str. \
             Use .to_string() when inserting string literals.",
        )
        .with_transform(CodeTransform::new(
            "Convert &str key to String",
            r#""([^"]+)""#,
            r#""$1".to_string()"#,
            r#"map.insert("key", value);"#,
            r#"map.insert("key".to_string(), value);"#,
        ))
        .with_suggestions(&[
            "Use .to_string() for literal keys",
            "Consider HashMap<&str, V> if all keys are static",
        ])
        .with_priority(85)
        .build(),
    );

    // Pattern 31: Vec::new() vs vec![] type inference
    registry.register(
        FixTemplate::builder(
            "e0282-vec-new",
            "Vec::new() Type Inference",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["Vec::new", "cannot infer", "type"])
        .with_explanation(
            "Vec::new() without elements can't infer the element type. \
             Add a type annotation or use vec![].",
        )
        .with_suggestions(&[
            "Add type: let v: Vec<i32> = Vec::new();",
            "Use turbofish: Vec::<i32>::new()",
            "Use vec![] with elements if possible",
        ])
        .with_priority(80)
        .build(),
    );

    // Pattern 32: Result error type mismatch
    registry.register(
        FixTemplate::builder(
            "e0308-result-error",
            "Result Error Type Mismatch",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["Result", "expected", "found", "Err"])
        .with_explanation(
            "Different Result types have different error types. \
             Use .map_err() to convert between error types.",
        )
        .with_suggestions(&[
            "Use .map_err(|e| e.into()) for Into-compatible errors",
            "Use anyhow::Result for uniform error handling",
            "Consider using thiserror for custom error types",
        ])
        .with_priority(80)
        .build(),
    );

    // Pattern 33: Index vs get for arrays/vectors
    registry.register(
        FixTemplate::builder(
            "e0277-index",
            "Index Trait Not Implemented",
            ErrorCategory::TraitBound,
        )
        .with_keywords(&["Index", "not implemented", "cannot index"])
        .with_explanation("The type doesn't support indexing with []. Use .get() for safe access.")
        .with_suggestions(&[
            "Use .get(index) which returns Option<&T>",
            "Check if the type should be a Vec or array",
            "For HashMap, use .get(&key)",
        ])
        .with_priority(80)
        .build(),
    );

    // Pattern 34: Deref coercion failure
    registry.register(
        FixTemplate::builder(
            "e0308-deref",
            "Deref Coercion Failure",
            ErrorCategory::TypeMismatch,
        )
        .with_keywords(&["expected", "&", "found", "Box", "Rc", "Arc"])
        .with_explanation(
            "Smart pointers (Box, Rc, Arc) deref automatically but sometimes need explicit deref.",
        )
        .with_suggestions(&[
            "Use &*ptr to explicitly deref",
            "Use .as_ref() for &T from smart pointer",
            "Clone if you need an owned value",
        ])
        .with_priority(75)
        .build(),
    );

    // Pattern 35: Lifetime elision failure
    registry.register(
        FixTemplate::builder(
            "e0106-lifetime",
            "Missing Lifetime Specifier",
            ErrorCategory::LifetimeError,
        )
        .with_keywords(&["missing lifetime specifier", "'"])
        .with_explanation(
            "The compiler can't infer the lifetime. Add explicit lifetime annotations.",
        )
        .with_suggestions(&[
            "Add lifetime parameter: fn foo<'a>(x: &'a str)",
            "Consider if you can use owned types instead",
            "Use 'static for literals and constants",
        ])
        .with_priority(80)
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
        let template =
            FixTemplate::builder("test-id", "Test Template", ErrorCategory::TypeMismatch)
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
        let template = FixTemplate::builder("test", "Test", ErrorCategory::TypeMismatch).build();

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

        registry
            .register(FixTemplate::builder("test", "Test", ErrorCategory::TypeMismatch).build());

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
