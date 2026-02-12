from typing import List, Tuple

def simple_hash(data: List[int]) -> List[int]:
    h: List[int] = [0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89]
    for i in range(len(data)):
        idx: int = i % 8
        h[idx] = ((h[idx] * 31 + data[i]) ^ (h[(idx + 3) % 8])) & 0xFF
    return h

def pad_input(mac_input: List[int], block_size: int) -> List[int]:
    padded: List[int] = []
    for b in mac_input:
        padded.append(b)
    while len(padded) < block_size:
        padded.append(0)
    return padded

def hmac_compute(mac_input: List[int], message: List[int]) -> List[int]:
    block_size: int = 64
    k: List[int] = []
    for b in mac_input:
        k.append(b)
    while len(k) < block_size:
        k.append(0)
    inner_data: List[int] = []
    for i in range(block_size):
        inner_data.append(k[i] ^ 0x36)
    for b in message:
        inner_data.append(b)
    ih: List[int] = [0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89]
    for i in range(len(inner_data)):
        idx: int = i % 8
        ih[idx] = ((ih[idx] * 31 + inner_data[i]) ^ (ih[(idx + 3) % 8])) & 0xFF
    outer_data: List[int] = []
    for i in range(block_size):
        outer_data.append(k[i] ^ 0x5C)
    for b in ih:
        outer_data.append(b)
    oh: List[int] = [0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89]
    for i in range(len(outer_data)):
        idx2: int = i % 8
        oh[idx2] = ((oh[idx2] * 31 + outer_data[i]) ^ (oh[(idx2 + 3) % 8])) & 0xFF
    return oh

def verify_hmac(expected: List[int], computed: List[int]) -> bool:
    if len(expected) < 8 or len(computed) < 8:
        return False
    diff: int = 0
    for i in range(8):
        diff = diff | (expected[i] ^ computed[i])
    return diff == 0

def xor_pad(mac_input: List[int], pad_byte: int) -> List[int]:
    result: List[int] = []
    for b in mac_input:
        result.append(b ^ pad_byte)
    return result

def hash_block(data: List[int], init: List[int]) -> List[int]:
    h: List[int] = []
    for b in init:
        h.append(b)
    for i in range(len(data)):
        idx: int = i % 8
        h[idx] = ((h[idx] * 31 + data[i]) ^ (h[(idx + 3) % 8])) & 0xFF
    return h
