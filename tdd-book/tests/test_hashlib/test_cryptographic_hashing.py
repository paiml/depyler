"""Test hashlib module - Cryptographic hash functions.

This module tests hashlib for generating cryptographic hash values
for data integrity, digital signatures, and password storage.
"""

import hashlib
import pytest


class TestBasicHashing:
    """Basic cryptographic hash functions."""

    def test_md5_basic(self):
        """Basic: MD5 hash."""
        h = hashlib.md5(b"hello")
        assert len(h.digest()) == 16  # MD5 is 128 bits
        assert isinstance(h.hexdigest(), str)
        assert len(h.hexdigest()) == 32  # 16 bytes = 32 hex chars

    def test_sha1_basic(self):
        """Basic: SHA-1 hash."""
        h = hashlib.sha1(b"hello")
        assert len(h.digest()) == 20  # SHA-1 is 160 bits
        assert len(h.hexdigest()) == 40  # 20 bytes = 40 hex chars

    def test_sha256_basic(self):
        """Basic: SHA-256 hash."""
        h = hashlib.sha256(b"hello")
        assert len(h.digest()) == 32  # SHA-256 is 256 bits
        assert len(h.hexdigest()) == 64  # 32 bytes = 64 hex chars

    def test_sha512_basic(self):
        """Basic: SHA-512 hash."""
        h = hashlib.sha512(b"hello")
        assert len(h.digest()) == 64  # SHA-512 is 512 bits
        assert len(h.hexdigest()) == 128  # 64 bytes = 128 hex chars

    def test_digest_vs_hexdigest(self):
        """Feature: digest() returns bytes, hexdigest() returns hex string."""
        h = hashlib.sha256(b"test")
        digest = h.digest()
        hexdigest = h.hexdigest()
        assert isinstance(digest, bytes)
        assert isinstance(hexdigest, str)
        assert hexdigest == digest.hex()


class TestHashUpdate:
    """Incremental hash updates."""

    def test_update_single(self):
        """Basic: Update hash with data."""
        h = hashlib.sha256()
        h.update(b"hello")
        result1 = h.hexdigest()

        h2 = hashlib.sha256(b"hello")
        result2 = h2.hexdigest()

        assert result1 == result2

    def test_update_multiple(self):
        """Feature: Multiple updates concatenate."""
        h1 = hashlib.sha256()
        h1.update(b"hello")
        h1.update(b"world")

        h2 = hashlib.sha256(b"helloworld")

        assert h1.hexdigest() == h2.hexdigest()

    def test_update_incremental(self):
        """Property: Incremental hashing equals one-shot."""
        data = b"The quick brown fox jumps over the lazy dog"

        h1 = hashlib.sha256(data)

        h2 = hashlib.sha256()
        for byte in data:
            h2.update(bytes([byte]))

        assert h1.hexdigest() == h2.hexdigest()

    def test_update_empty(self):
        """Edge: Update with empty bytes."""
        h = hashlib.sha256(b"test")
        digest1 = h.hexdigest()
        h.update(b"")
        digest2 = h.hexdigest()
        assert digest1 == digest2


