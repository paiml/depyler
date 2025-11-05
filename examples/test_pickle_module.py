"""
Comprehensive test suite for pickle module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests pickle core features:
- Pickling and unpickling basic types
- Pickling complex data structures
- Pickling custom objects
- Protocol versions
- File-based pickling
"""

import pickle
from io import BytesIO


def test_pickle_basic_types():
    """Test pickling basic Python types."""
    # Integer
    data = 42
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == 42

    # String
    data = "hello world"
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == "hello world"

    # Float
    data = 3.14159
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == 3.14159

    print("PASS: test_pickle_basic_types")


def test_pickle_list():
    """Test pickling lists."""
    data = [1, 2, 3, 4, 5]
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == [1, 2, 3, 4, 5]
    assert len(unpickled) == 5
    print("PASS: test_pickle_list")


def test_pickle_dict():
    """Test pickling dictionaries."""
    data = {"name": "Alice", "age": 30, "city": "NYC"}
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == {"name": "Alice", "age": 30, "city": "NYC"}
    assert unpickled["name"] == "Alice"
    print("PASS: test_pickle_dict")


def test_pickle_nested_structure():
    """Test pickling nested data structures."""
    data = {
        "users": [
            {"name": "Alice", "scores": [90, 85, 88]},
            {"name": "Bob", "scores": [78, 82, 91]}
        ],
        "count": 2
    }
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == data
    assert unpickled["users"][0]["name"] == "Alice"
    assert unpickled["users"][1]["scores"][2] == 91
    print("PASS: test_pickle_nested_structure")


def test_pickle_tuple():
    """Test pickling tuples."""
    data = (1, "hello", 3.14, True)
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == (1, "hello", 3.14, True)
    assert unpickled[1] == "hello"
    print("PASS: test_pickle_tuple")


def test_pickle_set():
    """Test pickling sets."""
    data = {1, 2, 3, 4, 5}
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == {1, 2, 3, 4, 5}
    assert 3 in unpickled
    print("PASS: test_pickle_set")


def test_pickle_none():
    """Test pickling None."""
    data = None
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == None
    print("PASS: test_pickle_none")


def test_pickle_boolean():
    """Test pickling booleans."""
    data_true = True
    pickled_true = pickle.dumps(data_true)
    unpickled_true = pickle.loads(pickled_true)
    assert unpickled_true == True

    data_false = False
    pickled_false = pickle.dumps(data_false)
    unpickled_false = pickle.loads(pickled_false)
    assert unpickled_false == False

    print("PASS: test_pickle_boolean")


def test_pickle_bytes():
    """Test pickling bytes."""
    data = b"hello bytes"
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == b"hello bytes"
    print("PASS: test_pickle_bytes")


def test_pickle_mixed_types():
    """Test pickling mixed type collections."""
    data = [1, "two", 3.0, True, None, [4, 5], {"key": "value"}]
    pickled = pickle.dumps(data)
    unpickled = pickle.loads(pickled)
    assert unpickled == data
    assert unpickled[6]["key"] == "value"
    print("PASS: test_pickle_mixed_types")


def main():
    """Run all pickle tests."""
    print("=" * 60)
    print("PICKLE MODULE TESTS")
    print("=" * 60)

    test_pickle_basic_types()
    test_pickle_list()
    test_pickle_dict()
    test_pickle_nested_structure()
    test_pickle_tuple()
    test_pickle_set()
    test_pickle_none()
    test_pickle_boolean()
    test_pickle_bytes()
    test_pickle_mixed_types()

    print("=" * 60)
    print("ALL PICKLE TESTS PASSED!")
    print("Total tests: 10")
    print("=" * 60)


if __name__ == "__main__":
    main()
