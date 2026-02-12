from typing import List, Tuple

def hkdf_extract(salt: List[int], ikm: List[int]) -> List[int]:
    h: List[int] = [0] * 8
    combined: List[int] = []
    for s in salt:
        combined.append(s)
    for m in ikm:
        combined.append(m)
    for i in range(len(combined)):
        idx: int = i % 8
        h[idx] = ((h[idx] * 37 + combined[i]) ^ h[(idx + 3) % 8]) & 0xFF
    return h

def derive_key(pw: List[int], salt: List[int], length: int) -> List[int]:
    h: List[int] = [0] * 8
    combined: List[int] = []
    for s in salt:
        combined.append(s)
    for m in pw:
        combined.append(m)
    for i in range(len(combined)):
        idx: int = i % 8
        h[idx] = ((h[idx] * 37 + combined[i]) ^ h[(idx + 3) % 8]) & 0xFF
    output: List[int] = []
    counter: int = 1
    while len(output) < length:
        data: List[int] = []
        for b in h:
            data.append(b)
        data.append(counter & 0xFF)
        t: List[int] = [0] * 8
        for i in range(len(data)):
            idx2: int = i % 8
            t[idx2] = ((t[idx2] * 37 + data[i]) ^ t[(idx2 + 3) % 8]) & 0xFF
        for b in t:
            output.append(b)
        counter = counter + 1
    result: List[int] = []
    for i in range(length):
        result.append(output[i])
    return result

def const_compare(a: List[int], b: List[int]) -> bool:
    if len(a) != len(b):
        return False
    diff: int = 0
    for i in range(len(a)):
        diff = diff | (a[i] ^ b[i])
    return diff == 0

def derive_multiple_flat(master: List[int], salt: List[int], count: int, kl: int) -> List[int]:
    keys: List[int] = []
    for idx in range(count):
        h: List[int] = [0] * 8
        combined: List[int] = []
        for s in salt:
            combined.append(s)
        for m in master:
            combined.append(m)
        combined.append(idx & 0xFF)
        for j in range(len(combined)):
            pos: int = j % 8
            h[pos] = ((h[pos] * 37 + combined[j]) ^ h[(pos + 3) % 8]) & 0xFF
        for j in range(kl):
            keys.append(h[j % 8])
    return keys

def key_length(key: List[int]) -> int:
    return len(key)
