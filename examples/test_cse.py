"""
Example demonstrating Common Subexpression Elimination (CSE).
"""

def repeated_complex_expressions(a: int, b: int, c: int) -> int:
    """Multiple uses of the same complex expression."""
    # (a + b) * c appears multiple times
    x = (a + b) * c + 10
    y = (a + b) * c - 5
    z = ((a + b) * c) * 2
    
    # This should be optimized to compute (a + b) * c once
    return x + y + z


def repeated_function_calls(n: int) -> int:
    """Repeated calls to pure functions."""
    # abs(n - 10) is computed multiple times
    if abs(n - 10) > 5:
        result = abs(n - 10) * 2
    else:
        result = abs(n - 10) + 100
    
    # Should compute abs(n - 10) once
    return result + abs(n - 10)


def nested_expressions(x: int, y: int) -> int:
    """Nested common subexpressions."""
    # x * y appears in multiple places
    a = (x * y) + (x * y) * 2
    b = (x * y) * (x * y)
    
    # More complex: (x + 1) * (y + 1)
    c = (x + 1) * (y + 1) + 10
    d = (x + 1) * (y + 1) - 20
    
    return a + b + c + d


def conditional_cse(flag: bool, a: int, b: int) -> int:
    """CSE across conditional branches."""
    base = (a * b) + (a - b)
    
    if flag:
        # Same subexpression in then branch
        result = (a * b) + (a - b) + 10
    else:
        # Same subexpression in else branch
        result = (a * b) + (a - b) - 10
    
    return result + base


def loop_invariant_expressions(items: list) -> int:
    """Expressions that don't change in loops."""
    x = 10
    y = 20
    total = 0
    
    for item in items:
        # (x + y) * 2 is loop invariant
        total += item + (x + y) * 2
    
    return total