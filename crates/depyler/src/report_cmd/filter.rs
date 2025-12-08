//! Corpus filtering and bisection module (DEPYLER-BISECT-001)
//!
//! Implements targeted segmentation and bisection for granular corpus analysis.
//! Based on Delta Debugging (Zeller & Hildebrandt, 2002) and Regression Test
//! Selection (Rothermel & Harrold, 1997).

use regex::Regex;
use std::collections::HashSet;
use std::path::PathBuf;

/// Filter configuration for corpus segmentation.
#[derive(Debug, Clone, Default)]
pub struct FilterConfig {
    /// Regex/glob pattern to filter files by path
    pub pattern: Option<String>,
    /// Semantic tags to filter by (e.g., "Dict", "argparse")
    pub tags: Vec<String>,
    /// Maximum number of files to process
    pub limit: Option<usize>,
    /// Random sample size
    pub sample: Option<usize>,
    /// Stop on first failure
    pub fail_fast: bool,
}

/// Semantic tags extracted from Python source files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticTag {
    /// Dictionary operations (dict, Dict, {})
    Dict,
    /// List operations (list, List, [])
    List,
    /// Set operations (set, Set)
    Set,
    /// Argparse CLI handling
    Argparse,
    /// Async/await patterns
    Async,
    /// Class definitions
    Class,
    /// Dataclasses
    Dataclass,
    /// Lambda expressions
    Lambda,
    /// Generator expressions
    Generator,
    /// Comprehensions (list, dict, set)
    Comprehension,
    /// File I/O operations
    FileIO,
    /// JSON operations
    Json,
    /// Regular expressions
    Regex,
    /// Datetime operations
    Datetime,
    /// Custom tag
    Custom(String),
}

