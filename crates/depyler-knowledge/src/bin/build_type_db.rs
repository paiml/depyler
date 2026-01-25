//! CLI tool for building the Sovereign Type Database.
//!
//! # Usage
//!
//! ```bash
//! # Build type database for specific packages
//! cargo run -p depyler-knowledge --bin build-type-db -- \
//!     --packages "requests,numpy,pandas" \
//!     --output crates/depyler-core/src/data/stdlib_types.parquet
//!
//! # Query the database
//! cargo run -p depyler-knowledge --bin build-type-db -- \
//!     --query requests.get \
//!     --db types.parquet
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use depyler_knowledge::{Extractor, Harvester, TypeDatabase, TypeFact, TypeQuery};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "build-type-db")]
#[command(about = "Build and query the Sovereign Type Database")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a type database from Python packages
    Build {
        /// Comma-separated list of packages to harvest
        #[arg(short, long)]
        packages: String,

        /// Output path for the Parquet database
        #[arg(short, long, default_value = "types.parquet")]
        output: PathBuf,

        /// Target directory for package installation (temp by default)
        #[arg(short, long)]
        target: Option<PathBuf>,
    },

    /// Query the type database
    Query {
        /// Database file path
        #[arg(short, long, default_value = "types.parquet")]
        db: PathBuf,

        /// Symbol to look up (e.g., "requests.get")
        symbol: String,
    },

    /// Show database statistics
    Stats {
        /// Database file path
        #[arg(short, long, default_value = "types.parquet")]
        db: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Commands::Build {
            packages,
            output,
            target,
        } => {
            build_database(&packages, &output, target.as_deref())?;
        }
        Commands::Query { db, symbol } => {
            query_database(&db, &symbol)?;
        }
        Commands::Stats { db } => {
            show_stats(&db)?;
        }
    }

    Ok(())
}

fn build_database(
    packages: &str,
    output: &PathBuf,
    target: Option<&std::path::Path>,
) -> Result<()> {
    let package_list: Vec<&str> = packages.split(',').map(|s| s.trim()).collect();
    info!(packages = ?package_list, "Building type database");

    let harvester = create_harvester(target)?;
    let extractor = Extractor::new();
    let mut all_facts: Vec<TypeFact> = Vec::new();

    for package in &package_list {
        let facts = harvest_package(&harvester, &extractor, package);
        all_facts.extend(facts);
    }

    write_database(output, &all_facts)
}

fn create_harvester(target: Option<&std::path::Path>) -> Result<Harvester> {
    match target {
        Some(dir) => Ok(Harvester::new(dir)?),
        None => Ok(Harvester::temp()?),
    }
}

fn harvest_package(harvester: &Harvester, extractor: &Extractor, package: &str) -> Vec<TypeFact> {
    info!(package = %package, "Harvesting package");

    match harvester.fetch(package) {
        Ok(result) => {
            info!(
                package = %package,
                stubs = result.stub_files.len(),
                sources = result.source_files.len(),
                "Package harvested"
            );
            extract_facts_from_result(extractor, &result)
        }
        Err(e) => {
            tracing::error!(package = %package, error = %e, "Failed to harvest package");
            Vec::new()
        }
    }
}

fn extract_facts_from_result(
    extractor: &Extractor,
    result: &depyler_knowledge::HarvestResult,
) -> Vec<TypeFact> {
    let mut facts = Vec::new();
    for file in result.all_files() {
        let module = derive_module_name(file, &result.root);
        match extractor.extract_file(file, &module) {
            Ok(extracted) => {
                info!(file = %file.display(), facts = extracted.len(), "Extracted facts");
                facts.extend(extracted);
            }
            Err(e) => {
                tracing::warn!(file = %file.display(), error = %e, "Failed to extract from file");
            }
        }
    }
    facts
}

