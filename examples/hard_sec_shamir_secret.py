from typing import List, Tuple

def eval_poly(coeffs: List[int], x: int, prime: int) -> int:
    result: int = 0
    power: int = 1
    for c in coeffs:
        result = (result + c * power) % prime
        power = (power * x) % prime
    return result

def mod_pow_ss(base: int, exp: int, mod: int) -> int:
    result: int = 1
    b: int = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * b) % mod
        exp = exp >> 1
        b = (b * b) % mod
    return result

def create_shares(secret: int, threshold: int, n: int, prime: int, seed: int) -> List[Tuple[int, int]]:
    coeffs: List[int] = [secret]
    val: int = seed
    for i in range(1, threshold):
        val = ((val * 1103515245) + 12345) & 0x7FFFFFFF
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

def reconstruct(shares: List[Tuple[int, int]], prime: int) -> int:
    secret: int = 0
    for j in range(len(shares)):
        xj: int = shares[j][0]
        num: int = 1
        den: int = 1
        for m in range(len(shares)):
            if m != j:
                num = (num * (0 - shares[m][0])) % prime
                den = (den * (xj - shares[m][0])) % prime
        result: int = 1
        b: int = den % prime
        exp: int = prime - 2
        while exp > 0:
            if exp % 2 == 1:
                result = (result * b) % prime
            exp = exp >> 1
            b = (b * b) % prime
        basis: int = (num * result) % prime
        secret = (secret + shares[j][1] * basis) % prime
    return (secret + prime) % prime

def verify_share(share_x: int, share_y: int, coeffs: List[int], prime: int) -> bool:
    y: int = 0
    power: int = 1
    for c in coeffs:
        y = (y + c * power) % prime
        power = (power * share_x) % prime
    return y == share_y
