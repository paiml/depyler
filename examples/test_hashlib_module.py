"""
Comprehensive test suite for hashlib module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests hashlib core features:
- Common hash algorithms (SHA256, SHA1, MD5)
- Hash updates and digests
- Hexdigest and raw digest
- Hash of binary data
"""

import hashlib


def test_sha256_basic():
    """Test basic SHA256 hashing."""
    data = b"Hello, World!"
    hash_obj = hashlib.sha256(data)
    result = hash_obj.hexdigest()

    # Known SHA256 hash for "Hello, World!"
    expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    assert result == expected
    print("PASS: test_sha256_basic")


def test_sha256_empty():
    """Test SHA256 of empty data."""
    data = b""
    hash_obj = hashlib.sha256(data)
    result = hash_obj.hexdigest()

    # Known SHA256 hash for empty string
    expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    assert result == expected
    print("PASS: test_sha256_empty")


def test_sha256_update():
    """Test SHA256 with multiple updates."""
    hash_obj = hashlib.sha256()
    hash_obj.update(b"Hello, ")
    hash_obj.update(b"World!")
    result = hash_obj.hexdigest()

    # Should match single-call hash
    expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    assert result == expected
    print("PASS: test_sha256_update")


def test_sha1_basic():
    """Test basic SHA1 hashing."""
    data = b"test data"
    hash_obj = hashlib.sha1(data)
    result = hash_obj.hexdigest()

    # Known SHA1 hash for "test data"
    expected = "f48dd853820860816c75d54d0f584dc863327a7c"
    assert result == expected
    print("PASS: test_sha1_basic")


def test_md5_basic():
    """Test basic MD5 hashing."""
    data = b"test"
    hash_obj = hashlib.md5(data)
    result = hash_obj.hexdigest()

    # Known MD5 hash for "test"
    expected = "098f6bcd4621d373cade4e832627b4f6"
    assert result == expected
    print("PASS: test_md5_basic")


def test_sha256_binary_data():
    """Test hashing binary data."""
    data = bytes(range(256))
    hash_obj = hashlib.sha256(data)
    result = hash_obj.hexdigest()

    # Just verify it returns a 64-char hex string
    assert len(result) == 64
    assert all(c in "0123456789abcdef" for c in result)
    print("PASS: test_sha256_binary_data")


def test_sha256_large_data():
    """Test hashing larger data."""
    data = b"A" * 10000
    hash_obj = hashlib.sha256(data)
    result = hash_obj.hexdigest()

    # Verify it's a valid hex string
    assert len(result) == 64
    assert all(c in "0123456789abcdef" for c in result)
    print("PASS: test_sha256_large_data")


def test_hash_different_data():
    """Test that different data produces different hashes."""
    hash1 = hashlib.sha256(b"data1").hexdigest()
    hash2 = hashlib.sha256(b"data2").hexdigest()

    assert hash1 != hash2
    print("PASS: test_hash_different_data")


def test_hash_deterministic():
    """Test that same data produces same hash."""
    data = b"deterministic test"
    hash1 = hashlib.sha256(data).hexdigest()
    hash2 = hashlib.sha256(data).hexdigest()

    assert hash1 == hash2
    print("PASS: test_hash_deterministic")


def test_sha256_text():
    """Test hashing text (encoded to bytes)."""
    text = "Hello, 世界!"
    data = text.encode('utf-8')
    hash_obj = hashlib.sha256(data)
    result = hash_obj.hexdigest()

    # Verify it's a valid hash
    assert len(result) == 64
    assert all(c in "0123456789abcdef" for c in result)
    print("PASS: test_sha256_text")


def main():
    """Run all hashlib tests."""
    print("=" * 60)
    print("HASHLIB MODULE TESTS")
    print("=" * 60)

    test_sha256_basic()
    test_sha256_empty()
    test_sha256_update()
    test_sha1_basic()
    test_md5_basic()
    test_sha256_binary_data()
    test_sha256_large_data()
    test_hash_different_data()
    test_hash_deterministic()
    test_sha256_text()

    print("=" * 60)
    print("ALL HASHLIB TESTS PASSED!")
    print("Total tests: 10")
    print("=" * 60)


if __name__ == "__main__":
    main()
