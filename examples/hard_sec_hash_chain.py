from typing import List, Tuple

def hash_hc(value: int) -> int:
    h: int = value
    h = ((h >> 16) ^ h) * 0x45D9F3B
    h = ((h >> 16) ^ h) * 0x45D9F3B
    h = (h >> 16) ^ h
    return h & 0xFFFFFFFF

def build_chain(seed: int, length: int) -> List[int]:
    chain: List[int] = [seed]
    current: int = seed
    for i in range(length - 1):
        current = hash_hc(current)
        chain.append(current)
    return chain

def verify_link(prev: int, current: int) -> bool:
    return hash_hc(prev) == current

def verify_chain(chain: List[int]) -> bool:
    for i in range(1, len(chain)):
        if not verify_link(chain[i - 1], chain[i]):
            return False
    return True

def chain_auth(chain: List[int], idx: int, value: int) -> bool:
    if idx < 0 or idx >= len(chain):
        return False
    current: int = value
    for i in range(idx, len(chain) - 1):
        current = hash_hc(current)
    return current == chain[len(chain) - 1]

def chain_length_find(seed: int, target: int, max_len: int) -> int:
    current: int = seed
    for i in range(max_len):
        if current == target:
            return i
        current = hash_hc(current)
    return -1
