from typing import List, Tuple

def mix_hash(data: List[int], rounds: int) -> List[int]:
    state: List[int] = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A]
    for r in range(rounds):
        for i in range(len(data)):
            idx: int = i % 4
            state[idx] = ((state[idx] * 31 + data[i]) ^ state[(idx + 1) % 4]) & 0xFFFFFFFF
    return state

def gen_salt(seed: int, length: int) -> List[int]:
    salt: List[int] = []
    val: int = seed
    for i in range(length):
        val = ((val * 1103515245) + 12345) & 0x7FFFFFFF
        salt.append(val & 0xFF)
    return salt

def hash_password(pw: List[int], salt: List[int], iters: int) -> List[int]:
    combined: List[int] = []
    for s in salt:
        combined.append(s)
    for p in pw:
        combined.append(p)
    state: List[int] = mix_hash(combined, 1)
    for i in range(iters):
        data: List[int] = []
        for s in state:
            data.append(s & 0xFF)
            data.append((s >> 8) & 0xFF)
        for p in pw:
            data.append(p)
        state = mix_hash(data, 1)
    result: List[int] = []
    for s in state:
        result.append(s & 0xFF)
        result.append((s >> 8) & 0xFF)
        result.append((s >> 16) & 0xFF)
        result.append((s >> 24) & 0xFF)
    return result

def verify_pw(pw: List[int], salt: List[int], expected: List[int], iters: int) -> bool:
    computed: List[int] = hash_password(pw, salt, iters)
    diff: int = 0
    for i in range(len(expected)):
        if i < len(computed):
            diff = diff | (computed[i] ^ expected[i])
    return diff == 0

def pw_strength(pw: List[int]) -> int:
    score: int = len(pw) * 4
    unique: List[int] = []
    for p in pw:
        found: bool = False
        for u in unique:
            if u == p:
                found = True
        if not found:
            unique.append(p)
    return score + len(unique) * 2
