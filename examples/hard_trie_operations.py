def trie_create(size: int) -> list[int]:
    total: int = size * 27
    data: list[int] = []
    i: int = 0
    while i < total:
        data.append(0)
        i = i + 1
    return data


def trie_child_idx(node: int, ch: int) -> int:
    return node * 27 + ch


def trie_end_idx(node: int) -> int:
    return node * 27 + 26


def trie_insert(data: list[int], word: list[int], next_node: int) -> int:
    current: int = 0
    i: int = 0
    while i < len(word):
        ch: int = word[i]
        idx: int = trie_child_idx(current, ch)
        if data[idx] == 0:
            data[idx] = next_node
            next_node = next_node + 1
        current = data[idx]
        i = i + 1
    end: int = trie_end_idx(current)
    data[end] = data[end] + 1
    return next_node


def trie_count(data: list[int], word: list[int]) -> int:
    current: int = 0
    i: int = 0
    while i < len(word):
        ch: int = word[i]
        idx: int = trie_child_idx(current, ch)
        if data[idx] == 0:
            return 0
        current = data[idx]
        i = i + 1
    return data[trie_end_idx(current)]


def trie_has_prefix(data: list[int], prefix: list[int]) -> int:
    current: int = 0
    i: int = 0
    while i < len(prefix):
        ch: int = prefix[i]
        idx: int = trie_child_idx(current, ch)
        if data[idx] == 0:
            return 0
        current = data[idx]
        i = i + 1
    return 1


def trie_prefix_count_children(data: list[int], prefix: list[int]) -> int:
    current: int = 0
    i: int = 0
    while i < len(prefix):
        ch: int = prefix[i]
        idx: int = trie_child_idx(current, ch)
        if data[idx] == 0:
            return 0
        current = data[idx]
        i = i + 1
    count: int = 0
    c: int = 0
    while c < 26:
        if data[trie_child_idx(current, c)] != 0:
            count = count + 1
        c = c + 1
    return count


def test_module() -> int:
    passed: int = 0
    data: list[int] = trie_create(100)
    nn: int = 1
    nn = trie_insert(data, [2, 0, 19], nn)
    nn = trie_insert(data, [2, 0, 17], nn)
    nn = trie_insert(data, [2, 0, 19], nn)
    if trie_count(data, [2, 0, 19]) == 2:
        passed = passed + 1
    if trie_count(data, [2, 0, 17]) == 1:
        passed = passed + 1
    if trie_count(data, [3, 14, 6]) == 0:
        passed = passed + 1
    if trie_has_prefix(data, [2, 0]) == 1:
        passed = passed + 1
    if trie_has_prefix(data, [3]) == 0:
        passed = passed + 1
    pc: int = trie_prefix_count_children(data, [2, 0])
    if pc == 2:
        passed = passed + 1
    if trie_has_prefix(data, []) == 1:
        passed = passed + 1
    return passed
