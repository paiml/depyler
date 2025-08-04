# Depyler Real-World Examples

This directory contains comprehensive real-world examples demonstrating Depyler's transpilation capabilities across various domains.

## Example Categories

### 1. Algorithms (`algorithms/`)
- **quicksort.py** - Classic quicksort implementation with in-place variant
- **fibonacci.py** - Multiple fibonacci implementations (recursive, memoized, iterative)

### 2. Data Structures (`data_structures/`)
- **stack.py** - Thread-safe stack with balanced parentheses checker
- **queue.py** - Arena-allocated queue with binary tree level-order traversal

### 3. String Processing (`string_processing/`)
- **text_analyzer.py** - Word frequency analysis, anagram detection, palindrome checking

### 4. Mathematical (`mathematical/`)
- **statistics.py** - Statistical functions (mean, median, mode, variance, correlation)
- **geometry.py** - 2D geometry with Point, Rectangle, Circle classes

### 5. Web Scraping (`web_scraping/`)
- **url_parser.py** - URL parsing and validation without external dependencies

### 6. File Processing (`file_processing/`)
- **csv_parser.py** - CSV parsing and analysis tools
- **log_analyzer.py** - Log file analysis and health monitoring

### 7. Networking (`networking/`)
- **http_client.py** - HTTP request/response handling simulation

### 8. Game Development (`game_development/`)
- **tic_tac_toe.py** - Complete tic-tac-toe game with AI using minimax algorithm

## Annotation Patterns Demonstrated

### Performance Annotations
- `@depyler: optimization_level = "aggressive"` - Maximum optimization
- `@depyler: optimization_level = "size"` - Size-optimized code
- `@depyler: bounds_checking = "explicit"` - Explicit bounds checking

### Memory Management
- `@depyler: ownership = "owned"` - Owned data structures
- `@depyler: ownership = "borrowed"` - Borrowed references
- `@depyler: memory_strategy = "arena"` - Arena allocation

### Safety Annotations
- `@depyler: thread_safety = "required"` - Thread-safe implementations
- `@depyler: string_strategy = "zero_copy"` - Efficient string handling

## Testing Examples

Each example can be transpiled and tested:

```bash
# Transpile an example
cargo run -- transpile examples/algorithms/quicksort.py

# Transpile with verification
cargo run -- transpile examples/algorithms/quicksort.py --verify

# Run property tests
make test-property
```

## Code Quality Features

These examples demonstrate:

1. **Type Safety** - Full type annotations for reliable transpilation
2. **Memory Safety** - Proper ownership patterns and bounds checking
3. **Performance** - Optimization annotations for critical paths
4. **Real-World Patterns** - Practical algorithms and data structures
5. **Error Handling** - Robust error conditions and edge cases
6. **Documentation** - Clear docstrings and comments

## Coverage Statistics

- **Total Examples**: 20+ distinct real-world scenarios
- **Annotation Coverage**: All major Depyler annotations demonstrated
- **Domain Coverage**: 8 different application domains
- **Complexity Range**: From simple utilities to complex algorithms
- **Type Coverage**: Primitive types, collections, classes, optional types

## Integration with Property Tests

These examples serve as:
- **Test Cases** for property-based testing
- **Benchmarks** for performance testing  
- **Documentation** for annotation usage
- **Validation** for transpilation correctness

Each example is designed to compile successfully and demonstrate specific Depyler features while solving real programming problems.