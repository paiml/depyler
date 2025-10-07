"""
TDD Book - Phase 4: Network & IPC
Module: uuid - UUID objects
Coverage: UUID1, UUID3, UUID4, UUID5, UUID parsing, properties

Test Categories:
- UUID generation (uuid1, uuid3, uuid4, uuid5)
- UUID parsing and formatting
- UUID properties (version, variant, fields)
- UUID comparison and hashing
- UUID namespace constants
- Edge cases
"""

import uuid
import pytest


class TestUUID4:
    """Test uuid.uuid4() - random UUID generation."""

    def test_uuid4_basic(self):
        """Property: uuid4() generates random UUID."""
        u = uuid.uuid4()

        assert isinstance(u, uuid.UUID)

    def test_uuid4_version(self):
        """Property: uuid4() creates version 4 UUID."""
        u = uuid.uuid4()

        assert u.version == 4

    def test_uuid4_different_each_time(self):
        """Property: uuid4() generates unique UUIDs."""
        u1 = uuid.uuid4()
        u2 = uuid.uuid4()
        u3 = uuid.uuid4()

        assert u1 != u2 != u3

    def test_uuid4_string_format(self):
        """Property: uuid4() string is properly formatted."""
        u = uuid.uuid4()
        s = str(u)

        # Format: 8-4-4-4-12 hex digits with hyphens
        assert len(s) == 36
        assert s[8] == s[13] == s[18] == s[23] == "-"

    def test_uuid4_hex_property(self):
        """Property: UUID has hex property (32 hex digits)."""
        u = uuid.uuid4()

        assert len(u.hex) == 32
        assert all(c in "0123456789abcdef" for c in u.hex)


class TestUUID1:
    """Test uuid.uuid1() - time-based UUID generation."""

    def test_uuid1_basic(self):
        """Property: uuid1() generates time-based UUID."""
        u = uuid.uuid1()

        assert isinstance(u, uuid.UUID)

    def test_uuid1_version(self):
        """Property: uuid1() creates version 1 UUID."""
        u = uuid.uuid1()

        assert u.version == 1

    def test_uuid1_sequential(self):
        """Property: uuid1() generates different UUIDs sequentially."""
        u1 = uuid.uuid1()
        u2 = uuid.uuid1()

        assert u1 != u2

    def test_uuid1_has_timestamp(self):
        """Property: uuid1() contains timestamp information."""
        u = uuid.uuid1()

        # Version 1 UUIDs have time fields
        assert hasattr(u, "time")
        assert u.time is not None


class TestUUID3:
    """Test uuid.uuid3() - name-based UUID with MD5."""

    def test_uuid3_basic(self):
        """Property: uuid3() generates UUID from namespace and name."""
        namespace = uuid.NAMESPACE_DNS
        name = "example.com"

        u = uuid.uuid3(namespace, name)

        assert isinstance(u, uuid.UUID)

    def test_uuid3_version(self):
        """Property: uuid3() creates version 3 UUID."""
        u = uuid.uuid3(uuid.NAMESPACE_DNS, "test.com")

        assert u.version == 3

    def test_uuid3_deterministic(self):
        """Property: uuid3() is deterministic (same input = same UUID)."""
        namespace = uuid.NAMESPACE_DNS
        name = "example.com"

        u1 = uuid.uuid3(namespace, name)
        u2 = uuid.uuid3(namespace, name)
        u3 = uuid.uuid3(namespace, name)

        assert u1 == u2 == u3

    def test_uuid3_different_names(self):
        """Property: Different names produce different UUIDs."""
        namespace = uuid.NAMESPACE_DNS

        u1 = uuid.uuid3(namespace, "example1.com")
        u2 = uuid.uuid3(namespace, "example2.com")

        assert u1 != u2

    def test_uuid3_different_namespaces(self):
        """Property: Different namespaces produce different UUIDs."""
        name = "example.com"

        u1 = uuid.uuid3(uuid.NAMESPACE_DNS, name)
        u2 = uuid.uuid3(uuid.NAMESPACE_URL, name)

        assert u1 != u2


class TestUUID5:
    """Test uuid.uuid5() - name-based UUID with SHA-1."""

    def test_uuid5_basic(self):
        """Property: uuid5() generates UUID from namespace and name."""
        namespace = uuid.NAMESPACE_DNS
        name = "example.com"

        u = uuid.uuid5(namespace, name)

        assert isinstance(u, uuid.UUID)

    def test_uuid5_version(self):
        """Property: uuid5() creates version 5 UUID."""
        u = uuid.uuid5(uuid.NAMESPACE_DNS, "test.com")

        assert u.version == 5

    def test_uuid5_deterministic(self):
        """Property: uuid5() is deterministic."""
        namespace = uuid.NAMESPACE_DNS
        name = "example.com"

        u1 = uuid.uuid5(namespace, name)
        u2 = uuid.uuid5(namespace, name)

        assert u1 == u2

    def test_uuid5_different_from_uuid3(self):
        """Property: uuid5() produces different result than uuid3()."""
        namespace = uuid.NAMESPACE_DNS
        name = "example.com"

        u3 = uuid.uuid3(namespace, name)
        u5 = uuid.uuid5(namespace, name)

        assert u3 != u5


