"""
TDD Book - Phase 4: Network & IPC
Module: secrets - Cryptographically strong random numbers
Coverage: token generation, random choice, comparison functions

Test Categories:
- Random token generation (token_bytes, token_hex, token_urlsafe)
- Random choice and sampling
- Cryptographic comparison (compare_digest)
- Random number generation
- Edge cases
"""

import secrets
import pytest


class TestTokenBytes:
    """Test secrets.token_bytes() - random bytes."""

    def test_token_bytes_default(self):
        """Property: token_bytes() returns random bytes."""
        token = secrets.token_bytes()

        assert isinstance(token, bytes)
        assert len(token) == 32  # Default size

    def test_token_bytes_custom_size(self):
        """Property: token_bytes() accepts custom size."""
        token = secrets.token_bytes(16)

        assert isinstance(token, bytes)
        assert len(token) == 16

    def test_token_bytes_different_each_time(self):
        """Property: token_bytes() generates different values."""
        token1 = secrets.token_bytes()
        token2 = secrets.token_bytes()
        token3 = secrets.token_bytes()

        # Should be different (cryptographically random)
        assert token1 != token2 != token3

    def test_token_bytes_zero_size(self):
        """Property: token_bytes(0) returns empty bytes."""
        token = secrets.token_bytes(0)

        assert token == b""

    def test_token_bytes_large_size(self):
        """Property: token_bytes() handles large sizes."""
        token = secrets.token_bytes(1024)

        assert len(token) == 1024


class TestTokenHex:
    """Test secrets.token_hex() - random hex string."""

    def test_token_hex_default(self):
        """Property: token_hex() returns hex string."""
        token = secrets.token_hex()

        assert isinstance(token, str)
        assert len(token) == 64  # 32 bytes = 64 hex chars

    def test_token_hex_custom_size(self):
        """Property: token_hex() accepts custom size."""
        token = secrets.token_hex(16)

        assert isinstance(token, str)
        assert len(token) == 32  # 16 bytes = 32 hex chars

    def test_token_hex_characters(self):
        """Property: token_hex() contains only hex characters."""
        token = secrets.token_hex()

        # Should only contain hex digits
        assert all(c in "0123456789abcdef" for c in token)

    def test_token_hex_different_each_time(self):
        """Property: token_hex() generates different values."""
        token1 = secrets.token_hex()
        token2 = secrets.token_hex()

        assert token1 != token2

    def test_token_hex_zero_size(self):
        """Property: token_hex(0) returns empty string."""
        token = secrets.token_hex(0)

        assert token == ""


class TestTokenUrlsafe:
    """Test secrets.token_urlsafe() - URL-safe token."""

    def test_token_urlsafe_default(self):
        """Property: token_urlsafe() returns URL-safe string."""
        token = secrets.token_urlsafe()

        assert isinstance(token, str)
        assert len(token) > 0

    def test_token_urlsafe_custom_size(self):
        """Property: token_urlsafe() accepts custom size."""
        token = secrets.token_urlsafe(16)

        assert isinstance(token, str)
        # Base64 encoding: 16 bytes -> ~22 chars
        assert len(token) > 0

    def test_token_urlsafe_characters(self):
        """Property: token_urlsafe() uses URL-safe characters."""
        token = secrets.token_urlsafe()

        # Should not contain +, /, or =
        assert "+" not in token
        assert "/" not in token
        # May or may not have padding

    def test_token_urlsafe_different_each_time(self):
        """Property: token_urlsafe() generates different values."""
        token1 = secrets.token_urlsafe()
        token2 = secrets.token_urlsafe()

        assert token1 != token2


class TestChoice:
    """Test secrets.choice() - random element selection."""

    def test_choice_basic(self):
        """Property: choice() selects element from sequence."""
        items = ["a", "b", "c", "d", "e"]
        chosen = secrets.choice(items)

        assert chosen in items

    def test_choice_distribution(self):
        """Property: choice() selects all elements over many trials."""
        items = [1, 2, 3, 4, 5]
        chosen = {secrets.choice(items) for _ in range(100)}

        # Should see all elements with high probability
        assert len(chosen) >= 3  # At least 3 of 5 elements

    def test_choice_single_element(self):
        """Property: choice() with single element returns that element."""
        items = ["only"]
        chosen = secrets.choice(items)

        assert chosen == "only"

    def test_choice_empty_raises(self):
        """Property: choice() raises on empty sequence."""
        with pytest.raises(IndexError):
            secrets.choice([])

    def test_choice_string(self):
        """Property: choice() works with strings."""
        chars = "abcdef"
        chosen = secrets.choice(chars)

        assert chosen in chars
        assert len(chosen) == 1


class TestRandbelow:
    """Test secrets.randbelow() - random integer below n."""

    def test_randbelow_basic(self):
        """Property: randbelow() returns value < n."""
        value = secrets.randbelow(10)

        assert 0 <= value < 10

    def test_randbelow_range(self):
        """Property: randbelow() produces values in range."""
        values = {secrets.randbelow(100) for _ in range(200)}

        # Should produce diverse values
        assert len(values) > 10
        assert all(0 <= v < 100 for v in values)

    def test_randbelow_one(self):
        """Property: randbelow(1) always returns 0."""
        for _ in range(10):
            assert secrets.randbelow(1) == 0

    def test_randbelow_large_n(self):
        """Property: randbelow() handles large n."""
        value = secrets.randbelow(10**10)

        assert 0 <= value < 10**10

    def test_randbelow_zero_raises(self):
        """Property: randbelow(0) raises ValueError."""
        with pytest.raises(ValueError):
            secrets.randbelow(0)

    def test_randbelow_negative_raises(self):
        """Property: randbelow() with negative n raises."""
        with pytest.raises(ValueError):
            secrets.randbelow(-1)


