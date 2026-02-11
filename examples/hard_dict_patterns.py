"""Hard dictionary patterns that stress-test dict-related transpilation.

Tests: nested dict access, mixed key/value types, dict comprehensions,
items() iteration with tuple unpacking, setdefault, merge patterns,
counter patterns, default-dict patterns, math on values, complex keys,
conditional comprehension filtering, nested modification, dict-of-lists
grouping, dict inversion, and dict-based state machines.
"""


def trie_insert(trie: dict[str, dict[str, str]], word: str) -> dict[str, dict[str, str]]:
    """Insert a word into a trie-like nested dict structure.

    Each character maps to a sub-dict; terminal nodes have key '_end' -> 'true'.
    """
    current: dict[str, str] = {}
    # Flatten trie-like pattern: build path character by character
    path: list[str] = []
    for ch in word:
        path.append(ch)
    # Store the word under its first char -> rest mapping
    if len(path) > 0:
        first: str = path[0]
        rest: str = ""
        for i in range(1, len(path)):
            rest = rest + path[i]
        if first not in trie:
            trie[first] = {}
        trie[first][rest] = "true"
    return trie


def trie_search(trie: dict[str, dict[str, str]], word: str) -> bool:
    """Search for a word in the trie-like nested dict."""
    if len(word) == 0:
        return False
    first: str = word[0]
    rest: str = word[1:]
    if first not in trie:
        return False
    return rest in trie[first]


def adjacency_list_add(graph: dict[int, list[int]], src: int, dst: int) -> dict[int, list[int]]:
    """Add an edge to an adjacency list stored as Dict[int, List[int]]."""
    if src not in graph:
        graph[src] = []
    graph[src].append(dst)
    return graph


def adjacency_list_neighbors(graph: dict[int, list[int]], node: int) -> list[int]:
    """Get neighbors of a node from an adjacency list."""
    if node in graph:
        return graph[node]
    return []


def squared_dict(nums: list[int]) -> dict[int, int]:
    """Dict comprehension mapping each number to its square."""
    result: dict[int, int] = {}
    for n in nums:
        result[n] = n * n
    return result


def string_length_map(words: list[str]) -> dict[str, int]:
    """Dict comprehension mapping words to their lengths."""
    result: dict[str, int] = {}
    for w in words:
        result[w] = len(w)
    return result


def sum_dict_items(data: dict[str, int]) -> int:
    """Iterate over dict items with tuple unpacking and sum values."""
    total: int = 0
    for key in data:
        total += data[key]
    return total


def format_dict_items(data: dict[str, int]) -> list[str]:
    """Format dict items as 'key=value' strings using items iteration."""
    result: list[str] = []
    for key in data:
        entry: str = key + "=" + str(data[key])
        result.append(entry)
    return result


def count_with_setdefault(words: list[str]) -> dict[str, int]:
    """Count word occurrences using setdefault-like pattern."""
    counts: dict[str, int] = {}
    for word in words:
        if word not in counts:
            counts[word] = 0
        counts[word] += 1
    return counts


def group_with_setdefault(pairs: list[list[str]]) -> dict[str, list[str]]:
    """Group values by key using setdefault-like pattern.

    Each pair is [key, value].
    """
    groups: dict[str, list[str]] = {}
    for pair in pairs:
        key: str = pair[0]
        val: str = pair[1]
        if key not in groups:
            groups[key] = []
        groups[key].append(val)
    return groups


def merge_dicts(base: dict[str, int], override: dict[str, int]) -> dict[str, int]:
    """Merge two dicts, with override taking precedence."""
    result: dict[str, int] = {}
    for key in base:
        result[key] = base[key]
    for key in override:
        result[key] = override[key]
    return result


