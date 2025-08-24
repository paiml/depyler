"""
Fibonacci sequence calculator - Example for Depyler transpilation
Demonstrates type hints, recursion, and iteration patterns
"""
from typing import List, Iterator, Optional


def fibonacci_recursive(n: int) -> int:
    """Calculate nth Fibonacci number recursively."""
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)


def fibonacci_iterative(n: int) -> int:
    """Calculate nth Fibonacci number iteratively."""
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    
    return curr


def fibonacci_sequence(limit: int) -> List[int]:
    """Generate Fibonacci sequence up to n terms."""
    if limit <= 0:
        return []
    
    sequence: List[int] = []
    a, b = 0, 1
    
    for _ in range(limit):
        sequence.append(a)
        a, b = b, a + b
    
    return sequence


def fibonacci_generator(limit: Optional[int] = None) -> Iterator[int]:
    """Generate Fibonacci numbers as an iterator."""
    a, b = 0, 1
    count = 0
    
    while limit is None or count < limit:
        yield a
        a, b = b, a + b
        count += 1


def fibonacci_memoized(n: int, memo: Optional[dict] = None) -> int:
    """Calculate Fibonacci with memoization."""
    if memo is None:
        memo = {}
    
    if n in memo:
        return memo[n]
    
    if n <= 0:
        return 0
    elif n == 1:
        return 1
    
    result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    memo[n] = result
    return result


def find_fibonacci_index(target: int) -> Optional[int]:
    """Find the index of a target value in Fibonacci sequence."""
    if target < 0:
        return None
    
    a, b = 0, 1
    index = 0
    
    while a < target:
        a, b = b, a + b
        index += 1
    
    return index if a == target else None


def is_fibonacci_number(num: int) -> bool:
    """Check if a number is in the Fibonacci sequence."""
    if num < 0:
        return False
    
    # A number is Fibonacci if one of (5*n^2 + 4) or (5*n^2 - 4) is a perfect square
    def is_perfect_square(x: int) -> bool:
        root = int(x ** 0.5)
        return root * root == x
    
    return is_perfect_square(5 * num * num + 4) or is_perfect_square(5 * num * num - 4)


def main() -> None:
    """Test the Fibonacci functions."""
    n = 10
    
    print(f"Fibonacci({n}) recursive: {fibonacci_recursive(n)}")
    print(f"Fibonacci({n}) iterative: {fibonacci_iterative(n)}")
    print(f"Fibonacci({n}) memoized: {fibonacci_memoized(n)}")
    
    print(f"\nFirst {n} Fibonacci numbers: {fibonacci_sequence(n)}")
    
    print("\nUsing generator:")
    for i, fib in enumerate(fibonacci_generator(n)):
        print(f"  F({i}) = {fib}")
    
    target = 21
    index = find_fibonacci_index(target)
    if index is not None:
        print(f"\n{target} is at index {index} in Fibonacci sequence")
    else:
        print(f"\n{target} is not in Fibonacci sequence")
    
    test_nums = [0, 1, 2, 3, 4, 5, 8, 13, 20, 21]
    print("\nFibonacci number check:")
    for num in test_nums:
        print(f"  {num}: {is_fibonacci_number(num)}")


if __name__ == "__main__":
    main()