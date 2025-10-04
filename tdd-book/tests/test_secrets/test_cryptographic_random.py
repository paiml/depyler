"""Test secrets module - Cryptographically secure random generation.

This module tests secrets for generating cryptographically strong random
numbers suitable for security purposes.
"""

import secrets
import string
import pytest


class TestTokenGeneration:
    """Cryptographically secure token generation."""

    def test_token_bytes(self):
        """Basic: Generate random bytes."""
        token = secrets.token_bytes(16)
        assert len(token) == 16
        assert isinstance(token, bytes)

    def test_token_bytes_default(self):
        """Feature: Default token_bytes size."""
        token = secrets.token_bytes()
        assert isinstance(token, bytes)
        assert len(token) == 32  # Default size

    def test_token_bytes_uniqueness(self):
        """Property: Each call produces unique token."""
        token1 = secrets.token_bytes(16)
        token2 = secrets.token_bytes(16)
        assert token1 != token2

    def test_token_hex(self):
        """Basic: Generate hex token."""
        token = secrets.token_hex(16)
        assert len(token) == 32  # 16 bytes = 32 hex chars
        assert isinstance(token, str)
        # All characters should be hex digits
        assert all(c in '0123456789abcdef' for c in token)

    def test_token_hex_default(self):
        """Feature: Default token_hex size."""
        token = secrets.token_hex()
        assert isinstance(token, str)
        assert len(token) == 64  # 32 bytes = 64 hex chars

    def test_token_urlsafe(self):
        """Basic: Generate URL-safe token."""
        token = secrets.token_urlsafe(16)
        assert isinstance(token, str)
        # URL-safe characters: alphanumeric, -, _
        assert all(c.isalnum() or c in '-_' for c in token)

    def test_token_urlsafe_default(self):
        """Feature: Default token_urlsafe size."""
        token = secrets.token_urlsafe()
        assert isinstance(token, str)
        # Should be URL-safe
        assert all(c.isalnum() or c in '-_' for c in token)

    def test_token_urlsafe_no_padding(self):
        """Property: No padding characters in URL-safe token."""
        token = secrets.token_urlsafe(16)
        assert '=' not in token  # No base64 padding


class TestRandomChoice:
    """Cryptographically secure random choice."""

    def test_choice(self):
        """Basic: Choose from sequence."""
        items = [1, 2, 3, 4, 5]
        choice = secrets.choice(items)
        assert choice in items

    def test_choice_string(self):
        """Feature: Choose from string."""
        s = "abcde"
        choice = secrets.choice(s)
        assert choice in s

    def test_choice_uniqueness(self):
        """Property: Multiple choices can differ."""
        items = list(range(100))
        choices = [secrets.choice(items) for _ in range(10)]
        # Very unlikely all choices are the same
        assert len(set(choices)) > 1

    def test_error_choice_empty(self):
        """Error: Choice from empty sequence."""
        with pytest.raises(IndexError):
            secrets.choice([])


class TestRandbelow:
    """Cryptographically secure random integer below n."""

    def test_randbelow(self):
        """Basic: Random integer below n."""
        r = secrets.randbelow(10)
        assert 0 <= r < 10
        assert isinstance(r, int)

    def test_randbelow_range(self):
        """Property: Always in correct range."""
        for _ in range(100):
            r = secrets.randbelow(100)
            assert 0 <= r < 100

    def test_randbelow_one(self):
        """Edge: randbelow(1) always returns 0."""
        r = secrets.randbelow(1)
        assert r == 0

    def test_randbelow_distribution(self):
        """Property: Should cover full range over time."""
        n = 10
        results = {secrets.randbelow(n) for _ in range(100)}
        # Should see multiple different values
        assert len(results) > 1

    def test_error_randbelow_zero(self):
        """Error: randbelow(0) raises ValueError."""
        with pytest.raises(ValueError):
            secrets.randbelow(0)

    def test_error_randbelow_negative(self):
        """Error: randbelow with negative raises ValueError."""
        with pytest.raises(ValueError):
            secrets.randbelow(-1)


class TestCompareDigest:
    """Constant-time comparison for security."""

    def test_compare_digest_equal_strings(self):
        """Basic: Equal strings compare as True."""
        a = "secret123"
        b = "secret123"
        assert secrets.compare_digest(a, b) is True

    def test_compare_digest_unequal_strings(self):
        """Basic: Unequal strings compare as False."""
        a = "secret123"
        b = "secret456"
        assert secrets.compare_digest(a, b) is False

    def test_compare_digest_bytes(self):
        """Feature: Compare bytes."""
        a = b"secret123"
        b = b"secret123"
        assert secrets.compare_digest(a, b) is True

    def test_compare_digest_bytes_unequal(self):
        """Feature: Unequal bytes compare as False."""
        a = b"secret123"
        b = b"secret456"
        assert secrets.compare_digest(a, b) is False

    def test_compare_digest_different_lengths(self):
        """Edge: Different lengths compare as False."""
        a = "secret"
        b = "secret123"
        assert secrets.compare_digest(a, b) is False

    def test_compare_digest_empty_strings(self):
        """Edge: Empty strings compare as True."""
        assert secrets.compare_digest("", "") is True

    def test_compare_digest_constant_time(self):
        """Property: Comparison is constant-time (timing-safe)."""
        # This is a security property - all comparisons take same time
        # We can't easily test timing, but verify it works correctly
        a = "a" * 100
        b = "a" * 99 + "b"
        assert secrets.compare_digest(a, b) is False


