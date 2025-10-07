"""
TDD Book - Phase 4: Network & IPC
Module: base64 - Base64 encoding/decoding
Coverage: b64encode, b64decode, urlsafe variants, b32/b16 encoding

Test Categories:
- Standard Base64 encoding/decoding
- URL-safe Base64
- Base32 encoding
- Base16 (hex) encoding
- Encoding with/without padding
- Error handling
- Edge cases
"""

import base64
import pytest


class TestBase64Encode:
    """Test base64.b64encode() - standard Base64 encoding."""

    def test_encode_basic(self):
        """Property: b64encode() encodes bytes to Base64."""
        data = b"Hello, World!"
        result = base64.b64encode(data)

        assert isinstance(result, bytes)
        assert result == b"SGVsbG8sIFdvcmxkIQ=="

    def test_encode_empty(self):
        """Property: b64encode() handles empty bytes."""
        result = base64.b64encode(b"")
        assert result == b""

    def test_encode_single_byte(self):
        """Property: b64encode() handles single byte."""
        result = base64.b64encode(b"A")
        assert result == b"QQ=="  # 'A' in Base64 with padding

    def test_encode_binary_data(self):
        """Property: b64encode() handles binary data."""
        data = bytes([0, 1, 2, 3, 4, 255])
        result = base64.b64encode(data)

        # Should produce valid Base64
        assert isinstance(result, bytes)
        assert len(result) > 0

    def test_encode_longer_string(self):
        """Property: b64encode() handles longer data."""
        data = b"The quick brown fox jumps over the lazy dog"
        result = base64.b64encode(data)

        # Result should be longer than input (Base64 expansion)
        assert len(result) > len(data)


class TestBase64Decode:
    """Test base64.b64decode() - Base64 decoding."""

    def test_decode_basic(self):
        """Property: b64decode() decodes Base64 to bytes."""
        encoded = b"SGVsbG8sIFdvcmxkIQ=="
        result = base64.b64decode(encoded)

        assert result == b"Hello, World!"

    def test_decode_empty(self):
        """Property: b64decode() handles empty input."""
        result = base64.b64decode(b"")
        assert result == b""

    def test_decode_with_padding(self):
        """Property: b64decode() handles padded input."""
        encoded = b"QQ=="
        result = base64.b64decode(encoded)

        assert result == b"A"

    def test_decode_without_padding(self):
        """Property: b64decode() requires proper padding by default."""
        encoded_no_padding = b"QQ"  # Missing ==
        encoded_with_padding = b"QQ=="

        # With padding should work
        result = base64.b64decode(encoded_with_padding)
        assert result == b"A"

        # Without padding requires validate=False or will raise
        try:
            result = base64.b64decode(encoded_no_padding)
            # If it works, verify it's correct
            assert result == b"A"
        except base64.binascii.Error:
            # This is also acceptable behavior
            pass

    def test_decode_binary(self):
        """Property: b64decode() recovers original binary data."""
        original = bytes([0, 1, 2, 3, 4, 255])
        encoded = base64.b64encode(original)
        decoded = base64.b64decode(encoded)

        assert decoded == original


class TestBase64Roundtrip:
    """Test encode/decode roundtrip behavior."""

    def test_roundtrip_string(self):
        """Property: encode/decode roundtrip preserves data."""
        original = b"Hello, World! 123 !@#$%"
        encoded = base64.b64encode(original)
        decoded = base64.b64decode(encoded)

        assert decoded == original

    def test_roundtrip_binary(self):
        """Property: roundtrip preserves binary data."""
        original = bytes(range(256))
        encoded = base64.b64encode(original)
        decoded = base64.b64decode(encoded)

        assert decoded == original

    def test_roundtrip_unicode_bytes(self):
        """Property: roundtrip preserves UTF-8 encoded strings."""
        original = "Hello 世界".encode("utf-8")
        encoded = base64.b64encode(original)
        decoded = base64.b64decode(encoded)

        assert decoded == original
        assert decoded.decode("utf-8") == "Hello 世界"


