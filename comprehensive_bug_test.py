def type_mapping_bug(items: list) -> list:
    """Plain list should map to Vec<T>"""
    return items

def underflow_bug(arr: list) -> int:
    """Array length - 1 can underflow"""
    right = len(arr) - 1
    return right

def method_call_spacing(arr: list) -> int:
    """Method calls have weird spacing"""
    return len(arr)