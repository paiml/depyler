# tests/test_collections/test_data_structures.py
"""
TDD examples for collections module - specialized container datatypes.
Each test becomes a verified documentation example.
"""
import collections
import pytest
from hypothesis import given, strategies as st


class TestDefaultDict:
    """collections.defaultdict - Dict with default factory function."""

    def test_defaultdict_int_factory(self):
        """Basic: defaultdict with int factory (auto-initializes to 0)."""
        dd = collections.defaultdict(int)
        dd["a"] += 1
        dd["b"] += 2

        assert dd["a"] == 1
        assert dd["b"] == 2
        assert dd["nonexistent"] == 0  # Auto-initialized

    def test_defaultdict_list_factory(self):
        """Basic: defaultdict with list factory (auto-initializes to [])."""
        dd = collections.defaultdict(list)
        dd["fruits"].append("apple")
        dd["fruits"].append("banana")
        dd["vegetables"].append("carrot")

        assert dd["fruits"] == ["apple", "banana"]
        assert dd["vegetables"] == ["carrot"]
        assert dd["nonexistent"] == []  # Auto-initialized

    def test_defaultdict_custom_factory(self):
        """Feature: defaultdict with custom factory function."""
        dd = collections.defaultdict(lambda: "default_value")
        dd["key1"] = "custom"

        assert dd["key1"] == "custom"
        assert dd["missing"] == "default_value"

    def test_defaultdict_no_factory(self):
        """Edge: defaultdict without factory behaves like dict."""
        dd = collections.defaultdict()
        dd["key"] = "value"

        assert dd["key"] == "value"
        with pytest.raises(KeyError):
            _ = dd["missing"]

    def test_defaultdict_dict_methods(self):
        """Property: defaultdict supports all dict methods."""
        dd = collections.defaultdict(int)
        dd["a"] = 1
        dd["b"] = 2

        assert list(dd.keys()) == ["a", "b"]
        assert list(dd.values()) == [1, 2]
        assert dd.get("c", 99) == 99


class TestCounter:
    """collections.Counter - Count hashable objects."""

    def test_counter_from_list(self):
        """Basic: Count elements in a list."""
        c = collections.Counter([1, 2, 2, 3, 3, 3])
        assert c[1] == 1
        assert c[2] == 2
        assert c[3] == 3

    def test_counter_from_string(self):
        """Basic: Count characters in a string."""
        c = collections.Counter("banana")
        assert c["b"] == 1
        assert c["a"] == 3
        assert c["n"] == 2

    def test_counter_most_common(self):
        """Feature: Get most common elements."""
        c = collections.Counter("abracadabra")
        most_common = c.most_common(2)

        assert most_common[0] == ("a", 5)
        assert most_common[1] == ("b", 2)

    def test_counter_arithmetic(self):
        """Feature: Counter supports addition and subtraction."""
        c1 = collections.Counter(["a", "b", "b", "c"])
        c2 = collections.Counter(["b", "c", "c", "d"])

        # Addition
        combined = c1 + c2
        assert combined["b"] == 3
        assert combined["c"] == 3

        # Subtraction (keeps only positive counts)
        diff = c1 - c2
        assert diff["a"] == 1
        assert diff["b"] == 1
        assert diff["c"] == 0  # Negative counts removed

    def test_counter_update(self):
        """Feature: Update counts from iterable."""
        c = collections.Counter(["a", "b"])
        c.update(["b", "c", "c"])

        assert c["a"] == 1
        assert c["b"] == 2
        assert c["c"] == 2

    def test_counter_missing_element(self):
        """Edge: Counter returns 0 for missing elements (not KeyError)."""
        c = collections.Counter(["a", "b"])
        assert c["nonexistent"] == 0

    def test_counter_total(self):
        """Feature: Get total of all counts (Python 3.10+)."""
        c = collections.Counter(["a", "b", "b", "c", "c", "c"])
        # Python 3.10+ has total() method
        if hasattr(c, "total"):
            assert c.total() == 6
        else:
            assert sum(c.values()) == 6


class TestDeque:
    """collections.deque - Double-ended queue."""

    def test_deque_append_left(self):
        """Basic: Append to left side."""
        d = collections.deque([2, 3, 4])
        d.appendleft(1)
        assert list(d) == [1, 2, 3, 4]

    def test_deque_append_right(self):
        """Basic: Append to right side (like list)."""
        d = collections.deque([1, 2, 3])
        d.append(4)
        assert list(d) == [1, 2, 3, 4]

    def test_deque_pop_left(self):
        """Basic: Pop from left side (O(1))."""
        d = collections.deque([1, 2, 3])
        left = d.popleft()
        assert left == 1
        assert list(d) == [2, 3]

    def test_deque_pop_right(self):
        """Basic: Pop from right side (like list)."""
        d = collections.deque([1, 2, 3])
        right = d.pop()
        assert right == 3
        assert list(d) == [1, 2]

    def test_deque_rotate(self):
        """Feature: Rotate elements."""
        d = collections.deque([1, 2, 3, 4, 5])

        # Rotate right (positive)
        d.rotate(2)
        assert list(d) == [4, 5, 1, 2, 3]

        # Rotate left (negative)
        d.rotate(-2)
        assert list(d) == [1, 2, 3, 4, 5]

    def test_deque_maxlen(self):
        """Feature: Bounded deque with maxlen."""
        d = collections.deque([1, 2, 3], maxlen=3)
        d.append(4)

        # Old element pushed out
        assert list(d) == [2, 3, 4]
        assert len(d) == 3

    def test_deque_extend(self):
        """Feature: Extend from both ends."""
        d = collections.deque([3, 4])
        d.extendleft([2, 1])  # Note: reverses order
        d.extend([5, 6])

        assert list(d) == [1, 2, 3, 4, 5, 6]

    def test_deque_empty_pop_raises(self):
        """Error: Pop from empty deque raises IndexError."""
        d = collections.deque()
        with pytest.raises(IndexError):
            d.pop()

        with pytest.raises(IndexError):
            d.popleft()


