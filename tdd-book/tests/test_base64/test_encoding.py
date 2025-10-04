"""Test base64 module - Base64 encoding and decoding.

This module tests base64 for encoding binary data to ASCII text
and decoding it back, including standard, URL-safe, and other variants.
"""

import base64
import pytest


class TestStandardBase64:
    """Standard Base64 encoding/decoding."""

    def test_encode_basic(self):
        """Basic: Encode bytes to base64."""
        data = b"hello"
        encoded = base64.b64encode(data)
        assert encoded == b"aGVsbG8="
        assert isinstance(encoded, bytes)

    def test_decode_basic(self):
        """Basic: Decode base64 to bytes."""
        encoded = b"aGVsbG8="
        decoded = base64.b64decode(encoded)
        assert decoded == b"hello"

    def test_roundtrip(self):
        """Property: Encode then decode returns original."""
        data = b"The quick brown fox"
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)
        assert decoded == data

    def test_empty_bytes(self):
        """Edge: Encode empty bytes."""
        encoded = base64.b64encode(b"")
        assert encoded == b""
        decoded = base64.b64decode(b"")
        assert decoded == b""

    def test_single_byte(self):
        """Edge: Encode single byte."""
        encoded = base64.b64encode(b"A")
        assert encoded == b"QQ=="  # Padding to 4 chars

    def test_padding(self):
        """Property: Base64 uses = for padding."""
        # 1 byte -> 2 chars + 2 padding
        assert base64.b64encode(b"A") == b"QQ=="
        # 2 bytes -> 3 chars + 1 padding
        assert base64.b64encode(b"AB") == b"QUI="
        # 3 bytes -> 4 chars + 0 padding
        assert base64.b64encode(b"ABC") == b"QUJD"


class TestUrlSafeBase64:
    """URL-safe Base64 encoding/decoding."""

    def test_encode_urlsafe(self):
        """Feature: URL-safe encoding uses - and _ instead of + and /."""
        data = b"\xfb\xff\xfe"  # Results in + and / in standard base64
        standard = base64.b64encode(data)
        urlsafe = base64.urlsafe_b64encode(data)
        # Standard has + and /
        # URL-safe replaces them with - and _
        assert b"+" not in urlsafe or b"/" not in urlsafe

    def test_decode_urlsafe(self):
        """Feature: Decode URL-safe base64."""
        encoded = b"-_-_"  # URL-safe characters
        decoded = base64.urlsafe_b64decode(encoded)
        assert isinstance(decoded, bytes)

    def test_urlsafe_roundtrip(self):
        """Property: URL-safe encode/decode roundtrip."""
        data = b"URL safe encoding test \xff\xfe"
        encoded = base64.urlsafe_b64encode(data)
        decoded = base64.urlsafe_b64decode(encoded)
        assert decoded == data

    def test_urlsafe_no_special_chars(self):
        """Property: URL-safe has no + or / characters."""
        data = bytes(range(256))
        encoded = base64.urlsafe_b64encode(data)
        assert b"+" not in encoded
        assert b"/" not in encoded


class TestBase32:
    """Base32 encoding/decoding."""

    def test_encode_base32(self):
        """Feature: Base32 encoding."""
        data = b"hello"
        encoded = base64.b32encode(data)
        assert encoded == b"NBSWY3DP"
        assert isinstance(encoded, bytes)

    def test_decode_base32(self):
        """Feature: Base32 decoding."""
        encoded = b"NBSWY3DP"
        decoded = base64.b32decode(encoded)
        assert decoded == b"hello"

    def test_base32_roundtrip(self):
        """Property: Base32 roundtrip."""
        data = b"Base32 test data"
        encoded = base64.b32encode(data)
        decoded = base64.b32decode(encoded)
        assert decoded == data

    def test_base32_uppercase(self):
        """Property: Base32 uses uppercase letters."""
        data = b"test"
        encoded = base64.b32encode(data)
        assert encoded == encoded.upper()

    def test_base32_padding(self):
        """Property: Base32 uses = for padding."""
        # Different lengths produce different padding
        assert b"=" in base64.b32encode(b"a")


class TestBase16:
    """Base16 (hex) encoding/decoding."""

    def test_encode_base16(self):
        """Feature: Base16 encoding (hexadecimal)."""
        data = b"hello"
        encoded = base64.b16encode(data)
        assert encoded == b"68656C6C6F"  # Uppercase hex

    def test_decode_base16(self):
        """Feature: Base16 decoding."""
        encoded = b"68656C6C6F"
        decoded = base64.b16decode(encoded)
        assert decoded == b"hello"

    def test_base16_roundtrip(self):
        """Property: Base16 roundtrip."""
        data = bytes(range(256))
        encoded = base64.b16encode(data)
        decoded = base64.b16decode(encoded)
        assert decoded == data

    def test_base16_uppercase(self):
        """Property: Base16 encoding is uppercase."""
        data = b"test"
        encoded = base64.b16encode(data)
        assert encoded == encoded.upper()

    def test_base16_lowercase_decode(self):
        """Feature: Base16 can decode lowercase."""
        lowercase = b"68656c6c6f"
        decoded = base64.b16decode(lowercase, casefold=True)
        assert decoded == b"hello"


