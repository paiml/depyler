# Pathological string: String comparison and sorting algorithms
# Tests: lexicographic comparison, string sorting, prefix/suffix matching
# Workaround: avoid words[i][j] (chained indexing into list[str] then str).
# Use local var: word = words[i]; then word[j]


def str_less_than(a: str, b: str) -> bool:
    """Lexicographic comparison: return True if a < b."""
    min_len: int = len(a)
    if len(b) < min_len:
        min_len = len(b)
    i: int = 0
    while i < min_len:
        if a[i] < b[i]:
            return True
        if a[i] > b[i]:
            return False
        i = i + 1
    return len(a) < len(b)


def sort_strings(words: list[str]) -> list[str]:
    """Bubble sort list of strings lexicographically."""
    result: list[str] = []
    i: int = 0
    while i < len(words):
        result.append(words[i])
        i = i + 1
    n: int = len(result)
    outer: int = 0
    while outer < n:
        inner: int = 0
        while inner < n - outer - 1:
            left: str = result[inner]
            right: str = result[inner + 1]
            if str_less_than(right, left) == True:
                # Swap via rebuild
                new_result: list[str] = []
                k: int = 0
                while k < len(result):
                    if k == inner:
                        new_result.append(right)
                    elif k == inner + 1:
                        new_result.append(left)
                    else:
                        new_result.append(result[k])
                    k = k + 1
                result = new_result
            inner = inner + 1
        outer = outer + 1
    return result


def starts_with(text: str, prefix: str) -> bool:
    """Check if text starts with prefix."""
    if len(prefix) > len(text):
        return False
    i: int = 0
    while i < len(prefix):
        if text[i] != prefix[i]:
            return False
        i = i + 1
    return True


def ends_with(text: str, suffix: str) -> bool:
    """Check if text ends with suffix."""
    if len(suffix) > len(text):
        return False
    offset: int = len(text) - len(suffix)
    i: int = 0
    while i < len(suffix):
        if text[offset + i] != suffix[i]:
            return False
        i = i + 1
    return True


def longest_common_prefix(words: list[str]) -> str:
    """Find longest common prefix of a list of strings."""
    if len(words) == 0:
        return ""
    prefix: str = words[0]
    i: int = 1
    while i < len(words):
        new_prefix: str = ""
        current_word: str = words[i]
        j: int = 0
        while j < len(prefix) and j < len(current_word):
            pc: str = prefix[j]
            wc: str = current_word[j]
            if pc == wc:
                new_prefix = new_prefix + pc
            else:
                break
            j = j + 1
        prefix = new_prefix
        i = i + 1
    return prefix


def test_module() -> int:
    passed: int = 0
    # Test 1: comparison
    if str_less_than("abc", "abd") == True:
        passed = passed + 1
    # Test 2: equal strings
    if str_less_than("abc", "abc") == False:
        passed = passed + 1
    # Test 3: sort
    sorted_words: list[str] = sort_strings(["banana", "apple", "cherry"])
    if sorted_words[0] == "apple":
        passed = passed + 1
    # Test 4: starts_with
    if starts_with("hello world", "hello") == True:
        passed = passed + 1
    # Test 5: ends_with
    if ends_with("hello world", "world") == True:
        passed = passed + 1
    # Test 6: longest common prefix
    if longest_common_prefix(["flower", "flow", "flight"]) == "fl":
        passed = passed + 1
    # Test 7: no common prefix
    if longest_common_prefix(["abc", "xyz"]) == "":
        passed = passed + 1
    return passed
