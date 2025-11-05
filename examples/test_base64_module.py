"""
Comprehensive test suite for base64 module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests base64 core features:
- Encoding and decoding with standard alphabet
- URL-safe encoding/decoding
- Padding handling
- Binary data encoding
- Error handling
"""

import base64


def test_base64_encode_basic():
    """Test basic base64 encoding."""
    data = b"Hello, World!"
    encoded = base64.b64encode(data)
    assert encoded == b"SGVsbG8sIFdvcmxkIQ=="
    print("PASS: test_base64_encode_basic")


def test_base64_decode_basic():
    """Test basic base64 decoding."""
    encoded = b"SGVsbG8sIFdvcmxkIQ=="
    decoded = base64.b64decode(encoded)
    assert decoded == b"Hello, World!"
    print("PASS: test_base64_decode_basic")


def test_base64_roundtrip():
    """Test encode-decode round trip."""
    original = b"Python to Rust transpilation!"
    encoded = base64.b64encode(original)
    decoded = base64.b64decode(encoded)
    assert decoded == original
    print("PASS: test_base64_roundtrip")


def test_base64_empty():
    """Test encoding/decoding empty data."""
    data = b""
    encoded = base64.b64encode(data)
    assert encoded == b""

    decoded = base64.b64decode(b"")
    assert decoded == b""
    print("PASS: test_base64_empty")


def test_base64_binary_data():
    """Test encoding binary data."""
    data = bytes(range(256))
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
    print("PASS: test_base64_binary_data")


def test_base64_urlsafe_encode():
    """Test URL-safe base64 encoding."""
    # URL-safe uses - and _ instead of + and /
    data = b"Hello>>???World"
    encoded = base64.urlsafe_b64encode(data)
    # Should not contain + or /
    assert b"+" not in encoded
    assert b"/" not in encoded
    print("PASS: test_base64_urlsafe_encode")


def test_base64_urlsafe_decode():
    """Test URL-safe base64 decoding."""
    data = b"Test data with special chars"
    encoded = base64.urlsafe_b64encode(data)
    decoded = base64.urlsafe_b64decode(encoded)
    assert decoded == data
    print("PASS: test_base64_urlsafe_decode")


def test_base64_padding():
    """Test base64 padding handling."""
    # Different lengths produce different padding
    data1 = b"a"
    encoded1 = base64.b64encode(data1)
    assert encoded1 == b"YQ=="  # 2 padding chars

    data2 = b"ab"
    encoded2 = base64.b64encode(data2)
    assert encoded2 == b"YWI="  # 1 padding char

    data3 = b"abc"
    encoded3 = base64.b64encode(data3)
    assert encoded3 == b"YWJj"  # No padding
    print("PASS: test_base64_padding")


def test_base64_multiline():
    """Test encoding larger data."""
    data = b"The quick brown fox jumps over the lazy dog. " * 10
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
    print("PASS: test_base64_multiline")


def test_base64_unicode():
    """Test encoding Unicode text."""
    text = "Hello 世界 🌍"
    data = text.encode('utf-8')
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    result = decoded.decode('utf-8')
    assert result == text
    print("PASS: test_base64_unicode")


def main():
    """Run all base64 tests."""
    print("=" * 60)
    print("BASE64 MODULE TESTS")
    print("=" * 60)

    test_base64_encode_basic()
    test_base64_decode_basic()
    test_base64_roundtrip()
    test_base64_empty()
    test_base64_binary_data()
    test_base64_urlsafe_encode()
    test_base64_urlsafe_decode()
    test_base64_padding()
    test_base64_multiline()
    test_base64_unicode()

    print("=" * 60)
    print("ALL BASE64 TESTS PASSED!")
    print("Total tests: 10")
    print("=" * 60)


if __name__ == "__main__":
    main()
