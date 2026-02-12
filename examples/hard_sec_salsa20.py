from typing import List, Tuple

def rotl32(v: int, n: int) -> int:
    return ((v << n) | (v >> (32 - n))) & 0xFFFFFFFF

def salsa_qr(y: List[int], a: int, b: int, c: int, d: int) -> List[int]:
    s: List[int] = []
    for x in y:
        s.append(x)
    s[b] = s[b] ^ rotl32((s[a] + s[d]) & 0xFFFFFFFF, 7)
    s[c] = s[c] ^ rotl32((s[b] + s[a]) & 0xFFFFFFFF, 9)
    s[d] = s[d] ^ rotl32((s[c] + s[b]) & 0xFFFFFFFF, 13)
    s[a] = s[a] ^ rotl32((s[d] + s[c]) & 0xFFFFFFFF, 18)
    return s

def salsa20_core(inp: List[int]) -> List[int]:
    x: List[int] = []
    for v in inp:
        x.append(v)
    for i in range(10):
        x = salsa_qr(x, 0, 4, 8, 12)
        x = salsa_qr(x, 5, 9, 13, 1)
        x = salsa_qr(x, 10, 14, 2, 6)
        x = salsa_qr(x, 15, 3, 7, 11)
        x = salsa_qr(x, 0, 1, 2, 3)
        x = salsa_qr(x, 5, 6, 7, 4)
        x = salsa_qr(x, 10, 11, 8, 9)
        x = salsa_qr(x, 15, 12, 13, 14)
    result: List[int] = []
    for i in range(16):
        result.append((x[i] + inp[i]) & 0xFFFFFFFF)
    return result

def word_to_bytes(w: int) -> List[int]:
    return [w & 0xFF, (w >> 8) & 0xFF, (w >> 16) & 0xFF, (w >> 24) & 0xFF]

def salsa20_expand(key: List[int], nonce: List[int], counter: int) -> List[int]:
    state: List[int] = [0] * 16
    state[0] = 0x61707865
    state[5] = 0x3320646E
    state[10] = 0x79622D32
    state[15] = 0x6B206574
    for i in range(4):
        state[1 + i] = key[i]
        state[11 + i] = key[4 + i]
    state[6] = nonce[0]
    state[7] = nonce[1]
    state[8] = counter
    state[9] = 0
    return salsa20_core(state)

def salsa20_encrypt(key: List[int], nonce: List[int], plaintext: List[int]) -> List[int]:
    ct: List[int] = []
    blocks: int = (len(plaintext) + 63) // 64
    for blk in range(blocks):
        block: List[int] = salsa20_expand(key, nonce, blk)
        ks: List[int] = []
        for w in block:
            for b in word_to_bytes(w):
                ks.append(b)
        start: int = blk * 64
        end: int = start + 64
        if end > len(plaintext):
            end = len(plaintext)
        for i in range(start, end):
            ct.append(plaintext[i] ^ ks[i - start])
    return ct
