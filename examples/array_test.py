"""Test array generation from fixed-size lists"""

def test_array_literals():
    # Small literal arrays should generate Rust arrays
    arr1 = [1, 2, 3, 4, 5]
    arr2 = [0, 0, 0, 0]
    arr3 = [True, False, True]
    
    # Mixed literals
    arr4 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    return arr1, arr2, arr3, arr4

def test_array_multiplication():
    # Array multiplication patterns
    zeros = [0] * 10
    ones = [1] * 5
    pattern = [42] * 8
    
    # Reverse multiplication
    reverse_zeros = 10 * [0]
    reverse_ones = 5 * [1]
    
    return zeros, ones, pattern, reverse_zeros, reverse_ones

def test_array_init_functions():
    # zeros, ones, full functions
    z = zeros(10)
    o = ones(5)
    f = full(8, 42)
    
    return z, o, f

def test_large_arrays():
    # Arrays larger than 32 should use vec!
    large = [0] * 50
    very_large = [1] * 100
    
    # Non-literal arrays should use vec!
    x = 5
    dynamic = [x] * 10
    
    return large, very_large, dynamic

def test_nested_arrays():
    # 2D arrays
    matrix = [[0, 0], [0, 0], [0, 0]]
    identity = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    
    return matrix, identity