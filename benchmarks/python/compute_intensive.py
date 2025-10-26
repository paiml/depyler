#!/usr/bin/env python3
"""
Compute-intensive benchmark: Fibonacci calculation and sum operations.

This benchmark tests:
- Integer arithmetic performance
- Recursive function calls
- Loop performance
- List operations (validated stdlib)

Uses only validated stdlib features from TDD Book.
"""

def fibonacci_iterative(n: int) -> int:
    """Calculate nth Fibonacci number iteratively."""
    if n <= 1:
        return n

    a = 0
    b = 1
    for i in range(2, n + 1):
        c = a + b
        a = b
        b = c

    return b


def sum_fibonacci_numbers(limit: int) -> int:
    """Sum first 'limit' Fibonacci numbers."""
    total = 0
    for i in range(limit):
        total += fibonacci_iterative(i)
    return total


def calculate_statistics(numbers: list[int]) -> dict[str, int]:
    """Calculate basic statistics on a list of numbers."""
    if not numbers:
        return {"count": 0, "sum": 0, "min": 0, "max": 0}

    count = len(numbers)
    total = 0
    min_val = numbers[0]
    max_val = numbers[0]

    for num in numbers:
        total += num
        if num < min_val:
            min_val = num
        if num > max_val:
            max_val = num

    return {
        "count": count,
        "sum": total,
        "min": min_val,
        "max": max_val
    }


def main():
    """Run benchmark with different limits."""
    limits = [25, 30, 35]

    for limit in limits:
        result = sum_fibonacci_numbers(limit)

        # Generate fibonacci sequence for statistics
        fib_sequence = []
        for i in range(limit):
            fib_sequence.append(fibonacci_iterative(i))

        stats = calculate_statistics(fib_sequence)

        print(f"Limit: {limit} | Sum: {result} | Count: {stats['count']} | Max: {stats['max']}")


if __name__ == "__main__":
    main()
