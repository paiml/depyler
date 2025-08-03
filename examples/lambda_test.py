"""Test lambda function support"""

def test_simple_lambdas():
    # Simple lambda functions
    add = lambda x, y: x + y
    square = lambda x: x * x
    constant = lambda: 42
    
    # Use the lambdas
    result1 = add(3, 5)
    result2 = square(4)
    result3 = constant()
    
    return result1, result2, result3

def test_lambda_with_list_operations():
    # Lambda with list operations
    numbers = [1, 2, 3, 4, 5]
    
    # Map operation
    squares = list(map(lambda x: x * x, numbers))
    
    # Filter operation
    evens = list(filter(lambda x: x % 2 == 0, numbers))
    
    # Custom operation
    double = lambda lst: [x * 2 for x in lst]
    doubled = double(numbers)
    
    return squares, evens, doubled

def test_lambda_in_expressions():
    # Lambda in conditional
    get_operation = lambda is_add: (lambda x, y: x + y) if is_add else (lambda x, y: x - y)
    
    add_op = get_operation(True)
    sub_op = get_operation(False)
    
    return add_op(10, 5), sub_op(10, 5)

def test_lambda_with_closure():
    # Lambda capturing outer variables
    multiplier = 3
    scale = lambda x: x * multiplier
    
    result = scale(7)
    return result