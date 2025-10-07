"""
TDD Book - Phase 4: Network & IPC
Module: hashlib - Secure hashes and message digests
Coverage: SHA256, SHA1, MD5, blake2, shake, common hash operations

Test Categories:
- Hash algorithms (MD5, SHA1, SHA256, SHA512)
- Hash operations (update, digest, hexdigest)
- Hash construction (hashlib.new, hashlib.sha256)
- BLAKE2 variants
- SHAKE (extendable-output functions)
- Hash properties
- Edge cases
"""

import hashlib
import pytest


class TestMD5:
    """Test MD5 hashing."""

    def test_md5_basic(self):
        """Property: md5() creates MD5 hash object."""
        h = hashlib.md5()
        assert h is not None
        assert h.name == "md5"

    def test_md5_digest_size(self):
        """Property: MD5 produces 128-bit (16-byte) digest."""
        h = hashlib.md5()
        h.update(b"test")

        digest = h.digest()
        assert len(digest) == 16

    def test_md5_hexdigest(self):
        """Property: MD5 hexdigest is 32 hex characters."""
        h = hashlib.md5(b"hello")
        hexdigest = h.hexdigest()

        assert len(hexdigest) == 32
        assert all(c in "0123456789abcdef" for c in hexdigest)

    def test_md5_known_value(self):
        """Property: MD5 produces known hash for given input."""
        h = hashlib.md5(b"hello")
        hexdigest = h.hexdigest()

        # Known MD5 of "hello"
        assert hexdigest == "5d41402abc4b2a76b9719d911017c592"

    def test_md5_empty_string(self):
        """Property: MD5 of empty string is known value."""
        h = hashlib.md5(b"")
        hexdigest = h.hexdigest()

        # MD5 of empty string
        assert hexdigest == "d41d8cd98f00b204e9800998ecf8427e"


class TestSHA1:
    """Test SHA-1 hashing."""

    def test_sha1_basic(self):
        """Property: sha1() creates SHA-1 hash object."""
        h = hashlib.sha1()
        assert h is not None
        assert h.name == "sha1"

    def test_sha1_digest_size(self):
        """Property: SHA-1 produces 160-bit (20-byte) digest."""
        h = hashlib.sha1()
        h.update(b"test")

        digest = h.digest()
        assert len(digest) == 20

    def test_sha1_hexdigest(self):
        """Property: SHA-1 hexdigest is 40 hex characters."""
        h = hashlib.sha1(b"hello")
        hexdigest = h.hexdigest()

        assert len(hexdigest) == 40

    def test_sha1_known_value(self):
        """Property: SHA-1 produces known hash for given input."""
        h = hashlib.sha1(b"hello")
        hexdigest = h.hexdigest()

        # Known SHA-1 of "hello"
        assert hexdigest == "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"


class TestSHA256:
    """Test SHA-256 hashing."""

    def test_sha256_basic(self):
        """Property: sha256() creates SHA-256 hash object."""
        h = hashlib.sha256()
        assert h is not None
        assert h.name == "sha256"

    def test_sha256_digest_size(self):
        """Property: SHA-256 produces 256-bit (32-byte) digest."""
        h = hashlib.sha256()
        h.update(b"test")

        digest = h.digest()
        assert len(digest) == 32

    def test_sha256_hexdigest(self):
        """Property: SHA-256 hexdigest is 64 hex characters."""
        h = hashlib.sha256(b"hello")
        hexdigest = h.hexdigest()

        assert len(hexdigest) == 64

    def test_sha256_known_value(self):
        """Property: SHA-256 produces known hash for given input."""
        h = hashlib.sha256(b"hello")
        hexdigest = h.hexdigest()

        # Known SHA-256 of "hello"
        assert (
            hexdigest
            == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        )

    def test_sha256_empty(self):
        """Property: SHA-256 of empty string is known value."""
        h = hashlib.sha256(b"")
        hexdigest = h.hexdigest()

        assert (
            hexdigest
            == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        )


class TestSHA512:
    """Test SHA-512 hashing."""

    def test_sha512_basic(self):
        """Property: sha512() creates SHA-512 hash object."""
        h = hashlib.sha512()
        assert h is not None
        assert h.name == "sha512"

    def test_sha512_digest_size(self):
        """Property: SHA-512 produces 512-bit (64-byte) digest."""
        h = hashlib.sha512()
        h.update(b"test")

        digest = h.digest()
        assert len(digest) == 64

    def test_sha512_hexdigest(self):
        """Property: SHA-512 hexdigest is 128 hex characters."""
        h = hashlib.sha512(b"hello")
        hexdigest = h.hexdigest()

        assert len(hexdigest) == 128


