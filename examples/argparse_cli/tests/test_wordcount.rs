#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::io::Cursor;
#[doc = "// NOTE: Map Python module 'pytest'(tracked in DEPYLER-0424)"]
#[doc = "// NOTE: Map Python module 'hypothesis'(tracked in DEPYLER-0424)"]
#[doc = "// NOTE: Map Python module 'wordcount'(tracked in DEPYLER-0424)"]
use std::path::PathBuf;
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TestCountFile {}
impl TestCountFile {
    pub fn new() -> Self {
        Self {}
    }
    pub fn test_count_file_success(&self, _tmp_path: ()) {
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text("line1\nline2\nline3".to_string());
        let stats = wordcount.count_file(test_file);
        assert!(stats.lines == 3);
        assert!(stats.words == 3);
        assert!(stats.chars == 17);
        assert!(stats.filename == test_file.to_string());
    }
    pub fn test_count_file_empty(&self, _tmp_path: ()) {
        let test_file = tmp_path / "empty.txt".to_string();
        test_file.write_text("".to_string());
        let stats = wordcount.count_file(test_file);
        assert!(stats.lines == 0);
        assert!(stats.words == 0);
        assert!(stats.chars == 0);
        assert!(stats.filename == test_file.to_string());
    }
    pub fn test_count_file_single_line_no_newline(&self, _tmp_path: ()) {
        let test_file = tmp_path / "single.txt".to_string();
        test_file.write_text("hello world".to_string());
        let stats = wordcount.count_file(test_file);
        assert!(stats.lines == 1);
        assert!(stats.words == 2);
        assert!(stats.chars == 11);
    }
    pub fn test_count_file_multiple_spaces(&self, _tmp_path: ()) {
        let test_file = tmp_path / "spaces.txt".to_string();
        test_file.write_text("hello    world\nfoo  bar".to_string());
        let stats = wordcount.count_file(test_file);
        assert!(stats.lines == 2);
        assert!(stats.words == 4);
        assert!(stats.chars == 23);
    }
    pub fn test_count_file_blank_lines(&self, _tmp_path: ()) {
        let test_file = tmp_path / "blanks.txt".to_string();
        test_file.write_text("line1\n\nline3\n\n".to_string());
        let stats = wordcount.count_file(test_file);
        assert!(stats.lines == 4);
        assert!(stats.words == 2);
    }
    pub fn test_count_file_unicode(&self, _tmp_path: ()) {
        let test_file = tmp_path / "unicode.txt".to_string();
        test_file.write_text("Hello 世界\nПривет мир\n".to_string());
        let stats = wordcount.count_file(test_file);
        assert!(stats.lines == 2);
        assert!(stats.words == 4);
        assert!(stats.chars == 20);
    }
    pub fn test_count_file_ioerror(&self, _tmp_path: (), _capsys: ()) {
        let nonexistent = tmp_path / "nonexistent.txt".to_string();
        let stats = wordcount.count_file(nonexistent);
        assert!(stats.lines == 0);
        assert!(stats.words == 0);
        assert!(stats.chars == 0);
        assert!(stats.filename == nonexistent.to_string());
        let captured = capsys.readouterr();
        assert!(captured.err.contains_key(&"Error reading".to_string()));
        assert!(captured.err.contains_key(&nonexistent.to_string()));
    }
}
#[derive(Debug, Clone)]
pub struct TestFormatStats {}
impl TestFormatStats {
    pub fn new() -> Self {
        Self {}
    }
    pub fn test_format_stats_with_filename(&self) {
        let stats = wordcount.Stats(10, 20, 30, "test.txt".to_string());
        let result = wordcount.format_stats(stats);
        assert!(result == "      10       20       30 test.txt".to_string());
    }
    pub fn test_format_stats_without_filename(&self) {
        let stats = wordcount.Stats(5, 15, 25, "test.txt".to_string());
        let result = wordcount.format_stats(stats);
        assert!(result == "       5       15       25".to_string());
        assert!(!result.contains_key(&"test.txt".to_string()));
    }
    pub fn test_format_stats_default_with_filename(&self) {
        let stats = wordcount.Stats(1, 2, 3, "file.txt".to_string());
        let result = wordcount.format_stats(stats);
        assert!(result.contains_key(&"file.txt".to_string()));
    }
    pub fn test_format_stats_alignment(&self) {
        let stats = wordcount.Stats(999, 8888, 77777, "test.txt".to_string());
        let result = wordcount.format_stats(stats);
        let parts = result
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!((parts.len() as i32) == 3);
        assert!(parts[0 as usize] == "999".to_string());
        assert!(parts[1 as usize] == "8888".to_string());
        assert!(parts[2 as usize] == "77777".to_string());
    }
}
#[derive(Debug, Clone)]
pub struct TestMainFunction {}
impl TestMainFunction {
    pub fn new() -> Self {
        Self {}
    }
    pub fn test_main_single_file(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text("hello world\nfoo bar\n".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec!["wordcount.py".to_string(), test_file.to_string()],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        assert!(captured.out.contains_key(&"2".to_string()));
        assert!(captured.out.contains_key(&"4".to_string()));
        assert!(captured.out.contains_key(&test_file.to_string()));
    }
    pub fn test_main_multiple_files(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let file1 = tmp_path / "file1.txt".to_string();
        file1.write_text("one\ntwo\n".to_string());
        let file2 = tmp_path / "file2.txt".to_string();
        file2.write_text("three\nfour\n".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec![
                "wordcount.py".to_string(),
                file1.to_string(),
                file2.to_string(),
            ],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        assert!(captured.out.contains_key(&file1.to_string()));
        assert!(captured.out.contains_key(&file2.to_string()));
        assert!(captured.out.contains_key(&"total".to_string()));
    }
    pub fn test_main_lines_only(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text("line1\nline2\nline3\n".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec![
                "wordcount.py".to_string(),
                "-l".to_string(),
                test_file.to_string(),
            ],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        let lines = captured
            .out
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!(lines[0 as usize] == "3".to_string());
        assert!(captured.out.contains_key(&test_file.to_string()));
    }
    pub fn test_main_words_only(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text("one two three four five".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec![
                "wordcount.py".to_string(),
                "--words".to_string(),
                test_file.to_string(),
            ],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        let lines = captured
            .out
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!(lines[0 as usize] == "5".to_string());
    }
    pub fn test_main_chars_only(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text("12345".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec![
                "wordcount.py".to_string(),
                "-c".to_string(),
                test_file.to_string(),
            ],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        let lines = captured
            .out
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!(lines[0 as usize] == "5".to_string());
    }
    pub fn test_main_no_files_shows_help(&self, _monkeypatch: (), _capsys: ()) {
        monkeypatch.setattr(sys, "argv".to_string(), vec!["wordcount.py".to_string()]);
        {
            let mut exc_info = pytest.raises(SystemExit);
            {
                wordcount.main();
            }
        }
        assert!(exc_info.value.code == 2);
    }
    pub fn test_main_nonexistent_file(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let nonexistent = tmp_path / "doesnotexist.txt".to_string();
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec!["wordcount.py".to_string(), nonexistent.to_string()],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        assert!(captured.out.contains_key(&"0".to_string()));
        assert!(captured.err.contains_key(&"Error reading".to_string()));
    }
}
#[derive(Debug, Clone)]
pub struct TestPropertyBased {}
impl TestPropertyBased {
    pub fn new() -> Self {
        Self {}
    }
    pub fn test_count_file_chars_property(&self, _tmp_path_factory: (), _content: ()) {
        let tmp_path = tmp_path_factory.mktemp("data".to_string());
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text(content);
        let stats = wordcount.count_file(test_file);
        assert!(stats.chars = = (content.len() as i32));
    }
    pub fn test_count_file_words_property(&self, _tmp_path_factory: (), _words: ()) {
        let tmp_path = tmp_path_factory.mktemp("data".to_string());
        let test_file = tmp_path / "test.txt".to_string();
        let content = words.join(" ".to_string());
        test_file.write_text(content);
        let stats = wordcount.count_file(test_file);
        let expected_words = content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .len() as i32;
        assert!(stats.words == expected_words);
    }
    pub fn test_count_file_lines_property(&self, _tmp_path_factory: (), _lines: ()) {
        let tmp_path = tmp_path_factory.mktemp("data".to_string());
        let test_file = tmp_path / "test.txt".to_string();
        let content = lines.join("\n".to_string());
        test_file.write_text(content);
        let stats = wordcount.count_file(test_file);
        let expected_lines = content.splitlines().len() as i32;
        assert!(stats.lines == expected_lines);
    }
    pub fn test_format_stats_property(&self, _lines: (), _words: (), _chars: ()) {
        let stats = wordcount.Stats(lines, words, chars, "test.txt".to_string());
        let result = wordcount.format_stats(stats);
        let parts = result
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!((parts.len() as i32) == 3);
        assert!(parts[0 as usize].parse::<i32>().unwrap_or(0) == lines);
        assert!(parts[1 as usize].parse::<i32>().unwrap_or(0) == words);
        assert!(parts[2 as usize].parse::<i32>().unwrap_or(0) == chars);
    }
}
#[derive(Debug, Clone)]
pub struct TestIntegration {}
impl TestIntegration {
    pub fn new() -> Self {
        Self {}
    }
    pub fn test_main_entry_point_via_subprocess(&self, _tmp_path: ()) {
        {}
        let test_file = tmp_path / "test.txt".to_string();
        test_file.write_text("hello world\n".to_string());
        let result = subprocess.run(vec![
            sys.executable,
            PyPath::new(__file__).parent.parent
                / "python".to_string()
                / "wordcount.py".to_string().to_string(),
            test_file.to_string(),
        ]);
        assert!(result.returncode == 0);
        assert!(result.stdout.contains(&*test_file.to_string()));
        assert!(result.stdout.contains("1"));
        assert!(result.stdout.contains("2"));
    }
    pub fn test_sample_file_exact_output(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let sample = tmp_path / "sample.txt".to_string();
        sample.write_text("The quick brown fox jumps over the lazy dog.\nThis is a test file for word count demonstration.\nIt contains multiple lines of text.\n\nPython is a great programming language.\nRust is fast and safe.\nDepyler converts Python to Rust.\n".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec!["wordcount.py".to_string(), sample.to_string()],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        assert!(captured.out.contains_key(&sample.to_string()));
        let output_parts = captured
            .out
            .trim()
            .to_string()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!((output_parts.len() as i32) == 4);
    }
    pub fn test_mixed_existing_and_nonexistent_files(
        &self,
        _tmp_path: (),
        _monkeypatch: (),
        _capsys: (),
    ) {
        let valid_file = tmp_path / "valid.txt".to_string();
        valid_file.write_text("content".to_string());
        let invalid_file = tmp_path / "invalid.txt".to_string();
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec![
                "wordcount.py".to_string(),
                valid_file.to_string(),
                invalid_file.to_string(),
            ],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        assert!(captured.out.contains_key(&valid_file.to_string()));
        assert!(captured.err.contains_key(&"Error reading".to_string()));
    }
    pub fn test_all_flags_with_multiple_files(&self, _tmp_path: (), _monkeypatch: (), _capsys: ()) {
        let file1 = tmp_path / "f1.txt".to_string();
        file1.write_text("one two\nthree\n".to_string());
        let file2 = tmp_path / "f2.txt".to_string();
        file2.write_text("four five\nsix\n".to_string());
        monkeypatch.setattr(
            sys,
            "argv".to_string(),
            vec![
                "wordcount.py".to_string(),
                "-l".to_string(),
                file1.to_string(),
                file2.to_string(),
            ],
        );
        let exit_code = wordcount.main();
        assert!(exit_code == 0);
        let captured = capsys.readouterr();
        let lines = captured
            .out
            .trim()
            .to_string()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!((lines.len() as i32) == 3);
        assert!(captured.out.contains_key(&"total".to_string()));
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn given<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn strategies<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn settings<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn assume<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
