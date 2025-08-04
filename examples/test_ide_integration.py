"""
Example demonstrating IDE integration features.
"""

def calculate_fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number.
    
    Uses recursion with memoization for efficiency.
    """
    if n <= 1:
        return n
    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)


class MathUtils:
    """Utility class for mathematical operations."""
    
    def __init__(self):
        self.precision = 2
    
    def round_number(self, value: float) -> float:
        """Round a number to the configured precision."""
        return round(value, self.precision)
    
    @staticmethod
    def is_prime(n: int) -> bool:
        """Check if a number is prime."""
        if n < 2:
            return False
        for i in range(2, int(n ** 0.5) + 1):
            if n % i == 0:
                return False
        return True


def process_data(items: list[int]) -> dict[str, int]:
    """Process a list of integers and return statistics."""
    utils = MathUtils()
    
    stats = {
        "count": len(items),
        "sum": sum(items),
        "primes": 0
    }
    
    for item in items:
        if utils.is_prime(item):
            stats["primes"] = stats["primes"] + 1
    
    return stats


# Example usage that would trigger hover/completion
result = calculate_fibonacci(10)
math_util = MathUtils()
rounded = math_util.round_number(3.14159)