class TestUUIDParsing:
    """Test UUID parsing from strings."""

    def test_uuid_from_string(self):
        """Property: UUID() parses UUID string."""
        s = "550e8400-e29b-41d4-a716-446655440000"
        u = uuid.UUID(s)

        assert str(u) == s

    def test_uuid_from_hex(self):
        """Property: UUID() parses hex string."""
        hex_str = "550e8400e29b41d4a716446655440000"
        u = uuid.UUID(hex=hex_str)

        assert u.hex == hex_str

    def test_uuid_from_bytes(self):
        """Property: UUID() parses from bytes."""
        u1 = uuid.uuid4()
        u2 = uuid.UUID(bytes=u1.bytes)

        assert u1 == u2

    def test_uuid_from_int(self):
        """Property: UUID() parses from integer."""
        u1 = uuid.uuid4()
        u2 = uuid.UUID(int=u1.int)

        assert u1 == u2

    def test_uuid_from_fields(self):
        """Property: UUID() can be constructed from fields."""
        u = uuid.UUID(fields=(0x12345678, 0x1234, 0x5678, 0x12, 0x34, 0x567812345678))

        assert isinstance(u, uuid.UUID)

    def test_uuid_invalid_string_raises(self):
        """Property: Invalid UUID string raises ValueError."""
        with pytest.raises(ValueError):
            uuid.UUID("not-a-uuid")

    def test_uuid_wrong_length_raises(self):
        """Property: Wrong length string raises ValueError."""
        with pytest.raises(ValueError):
            uuid.UUID("550e8400-e29b-41d4")  # Too short


class TestUUIDProperties:
    """Test UUID object properties."""

    def test_uuid_bytes_property(self):
        """Property: UUID has bytes property (16 bytes)."""
        u = uuid.uuid4()

        assert len(u.bytes) == 16
        assert isinstance(u.bytes, bytes)

    def test_uuid_int_property(self):
        """Property: UUID has int property."""
        u = uuid.uuid4()

        assert isinstance(u.int, int)
        assert 0 <= u.int < 2**128

    def test_uuid_hex_property(self):
        """Property: UUID hex is 32 hex characters."""
        u = uuid.uuid4()

        assert len(u.hex) == 32

    def test_uuid_urn_property(self):
        """Property: UUID has URN representation."""
        u = uuid.uuid4()
        urn = u.urn

        assert urn.startswith("urn:uuid:")
        assert str(u) in urn

    def test_uuid_variant_property(self):
        """Property: UUID has variant property."""
        u = uuid.uuid4()

        # RFC 4122 variant
        assert hasattr(u, "variant")
        assert u.variant is not None

    def test_uuid_fields_property(self):
        """Property: UUID has fields tuple."""
        u = uuid.uuid4()
        fields = u.fields

        assert isinstance(fields, tuple)
        assert len(fields) == 6  # 6 field components


class TestUUIDComparison:
    """Test UUID comparison and equality."""

    def test_uuid_equality(self):
        """Property: Equal UUIDs are equal."""
        s = "550e8400-e29b-41d4-a716-446655440000"
        u1 = uuid.UUID(s)
        u2 = uuid.UUID(s)

        assert u1 == u2

    def test_uuid_inequality(self):
        """Property: Different UUIDs are not equal."""
        u1 = uuid.uuid4()
        u2 = uuid.uuid4()

        assert u1 != u2

    def test_uuid_comparison(self):
        """Property: UUIDs can be compared."""
        u1 = uuid.UUID("00000000-0000-0000-0000-000000000001")
        u2 = uuid.UUID("00000000-0000-0000-0000-000000000002")

        assert u1 < u2
        assert u2 > u1

    def test_uuid_hash(self):
        """Property: UUIDs are hashable."""
        u = uuid.uuid4()

        hash_val = hash(u)
        assert isinstance(hash_val, int)

    def test_uuid_in_set(self):
        """Property: UUIDs can be used in sets."""
        u1 = uuid.uuid4()
        u2 = uuid.uuid4()

        s = {u1, u2}
        assert u1 in s
        assert u2 in s
        assert len(s) == 2


class TestUUIDNamespaces:
    """Test UUID namespace constants."""

    def test_namespace_dns(self):
        """Property: NAMESPACE_DNS is defined."""
        assert isinstance(uuid.NAMESPACE_DNS, uuid.UUID)

    def test_namespace_url(self):
        """Property: NAMESPACE_URL is defined."""
        assert isinstance(uuid.NAMESPACE_URL, uuid.UUID)

    def test_namespace_oid(self):
        """Property: NAMESPACE_OID is defined."""
        assert isinstance(uuid.NAMESPACE_OID, uuid.UUID)

    def test_namespace_x500(self):
        """Property: NAMESPACE_X500 is defined."""
        assert isinstance(uuid.NAMESPACE_X500, uuid.UUID)

    def test_namespaces_different(self):
        """Property: All namespaces are different."""
        namespaces = {
            uuid.NAMESPACE_DNS,
            uuid.NAMESPACE_URL,
            uuid.NAMESPACE_OID,
            uuid.NAMESPACE_X500,
        }

        assert len(namespaces) == 4


