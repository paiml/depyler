#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz test the Depyler transpiler (renacer pattern)
    // Tests parser robustness with arbitrary Python-like input

    if let Ok(input) = std::str::from_utf8(data) {
        // Skip empty or overly large inputs
        if input.is_empty() || input.len() > 10_000 {
            return;
        }

        // Attempt to transpile - should never panic
        use depyler::DepylerPipeline;
        let pipeline = DepylerPipeline::new();

        // Ignore errors, we only care about panics/crashes
        let _ = pipeline.transpile(input);
    }
});
