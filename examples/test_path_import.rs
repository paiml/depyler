use std::path::splitext;
#[doc = "Build a file path from components"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn build_file_path(base_dir: String, components: &[String]) -> String {
    std::path::PathBuf::from(base_dir)
        .join(components)
        .to_string_lossy()
        .to_string()
}
#[doc = "Check if a file exists"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn check_file_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}
#[doc = "Get directory, filename, and extension"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_file_info(path: String) -> (String, String, String) {
    let dir_path = std::path::Path::new(&path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
        .to_string();
    let base_name = std::path::Path::new(&path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    let (name, ext) = splitext(base_name);
    (dir_path, name, ext)
}
#[doc = "Find all Python files in a directory"]
#[doc = " Depyler: verified panic-free"]
pub fn find_python_files(directory: &str) -> Vec<String> {
    let mut python_files = vec![];
    for (root, _dirs, files) in walkdir::WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|dir_entry| {
            let root = dir_entry.path().to_string_lossy().to_string();
            let mut dirs = vec![];
            let mut files = vec![];
            if let Ok(entries) = std::fs::read_dir(dir_entry.path()) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        dirs.push(name);
                    } else {
                        files.push(name);
                    }
                }
            }
            (root, dirs, files)
        })
        .collect::<Vec<_>>()
    {
        for file in files.iter().cloned() {
            if file.ends_with(".py") {
                python_files.push(
                    std::path::PathBuf::from(root)
                        .join(file)
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }
    python_files
}
#[doc = "Normalize a file path"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn normalize_path(path: &str) -> String {
    {
        let p = std::path::Path::new(&path);
        let mut components = Vec::new();
        for component in p.components() {
            match component {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    components.pop();
                }
                _ => components.push(component),
            }
        }
        components
            .iter()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join(std::path::MAIN_SEPARATOR_STR)
    }
}
#[doc = "Get relative path from start to path"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_relative_path<'a, 'b>(path: &'a str, start: &'b str) -> String {
    {
        let path_obj = std::path::Path::new(path);
        let start_obj = std::path::Path::new(start);
        path_obj
            .strip_prefix(start_obj)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string())
    }
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
