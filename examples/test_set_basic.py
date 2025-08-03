def test_set_creation():
    # Empty set
    s1 = set()
    
    # Set with integers
    s2 = {1, 2, 3, 4, 5}
    
    # Set with strings
    s3 = {"apple", "banana", "cherry"}
    
    # Set from list
    s4 = set([1, 2, 3, 3, 4, 4, 5])
    
    return s2

def test_set_with_duplicates():
    # Duplicates should be removed
    s = {1, 2, 2, 3, 3, 3, 4}
    return s