"""Test copy module - Shallow and deep copy operations.

This module tests copy for creating shallow and deep copies of objects,
crucial for avoiding unintended mutations and managing object lifecycles.
"""

import copy
import pytest


class TestShallowCopy:
    """Shallow copy with copy.copy()."""

    def test_copy_list(self):
        """Basic: Shallow copy a list."""
        original = [1, 2, 3]
        copied = copy.copy(original)
        assert copied == original
        assert copied is not original  # Different objects

    def test_copy_dict(self):
        """Basic: Shallow copy a dict."""
        original = {"a": 1, "b": 2}
        copied = copy.copy(original)
        assert copied == original
        assert copied is not original

    def test_shallow_copy_independence(self):
        """Property: Shallow copy creates independent top-level object."""
        original = [1, 2, 3]
        copied = copy.copy(original)
        copied.append(4)
        assert copied == [1, 2, 3, 4]
        assert original == [1, 2, 3]  # Original unchanged

    def test_shallow_copy_shares_nested(self):
        """Property: Shallow copy shares nested objects."""
        inner = [1, 2]
        original = [inner, 3]
        copied = copy.copy(original)

        # Modifying nested object affects both
        copied[0].append(3)
        assert original == [[1, 2, 3], 3]  # Original changed!
        assert copied == [[1, 2, 3], 3]

    def test_shallow_copy_dict_shares_values(self):
        """Property: Shallow dict copy shares value objects."""
        inner = [1, 2]
        original = {"list": inner, "num": 42}
        copied = copy.copy(original)

        copied["list"].append(3)
        assert original["list"] == [1, 2, 3]  # Shared!


class TestDeepCopy:
    """Deep copy with copy.deepcopy()."""

    def test_deepcopy_list(self):
        """Basic: Deep copy a list."""
        original = [1, 2, 3]
        copied = copy.deepcopy(original)
        assert copied == original
        assert copied is not original

    def test_deepcopy_nested_independence(self):
        """Property: Deep copy creates independent nested objects."""
        inner = [1, 2]
        original = [inner, 3]
        copied = copy.deepcopy(original)

        # Modifying nested object doesn't affect original
        copied[0].append(3)
        assert original == [[1, 2], 3]  # Original unchanged!
        assert copied == [[1, 2, 3], 3]

    def test_deepcopy_dict_independence(self):
        """Property: Deep copy dict with independent values."""
        inner = [1, 2]
        original = {"list": inner, "num": 42}
        copied = copy.deepcopy(original)

        copied["list"].append(3)
        assert original["list"] == [1, 2]  # Independent!
        assert copied["list"] == [1, 2, 3]

    def test_deepcopy_deeply_nested(self):
        """Property: Deep copy handles deeply nested structures."""
        original = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
        copied = copy.deepcopy(original)

        copied[0][0].append(99)
        assert "99" not in str(original)  # Original unchanged


class TestImmutableObjects:
    """Copying immutable objects."""

    def test_copy_int(self):
        """Property: Copying immutable objects returns same object."""
        x = 42
        copied = copy.copy(x)
        assert copied == x
        assert copied is x  # Same object (immutables aren't copied)

    def test_copy_string(self):
        """Property: String copy returns same object."""
        s = "hello"
        copied = copy.copy(s)
        assert copied == s
        assert copied is s

    def test_copy_tuple(self):
        """Property: Tuple copy returns same object."""
        t = (1, 2, 3)
        copied = copy.copy(t)
        assert copied == t
        assert copied is t

    def test_deepcopy_immutables(self):
        """Property: Deep copy of immutables may return same object."""
        # Immutables are safe to share
        x = 42
        copied = copy.deepcopy(x)
        assert copied == x


class TestCustomObjects:
    """Copying custom class instances."""

    def test_copy_simple_object(self):
        """Basic: Copy simple custom object."""
        class Point:
            def __init__(self, x, y):
                self.x = x
                self.y = y

        p1 = Point(1, 2)
        p2 = copy.copy(p1)

        assert p2.x == p1.x
        assert p2.y == p1.y
        assert p2 is not p1

    def test_shallow_copy_object_with_list(self):
        """Property: Shallow copy shares mutable attributes."""
        class Container:
            def __init__(self, items):
                self.items = items

        c1 = Container([1, 2, 3])
        c2 = copy.copy(c1)

        c2.items.append(4)
        assert c1.items == [1, 2, 3, 4]  # Shared!

    def test_deepcopy_object_with_list(self):
        """Property: Deep copy creates independent attributes."""
        class Container:
            def __init__(self, items):
                self.items = items

        c1 = Container([1, 2, 3])
        c2 = copy.deepcopy(c1)

        c2.items.append(4)
        assert c1.items == [1, 2, 3]  # Independent!
        assert c2.items == [1, 2, 3, 4]


