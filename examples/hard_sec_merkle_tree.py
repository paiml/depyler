from typing import List, Tuple

def hash_pair(a: int, b: int) -> int:
    combined: int = ((a * 31) ^ (b * 37)) & 0xFFFFFFFF
    combined = ((combined >> 16) ^ combined) * 0x45D9F3B
    return combined & 0xFFFFFFFF

def hash_leaf(data: int) -> int:
    return ((data * 0x5BD1E995) ^ (data >> 15)) & 0xFFFFFFFF

def build_tree(leaves: List[int]) -> List[int]:
    n: int = len(leaves)
    if n == 0:
        return [0]
    size: int = 1
    while size < n:
        size = size * 2
    tree: List[int] = [0] * (2 * size)
    for i in range(n):
        tree[size + i] = hash_leaf(leaves[i])
    idx: int = size - 1
    while idx >= 1:
        tree[idx] = hash_pair(tree[2 * idx], tree[2 * idx + 1])
        idx = idx - 1
    return tree

def get_root(tree: List[int]) -> int:
    if len(tree) < 2:
        return 0
    return tree[1]

def get_proof(tree: List[int], leaf_idx: int, num_leaves: int) -> List[int]:
    size: int = 1
    while size < num_leaves:
        size = size * 2
    proof: List[int] = []
    idx: int = size + leaf_idx
    while idx > 1:
        if idx % 2 == 0:
            proof.append(tree[idx + 1])
        else:
            proof.append(tree[idx - 1])
        idx = idx // 2
    return proof

def verify_proof(leaf_data: int, proof: List[int], leaf_idx: int, root: int) -> bool:
    current: int = hash_leaf(leaf_data)
    idx: int = leaf_idx
    for p in proof:
        if idx % 2 == 0:
            current = hash_pair(current, p)
        else:
            current = hash_pair(p, current)
        idx = idx // 2
    return current == root
