from typing import Dict

def test_string_dict() -> Dict[str, str]:
    """Test dictionary assignment with string keys"""
    d: Dict[str, str] = {}
    d["key1"] = "value1"
    d["key2"] = "value2"
    return d

def test_int_dict() -> Dict[int, str]:
    """Test dictionary assignment with integer keys"""
    d: Dict[int, str] = {}
    d[42] = "number key"
    d[100] = "another number"
    return d

def test_nested_dict() -> Dict[str, Dict[str, str]]:
    """Test nested dictionary (but not nested assignment yet)"""
    d: Dict[str, Dict[str, str]] = {}
    d["outer"] = {}
    # d["outer"]["inner"] = "value"  # TODO: This requires chained assignment
    return d