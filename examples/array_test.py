"""Test file for const generic array generation"""

def test_literal_arrays():
    """Test literal array patterns"""
    # Small literal arrays should become Rust arrays
    arr1 = [1, 2, 3, 4, 5]
    arr2 = [0, 0, 0, 0]
    arr3 = ['a', 'b', 'c']
    
    # Array multiplication patterns
    zeros_array = [0] * 10
    ones_array = [1] * 5
    pattern_array = [42] * 8
    
    return arr1, zeros_array, ones_array

def test_array_functions():
    """Test array initialization functions"""
    # Create our own array functions for testing
    def zeros(n: int) -> list[int]:
        return [0] * n
    
    def ones(n: int) -> list[int]:
        return [1] * n
        
    def full(n: int, val: float) -> list[float]:
        return [val] * n
    
    # These should generate fixed-size arrays when size is literal
    z1 = zeros(5)
    o1 = ones(10)
    f1 = full(8, 3.14)
    
    # Large sizes should still use vec!
    z2 = zeros(100)
    
    return z1, o1, f1

def test_nested_arrays():
    """Test nested array patterns"""
    # Simple nested array
    row1 = [1, 2, 3]
    row2 = [4, 5, 6]
    row3 = [7, 8, 9]
    matrix = [row1, row2, row3]
    
    return matrix

def process_fixed_array(data: list[int]) -> list[int]:
    """Process array with known size constraints"""
    # This could potentially use const generics
    result = [0] * 10
    
    for i in range(10):
        if i < len(data):
            result[i] = data[i] * 2
            
    return result