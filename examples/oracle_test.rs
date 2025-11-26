// Integration test for depyler-oracle API
use depyler_oracle::{NgramFixPredictor, TrainingDataset};

fn main() {
    println!("Testing depyler-oracle API...\n");

    // 1. Create an NgramFixPredictor
    let mut predictor = NgramFixPredictor::new();
    println!("✓ Created NgramFixPredictor");

    // 2. Train with rustc defaults
    let dataset = TrainingDataset::with_rustc_defaults();
    println!("✓ Created TrainingDataset with {} samples", dataset.len());

    predictor.train(&dataset);
    println!("✓ Trained predictor\n");

    // 3. Predict fixes for type mismatch error
    let error_msg = "error[E0308]: expected i32, found &str";
    let predictions = predictor.predict(error_msg, 3);
    
    println!("Predictions for: \"{}\"", error_msg);
    println!("─────────────────────────────────────────");
    
    if predictions.is_empty() {
        println!("⚠ No predictions returned");
    } else {
        for (i, pred) in predictions.iter().enumerate() {
            println!("  {}. {} (confidence: {:.2}%)", 
                i + 1, 
                pred.fix_suggestion,
                pred.confidence * 100.0
            );
            
            // 4. Verify confidence is valid (0.0 to 1.0)
            assert!(pred.confidence >= 0.0 && pred.confidence <= 1.0, 
                "Invalid confidence: {}", pred.confidence);
        }
        println!("\n✓ All confidence scores in valid range [0.0, 1.0]");
    }

    // Test another error type
    let error_msg2 = "error[E0425]: cannot find value `foo` in this scope";
    let predictions2 = predictor.predict(error_msg2, 3);
    
    println!("\nPredictions for: \"{}\"", error_msg2);
    println!("─────────────────────────────────────────");
    for (i, pred) in predictions2.iter().enumerate() {
        println!("  {}. {} (confidence: {:.2}%)", 
            i + 1, 
            pred.fix_suggestion,
            pred.confidence * 100.0
        );
    }

    println!("\n✅ All integration tests passed!");
}
