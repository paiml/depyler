"""Test passing lambdas as arguments"""

def apply_func(x: int, f) -> int:
    return f(x)

def test_lambda_passing():
    # Pass lambda to function
    result1 = apply_func(10, lambda x: x * 2)
    result2 = apply_func(5, lambda x: x + 10)
    
    # Store lambda and pass it
    triple = lambda x: x * 3
    result3 = apply_func(7, triple)
    
    return result1 + result2 + result3