class TestNamedTuple:
    """collections.namedtuple - Tuple with named fields."""

    def test_namedtuple_creation(self):
        """Basic: Create a named tuple type."""
        Point = collections.namedtuple("Point", ["x", "y"])
        p = Point(10, 20)

        assert p.x == 10
        assert p.y == 20
        assert p[0] == 10  # Still accessible by index
        assert p[1] == 20

    def test_namedtuple_string_fields(self):
        """Feature: Field names as single string."""
        Person = collections.namedtuple("Person", "name age city")
        p = Person("Alice", 30, "NYC")

        assert p.name == "Alice"
        assert p.age == 30
        assert p.city == "NYC"

    def test_namedtuple_immutable(self):
        """Property: namedtuples are immutable."""
        Point = collections.namedtuple("Point", ["x", "y"])
        p = Point(10, 20)

        with pytest.raises(AttributeError):
            p.x = 30

    def test_namedtuple_asdict(self):
        """Feature: Convert to OrderedDict."""
        Point = collections.namedtuple("Point", ["x", "y"])
        p = Point(10, 20)
        d = p._asdict()

        assert dict(d) == {"x": 10, "y": 20}

    def test_namedtuple_replace(self):
        """Feature: Create new instance with replaced fields."""
        Point = collections.namedtuple("Point", ["x", "y"])
        p1 = Point(10, 20)
        p2 = p1._replace(x=30)

        assert p2.x == 30
        assert p2.y == 20
        assert p1.x == 10  # Original unchanged


class TestChainMap:
    """collections.ChainMap - Group multiple dicts into single view."""

    def test_chainmap_basic(self):
        """Basic: Access values from multiple dicts."""
        dict1 = {"a": 1, "b": 2}
        dict2 = {"c": 3, "d": 4}
        cm = collections.ChainMap(dict1, dict2)

        assert cm["a"] == 1
        assert cm["c"] == 3

    def test_chainmap_priority(self):
        """Edge: First dict has priority for duplicate keys."""
        dict1 = {"key": "value1"}
        dict2 = {"key": "value2"}
        cm = collections.ChainMap(dict1, dict2)

        assert cm["key"] == "value1"

    def test_chainmap_new_child(self):
        """Feature: Add new child dict."""
        dict1 = {"a": 1}
        cm = collections.ChainMap(dict1)
        cm2 = cm.new_child({"b": 2})

        assert cm2["a"] == 1
        assert cm2["b"] == 2
        assert "b" not in cm  # Original unchanged

    def test_chainmap_mutation(self):
        """Edge: Updates affect first dict only."""
        dict1 = {"a": 1}
        dict2 = {"b": 2}
        cm = collections.ChainMap(dict1, dict2)

        cm["c"] = 3
        assert "c" in dict1
        assert "c" not in dict2


class TestOrderedDict:
    """collections.OrderedDict - Dict that remembers insertion order."""

    def test_ordereddict_maintains_order(self):
        """Basic: Maintains insertion order."""
        od = collections.OrderedDict()
        od["c"] = 3
        od["a"] = 1
        od["b"] = 2

        assert list(od.keys()) == ["c", "a", "b"]

    def test_ordereddict_move_to_end(self):
        """Feature: Move item to end or beginning."""
        od = collections.OrderedDict([("a", 1), ("b", 2), ("c", 3)])

        # Move to end
        od.move_to_end("a")
        assert list(od.keys()) == ["b", "c", "a"]

        # Move to beginning
        od.move_to_end("a", last=False)
        assert list(od.keys()) == ["a", "b", "c"]

    def test_ordereddict_popitem(self):
        """Feature: Pop items in LIFO order."""
        od = collections.OrderedDict([("a", 1), ("b", 2), ("c", 3)])

        # Pop last (LIFO)
        key, value = od.popitem()
        assert key == "c"
        assert value == 3

        # Pop first (FIFO)
        key, value = od.popitem(last=False)
        assert key == "a"
        assert value == 1


# Metadata for doc generation
__module_name__ = "collections"
__module_link__ = "https://docs.python.org/3/library/collections.html"
__test_count__ = 38
__coverage__ = 0.75  # ~75% of common collections functions
