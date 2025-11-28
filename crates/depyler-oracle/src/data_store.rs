//! Alimentar-based data storage for oracle training corpus.
//!
//! Uses alimentar for:
//! - Parquet storage (efficient, versioned)
//! - Drift detection (retraining triggers)
//! - Registry integration (future: HuggingFace publishing)

use crate::classifier::ErrorCategory;
use crate::training::{TrainingDataset, TrainingSample};
use alimentar::{ArrowDataset, Dataset};
use arrow::array::{Array, ArrayRef, RecordBatch, StringArray, UInt8Array};
use arrow::datatypes::{DataType, Field, Schema};
use std::path::Path;
use std::sync::Arc;

/// Default corpus path
pub const DEFAULT_CORPUS_PATH: &str = "data/oracle_corpus.parquet";

/// Convert TrainingDataset to Arrow RecordBatch for Parquet storage.
pub fn dataset_to_arrow(dataset: &TrainingDataset) -> Result<RecordBatch, arrow::error::ArrowError> {
    let samples = dataset.samples();

    let messages: Vec<&str> = samples.iter().map(|s| s.message.as_str()).collect();
    let categories: Vec<u8> = samples.iter().map(|s| s.category.index() as u8).collect();
    let fixes: Vec<&str> = samples
        .iter()
        .map(|s| s.fix.as_deref().unwrap_or(""))
        .collect();

    let schema = Schema::new(vec![
        Field::new("message", DataType::Utf8, false),
        Field::new("category", DataType::UInt8, false),
        Field::new("fix", DataType::Utf8, true),
    ]);

    let message_array: ArrayRef = Arc::new(StringArray::from(messages));
    let category_array: ArrayRef = Arc::new(UInt8Array::from(categories));
    let fix_array: ArrayRef = Arc::new(StringArray::from(fixes));

    RecordBatch::try_new(
        Arc::new(schema),
        vec![message_array, category_array, fix_array],
    )
}

/// Convert Arrow RecordBatch back to TrainingDataset.
pub fn arrow_to_dataset(batch: &RecordBatch) -> TrainingDataset {
    let mut dataset = TrainingDataset::new();

    let messages = batch
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("message column");
    let categories = batch
        .column(1)
        .as_any()
        .downcast_ref::<UInt8Array>()
        .expect("category column");
    let fixes = batch
        .column(2)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("fix column");

    for i in 0..batch.num_rows() {
        let message = messages.value(i);
        let category_idx = categories.value(i) as usize;
        let fix = if fixes.is_null(i) || fixes.value(i).is_empty() {
            None
        } else {
            Some(fixes.value(i).to_string())
        };

        let category = match category_idx {
            0 => ErrorCategory::TypeMismatch,
            1 => ErrorCategory::BorrowChecker,
            2 => ErrorCategory::MissingImport,
            3 => ErrorCategory::SyntaxError,
            4 => ErrorCategory::LifetimeError,
            5 => ErrorCategory::TraitBound,
            _ => ErrorCategory::Other,
        };

        if let Some(fix_str) = fix {
            dataset.add(TrainingSample::with_fix(message, category, &fix_str));
        } else {
            dataset.add(TrainingSample::new(message, category));
        }
    }

    dataset
}

/// Save training corpus to Parquet file.
pub fn save_corpus(dataset: &TrainingDataset, path: &Path) -> crate::Result<()> {
    use parquet::arrow::ArrowWriter;
    use std::fs::File;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| crate::OracleError::Io(e))?;
    }

    let batch = dataset_to_arrow(dataset)
        .map_err(|e| crate::OracleError::Model(format!("Arrow conversion failed: {e}")))?;

    let file = File::create(path).map_err(|e| crate::OracleError::Io(e))?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), None)
        .map_err(|e| crate::OracleError::Model(format!("Parquet writer failed: {e}")))?;

    writer
        .write(&batch)
        .map_err(|e| crate::OracleError::Model(format!("Write failed: {e}")))?;
    writer
        .close()
        .map_err(|e| crate::OracleError::Model(format!("Close failed: {e}")))?;

    Ok(())
}

/// Load training corpus from Parquet file using alimentar.
pub fn load_corpus(path: &Path) -> crate::Result<TrainingDataset> {
    let arrow_dataset = ArrowDataset::from_parquet(path.to_str().unwrap_or(""))
        .map_err(|e| crate::OracleError::Model(format!("Failed to load parquet: {e}")))?;

    // Collect all batches
    let mut dataset = TrainingDataset::new();
    for batch in arrow_dataset.iter() {
        let partial = arrow_to_dataset(&batch);
        for sample in partial.samples() {
            dataset.add(sample.clone());
        }
    }

    Ok(dataset)
}

/// Load corpus from Parquet if exists, otherwise use hardcoded and save.
pub fn load_or_create_corpus() -> crate::Result<TrainingDataset> {
    let path = Path::new(DEFAULT_CORPUS_PATH);

    if path.exists() {
        load_corpus(path)
    } else {
        // Generate from hardcoded + synthetic
        let dataset = crate::synthetic::generate_synthetic_corpus();

        // Save for next time
        if let Err(e) = save_corpus(&dataset, path) {
            eprintln!("Warning: Failed to cache corpus: {e}");
        }

        Ok(dataset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let mut original = TrainingDataset::new();
        original.add(TrainingSample::with_fix(
            "error[E0308]: type mismatch",
            ErrorCategory::TypeMismatch,
            "Fix the type",
        ));
        original.add(TrainingSample::new(
            "error[E0277]: trait not satisfied",
            ErrorCategory::TraitBound,
        ));

        let batch = dataset_to_arrow(&original).unwrap();
        let restored = arrow_to_dataset(&batch);

        assert_eq!(original.len(), restored.len());
    }

    #[test]
    fn test_save_load() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::with_fix(
            "error[E0308]: expected i32",
            ErrorCategory::TypeMismatch,
            "Convert type",
        ));

        let temp_path = std::env::temp_dir().join("test_oracle_corpus.parquet");
        save_corpus(&dataset, &temp_path).unwrap();

        let loaded = load_corpus(&temp_path).unwrap();
        assert_eq!(dataset.len(), loaded.len());

        // Cleanup
        let _ = std::fs::remove_file(temp_path);
    }
}
