"""Simple lambda function tests"""

def test_basic_lambdas():
    # Single parameter lambda
    square = lambda x: x * x
    result1 = square(5)
    
    # Two parameter lambda
    add = lambda x, y: x + y
    result2 = add(3, 4)
    
    # No parameter lambda
    get_value = lambda: 42
    result3 = get_value()
    
    return result1 + result2 + result3