impl SemanticTag {
    /// Parse a tag from string.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dict" | "dictionary" => Self::Dict,
            "list" => Self::List,
            "set" => Self::Set,
            "argparse" | "cli" => Self::Argparse,
            "async" | "asyncio" => Self::Async,
            "class" => Self::Class,
            "dataclass" => Self::Dataclass,
            "lambda" => Self::Lambda,
            "generator" => Self::Generator,
            "comprehension" | "comp" => Self::Comprehension,
            "file" | "fileio" | "io" => Self::FileIO,
            "json" => Self::Json,
            "regex" | "re" => Self::Regex,
            "datetime" | "date" | "time" => Self::Datetime,
            other => Self::Custom(other.to_string()),
        }
    }

    /// Get detection patterns for this tag.
    pub fn detection_patterns(&self) -> Vec<&'static str> {
        match self {
            Self::Dict => vec![
                r#"dict\s*\("#,
                r#"Dict\["#,
                r#"\{\s*['\"]"#,
                r#"\.get\("#,
                r#"\.items\("#,
                r#"\.keys\("#,
                r#"\.values\("#,
            ],
            Self::List => vec![
                r#"list\s*\("#,
                r#"List\["#,
                r#"\[\s*\d"#,
                r#"\.append\("#,
                r#"\.extend\("#,
            ],
            Self::Set => vec![r#"set\s*\("#, r#"Set\["#, r#"\{[^:]+\}"#],
            Self::Argparse => vec![
                r#"import\s+argparse"#,
                r#"from\s+argparse"#,
                "ArgumentParser",
                "add_argument",
            ],
            Self::Async => vec![r#"async\s+def"#, r#"await\s+"#, "asyncio"],
            Self::Class => vec![r#"class\s+\w+"#],
            Self::Dataclass => vec!["@dataclass", r#"from\s+dataclasses"#],
            Self::Lambda => vec![r#"lambda\s+"#],
            Self::Generator => vec![r#"yield\s+"#, r#"\(.*\s+for\s+.*\s+in\s+"#],
            Self::Comprehension => vec![
                r#"\[.*\s+for\s+.*\s+in\s+.*\]"#,
                r#"\{.*\s+for\s+.*\s+in\s+.*\}"#,
            ],
            Self::FileIO => vec![r#"open\s*\("#, r#"with\s+open"#, r#"\.read\("#, r#"\.write\("#],
            Self::Json => vec![r#"import\s+json"#, r#"json\."#, r#"\.loads\("#, r#"\.dumps\("#],
            Self::Regex => vec![r#"import\s+re"#, r#"re\."#, r#"\.match\("#, r#"\.search\("#],
            Self::Datetime => vec![r#"import\s+datetime"#, r#"datetime\."#, "timedelta"],
            Self::Custom(_) => vec![],
        }
    }
}

/// Extract semantic tags from Python source code.
pub fn extract_tags(source: &str) -> HashSet<SemanticTag> {
    let mut tags = HashSet::new();

    let all_tags = [
        SemanticTag::Dict,
        SemanticTag::List,
        SemanticTag::Set,
        SemanticTag::Argparse,
        SemanticTag::Async,
        SemanticTag::Class,
        SemanticTag::Dataclass,
        SemanticTag::Lambda,
        SemanticTag::Generator,
        SemanticTag::Comprehension,
        SemanticTag::FileIO,
        SemanticTag::Json,
        SemanticTag::Regex,
        SemanticTag::Datetime,
    ];

    for tag in all_tags {
        for pattern in tag.detection_patterns() {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(source) {
                    tags.insert(tag.clone());
                    break;
                }
            }
        }
    }

    tags
}

/// Filter a list of Python files based on configuration.
pub fn filter_files(files: &[PathBuf], config: &FilterConfig) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = files.to_vec();

    // Apply pattern filter
    if let Some(pattern) = &config.pattern {
        if let Ok(re) = Regex::new(pattern) {
            result.retain(|path| {
                let path_str = path.to_string_lossy();
                re.is_match(&path_str)
            });
        } else {
            // Fallback to glob-style matching
            result.retain(|path| {
                let path_str = path.to_string_lossy();
                path_str.contains(pattern)
            });
        }
    }

    // Apply tag filter
    if !config.tags.is_empty() {
        let required_tags: HashSet<SemanticTag> =
            config.tags.iter().map(|s| SemanticTag::from_str(s)).collect();

        result.retain(|path| {
            if let Ok(source) = std::fs::read_to_string(path) {
                let file_tags = extract_tags(&source);
                // File must have at least one of the required tags
                required_tags.iter().any(|t| file_tags.contains(t))
            } else {
                false
            }
        });
    }

    // Apply random sampling
    if let Some(sample_size) = config.sample {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        result.shuffle(&mut rng);
        result.truncate(sample_size);
    }

    // Apply limit (after sampling)
    if let Some(limit) = config.limit {
        result.truncate(limit);
    }

    result
}

/// Bisection state for isolating failures.
#[derive(Debug, Clone)]
pub struct BisectionState {
    /// Files being bisected
    pub files: Vec<PathBuf>,
    /// Current search bounds (inclusive)
    pub low: usize,
    pub high: usize,
    /// Iteration count
    pub iteration: usize,
    /// Maximum iterations (safety limit)
    pub max_iterations: usize,
    /// Found minimal failing set
    pub result: Option<Vec<PathBuf>>,
}

impl BisectionState {
    /// Create new bisection state.
    pub fn new(files: Vec<PathBuf>) -> Self {
        let len = files.len();
        Self {
            files,
            low: 0,
            high: len.saturating_sub(1),
            iteration: 0,
            max_iterations: 20, // Safety limit per spec
            result: None,
        }
    }

    /// Check if bisection is complete.
    pub fn is_complete(&self) -> bool {
        self.result.is_some() || self.iteration >= self.max_iterations || self.low >= self.high
    }

    /// Get current test set (first half or second half).
    pub fn current_test_set(&self) -> Vec<PathBuf> {
        let mid = (self.low + self.high) / 2;
        self.files[self.low..=mid].to_vec()
    }

    /// Advance bisection based on test result.
    /// `failure_in_first_half` indicates if the failure was found in the first half.
    pub fn advance(&mut self, failure_in_first_half: bool) {
        self.iteration += 1;

        let mid = (self.low + self.high) / 2;

        if failure_in_first_half {
            self.high = mid;
        } else {
            self.low = mid + 1;
        }

        // Check for completion
        if self.low >= self.high {
            // Isolated to a single file (or range exhausted)
            if self.low < self.files.len() {
                self.result = Some(vec![self.files[self.low].clone()]);
            }
        }
    }

    /// Get the isolated failing file(s).
    pub fn get_result(&self) -> Option<&Vec<PathBuf>> {
        self.result.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    // ============================================================
    // RED PHASE: Failing tests for DEPYLER-BISECT-001
    // ============================================================

    #[test]
    fn test_filter_by_pattern_regex() {
        let temp = TempDir::new().unwrap();
        let files = create_test_files(&temp, &["argparse_cli.py", "dict_ops.py", "list_test.py"]);

        let config = FilterConfig {
            pattern: Some("argparse".to_string()),
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].to_string_lossy().contains("argparse"));
    }

    #[test]
    fn test_filter_by_pattern_glob_style() {
        let temp = TempDir::new().unwrap();
        let files = create_test_files(&temp, &["test_dict.py", "test_list.py", "main.py"]);

        let config = FilterConfig {
            pattern: Some("test_".to_string()),
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_limit() {
        let temp = TempDir::new().unwrap();
        let files = create_test_files(&temp, &["a.py", "b.py", "c.py", "d.py", "e.py"]);

        let config = FilterConfig {
            limit: Some(3),
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_filter_by_sample() {
        let temp = TempDir::new().unwrap();
        let files = create_test_files(&temp, &["a.py", "b.py", "c.py", "d.py", "e.py"]);

        let config = FilterConfig {
            sample: Some(2),
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_tag_dict() {
        let temp = TempDir::new().unwrap();

        // Create files with specific content
        let dict_file = temp.path().join("dict_ops.py");
        std::fs::write(&dict_file, "data = {\"key\": \"value\"}\ndata.get(\"key\")").unwrap();

        let list_file = temp.path().join("list_ops.py");
        std::fs::write(&list_file, "items = [1, 2, 3]\nitems.append(4)").unwrap();

        let files = vec![dict_file, list_file];

        let config = FilterConfig {
            tags: vec!["dict".to_string()],
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].to_string_lossy().contains("dict"));
    }

    #[test]
    fn test_filter_by_tag_argparse() {
        let temp = TempDir::new().unwrap();

        let argparse_file = temp.path().join("cli.py");
        std::fs::write(
            &argparse_file,
            "import argparse\nparser = argparse.ArgumentParser()",
        )
        .unwrap();

        let simple_file = temp.path().join("simple.py");
        std::fs::write(&simple_file, "print(\"hello\")").unwrap();

        let files = vec![argparse_file, simple_file];

        let config = FilterConfig {
            tags: vec!["argparse".to_string()],
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].to_string_lossy().contains("cli"));
    }

    #[test]
    fn test_extract_tags_dict() {
        let source = r#"
data = {"key": "value"}
result = data.get("key", "default")
for k, v in data.items():
    print(k, v)
"#;
        let tags = extract_tags(source);
        assert!(tags.contains(&SemanticTag::Dict));
    }

    #[test]
    fn test_extract_tags_argparse() {
        let source = r#"
import argparse

parser = argparse.ArgumentParser(description="Test CLI")
parser.add_argument("--input", help="Input file")
args = parser.parse_args()
"#;
        let tags = extract_tags(source);
        assert!(tags.contains(&SemanticTag::Argparse));
    }

    #[test]
    fn test_extract_tags_async() {
        let source = r#"
import asyncio

async def fetch_data():
    await asyncio.sleep(1)
    return "data"
"#;
        let tags = extract_tags(source);
        assert!(tags.contains(&SemanticTag::Async));
    }

    #[test]
    fn test_extract_tags_multiple() {
        let source = r#"
import json

class DataProcessor:
    def __init__(self):
        self.data = {}

    def process(self, items: list):
        return [x * 2 for x in items]
"#;
        let tags = extract_tags(source);
        assert!(tags.contains(&SemanticTag::Class));
        assert!(tags.contains(&SemanticTag::Json));
        assert!(tags.contains(&SemanticTag::Comprehension));
    }

    #[test]
    fn test_semantic_tag_from_str() {
        assert_eq!(SemanticTag::from_str("dict"), SemanticTag::Dict);
        assert_eq!(SemanticTag::from_str("Dict"), SemanticTag::Dict);
        assert_eq!(SemanticTag::from_str("dictionary"), SemanticTag::Dict);
        assert_eq!(SemanticTag::from_str("argparse"), SemanticTag::Argparse);
        assert_eq!(SemanticTag::from_str("cli"), SemanticTag::Argparse);
        assert_eq!(SemanticTag::from_str("async"), SemanticTag::Async);
        assert_eq!(
            SemanticTag::from_str("custom_tag"),
            SemanticTag::Custom("custom_tag".to_string())
        );
    }

    // ============================================================
    // Bisection Tests
    // ============================================================

    #[test]
    fn test_bisection_state_new() {
        let files: Vec<PathBuf> = (0..10).map(|i| PathBuf::from(format!("{}.py", i))).collect();
        let state = BisectionState::new(files.clone());

        assert_eq!(state.low, 0);
        assert_eq!(state.high, 9);
        assert_eq!(state.iteration, 0);
        assert!(!state.is_complete());
    }

    #[test]
    fn test_bisection_isolate_single_failure() {
        // Simulate bisection to find file at index 7
        let files: Vec<PathBuf> = (0..16).map(|i| PathBuf::from(format!("{}.py", i))).collect();
        let mut state = BisectionState::new(files);

        // Iteration 1: mid=7, failure NOT in first half (0-7), so go to second half
        state.advance(false); // failure in 8-15
        assert_eq!(state.low, 8);
        assert_eq!(state.high, 15);

        // Iteration 2: mid=11, failure in first half (8-11)
        state.advance(true);
        assert_eq!(state.low, 8);
        assert_eq!(state.high, 11);

        // Iteration 3: mid=9, failure NOT in first half
        state.advance(false); // failure in 10-11
        assert_eq!(state.low, 10);
        assert_eq!(state.high, 11);

        // Iteration 4: mid=10, failure in first half
        state.advance(true); // failure at 10
        assert_eq!(state.low, 10);
        assert_eq!(state.high, 10);

        assert!(state.is_complete());
        let result = state.get_result().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], PathBuf::from("10.py"));
    }

    #[test]
    fn test_bisection_max_iterations_safety() {
        let files: Vec<PathBuf> = (0..1000)
            .map(|i| PathBuf::from(format!("{}.py", i)))
            .collect();
        let mut state = BisectionState::new(files);
        state.max_iterations = 5;

        // Force max iterations
        for _ in 0..10 {
            if state.is_complete() {
                break;
            }
            state.advance(true);
        }

        assert!(state.iteration <= 5);
    }

    #[test]
    fn test_bisection_log_n_complexity() {
        // For 1671 files, should take ~11 iterations (log2(1671) ≈ 10.7)
        let files: Vec<PathBuf> = (0..1671)
            .map(|i| PathBuf::from(format!("{}.py", i)))
            .collect();
        let mut state = BisectionState::new(files);

        // Simulate always finding in first half (worst case for iteration count)
        while !state.is_complete() {
            state.advance(true);
        }

        // Should complete in ~11 iterations
        assert!(state.iteration <= 15, "Bisection took {} iterations", state.iteration);
    }

    #[test]
    fn test_filter_combined_pattern_and_limit() {
        let temp = TempDir::new().unwrap();
        let files = create_test_files(
            &temp,
            &[
                "test_a.py",
                "test_b.py",
                "test_c.py",
                "test_d.py",
                "main.py",
            ],
        );

        let config = FilterConfig {
            pattern: Some("test_".to_string()),
            limit: Some(2),
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|f| f.to_string_lossy().contains("test_")));
    }

    #[test]
    fn test_filter_combined_tag_and_sample() {
        let temp = TempDir::new().unwrap();

        // Create 5 dict files and 5 list files
        for i in 0..5 {
            let dict_file = temp.path().join(format!("dict_{}.py", i));
            std::fs::write(&dict_file, "data = {\"k\": \"v\"}").unwrap();

            let list_file = temp.path().join(format!("list_{}.py", i));
            std::fs::write(&list_file, "items = [1, 2, 3]").unwrap();
        }

        let files: Vec<PathBuf> = std::fs::read_dir(temp.path())
            .unwrap()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();

        let config = FilterConfig {
            tags: vec!["dict".to_string()],
            sample: Some(3),
            ..Default::default()
        };

        let filtered = filter_files(&files, &config);
        assert_eq!(filtered.len(), 3);
        // All should be dict files
        for f in &filtered {
            let content = std::fs::read_to_string(f).unwrap();
            assert!(content.contains("\"k\"") || content.contains("'k'"));
        }
    }

    // Helper to create test files
    fn create_test_files(temp: &TempDir, names: &[&str]) -> Vec<PathBuf> {
        names
            .iter()
            .map(|name| {
                let path = temp.path().join(name);
                let mut file = std::fs::File::create(&path).unwrap();
                writeln!(file, "# Python file: {}", name).unwrap();
                path
            })
            .collect()
    }
}
