"""
Comprehensive test of Python collections module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's collections module
types to their Rust equivalents.

Expected Rust mappings:
- deque -> VecDeque
- Counter -> HashMap<T, usize>
- defaultdict -> HashMap with default values
- OrderedDict -> LinkedHashMap or IndexMap
- namedtuple -> struct

Note: Some collection types may have limited support.
"""

from collections import deque, Counter, defaultdict, OrderedDict
from typing import List, Dict, Tuple


def test_deque_basic() -> List[int]:
    """Test basic deque operations"""
    # Create deque
    d: deque = deque([1, 2, 3])

    # Append to right
    d.append(4)

    # Append to left
    d.appendleft(0)

    # Convert to list for return
    result: List[int] = list(d)

    return result


def test_deque_pop() -> Tuple[int, int]:
    """Test deque pop operations"""
    d: deque = deque([1, 2, 3, 4, 5])

    # Pop from right
    right: int = d.pop()

    # Pop from left
    left: int = d.popleft()

    return (left, right)


def test_deque_extend() -> List[int]:
    """Test deque extend operations"""
    d: deque = deque([1, 2, 3])

    # Extend right
    d.extend([4, 5])

    # Extend left
    d.extendleft([0, -1])

    result: List[int] = list(d)

    return result


def test_deque_rotate() -> List[int]:
    """Test deque rotation"""
    d: deque = deque([1, 2, 3, 4, 5])

    # Rotate right by 2
    # Manual implementation since rotate may not be supported
    for i in range(2):
        item: int = d.pop()
        d.appendleft(item)

    result: List[int] = list(d)

    return result


def test_counter_basic() -> Dict[str, int]:
    """Test Counter basic functionality"""
    # Count elements in list (manual implementation)
    items: List[str] = ["apple", "banana", "apple", "cherry", "banana", "apple"]

    counts: Dict[str, int] = {}
    for item in items:
        if item in counts:
            counts[item] = counts[item] + 1
        else:
            counts[item] = 1

    return counts


def test_counter_most_common(items: List[str], n: int) -> List[Tuple[str, int]]:
    """Test getting most common elements"""
    # Manual Counter implementation
    counts: Dict[str, int] = {}
    for item in items:
        if item in counts:
            counts[item] = counts[item] + 1
        else:
            counts[item] = 1

    # Convert to list of tuples
    count_list: List[Tuple[str, int]] = []
    for key in counts.keys():
        pair: Tuple[str, int] = (key, counts[key])
        count_list.append(pair)

    # Sort by count (manual bubble sort for simplicity)
    for i in range(len(count_list)):
        for j in range(i + 1, len(count_list)):
            if count_list[j][1] > count_list[i][1]:
                temp: Tuple[str, int] = count_list[i]
                count_list[i] = count_list[j]
                count_list[j] = temp

    # Return top n
    result: List[Tuple[str, int]] = count_list[:n]

    return result


def test_counter_arithmetic() -> Dict[str, int]:
    """Test Counter arithmetic operations"""
    # Manual implementation of counter addition
    counter1: Dict[str, int] = {"a": 3, "b": 1}
    counter2: Dict[str, int] = {"a": 1, "b": 2, "c": 1}

    # Add counters (merge counts)
    result: Dict[str, int] = {}

    # Add from counter1
    for key in counter1.keys():
        result[key] = counter1[key]

    # Add from counter2
    for key in counter2.keys():
        if key in result:
            result[key] = result[key] + counter2[key]
        else:
            result[key] = counter2[key]

    return result


def test_defaultdict_int() -> Dict[str, int]:
    """Test defaultdict with int default"""
    # Manual implementation of defaultdict behavior
    counts: Dict[str, int] = {}

    words: List[str] = ["hello", "world", "hello", "python", "world", "hello"]

    for word in words:
        # Get with default 0
        current: int = counts.get(word, 0)
        counts[word] = current + 1

    return counts


def test_defaultdict_list() -> Dict[str, List[int]]:
    """Test defaultdict with list default"""
    # Manual implementation
    groups: Dict[str, List[int]] = {}

    pairs: List[Tuple[str, int]] = [("a", 1), ("b", 2), ("a", 3), ("b", 4), ("a", 5)]

    for pair in pairs:
        key: str = pair[0]
        value: int = pair[1]

        # Get or create list
        if key not in groups:
            groups[key] = []

        groups[key].append(value)

    return groups


def test_ordereddict_basic() -> List[Tuple[str, int]]:
    """Test OrderedDict basic operations"""
    # Regular dict maintains insertion order in Python 3.7+
    od: Dict[str, int] = {}

    od["first"] = 1
    od["second"] = 2
    od["third"] = 3

    # Convert to list of tuples maintaining order
    result: List[Tuple[str, int]] = []
    for key in od.keys():
        pair: Tuple[str, int] = (key, od[key])
        result.append(pair)

    return result


