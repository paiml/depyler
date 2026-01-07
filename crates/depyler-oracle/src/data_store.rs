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
        std::fs::create_dir_all(parent).map_err(crate::OracleError::Io)?;
    }

    let batch = dataset_to_arrow(dataset)
        .map_err(|e| crate::OracleError::Model(format!("Arrow conversion failed: {e}")))?;

    let file = File::create(path).map_err(crate::OracleError::Io)?;
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
    fn test_default_corpus_path() {
        assert!(!DEFAULT_CORPUS_PATH.is_empty());
        assert!(DEFAULT_CORPUS_PATH.ends_with(".parquet"));
    }

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
    fn test_roundtrip_all_categories() {
        let mut original = TrainingDataset::new();

        // Add samples for all categories
        original.add(TrainingSample::with_fix(
            "error[E0308]: type mismatch",
            ErrorCategory::TypeMismatch,
            "Fix type",
        ));
        original.add(TrainingSample::with_fix(
            "error[E0502]: borrow error",
            ErrorCategory::BorrowChecker,
            "Fix borrow",
        ));
        original.add(TrainingSample::with_fix(
            "error[E0432]: missing import",
            ErrorCategory::MissingImport,
            "Add import",
        ));
        original.add(TrainingSample::with_fix(
            "error: syntax error",
            ErrorCategory::SyntaxError,
            "Fix syntax",
        ));
        original.add(TrainingSample::with_fix(
            "error[E0597]: lifetime error",
            ErrorCategory::LifetimeError,
            "Fix lifetime",
        ));
        original.add(TrainingSample::with_fix(
            "error[E0277]: trait bound",
            ErrorCategory::TraitBound,
            "Implement trait",
        ));
        original.add(TrainingSample::with_fix(
            "error: other error",
            ErrorCategory::Other,
            "Fix error",
        ));

        let batch = dataset_to_arrow(&original).unwrap();
        let restored = arrow_to_dataset(&batch);

        assert_eq!(original.len(), restored.len());
        assert_eq!(restored.len(), 7);
    }

    #[test]
    fn test_dataset_to_arrow_empty() {
        let dataset = TrainingDataset::new();
        let batch = dataset_to_arrow(&dataset).unwrap();
        assert_eq!(batch.num_rows(), 0);
        assert_eq!(batch.num_columns(), 3);
    }

    #[test]
    fn test_dataset_to_arrow_schema() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new("error", ErrorCategory::TypeMismatch));

        let batch = dataset_to_arrow(&dataset).unwrap();
        let schema = batch.schema();

        assert_eq!(schema.fields().len(), 3);
        assert_eq!(schema.field(0).name(), "message");
        assert_eq!(schema.field(1).name(), "category");
        assert_eq!(schema.field(2).name(), "fix");
    }

    #[test]
    fn test_dataset_to_arrow_data_types() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new("error", ErrorCategory::TypeMismatch));

        let batch = dataset_to_arrow(&dataset).unwrap();
        let schema = batch.schema();

        assert_eq!(*schema.field(0).data_type(), DataType::Utf8);
        assert_eq!(*schema.field(1).data_type(), DataType::UInt8);
        assert_eq!(*schema.field(2).data_type(), DataType::Utf8);
    }

    #[test]
    fn test_arrow_to_dataset_with_none_fix() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new(
            "error without fix",
            ErrorCategory::TypeMismatch,
        ));

        let batch = dataset_to_arrow(&dataset).unwrap();
        let restored = arrow_to_dataset(&batch);

        assert_eq!(restored.len(), 1);
        // Fix should be empty string which converts to None
    }

    #[test]
    fn test_arrow_to_dataset_with_fix() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::with_fix(
            "error with fix",
            ErrorCategory::TypeMismatch,
            "the fix",
        ));

        let batch = dataset_to_arrow(&dataset).unwrap();
        let restored = arrow_to_dataset(&batch);

        assert_eq!(restored.len(), 1);
        let sample = &restored.samples()[0];
        assert_eq!(sample.fix.as_deref(), Some("the fix"));
    }

    #[test]
    fn test_arrow_to_dataset_category_mapping() {
        // Test that all category indices map correctly
        let categories = vec![
            (0u8, ErrorCategory::TypeMismatch),
            (1u8, ErrorCategory::BorrowChecker),
            (2u8, ErrorCategory::MissingImport),
            (3u8, ErrorCategory::SyntaxError),
            (4u8, ErrorCategory::LifetimeError),
            (5u8, ErrorCategory::TraitBound),
        ];

        for (idx, category) in categories {
            let mut dataset = TrainingDataset::new();
            dataset.add(TrainingSample::new("error", category));

            let batch = dataset_to_arrow(&dataset).unwrap();
            let categories_arr = batch
                .column(1)
                .as_any()
                .downcast_ref::<UInt8Array>()
                .unwrap();

            assert_eq!(categories_arr.value(0), idx);

            let restored = arrow_to_dataset(&batch);
            assert_eq!(restored.samples()[0].category, category);
        }
    }

    #[test]
    fn test_arrow_to_dataset_unknown_category() {
        // Create batch with category index > 5
        let messages: ArrayRef = Arc::new(StringArray::from(vec!["error"]));
        let categories: ArrayRef = Arc::new(UInt8Array::from(vec![99u8])); // Unknown
        let fixes: ArrayRef = Arc::new(StringArray::from(vec![""]));

        let schema = Schema::new(vec![
            Field::new("message", DataType::Utf8, false),
            Field::new("category", DataType::UInt8, false),
            Field::new("fix", DataType::Utf8, true),
        ]);

        let batch =
            RecordBatch::try_new(Arc::new(schema), vec![messages, categories, fixes]).unwrap();

        let restored = arrow_to_dataset(&batch);
        assert_eq!(restored.samples()[0].category, ErrorCategory::Other);
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

    #[test]
    fn test_save_corpus_creates_parent_dir() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new("error", ErrorCategory::TypeMismatch));

        let temp_dir = std::env::temp_dir().join("oracle_test_subdir");
        let temp_path = temp_dir.join("corpus.parquet");

        // Ensure dir doesn't exist
        let _ = std::fs::remove_dir_all(&temp_dir);

        let result = save_corpus(&dataset, &temp_path);
        assert!(result.is_ok());
        assert!(temp_path.exists());

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_load_corpus_nonexistent() {
        let path = Path::new("/nonexistent/path/corpus.parquet");
        let result = load_corpus(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_save_load_empty_dataset() {
        let dataset = TrainingDataset::new();

        let temp_path = std::env::temp_dir().join("test_empty_corpus.parquet");
        save_corpus(&dataset, &temp_path).unwrap();

        // Loading empty parquet returns error (dataset is empty)
        let result = load_corpus(&temp_path);
        assert!(result.is_err());

        // Cleanup
        let _ = std::fs::remove_file(temp_path);
    }

    #[test]
    fn test_save_load_large_dataset() {
        let mut dataset = TrainingDataset::new();
        for i in 0..100 {
            dataset.add(TrainingSample::with_fix(
                &format!("error[E{:04}]: test error {}", i, i),
                ErrorCategory::TypeMismatch,
                &format!("Fix {}", i),
            ));
        }

        let temp_path = std::env::temp_dir().join("test_large_corpus.parquet");
        save_corpus(&dataset, &temp_path).unwrap();

        let loaded = load_corpus(&temp_path).unwrap();
        assert_eq!(loaded.len(), 100);

        // Cleanup
        let _ = std::fs::remove_file(temp_path);
    }

    #[test]
    fn test_multiple_messages_preservation() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::with_fix("msg1", ErrorCategory::TypeMismatch, "fix1"));
        dataset.add(TrainingSample::with_fix("msg2", ErrorCategory::BorrowChecker, "fix2"));
        dataset.add(TrainingSample::with_fix("msg3", ErrorCategory::MissingImport, "fix3"));

        let batch = dataset_to_arrow(&dataset).unwrap();
        let messages = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        assert_eq!(messages.value(0), "msg1");
        assert_eq!(messages.value(1), "msg2");
        assert_eq!(messages.value(2), "msg3");
    }
}