class TestHashUpdate:
    """Test hash update() method for incremental hashing."""

    def test_update_once(self):
        """Property: update() can be called once."""
        h = hashlib.sha256()
        h.update(b"hello world")

        hexdigest = h.hexdigest()
        assert len(hexdigest) == 64

    def test_update_multiple(self):
        """Property: update() can be called multiple times."""
        h = hashlib.sha256()
        h.update(b"hello")
        h.update(b" ")
        h.update(b"world")

        hexdigest = h.hexdigest()
        # Should equal single update
        h2 = hashlib.sha256(b"hello world")
        assert hexdigest == h2.hexdigest()

    def test_update_incremental_equals_full(self):
        """Property: Incremental update equals full update."""
        data = b"The quick brown fox jumps over the lazy dog"

        # Incremental
        h1 = hashlib.sha256()
        for byte in data:
            h1.update(bytes([byte]))

        # Full
        h2 = hashlib.sha256(data)

        assert h1.hexdigest() == h2.hexdigest()

    def test_update_empty_bytes(self):
        """Property: update() with empty bytes doesn't change hash."""
        h1 = hashlib.sha256(b"test")
        digest1 = h1.hexdigest()

        h2 = hashlib.sha256()
        h2.update(b"test")
        h2.update(b"")
        digest2 = h2.hexdigest()

        assert digest1 == digest2


class TestHashCopy:
    """Test hash copy() method."""

    def test_copy_basic(self):
        """Property: copy() creates independent hash object."""
        h1 = hashlib.sha256(b"hello")
        h2 = h1.copy()

        # Should have same state initially
        assert h1.hexdigest() == h2.hexdigest()

    def test_copy_independence(self):
        """Property: Copied hash is independent."""
        h1 = hashlib.sha256(b"hello")
        h2 = h1.copy()

        # Update one, shouldn't affect the other
        h1.update(b" world")

        assert h1.hexdigest() != h2.hexdigest()


class TestHashNew:
    """Test hashlib.new() constructor."""

    def test_new_md5(self):
        """Property: new() can create MD5 hash."""
        h = hashlib.new("md5", b"test")
        assert h.name == "md5"

    def test_new_sha256(self):
        """Property: new() can create SHA-256 hash."""
        h = hashlib.new("sha256", b"test")
        assert h.name == "sha256"

    def test_new_invalid_algorithm(self):
        """Property: new() raises on invalid algorithm."""
        with pytest.raises(ValueError):
            hashlib.new("invalid_algo")

    def test_new_equals_direct_constructor(self):
        """Property: new() produces same result as direct constructor."""
        data = b"test data"

        h1 = hashlib.new("sha256", data)
        h2 = hashlib.sha256(data)

        assert h1.hexdigest() == h2.hexdigest()


class TestHashAlgorithmsAvailable:
    """Test available hash algorithms."""

    def test_algorithms_guaranteed(self):
        """Property: Certain algorithms are always available."""
        guaranteed = {"md5", "sha1", "sha224", "sha256", "sha384", "sha512"}

        for algo in guaranteed:
            assert algo in hashlib.algorithms_guaranteed

    def test_algorithms_available(self):
        """Property: algorithms_available lists available algorithms."""
        available = hashlib.algorithms_available

        assert isinstance(available, set)
        assert "sha256" in available

    def test_create_all_guaranteed(self):
        """Property: All guaranteed algorithms can be created."""
        for algo in hashlib.algorithms_guaranteed:
            h = hashlib.new(algo)
            assert h is not None


class TestBLAKE2:
    """Test BLAKE2 hash functions."""

    def test_blake2b_basic(self):
        """Property: blake2b() creates BLAKE2b hash."""
        h = hashlib.blake2b()
        assert h is not None
        assert "blake2b" in h.name

    def test_blake2s_basic(self):
        """Property: blake2s() creates BLAKE2s hash."""
        h = hashlib.blake2s()
        assert h is not None
        assert "blake2s" in h.name

    def test_blake2b_digest_size(self):
        """Property: blake2b default digest is 64 bytes."""
        h = hashlib.blake2b(b"test")
        digest = h.digest()

        assert len(digest) == 64

    def test_blake2s_digest_size(self):
        """Property: blake2s default digest is 32 bytes."""
        h = hashlib.blake2s(b"test")
        digest = h.digest()

        assert len(digest) == 32

    def test_blake2b_custom_digest_size(self):
        """Property: blake2b supports custom digest size."""
        h = hashlib.blake2b(b"test", digest_size=16)
        digest = h.digest()

        assert len(digest) == 16


class TestSHAKE:
    """Test SHAKE extendable-output functions."""

    def test_shake128_basic(self):
        """Property: shake_128() creates SHAKE128 hash."""
        h = hashlib.shake_128()
        assert h is not None
        assert "shake_128" in h.name

    def test_shake256_basic(self):
        """Property: shake_256() creates SHAKE256 hash."""
        h = hashlib.shake_256()
        assert h is not None
        assert "shake_256" in h.name

    def test_shake_hexdigest_length(self):
        """Property: SHAKE hexdigest accepts length parameter."""
        h = hashlib.shake_128(b"test")

        # Request 16 bytes (32 hex chars)
        hexdigest = h.hexdigest(16)
        assert len(hexdigest) == 32

    def test_shake_digest_length(self):
        """Property: SHAKE digest accepts length parameter."""
        h = hashlib.shake_256(b"test")

        digest = h.digest(32)
        assert len(digest) == 32