class TestCircularReferences:
    """Handling circular references."""

    def test_deepcopy_circular_list(self):
        """Feature: Deep copy handles circular references."""
        lst = [1, 2]
        lst.append(lst)  # Circular reference

        copied = copy.deepcopy(lst)
        assert copied[0] == 1
        assert copied[1] == 2
        assert copied[2] is copied  # Circular preserved

    def test_deepcopy_circular_dict(self):
        """Feature: Deep copy handles circular dict."""
        d = {"a": 1}
        d["self"] = d  # Circular reference

        copied = copy.deepcopy(d)
        assert copied["a"] == 1
        assert copied["self"] is copied  # Circular preserved

    def test_circular_object_graph(self):
        """Feature: Handle circular object graphs."""
        class Node:
            def __init__(self, value):
                self.value = value
                self.next = None

        n1 = Node(1)
        n2 = Node(2)
        n1.next = n2
        n2.next = n1  # Circular

        copied = copy.deepcopy(n1)
        assert copied.value == 1
        assert copied.next.value == 2
        assert copied.next.next is copied  # Circular preserved


class TestCopyProtocol:
    """Custom copy behavior via __copy__ and __deepcopy__."""

    def test_custom_copy(self):
        """Feature: Custom __copy__ method."""
        class CustomCopy:
            def __init__(self, value):
                self.value = value

            def __copy__(self):
                # Custom copy behavior
                return CustomCopy(self.value * 2)

        obj = CustomCopy(5)
        copied = copy.copy(obj)
        assert copied.value == 10  # Custom behavior

    def test_custom_deepcopy(self):
        """Feature: Custom __deepcopy__ method."""
        class CustomDeepCopy:
            def __init__(self, value):
                self.value = value

            def __deepcopy__(self, memo):
                # Custom deep copy behavior
                return CustomDeepCopy(self.value + 100)

        obj = CustomDeepCopy(5)
        copied = copy.deepcopy(obj)
        assert copied.value == 105  # Custom behavior

    def test_deepcopy_memo(self):
        """Feature: __deepcopy__ receives memo dict."""
        class MemoAware:
            def __init__(self, value):
                self.value = value

            def __deepcopy__(self, memo):
                # memo tracks already-copied objects
                assert isinstance(memo, dict)
                new_obj = MemoAware(self.value)
                memo[id(self)] = new_obj
                return new_obj

        obj = MemoAware(42)
        copied = copy.deepcopy(obj)
        assert copied.value == 42


class TestCopyModule:
    """Module-level utilities."""

    def test_copy_module_exists(self):
        """Basic: Copy module has expected functions."""
        assert hasattr(copy, 'copy')
        assert hasattr(copy, 'deepcopy')
        assert callable(copy.copy)
        assert callable(copy.deepcopy)


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_copy_empty_list(self):
        """Edge: Copy empty list."""
        original = []
        copied = copy.copy(original)
        assert copied == []
        assert copied is not original

    def test_copy_empty_dict(self):
        """Edge: Copy empty dict."""
        original = {}
        copied = copy.copy(original)
        assert copied == {}
        assert copied is not original

    def test_copy_none(self):
        """Edge: Copy None."""
        copied = copy.copy(None)
        assert copied is None

    def test_deepcopy_none(self):
        """Edge: Deep copy None."""
        copied = copy.deepcopy(None)
        assert copied is None

    def test_copy_boolean(self):
        """Edge: Copy boolean."""
        copied_true = copy.copy(True)
        copied_false = copy.copy(False)
        assert copied_true is True
        assert copied_false is False

    def test_large_nested_structure(self):
        """Performance: Deep copy large nested structure."""
        # Create deeply nested list
        data = [1, 2, 3]
        for _ in range(10):
            data = [data, data]

        copied = copy.deepcopy(data)
        assert copied is not data

    def test_copy_set(self):
        """Feature: Copy set."""
        original = {1, 2, 3}
        copied = copy.copy(original)
        assert copied == original
        assert copied is not original

        copied.add(4)
        assert 4 not in original

    def test_deepcopy_set_with_lists(self):
        """Property: Can't have mutable items in sets."""
        # Sets can't contain lists (unhashable)
        original = {1, 2, 3}
        copied = copy.deepcopy(original)
        assert copied == original

    def test_copy_frozenset(self):
        """Feature: Copy frozenset."""
        original = frozenset([1, 2, 3])
        copied = copy.copy(original)
        # Immutable, so may be same object
        assert copied == original


