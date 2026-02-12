from typing import List, Tuple

def mt_init(seed: int) -> List[int]:
    mt: List[int] = [0] * 624
    mt[0] = seed & 0xFFFFFFFF
    for i in range(1, 624):
        mt[i] = (1812433253 * (mt[i - 1] ^ (mt[i - 1] >> 30)) + i) & 0xFFFFFFFF
    return mt

def mt_twist(mt: List[int]) -> List[int]:
    nmt: List[int] = []
    for v in mt:
        nmt.append(v)
    for i in range(624):
        y: int = (nmt[i] & 0x80000000) + (nmt[(i + 1) % 624] & 0x7FFFFFFF)
        nmt[i] = nmt[(i + 397) % 624] ^ (y >> 1)
        if y % 2 != 0:
            nmt[i] = nmt[i] ^ 0x9908B0DF
    return nmt

def mt_temper(y: int) -> int:
    y = y ^ (y >> 11)
    y = y ^ ((y << 7) & 0x9D2C5680)
    y = y ^ ((y << 15) & 0xEFC60000)
    y = y ^ (y >> 18)
    return y & 0xFFFFFFFF

def mt_sequence(seed: int, count: int) -> List[int]:
    mt: List[int] = [0] * 624
    mt[0] = seed & 0xFFFFFFFF
    for i in range(1, 624):
        mt[i] = (1812433253 * (mt[i - 1] ^ (mt[i - 1] >> 30)) + i) & 0xFFFFFFFF
    idx: int = 624
    results: List[int] = []
    for n in range(count):
        if idx >= 624:
            for i in range(624):
                y: int = (mt[i] & 0x80000000) + (mt[(i + 1) % 624] & 0x7FFFFFFF)
                mt[i] = mt[(i + 397) % 624] ^ (y >> 1)
                if y % 2 != 0:
                    mt[i] = mt[i] ^ 0x9908B0DF
            idx = 0
        val: int = mt[idx]
        val = val ^ (val >> 11)
        val = val ^ ((val << 7) & 0x9D2C5680)
        val = val ^ ((val << 15) & 0xEFC60000)
        val = val ^ (val >> 18)
        results.append(val & 0xFFFFFFFF)
        idx = idx + 1
    return results

def mt_untemper(y: int) -> int:
    y = y ^ (y >> 18)
    y = y ^ ((y << 15) & 0xEFC60000)
    t: int = y
    for i in range(4):
        t = y ^ ((t << 7) & 0x9D2C5680)
    y = t
    t = y
    for i in range(2):
        t = y ^ (t >> 11)
    return t & 0xFFFFFFFF
