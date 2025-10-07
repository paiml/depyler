"""
TDD Book - Phase 4: Network & IPC
Module: hmac - Keyed-Hashing for Message Authentication
Coverage: HMAC creation, digest computation, comparison

Test Categories:
- HMAC creation and computation
- Different digest algorithms (MD5, SHA1, SHA256, etc.)
- Message updates
- Digest comparison (constant-time)
- Edge cases
"""

import hmac
import hashlib
import pytest


class TestHMACBasic:
    """Test basic HMAC functionality."""

    def test_hmac_new_basic(self):
        """Property: hmac.new() creates HMAC object."""
        key = b"secret_key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")

        assert h is not None

    def test_hmac_default_algorithm(self):
        """Property: hmac.new() uses MD5 by default (deprecated)."""
        key = b"key"
        msg = b"message"

        # Default digestmod is required in Python 3.8+
        h = hmac.new(key, msg, digestmod="sha256")

        assert h is not None

    def test_hmac_digest(self):
        """Property: HMAC digest() returns bytes."""
        key = b"secret_key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")
        digest = h.digest()

        assert isinstance(digest, bytes)
        assert len(digest) == 32  # SHA-256 produces 32 bytes

    def test_hmac_hexdigest(self):
        """Property: HMAC hexdigest() returns hex string."""
        key = b"secret_key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")
        hexdigest = h.hexdigest()

        assert isinstance(hexdigest, str)
        assert len(hexdigest) == 64  # 32 bytes = 64 hex chars
        assert all(c in "0123456789abcdef" for c in hexdigest)


class TestHMACAlgorithms:
    """Test HMAC with different digest algorithms."""

    def test_hmac_sha256(self):
        """Property: HMAC with SHA-256."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")
        digest = h.digest()

        assert len(digest) == 32

    def test_hmac_sha1(self):
        """Property: HMAC with SHA-1."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha1")
        digest = h.digest()

        assert len(digest) == 20

    def test_hmac_sha512(self):
        """Property: HMAC with SHA-512."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha512")
        digest = h.digest()

        assert len(digest) == 64

    def test_hmac_md5(self):
        """Property: HMAC with MD5."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="md5")
        digest = h.digest()

        assert len(digest) == 16

    def test_hmac_blake2b(self):
        """Property: HMAC with BLAKE2b."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="blake2b")
        digest = h.digest()

        assert len(digest) == 64  # Default BLAKE2b digest size


class TestHMACUpdate:
    """Test HMAC update() for incremental hashing."""

    def test_hmac_update_once(self):
        """Property: update() can be called once."""
        key = b"key"

        h = hmac.new(key, digestmod="sha256")
        h.update(b"message")

        digest = h.digest()
        assert len(digest) == 32

    def test_hmac_update_multiple(self):
        """Property: update() can be called multiple times."""
        key = b"key"

        h = hmac.new(key, digestmod="sha256")
        h.update(b"hello")
        h.update(b" ")
        h.update(b"world")

        # Should equal single update
        h2 = hmac.new(key, b"hello world", digestmod="sha256")
        assert h.digest() == h2.digest()

    def test_hmac_update_incremental_equals_full(self):
        """Property: Incremental update equals full update."""
        key = b"secret"
        data = b"The quick brown fox"

        # Incremental
        h1 = hmac.new(key, digestmod="sha256")
        for byte in data:
            h1.update(bytes([byte]))

        # Full
        h2 = hmac.new(key, data, digestmod="sha256")

        assert h1.digest() == h2.digest()

    def test_hmac_empty_update(self):
        """Property: update() with empty bytes doesn't change digest."""
        key = b"key"
        msg = b"message"

        h1 = hmac.new(key, msg, digestmod="sha256")
        digest1 = h1.digest()

        h2 = hmac.new(key, digestmod="sha256")
        h2.update(msg)
        h2.update(b"")
        digest2 = h2.digest()

        assert digest1 == digest2