def test_ordereddict_move_to_end() -> List[str]:
    """Test moving item to end in OrderedDict"""
    # Manual implementation
    od: Dict[str, int] = {"a": 1, "b": 2, "c": 3}

    # To move 'a' to end, remove and re-add
    value: int = od.pop("a")
    od["a"] = value

    # Get keys in order
    keys: List[str] = list(od.keys())

    return keys


def test_chainmap(dict1: Dict[str, int], dict2: Dict[str, int]) -> int:
    """Test ChainMap-like lookup (manual)"""
    # Look up key in first dict, then second
    key: str = "x"

    if key in dict1:
        return dict1[key]
    elif key in dict2:
        return dict2[key]
    else:
        return -1


def word_frequency_counter(text: str) -> Dict[str, int]:
    """Count word frequencies using Counter concept"""
    # Split text into words (simplified)
    words: List[str] = text.split()

    # Count frequencies
    freq: Dict[str, int] = {}
    for word in words:
        if word in freq:
            freq[word] = freq[word] + 1
        else:
            freq[word] = 1

    return freq


def group_by_first_letter(words: List[str]) -> Dict[str, List[str]]:
    """Group words by first letter using defaultdict concept"""
    groups: Dict[str, List[str]] = {}

    for word in words:
        if len(word) == 0:
            continue

        first_letter: str = word[0]

        if first_letter not in groups:
            groups[first_letter] = []

        groups[first_letter].append(word)

    return groups


def test_deque_as_stack() -> List[int]:
    """Use deque as a stack (LIFO)"""
    stack: deque = deque()

    # Push items
    stack.append(1)
    stack.append(2)
    stack.append(3)

    result: List[int] = []

    # Pop items (LIFO)
    while len(stack) > 0:
        item: int = stack.pop()
        result.append(item)

    return result


def test_deque_as_queue() -> List[int]:
    """Use deque as a queue (FIFO)"""
    queue: deque = deque()

    # Enqueue items
    queue.append(1)
    queue.append(2)
    queue.append(3)

    result: List[int] = []

    # Dequeue items (FIFO)
    while len(queue) > 0:
        item: int = queue.popleft()
        result.append(item)

    return result


def test_lru_cache_manual(cache_size: int) -> List[int]:
    """Manual implementation of LRU cache concept using deque"""
    cache: deque = deque()
    max_size: int = cache_size

    items: List[int] = [1, 2, 3, 1, 4, 2, 5, 1, 6]
    result: List[int] = []

    for item in items:
        # Check if item in cache (linear search)
        found: bool = False
        for cached in cache:
            if cached == item:
                found = True
                break

        if not found:
            # Add to cache
            cache.append(item)

            # Evict oldest if full
            if len(cache) > max_size:
                evicted: int = cache.popleft()

        result.append(item)

    return result


def test_all_collections_features() -> None:
    """Run all collections module tests"""
    # Deque tests
    deque_basic: List[int] = test_deque_basic()
    deque_pops: Tuple[int, int] = test_deque_pop()
    deque_extended: List[int] = test_deque_extend()
    deque_rotated: List[int] = test_deque_rotate()

    # Counter tests
    counts: Dict[str, int] = test_counter_basic()

    items: List[str] = ["a", "b", "a", "c", "a", "b", "d", "a"]
    most_common: List[Tuple[str, int]] = test_counter_most_common(items, 2)

    merged: Dict[str, int] = test_counter_arithmetic()

    # Defaultdict tests
    int_default: Dict[str, int] = test_defaultdict_int()
    list_default: Dict[str, List[int]] = test_defaultdict_list()

    # OrderedDict tests
    ordered: List[Tuple[str, int]] = test_ordereddict_basic()
    moved: List[str] = test_ordereddict_move_to_end()

    # ChainMap test
    d1: Dict[str, int] = {"x": 1, "y": 2}
    d2: Dict[str, int] = {"y": 3, "z": 4}
    chain_result: int = test_chainmap(d1, d2)

    # Utility tests
    text: str = "hello world hello python world"
    freq: Dict[str, int] = word_frequency_counter(text)

    words: List[str] = ["apple", "banana", "apricot", "blueberry", "cherry"]
    grouped: Dict[str, List[str]] = group_by_first_letter(words)

    # Deque as data structures
    stack_result: List[int] = test_deque_as_stack()
    queue_result: List[int] = test_deque_as_queue()

    # LRU cache concept
    lru_result: List[int] = test_lru_cache_manual(3)

    print("All collections module tests completed successfully")
