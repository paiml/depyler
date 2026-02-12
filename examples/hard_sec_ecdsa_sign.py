from typing import List, Tuple

def mod_pow_ec(base: int, exp: int, mod: int) -> int:
    result: int = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp >> 1
        base = (base * base) % mod
    return result

def mod_inverse_ec(a: int, m: int) -> int:
    return mod_pow_ec(a, m - 2, m)

def point_add_x(x1: int, y1: int, x2: int, y2: int, p: int) -> int:
    if x1 == x2 and y1 == y2:
        s: int = ((3 * x1 * x1) * mod_inverse_ec(2 * y1, p)) % p
    else:
        s = ((y2 - y1) * mod_inverse_ec(x2 - x1 + p, p)) % p
    x3: int = (s * s - x1 - x2) % p
    return (x3 + p) % p

def point_add_y(x1: int, y1: int, x2: int, y2: int, p: int) -> int:
    if x1 == x2 and y1 == y2:
        s: int = ((3 * x1 * x1) * mod_inverse_ec(2 * y1, p)) % p
    else:
        s = ((y2 - y1) * mod_inverse_ec(x2 - x1 + p, p)) % p
    x3: int = (s * s - x1 - x2) % p
    y3: int = (s * (x1 - x3) - y1) % p
    return (y3 + p) % p

def hash_msg_ec(data: List[int]) -> int:
    h: int = 0x811C9DC5
    for b in data:
        h = h ^ b
        h = (h * 0x01000193) & 0xFFFFFFFF
    return h

def ecdsa_sign_r(k: int, gx: int, gy: int, p: int, n: int) -> int:
    rx: int = gx
    ry: int = gy
    for i in range(k - 1):
        nx: int = point_add_x(rx, ry, gx, gy, p)
        ny: int = point_add_y(rx, ry, gx, gy, p)
        rx = nx
        ry = ny
    return rx % n

def ecdsa_sign_s(k: int, r: int, priv_key: int, msg_hash: int, n: int) -> int:
    k_inv: int = mod_inverse_ec(k, n)
    return (k_inv * (msg_hash + r * priv_key)) % n
