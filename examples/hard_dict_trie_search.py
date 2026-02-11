def trie_insert(nodes_keys: list[str], nodes_children: list[list[int]], nodes_end: list[int], word: str) -> int:
    node: int = 0
    i: int = 0
    while i < len(word):
        ch: str = word[i]
        children: list[int] = nodes_children[node]
        nk: str = nodes_keys[node]
        found: int = -1
        j: int = 0
        while j < len(nk):
            if nk[j] == ch:
                found = children[j]
            j = j + 1
        if found >= 0:
            node = found
        else:
            new_id: int = len(nodes_keys)
            nodes_keys.append("")
            nodes_children.append([])
            nodes_end.append(0)
            nodes_keys[node] = nk + ch
            children.append(new_id)
            node = new_id
        i = i + 1
    nodes_end[node] = 1
    return node

def trie_search(nodes_keys: list[str], nodes_children: list[list[int]], nodes_end: list[int], word: str) -> int:
    node: int = 0
    i: int = 0
    while i < len(word):
        ch: str = word[i]
        nk: str = nodes_keys[node]
        children: list[int] = nodes_children[node]
        found: int = -1
        j: int = 0
        while j < len(nk):
            if nk[j] == ch:
                found = children[j]
            j = j + 1
        if found < 0:
            return 0
        node = found
        i = i + 1
    return nodes_end[node]

def trie_prefix(nodes_keys: list[str], nodes_children: list[list[int]], pref: str) -> int:
    node: int = 0
    i: int = 0
    while i < len(pref):
        ch: str = pref[i]
        nk: str = nodes_keys[node]
        children: list[int] = nodes_children[node]
        found: int = -1
        j: int = 0
        while j < len(nk):
            if nk[j] == ch:
                found = children[j]
            j = j + 1
        if found < 0:
            return 0
        node = found
        i = i + 1
    return 1

def trie_node_count(nodes_keys: list[str]) -> int:
    return len(nodes_keys)

def test_module() -> int:
    passed: int = 0
    nk: list[str] = [""]
    nc: list[list[int]] = [[]]
    ne: list[int] = [0]
    trie_insert(nk, nc, ne, "apple")
    trie_insert(nk, nc, ne, "app")
    trie_insert(nk, nc, ne, "bat")
    if trie_search(nk, nc, ne, "apple") == 1:
        passed = passed + 1
    if trie_search(nk, nc, ne, "app") == 1:
        passed = passed + 1
    if trie_search(nk, nc, ne, "ap") == 0:
        passed = passed + 1
    if trie_prefix(nk, nc, "ap") == 1:
        passed = passed + 1
    if trie_prefix(nk, nc, "cat") == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
