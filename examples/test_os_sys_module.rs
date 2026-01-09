#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR__: &'static str = "/";
const STR_EMPTY: &'static str = "";
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
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
#[doc = "Test command-line arguments access"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sys_argv() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    args
}
#[doc = "Test Python version information"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sys_version_info() -> String {
    let version: String = "Python 3.x".to_string();
    version.to_string()
}
#[doc = "Test platform detection"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sys_platform() -> String {
    let platform: String = "darwin".to_string();
    platform.to_string()
}
#[doc = "Test exit code handling(without actually exiting)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sys_exit_code() -> i32 {
    let mut exit_code: i32 = Default::default();
    exit_code = 0;
    let condition: bool = true;
    if !condition {
        exit_code = 1;
    }
    exit_code
}
#[doc = "Test environment variable access"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_env_variable_access() -> String {
    let home: String =
        std::env::var("HOME".to_string()).unwrap_or_else(|_| "/home/user".to_string());
    home.to_string()
}
#[doc = "Test if environment variable exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_env_variable_exists() -> bool {
    let var_name: String = "PATH".to_string();
    let _cse_temp_0 = std::env::var(var_name).is_ok();
    let exists: bool = _cse_temp_0;
    exists
}
#[doc = "Test getting current working directory"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_current_directory() -> String {
    let cwd: String = std::env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();
    cwd.to_string()
}
#[doc = "Test path joining"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_join() -> String {
    let base: String = "/home/user".to_string();
    let relative: String = "documents".to_string();
    let filename: String = "file.txt".to_string();
    let _cse_temp_0 = format!("{}{}", format!("{}{}", base, STR__), relative);
    let _cse_temp_1 = format!("{}{}", format!("{}{}", _cse_temp_0, STR__), filename);
    let full_path: String = _cse_temp_1.to_string();
    full_path.to_string()
}
#[doc = "Test extracting basename from path"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_basename() -> Result<String, Box<dyn std::error::Error>> {
    let mut last_slash: i32 = Default::default();
    let mut basename: String = Default::default();
    let path: String = "/home/user/documents/file.txt".to_string();
    last_slash = -1;
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(path.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        if {
            let base = &path;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == STR__
        {
            last_slash = i;
            break;
        }
    }
    let _cse_temp_0 = last_slash >= 0;
    if _cse_temp_0 {
        basename = {
            let base = path;
            let start_idx: i32 = last_slash + 1;
            let len = base.chars().count() as i32;
            let actual_start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx.min(len) as usize
            };
            base.chars().skip(actual_start).collect::<String>()
        };
    } else {
        basename = path.to_string();
    }
    Ok(basename.to_string())
}
#[doc = "Test extracting directory name from path"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_dirname() -> Result<String, Box<dyn std::error::Error>> {
    let mut last_slash: i32 = Default::default();
    let mut dirname: String = Default::default();
    let path: String = "/home/user/documents/file.txt".to_string();
    last_slash = -1;
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(path.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        if {
            let base = &path;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == STR__
        {
            last_slash = i;
            break;
        }
    }
    let _cse_temp_0 = last_slash > 0;
    if _cse_temp_0 {
        dirname = {
            let base = path;
            let stop_idx: i32 = last_slash;
            let len = base.chars().count() as i32;
            let actual_stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx.min(len) as usize
            };
            base.chars().take(actual_stop).collect::<String>()
        };
    } else {
        dirname = STR__.to_string();
    }
    Ok(dirname.to_string())
}
#[doc = "Test splitting path into directory and basename"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_split() -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut last_slash: i32 = Default::default();
    let mut dirname: String = Default::default();
    let mut basename: String = Default::default();
    let path: String = "/home/user/documents/file.txt".to_string();
    last_slash = -1;
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(path.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        if {
            let base = &path;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == STR__
        {
            last_slash = i;
            break;
        }
    }
    let _cse_temp_0 = last_slash >= 0;
    if _cse_temp_0 {
        dirname = {
            let base = path;
            let stop_idx: i32 = last_slash;
            let len = base.chars().count() as i32;
            let actual_stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx.min(len) as usize
            };
            base.chars().take(actual_stop).collect::<String>()
        };
        basename = {
            let base = path;
            let start_idx: i32 = last_slash + 1;
            let len = base.chars().count() as i32;
            let actual_start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx.min(len) as usize
            };
            base.chars().skip(actual_start).collect::<String>()
        };
    } else {
        dirname = STR_EMPTY.to_string();
        basename = path.to_string();
    }
    Ok((dirname, basename))
}
#[doc = "Test splitting path into name and extension"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_splitext() -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut last_dot: i32 = Default::default();
    let mut name: String = Default::default();
    let mut ext: String = Default::default();
    let path: String = "document.txt".to_string();
    last_dot = -1;
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(path.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        if {
            let base = &path;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == "."
        {
            last_dot = i;
            break;
        }
    }
    let _cse_temp_0 = last_dot > 0;
    if _cse_temp_0 {
        name = {
            let base = path;
            let stop_idx: i32 = last_dot;
            let len = base.chars().count() as i32;
            let actual_stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx.min(len) as usize
            };
            base.chars().take(actual_stop).collect::<String>()
        };
        ext = {
            let base = path;
            let start_idx: i32 = last_dot;
            let len = base.chars().count() as i32;
            let actual_start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx.min(len) as usize
            };
            base.chars().skip(actual_start).collect::<String>()
        };
    } else {
        name = path.to_string();
        ext = STR_EMPTY.to_string();
    }
    Ok((name, ext))
}
#[doc = "Test if path is absolute"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_isabs() -> Result<bool, Box<dyn std::error::Error>> {
    let path: String = "/home/user/file.txt".to_string();
    let _cse_temp_0 = path.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 0;
    let _cse_temp_2 = {
        let base = &path;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.chars().count().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.chars()
            .nth(actual_idx)
            .map(|c| c.to_string())
            .unwrap_or_default()
    } == STR__;
    let _cse_temp_3 = (_cse_temp_1) && (_cse_temp_2);
    let is_absolute: bool = _cse_temp_3;
    Ok(is_absolute)
}
#[doc = "Test normalizing path(remove redundant separators)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_path_normpath() -> String {
    let path: String = "/home//user/../user/./documents".to_string();
    let normalized: String = path.replace("//", "/");
    normalized.to_string()
}
#[doc = "Get file extension from filename"]
#[doc = " Depyler: proven to terminate"]
pub fn get_file_extension(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut last_dot: i32 = Default::default();
    let mut extension: String = Default::default();
    last_dot = -1;
    for i in {
        let step = (-1 as i32).abs() as usize;
        if step == 0 {
            panic!("range() arg 3 must not be zero");
        }
        (-1..(filename.len() as i32).saturating_sub(1))
            .rev()
            .step_by(step.max(1))
    } {
        if {
            let base = &filename;
            let idx: i32 = i;
            let actual_idx = if idx < 0 {
                base.chars().count().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.chars()
                .nth(actual_idx)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } == "."
        {
            last_dot = i;
            break;
        }
    }
    let _cse_temp_0 = last_dot >= 0;
    if _cse_temp_0 {
        extension = {
            let base = filename;
            let start_idx: i32 = last_dot + 1;
            let len = base.chars().count() as i32;
            let actual_start = if start_idx < 0 {
                (len + start_idx).max(0) as usize
            } else {
                start_idx.min(len) as usize
            };
            base.chars().skip(actual_start).collect::<String>()
        };
    } else {
        extension = STR_EMPTY.to_string();
    }
    Ok(extension.to_string())
}
#[doc = "Check if file is hidden(starts with dot)"]
#[doc = " Depyler: proven to terminate"]
pub fn is_hidden_file(filename: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = filename.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(false);
    }
    let _cse_temp_2 = {
        let base = &filename;
        let idx: i32 = 0;
        let actual_idx = if idx < 0 {
            base.chars().count().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.chars()
            .nth(actual_idx)
            .map(|c| c.to_string())
            .unwrap_or_default()
    } == ".";
    let is_hidden: bool = _cse_temp_2;
    Ok(is_hidden)
}
#[doc = "Build path from list of components"]
#[doc = " Depyler: proven to terminate"]
pub fn build_path_from_parts(parts: &Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut path: String = Default::default();
    let _cse_temp_0 = parts.len() as i32;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(STR_EMPTY);
    }
    path = parts
        .get(0usize)
        .cloned()
        .expect("IndexError: list index out of range");
    for i in (1)..(parts.len() as i32) {
        path = format!(
            "{}{}",
            format!("{}{}", path, STR__),
            parts
                .get(i as usize)
                .cloned()
                .expect("IndexError: list index out of range")
        );
    }
    Ok(path.to_string())
}
#[doc = "Simulate directory listing(manual implementation)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_listdir_simulation() -> Vec<String> {
    let files: Vec<String> = vec![
        "file1.txt".to_string(),
        "file2.py".to_string(),
        "dir1".to_string(),
        ".hidden".to_string(),
    ];
    files
}
#[doc = "Filter files by extension"]
#[doc = " Depyler: verified panic-free"]
pub fn filter_by_extension<'a, 'b>(
    files: &'a Vec<String>,
    ext: &'b str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut filtered: Vec<String> = vec![];
    for file in files.iter().cloned() {
        let file_ext: String = get_file_extension(&file)?;
        if file_ext == ext {
            filtered.push(file);
        }
    }
    Ok(filtered)
}
#[doc = "Count files grouped by extension"]
pub fn count_files_by_extension(
    files: &Vec<String>,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut ext: String = Default::default();
    let mut counts: std::collections::HashMap<String, i32> = {
        let map: HashMap<String, i32> = HashMap::new();
        map
    };
    for file in files.iter().cloned() {
        ext = get_file_extension(&file)?;
        if ext == STR_EMPTY {
            ext = "no_extension".to_string();
        }
        if counts.get(&ext).is_some() {
            {
                let _key = ext;
                let _old_val = counts.get(&_key).cloned().unwrap_or_default();
                counts.insert(_key, _old_val + 1);
            }
        } else {
            counts.insert(ext.to_string().clone(), 1);
        }
    }
    Ok(counts)
}
#[doc = "Simulate path traversal with depth limit"]
#[doc = " Depyler: verified panic-free"]
pub fn test_path_traversal(path: &str, _max_depth: i32) -> i32 {
    let mut depth: i32 = Default::default();
    depth = 0;
    for char in path.chars() {
        if char.to_string() == STR__ {
            depth = depth + 1;
        }
    }
    depth
}
#[doc = "Remove invalid characters from filename"]
#[doc = " Depyler: verified panic-free"]
pub fn sanitize_filename(filename: &str) -> String {
    let mut sanitized: String = Default::default();
    let invalid_chars: String = "<>:\"|?*".to_string();
    sanitized = STR_EMPTY.to_string();
    for char in filename.chars() {
        let mut is_invalid: bool = false;
        for invalid in invalid_chars.chars() {
            if char == invalid {
                is_invalid = true;
                break;
            }
        }
        if !is_invalid {
            sanitized = format!("{}{}", sanitized, char);
        }
    }
    sanitized.to_string()
}
#[doc = "Run all os/sys module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_os_sys_features() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = test_sys_argv();
    let version: String = test_sys_version_info();
    let platform: String = test_sys_platform();
    let exit_code: i32 = test_sys_exit_code();
    let home: String = test_env_variable_access();
    let env_exists: bool = test_env_variable_exists();
    let cwd: String = test_current_directory();
    let joined: String = test_path_join();
    let basename: String = test_path_basename()?;
    let dirname: String = test_path_dirname()?;
    let split_result: (String, String) = test_path_split()?;
    let splitext_result: (String, String) = test_path_splitext()?;
    let is_abs: bool = test_path_isabs()?;
    let normalized: String = test_path_normpath();
    let ext: String = get_file_extension("document.txt")?;
    let hidden: bool = is_hidden_file(".gitignore")?;
    let parts: Vec<String> = vec![
        "home".to_string(),
        "user".to_string(),
        "documents".to_string(),
    ];
    let built_path: String = build_path_from_parts(&parts)?;
    let files: Vec<String> = test_listdir_simulation();
    let txt_files: Vec<String> = filter_by_extension(&files, "txt")?;
    let file_counts: std::collections::HashMap<String, i32> = count_files_by_extension(&files)?;
    let depth: i32 = test_path_traversal("/home/user/docs", 5);
    let safe_name: String = sanitize_filename("file<>name.txt");
    println!("{}", "All os/sys module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_sys_exit_code_examples() {
        let _ = test_sys_exit_code();
    }
    #[test]
    fn quickcheck_test_path_isabs() {
        fn prop() -> TestResult {
            let result = test_path_isabs();
            if result < 0 {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn() -> TestResult);
    }
    #[test]
    fn test_is_hidden_file_examples() {
        let _ = is_hidden_file(Default::default());
    }
}
