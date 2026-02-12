from typing import List, Tuple

def quarter_round(s: List[int], a: int, b: int, c: int, d: int) -> List[int]:
    r: List[int] = []
    for x in s:
        r.append(x)
    r[a] = (r[a] + r[b]) & 0xFFFFFFFF
    r[d] = r[d] ^ r[a]
    r[d] = ((r[d] << 16) | (r[d] >> 16)) & 0xFFFFFFFF
    r[c] = (r[c] + r[d]) & 0xFFFFFFFF
    r[b] = r[b] ^ r[c]
    r[b] = ((r[b] << 12) | (r[b] >> 20)) & 0xFFFFFFFF
    r[a] = (r[a] + r[b]) & 0xFFFFFFFF
    r[d] = r[d] ^ r[a]
    r[d] = ((r[d] << 8) | (r[d] >> 24)) & 0xFFFFFFFF
    r[c] = (r[c] + r[d]) & 0xFFFFFFFF
    r[b] = r[b] ^ r[c]
    r[b] = ((r[b] << 7) | (r[b] >> 25)) & 0xFFFFFFFF
    return r

def chacha20_block(key: List[int], nonce: List[int], counter: int) -> List[int]:
    state: List[int] = [0x61707865, 0x3320646E, 0x79622D32, 0x6B206574]
    for i in range(8):
        state.append(key[i])
    state.append(counter)
    for i in range(3):
        state.append(nonce[i])
    w: List[int] = []
    for sv in state:
        w.append(sv)
    for i in range(10):
        w = quarter_round(w, 0, 4, 8, 12)
        w = quarter_round(w, 1, 5, 9, 13)
        w = quarter_round(w, 2, 6, 10, 14)
        w = quarter_round(w, 3, 7, 11, 15)
        w = quarter_round(w, 0, 5, 10, 15)
        w = quarter_round(w, 1, 6, 11, 12)
        w = quarter_round(w, 2, 7, 8, 13)
        w = quarter_round(w, 3, 4, 9, 14)
    result: List[int] = []
    for i in range(16):
        result.append((w[i] + state[i]) & 0xFFFFFFFF)
    return result

def serialize_block(block: List[int]) -> List[int]:
    out: List[int] = []
    for word in block:
        out.append(word & 0xFF)
        out.append((word >> 8) & 0xFF)
        out.append((word >> 16) & 0xFF)
        out.append((word >> 24) & 0xFF)
    return out

def chacha20_encrypt(key: List[int], nonce: List[int], plaintext: List[int]) -> List[int]:
    ct: List[int] = []
    num_blocks: int = (len(plaintext) + 63) // 64
    for blk in range(num_blocks):
        ks: List[int] = serialize_block(chacha20_block(key, nonce, blk))
        start: int = blk * 64
        end: int = start + 64
        if end > len(plaintext):
            end = len(plaintext)
        for i in range(start, end):
            ct.append(plaintext[i] ^ ks[i - start])
    return ct

def chacha20_decrypt(key: List[int], nonce: List[int], ct: List[int]) -> List[int]:
    return chacha20_encrypt(key, nonce, ct)
