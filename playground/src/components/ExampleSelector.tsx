import { useState } from "react";
import { usePlaygroundStore } from "@/store";

interface Example {
  id: string;
  title: string;
  description: string;
  code: string;
  category: "basic" | "advanced" | "optimization" | "patterns";
}

const examples: Example[] = [
  {
    id: "fibonacci",
    title: "Fibonacci Sequence",
    description: "Efficient fibonacci calculation with memoization",
    category: "basic",
    code: `# @depyler: optimize_energy=true
def calculate_fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number efficiently."""
    if n <= 1:
        return n
    
    a, b = 0, 1
    for i in range(2, n + 1):
        a, b = b, a + b
    
    return b

# Example usage
result = calculate_fibonacci(20)
print(f"The 20th Fibonacci number is: {result}")`,
  },
  {
    id: "prime_sieve",
    title: "Prime Number Sieve",
    description: "Sieve of Eratosthenes for finding prime numbers",
    category: "optimization",
    code: `# @depyler: optimize_energy=true, string_strategy=zero_copy
def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Find all prime numbers up to the given limit."""
    if limit < 2:
        return []
    
    # Initialize sieve
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    
    # Sieving process
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, limit + 1, i):
                is_prime[j] = False
    
    # Collect primes
    return [i for i in range(2, limit + 1) if is_prime[i]]

primes = sieve_of_eratosthenes(30)
print(f"Primes up to 30: {primes}")`,
  },
  {
    id: "binary_search",
    title: "Binary Search",
    description: "Efficient search algorithm for sorted lists",
    category: "basic",
    code: `# @depyler: safety_level=strict
def binary_search(arr: list[int], target: int) -> int:
    """Binary search implementation with O(log n) complexity."""
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1  # Not found

# Example usage
numbers = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
index = binary_search(numbers, 11)
print(f"Found 11 at index: {index}")`,
  },
  {
    id: "matrix_multiply",
    title: "Matrix Multiplication",
    description: "Optimized matrix multiplication with energy annotations",
    category: "advanced",
    code: `# @depyler: optimize_energy=true, ownership_model=borrowed
def matrix_multiply(a: list[list[float]], b: list[list[float]]) -> list[list[float]]:
    """Multiply two matrices with optimized memory access patterns."""
    rows_a, cols_a = len(a), len(a[0]) if a else 0
    rows_b, cols_b = len(b), len(b[0]) if b else 0
    
    if cols_a != rows_b:
        raise ValueError("Incompatible matrix dimensions")
    
    # Initialize result matrix
    result = [[0.0 for _ in range(cols_b)] for _ in range(rows_a)]
    
    # Cache-friendly multiplication
    for i in range(rows_a):
        for k in range(cols_a):
            for j in range(cols_b):
                result[i][j] += a[i][k] * b[k][j]
    
    return result

# Example usage
a = [[1, 2], [3, 4]]
b = [[5, 6], [7, 8]]
result = matrix_multiply(a, b)
print("Result:", result)`,
  },
  {
    id: "decorator_pattern",
    title: "Decorator Pattern",
    description: "Python decorator with Rust performance",
    category: "patterns",
    code: `# @depyler: emit_docs=true
import time
from typing import Callable, Any

def timing_decorator(func: Callable) -> Callable:
    """Decorator to measure function execution time."""
    def wrapper(*args: Any, **kwargs: Any) -> Any:
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"{func.__name__} took {(end - start)*1000:.2f}ms")
        return result
    return wrapper

@timing_decorator
def expensive_operation(n: int) -> int:
    """Simulate an expensive computation."""
    total = 0
    for i in range(n):
        total += i ** 2
    return total

# Example usage
result = expensive_operation(10000)
print(f"Result: {result}")`,
  },
];

export function ExampleSelector() {
  const [isOpen, setIsOpen] = useState(false);
  const { setPythonCode } = usePlaygroundStore();

  const loadExample = (example: Example) => {
    setPythonCode(example.code);
    setIsOpen(false);
  };

  const categories = [...new Set(examples.map(e => e.category))];

  return (
    <div className="relative">
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        aria-expanded={isOpen}
        aria-haspopup="true"
      >
        <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
        </svg>
        Examples
      </button>

      {isOpen && (
        <div className="absolute left-0 mt-2 w-96 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 z-50 max-h-96 overflow-y-auto">
          <div className="py-1" role="menu" aria-orientation="vertical">
            {categories.map(category => (
              <div key={category}>
                <div className="px-4 py-2 bg-gray-50">
                  <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                    {category}
                  </h3>
                </div>
                {examples
                  .filter(e => e.category === category)
                  .map(example => (
                    <button
                      type="button"
                      key={example.id}
                      onClick={() => loadExample(example)}
                      className="w-full text-left px-4 py-3 hover:bg-gray-50 border-b border-gray-100 last:border-b-0"
                    >
                      <div className="text-sm font-medium text-gray-900">{example.title}</div>
                      <div className="text-xs text-gray-500 mt-1">{example.description}</div>
                    </button>
                  ))}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}