class TestHashAlgorithms:
    """Different hash algorithms."""

    def test_sha224(self):
        """Feature: SHA-224 hash."""
        h = hashlib.sha224(b"test")
        assert len(h.digest()) == 28  # 224 bits

    def test_sha384(self):
        """Feature: SHA-384 hash."""
        h = hashlib.sha384(b"test")
        assert len(h.digest()) == 48  # 384 bits

    def test_sha3_256(self):
        """Feature: SHA3-256 hash."""
        h = hashlib.sha3_256(b"test")
        assert len(h.digest()) == 32  # 256 bits

    def test_sha3_512(self):
        """Feature: SHA3-512 hash."""
        h = hashlib.sha3_512(b"test")
        assert len(h.digest()) == 64  # 512 bits

    def test_blake2b(self):
        """Feature: BLAKE2b hash."""
        h = hashlib.blake2b(b"test")
        assert len(h.digest()) == 64  # Default 512 bits

    def test_blake2s(self):
        """Feature: BLAKE2s hash."""
        h = hashlib.blake2s(b"test")
        assert len(h.digest()) == 32  # Default 256 bits

    def test_shake_128(self):
        """Feature: SHAKE128 variable-length hash."""
        h = hashlib.shake_128(b"test")
        digest1 = h.hexdigest(16)
        assert len(digest1) == 32  # 16 bytes = 32 hex chars

        h2 = hashlib.shake_128(b"test")
        digest2 = h2.hexdigest(32)
        assert len(digest2) == 64  # 32 bytes = 64 hex chars

    def test_shake_256(self):
        """Feature: SHAKE256 variable-length hash."""
        h = hashlib.shake_256(b"test")
        digest = h.hexdigest(64)
        assert len(digest) == 128  # 64 bytes = 128 hex chars


class TestHashCopy:
    """Hash object copying."""

    def test_copy_preserves_state(self):
        """Basic: Copy preserves hash state."""
        h1 = hashlib.sha256(b"hello")
        h2 = h1.copy()

        assert h1.hexdigest() == h2.hexdigest()

    def test_copy_independent(self):
        """Property: Copies are independent."""
        h1 = hashlib.sha256(b"hello")
        h2 = h1.copy()

        h1.update(b"world")
        h2.update(b"python")

        assert h1.hexdigest() != h2.hexdigest()

    def test_copy_before_finalize(self):
        """Use case: Branch hash computation."""
        h = hashlib.sha256(b"prefix")

        h1 = h.copy()
        h1.update(b"suffix1")

        h2 = h.copy()
        h2.update(b"suffix2")

        assert h1.hexdigest() != h2.hexdigest()


class TestHashProperties:
    """Hash function properties."""

    def test_deterministic(self):
        """Property: Same input produces same hash."""
        data = b"test data"
        h1 = hashlib.sha256(data).hexdigest()
        h2 = hashlib.sha256(data).hexdigest()
        assert h1 == h2

    def test_empty_string_hash(self):
        """Edge: Hash of empty bytes."""
        h = hashlib.sha256(b"")
        # Empty SHA-256 is well-defined
        assert h.hexdigest() == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

    def test_different_input_different_hash(self):
        """Property: Different inputs produce different hashes."""
        h1 = hashlib.sha256(b"test1").hexdigest()
        h2 = hashlib.sha256(b"test2").hexdigest()
        assert h1 != h2

    def test_small_change_different_hash(self):
        """Property: Small input change produces completely different hash."""
        h1 = hashlib.sha256(b"test").hexdigest()
        h2 = hashlib.sha256(b"Test").hexdigest()
        # Should be completely different (avalanche effect)
        assert h1 != h2

    def test_hash_length_consistent(self):
        """Property: Hash length is consistent for algorithm."""
        h1 = hashlib.sha256(b"short").hexdigest()
        h2 = hashlib.sha256(b"a" * 10000).hexdigest()
        assert len(h1) == len(h2) == 64


class TestHashAttributes:
    """Hash object attributes."""

    def test_name_attribute(self):
        """Feature: Hash object has name attribute."""
        h = hashlib.sha256()
        assert h.name == "sha256"

    def test_digest_size(self):
        """Feature: Hash object has digest_size attribute."""
        assert hashlib.md5().digest_size == 16
        assert hashlib.sha1().digest_size == 20
        assert hashlib.sha256().digest_size == 32
        assert hashlib.sha512().digest_size == 64

    def test_block_size(self):
        """Feature: Hash object has block_size attribute."""
        h = hashlib.sha256()
        assert hasattr(h, 'block_size')
        assert h.block_size > 0


