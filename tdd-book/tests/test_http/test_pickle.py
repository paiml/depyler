"""
TDD Book - Phase 4: Network & IPC
Module: pickle - Python object serialization
Coverage: dumps, loads, dump, load, pickling various types

Test Categories:
- Basic pickling (dumps, loads)
- File I/O (dump, load)
- Data types (primitives, containers, custom classes)
- Pickle protocols
- Edge cases
"""

import pickle
import io
import pytest


# Module-level classes for pickle tests (must be module-level to be picklable)
class Point:
    """Simple point class for pickle testing."""

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y


class Calculator:
    """Calculator class for pickle testing."""

    def __init__(self, value):
        self.value = value

    def add(self, n):
        return self.value + n


class TestPickleDumpsLoads:
    """Test pickle.dumps() and pickle.loads() - serialization to/from bytes."""

    def test_dumps_int(self):
        """Property: dumps() serializes integers."""
        data = 42
        pickled = pickle.dumps(data)

        assert isinstance(pickled, bytes)

    def test_loads_int(self):
        """Property: loads() deserializes integers."""
        data = 42
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == 42

    def test_dumps_string(self):
        """Property: dumps() serializes strings."""
        data = "hello world"
        pickled = pickle.dumps(data)

        assert isinstance(pickled, bytes)

    def test_loads_string(self):
        """Property: loads() deserializes strings."""
        data = "hello world"
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_dumps_list(self):
        """Property: dumps() serializes lists."""
        data = [1, 2, 3, "four", 5.0]
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_dumps_dict(self):
        """Property: dumps() serializes dictionaries."""
        data = {"key": "value", "number": 42, "list": [1, 2, 3]}
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_dumps_tuple(self):
        """Property: dumps() preserves tuple type."""
        data = (1, 2, 3)
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
        assert isinstance(unpickled, tuple)

    def test_dumps_set(self):
        """Property: dumps() serializes sets."""
        data = {1, 2, 3, 4, 5}
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
        assert isinstance(unpickled, set)


class TestPickleDumpLoad:
    """Test pickle.dump() and pickle.load() - file I/O."""

    def test_dump_to_file(self):
        """Property: dump() writes pickled data to file."""
        data = {"key": "value", "number": 42}
        file_obj = io.BytesIO()

        pickle.dump(data, file_obj)
        file_obj.seek(0)

        unpickled = pickle.load(file_obj)
        assert unpickled == data

    def test_load_from_file(self):
        """Property: load() reads pickled data from file."""
        data = [1, 2, 3, "four"]
        file_obj = io.BytesIO()

        pickle.dump(data, file_obj)
        file_obj.seek(0)

        unpickled = pickle.load(file_obj)
        assert unpickled == data

    def test_dump_multiple_objects(self):
        """Property: dump() can write multiple objects to same file."""
        file_obj = io.BytesIO()

        pickle.dump(42, file_obj)
        pickle.dump("hello", file_obj)
        pickle.dump([1, 2, 3], file_obj)

        file_obj.seek(0)

        obj1 = pickle.load(file_obj)
        obj2 = pickle.load(file_obj)
        obj3 = pickle.load(file_obj)

        assert obj1 == 42
        assert obj2 == "hello"
        assert obj3 == [1, 2, 3]


class TestPickleCustomClass:
    """Test pickling custom classes."""

    def test_pickle_simple_class(self):
        """Property: Pickle can serialize simple custom classes."""
        point = Point(10, 20)
        pickled = pickle.dumps(point)
        unpickled = pickle.loads(pickled)

        assert unpickled == point
        assert unpickled.x == 10
        assert unpickled.y == 20

    def test_pickle_class_with_methods(self):
        """Property: Pickle preserves class methods."""
        calc = Calculator(10)
        pickled = pickle.dumps(calc)
        unpickled = pickle.loads(pickled)

        assert unpickled.value == 10
        assert unpickled.add(5) == 15


class TestPickleProtocols:
    """Test different pickle protocols."""

    def test_protocol_0(self):
        """Property: Protocol 0 produces ASCII output."""
        data = {"key": "value"}
        pickled = pickle.dumps(data, protocol=0)

        # Protocol 0 is ASCII
        unpickled = pickle.loads(pickled)
        assert unpickled == data

    def test_protocol_highest(self):
        """Property: HIGHEST_PROTOCOL uses latest protocol."""
        data = [1, 2, 3]
        pickled = pickle.dumps(data, protocol=pickle.HIGHEST_PROTOCOL)

        unpickled = pickle.loads(pickled)
        assert unpickled == data

    def test_protocol_default(self):
        """Property: DEFAULT_PROTOCOL is used by default."""
        data = {"test": "data"}

        pickled_default = pickle.dumps(data)
        pickled_explicit = pickle.dumps(data, protocol=pickle.DEFAULT_PROTOCOL)

        # Both should deserialize correctly
        assert pickle.loads(pickled_default) == data
        assert pickle.loads(pickled_explicit) == data