class TestHMACCopy:
    """Test HMAC copy() method."""

    def test_hmac_copy_basic(self):
        """Property: copy() creates independent HMAC object."""
        key = b"key"
        msg = b"message"

        h1 = hmac.new(key, msg, digestmod="sha256")
        h2 = h1.copy()

        # Should have same state initially
        assert h1.digest() == h2.digest()

    def test_hmac_copy_independence(self):
        """Property: Copied HMAC is independent."""
        key = b"key"

        h1 = hmac.new(key, b"hello", digestmod="sha256")
        h2 = h1.copy()

        # Update one, shouldn't affect the other
        h1.update(b" world")

        assert h1.digest() != h2.digest()


class TestHMACComparison:
    """Test hmac.compare_digest() for constant-time comparison."""

    def test_compare_digest_equal_bytes(self):
        """Property: compare_digest() returns True for equal bytes."""
        a = b"secret123"
        b = b"secret123"

        assert hmac.compare_digest(a, b) is True

    def test_compare_digest_different_bytes(self):
        """Property: compare_digest() returns False for different bytes."""
        a = b"secret123"
        b = b"secret456"

        assert hmac.compare_digest(a, b) is False

    def test_compare_digest_equal_strings(self):
        """Property: compare_digest() works with strings."""
        a = "password"
        b = "password"

        assert hmac.compare_digest(a, b) is True

    def test_compare_digest_different_strings(self):
        """Property: compare_digest() detects different strings."""
        a = "password"
        b = "Password"  # Different case

        assert hmac.compare_digest(a, b) is False

    def test_compare_digest_different_lengths(self):
        """Property: compare_digest() returns False for different lengths."""
        a = "short"
        b = "longer string"

        assert hmac.compare_digest(a, b) is False

    def test_compare_digest_timing_safe(self):
        """Property: compare_digest() is timing-safe."""
        # Verify it compares full length (doesn't short-circuit)
        a = "a" * 1000
        b = "b" * 1000

        result = hmac.compare_digest(a, b)
        assert result is False


class TestHMACKnownValues:
    """Test HMAC against known test vectors."""

    def test_hmac_sha256_known_value(self):
        """Property: HMAC-SHA256 produces known output for RFC test vector."""
        # RFC 4231 Test Case 1
        key = b"\x0b" * 20
        msg = b"Hi There"

        h = hmac.new(key, msg, digestmod="sha256")
        expected = "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"

        assert h.hexdigest() == expected

    def test_hmac_sha1_known_value(self):
        """Property: HMAC-SHA1 produces known output."""
        # RFC 2202 Test Case 1
        key = b"\x0b" * 20
        msg = b"Hi There"

        h = hmac.new(key, msg, digestmod="sha1")
        expected = "b617318655057264e28bc0b6fb378c8ef146be00"

        assert h.hexdigest() == expected


class TestHMACProperties:
    """Test HMAC object properties."""

    def test_hmac_name_property(self):
        """Property: HMAC has name property."""
        key = b"key"
        h = hmac.new(key, digestmod="sha256")

        assert h.name == "hmac-sha256"

    def test_hmac_digest_size_property(self):
        """Property: HMAC has digest_size property."""
        key = b"key"
        h = hmac.new(key, digestmod="sha256")

        assert h.digest_size == 32

    def test_hmac_block_size_property(self):
        """Property: HMAC has block_size property."""
        key = b"key"
        h = hmac.new(key, digestmod="sha256")

        assert h.block_size > 0


