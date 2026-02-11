"""Trie using dict of dicts for string prefix operations.

Tests: insert, search, starts_with, count_prefix, all words count.
"""


def trie_create() -> dict[str, int]:
    """Create trie root node. Uses flat encoding: 'path#end'=1 for words."""
    return {}


def trie_insert(trie: dict[str, int], word: str) -> int:
    """Insert word into trie. Returns 1."""
    prefix: str = ""
    i: int = 0
    length: int = len(word)
    while i < length:
        ch: str = word[i]
        prefix = prefix + ch
        marker: str = prefix + "#node"
        trie[marker] = 1
        i = i + 1
    end_marker: str = word + "#end"
    trie[end_marker] = 1
    return 1


def trie_search(trie: dict[str, int], word: str) -> int:
    """Search for exact word. Returns 1 if found, 0 otherwise."""
    end_marker: str = word + "#end"
    if end_marker in trie:
        return 1
    return 0


def trie_starts_with(trie: dict[str, int], prefix: str) -> int:
    """Check if any word starts with prefix. Returns 1 or 0."""
    if len(prefix) == 0:
        return 1
    marker: str = prefix + "#node"
    if marker in trie:
        return 1
    end_marker: str = prefix + "#end"
    if end_marker in trie:
        return 1
    return 0


def trie_count_with_prefix(trie: dict[str, int], words: list[str], prefix: str) -> int:
    """Count how many inserted words start with given prefix."""
    count: int = 0
    i: int = 0
    n_words: int = len(words)
    plen: int = len(prefix)
    while i < n_words:
        w: str = words[i]
        wlen: int = len(w)
        if wlen >= plen:
            end_marker: str = w + "#end"
            if end_marker in trie:
                match: int = 1
                j: int = 0
                while j < plen:
                    if w[j] != prefix[j]:
                        match = 0
                    j = j + 1
                if match == 1:
                    count = count + 1
        i = i + 1
    return count


def trie_longest_common_prefix(words: list[str]) -> str:
    """Find longest common prefix of list of words."""
    n_words: int = len(words)
    if n_words == 0:
        return ""
    first: str = words[0]
    prefix: str = ""
    ci: int = 0
    first_len: int = len(first)
    while ci < first_len:
        ch: str = first[ci]
        all_match: int = 1
        wi: int = 1
        while wi < n_words:
            w: str = words[wi]
            if ci >= len(w):
                all_match = 0
            else:
                if w[ci] != ch:
                    all_match = 0
            wi = wi + 1
        if all_match == 1:
            prefix = prefix + ch
        else:
            return prefix
        ci = ci + 1
    return prefix


def test_module() -> int:
    """Test trie operations."""
    passed: int = 0

    t: dict[str, int] = trie_create()
    trie_insert(t, "apple")
    trie_insert(t, "app")
    trie_insert(t, "banana")

    if trie_search(t, "apple") == 1:
        passed = passed + 1

    if trie_search(t, "app") == 1:
        passed = passed + 1

    if trie_search(t, "appl") == 0:
        passed = passed + 1

    if trie_starts_with(t, "app") == 1:
        passed = passed + 1

    if trie_starts_with(t, "ban") == 1:
        passed = passed + 1

    if trie_starts_with(t, "cat") == 0:
        passed = passed + 1

    ws: list[str] = ["apple", "app", "banana"]
    ct: int = trie_count_with_prefix(t, ws, "app")
    if ct == 2:
        passed = passed + 1

    lcp: str = trie_longest_common_prefix(["flower", "flow", "flight"])
    if lcp == "fl":
        passed = passed + 1

    return passed