class TestBlake2:
    """BLAKE2 hash functions with parameters."""

    def test_blake2b_digest_size(self):
        """Feature: BLAKE2b custom digest size."""
        h = hashlib.blake2b(b"test", digest_size=32)
        assert len(h.digest()) == 32

    def test_blake2b_key(self):
        """Feature: BLAKE2b keyed hashing."""
        key = b"secret key"
        h1 = hashlib.blake2b(b"message", key=key)
        h2 = hashlib.blake2b(b"message", key=key)
        assert h1.hexdigest() == h2.hexdigest()

    def test_blake2b_different_keys(self):
        """Property: Different keys produce different hashes."""
        h1 = hashlib.blake2b(b"message", key=b"key1")
        h2 = hashlib.blake2b(b"message", key=b"key2")
        assert h1.hexdigest() != h2.hexdigest()

    def test_blake2s_digest_size(self):
        """Feature: BLAKE2s custom digest size."""
        h = hashlib.blake2s(b"test", digest_size=16)
        assert len(h.digest()) == 16

    def test_blake2b_salt(self):
        """Feature: BLAKE2b with salt."""
        salt = b"random salt"
        h = hashlib.blake2b(b"message", salt=salt)
        assert len(h.digest()) == 64

    def test_blake2b_person(self):
        """Feature: BLAKE2b with personalization."""
        person = b"my app"
        h = hashlib.blake2b(b"message", person=person)
        assert len(h.digest()) == 64


class TestAlgorithmsAvailable:
    """Available algorithms."""

    def test_algorithms_guaranteed(self):
        """Feature: Guaranteed algorithms are available."""
        guaranteed = {'md5', 'sha1', 'sha224', 'sha256', 'sha384', 'sha512'}
        for algo in guaranteed:
            assert algo in hashlib.algorithms_guaranteed

    def test_algorithms_available(self):
        """Feature: Check available algorithms."""
        # algorithms_available includes all platform-supported algorithms
        assert 'sha256' in hashlib.algorithms_available

    def test_new_with_algorithm_name(self):
        """Feature: Create hash using new() with algorithm name."""
        h1 = hashlib.new('sha256', b"test")
        h2 = hashlib.sha256(b"test")
        assert h1.hexdigest() == h2.hexdigest()


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_large_data_hash(self):
        """Performance: Hash large data."""
        data = b"x" * 1000000  # 1MB
        h = hashlib.sha256(data)
        assert len(h.hexdigest()) == 64

    def test_hash_after_digest(self):
        """Property: Can call digest() multiple times."""
        h = hashlib.sha256(b"test")
        digest1 = h.hexdigest()
        digest2 = h.hexdigest()
        assert digest1 == digest2

    def test_update_after_digest(self):
        """Property: Can update after calling digest()."""
        h = hashlib.sha256(b"test")
        digest1 = h.hexdigest()
        h.update(b"more")
        digest2 = h.hexdigest()
        assert digest1 != digest2

    def test_hex_lowercase(self):
        """Property: hexdigest() returns lowercase."""
        h = hashlib.sha256(b"TEST")
        hexdigest = h.hexdigest()
        assert hexdigest == hexdigest.lower()

    def test_binary_data(self):
        """Feature: Hash binary data."""
        data = bytes(range(256))
        h = hashlib.sha256(data)
        assert len(h.digest()) == 32

    def test_unicode_requires_encoding(self):
        """Error: Cannot hash string directly."""
        with pytest.raises(TypeError):
            hashlib.sha256("string")  # Must be bytes

    def test_shake_requires_length(self):
        """Feature: SHAKE requires length parameter."""
        h = hashlib.shake_128(b"test")
        # digest() and hexdigest() require length argument
        digest = h.digest(16)
        assert len(digest) == 16

    def test_blake2_max_digest_size(self):
        """Edge: BLAKE2 digest size limits."""
        # BLAKE2b max is 64 bytes
        h = hashlib.blake2b(b"test", digest_size=64)
        assert len(h.digest()) == 64

        # BLAKE2s max is 32 bytes
        h2 = hashlib.blake2s(b"test", digest_size=32)
        assert len(h2.digest()) == 32

    def test_error_blake2_digest_size_too_large(self):
        """Error: BLAKE2 digest size too large."""
        with pytest.raises(ValueError):
            hashlib.blake2b(digest_size=65)

    def test_error_blake2s_digest_size_too_large(self):
        """Error: BLAKE2s digest size too large."""
        with pytest.raises(ValueError):
            hashlib.blake2s(digest_size=33)

    def test_hash_consistency_across_calls(self):
        """Property: Hash is consistent across multiple objects."""
        data = b"consistency test"
        hashes = [hashlib.sha256(data).hexdigest() for _ in range(10)]
        assert len(set(hashes)) == 1  # All identical

    def test_md5_collision_resistance(self):
        """Property: Different messages produce different MD5 hashes."""
        # Note: MD5 is cryptographically broken, but still deterministic
        h1 = hashlib.md5(b"message1").hexdigest()
        h2 = hashlib.md5(b"message2").hexdigest()
        assert h1 != h2

    def test_sha256_common_hash(self):
        """Use case: Common SHA-256 hash."""
        # SHA-256 of "hello world"
        h = hashlib.sha256(b"hello world")
        expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        assert h.hexdigest() == expected