class TestUUIDRepresentation:
    """Test UUID string representations."""

    def test_uuid_str(self):
        """Property: str(UUID) returns canonical format."""
        u = uuid.UUID("550e8400-e29b-41d4-a716-446655440000")
        s = str(u)

        assert s == "550e8400-e29b-41d4-a716-446655440000"
        assert len(s) == 36

    def test_uuid_repr(self):
        """Property: repr(UUID) shows UUID constructor."""
        u = uuid.UUID("550e8400-e29b-41d4-a716-446655440000")
        r = repr(u)

        assert "UUID" in r
        assert "550e8400-e29b-41d4-a716-446655440000" in r

    def test_uuid_bytes_le(self):
        """Property: UUID has little-endian bytes representation."""
        u = uuid.uuid4()

        assert len(u.bytes_le) == 16
        assert isinstance(u.bytes_le, bytes)


class TestUUIDEdgeCases:
    """Test edge cases and special scenarios."""

    def test_nil_uuid(self):
        """Property: All-zeros UUID is valid (nil UUID)."""
        nil = uuid.UUID("00000000-0000-0000-0000-000000000000")

        assert nil.int == 0
        assert nil.hex == "00000000000000000000000000000000"

    def test_max_uuid(self):
        """Property: All-ones UUID is valid."""
        max_uuid = uuid.UUID("ffffffff-ffff-ffff-ffff-ffffffffffff")

        assert max_uuid.int == 2**128 - 1

    def test_uuid_immutable(self):
        """Property: UUID objects are immutable."""
        u = uuid.uuid4()

        # Should not be able to modify attributes
        with pytest.raises(TypeError, match="immutable"):
            u.version = 99

    def test_uuid_roundtrip_string(self):
        """Property: UUID string roundtrip preserves value."""
        original = uuid.uuid4()
        s = str(original)
        recovered = uuid.UUID(s)

        assert original == recovered

    def test_uuid_roundtrip_bytes(self):
        """Property: UUID bytes roundtrip preserves value."""
        original = uuid.uuid4()
        b = original.bytes
        recovered = uuid.UUID(bytes=b)

        assert original == recovered

    def test_uuid_roundtrip_int(self):
        """Property: UUID int roundtrip preserves value."""
        original = uuid.uuid4()
        i = original.int
        recovered = uuid.UUID(int=i)

        assert original == recovered

    def test_uuid_case_insensitive_parsing(self):
        """Property: UUID parsing is case-insensitive."""
        lower = uuid.UUID("550e8400-e29b-41d4-a716-446655440000")
        upper = uuid.UUID("550E8400-E29B-41D4-A716-446655440000")

        assert lower == upper

    def test_uuid_braces_parsing(self):
        """Property: UUID() accepts braced format."""
        u1 = uuid.UUID("550e8400-e29b-41d4-a716-446655440000")
        u2 = uuid.UUID("{550e8400-e29b-41d4-a716-446655440000}")

        assert u1 == u2

    def test_uuid_urn_parsing(self):
        """Property: UUID() parses URN format."""
        u1 = uuid.UUID("550e8400-e29b-41d4-a716-446655440000")
        u2 = uuid.UUID("urn:uuid:550e8400-e29b-41d4-a716-446655440000")

        assert u1 == u2

    def test_uuid3_with_bytes_name(self):
        """Property: uuid3() accepts bytes name."""
        namespace = uuid.NAMESPACE_DNS

        u1 = uuid.uuid3(namespace, b"example.com")
        u2 = uuid.uuid3(namespace, b"example.com")

        assert u1 == u2

    def test_uuid5_with_bytes_name(self):
        """Property: uuid5() accepts bytes name."""
        namespace = uuid.NAMESPACE_DNS

        u1 = uuid.uuid5(namespace, b"example.com")
        u2 = uuid.uuid5(namespace, b"example.com")

        assert u1 == u2

    def test_uuid_version_for_all_types(self):
        """Property: All UUID versions have version property."""
        u1 = uuid.uuid1()
        u3 = uuid.uuid3(uuid.NAMESPACE_DNS, "test")
        u4 = uuid.uuid4()
        u5 = uuid.uuid5(uuid.NAMESPACE_DNS, "test")

        assert u1.version == 1
        assert u3.version == 3
        assert u4.version == 4
        assert u5.version == 5

    def test_uuid_safe_attr(self):
        """Property: uuid1() has safe attribute."""
        u = uuid.uuid1()

        # safe indicates whether node was generated safely
        assert hasattr(u, "is_safe")

    def test_uuid_clock_seq_attr(self):
        """Property: uuid1() has clock_seq attribute."""
        u = uuid.uuid1()

        assert hasattr(u, "clock_seq")
        assert isinstance(u.clock_seq, int)
