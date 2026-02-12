from typing import List, Tuple

def xor_encrypt(pt: List[int], key: List[int]) -> List[int]:
    ct: List[int] = []
    kl: int = len(key)
    for i in range(len(pt)):
        ct.append(pt[i] ^ key[i % kl])
    return ct

def xor_decrypt(ct: List[int], key: List[int]) -> List[int]:
    return xor_encrypt(ct, key)

def xor_single(data: List[int], kb: int) -> List[int]:
    result: List[int] = []
    for b in data:
        result.append(b ^ kb)
    return result

def score_english(data: List[int]) -> int:
    score: int = 0
    common: List[int] = [32, 101, 116, 97, 111, 105, 110, 115]
    for b in data:
        for c in common:
            if b == c:
                score = score + 1
    return score

def break_single_xor(ct: List[int]) -> Tuple[int, int]:
    best_key: int = 0
    best_score: int = 0
    for key in range(256):
        dec: List[int] = xor_single(ct, key)
        s: int = score_english(dec)
        if s > best_score:
            best_score = s
            best_key = key
    return (best_key, best_score)

def hamming_dist(a: List[int], b: List[int]) -> int:
    dist: int = 0
    length: int = len(a)
    if len(b) < length:
        length = len(b)
    for i in range(length):
        xor: int = a[i] ^ b[i]
        while xor > 0:
            dist = dist + (xor & 1)
            xor = xor >> 1
    return dist