fn write_database(output: &PathBuf, facts: &[TypeFact]) -> Result<()> {
    let db = TypeDatabase::new(output)?;
    db.write(facts)?;

    let size = db.size_bytes()?;
    info!(output = %output.display(), facts = facts.len(), size_kb = size / 1024, "Database written");

    println!("\n✅ Type database built successfully!");
    println!("   Facts: {}", facts.len());
    println!("   Size:  {} KB", size / 1024);
    println!("   Path:  {}", output.display());

    Ok(())
}

fn query_database(db_path: &PathBuf, symbol: &str) -> Result<()> {
    let mut query = TypeQuery::new(db_path)?;

    // Parse symbol into module and name
    let parts: Vec<&str> = symbol.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid symbol format. Expected: module.symbol (e.g., requests.get)");
    }

    let (name, module) = (parts[0], parts[1]);

    match query.find_fact(module, name) {
        Ok(fact) => {
            println!("\n📋 {}.{}", fact.module, fact.symbol);
            println!("   Kind:      {:?}", fact.kind);
            println!("   Signature: {}", fact.signature);
            println!("   Returns:   {}", fact.return_type);
        }
        Err(e) => {
            println!("\n❌ Symbol not found: {}", e);

            // Suggest similar symbols
            let results = query.search(name)?;
            if !results.is_empty() {
                println!("\n💡 Did you mean:");
                for result in results.iter().take(5) {
                    println!("   - {}.{}", result.module, result.symbol);
                }
            }
        }
    }

    Ok(())
}

fn show_stats(db_path: &PathBuf) -> Result<()> {
    let mut query = TypeQuery::new(db_path)?;
    query.warm_cache()?;

    let db = TypeDatabase::new(db_path)?;
    let size = db.size_bytes()?;
    let count = query.count();

    println!("\n📊 Type Database Statistics");
    println!("   Path:      {}", db_path.display());
    println!("   Size:      {} KB", size / 1024);
    println!("   Facts:     {}", count);

    Ok(())
}

/// Derive a module name from a file path relative to the package root.
///
/// Normalizes stub package names by removing `-stubs` suffix.
/// Example: `requests-stubs/api.pyi` -> `requests.api`
fn derive_module_name(file: &std::path::Path, root: &std::path::Path) -> String {
    file.strip_prefix(root)
        .ok()
        .and_then(|p| p.to_str())
        .map(|s| {
            let module = s
                .trim_end_matches(".pyi")
                .trim_end_matches(".py")
                .replace(['/', '\\'], ".")
                .trim_start_matches('.')
                .to_string();

            // Normalize stub package names: requests-stubs -> requests
            if let Some(pos) = module.find("-stubs") {
                let base = &module[..pos];
                let rest = &module[pos + 6..]; // Skip "-stubs"
                format!("{base}{rest}")
            } else {
                module
            }
        })
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_module_name() {
        let root = PathBuf::from("/tmp/harvest");
        let file = PathBuf::from("/tmp/harvest/requests/api.pyi");
        assert_eq!(derive_module_name(&file, &root), "requests.api");

        let init_file = PathBuf::from("/tmp/harvest/requests/__init__.pyi");
        assert_eq!(derive_module_name(&init_file, &root), "requests.__init__");
    }

    #[test]
    fn test_derive_module_name_normalizes_stubs() {
        let root = PathBuf::from("/tmp/harvest");

        // Stub package name should be normalized
        let stub_file = PathBuf::from("/tmp/harvest/requests-stubs/api.pyi");
        assert_eq!(derive_module_name(&stub_file, &root), "requests.api");

        let stub_model = PathBuf::from("/tmp/harvest/requests-stubs/models.pyi");
        assert_eq!(derive_module_name(&stub_model, &root), "requests.models");

        let stub_init = PathBuf::from("/tmp/harvest/requests-stubs/__init__.pyi");
        assert_eq!(derive_module_name(&stub_init, &root), "requests.__init__");
    }
}
