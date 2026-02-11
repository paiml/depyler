"""Trie-like counting using flat arrays to simulate trie nodes."""


def trie_create(max_nodes: int) -> list[int]:
    """Create trie storage. Returns flat array: children[node*128+char] = next_node.
    Also stores count at index max_nodes*128 + node.
    """
    total: int = max_nodes * 128 + max_nodes
    storage: list[int] = []
    i: int = 0
    while i < total:
        storage.append(0)
        i = i + 1
    return storage


def trie_insert(storage: list[int], max_nodes: int, next_id: int, word: str) -> int:
    """Insert word into trie. Returns updated next_id."""
    node: int = 0
    i: int = 0
    n: int = len(word)
    curr_next: int = next_id
    while i < n:
        ch: int = ord(word[i])
        child: int = storage[node * 128 + ch]
        if child == 0:
            storage[node * 128 + ch] = curr_next
            child = curr_next
            curr_next = curr_next + 1
        node = child
        i = i + 1
    count_idx: int = max_nodes * 128 + node
    storage[count_idx] = storage[count_idx] + 1
    return curr_next


def trie_search(storage: list[int], max_nodes: int, word: str) -> int:
    """Search for word in trie. Returns count (0 if not found)."""
    node: int = 0
    i: int = 0
    n: int = len(word)
    while i < n:
        ch: int = ord(word[i])
        child: int = storage[node * 128 + ch]
        if child == 0:
            return 0
        node = child
        i = i + 1
    return storage[max_nodes * 128 + node]


def trie_starts_with(storage: list[int], prefix: str) -> int:
    """Check if any word starts with prefix. Returns 1 if yes."""
    node: int = 0
    i: int = 0
    n: int = len(prefix)
    while i < n:
        ch: int = ord(prefix[i])
        child: int = storage[node * 128 + ch]
        if child == 0:
            return 0
        node = child
        i = i + 1
    return 1


def test_module() -> int:
    """Test trie operations."""
    passed: int = 0

    max_n: int = 100
    storage: list[int] = trie_create(max_n)
    nid: int = 1

    nid = trie_insert(storage, max_n, nid, "cat")
    nid = trie_insert(storage, max_n, nid, "car")
    nid = trie_insert(storage, max_n, nid, "cat")
    nid = trie_insert(storage, max_n, nid, "dog")

    if trie_search(storage, max_n, "cat") == 2:
        passed = passed + 1

    if trie_search(storage, max_n, "car") == 1:
        passed = passed + 1

    if trie_search(storage, max_n, "dog") == 1:
        passed = passed + 1

    if trie_search(storage, max_n, "dot") == 0:
        passed = passed + 1

    if trie_starts_with(storage, "ca") == 1:
        passed = passed + 1

    if trie_starts_with(storage, "xyz") == 0:
        passed = passed + 1

    return passed