class TestDictCopy:
    """Special dict copying behavior."""

    def test_dict_copy_method(self):
        """Feature: dict.copy() is shallow."""
        original = {"a": [1, 2], "b": 3}
        copied = original.copy()  # Built-in dict method

        # Top-level independence
        copied["c"] = 4
        assert "c" not in original

        # Nested sharing
        copied["a"].append(3)
        assert original["a"] == [1, 2, 3]  # Shared!

    def test_dict_copy_vs_copy_copy(self):
        """Property: dict.copy() equivalent to copy.copy()."""
        original = {"a": 1, "b": [2, 3]}
        method_copy = original.copy()
        module_copy = copy.copy(original)

        # Both are shallow copies
        assert method_copy == module_copy
        assert method_copy is not original
        assert module_copy is not original


class TestListCopy:
    """Special list copying behavior."""

    def test_list_copy_method(self):
        """Feature: list.copy() is shallow."""
        original = [[1, 2], 3]
        copied = original.copy()  # Built-in list method

        # Top-level independence
        copied.append(4)
        assert 4 not in original

        # Nested sharing
        copied[0].append(3)
        assert original[0] == [1, 2, 3]  # Shared!

    def test_list_copy_vs_copy_copy(self):
        """Property: list.copy() equivalent to copy.copy()."""
        original = [1, [2, 3]]
        method_copy = original.copy()
        module_copy = copy.copy(original)

        assert method_copy == module_copy
        assert method_copy is not original

    def test_list_slice_copy(self):
        """Property: List slicing creates shallow copy."""
        original = [1, [2, 3], 4]
        copied = original[:]  # Slice copy

        # Top-level independence
        copied.append(5)
        assert 5 not in original

        # Nested sharing
        copied[1].append(4)
        assert original[1] == [2, 3, 4]  # Shared!


class TestCopyVsAssignment:
    """Copy vs simple assignment."""

    def test_assignment_shares_reference(self):
        """Property: Assignment creates alias, not copy."""
        original = [1, 2, 3]
        alias = original  # Not a copy!

        alias.append(4)
        assert original == [1, 2, 3, 4]  # Same object!
        assert alias is original

    def test_copy_creates_new_object(self):
        """Property: Copy creates independent object."""
        original = [1, 2, 3]
        copied = copy.copy(original)

        copied.append(4)
        assert original == [1, 2, 3]  # Different objects!
        assert copied is not original


class TestMixedTypes:
    """Copying mixed data structures."""

    def test_list_of_dicts(self):
        """Feature: Copy list of dicts."""
        original = [{"a": 1}, {"b": 2}]
        shallow = copy.copy(original)
        deep = copy.deepcopy(original)

        # Shallow copy shares dicts
        shallow[0]["a"] = 99
        assert original[0]["a"] == 99

        # Deep copy doesn't share
        deep[1]["b"] = 88
        assert original[1]["b"] == 2

    def test_dict_of_lists(self):
        """Feature: Copy dict of lists."""
        original = {"nums": [1, 2], "chars": ["a", "b"]}
        shallow = copy.copy(original)
        deep = copy.deepcopy(original)

        # Shallow shares lists
        shallow["nums"].append(3)
        assert original["nums"] == [1, 2, 3]

        # Deep doesn't share
        deep["chars"].append("c")
        assert original["chars"] == ["a", "b"]

    def test_nested_mixed_structures(self):
        """Feature: Deep copy complex nested structures."""
        original = {
            "list": [1, [2, 3]],
            "dict": {"nested": {"deep": [4, 5]}},
            "tuple": (6, [7, 8])
        }
        copied = copy.deepcopy(original)

        # Modify deep nested list
        copied["dict"]["nested"]["deep"].append(99)
        assert "99" not in str(original)  # Original unchanged


class TestModuleAttributes:
    """Module attributes and error handling."""

    def test_copy_error_uncopyable(self):
        """Error: Some objects may not be copyable."""
        # Some types like modules, functions can't be deep copied easily
        import sys

        # Modules typically can't be deep copied
        with pytest.raises((TypeError, AttributeError)):
            copy.deepcopy(sys)

    def test_copy_function(self):
        """Edge: Copy function object."""
        def func():
            return 42

        # Functions are typically same object
        copied = copy.copy(func)
        assert copied is func

    def test_copy_lambda(self):
        """Edge: Copy lambda."""
        lam = lambda x: x * 2
        copied = copy.copy(lam)
        # Lambdas are typically same object
        assert copied(5) == 10


class TestReplaceMethod:
    """Using copy.replace() for immutable objects (Python 3.13+)."""

    def test_replace_not_in_older_python(self):
        """Note: copy.replace() added in Python 3.13."""
        # This test documents that replace may not exist
        # In Python < 3.13
        has_replace = hasattr(copy, 'replace')
        # Just documenting - test passes either way
        assert isinstance(has_replace, bool)
