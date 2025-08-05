#!/usr/bin/env python3
"""
Example: Interactive Annotation Mode

This example demonstrates Depyler's interactive annotation mode,
which helps improve transpilation success by suggesting optimizations
and ownership annotations.

To run in interactive mode:
    depyler transpile examples/interactive_annotation.py --interactive --annotate

The interactive mode will:
1. Attempt initial transpilation
2. Analyze failures and suggest annotations
3. Allow you to select which suggestions to apply
4. Re-attempt transpilation with annotations
"""

import math
from typing import List, Dict, Optional

# Example 1: Performance-critical function that could benefit from annotations
def matrix_multiply(a: List[List[float]], b: List[List[float]]) -> List[List[float]]:
    """
    Matrix multiplication with nested loops.
    
    Interactive mode will suggest:
    - Aggressive optimization for nested loops
    - Potential SIMD vectorization
    - Loop unrolling opportunities
    """
    n = len(a)
    m = len(b[0])
    k = len(b)
    
    result = [[0.0 for _ in range(m)] for _ in range(n)]
    
    for i in range(n):
        for j in range(m):
            for p in range(k):
                result[i][j] += a[i][p] * b[p][j]
    
    return result


# Example 2: String processing that needs ownership hints
def process_text_data(texts: List[str], keywords: List[str]) -> Dict[str, int]:
    """
    Process text data to count keyword occurrences.
    
    Interactive mode will suggest:
    - String ownership strategy (borrowed vs owned)
    - Potential zero-copy optimizations
    """
    keyword_counts = {kw: 0 for kw in keywords}
    
    for text in texts:
        normalized = text.lower().strip()
        for keyword in keywords:
            if keyword in normalized:
                keyword_counts[keyword] += 1
    
    return keyword_counts


# Example 3: Recursive function that may need stack hints
def quicksort(arr: List[int]) -> List[int]:
    """
    Recursive quicksort implementation.
    
    Interactive mode will suggest:
    - Stack depth limits
    - Tail recursion optimization
    - Memory allocation strategy
    """
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)


# Example 4: Error-prone function needing safety annotations
def safe_divide(numbers: List[float], divisors: List[float]) -> List[Optional[float]]:
    """
    Safe division with error handling.
    
    Interactive mode will suggest:
    - Error handling strategy
    - Result type usage
    - Panic-free guarantees
    """
    results = []
    
    for i in range(min(len(numbers), len(divisors))):
        if divisors[i] != 0:
            results.append(numbers[i] / divisors[i])
        else:
            results.append(None)
    
    return results


# Example 5: Concurrent data processing candidate
def parallel_map(func, data: List[int]) -> List[int]:
    """
    Function that could benefit from parallelization.
    
    Interactive mode will suggest:
    - Thread safety requirements
    - Parallelization strategy
    - Send/Sync trait bounds
    """
    results = []
    for item in data:
        # Simulate expensive computation
        result = func(item)
        for _ in range(1000):
            result = (result * 7 + 13) % 1000000
        results.append(result)
    
    return results


# Example 6: Memory-intensive operation
class DataBuffer:
    """
    Large data buffer that needs memory annotations.
    
    Interactive mode will suggest:
    - Memory allocation strategy
    - Ownership model (Rc, Arc, Box)
    - Interior mutability needs
    """
    def __init__(self, size: int):
        self.data = [0] * size
        self.position = 0
    
    def write(self, values: List[int]) -> None:
        for value in values:
            if self.position < len(self.data):
                self.data[self.position] = value
                self.position += 1
    
    def read(self, count: int) -> List[int]:
        start = max(0, self.position - count)
        return self.data[start:self.position]


# Example 7: Complex algorithm needing multiple annotations
def optimize_route(
    distances: Dict[str, Dict[str, float]], 
    start: str, 
    end: str
) -> Optional[List[str]]:
    """
    Route optimization using dynamic programming.
    
    Interactive mode will suggest multiple annotations:
    - Algorithm complexity hints
    - Memory vs speed tradeoffs
    - Caching strategy
    - Error handling approach
    """
    # Simplified Dijkstra's algorithm
    visited = set()
    distances_from_start = {start: 0}
    previous = {}
    
    while len(visited) < len(distances):
        current = None
        current_distance = float('inf')
        
        for node in distances:
            if node not in visited and node in distances_from_start:
                if distances_from_start[node] < current_distance:
                    current = node
                    current_distance = distances_from_start[node]
        
        if current is None:
            break
            
        visited.add(current)
        
        for neighbor in distances[current]:
            if neighbor not in visited:
                new_distance = distances_from_start[current] + distances[current][neighbor]
                if neighbor not in distances_from_start or new_distance < distances_from_start[neighbor]:
                    distances_from_start[neighbor] = new_distance
                    previous[neighbor] = current
    
    # Reconstruct path
    if end not in previous:
        return None
        
    path = []
    current = end
    while current != start:
        path.append(current)
        current = previous[current]
    path.append(start)
    path.reverse()
    
    return path


def main():
    """
    Demonstration of functions that benefit from interactive annotation.
    
    When run with --interactive --annotate, Depyler will:
    1. Analyze each function's characteristics
    2. Suggest appropriate annotations
    3. Guide you through the annotation process
    4. Show before/after transpilation results
    """
    print("Interactive Annotation Examples")
    print("=" * 40)
    
    # Test matrix multiplication
    a = [[1, 2], [3, 4]]
    b = [[5, 6], [7, 8]]
    result = matrix_multiply(a, b)
    print(f"Matrix multiplication result: {result}")
    
    # Test text processing
    texts = ["Hello world", "Hello Python", "Rust is fast"]
    keywords = ["hello", "rust", "python"]
    counts = process_text_data(texts, keywords)
    print(f"Keyword counts: {counts}")
    
    # Test sorting
    numbers = [64, 34, 25, 12, 22, 11, 90]
    sorted_nums = quicksort(numbers)
    print(f"Sorted: {sorted_nums}")
    
    # Test safe division
    nums = [10.0, 20.0, 30.0]
    divs = [2.0, 0.0, 5.0]
    results = safe_divide(nums, divs)
    print(f"Division results: {results}")
    
    # Test parallel map
    data = list(range(10))
    mapped = parallel_map(lambda x: x * x, data)
    print(f"Mapped data: {mapped}")
    
    # Test data buffer
    buffer = DataBuffer(100)
    buffer.write([1, 2, 3, 4, 5])
    recent = buffer.read(3)
    print(f"Recent buffer data: {recent}")
    
    # Test route optimization
    graph = {
        'A': {'B': 1, 'C': 4},
        'B': {'A': 1, 'C': 2, 'D': 5},
        'C': {'A': 4, 'B': 2, 'D': 1},
        'D': {'B': 5, 'C': 1}
    }
    route = optimize_route(graph, 'A', 'D')
    print(f"Optimal route: {route}")


if __name__ == "__main__":
    main()