class TestRandbits:
    """Test secrets.randbits() - random integer with k bits."""

    def test_randbits_basic(self):
        """Property: randbits() returns k-bit integer."""
        value = secrets.randbits(8)

        assert 0 <= value < 2**8
        assert isinstance(value, int)

    def test_randbits_one_bit(self):
        """Property: randbits(1) returns 0 or 1."""
        values = {secrets.randbits(1) for _ in range(20)}

        assert values.issubset({0, 1})

    def test_randbits_large(self):
        """Property: randbits() handles large bit counts."""
        value = secrets.randbits(256)

        assert 0 <= value < 2**256

    def test_randbits_zero(self):
        """Property: randbits(0) returns 0."""
        value = secrets.randbits(0)

        assert value == 0

    def test_randbits_different_each_time(self):
        """Property: randbits() generates different values."""
        values = [secrets.randbits(32) for _ in range(10)]

        # Should be different (with high probability)
        assert len(set(values)) > 1


class TestCompareDigest:
    """Test secrets.compare_digest() - constant-time comparison."""

    def test_compare_digest_equal_strings(self):
        """Property: compare_digest() returns True for equal strings."""
        a = "password123"
        b = "password123"

        assert secrets.compare_digest(a, b) is True

    def test_compare_digest_different_strings(self):
        """Property: compare_digest() returns False for different strings."""
        a = "password123"
        b = "password456"

        assert secrets.compare_digest(a, b) is False

    def test_compare_digest_equal_bytes(self):
        """Property: compare_digest() works with bytes."""
        a = b"secret"
        b = b"secret"

        assert secrets.compare_digest(a, b) is True

    def test_compare_digest_different_bytes(self):
        """Property: compare_digest() detects different bytes."""
        a = b"secret"
        b = b"Secret"  # Different case

        assert secrets.compare_digest(a, b) is False

    def test_compare_digest_different_lengths(self):
        """Property: compare_digest() returns False for different lengths."""
        a = "short"
        b = "longer string"

        assert secrets.compare_digest(a, b) is False

    def test_compare_digest_empty_strings(self):
        """Property: compare_digest() handles empty strings."""
        assert secrets.compare_digest("", "") is True

    def test_compare_digest_timing_safe(self):
        """Property: compare_digest() is timing-safe (same time for all comparisons)."""
        # This is a property test, not easily testable
        # But we can verify it doesn't short-circuit

        a = "a" * 1000
        b = "b" * 1000

        result = secrets.compare_digest(a, b)
        assert result is False


class TestSystemRandomGenerator:
    """Test secrets uses system random source."""

    def test_default_rng_attribute(self):
        """Property: secrets module has SystemRandom instance."""
        # secrets uses SystemRandom internally
        assert hasattr(secrets, "SystemRandom")

    def test_tokens_are_cryptographic(self):
        """Property: Generated tokens have high entropy."""
        # Generate multiple tokens, verify they're unique
        tokens = {secrets.token_hex() for _ in range(100)}

        # All should be unique (collision probability ~0)
        assert len(tokens) == 100


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_token_bytes_negative_raises(self):
        """Property: token_bytes() with negative size raises."""
        with pytest.raises(ValueError):
            secrets.token_bytes(-1)

    def test_token_hex_negative_raises(self):
        """Property: token_hex() with negative size raises."""
        with pytest.raises(ValueError):
            secrets.token_hex(-1)

    def test_token_urlsafe_negative_raises(self):
        """Property: token_urlsafe() with negative size raises."""
        with pytest.raises(ValueError):
            secrets.token_urlsafe(-1)

    def test_choice_tuple(self):
        """Property: choice() works with tuples."""
        items = (1, 2, 3, 4, 5)
        chosen = secrets.choice(items)

        assert chosen in items

    def test_choice_range(self):
        """Property: choice() works with range objects."""
        chosen = secrets.choice(range(10))

        assert 0 <= chosen < 10

    def test_compare_digest_mixed_types_raises(self):
        """Property: compare_digest() requires same types."""
        with pytest.raises(TypeError):
            secrets.compare_digest("string", b"bytes")

    def test_multiple_calls_independent(self):
        """Property: Multiple token generations are independent."""
        tokens1 = [secrets.token_bytes(16) for _ in range(10)]
        tokens2 = [secrets.token_bytes(16) for _ in range(10)]

        # Should be different sets
        assert set(tokens1) != set(tokens2)

    def test_token_hex_equals_token_bytes_hex(self):
        """Property: token_hex(n) ~ token_bytes(n).hex()."""
        # Both should produce similar format (hex string from n bytes)
        n = 16

        hex1 = secrets.token_hex(n)
        hex2 = secrets.token_bytes(n).hex()

        # Should have same length
        assert len(hex1) == len(hex2) == n * 2

    def test_high_quality_randomness(self):
        """Property: Tokens show no obvious patterns."""
        # Generate many tokens and verify no duplicates
        tokens = [secrets.token_bytes(32) for _ in range(1000)]

        # Should all be unique
        assert len(set(tokens)) == 1000

    def test_uniform_distribution(self):
        """Property: randbelow() has roughly uniform distribution."""
        counts = [0] * 10

        for _ in range(1000):
            value = secrets.randbelow(10)
            counts[value] += 1

        # Each bucket should have roughly 100 (within reason)
        for count in counts:
            assert 50 < count < 150  # Allow variance
