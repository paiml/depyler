use std as os;
use std::path::splitext;
use std::path::Path::exists;
use std::path::Path::file_name;
use std::path::Path::join;
use std::path::Path::parent;
#[doc = "Build a file path from components"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn build_file_path(base_dir: String) -> String {
    std::path::Path::join(base_dir, components)
}
#[doc = "Check if a file exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn check_file_exists(path: String) -> bool {
    std::path::Path::exists(path)
}
#[doc = "Get directory, filename, and extension"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_file_info(path: String) -> (String, String, String) {
    let dir_path = std::path::Path::parent(path);
    let base_name = std::path::Path::file_name(path);
    let (name, ext) = splitext(base_name);
    (dir_path, name, ext)
}
#[doc = "Find all Python files in a directory"]
#[doc = " Depyler: verified panic-free"]
pub fn find_python_files(directory: &str) -> Vec<String> {
    let mut python_files = vec![];
    for (root, _dirs, files) in os.walk(directory) {
        for file in files.iter().cloned() {
            if file.ends_with(".py") {
                python_files.push(std::path::Path::join(root, file));
            }
        }
    }
    python_files
}
#[doc = "Normalize a file path"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn normalize_path(path: &str) -> String {
    std::path::Path.normpath(path)
}
#[doc = "Get relative path from start to path"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_relative_path<'b, 'a>(path: &'a str, start: &'b str) -> String {
    std::path::Path.relpath(path, start)
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_check_file_exists_examples() {
        let _ = check_file_exists(Default::default());
    }
    #[test]
    fn quickcheck_normalize_path() {
        fn prop(path: String) -> TestResult {
            let once = normalize_path((&*path).into());
            let twice = normalize_path(once.clone());
            if once != twice {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
}
