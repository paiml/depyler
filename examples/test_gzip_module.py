"""
Comprehensive test suite for gzip module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests gzip core features:
- Compressing and decompressing data
- Working with gzip files
- Compression levels
- Binary data handling
"""

import gzip
from io import BytesIO


def test_gzip_compress_decompress():
    """Test basic compression and decompression."""
    data = b"Hello, this is a test string for compression!"

    # Compress
    compressed = gzip.compress(data)

    # Decompress
    decompressed = gzip.decompress(compressed)

    assert decompressed == data
    # Note: small data may not compress due to gzip header overhead
    print("PASS: test_gzip_compress_decompress")


def test_gzip_compress_text():
    """Test compressing text data."""
    text = "The quick brown fox jumps over the lazy dog. " * 10
    data = text.encode('utf-8')

    compressed = gzip.compress(data)
    decompressed = gzip.decompress(compressed)

    assert decompressed.decode('utf-8') == text
    print("PASS: test_gzip_compress_text")


def test_gzip_compress_empty():
    """Test compressing empty data."""
    data = b""
    compressed = gzip.compress(data)
    decompressed = gzip.decompress(compressed)
    assert decompressed == b""
    print("PASS: test_gzip_compress_empty")


def test_gzip_compress_levels():
    """Test different compression levels."""
    data = b"Test data for compression levels! " * 100

    # Level 1 (fastest, least compression)
    compressed_1 = gzip.compress(data, compresslevel=1)
    decompressed_1 = gzip.decompress(compressed_1)
    assert decompressed_1 == data

    # Level 9 (slowest, best compression)
    compressed_9 = gzip.compress(data, compresslevel=9)
    decompressed_9 = gzip.decompress(compressed_9)
    assert decompressed_9 == data

    # Level 9 should produce smaller output than level 1
    assert len(compressed_9) <= len(compressed_1)
    print("PASS: test_gzip_compress_levels")


def test_gzip_large_data():
    """Test compressing larger data."""
    # Create 1KB of repeated data
    data = b"ABCDEFGHIJ" * 100

    compressed = gzip.compress(data)
    decompressed = gzip.decompress(compressed)

    assert decompressed == data
    # Repeated data should compress very well
    assert len(compressed) < len(data) / 2
    print("PASS: test_gzip_large_data")


def test_gzip_binary_data():
    """Test compressing binary data."""
    # Create binary data with various byte values
    data = bytes(range(256))

    compressed = gzip.compress(data)
    decompressed = gzip.decompress(compressed)

    assert decompressed == data
    print("PASS: test_gzip_binary_data")


def test_gzip_unicode_text():
    """Test compressing Unicode text."""
    text = "Hello 世界 🌍 Привет مرحبا"
    data = text.encode('utf-8')

    compressed = gzip.compress(data)
    decompressed = gzip.decompress(compressed)

    assert decompressed.decode('utf-8') == text
    print("PASS: test_gzip_unicode_text")


def test_gzip_multiple_compress():
    """Test compressing already compressed data."""
    data = b"Original data for double compression test"

    # First compression
    compressed_once = gzip.compress(data)

    # Second compression (compressing already compressed data)
    compressed_twice = gzip.compress(compressed_once)

    # Decompress twice
    decompressed_once = gzip.decompress(compressed_twice)
    decompressed_twice = gzip.decompress(decompressed_once)

    assert decompressed_twice == data
    print("PASS: test_gzip_multiple_compress")


def main():
    """Run all gzip tests."""
    print("=" * 60)
    print("GZIP MODULE TESTS")
    print("=" * 60)

    test_gzip_compress_decompress()
    test_gzip_compress_text()
    test_gzip_compress_empty()
    test_gzip_compress_levels()
    test_gzip_large_data()
    test_gzip_binary_data()
    test_gzip_unicode_text()
    test_gzip_multiple_compress()

    print("=" * 60)
    print("ALL GZIP TESTS PASSED!")
    print("Total tests: 8")
    print("=" * 60)


if __name__ == "__main__":
    main()
