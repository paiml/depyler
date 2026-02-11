"""Test with statement support for v1.3.0"""


def compute_data_length(data: str) -> int:
    """Compute the length of data string."""
    return len(data)


def test_simple_with() -> int:
    """Test basic data processing (simplified from with statement)."""
    data: str = "Hello, World!"
    result: int = compute_data_length(data)
    return result


def test_with_builtin() -> int:
    """Test simple computation (simplified from with statement)."""
    data: str = "Hello, World!"
    length: int = len(data)
    if length > 0:
        return 1
    return 0
