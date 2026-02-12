from typing import List, Tuple

def xor_lists(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(a)):
        result.append(a[i] ^ b[i])
    return result

def simple_prf(key: List[int], data: List[int]) -> List[int]:
    h: List[int] = [0] * 8
    combined: List[int] = []
    for b in key:
        combined.append(b)
    for b in data:
        combined.append(b)
    for i in range(len(combined)):
        idx: int = i % 8
        h[idx] = ((h[idx] * 37 + combined[i] + 1) ^ h[(idx + 5) % 8]) & 0xFF
    return h

def int_to_bytes(n: int) -> List[int]:
    result: List[int] = [0, 0, 0, 0]
    result[0] = (n >> 24) & 0xFF
    result[1] = (n >> 16) & 0xFF
    result[2] = (n >> 8) & 0xFF
    result[3] = n & 0xFF
    return result

def pbkdf2_block(password: List[int], salt: List[int], iterations: int, block_num: int) -> List[int]:
    salt_block: List[int] = []
    for b in salt:
        salt_block.append(b)
    for b in int_to_bytes(block_num):
        salt_block.append(b)
    u: List[int] = simple_prf(password, salt_block)
    result: List[int] = []
    for b in u:
        result.append(b)
    for i in range(1, iterations):
        u = simple_prf(password, u)
        result = xor_lists(result, u)
    return result

def pbkdf2_derive(password: List[int], salt: List[int], iterations: int, key_len: int) -> List[int]:
    block_size: int = 8
    blocks_needed: int = (key_len + block_size - 1) // block_size
    derived: List[int] = []
    for i in range(1, blocks_needed + 1):
        block: List[int] = pbkdf2_block(password, salt, iterations, i)
        for b in block:
            derived.append(b)
    result: List[int] = []
    for i in range(key_len):
        result.append(derived[i])
    return result

def verify_password(password: List[int], salt: List[int], expected: List[int], iterations: int) -> bool:
    derived: List[int] = pbkdf2_derive(password, salt, iterations, len(expected))
    diff: int = 0
    for i in range(len(expected)):
        diff = diff | (derived[i] ^ expected[i])
    return diff == 0
