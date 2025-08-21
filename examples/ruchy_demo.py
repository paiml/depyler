# Demo of Python code that can be interpreted via Ruchy

def factorial(n: int) -> int:
    """Calculate factorial recursively"""
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)

def fibonacci(n: int) -> int:
    """Calculate fibonacci number"""
    if n <= 1:
        return n
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)

# Calculate some values
result1 = factorial(5)
result2 = fibonacci(10)

print(f"Factorial of 5: {result1}")
print(f"Fibonacci of 10: {result2}")

# List comprehension example
squares = [x * x for x in range(1, 6)]
print(f"Squares: {squares}")

# Lambda example (when supported)
# double = lambda x: x * 2
# print(f"Double of 21: {double(21)}")