"""Prefix tree (trie) using flat arrays for insert, search, starts_with.

Tests: insert words, search existing/missing, starts_with prefix matching.
"""


def trie_create() -> list[list[int]]:
    """Create a new trie as flat array of 26-children nodes. Node 0 is root."""
    root: list[int] = []
    i: int = 0
    while i < 27:
        root.append(0)
        i = i + 1
    result: list[list[int]] = [root]
    return result


def trie_insert(trie: list[list[int]], word: str) -> list[list[int]]:
    """Insert a word into the trie. Last element of each node is end marker."""
    result: list[list[int]] = []
    k: int = 0
    while k < len(trie):
        row: list[int] = trie[k][:]
        result.append(row)
        k = k + 1
    node: int = 0
    idx: int = 0
    while idx < len(word):
        c: int = ord(word[idx]) - ord("a")
        if result[node][c] == 0:
            new_node: list[int] = []
            j: int = 0
            while j < 27:
                new_node.append(0)
                j = j + 1
            result.append(new_node)
            result[node][c] = len(result) - 1
        node = result[node][c]
        idx = idx + 1
    result[node][26] = 1
    return result


def trie_search(trie: list[list[int]], word: str) -> int:
    """Search for exact word in trie. Returns 1 if found, 0 otherwise."""
    node: int = 0
    idx: int = 0
    while idx < len(word):
        c: int = ord(word[idx]) - ord("a")
        next_node: int = trie[node][c]
        if next_node == 0:
            return 0
        node = next_node
        idx = idx + 1
    return trie[node][26]


def trie_starts_with(trie: list[list[int]], prefix: str) -> int:
    """Check if any word starts with prefix. Returns 1 if yes, 0 otherwise."""
    node: int = 0
    idx: int = 0
    while idx < len(prefix):
        c: int = ord(prefix[idx]) - ord("a")
        if trie[node][c] == 0:
            return 0
        node = trie[node][c]
        idx = idx + 1
    return 1


def test_module() -> int:
    """Test prefix tree operations."""
    ok: int = 0

    t: list[list[int]] = trie_create()
    t = trie_insert(t, "apple")
    t = trie_insert(t, "app")
    t = trie_insert(t, "banana")

    if trie_search(t, "apple") == 1:
        ok = ok + 1

    if trie_search(t, "app") == 1:
        ok = ok + 1

    if trie_search(t, "ap") == 0:
        ok = ok + 1

    if trie_search(t, "banana") == 1:
        ok = ok + 1

    if trie_starts_with(t, "app") == 1:
        ok = ok + 1

    if trie_starts_with(t, "ban") == 1:
        ok = ok + 1

    if trie_starts_with(t, "cat") == 0:
        ok = ok + 1

    return ok
