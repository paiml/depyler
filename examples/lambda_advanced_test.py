"""Advanced lambda function tests"""

def test_lambda_with_operations():
    # Lambda with arithmetic
    calc = lambda x, y, z: (x + y) * z
    result1 = calc(2, 3, 4)
    
    # Lambda with simple operation
    negate = lambda x: -x
    result2 = negate(5)
    
    # Lambda with multiple parameters
    multiply = lambda a, b: a * b
    result3 = multiply(10, 20)
    
    return result1, result2, result3

def test_lambda_as_argument():
    # Using lambda with map-like operation
    numbers = [1, 2, 3, 4, 5]
    
    # Apply function to each element
    def apply_to_list(lst: list[int], func) -> list[int]:
        result = []
        for item in lst:
            result.append(func(item))
        return result
    
    # Use lambda as argument
    doubled = apply_to_list(numbers, lambda x: x * 2)
    squared = apply_to_list(numbers, lambda x: x * x)
    
    return doubled, squared