def test_frozenset():
    """Test frozenset functionality"""
    # Empty frozenset
    fs1 = frozenset()
    
    # Frozenset from list
    fs2 = frozenset([1, 2, 3])
    
    # Frozenset from tuple
    fs3 = frozenset((4, 5, 6))
    
    return fs1, fs2, fs3