class TestPasswordHashing:
    """Use cases for password hashing."""

    def test_pbkdf2_hmac_basic(self):
        """Use case: PBKDF2 key derivation."""
        password = b"mypassword"
        salt = b"salt1234"
        key = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
        assert len(key) == 32  # SHA-256 output

    def test_pbkdf2_hmac_different_passwords(self):
        """Property: Different passwords produce different keys."""
        salt = b"salt"
        key1 = hashlib.pbkdf2_hmac('sha256', b"pass1", salt, 100000)
        key2 = hashlib.pbkdf2_hmac('sha256', b"pass2", salt, 100000)
        assert key1 != key2

    def test_pbkdf2_hmac_different_salts(self):
        """Property: Different salts produce different keys."""
        password = b"password"
        key1 = hashlib.pbkdf2_hmac('sha256', password, b"salt1", 100000)
        key2 = hashlib.pbkdf2_hmac('sha256', password, b"salt2", 100000)
        assert key1 != key2

    def test_pbkdf2_hmac_iterations(self):
        """Property: Different iterations produce different keys."""
        password = b"password"
        salt = b"salt"
        key1 = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
        key2 = hashlib.pbkdf2_hmac('sha256', password, salt, 200000)
        assert key1 != key2

    def test_pbkdf2_hmac_reproducible(self):
        """Property: Same parameters produce same key."""
        password = b"password"
        salt = b"salt"
        key1 = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
        key2 = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
        assert key1 == key2


class TestFileHashing:
    """Use cases for file hashing."""

    def test_hash_file_simulation(self):
        """Use case: Hash file-like data in chunks."""
        # Simulate reading file in chunks
        data = b"a" * 10000
        chunk_size = 1024

        h = hashlib.sha256()
        for i in range(0, len(data), chunk_size):
            chunk = data[i:i+chunk_size]
            h.update(chunk)

        # Compare with one-shot hash
        h2 = hashlib.sha256(data)
        assert h.hexdigest() == h2.hexdigest()


class TestScrypt:
    """Scrypt key derivation function."""

    def test_scrypt_basic(self):
        """Feature: scrypt key derivation."""
        password = b"password"
        salt = b"salt"
        key = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
        assert len(key) == 64  # Default dklen

    def test_scrypt_custom_length(self):
        """Feature: scrypt with custom key length."""
        password = b"password"
        salt = b"salt"
        key = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1, dklen=32)
        assert len(key) == 32

    def test_scrypt_reproducible(self):
        """Property: scrypt is reproducible."""
        password = b"password"
        salt = b"salt"
        key1 = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
        key2 = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
        assert key1 == key2

    def test_scrypt_different_n(self):
        """Property: Different n parameter produces different keys."""
        password = b"password"
        salt = b"salt"
        key1 = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
        key2 = hashlib.scrypt(password, salt=salt, n=32, r=8, p=1)
        assert key1 != key2