class TestSystemRandom:
    """SystemRandom for cryptographic randomness."""

    def test_systemrandom_available(self):
        """Basic: SystemRandom is available via secrets."""
        # secrets uses SystemRandom internally
        r = secrets.randbelow(10)
        assert isinstance(r, int)

    def test_systemrandom_not_reproducible(self):
        """Property: Cannot be seeded (cryptographically secure)."""
        # Unlike random.random(), secrets cannot be seeded
        r1 = secrets.token_bytes(16)
        r2 = secrets.token_bytes(16)
        assert r1 != r2


class TestPasswordGeneration:
    """Generate secure passwords and tokens."""

    def test_generate_password_basic(self):
        """Use case: Generate secure password."""
        alphabet = string.ascii_letters + string.digits
        password = ''.join(secrets.choice(alphabet) for _ in range(10))
        assert len(password) == 10
        assert all(c in alphabet for c in password)

    def test_generate_password_with_punctuation(self):
        """Use case: Password with special characters."""
        alphabet = string.ascii_letters + string.digits + string.punctuation
        password = ''.join(secrets.choice(alphabet) for _ in range(12))
        assert len(password) == 12

    def test_generate_secure_token(self):
        """Use case: Generate secure reset token."""
        token = secrets.token_urlsafe(32)
        assert len(token) > 0
        # Should be suitable for URLs
        assert all(c.isalnum() or c in '-_' for c in token)

    def test_generate_api_key(self):
        """Use case: Generate API key."""
        api_key = secrets.token_hex(32)
        assert len(api_key) == 64  # 32 bytes = 64 hex chars
        # Should be hex
        assert all(c in '0123456789abcdef' for c in api_key)


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_token_bytes_zero(self):
        """Edge: Zero-length token."""
        token = secrets.token_bytes(0)
        assert token == b''

    def test_token_hex_zero(self):
        """Edge: Zero-length hex token."""
        token = secrets.token_hex(0)
        assert token == ''

    def test_token_urlsafe_zero(self):
        """Edge: Zero-length URL-safe token."""
        token = secrets.token_urlsafe(0)
        assert token == ''

    def test_token_bytes_large(self):
        """Performance: Large token generation."""
        token = secrets.token_bytes(1024)
        assert len(token) == 1024

    def test_choice_single_element(self):
        """Edge: Choice from single element."""
        choice = secrets.choice([42])
        assert choice == 42

    def test_randbelow_large(self):
        """Performance: randbelow with large n."""
        r = secrets.randbelow(1000000)
        assert 0 <= r < 1000000

    def test_token_entropy(self):
        """Property: Tokens have high entropy."""
        # Generate multiple tokens - should all be unique
        tokens = [secrets.token_bytes(16) for _ in range(100)]
        # All should be unique (extremely high probability)
        assert len(set(tokens)) == 100

    def test_hex_lowercase(self):
        """Property: Hex tokens are lowercase."""
        token = secrets.token_hex(16)
        assert token == token.lower()
        assert token.islower() or token.isdigit() or all(c in '0123456789' for c in token)

    def test_urlsafe_base64_variant(self):
        """Property: URL-safe tokens use base64url encoding."""
        token = secrets.token_urlsafe(16)
        # No '+' or '/' (standard base64), only alphanumeric, '-', '_'
        assert '+' not in token
        assert '/' not in token

    def test_compare_digest_type_mismatch(self):
        """Error: Type mismatch in compare_digest."""
        # Comparing str and bytes should raise TypeError
        with pytest.raises(TypeError):
            secrets.compare_digest("string", b"bytes")

    def test_multiple_tokens_unique(self):
        """Property: Multiple tokens are unique."""
        tokens = [secrets.token_hex(16) for _ in range(50)]
        # All should be unique
        assert len(tokens) == len(set(tokens))

    def test_choice_works_with_tuple(self):
        """Feature: Choice works with tuple."""
        items = (1, 2, 3, 4, 5)
        choice = secrets.choice(items)
        assert choice in items

    def test_choice_works_with_range(self):
        """Feature: Choice works with range."""
        r = range(10)
        choice = secrets.choice(r)
        assert choice in r

    def test_randbelow_two(self):
        """Edge: randbelow(2) returns 0 or 1."""
        results = {secrets.randbelow(2) for _ in range(50)}
        # Should see both 0 and 1
        assert results <= {0, 1}
        assert len(results) > 1  # Very likely to see both

    def test_token_bytes_consistency(self):
        """Property: Same nbytes gives same length."""
        for nbytes in [8, 16, 32, 64]:
            token = secrets.token_bytes(nbytes)
            assert len(token) == nbytes

    def test_token_hex_double_length(self):
        """Property: Hex token is 2x byte length."""
        for nbytes in [8, 16, 32]:
            token = secrets.token_hex(nbytes)
            assert len(token) == nbytes * 2

    def test_compare_digest_case_sensitive(self):
        """Property: compare_digest is case-sensitive."""
        assert secrets.compare_digest("Secret", "secret") is False
        assert secrets.compare_digest("SECRET", "secret") is False

    def test_secure_random_not_seeded(self):
        """Property: Cryptographic random cannot be predicted."""
        # Generate sequence, verify it's not reproducible
        seq1 = [secrets.randbelow(100) for _ in range(10)]
        seq2 = [secrets.randbelow(100) for _ in range(10)]
        # Sequences should differ (extremely high probability)
        assert seq1 != seq2
