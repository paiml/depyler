from typing import List, Tuple

def gcd(a: int, b: int) -> int:
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a

def mod_pow(base: int, exp: int, mod: int) -> int:
    result: int = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp >> 1
        base = (base * base) % mod
    return result

def is_prime_trial(n: int) -> bool:
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0:
        return False
    i: int = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 2
    return True

def extended_gcd(a: int, b: int) -> Tuple[int, int, int]:
    if a == 0:
        return (b, 0, 1)
    rest: Tuple[int, int, int] = extended_gcd(b % a, a)
    g: int = rest[0]
    x: int = rest[1]
    y: int = rest[2]
    return (g, y - (b // a) * x, x)

def mod_inverse(a: int, m: int) -> int:
    result: Tuple[int, int, int] = extended_gcd(a % m, m)
    g: int = result[0]
    x: int = result[1]
    if g != 1:
        return -1
    return (x % m + m) % m

def generate_keypair(p: int, q: int) -> Tuple[int, int, int]:
    n: int = p * q
    phi: int = (p - 1) * (q - 1)
    e: int = 65537
    if gcd(e, phi) != 1:
        e = 3
        while gcd(e, phi) != 1:
            e = e + 2
    d: int = mod_inverse(e, phi)
    return (n, e, d)

def rsa_encrypt(msg: int, e: int, n: int) -> int:
    return mod_pow(msg, e, n)

def rsa_decrypt(cipher: int, d: int, n: int) -> int:
    return mod_pow(cipher, d, n)
