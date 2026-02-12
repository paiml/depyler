from typing import List, Tuple

def init_p_array(seed: int) -> List[int]:
    p: List[int] = []
    val: int = seed
    for i in range(18):
        val = ((val * 2654435761) + i) & 0xFFFFFFFF
        p.append(val)
    return p

def init_s_box(seed: int, box_id: int) -> List[int]:
    s: List[int] = []
    val: int = seed ^ (box_id * 0x9E3779B9)
    for i in range(256):
        val = ((val * 1664525 + 1013904223) ^ (i * 17)) & 0xFFFFFFFF
        s.append(val)
    return s

def feistel_f(x: int, s0: List[int], s1: List[int], s2: List[int], s3: List[int]) -> int:
    a: int = (x >> 24) & 0xFF
    b: int = (x >> 16) & 0xFF
    c: int = (x >> 8) & 0xFF
    d: int = x & 0xFF
    result: int = (s0[a] + s1[b]) & 0xFFFFFFFF
    result = result ^ s2[c]
    result = (result + s3[d]) & 0xFFFFFFFF
    return result

def encrypt_pair(left: int, right: int, p: List[int], s0: List[int], s1: List[int], s2: List[int], s3: List[int]) -> Tuple[int, int]:
    xl: int = left
    xr: int = right
    for i in range(16):
        xl = xl ^ p[i]
        xr = xr ^ feistel_f(xl, s0, s1, s2, s3)
        temp: int = xl
        xl = xr
        xr = temp
    temp2: int = xl
    xl = xr
    xr = temp2
    xr = xr ^ p[16]
    xl = xl ^ p[17]
    return (xl & 0xFFFFFFFF, xr & 0xFFFFFFFF)

def decrypt_pair(left: int, right: int, p: List[int], s0: List[int], s1: List[int], s2: List[int], s3: List[int]) -> Tuple[int, int]:
    xl: int = left
    xr: int = right
    xl = xl ^ p[17]
    xr = xr ^ p[16]
    for i in range(15, -1, -1):
        temp: int = xl
        xl = xr
        xr = temp
        xr = xr ^ feistel_f(xl, s0, s1, s2, s3)
        xl = xl ^ p[i]
    return (xl & 0xFFFFFFFF, xr & 0xFFFFFFFF)

def encrypt_blocks(data: List[int], p: List[int], s0: List[int], s1: List[int], s2: List[int], s3: List[int]) -> List[int]:
    result: List[int] = []
    i: int = 0
    while i + 1 < len(data):
        pair: Tuple[int, int] = encrypt_pair(data[i], data[i + 1], p, s0, s1, s2, s3)
        result.append(pair[0])
        result.append(pair[1])
        i = i + 2
    return result
