//! Harvester: Package fetching using `uv pip install --target`.
//!
//! The Harvester is responsible for downloading Python packages and their
//! type stubs using the `uv` package manager for deterministic resolution.
//!
//! # Stub Package Strategy
//!
//! When fetching a package like `requests`, the Harvester also attempts to
//! fetch `types-requests` which contains complete `.pyi` type stubs from
//! typeshed. This provides accurate return types like `-> Response` instead
//! of the incomplete annotations in source `.py` files.

use crate::{KnowledgeError, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info, warn};
use walkdir::WalkDir;

/// Result of a successful package harvest.
#[derive(Debug, Clone)]
pub struct HarvestResult {
    /// The package name that was harvested
    pub package: String,
    /// Root directory where package was installed
    pub root: PathBuf,
    /// List of .pyi stub files found
    pub stub_files: Vec<PathBuf>,
    /// List of .py source files (fallback if no stubs)
    pub source_files: Vec<PathBuf>,
    /// Whether types-* stub package was found
    pub has_types_package: bool,
}

impl HarvestResult {
    /// Returns all parseable files (stubs preferred, sources as fallback).
    pub fn all_files(&self) -> Vec<&Path> {
        if self.stub_files.is_empty() {
            self.source_files.iter().map(|p| p.as_path()).collect()
        } else {
            self.stub_files.iter().map(|p| p.as_path()).collect()
        }
    }

    /// Check if package has type stubs.
    pub fn has_stubs(&self) -> bool {
        !self.stub_files.is_empty()
    }
}

/// Harvester for fetching Python packages via `uv`.
pub struct Harvester {
    /// Target directory for package installation
    target_dir: PathBuf,
}

