from typing import List, Tuple

def hash_commit(value: int, nonce: int) -> int:
    c: int = value * 0x5BD1E995 + nonce * 0x1B873593
    c = c ^ (c >> 13)
    c = (c * 0xC2B2AE35) & 0xFFFFFFFF
    c = c ^ (c >> 16)
    return c & 0xFFFFFFFF

def create_commitment(value: int, nonce: int) -> Tuple[int, int]:
    return (hash_commit(value, nonce), nonce)

def verify_commitment(commitment: int, value: int, nonce: int) -> bool:
    return hash_commit(value, nonce) == commitment

def batch_commit(values: List[int], nonces: List[int]) -> List[int]:
    results: List[int] = []
    for i in range(len(values)):
        results.append(hash_commit(values[i], nonces[i]))
    return results

def batch_verify(commitments: List[int], values: List[int], nonces: List[int]) -> bool:
    for i in range(len(commitments)):
        if not verify_commitment(commitments[i], values[i], nonces[i]):
            return False
    return True

def generate_nonces(seed: int, count: int) -> List[int]:
    nonces: List[int] = []
    val: int = seed
    for i in range(count):
        val = ((val * 1664525 + 1013904223)) & 0xFFFFFFFF
        nonces.append(val)
    return nonces
