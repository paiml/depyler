"""
Power operator and floor division demonstration
Shows how Python ** and // operators work in Depyler
"""

def power_examples(base: int, exponent: int) -> int:
    """Demonstrate power operator with different cases"""
    
    # Simple positive power
    result1 = 2 ** 3  # Should be 8
    
    # Variable power
    result2 = base ** exponent
    
    # Zero exponent (always 1)
    result3 = base ** 0
    
    return result1 + result2 + result3

def floor_division_examples(dividend: int, divisor: int) -> int:
    """Demonstrate floor division with Python semantics"""
    
    # Positive floor division
    result1 = 17 // 5  # Should be 3 (not 3.4)
    
    # Negative dividend (Python floors towards negative infinity)
    result2 = -17 // 5  # Should be -4 (not -3)
    
    # Negative divisor
    result3 = 17 // -5  # Should be -4
    
    # Both negative
    result4 = -17 // -5  # Should be 3
    
    # Variable division
    result5 = dividend // divisor
    
    return result1 + result2 + result3 + result4 + result5

def combined_operations(a: int, b: int) -> int:
    """Combine power and floor division"""
    
    # Power then floor divide
    step1 = a ** 2
    result1 = step1 // b
    
    # Floor divide then power
    step2 = a // b
    result2 = step2 ** 2
    
    return result1 + result2

def mathematical_sequence(n: int) -> int:
    """Calculate sum of squares using both operators"""
    total = 0
    i = 1
    
    while i <= n:
        # Add i squared, using floor division to ensure integer arithmetic
        square = i ** 2
        contribution = square // 1  # This is just square, but demonstrates floor div
        total = total + contribution
        i = i + 1
    
    return total