impl Harvester {
    /// Create a new Harvester with the specified target directory.
    pub fn new<P: AsRef<Path>>(target_dir: P) -> Result<Self> {
        let target_dir = target_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&target_dir)?;
        Ok(Self { target_dir })
    }

    /// Create a Harvester with a temporary directory.
    pub fn temp() -> Result<Self> {
        let temp_dir = std::env::temp_dir().join("depyler-harvest");
        Self::new(temp_dir)
    }

    /// Fetch a package and its type stubs.
    ///
    /// This runs `uv pip install --target <dir> <package>` and then
    /// searches for .pyi stub files in the installed package.
    ///
    /// # Stub Package Strategy
    ///
    /// For packages without inline stubs (like `requests`), we also attempt
    /// to fetch `types-{package}` which contains typeshed stubs with complete
    /// type annotations. This provides accurate return types.
    pub fn fetch(&self, package: &str) -> Result<HarvestResult> {
        info!(package = %package, "Harvesting package");

        // First, try to fetch types-{package} for complete stubs
        let types_package = format!("types-{package}");
        let has_types_package = self.try_install_types_package(&types_package);

        // Then install the main package
        self.run_uv_install(package)?;

        // Find stub and source files
        let stub_files = self.find_files_by_extension(&self.target_dir, "pyi");
        let source_files = self.find_files_by_extension(&self.target_dir, "py");

        debug!(
            package = %package,
            types_package = has_types_package,
            stubs = stub_files.len(),
            sources = source_files.len(),
            "Harvest complete"
        );

        if stub_files.is_empty() && source_files.is_empty() {
            warn!(package = %package, "No Python files found in package");
        }

        Ok(HarvestResult {
            package: package.to_string(),
            root: self.target_dir.clone(),
            stub_files,
            source_files,
            has_types_package,
        })
    }

    /// Try to install a types-* stub package. Returns true if successful.
    fn try_install_types_package(&self, package: &str) -> bool {
        info!(package = %package, "Attempting to fetch stub package");

        let output = Command::new("uv")
            .args(["pip", "install", "--target", self.target_dir.to_str().unwrap_or("."), package])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                info!(package = %package, "Stub package installed successfully");
                true
            }
            Ok(_) => {
                debug!(package = %package, "Stub package not available");
                false
            }
            Err(e) => {
                debug!(package = %package, error = %e, "Failed to check stub package");
                false
            }
        }
    }

    /// Run `uv pip install --target <dir> <package>`.
    fn run_uv_install(&self, package: &str) -> Result<()> {
        let output = Command::new("uv")
            .args([
                "pip",
                "install",
                "--target",
                self.target_dir.to_str().unwrap_or("."),
                package,
            ])
            .output()
            .map_err(|e| {
                KnowledgeError::UvCommandFailed(format!(
                    "Failed to execute uv: {}. Is uv installed? Try: curl -LsSf https://astral.sh/uv/install.sh | sh",
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("No matching distribution") {
                return Err(KnowledgeError::PackageNotFound(package.to_string()));
            }
            return Err(KnowledgeError::UvCommandFailed(stderr.to_string()));
        }

        Ok(())
    }

    /// Find all files with a given extension in a directory tree.
    fn find_files_by_extension(&self, root: &Path, ext: &str) -> Vec<PathBuf> {
        WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().is_some_and(|x| x == ext))
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    /// Get the target directory path.
    pub fn target_dir(&self) -> &Path {
        &self.target_dir
    }

    /// Clean up the target directory.
    pub fn cleanup(&self) -> Result<()> {
        if self.target_dir.exists() {
            std::fs::remove_dir_all(&self.target_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_harvester_new() {
        let temp = TempDir::new().unwrap();
        let harvester = Harvester::new(temp.path()).unwrap();
        assert!(harvester.target_dir().exists());
    }

    #[test]
    fn test_harvest_result_all_files_prefers_stubs() {
        let result = HarvestResult {
            package: "test".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![PathBuf::from("a.pyi")],
            source_files: vec![PathBuf::from("b.py")],
            has_types_package: true,
        };
        assert!(result.has_stubs());
        assert_eq!(result.all_files().len(), 1);
        assert_eq!(result.all_files()[0], Path::new("a.pyi"));
    }

    #[test]
    fn test_harvest_result_fallback_to_sources() {
        let result = HarvestResult {
            package: "test".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![],
            source_files: vec![PathBuf::from("b.py")],
            has_types_package: false,
        };
        assert!(!result.has_stubs());
        assert_eq!(result.all_files().len(), 1);
        assert_eq!(result.all_files()[0], Path::new("b.py"));
    }

    #[test]
    fn test_harvest_result_empty_no_files() {
        let result = HarvestResult {
            package: "empty".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![],
            source_files: vec![],
            has_types_package: false,
        };
        assert!(!result.has_stubs());
        assert!(result.all_files().is_empty());
    }

    #[test]
    fn test_harvest_result_multiple_stubs() {
        let result = HarvestResult {
            package: "multi".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![
                PathBuf::from("a.pyi"),
                PathBuf::from("b.pyi"),
                PathBuf::from("c.pyi"),
            ],
            source_files: vec![PathBuf::from("d.py")],
            has_types_package: true,
        };
        assert!(result.has_stubs());
        // Should return stubs, not sources
        assert_eq!(result.all_files().len(), 3);
    }

    #[test]
    fn test_harvest_result_clone() {
        let result = HarvestResult {
            package: "pkg".to_string(),
            root: PathBuf::from("/tmp/test"),
            stub_files: vec![PathBuf::from("x.pyi")],
            source_files: vec![],
            has_types_package: true,
        };
        let cloned = result.clone();
        assert_eq!(cloned.package, "pkg");
        assert_eq!(cloned.root, PathBuf::from("/tmp/test"));
        assert_eq!(cloned.stub_files.len(), 1);
        assert!(cloned.has_types_package);
    }

    #[test]
    fn test_harvest_result_debug() {
        let result = HarvestResult {
            package: "test".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![],
            source_files: vec![],
            has_types_package: false,
        };
        let debug_str = format!("{result:?}");
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("HarvestResult"));
    }

    #[test]
    fn test_harvester_creates_directory() {
        let temp = TempDir::new().unwrap();
        let nested = temp.path().join("a").join("b").join("c");
        let harvester = Harvester::new(&nested).unwrap();
        assert!(harvester.target_dir().exists());
        assert!(nested.is_dir());
    }

    #[test]
    fn test_harvester_target_dir() {
        let temp = TempDir::new().unwrap();
        let harvester = Harvester::new(temp.path()).unwrap();
        assert_eq!(harvester.target_dir(), temp.path());
    }

    #[test]
    fn test_harvester_cleanup() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("harvest");
        let harvester = Harvester::new(&target).unwrap();
        assert!(target.exists());

        harvester.cleanup().unwrap();
        assert!(!target.exists());
    }

    #[test]
    fn test_harvester_cleanup_nonexistent_is_ok() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("nonexistent");
        // Create and immediately remove
        std::fs::create_dir_all(&target).unwrap();
        let harvester = Harvester::new(&target).unwrap();
        std::fs::remove_dir_all(&target).unwrap();

        // Cleanup on nonexistent path should succeed
        assert!(harvester.cleanup().is_ok());
    }

    #[test]
    fn test_harvester_find_files_by_extension() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create test files
        std::fs::write(root.join("a.pyi"), "").unwrap();
        std::fs::write(root.join("b.pyi"), "").unwrap();
        std::fs::write(root.join("c.py"), "").unwrap();
        std::fs::write(root.join("d.txt"), "").unwrap();

        let harvester = Harvester::new(root).unwrap();
        let pyi_files = harvester.find_files_by_extension(root, "pyi");
        assert_eq!(pyi_files.len(), 2);

        let py_files = harvester.find_files_by_extension(root, "py");
        assert_eq!(py_files.len(), 1);

        let txt_files = harvester.find_files_by_extension(root, "txt");
        assert_eq!(txt_files.len(), 1);
    }

    #[test]
    fn test_harvester_find_files_nested_dirs() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        let subdir = root.join("subpkg");
        std::fs::create_dir_all(&subdir).unwrap();
        std::fs::write(root.join("top.pyi"), "").unwrap();
        std::fs::write(subdir.join("nested.pyi"), "").unwrap();

        let harvester = Harvester::new(root).unwrap();
        let files = harvester.find_files_by_extension(root, "pyi");
        assert_eq!(files.len(), 2);
    }

    // ========================================================================
    // S9B7: Coverage tests for harvester
    // ========================================================================

    #[test]
    fn test_s9b7_harvest_result_multiple_sources_no_stubs() {
        let result = HarvestResult {
            package: "pkg".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![],
            source_files: vec![PathBuf::from("a.py"), PathBuf::from("b.py"), PathBuf::from("c.py")],
            has_types_package: false,
        };
        assert!(!result.has_stubs());
        assert_eq!(result.all_files().len(), 3);
    }

    #[test]
    fn test_s9b7_harvest_result_both_stubs_and_sources() {
        let result = HarvestResult {
            package: "dual".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![PathBuf::from("x.pyi"), PathBuf::from("y.pyi")],
            source_files: vec![PathBuf::from("x.py"), PathBuf::from("y.py")],
            has_types_package: true,
        };
        assert!(result.has_stubs());
        // Should prefer stubs
        let files = result.all_files();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.extension().unwrap() == "pyi"));
    }

    #[test]
    fn test_s9b7_harvester_target_dir_path() {
        let temp = TempDir::new().unwrap();
        let nested = temp.path().join("deep").join("path");
        let harvester = Harvester::new(&nested).unwrap();
        assert_eq!(harvester.target_dir(), nested.as_path());
    }

    #[test]
    fn test_s9b7_harvest_result_has_types_package_true() {
        let result = HarvestResult {
            package: "requests".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![PathBuf::from("__init__.pyi")],
            source_files: vec![],
            has_types_package: true,
        };
        assert!(result.has_types_package);
        assert!(result.has_stubs());
    }

    #[test]
    fn test_s9b7_find_files_no_matching_extension() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("file.rs"), "").unwrap();
        std::fs::write(temp.path().join("file.txt"), "").unwrap();
        let harvester = Harvester::new(temp.path()).unwrap();
        let pyi_files = harvester.find_files_by_extension(temp.path(), "pyi");
        assert!(pyi_files.is_empty());
    }

    #[test]
    fn test_s9b7_cleanup_already_removed() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("to_clean");
        let harvester = Harvester::new(&target).unwrap();
        // Remove first
        std::fs::remove_dir_all(&target).unwrap();
        // Second cleanup should be fine
        assert!(harvester.cleanup().is_ok());
    }

    #[test]
    fn test_harvester_find_files_empty_dir() {
        let temp = TempDir::new().unwrap();
        let harvester = Harvester::new(temp.path()).unwrap();
        let files = harvester.find_files_by_extension(temp.path(), "pyi");
        assert!(files.is_empty());
    }

    #[test]
    fn test_s12_harvester_temp_creates_dir() {
        let harvester = Harvester::temp().unwrap();
        assert!(harvester.target_dir().exists());
        // cleanup
        harvester.cleanup().unwrap();
    }

    #[test]
    fn test_s12_harvester_temp_target_path_contains_depyler() {
        let harvester = Harvester::temp().unwrap();
        let path_str = harvester.target_dir().to_string_lossy().to_string();
        assert!(
            path_str.contains("depyler-harvest"),
            "Expected path to contain 'depyler-harvest', got: {}",
            path_str
        );
        harvester.cleanup().unwrap();
    }

    #[test]
    fn test_s12_harvest_result_display_package_name() {
        let result = HarvestResult {
            package: "requests".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![],
            source_files: vec![],
            has_types_package: false,
        };
        assert_eq!(result.package, "requests");
    }

    #[test]
    fn test_s12_harvest_result_root_path() {
        let result = HarvestResult {
            package: "test".to_string(),
            root: PathBuf::from("/custom/path"),
            stub_files: vec![],
            source_files: vec![],
            has_types_package: false,
        };
        assert_eq!(result.root, PathBuf::from("/custom/path"));
    }

    #[test]
    fn test_s12_harvester_new_nested_dir() {
        let temp = TempDir::new().unwrap();
        let nested = temp.path().join("a").join("b").join("c");
        let harvester = Harvester::new(&nested).unwrap();
        assert!(harvester.target_dir().exists());
        assert_eq!(harvester.target_dir(), &nested);
    }

    #[test]
    fn test_s12_harvester_find_multiple_extensions() {
        let temp = TempDir::new().unwrap();
        let harvester = Harvester::new(temp.path()).unwrap();

        // Create files with different extensions
        std::fs::write(temp.path().join("a.pyi"), "# stub").unwrap();
        std::fs::write(temp.path().join("b.pyi"), "# stub2").unwrap();
        std::fs::write(temp.path().join("c.py"), "# source").unwrap();
        std::fs::write(temp.path().join("d.txt"), "text").unwrap();

        let pyi_files = harvester.find_files_by_extension(temp.path(), "pyi");
        assert_eq!(pyi_files.len(), 2);
        let py_files = harvester.find_files_by_extension(temp.path(), "py");
        assert_eq!(py_files.len(), 1);
        let txt_files = harvester.find_files_by_extension(temp.path(), "txt");
        assert_eq!(txt_files.len(), 1);
    }

    #[test]
    fn test_s12_harvester_cleanup_then_recreate() {
        let temp = TempDir::new().unwrap();
        let dir = temp.path().join("harvest-dir");
        let harvester = Harvester::new(&dir).unwrap();
        assert!(harvester.target_dir().exists());

        harvester.cleanup().unwrap();
        assert!(!dir.exists());

        // Can recreate
        let harvester2 = Harvester::new(&dir).unwrap();
        assert!(harvester2.target_dir().exists());
    }

    #[test]
    fn test_s12_harvest_result_stubs_and_sources_counts() {
        let result = HarvestResult {
            package: "numpy".to_string(),
            root: PathBuf::from("/tmp"),
            stub_files: vec![PathBuf::from("numpy/__init__.pyi"), PathBuf::from("numpy/core.pyi")],
            source_files: vec![
                PathBuf::from("numpy/__init__.py"),
                PathBuf::from("numpy/core.py"),
                PathBuf::from("numpy/linalg.py"),
            ],
            has_types_package: true,
        };
        assert!(result.has_stubs());
        assert_eq!(result.stub_files.len(), 2);
        assert_eq!(result.source_files.len(), 3);
        // all_files returns stubs when available
        assert_eq!(result.all_files().len(), 2);
    }

    // ===== Session 12 Batch 30: Additional harvester tests =====

    #[test]
    fn test_s12_harvest_result_no_stubs() {
        let result = HarvestResult {
            package: "simple".to_string(),
            root: PathBuf::from("/tmp/simple"),
            stub_files: vec![],
            source_files: vec![
                PathBuf::from("simple/__init__.py"),
                PathBuf::from("simple/core.py"),
            ],
            has_types_package: false,
        };
        assert!(!result.has_stubs());
        // all_files falls back to source files
        assert_eq!(result.all_files().len(), 2);
    }

    #[test]
    fn test_s12_harvest_result_empty() {
        let result = HarvestResult {
            package: "empty".to_string(),
            root: PathBuf::from("/tmp/empty"),
            stub_files: vec![],
            source_files: vec![],
            has_types_package: false,
        };
        assert!(!result.has_stubs());
        assert!(result.all_files().is_empty());
    }

    #[test]
    fn test_s12_harvester_new() {
        let dir = tempfile::tempdir().unwrap();
        let harvester = Harvester::new(dir.path());
        assert!(harvester.is_ok());
        assert_eq!(harvester.unwrap().target_dir(), dir.path());
    }

    #[test]
    fn test_s12_harvester_temp() {
        let harvester = Harvester::temp();
        assert!(harvester.is_ok());
        let h = harvester.unwrap();
        assert!(h.target_dir().exists());
    }

    #[test]
    fn test_s12_harvester_cleanup() {
        let harvester = Harvester::temp().unwrap();
        let dir = harvester.target_dir().to_path_buf();
        assert!(dir.exists());
        let _ = harvester.cleanup();
        assert!(!dir.exists());
    }
}
