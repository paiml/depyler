"""
Comprehensive string method test for Depyler transpiler
Tests all implemented Python string methods

Purpose: Verify that string methods transpile correctly to Rust
Expected: All methods should generate idiomatic, compilable Rust code
"""

# ============================================================================
# STRING TRANSFORMATION METHODS
# ============================================================================

def test_str_upper() -> str:
    """Test str.upper() method"""
    text = "hello world"
    result = text.upper()
    return result  # Expected: "HELLO WORLD"


def test_str_lower() -> str:
    """Test str.lower() method"""
    text = "HELLO WORLD"
    result = text.lower()
    return result  # Expected: "hello world"


def test_str_strip() -> str:
    """Test str.strip() method"""
    text = "  hello world  "
    result = text.strip()
    return result  # Expected: "hello world"


# ============================================================================
# STRING QUERY METHODS
# ============================================================================

def test_str_startswith() -> bool:
    """Test str.startswith() method"""
    text = "hello world"
    result = text.startswith("hello")
    return result  # Expected: True


def test_str_startswith_false() -> bool:
    """Test str.startswith() returns False"""
    text = "hello world"
    result = text.startswith("world")
    return result  # Expected: False


def test_str_endswith() -> bool:
    """Test str.endswith() method"""
    text = "hello world"
    result = text.endswith("world")
    return result  # Expected: True


def test_str_endswith_false() -> bool:
    """Test str.endswith() returns False"""
    text = "hello world"
    result = text.endswith("hello")
    return result  # Expected: False


# ============================================================================
# STRING SPLITTING AND JOINING
# ============================================================================

def test_str_split_whitespace() -> int:
    """Test str.split() with default whitespace"""
    text = "hello world foo bar"
    parts = text.split()
    return len(parts)  # Expected: 4


def test_str_split_separator() -> int:
    """Test str.split(sep) with custom separator"""
    text = "hello,world,foo,bar"
    parts = text.split(",")
    return len(parts)  # Expected: 4


def test_str_join() -> str:
    """Test str.join() method"""
    parts = ["hello", "world"]
    result = ",".join(parts)
    return result  # Expected: "hello,world"


def test_str_join_space() -> str:
    """Test str.join() with space separator"""
    parts = ["hello", "world", "foo"]
    result = " ".join(parts)
    return result  # Expected: "hello world foo"


# ============================================================================
# STRING SEARCH AND REPLACE
# ============================================================================

def test_str_find_found() -> int:
    """Test str.find() when substring exists"""
    text = "hello world"
    pos = text.find("world")
    return pos  # Expected: 6


def test_str_find_not_found() -> int:
    """Test str.find() when substring doesn't exist"""
    text = "hello world"
    pos = text.find("xyz")
    return pos  # Expected: -1


def test_str_replace() -> str:
    """Test str.replace() method"""
    text = "hello world"
    result = text.replace("world", "rust")
    return result  # Expected: "hello rust"


def test_str_replace_multiple() -> str:
    """Test str.replace() with multiple occurrences"""
    text = "hello hello hello"
    result = text.replace("hello", "hi")
    return result  # Expected: "hi hi hi"


def test_str_count() -> int:
    """Test str.count() method"""
    text = "hello hello world"
    count = text.count("hello")
    return count  # Expected: 2


def test_str_count_single() -> int:
    """Test str.count() with single occurrence"""
    text = "hello world"
    count = text.count("world")
    return count  # Expected: 1


def test_str_count_none() -> int:
    """Test str.count() with no occurrences"""
    text = "hello world"
    count = text.count("xyz")
    return count  # Expected: 0


# ============================================================================
# STRING CLASSIFICATION METHODS
# ============================================================================

def test_str_isdigit_true() -> bool:
    """Test str.isdigit() returns True for digits"""
    text = "12345"
    result = text.isdigit()
    return result  # Expected: True


def test_str_isdigit_false() -> bool:
    """Test str.isdigit() returns False for non-digits"""
    text = "hello"
    result = text.isdigit()
    return result  # Expected: False


def test_str_isalpha_true() -> bool:
    """Test str.isalpha() returns True for letters"""
    text = "hello"
    result = text.isalpha()
    return result  # Expected: True


def test_str_isalpha_false() -> bool:
    """Test str.isalpha() returns False for non-letters"""
    text = "hello123"
    result = text.isalpha()
    return result  # Expected: False


# ============================================================================
# EDGE CASES
# ============================================================================

def test_str_empty_split() -> int:
    """Test split on empty string"""
    text = ""
    parts = text.split()
    return len(parts)  # Expected: 0


def test_str_single_char() -> str:
    """Test string methods on single character"""
    text = "a"
    result = text.upper()
    return result  # Expected: "A"


def test_str_special_chars() -> bool:
    """Test string methods with special characters"""
    text = "hello-world_123"
    result = text.startswith("hello")
    return result  # Expected: True
