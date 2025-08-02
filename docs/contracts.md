# Contract-Based Verification in Depyler

Depyler supports Design by Contract (DbC) through Python-style annotations in docstrings. This allows you to specify preconditions, postconditions, and invariants that can be verified statically and enforced at runtime.

## Contract Annotations

### @requires (Preconditions)

Preconditions specify what must be true when a function is called:

```python
def binary_search(items: list[int], target: int) -> int:
    """
    Binary search implementation.
    
    @requires items is not None
    @requires all(items[i] <= items[i+1] for i in range(len(items)-1))
    @requires target is not None
    """
```

### @ensures (Postconditions)

Postconditions specify what must be true when a function returns:

```python
def safe_divide(numerator: float, denominator: float) -> float:
    """
    Safe division with contracts.
    
    @requires denominator != 0
    @ensures result == numerator / denominator
    @ensures result is not None
    """
```

### @invariant (Invariants)

Invariants specify conditions that must remain true throughout execution:

```python
def list_sum(numbers: list[float]) -> float:
    """
    Sum all numbers in a list.
    
    @requires numbers is not None
    @ensures result >= 0 if all(n >= 0 for n in numbers) else True
    @invariant len(numbers) >= 0
    """
```

## Predicate Language

The contract system supports a rich predicate language:

### Comparison Operators
- `==`, `!=`, `<`, `<=`, `>`, `>=`
- `in`, `not in`
- `is`, `is not`

### Logical Operators
- `and`, `or`, `not`
- `implies` (logical implication)

### Quantifiers
- `all(predicate for var in collection)`
- `any(predicate for var in collection)`
- `forall var in domain: predicate`
- `exists var in domain: predicate`

### Special Functions
- `old(expr)` - refers to the value at function entry
- `result` - refers to the return value
- `len(collection)` - length of a collection
- `type(var)` - type checking

## Type-Based Implicit Contracts

Depyler automatically generates implicit contracts based on type information:

1. **List/Dict Parameters**: Automatically adds null checks
2. **Optional Return Types**: Ensures result validity
3. **Function Properties**: Converts properties to invariants

## Contract Inheritance

Contracts follow the Liskov Substitution Principle (LSP):

```python
class Shape:
    def area(self) -> float:
        """
        @ensures result >= 0
        """
        pass

class Circle(Shape):
    def area(self) -> float:
        """
        @requires self.radius > 0  # Can add preconditions
        @ensures result >= 0       # Must maintain postconditions
        @ensures result == pi * self.radius ** 2  # Can strengthen
        """
        pass
```

## Verification Levels

Depyler supports three verification levels:

1. **Basic**: Type-based contracts only
2. **Standard**: Type + explicit contracts
3. **Strict**: All contracts with runtime checks

Enable verification with:
```bash
depyler transpile input.py --verify standard
```

## Runtime Contract Enforcement

When enabled, contracts are compiled into runtime assertions:

```rust
pub fn safe_divide(numerator: f64, denominator: f64) -> f64 {
    // Contract precondition validation
    assert!(denominator != 0.0, "Precondition violated: denominator != 0");
    
    let result = numerator / denominator;
    
    // Contract postcondition validation
    debug_assert!(result == numerator / denominator, 
                  "Postcondition failed: result == numerator / denominator");
    
    result
}
```

## Contract Verification Architecture

The contract system consists of several components:

1. **PreconditionChecker**: Validates function preconditions
2. **PostconditionVerifier**: Verifies postconditions with state tracking
3. **InvariantChecker**: Ensures invariants are maintained
4. **ContractInheritance**: Manages contract refinement and LSP

## Future Enhancements

1. **SMT Solver Integration**: Formal verification with Z3/CVC5
2. **Contract Inference**: Automatic contract generation from code
3. **Weakest Precondition Calculus**: Compute minimal preconditions
4. **Contract Monitoring**: Runtime performance impact analysis

## Examples

### Example 1: Array Bounds Checking
```python
def get_element(arr: list[int], index: int) -> int:
    """
    @requires arr is not None
    @requires 0 <= index < len(arr)
    @ensures result == arr[index]
    """
    return arr[index]
```

### Example 2: State Modification
```python
def increment_all(numbers: list[int]) -> None:
    """
    @requires numbers is not None
    @ensures all(numbers[i] == old(numbers[i]) + 1 for i in range(len(numbers)))
    @ensures len(numbers) == old(len(numbers))
    """
    for i in range(len(numbers)):
        numbers[i] += 1
```

### Example 3: Complex Invariant
```python
def maintain_sorted(items: list[int], value: int) -> None:
    """
    @requires all(items[i] <= items[i+1] for i in range(len(items)-1))
    @ensures all(items[i] <= items[i+1] for i in range(len(items)-1))
    @ensures value in items
    """
    # Insert value while maintaining sort order
    pass
```

## Best Practices

1. **Be Specific**: Write precise contracts that capture intent
2. **Check Nullability**: Always verify nullable parameters
3. **Document Assumptions**: Use contracts to document assumptions
4. **Test Contracts**: Ensure contracts are neither too weak nor too strong
5. **Progressive Enhancement**: Start with basic contracts, refine over time

## Integration with CI/CD

Add contract verification to your CI pipeline:

```yaml
- name: Verify Contracts
  run: |
    depyler transpile src/ --verify strict
    cargo test --features contracts
```

## Performance Considerations

- Precondition checks: ~2-5% overhead
- Postcondition checks: ~5-10% overhead
- Invariant checks: ~10-20% overhead

Use `--release` builds to disable debug assertions in production.