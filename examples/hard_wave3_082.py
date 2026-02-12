"""Systems: Memory allocator simulation.
Tests: first-fit, best-fit, fragmentation tracking.
"""
from typing import Dict, List, Tuple

def first_fit_allocate(blocks: List[int], request: int) -> int:
    """Find first block that fits request. Returns index or -1."""
    i: int = 0
    while i < len(blocks):
        if blocks[i] >= request:
            return i
        i += 1
    return -1

def best_fit_allocate(blocks: List[int], request: int) -> int:
    """Find smallest block that fits request. Returns index or -1."""
    best: int = -1
    best_size: int = 999999999
    i: int = 0
    while i < len(blocks):
        if blocks[i] >= request and blocks[i] < best_size:
            best = i
            best_size = blocks[i]
        i += 1
    return best

def worst_fit_allocate(blocks: List[int], request: int) -> int:
    """Find largest block that fits request."""
    best: int = -1
    best_size: int = -1
    i: int = 0
    while i < len(blocks):
        if blocks[i] >= request and blocks[i] > best_size:
            best = i
            best_size = blocks[i]
        i += 1
    return best

def simulate_allocation(blocks: List[int], requests: List[int]) -> List[int]:
    """Simulate first-fit allocation of multiple requests."""
    avail: List[int] = []
    for b in blocks:
        avail.append(b)
    results: List[int] = []
    for req in requests:
        idx: int = first_fit_allocate(avail, req)
        results.append(idx)
        if idx >= 0:
            avail[idx] = avail[idx] - req
    return results

def total_fragmentation(blocks: List[int]) -> int:
    """Sum of all block sizes (external fragmentation measure)."""
    total: int = 0
    for b in blocks:
        if b > 0:
            total = total + b
    return total

def test_allocator() -> bool:
    ok: bool = True
    blocks: List[int] = [100, 500, 200, 300, 600]
    idx: int = first_fit_allocate(blocks, 212)
    if idx != 1:
        ok = False
    bf: int = best_fit_allocate(blocks, 212)
    if bf != 3:
        ok = False
    return ok
