# Test Case 1: Simple addition
def add_two_numbers(a: int, b: int) -> int:
    return a + b

# Test Case 2: Simple subtraction  
def subtract_numbers(a: int, b: int) -> int:
    return a - b

# Test Case 3: Multiplication
def multiply_numbers(a: int, b: int) -> int:
    return a * b

# Test Case 4: Integer division
def divide_numbers(a: int, b: int) -> int:
    return a // b

# Test Case 5: Modulo operation
def modulo_operation(a: int, b: int) -> int:
    return a % b

# Test Case 6: Power operation
def power_operation(base: int, exponent: int) -> int:
    return base ** exponent

# Test Case 7: Absolute value
def absolute_value(n: int) -> int:
    if n >= 0:
        return n
    else:
        return -n

# Test Case 8: Maximum of two numbers
def max_two_numbers(a: int, b: int) -> int:
    if a > b:
        return a
    else:
        return b

# Test Case 9: Minimum of two numbers
def min_two_numbers(a: int, b: int) -> int:
    if a < b:
        return a
    else:
        return b

# Test Case 10: Sign function
def sign_function(n: int) -> int:
    if n > 0:
        return 1
    elif n < 0:
        return -1
    else:
        return 0