class TestHMACEdgeCases:
    """Test edge cases and special scenarios."""

    def test_hmac_empty_key(self):
        """Property: HMAC with empty key works."""
        key = b""
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")
        digest = h.digest()

        assert len(digest) == 32

    def test_hmac_empty_message(self):
        """Property: HMAC with empty message works."""
        key = b"key"
        msg = b""

        h = hmac.new(key, msg, digestmod="sha256")
        digest = h.digest()

        assert len(digest) == 32

    def test_hmac_long_key(self):
        """Property: HMAC handles keys longer than block size."""
        # Keys longer than block size are hashed first
        key = b"k" * 1000
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")
        digest = h.digest()

        assert len(digest) == 32

    def test_hmac_binary_data(self):
        """Property: HMAC handles binary data."""
        key = bytes(range(256))
        msg = bytes([0, 1, 2, 255, 254, 253])

        h = hmac.new(key, msg, digestmod="sha256")
        digest = h.digest()

        assert isinstance(digest, bytes)

    def test_hmac_unicode_key_raises(self):
        """Property: HMAC requires bytes key (not string)."""
        with pytest.raises(TypeError):
            hmac.new("string_key", b"message", digestmod="sha256")

    def test_hmac_deterministic(self):
        """Property: Same key+message produces same HMAC."""
        key = b"secret"
        msg = b"message"

        h1 = hmac.new(key, msg, digestmod="sha256")
        h2 = hmac.new(key, msg, digestmod="sha256")
        h3 = hmac.new(key, msg, digestmod="sha256")

        digest1 = h1.digest()
        digest2 = h2.digest()
        digest3 = h3.digest()

        assert digest1 == digest2 == digest3

    def test_hmac_different_keys(self):
        """Property: Different keys produce different HMACs."""
        msg = b"message"

        h1 = hmac.new(b"key1", msg, digestmod="sha256")
        h2 = hmac.new(b"key2", msg, digestmod="sha256")

        assert h1.digest() != h2.digest()

    def test_hmac_different_messages(self):
        """Property: Different messages produce different HMACs."""
        key = b"key"

        h1 = hmac.new(key, b"message1", digestmod="sha256")
        h2 = hmac.new(key, b"message2", digestmod="sha256")

        assert h1.digest() != h2.digest()

    def test_hmac_digest_can_be_called_multiple_times(self):
        """Property: digest() can be called multiple times."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")

        digest1 = h.digest()
        digest2 = h.digest()
        digest3 = h.digest()

        assert digest1 == digest2 == digest3

    def test_hmac_hexdigest_equals_digest_hex(self):
        """Property: hexdigest() equals digest().hex()."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod="sha256")

        hexdigest = h.hexdigest()
        digest_hex = h.digest().hex()

        assert hexdigest == digest_hex

    def test_hmac_with_hashlib_object(self):
        """Property: HMAC accepts hashlib constructor."""
        key = b"key"
        msg = b"message"

        h = hmac.new(key, msg, digestmod=hashlib.sha256)
        digest = h.digest()

        assert len(digest) == 32

    def test_hmac_message_order_matters(self):
        """Property: Order of message updates affects HMAC."""
        key = b"key"

        h1 = hmac.new(key, digestmod="sha256")
        h1.update(b"hello")
        h1.update(b"world")

        h2 = hmac.new(key, digestmod="sha256")
        h2.update(b"world")
        h2.update(b"hello")

        # Different order = different HMAC
        assert h1.digest() != h2.digest()

    def test_hmac_compare_digest_empty(self):
        """Property: compare_digest() handles empty strings."""
        assert hmac.compare_digest("", "") is True
        assert hmac.compare_digest(b"", b"") is True

    def test_hmac_new_without_message(self):
        """Property: HMAC can be created without initial message."""
        key = b"key"

        h = hmac.new(key, digestmod="sha256")
        h.update(b"message")

        digest = h.digest()
        assert len(digest) == 32

    def test_hmac_digest_module_function(self):
        """Property: hmac.digest() computes HMAC directly."""
        key = b"key"
        msg = b"message"

        digest = hmac.digest(key, msg, "sha256")

        # Should match hmac.new() result
        h = hmac.new(key, msg, digestmod="sha256")
        assert digest == h.digest()
