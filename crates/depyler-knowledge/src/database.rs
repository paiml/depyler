//! Database: Parquet storage for type facts.
//!
//! Uses Apache Parquet via the `arrow` crate for efficient columnar storage
//! and fast queries on type signatures.

use crate::{KnowledgeError, Result, TypeFact, TypeFactKind};
use arrow::array::{ArrayRef, RecordBatch, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, info};

/// Type database backed by Parquet storage.
pub struct TypeDatabase {
    /// Path to the Parquet file
    path: PathBuf,
    /// Cached schema
    schema: Arc<Schema>,
}

impl TypeDatabase {
    /// Create a new TypeDatabase at the specified path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let schema = Arc::new(Self::create_schema());
        Ok(Self { path, schema })
    }

    /// Create a TypeDatabase using a temporary file.
    pub fn temp() -> Result<Self> {
        let path = std::env::temp_dir().join("depyler-types.parquet");
        Self::new(path)
    }

    /// Create the Arrow schema for TypeFact.
    fn create_schema() -> Schema {
        Schema::new(vec![
            Field::new("module", DataType::Utf8, false),
            Field::new("symbol", DataType::Utf8, false),
            Field::new("kind", DataType::Utf8, false),
            Field::new("signature", DataType::Utf8, false),
            Field::new("return_type", DataType::Utf8, false),
        ])
    }

    /// Write type facts to the database.
    pub fn write(&self, facts: &[TypeFact]) -> Result<()> {
        info!(path = %self.path.display(), count = facts.len(), "Writing type facts");

        let batch = self.facts_to_batch(facts)?;

        let file = File::create(&self.path)?;
        let mut writer = ArrowWriter::try_new(file, self.schema.clone(), None)?;
        writer.write(&batch)?;
        writer.close()?;

        debug!(path = %self.path.display(), "Write complete");
        Ok(())
    }

    /// Convert TypeFacts to an Arrow RecordBatch.
    fn facts_to_batch(&self, facts: &[TypeFact]) -> Result<RecordBatch> {
        let modules: Vec<&str> = facts.iter().map(|f| f.module.as_str()).collect();
        let symbols: Vec<&str> = facts.iter().map(|f| f.symbol.as_str()).collect();
        let kinds: Vec<String> = facts.iter().map(|f| f.kind.to_string()).collect();
        let signatures: Vec<&str> = facts.iter().map(|f| f.signature.as_str()).collect();
        let return_types: Vec<&str> = facts.iter().map(|f| f.return_type.as_str()).collect();

        let columns: Vec<ArrayRef> = vec![
            Arc::new(StringArray::from(modules)),
            Arc::new(StringArray::from(symbols)),
            Arc::new(StringArray::from(
                kinds.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            )),
            Arc::new(StringArray::from(signatures)),
            Arc::new(StringArray::from(return_types)),
        ];

        RecordBatch::try_new(self.schema.clone(), columns)
            .map_err(|e| KnowledgeError::DatabaseError(e.to_string()))
    }

    /// Read all type facts from the database.
    pub fn read_all(&self) -> Result<Vec<TypeFact>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let reader = builder.build()?;

        let mut facts = Vec::new();
        for batch in reader {
            let batch = batch?;
            let batch_facts = self.batch_to_facts(&batch)?;
            facts.extend(batch_facts);
        }

        debug!(path = %self.path.display(), count = facts.len(), "Read type facts");
        Ok(facts)
    }

    /// Convert an Arrow RecordBatch to TypeFacts.
    fn batch_to_facts(&self, batch: &RecordBatch) -> Result<Vec<TypeFact>> {
        let modules = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| KnowledgeError::DatabaseError("Invalid module column".to_string()))?;

        let symbols = batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| KnowledgeError::DatabaseError("Invalid symbol column".to_string()))?;

        let kinds = batch
            .column(2)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| KnowledgeError::DatabaseError("Invalid kind column".to_string()))?;

        let signatures = batch
            .column(3)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| KnowledgeError::DatabaseError("Invalid signature column".to_string()))?;

        let return_types = batch
            .column(4)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                KnowledgeError::DatabaseError("Invalid return_type column".to_string())
            })?;

        let mut facts = Vec::with_capacity(batch.num_rows());
        for i in 0..batch.num_rows() {
            let kind_str = kinds.value(i);
            let kind: TypeFactKind = kind_str.parse()?;

            facts.push(TypeFact {
                module: modules.value(i).to_string(),
                symbol: symbols.value(i).to_string(),
                kind,
                signature: signatures.value(i).to_string(),
                return_type: return_types.value(i).to_string(),
            });
        }

        Ok(facts)
    }

    /// Find a signature by module and symbol name.
    pub fn find_signature(&self, module: &str, symbol: &str) -> Option<String> {
        self.read_all()
            .ok()?
            .into_iter()
            .find(|f| f.module == module && f.symbol == symbol)
            .map(|f| f.signature)
    }

    /// Query facts by module prefix.
    pub fn query_by_module(&self, prefix: &str) -> Result<Vec<TypeFact>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|f| f.module.starts_with(prefix))
            .collect())
    }

    /// Get the database file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if the database file exists.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get the file size in bytes.
    pub fn size_bytes(&self) -> Result<u64> {
        let metadata = std::fs::metadata(&self.path)?;
        Ok(metadata.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_roundtrip() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.parquet");
        let db = TypeDatabase::new(&db_path).unwrap();

        let facts = vec![
            TypeFact::function("requests", "get", "(url: str) -> Response", "Response"),
            TypeFact::class("requests.models", "Response"),
            TypeFact::method(
                "requests.models",
                "Response",
                "json",
                "(self) -> dict",
                "dict",
            ),
        ];

        db.write(&facts).unwrap();
        assert!(db.exists());

        let loaded = db.read_all().unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].module, "requests");
        assert_eq!(loaded[0].symbol, "get");
        assert_eq!(loaded[1].kind, TypeFactKind::Class);
        assert_eq!(loaded[2].symbol, "Response.json");
    }

    #[test]
    fn test_find_signature() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.parquet");
        let db = TypeDatabase::new(&db_path).unwrap();

        let facts = vec![TypeFact::function(
            "requests",
            "get",
            "(url: str, **kwargs) -> Response",
            "Response",
        )];

        db.write(&facts).unwrap();

        let sig = db.find_signature("requests", "get");
        assert!(sig.is_some());
        assert!(sig.unwrap().contains("url: str"));

        let missing = db.find_signature("requests", "post");
        assert!(missing.is_none());
    }

    #[test]
    fn test_query_by_module() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.parquet");
        let db = TypeDatabase::new(&db_path).unwrap();

        let facts = vec![
            TypeFact::function("requests.api", "get", "(url: str) -> Response", "Response"),
            TypeFact::function("requests.api", "post", "(url: str) -> Response", "Response"),
            TypeFact::class("requests.models", "Response"),
        ];

        db.write(&facts).unwrap();

        let api_facts = db.query_by_module("requests.api").unwrap();
        assert_eq!(api_facts.len(), 2);

        let all_requests = db.query_by_module("requests").unwrap();
        assert_eq!(all_requests.len(), 3);
    }
}
