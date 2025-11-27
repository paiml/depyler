//! Train MoE Oracle on real compilation errors from reprorusted-python-cli
//!
//! Usage: cargo run --example train_moe_on_real_corpus

use depyler_oracle::{
    load_real_corpus, train_moe_on_real_corpus,
    moe_oracle::ExpertDomain,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(60));
    println!("     MoE Oracle Training on Real Error Corpus");
    println!("{}", "=".repeat(60));

    // Step 1: Load and analyze real corpus
    let samples = load_real_corpus("/tmp/real_errors.txt");
    println!("\n   CORPUS STATISTICS:");
    println!("   Total samples: {}", samples.len());

    // Count by error code
    let mut code_counts: HashMap<String, usize> = HashMap::new();
    for (code, _, _) in &samples {
        *code_counts.entry(code.clone()).or_default() += 1;
    }

    // Count by expert domain
    let mut domain_counts: HashMap<ExpertDomain, usize> = HashMap::new();
    for (code, _, _) in &samples {
        let domain = ExpertDomain::from_error_code(code);
        *domain_counts.entry(domain).or_default() += 1;
    }

    println!("\n   TOP 10 ERROR CODES:");
    let mut codes: Vec<_> = code_counts.iter().collect();
    codes.sort_by(|a, b| b.1.cmp(a.1));
    for (code, count) in codes.iter().take(10) {
        let domain = ExpertDomain::from_error_code(code);
        println!("   {}: {} occurrences -> {:?}", code, count, domain);
    }

    println!("\n   DISTRIBUTION BY EXPERT DOMAIN:");
    let mut domains: Vec<_> = domain_counts.iter().collect();
    domains.sort_by(|a, b| b.1.cmp(a.1));
    for (domain, count) in &domains {
        let pct = (**count as f64 / samples.len() as f64) * 100.0;
        println!("   {:?}: {} ({:.1}%)", domain, count, pct);
    }

    // Step 2: Train MoE Oracle
    println!("\n{}", "=".repeat(60));
    println!("   TRAINING MoE Oracle...");
    let oracle = train_moe_on_real_corpus()?;

    // Step 3: Evaluate on test cases
    println!("\n{}", "=".repeat(60));
    println!("   EVALUATION RESULTS:\n");

    let test_cases = [
        ("E0308", "mismatched types expected i32, found String"),
        ("E0277", "the trait bound `String: AsRef<OsStr>` is not satisfied"),
        ("E0425", "cannot find value `foo` in this scope"),
        ("E0599", "no method named `exists` found for type `String`"),
        ("E0609", "no field `x` on type `Value`"),
        ("E0027", "pattern does not mention field `y`"),
        ("E0382", "use of moved value: `data`"),
        ("E0433", "failed to resolve: use of undeclared crate or module"),
    ];

    let mut correct = 0;
    for (code, ctx) in test_cases {
        let result = oracle.classify(code, ctx);
        let expected_domain = ExpertDomain::from_error_code(code);
        let is_correct = result.primary_expert == expected_domain;
        if is_correct {
            correct += 1;
        }

        let status = if is_correct { "[PASS]" } else { "[FAIL]" };
        println!("   {} {} -> {:?} (conf: {:.2})",
            status, code, result.primary_expert, result.confidence);
        if let Some(fix) = &result.suggested_fix {
            println!("        Fix: {}", fix);
        }
    }

    let accuracy = (correct as f64 / test_cases.len() as f64) * 100.0;
    println!("\n{}", "=".repeat(60));
    println!("   ACCURACY: {}/{} ({:.1}%)", correct, test_cases.len(), accuracy);
    println!("{}", "=".repeat(60));

    Ok(())
}
