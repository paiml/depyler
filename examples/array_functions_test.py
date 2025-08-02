"""Test array initialization functions"""

def test_array_functions():
    # Small arrays should be generated as [T; N]
    z1 = zeros(5)      # Should become [0; 5]
    o1 = ones(10)      # Should become [1; 10]
    f1 = full(8, 42)   # Should become [42; 8]
    
    # Large arrays use vec!
    z2 = zeros(100)    # Should become vec![0; 100]
    
    return z1[0] + o1[0] + f1[0]