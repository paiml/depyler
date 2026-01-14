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
            .args([
                "pip",
                "install",
                "--target",
                self.target_dir.to_str().unwrap_or("."),
                package,
            ])
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
}
