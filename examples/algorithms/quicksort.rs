Type inference hints:
Hint: list[Any] for variable 'arr' [High] (usage patterns suggest this type)
Hint: int for variable 'middle' [Medium] (usage patterns suggest this type)

Type inference hints:
Hint: int for variable 'i' [Medium] (usage patterns suggest this type)
Hint: list[Any] for variable 'arr' [Medium] (usage patterns suggest this type)
Hint: int for variable 'low' [Medium] (usage patterns suggest this type)

Type inference hints:
Hint: int for variable 'pi' [Medium] (usage patterns suggest this type)


Performance Warnings
══════════════════════════════════════════════════

[1] [Medium] Large value 'arr' passed by copy
   Location: quicksort, line 0
   Impact: Complexity: O(n), Scales: Yes, Hot path: No
   Why: Passing large values by copy is inefficient
   Fix: Consider passing by reference (&) or using Box/Arc for large types

[2] [Medium] Large value 'arr' passed by copy
   Location: partition, line 0
   Impact: Complexity: O(n), Scales: Yes, Hot path: No
   Why: Passing large values by copy is inefficient
   Fix: Consider passing by reference (&) or using Box/Arc for large types

[3] [Medium] Large value 'arr' passed by copy
   Location: quicksort_inplace, line 0
   Impact: Complexity: O(n), Scales: Yes, Hot path: No
   Why: Passing large values by copy is inefficient
   Fix: Consider passing by reference (&) or using Box/Arc for large types

Summary: Found 3 warnings (0 critical, 0 high severity)


Profiling Report
══════════════════════════════════════════════════

Summary
  Total estimated instructions: 222
  Total estimated allocations: 0
  Functions analyzed: 3

Hot Paths
  [1] partition (558.6% of execution time)
  [2] quicksort_inplace (21.6% of execution time)
  [3] quicksort (22.5% of execution time)

Function Metrics
🔥 partition                       558.6% time |    124 inst |    0 alloc
🔥 quicksort                        22.5% time |     50 inst |    0 alloc
🔥 quicksort_inplace                21.6% time |     48 inst |    0 alloc

Performance Predictions
  • Rust's iterator fusion can optimize chained operations (1.2x speedup, 80% confidence)
  • Rust's memory layout is more cache-friendly than Python (1.3x speedup, 70% confidence)

🚀 Estimated overall speedup: 1.6x

Error: Complex tuple unpacking not yet supported