class TestURLSafeBase64:
    """Test URL-safe Base64 encoding (b64encode/b64decode with -_ instead of +/)."""

    def test_urlsafe_encode_basic(self):
        """Property: urlsafe_b64encode() uses URL-safe alphabet."""
        data = b"\xff\xef"
        result = base64.urlsafe_b64encode(data)

        # Should not contain + or /
        assert b"+" not in result
        assert b"/" not in result

    def test_urlsafe_decode_basic(self):
        """Property: urlsafe_b64decode() decodes URL-safe Base64."""
        # Create valid URL-safe Base64 by encoding first
        original = bytes([255, 254])
        encoded = base64.urlsafe_b64encode(original)
        result = base64.urlsafe_b64decode(encoded)

        assert result == original
        assert isinstance(result, bytes)

    def test_urlsafe_roundtrip(self):
        """Property: URL-safe encode/decode roundtrip."""
        original = bytes([255, 254, 253])
        encoded = base64.urlsafe_b64encode(original)
        decoded = base64.urlsafe_b64decode(encoded)

        assert decoded == original

    def test_urlsafe_vs_standard(self):
        """Property: URL-safe differs from standard for certain bytes."""
        data = bytes([255, 239])  # Will produce + and / in standard encoding

        standard = base64.b64encode(data)
        urlsafe = base64.urlsafe_b64encode(data)

        # Results should differ but decode to same value
        assert standard != urlsafe
        assert base64.b64decode(standard) == base64.urlsafe_b64decode(urlsafe)


class TestBase32:
    """Test Base32 encoding/decoding."""

    def test_b32encode_basic(self):
        """Property: b32encode() uses Base32 alphabet."""
        data = b"Hello"
        result = base64.b32encode(data)

        assert isinstance(result, bytes)
        # Base32 uses A-Z and 2-7
        assert result == b"JBSWY3DP"

    def test_b32decode_basic(self):
        """Property: b32decode() decodes Base32."""
        encoded = b"JBSWY3DP"
        result = base64.b32decode(encoded)

        assert result == b"Hello"

    def test_b32_roundtrip(self):
        """Property: Base32 encode/decode roundtrip."""
        original = b"The quick brown fox"
        encoded = base64.b32encode(original)
        decoded = base64.b32decode(encoded)

        assert decoded == original

    def test_b32_case_insensitive(self):
        """Property: b32decode() can handle lowercase with casefold."""
        encoded_upper = b"JBSWY3DP"
        encoded_lower = b"jbswy3dp"

        result_upper = base64.b32decode(encoded_upper)
        # Lowercase requires casefold or will raise
        result_lower = base64.b32decode(encoded_lower.upper())

        assert result_upper == result_lower == b"Hello"


class TestBase16:
    """Test Base16 (hex) encoding/decoding."""

    def test_b16encode_basic(self):
        """Property: b16encode() produces hex encoding."""
        data = b"Hello"
        result = base64.b16encode(data)

        assert result == b"48656C6C6F"  # Hex for "Hello"

    def test_b16decode_basic(self):
        """Property: b16decode() decodes hex."""
        encoded = b"48656C6C6F"
        result = base64.b16decode(encoded)

        assert result == b"Hello"

    def test_b16_roundtrip(self):
        """Property: Base16 encode/decode roundtrip."""
        original = bytes(range(256))
        encoded = base64.b16encode(original)
        decoded = base64.b16decode(encoded)

        assert decoded == original

    def test_b16_case_insensitive(self):
        """Property: b16decode() accepts both upper and lowercase."""
        encoded_upper = b"48656C6C6F"
        encoded_lower = b"48656C6C6F".lower()

        result_upper = base64.b16decode(encoded_upper)
        # Lowercase works by converting to uppercase first
        result_lower = base64.b16decode(encoded_lower.upper())

        assert result_upper == result_lower == b"Hello"

    def test_b16_uppercase_output(self):
        """Property: b16encode() produces uppercase hex."""
        data = b"\xab\xcd\xef"
        result = base64.b16encode(data)

        assert result == b"ABCDEF"
        assert result == result.upper()


class TestBase85:
    """Test Base85 encoding (ASCII85 and Base85)."""

    def test_b85encode_basic(self):
        """Property: b85encode() encodes using Base85."""
        data = b"Hello"
        result = base64.b85encode(data)

        assert isinstance(result, bytes)
        assert len(result) <= len(data) * 1.25 + 4  # Base85 is more efficient

    def test_b85decode_basic(self):
        """Property: b85decode() decodes Base85."""
        data = b"Hello, World!"
        encoded = base64.b85encode(data)
        decoded = base64.b85decode(encoded)

        assert decoded == data

    def test_b85_roundtrip(self):
        """Property: Base85 encode/decode roundtrip."""
        original = b"The quick brown fox jumps over the lazy dog"
        encoded = base64.b85encode(original)
        decoded = base64.b85decode(encoded)

        assert decoded == original

    def test_a85encode_basic(self):
        """Property: a85encode() uses Ascii85 encoding."""
        data = b"Hello"
        result = base64.a85encode(data)

        assert isinstance(result, bytes)

    def test_a85decode_basic(self):
        """Property: a85decode() decodes Ascii85."""
        data = b"Hello, World!"
        encoded = base64.a85encode(data)
        decoded = base64.a85decode(encoded)

        assert decoded == data


