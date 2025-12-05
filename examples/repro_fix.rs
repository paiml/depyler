#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn combine_paths(base: String, suffix: String) -> String {
    std::path::PathBuf::from(base)
        .join(suffix)
        .to_string_lossy()
        .to_string()
}
