#!/usr/bin/env python3
"""
Example: Property Verification with Depyler

This example demonstrates how Depyler's verification system
ensures correctness properties for transpiled code.
"""

from typing import List, Optional

# Example 1: Pure function verification
@depyler.pure
def add(a: int, b: int) -> int:
    """Pure function - no side effects"""
    return a + b

# Example 2: Memory safe function
@depyler.memory_safe
def safe_access(items: List[int], index: int) -> Optional[int]:
    """Safe array access with bounds checking"""
    if 0 <= index < len(items):
        return items[index]
    return None

# Example 3: Thread safe function
@depyler.thread_safe
@depyler.interior_mutability("arc_mutex")
def concurrent_counter(current: int, increment: int) -> int:
    """Thread-safe counter increment"""
    return current + increment

# Example 4: Terminating function
@depyler.always_terminates
def factorial(n: int) -> int:
    """Guaranteed to terminate for non-negative inputs"""
    if n <= 0:
        return 1
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result

# Example 5: Panic-free function
@depyler.panic_free
def safe_divide(a: int, b: int) -> Optional[float]:
    """Division that never panics"""
    if b == 0:
        return None
    return a / b

# Example 6: Lifetime-safe function
@depyler.lifetime_safe
def find_max(numbers: List[int]) -> Optional[int]:
    """Returns reference to max value with proper lifetime"""
    if not numbers:
        return None
    
    max_val = numbers[0]
    for num in numbers[1:]:
        if num > max_val:
            max_val = num
    return max_val

# Example 7: Contract verification
@depyler.contract(
    requires="n >= 0",
    ensures="result >= 1"
)
def fibonacci(n: int) -> int:
    """Fibonacci with formal contracts"""
    if n <= 1:
        return 1
    return fibonacci(n - 1) + fibonacci(n - 2)

# Example 8: Complex verification
@depyler.verify_all
class VerifiedStack:
    """Stack with full verification"""
    
    def __init__(self, capacity: int):
        self.items: List[int] = []
        self.capacity = capacity
    
    @depyler.contract(
        requires="not self.is_full()",
        ensures="self.size() == old(self.size()) + 1"
    )
    def push(self, item: int) -> None:
        """Push with verification"""
        if len(self.items) < self.capacity:
            self.items.append(item)
    
    @depyler.contract(
        requires="not self.is_empty()",
        ensures="self.size() == old(self.size()) - 1"
    )
    def pop(self) -> int:
        """Pop with verification"""
        if self.items:
            return self.items.pop()
        return 0  # Should not reach here due to precondition
    
    @depyler.pure
    def is_empty(self) -> bool:
        """Check if empty"""
        return len(self.items) == 0
    
    @depyler.pure
    def is_full(self) -> bool:
        """Check if full"""
        return len(self.items) >= self.capacity
    
    @depyler.pure
    def size(self) -> int:
        """Get current size"""
        return len(self.items)

def main():
    """Demonstrate verified functions"""
    print("=== Verification Demo ===")
    
    # Pure function
    result = add(5, 3)
    print(f"Pure add: {result}")
    
    # Memory safe
    items = [10, 20, 30]
    print(f"Safe access: {safe_access(items, 1)}")
    print(f"Safe access OOB: {safe_access(items, 10)}")
    
    # Thread safe
    print(f"Concurrent: {concurrent_counter(100, 5)}")
    
    # Terminating
    print(f"Factorial(5): {factorial(5)}")
    
    # Panic free
    print(f"Safe divide: {safe_divide(10, 2)}")
    print(f"Safe divide by zero: {safe_divide(10, 0)}")
    
    # Verified stack
    stack = VerifiedStack(3)
    stack.push(1)
    stack.push(2)
    print(f"Stack size: {stack.size()}")
    print(f"Stack pop: {stack.pop()}")

if __name__ == "__main__":
    main()

"""
Verification Properties:
=======================

1. Type Preservation
   - All types are preserved during transpilation
   - No implicit conversions that could fail

2. Memory Safety
   - No use-after-free
   - No buffer overflows
   - Proper bounds checking

3. Lifetime Safety
   - References are always valid
   - No dangling pointers
   - Proper ownership tracking

4. Thread Safety
   - No data races
   - Proper synchronization
   - Safe concurrent access

5. Termination
   - Functions marked as terminating always halt
   - No infinite loops
   - Bounded recursion

6. Panic Freedom
   - No runtime panics
   - All error cases handled
   - Graceful error propagation

7. Contract Verification
   - Preconditions checked
   - Postconditions ensured
   - Invariants maintained

Running Verification:
====================

$ depyler transpile verification_demo.py --verify

This will:
1. Check all annotated properties
2. Generate property tests
3. Run static analysis
4. Report verification results
5. Generate proof obligations
"""