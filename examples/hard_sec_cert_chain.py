from typing import List, Tuple

def hash_cert(issuer: int, subject: int, pub: int) -> int:
    h: int = issuer * 0x5BD1E995
    h = (h ^ (subject * 0x1B873593)) & 0xFFFFFFFF
    h = (h ^ (pub * 0xCC9E2D51)) & 0xFFFFFFFF
    h = ((h >> 16) ^ h) & 0xFFFFFFFF
    return h

def sign_cert(ch: int, priv: int, mod: int) -> int:
    result: int = 1
    base: int = ch % mod
    exp: int = priv
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp >> 1
        base = (base * base) % mod
    return result

def verify_cert(ch: int, sig: int, pub: int, mod: int) -> bool:
    result: int = 1
    base: int = sig % mod
    exp: int = pub
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp >> 1
        base = (base * base) % mod
    return result == ch

def build_chain(issuers: List[int], subjects: List[int], pubs: List[int], privs: List[int], mod: int) -> List[Tuple[int, int]]:
    chain: List[Tuple[int, int]] = []
    for i in range(len(issuers)):
        ch: int = hash_cert(issuers[i], subjects[i], pubs[i])
        sig: int = sign_cert(ch, privs[i], mod)
        chain.append((ch, sig))
    return chain

def verify_chain(chain: List[Tuple[int, int]], pubs: List[int], mod: int) -> bool:
    for i in range(len(chain)):
        pi: int = i
        if pi >= len(pubs):
            pi = len(pubs) - 1
        if not verify_cert(chain[i][0], chain[i][1], pubs[pi], mod):
            return False
    return True

def chain_depth(chain: List[Tuple[int, int]]) -> int:
    return len(chain)