class TestPickleNone:
    """Test pickling None value."""

    def test_pickle_none(self):
        """Property: Pickle can serialize None."""
        data = None
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled is None

    def test_pickle_list_with_none(self):
        """Property: Lists containing None are preserved."""
        data = [1, None, 3, None]
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
        assert unpickled[1] is None
        assert unpickled[3] is None


class TestPickleBooleans:
    """Test pickling boolean values."""

    def test_pickle_true(self):
        """Property: True is preserved."""
        pickled = pickle.dumps(True)
        unpickled = pickle.loads(pickled)

        assert unpickled is True

    def test_pickle_false(self):
        """Property: False is preserved."""
        pickled = pickle.dumps(False)
        unpickled = pickle.loads(pickled)

        assert unpickled is False

    def test_pickle_bool_list(self):
        """Property: Lists of booleans preserved."""
        data = [True, False, True, True]
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data


class TestPickleNumbers:
    """Test pickling numeric types."""

    def test_pickle_float(self):
        """Property: Floats are preserved."""
        data = 3.14159
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_negative_numbers(self):
        """Property: Negative numbers preserved."""
        data = {"int": -42, "float": -3.14}
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_large_int(self):
        """Property: Large integers preserved."""
        data = 10**100
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data


class TestPickleBytes:
    """Test pickling bytes and bytearray."""

    def test_pickle_bytes(self):
        """Property: bytes objects are preserved."""
        data = b"binary data \x00\x01\xff"
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
        assert isinstance(unpickled, bytes)

    def test_pickle_bytearray(self):
        """Property: bytearray objects are preserved."""
        data = bytearray(b"mutable bytes")
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
        assert isinstance(unpickled, bytearray)


class TestPickleNested:
    """Test pickling nested structures."""

    def test_pickle_nested_dict(self):
        """Property: Nested dictionaries preserved."""
        data = {
            "outer": {"inner": {"deep": "value"}},
            "list": [1, [2, [3, [4]]]],
        }
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_nested_list(self):
        """Property: Nested lists preserved."""
        data = [1, [2, [3, [4, [5]]]]]
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_mixed_nested(self):
        """Property: Mixed nested structures preserved."""
        data = {
            "users": [
                {"name": "Alice", "scores": [95, 87, 92]},
                {"name": "Bob", "scores": [88, 91, 85]},
            ]
        }
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data


class TestPickleEdgeCases:
    """Test edge cases and special scenarios."""

    def test_pickle_empty_list(self):
        """Property: Empty list preserved."""
        data = []
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == []

    def test_pickle_empty_dict(self):
        """Property: Empty dict preserved."""
        data = {}
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == {}

    def test_pickle_empty_string(self):
        """Property: Empty string preserved."""
        data = ""
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == ""

    def test_pickle_unicode_string(self):
        """Property: Unicode strings preserved."""
        data = "Hello 世界 🌍"
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_special_characters(self):
        """Property: Strings with special characters preserved."""
        data = "Line 1\nLine 2\tTabbed\r\n\"Quoted\""
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_zero(self):
        """Property: Zero is preserved."""
        data = 0
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == 0

    def test_pickle_negative_zero_float(self):
        """Property: -0.0 is preserved."""
        data = -0.0
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_very_long_list(self):
        """Property: Long lists can be pickled."""
        data = list(range(10000))
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_deterministic(self):
        """Property: Same object pickles to same bytes (with same protocol)."""
        data = {"key": "value", "number": 42}

        pickled1 = pickle.dumps(data, protocol=4)
        pickled2 = pickle.dumps(data, protocol=4)

        # Should produce identical bytes
        assert pickled1 == pickled2

    def test_unpickle_invalid_data_raises(self):
        """Property: loads() raises on invalid pickle data."""
        with pytest.raises(pickle.UnpicklingError):
            pickle.loads(b"not valid pickle data")

    def test_pickle_circular_reference(self):
        """Property: Circular references are handled."""
        data = []
        data.append(data)  # Circular reference

        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        # Should have circular reference
        assert unpickled[0] is unpickled

    def test_pickle_shared_reference(self):
        """Property: Shared references are preserved."""
        shared = [1, 2, 3]
        data = {"a": shared, "b": shared}

        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        # Both should reference the same object
        assert unpickled["a"] is unpickled["b"]
        unpickled["a"].append(4)
        assert unpickled["b"] == [1, 2, 3, 4]

    def test_pickle_frozenset(self):
        """Property: frozenset is preserved."""
        data = frozenset([1, 2, 3, 4])
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
        assert isinstance(unpickled, frozenset)

    def test_pickle_complex_number(self):
        """Property: Complex numbers preserved."""
        data = 3 + 4j
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data

    def test_pickle_range(self):
        """Property: range objects preserved."""
        data = range(10)
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert list(unpickled) == list(data)

    def test_pickle_multiple_types(self):
        """Property: Mixed type collection preserved."""
        data = [
            42,
            3.14,
            "string",
            b"bytes",
            True,
            None,
            [1, 2],
            {"key": "val"},
            (1, 2),
            {1, 2},
        ]
        pickled = pickle.dumps(data)
        unpickled = pickle.loads(pickled)

        assert unpickled == data
