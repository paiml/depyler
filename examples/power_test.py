"""Test power operator (**) transpilation"""

def test_integer_power():
    # Basic integer powers
    a = 2 ** 3      # 8
    b = 10 ** 2     # 100
    c = 5 ** 0      # 1
    
    # Variables
    base = 3
    exp = 4
    d = base ** exp  # 81
    
    return a, b, c, d

def test_float_power():
    # Float base
    a = 2.5 ** 2    # 6.25
    b = 10.0 ** 3   # 1000.0
    
    # Float exponent
    c = 4 ** 0.5    # 2.0 (square root)
    d = 8 ** (1/3)  # 2.0 (cube root)
    
    return a, b, c, d

def test_negative_exponent():
    # Negative exponents should convert to float
    a = 2 ** -1     # 0.5
    b = 10 ** -2    # 0.01
    c = 5 ** -3     # 0.008
    
    return a, b, c

def test_large_powers():
    # Large powers that might overflow
    a = 2 ** 10     # 1024
    b = 2 ** 20     # 1048576
    c = 10 ** 6     # 1000000
    
    return a, b, c

def test_mixed_operations():
    # Power with other operations
    a = 2 + 3 ** 2      # 11
    b = (2 + 3) ** 2    # 25
    c = 2 ** 3 * 4      # 32
    d = 2 ** (3 * 2)    # 64
    
    return a, b, c, d

def compute_power(base: int, exp: int) -> int:
    """Test power with function parameters"""
    return base ** exp