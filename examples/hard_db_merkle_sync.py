from typing import List, Tuple

def hash_node(left: int, right: int) -> int:
    return ((left * 31) ^ (right * 37)) & 0xFFFFFFFF

def build_merkle(data: List[int]) -> List[int]:
    n: int = 1
    while n < len(data):
        n = n * 2
    tree: List[int] = [0] * (2 * n)
    for i in range(len(data)):
        tree[n + i] = data[i]
    i2: int = n - 1
    while i2 >= 1:
        tree[i2] = hash_node(tree[2 * i2], tree[2 * i2 + 1])
        i2 = i2 - 1
    return tree

def find_diffs(tree_a: List[int], tree_b: List[int]) -> List[int]:
    diffs: List[int] = []
    n: int = len(tree_a) // 2
    worklist: List[int] = [1]
    while len(worklist) > 0:
        idx: int = worklist[len(worklist) - 1]
        worklist = worklist[0:len(worklist) - 1]
        if idx < len(tree_a) and idx < len(tree_b):
            if tree_a[idx] != tree_b[idx]:
                if idx >= n:
                    diffs.append(idx - n)
                else:
                    worklist.append(2 * idx)
                    worklist.append(2 * idx + 1)
    return diffs

def sync_data(local: List[int], remote: List[int], diffs: List[int]) -> List[int]:
    result: List[int] = []
    for d in local:
        result.append(d)
    for idx in diffs:
        if idx < len(remote) and idx < len(result):
            result[idx] = remote[idx]
    return result

def needs_sync(tree_a: List[int], tree_b: List[int]) -> bool:
    if len(tree_a) < 2 or len(tree_b) < 2:
        return True
    return tree_a[1] != tree_b[1]

def sync_progress(total_diffs: int, synced: int) -> float:
    if total_diffs == 0:
        return 1.0
    return float(synced) / float(total_diffs)