class TestBase85:
    """Base85 encoding/decoding."""

    def test_encode_base85(self):
        """Feature: Base85 encoding (Ascii85)."""
        data = b"hello"
        encoded = base64.b85encode(data)
        assert isinstance(encoded, bytes)
        assert len(encoded) > 0

    def test_decode_base85(self):
        """Feature: Base85 decoding."""
        data = b"hello world"
        encoded = base64.b85encode(data)
        decoded = base64.b85decode(encoded)
        assert decoded == data

    def test_base85_roundtrip(self):
        """Property: Base85 roundtrip."""
        data = b"Base85 is more efficient than base64"
        encoded = base64.b85encode(data)
        decoded = base64.b85decode(encoded)
        assert decoded == data

    def test_base85_efficiency(self):
        """Property: Base85 is more efficient than base64."""
        data = b"test data for comparison"
        b64 = base64.b64encode(data)
        b85 = base64.b85encode(data)
        # Base85 should be shorter
        assert len(b85) <= len(b64)


class TestAscii85:
    """Ascii85 encoding/decoding."""

    def test_encode_ascii85(self):
        """Feature: Ascii85 encoding."""
        data = b"hello"
        encoded = base64.a85encode(data)
        assert isinstance(encoded, bytes)

    def test_decode_ascii85(self):
        """Feature: Ascii85 decoding."""
        data = b"test"
        encoded = base64.a85encode(data)
        decoded = base64.a85decode(encoded)
        assert decoded == data

    def test_ascii85_roundtrip(self):
        """Property: Ascii85 roundtrip."""
        data = b"Ascii85 encoding test"
        encoded = base64.a85encode(data)
        decoded = base64.a85decode(encoded)
        assert decoded == data


class TestBinaryData:
    """Binary data encoding."""

    def test_encode_binary(self):
        """Feature: Encode arbitrary binary data."""
        data = bytes(range(256))
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)
        assert decoded == data

    def test_encode_nulls(self):
        """Edge: Encode data with null bytes."""
        data = b"\x00\x00\x00"
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)
        assert decoded == data

    def test_encode_high_bytes(self):
        """Edge: Encode high byte values."""
        data = b"\xff\xfe\xfd\xfc"
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)
        assert decoded == data


class TestValidation:
    """Input validation and errors."""

    def test_error_decode_invalid_base64(self):
        """Error: Decode invalid base64."""
        with pytest.raises(Exception):  # binascii.Error
            base64.b64decode(b"not valid base64!!!", validate=True)

    def test_decode_without_validation(self):
        """Feature: Decode without validation skips invalid chars."""
        # Non-base64 chars are ignored by default
        result = base64.b64decode(b"aGVs bG8=")  # Space is ignored
        assert result == b"hello"

    def test_error_decode_incorrect_padding(self):
        """Error: Invalid padding in strict mode."""
        with pytest.raises(Exception):
            base64.b64decode(b"aGVsbG8", validate=True)  # Missing padding

    def test_error_encode_string(self):
        """Error: Cannot encode string, must be bytes."""
        with pytest.raises(TypeError):
            base64.b64encode("string")  # Must be bytes