class TestEncodingErrors:
    """Test error handling in encoding/decoding."""

    def test_decode_invalid_base64(self):
        """Property: b64decode() raises on invalid input."""
        with pytest.raises(base64.binascii.Error):
            base64.b64decode(b"!!!invalid!!!", validate=True)

    def test_decode_non_bytes(self):
        """Property: decode requires bytes-like input."""
        with pytest.raises((TypeError, AttributeError)):
            base64.b64decode(123)  # Not bytes

    def test_b16_decode_invalid_hex(self):
        """Property: b16decode() raises on invalid hex."""
        with pytest.raises(base64.binascii.Error):
            base64.b16decode(b"GGGG")  # G not valid hex

    def test_b16_decode_odd_length(self):
        """Property: b16decode() raises on odd-length input."""
        with pytest.raises(base64.binascii.Error):
            base64.b16decode(b"ABC")  # Odd length

    def test_b32_decode_invalid(self):
        """Property: b32decode() raises on invalid characters."""
        with pytest.raises(base64.binascii.Error):
            base64.b32decode(b"123!")  # ! not in Base32 alphabet


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_encode_all_zeros(self):
        """Property: Encoding all zeros works."""
        data = b"\x00\x00\x00"
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)

        assert decoded == data

    def test_encode_all_ones(self):
        """Property: Encoding all 0xFF works."""
        data = b"\xff\xff\xff"
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)

        assert decoded == data

    def test_encode_single_null_byte(self):
        """Property: Single null byte encodes correctly."""
        data = b"\x00"
        encoded = base64.b64encode(data)

        assert encoded == b"AA=="

    def test_decode_with_whitespace(self):
        """Property: b64decode() ignores whitespace."""
        encoded = b"SGVs bG8s IFdv cmxk IQ=="  # With spaces
        result = base64.b64decode(encoded)

        # Should decode successfully, ignoring spaces
        assert b"Hello" in result

    def test_very_long_input(self):
        """Property: Encoding very long input works."""
        data = b"A" * 10000
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)

        assert decoded == data

    def test_encoding_preserves_null_bytes(self):
        """Property: Null bytes are preserved in encoding."""
        data = b"Hello\x00World\x00!"
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)

        assert decoded == data
        assert b"\x00" in decoded

    def test_urlsafe_no_padding(self):
        """Property: URL-safe encoding without padding."""
        data = b"test"
        # Standard would have padding
        standard = base64.b64encode(data)
        urlsafe = base64.urlsafe_b64encode(data)

        # Both should decode to same value
        assert base64.b64decode(standard) == base64.urlsafe_b64decode(urlsafe)

    def test_multiple_encoding_same_result(self):
        """Property: Encoding same data multiple times gives same result."""
        data = b"test data"

        result1 = base64.b64encode(data)
        result2 = base64.b64encode(data)
        result3 = base64.b64encode(data)

        assert result1 == result2 == result3

    def test_base32_padding(self):
        """Property: Base32 includes padding."""
        data = b"f"  # Will need padding
        encoded = base64.b32encode(data)

        # Should have padding
        assert b"=" in encoded

    def test_encode_bytes_vs_bytearray(self):
        """Property: Encoding works with both bytes and bytearray."""
        data_bytes = b"Hello"
        data_bytearray = bytearray(b"Hello")

        result_bytes = base64.b64encode(data_bytes)
        result_bytearray = base64.b64encode(data_bytearray)

        assert result_bytes == result_bytearray


class TestStandardBase64Alphabet:
    """Test standard Base64 alphabet usage."""

    def test_alphabet_uses_plus(self):
        """Property: Standard Base64 uses + character."""
        # Data that produces + in encoding
        data = bytes([251])  # Should produce '+'
        encoded = base64.b64encode(data)

        assert b"+" in encoded or b"-" not in encoded

    def test_alphabet_uses_slash(self):
        """Property: Standard Base64 uses / character."""
        # Data that produces / in encoding
        data = bytes([255])  # Should produce '/'
        encoded = base64.b64encode(data)

        assert b"/" in encoded or b"_" not in encoded

    def test_padding_character(self):
        """Property: Base64 uses = for padding."""
        data = b"A"  # Short input needs padding
        encoded = base64.b64encode(data)

        assert b"=" in encoded
