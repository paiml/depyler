"""
Example demonstrating debugging features.
"""

def fibonacci(n: int) -> int:
    """Calculate fibonacci number recursively."""
    if n <= 1:
        return n
    
    # Recursive calls
    return fibonacci(n - 1) + fibonacci(n - 2)


def find_prime_factors(n: int) -> list[int]:
    """Find all prime factors of a number."""
    factors = []
    d = 2
    
    while d * d <= n:
        while n % d == 0:
            factors.append(d)
            n = n // d
        d = d + 1
    
    if n > 1:
        factors.append(n)
    
    return factors


def binary_search(arr: list[int], target: int) -> int:
    """Perform binary search on sorted array."""
    left = 0
    right = len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1


# Test the functions
def test_functions():
    # Test fibonacci
    result = fibonacci(10)
    print(f"Fibonacci(10) = {result}")
    
    # Test prime factors
    factors = find_prime_factors(60)
    print(f"Prime factors of 60: {factors}")
    
    # Test binary search
    test_array = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
    index = binary_search(test_array, 7)
    print(f"Index of 7: {index}")


if __name__ == "__main__":
    test_functions()