class TestHashProperties:
    """Test hash object properties."""

    def test_digest_size_property(self):
        """Property: Hash has digest_size property."""
        h = hashlib.sha256()
        assert h.digest_size == 32

        h = hashlib.md5()
        assert h.digest_size == 16

    def test_block_size_property(self):
        """Property: Hash has block_size property."""
        h = hashlib.sha256()
        assert h.block_size > 0

    def test_name_property(self):
        """Property: Hash has name property."""
        h = hashlib.sha256()
        assert h.name == "sha256"

        h = hashlib.md5()
        assert h.name == "md5"


class TestHashEdgeCases:
    """Test edge cases and special scenarios."""

    def test_hash_very_long_data(self):
        """Property: Hashing very long data works."""
        data = b"A" * 1000000  # 1MB
        h = hashlib.sha256(data)
        hexdigest = h.hexdigest()

        assert len(hexdigest) == 64

    def test_hash_null_bytes(self):
        """Property: Hashing data with null bytes works."""
        data = b"Hello\x00World\x00"
        h = hashlib.sha256(data)

        # Should produce consistent hash
        h2 = hashlib.sha256(data)
        assert h.hexdigest() == h2.hexdigest()

    def test_hash_unicode_encoded(self):
        """Property: Hashing UTF-8 encoded Unicode works."""
        text = "Hello 世界"
        data = text.encode("utf-8")

        h = hashlib.sha256(data)
        hexdigest = h.hexdigest()

        assert len(hexdigest) == 64

    def test_multiple_hashes_independent(self):
        """Property: Multiple hash objects are independent."""
        h1 = hashlib.sha256(b"data1")
        h2 = hashlib.sha256(b"data2")

        assert h1.hexdigest() != h2.hexdigest()

    def test_digest_can_be_called_multiple_times(self):
        """Property: digest() can be called multiple times."""
        h = hashlib.sha256(b"test")

        digest1 = h.digest()
        digest2 = h.digest()
        digest3 = h.digest()

        assert digest1 == digest2 == digest3

    def test_hexdigest_equals_digest_hex(self):
        """Property: hexdigest() equals hex of digest()."""
        h = hashlib.sha256(b"test")

        hexdigest = h.hexdigest()
        digest_hex = h.digest().hex()

        assert hexdigest == digest_hex

    def test_same_data_same_hash(self):
        """Property: Same data always produces same hash."""
        data = b"consistent data"

        h1 = hashlib.sha256(data)
        h2 = hashlib.sha256(data)
        h3 = hashlib.sha256(data)

        assert h1.hexdigest() == h2.hexdigest() == h3.hexdigest()

    def test_different_data_different_hash(self):
        """Property: Different data produces different hash."""
        h1 = hashlib.sha256(b"data1")
        h2 = hashlib.sha256(b"data2")

        assert h1.hexdigest() != h2.hexdigest()

    def test_single_bit_change_changes_hash(self):
        """Property: Single bit change dramatically changes hash."""
        data1 = b"test"
        data2 = b"tesu"  # Changed last character

        h1 = hashlib.sha256(data1)
        h2 = hashlib.sha256(data2)

        # Hashes should be completely different
        hexdigest1 = h1.hexdigest()
        hexdigest2 = h2.hexdigest()

        assert hexdigest1 != hexdigest2

        # Count different characters (should be many)
        differences = sum(c1 != c2 for c1, c2 in zip(hexdigest1, hexdigest2))
        assert differences > 10  # Avalanche effect

    def test_hash_bytes_vs_bytearray(self):
        """Property: Hashing bytes and bytearray produces same result."""
        data_bytes = b"test data"
        data_bytearray = bytearray(b"test data")

        h1 = hashlib.sha256(data_bytes)
        h2 = hashlib.sha256(data_bytearray)

        assert h1.hexdigest() == h2.hexdigest()

    def test_constructor_with_data(self):
        """Property: Constructor accepts initial data."""
        # With data in constructor
        h1 = hashlib.sha256(b"hello")

        # With update
        h2 = hashlib.sha256()
        h2.update(b"hello")

        assert h1.hexdigest() == h2.hexdigest()

    def test_hash_order_matters(self):
        """Property: Order of data affects hash."""
        h1 = hashlib.sha256()
        h1.update(b"hello")
        h1.update(b"world")

        h2 = hashlib.sha256()
        h2.update(b"world")
        h2.update(b"hello")

        # Different order = different hash
        assert h1.hexdigest() != h2.hexdigest()
