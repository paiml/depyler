
Profiling Report
══════════════════════════════════════════════════

Summary
  Total estimated instructions: 24
  Total estimated allocations: 0
  Functions analyzed: 1

Hot Paths
  [1] test_calculator (100.0% of execution time)

Function Metrics
🔥 test_calculator                 100.0% time |     24 inst |    0 alloc

Performance Predictions
  • Rust's memory layout is more cache-friendly than Python (1.3x speedup, 70% confidence)

🚀 Estimated overall speedup: 1.3x

Error: Expression type not yet supported: FString { parts: [Literal("add("), Expr(Var("x")), Literal(")")] }
