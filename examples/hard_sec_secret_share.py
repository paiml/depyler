from typing import List, Tuple

def xor_split(secret: List[int], n: int, seed: int) -> List[List[int]]:
    shares: List[List[int]] = []
    val: int = seed
    for i in range(n - 1):
        share: List[int] = []
        for j in range(len(secret)):
            val = ((val * 6364136223846793005) + 1) & 0xFFFFFFFF
            share.append(val & 0xFF)
        shares.append(share)
    last: List[int] = []
    for j in range(len(secret)):
        xv: int = secret[j]
        for i in range(n - 1):
            xv = xv ^ shares[i][j]
        last.append(xv)
    shares.append(last)
    return shares

def xor_combine(shares: List[List[int]]) -> List[int]:
    if len(shares) == 0:
        return []
    result: List[int] = []
    for j in range(len(shares[0])):
        val: int = 0
        for i in range(len(shares)):
            val = val ^ shares[i][j]
        result.append(val)
    return result

def threshold_split(secret: int, t: int, n: int, prime: int, seed: int) -> List[Tuple[int, int]]:
    coeffs: List[int] = [secret]
    val: int = seed
    for i in range(1, t):
        val = ((val * 48271) + 1) & 0x7FFFFFFF
        coeffs.append(val % prime)
    shares: List[Tuple[int, int]] = []
    for x in range(1, n + 1):
        y: int = 0
        power: int = 1
        for c in coeffs:
            y = (y + c * power) % prime
            power = (power * x) % prime
        shares.append((x, y))
    return shares

def verify_secret(orig: List[int], recov: List[int]) -> bool:
    if len(orig) != len(recov):
        return False
    for i in range(len(orig)):
        if orig[i] != recov[i]:
            return False
    return True

def distribute(shares: List[List[int]], n: int) -> List[List[List[int]]]:
    dist: List[List[List[int]]] = []
    for i in range(n):
        bundle: List[List[int]] = []
        bundle.append(shares[i % len(shares)])
        dist.append(bundle)
    return dist