class TestStandardEncoding:
    """Standard encoding properties."""

    def test_alphabet_base64(self):
        """Property: Base64 uses A-Z, a-z, 0-9, +, /."""
        data = bytes(range(256))
        encoded = base64.b64encode(data)
        valid_chars = set(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=")
        assert all(byte in valid_chars for byte in encoded)

    def test_alphabet_urlsafe(self):
        """Property: URL-safe uses A-Z, a-z, 0-9, -, _."""
        data = bytes(range(256))
        encoded = base64.urlsafe_b64encode(data)
        valid_chars = set(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=")
        assert all(byte in valid_chars for byte in encoded)

    def test_deterministic(self):
        """Property: Encoding is deterministic."""
        data = b"test data"
        encoded1 = base64.b64encode(data)
        encoded2 = base64.b64encode(data)
        assert encoded1 == encoded2

    def test_case_sensitive(self):
        """Property: Base64 is case-sensitive."""
        # Different cases produce different decoded results
        upper = base64.b64decode(b"QQ==")  # 'A'
        # b64decode is case-sensitive for letters
        assert isinstance(upper, bytes)


class TestEncodedLength:
    """Encoded length properties."""

    def test_base64_expansion(self):
        """Property: Base64 expands data by ~33%."""
        data = b"x" * 100
        encoded = base64.b64encode(data)
        # Base64 is 4/3 the size (plus padding)
        assert len(encoded) >= len(data) * 4 // 3

    def test_base32_expansion(self):
        """Property: Base32 expands data by ~60%."""
        data = b"x" * 100
        encoded = base64.b32encode(data)
        # Base32 is 8/5 the size
        assert len(encoded) >= len(data) * 8 // 5

    def test_base16_doubles(self):
        """Property: Base16 doubles data size."""
        data = b"test"
        encoded = base64.b16encode(data)
        # Each byte becomes 2 hex chars
        assert len(encoded) == len(data) * 2


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_large_data(self):
        """Performance: Encode large data."""
        data = b"x" * 1000000  # 1MB
        encoded = base64.b64encode(data)
        decoded = base64.b64decode(encoded)
        assert decoded == data

    def test_newlines_ignored(self):
        """Feature: Newlines are ignored in decoding."""
        encoded_with_newlines = b"aGVs\nbG8="
        decoded = base64.b64decode(encoded_with_newlines)
        assert decoded == b"hello"

    def test_whitespace_ignored(self):
        """Feature: Whitespace is ignored in decoding."""
        encoded_with_spaces = b"aGVs bG8="
        decoded = base64.b64decode(encoded_with_spaces)
        assert decoded == b"hello"

    def test_decode_bytes_or_ascii(self):
        """Feature: Can decode bytes or ASCII string."""
        data = b"hello"
        encoded_bytes = base64.b64encode(data)
        encoded_str = encoded_bytes.decode('ascii')

        decoded_from_bytes = base64.b64decode(encoded_bytes)
        decoded_from_str = base64.b64decode(encoded_str)

        assert decoded_from_bytes == decoded_from_str == data

    def test_altchars_parameter(self):
        """Feature: Custom altchars for base64."""
        data = b"\xfb\xff"
        # Use custom characters instead of + and /
        encoded = base64.b64encode(data, altchars=b"-_")
        decoded = base64.b64decode(encoded, altchars=b"-_")
        assert decoded == data

    def test_urlsafe_is_altchars(self):
        """Property: urlsafe is equivalent to altchars='-_'."""
        data = b"test \xff\xfe"
        urlsafe = base64.urlsafe_b64encode(data)
        altchars = base64.b64encode(data, altchars=b"-_")
        assert urlsafe == altchars

    def test_encode_empty_preserves_type(self):
        """Property: Encoding empty bytes returns empty bytes."""
        result = base64.b64encode(b"")
        assert result == b""
        assert isinstance(result, bytes)


class TestUseCases:
    """Real-world use cases."""

    def test_encode_for_url(self):
        """Use case: Encode data for URL."""
        data = b"user_id=12345&token=secret"
        encoded = base64.urlsafe_b64encode(data)
        # Safe to use in URLs (no +, /, or =)
        assert b"+" not in encoded
        assert b"/" not in encoded

    def test_encode_for_json(self):
        """Use case: Encode binary for JSON."""
        binary_data = b"\x00\x01\x02\xff\xfe"
        encoded = base64.b64encode(binary_data).decode('ascii')
        # Can be stored as JSON string
        assert isinstance(encoded, str)
        # And decoded back
        decoded = base64.b64decode(encoded.encode('ascii'))
        assert decoded == binary_data

    def test_encode_image_data(self):
        """Use case: Encode image-like binary data."""
        # Simulate image header (PNG magic number)
        png_header = b"\x89PNG\r\n\x1a\n"
        encoded = base64.b64encode(png_header)
        decoded = base64.b64decode(encoded)
        assert decoded == png_header

    def test_data_uri(self):
        """Use case: Create data URI."""
        data = b"Hello, World!"
        encoded = base64.b64encode(data).decode('ascii')
        data_uri = f"data:text/plain;base64,{encoded}"
        assert "base64," in data_uri


class TestB32Hexadecimal:
    """Base32 hexadecimal variant."""

    def test_b32hexencode(self):
        """Feature: Base32 hex encoding."""
        data = b"hello"
        encoded = base64.b32hexencode(data)
        assert isinstance(encoded, bytes)

    def test_b32hexdecode(self):
        """Feature: Base32 hex decoding."""
        data = b"test"
        encoded = base64.b32hexencode(data)
        decoded = base64.b32hexdecode(encoded)
        assert decoded == data

    def test_b32hex_roundtrip(self):
        """Property: Base32 hex roundtrip."""
        data = b"Base32 hex variant"
        encoded = base64.b32hexencode(data)
        decoded = base64.b32hexdecode(encoded)
        assert decoded == data

    def test_b32hex_different_alphabet(self):
        """Property: Base32 hex uses different alphabet than standard."""
        data = b"test"
        standard = base64.b32encode(data)
        hexvariant = base64.b32hexencode(data)
        # Different encodings
        assert standard != hexvariant


class TestDecodeValidation:
    """Decoding with validation."""

    def test_validate_true(self):
        """Feature: Strict validation with validate=True."""
        valid = b"aGVsbG8="
        result = base64.b64decode(valid, validate=True)
        assert result == b"hello"

    def test_validate_rejects_invalid(self):
        """Feature: validate=True rejects invalid input."""
        invalid = b"not@valid!"
        with pytest.raises(Exception):
            base64.b64decode(invalid, validate=True)

    def test_default_validation_lenient(self):
        """Property: Default validation is lenient."""
        # By default, invalid chars are ignored
        lenient = b"aGVs bG8="  # Space in middle
        result = base64.b64decode(lenient)
        assert result == b"hello"