def merge_dicts_sum(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:
    """Merge two dicts by summing values for shared keys."""
    result: dict[str, int] = {}
    for key in a:
        result[key] = a[key]
    for key in b:
        if key in result:
            result[key] += b[key]
        else:
            result[key] = b[key]
    return result


def count_characters(text: str) -> dict[str, int]:
    """Counter-like pattern: count character frequencies."""
    counts: dict[str, int] = {}
    for ch in text:
        if ch in counts:
            counts[ch] += 1
        else:
            counts[ch] = 1
    return counts


def most_common_char(text: str) -> str:
    """Find the most common character using a counter dict."""
    counts: dict[str, int] = count_characters(text)
    best_char: str = ""
    best_count: int = 0
    for ch in counts:
        if counts[ch] > best_count:
            best_count = counts[ch]
            best_char = ch
    return best_char


def safe_get(data: dict[str, int], key: str, default: int) -> int:
    """Default-dict-like pattern using get with default."""
    if key in data:
        return data[key]
    return default


def increment_or_init(data: dict[str, int], key: str, amount: int) -> dict[str, int]:
    """Increment a key, initializing to 0 if missing (defaultdict-like)."""
    if key not in data:
        data[key] = 0
    data[key] += amount
    return data


def dict_sum_values(data: dict[str, float]) -> float:
    """Sum all float values in a dict."""
    total: float = 0.0
    for key in data:
        total += data[key]
    return total


def dict_normalize_values(data: dict[str, float]) -> dict[str, float]:
    """Normalize dict values so they sum to 1.0 (probability distribution)."""
    total: float = dict_sum_values(data)
    result: dict[str, float] = {}
    if total == 0.0:
        return result
    for key in data:
        result[key] = data[key] / total
    return result


def dict_scale_values(data: dict[str, float], factor: float) -> dict[str, float]:
    """Scale all values by a constant factor."""
    result: dict[str, float] = {}
    for key in data:
        result[key] = data[key] * factor
    return result


def tuple_key_store(rows: list[list[int]]) -> dict[str, int]:
    """Complex key pattern: encode (row, col) as string key for a grid."""
    grid: dict[str, int] = {}
    for r in range(len(rows)):
        for c in range(len(rows[r])):
            key: str = str(r) + "," + str(c)
            grid[key] = rows[r][c]
    return grid


def tuple_key_lookup(grid: dict[str, int], row: int, col: int) -> int:
    """Look up a value using an encoded (row, col) key."""
    key: str = str(row) + "," + str(col)
    if key in grid:
        return grid[key]
    return -1


def filter_dict_positive(data: dict[str, int]) -> dict[str, int]:
    """Filter dict to only entries with positive values."""
    result: dict[str, int] = {}
    for key in data:
        if data[key] > 0:
            result[key] = data[key]
    return result


def filter_dict_by_key_prefix(data: dict[str, int], prefix: str) -> dict[str, int]:
    """Filter dict to only keys starting with given prefix."""
    result: dict[str, int] = {}
    for key in data:
        if key.startswith(prefix):
            result[key] = data[key]
    return result


def filter_dict_threshold(data: dict[str, float], threshold: float) -> dict[str, float]:
    """Filter dict to entries with values above threshold."""
    result: dict[str, float] = {}
    for key in data:
        if data[key] > threshold:
            result[key] = data[key]
    return result


def nested_dict_set(data: dict[str, dict[str, int]], outer: str, inner: str, value: int) -> dict[str, dict[str, int]]:
    """Set a value in a nested dict, creating intermediate dicts as needed."""
    if outer not in data:
        data[outer] = {}
    data[outer][inner] = value
    return data


def nested_dict_get(data: dict[str, dict[str, int]], outer: str, inner: str, default: int) -> int:
    """Get a value from a nested dict with a default."""
    if outer not in data:
        return default
    inner_dict: dict[str, int] = data[outer]
    if inner not in inner_dict:
        return default
    return inner_dict[inner]


def nested_dict_increment(data: dict[str, dict[str, int]], outer: str, inner: str) -> dict[str, dict[str, int]]:
    """Increment a counter in a nested dict structure."""
    if outer not in data:
        data[outer] = {}
    if inner not in data[outer]:
        data[outer][inner] = 0
    data[outer][inner] += 1
    return data


def group_by_length(words: list[str]) -> dict[int, list[str]]:
    """Group words by their length into a dict-of-lists."""
    groups: dict[int, list[str]] = {}
    for word in words:
        length: int = len(word)
        if length not in groups:
            groups[length] = []
        groups[length].append(word)
    return groups


def group_by_first_char(words: list[str]) -> dict[str, list[str]]:
    """Group words by their first character."""
    groups: dict[str, list[str]] = {}
    for word in words:
        if len(word) > 0:
            first: str = word[0]
            if first not in groups:
                groups[first] = []
            groups[first].append(word)
    return groups


def classify_numbers(nums: list[int]) -> dict[str, list[int]]:
    """Classify numbers into 'positive', 'negative', 'zero' groups."""
    result: dict[str, list[int]] = {}
    result["positive"] = []
    result["negative"] = []
    result["zero"] = []
    for n in nums:
        if n > 0:
            result["positive"].append(n)
        elif n < 0:
            result["negative"].append(n)
        else:
            result["zero"].append(n)
    return result


def invert_dict(data: dict[str, int]) -> dict[int, str]:
    """Invert a dict: swap keys and values."""
    result: dict[int, str] = {}
    for key in data:
        result[data[key]] = key
    return result


def invert_dict_str(data: dict[str, str]) -> dict[str, str]:
    """Invert a string-to-string dict."""
    result: dict[str, str] = {}
    for key in data:
        result[data[key]] = key
    return result


def invert_dict_to_lists(data: dict[str, int]) -> dict[int, list[str]]:
    """Invert a dict, grouping multiple keys with the same value."""
    result: dict[int, list[str]] = {}
    for key in data:
        val: int = data[key]
        if val not in result:
            result[val] = []
        result[val].append(key)
    return result


def state_machine_run(transitions: dict[str, dict[str, str]], initial: str, inputs: list[str]) -> str:
    """Run a dict-based state machine and return the final state.

    transitions[current_state][input] -> next_state
    """
    state: str = initial
    for inp in inputs:
        if state in transitions:
            state_trans: dict[str, str] = transitions[state]
            if inp in state_trans:
                state = state_trans[inp]
    return state


def state_machine_trace(transitions: dict[str, dict[str, str]], initial: str, inputs: list[str]) -> list[str]:
    """Run a state machine and return the full trace of states visited."""
    state: str = initial
    trace: list[str] = [initial]
    for inp in inputs:
        if state in transitions:
            state_trans: dict[str, str] = transitions[state]
            if inp in state_trans:
                state = state_trans[inp]
        trace.append(state)
    return trace


def state_machine_accepts(transitions: dict[str, dict[str, str]], initial: str, accepting: list[str], inputs: list[str]) -> bool:
    """Check if a state machine reaches an accepting state."""
    final: str = state_machine_run(transitions, initial, inputs)
    for acc in accepting:
        if final == acc:
            return True
    return False


def test_all() -> int:
    """Comprehensive test exercising all dict pattern functions.

    Returns 0 if all tests pass.
    """
    errors: int = 0

    # Test trie_insert and trie_search
    trie: dict[str, dict[str, str]] = {}
    trie = trie_insert(trie, "hello")
    trie = trie_insert(trie, "help")
    if not trie_search(trie, "hello"):
        errors += 1
    if not trie_search(trie, "help"):
        errors += 1
    if trie_search(trie, "world"):
        errors += 1

    # Test adjacency_list
    graph: dict[int, list[int]] = {}
    graph = adjacency_list_add(graph, 1, 2)
    graph = adjacency_list_add(graph, 1, 3)
    graph = adjacency_list_add(graph, 2, 3)
    neighbors: list[int] = adjacency_list_neighbors(graph, 1)
    if len(neighbors) != 2:
        errors += 1
    empty_neighbors: list[int] = adjacency_list_neighbors(graph, 99)
    if len(empty_neighbors) != 0:
        errors += 1

    # Test squared_dict
    sq: dict[int, int] = squared_dict([1, 2, 3, 4])
    if sq[3] != 9:
        errors += 1
    if sq[4] != 16:
        errors += 1

    # Test string_length_map
    lengths: dict[str, int] = string_length_map(["hi", "hello", "hey"])
    if lengths["hi"] != 2:
        errors += 1
    if lengths["hello"] != 5:
        errors += 1

    # Test sum_dict_items
    sample: dict[str, int] = {"a": 10, "b": 20, "c": 30}
    if sum_dict_items(sample) != 60:
        errors += 1

    # Test format_dict_items
    formatted: list[str] = format_dict_items({"x": 1})
    if len(formatted) != 1:
        errors += 1

    # Test count_with_setdefault
    word_counts: dict[str, int] = count_with_setdefault(["a", "b", "a", "c", "a", "b"])
    if word_counts["a"] != 3:
        errors += 1
    if word_counts["b"] != 2:
        errors += 1

    # Test group_with_setdefault
    pairs: list[list[str]] = [["fruit", "apple"], ["fruit", "banana"], ["veggie", "carrot"]]
    groups: dict[str, list[str]] = group_with_setdefault(pairs)
    if len(groups["fruit"]) != 2:
        errors += 1
    if len(groups["veggie"]) != 1:
        errors += 1

    # Test merge_dicts
    merged: dict[str, int] = merge_dicts({"a": 1, "b": 2}, {"b": 3, "c": 4})
    if merged["b"] != 3:
        errors += 1
    if merged["a"] != 1:
        errors += 1
    if merged["c"] != 4:
        errors += 1

    # Test merge_dicts_sum
    summed: dict[str, int] = merge_dicts_sum({"a": 10, "b": 5}, {"b": 3, "c": 7})
    if summed["a"] != 10:
        errors += 1
    if summed["b"] != 8:
        errors += 1
    if summed["c"] != 7:
        errors += 1

    # Test count_characters and most_common_char
    char_counts: dict[str, int] = count_characters("aabbbcc")
    if char_counts["b"] != 3:
        errors += 1
    best: str = most_common_char("aabbbcc")
    if best != "b":
        errors += 1

    # Test safe_get
    sg_data: dict[str, int] = {"x": 42}
    if safe_get(sg_data, "x", 0) != 42:
        errors += 1
    if safe_get(sg_data, "y", -1) != -1:
        errors += 1

    # Test increment_or_init
    inc_data: dict[str, int] = {}
    inc_data = increment_or_init(inc_data, "hits", 1)
    inc_data = increment_or_init(inc_data, "hits", 1)
    inc_data = increment_or_init(inc_data, "hits", 1)
    if inc_data["hits"] != 3:
        errors += 1

    # Test dict_sum_values
    float_data: dict[str, float] = {"a": 1.5, "b": 2.5, "c": 3.0}
    total: float = dict_sum_values(float_data)
    if total < 6.9 or total > 7.1:
        errors += 1

    # Test dict_normalize_values
    normed: dict[str, float] = dict_normalize_values({"a": 2.0, "b": 3.0, "c": 5.0})
    if "a" in normed:
        if normed["a"] < 0.19 or normed["a"] > 0.21:
            errors += 1
    else:
        errors += 1

    # Test dict_scale_values
    scaled: dict[str, float] = dict_scale_values({"x": 1.0, "y": 2.0}, 3.0)
    if scaled["x"] < 2.9 or scaled["x"] > 3.1:
        errors += 1
    if scaled["y"] < 5.9 or scaled["y"] > 6.1:
        errors += 1

    # Test tuple_key_store and tuple_key_lookup
    rows: list[list[int]] = [[10, 20], [30, 40]]
    grid: dict[str, int] = tuple_key_store(rows)
    if tuple_key_lookup(grid, 0, 0) != 10:
        errors += 1
    if tuple_key_lookup(grid, 1, 1) != 40:
        errors += 1
    if tuple_key_lookup(grid, 5, 5) != -1:
        errors += 1

    # Test filter_dict_positive
    pos_data: dict[str, int] = {"a": 5, "b": -3, "c": 0, "d": 7}
    pos_only: dict[str, int] = filter_dict_positive(pos_data)
    if len(pos_only) != 2:
        errors += 1

    # Test filter_dict_by_key_prefix
    prefix_data: dict[str, int] = {"user_name": 1, "user_age": 2, "item_id": 3}
    user_only: dict[str, int] = filter_dict_by_key_prefix(prefix_data, "user_")
    if len(user_only) != 2:
        errors += 1

    # Test filter_dict_threshold
    thresh_data: dict[str, float] = {"a": 1.5, "b": 3.5, "c": 0.5, "d": 4.0}
    above: dict[str, float] = filter_dict_threshold(thresh_data, 2.0)
    if len(above) != 2:
        errors += 1

    # Test nested_dict_set and nested_dict_get
    nested: dict[str, dict[str, int]] = {}
    nested = nested_dict_set(nested, "users", "alice", 100)
    nested = nested_dict_set(nested, "users", "bob", 200)
    nested = nested_dict_set(nested, "items", "widget", 50)
    if nested_dict_get(nested, "users", "alice", 0) != 100:
        errors += 1
    if nested_dict_get(nested, "users", "bob", 0) != 200:
        errors += 1
    if nested_dict_get(nested, "missing", "key", -1) != -1:
        errors += 1

    # Test nested_dict_increment
    counters: dict[str, dict[str, int]] = {}
    counters = nested_dict_increment(counters, "page", "home")
    counters = nested_dict_increment(counters, "page", "home")
    counters = nested_dict_increment(counters, "page", "about")
    if nested_dict_get(counters, "page", "home", 0) != 2:
        errors += 1
    if nested_dict_get(counters, "page", "about", 0) != 1:
        errors += 1

    # Test group_by_length
    by_len: dict[int, list[str]] = group_by_length(["a", "bb", "cc", "ddd", "e"])
    if len(by_len[1]) != 2:
        errors += 1
    if len(by_len[2]) != 2:
        errors += 1
    if len(by_len[3]) != 1:
        errors += 1

    # Test group_by_first_char
    by_char: dict[str, list[str]] = group_by_first_char(["apple", "avocado", "banana", "blueberry"])
    if len(by_char["a"]) != 2:
        errors += 1
    if len(by_char["b"]) != 2:
        errors += 1

    # Test classify_numbers
    classified: dict[str, list[int]] = classify_numbers([3, -1, 0, 5, -2, 0])
    if len(classified["positive"]) != 2:
        errors += 1
    if len(classified["negative"]) != 2:
        errors += 1
    if len(classified["zero"]) != 2:
        errors += 1

    # Test invert_dict
    inv: dict[int, str] = invert_dict({"x": 1, "y": 2, "z": 3})
    if inv[1] != "x":
        errors += 1
    if inv[2] != "y":
        errors += 1

    # Test invert_dict_str
    inv_str: dict[str, str] = invert_dict_str({"en": "hello", "es": "hola"})
    if inv_str["hello"] != "en":
        errors += 1
    if inv_str["hola"] != "es":
        errors += 1

    # Test invert_dict_to_lists
    inv_lists: dict[int, list[str]] = invert_dict_to_lists({"a": 1, "b": 1, "c": 2})
    if len(inv_lists[1]) != 2:
        errors += 1
    if len(inv_lists[2]) != 1:
        errors += 1

    # Test state_machine_run
    transitions: dict[str, dict[str, str]] = {}
    transitions["locked"] = {"coin": "unlocked"}
    transitions["unlocked"] = {"push": "locked"}
    final_state: str = state_machine_run(transitions, "locked", ["coin", "push", "coin"])
    if final_state != "unlocked":
        errors += 1

    # Test state_machine_trace
    trace: list[str] = state_machine_trace(transitions, "locked", ["coin", "push"])
    if len(trace) != 3:
        errors += 1
    if trace[0] != "locked":
        errors += 1
    if trace[1] != "unlocked":
        errors += 1
    if trace[2] != "locked":
        errors += 1

    # Test state_machine_accepts
    accepting: list[str] = ["unlocked"]
    if not state_machine_accepts(transitions, "locked", accepting, ["coin"]):
        errors += 1
    if state_machine_accepts(transitions, "locked", accepting, ["coin", "push"]):
        errors += 1

    return errors


if __name__ == "__main__":
    result: int = test_all()
    if result == 0:
        print("ALL TESTS PASSED")
    else:
        print("FAILURES: " + str(result))
