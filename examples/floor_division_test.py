def test_floor_division_positive():
    """Test floor division with positive operands"""
    a = 7
    b = 3
    result = a // b
    return result  # Should be 2


def test_floor_division_negative():
    """Test floor division with negative dividend"""
    a = -7
    b = 3
    result = a // b
    return result  # Should be -3 (Python rounds towards negative infinity)


def test_floor_division_negative_divisor():
    """Test floor division with negative divisor"""
    a = 7
    b = -3
    result = a // b
    return result  # Should be -3


def test_floor_division_both_negative():
    """Test floor division with both operands negative"""
    a = -7
    b = -3
    result = a // b
    return result  # Should be 2


def test_floor_division_exact():
    """Test floor division with exact result"""
    a = 9
    b = 3
    result = a // b
    return result  # Should be 3


def test_floor_division_zero_remainder():
    """Test floor division with zero remainder edge case"""
    a = -9
    b = 3
    result = a // b
    return result  # Should be -3