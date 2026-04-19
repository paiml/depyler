fn main() {
    // Contracts enforcement (CB-1208) — migrated from archived provable-contracts
    // to aprender-contracts (APR-MONO, paiml/aprender#701).
    // Note: workspace package is "depyler-workspace" but contracts dir is "depyler"
    let contracts_dir = std::path::Path::new("../aprender/contracts");
    let pkg = "depyler";
    let binding = contracts_dir.join(pkg).join("binding.yaml");
    if binding.exists() {
        println!("cargo:rerun-if-changed={}", binding.display());
        // Read binding and set CONTRACT_* env vars for #[contract] macro
        let content = std::fs::read_to_string(&binding).unwrap_or_default();
        let count = content
            .lines()
            .filter(|l| l.trim().starts_with("status:") && l.contains("implemented"))
            .count();
        println!("cargo:warning=[contract] AllImplemented: {count} implemented bindings");
    }
}
