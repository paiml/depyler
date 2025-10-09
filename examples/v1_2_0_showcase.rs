Type inference hints:
Hint: int for variable 'future_value' [Medium] (usage patterns suggest this type)


Profiling Report
══════════════════════════════════════════════════

Summary
  Total estimated instructions: 130
  Total estimated allocations: 0
  Functions analyzed: 5

Hot Paths
  [1] demo_dataclass (15.4% of execution time)
  [2] demo_instance_methods (17.7% of execution time)
  [3] demo_all_features (28.5% of execution time)
  [4] main (35.4% of execution time)

Function Metrics
🔥 main                             35.4% time |     46 inst |    0 alloc
🔥 demo_all_features                28.5% time |     37 inst |    0 alloc
🔥 demo_instance_methods            17.7% time |     23 inst |    0 alloc
🔥 demo_dataclass                   15.4% time |     20 inst |    0 alloc
   demo_static_methods               3.1% time |      4 inst |    0 alloc

Performance Predictions
  • Rust's memory layout is more cache-friendly than Python (1.3x speedup, 70% confidence)

🚀 Estimated overall speedup: 1.3x

Error: Expression type not yet supported: FString { parts: [Expr(Attribute { value: Var("self"), attr: "owner" }), Literal(": $"), Expr(Attribute { value: Var("self"), attr: "balance" })] }
