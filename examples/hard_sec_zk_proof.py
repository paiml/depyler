from typing import List, Tuple

def mod_pow_zk(base: int, exp: int, mod: int) -> int:
    result: int = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp >> 1
        base = (base * base) % mod
    return result

def schnorr_commit(g: int, r: int, p: int) -> int:
    return mod_pow_zk(g, r, p)

def schnorr_challenge(commit: int, pub: int) -> int:
    h: int = ((commit * 0x5BD1E995) ^ (pub * 0x1B873593)) & 0xFFFFFFFF
    return h % 1000000007

def schnorr_response(r: int, challenge: int, secret: int, q: int) -> int:
    return (r + challenge * secret) % q

def schnorr_verify(g: int, pub: int, commit: int, ch: int, resp: int, p: int) -> bool:
    lhs: int = mod_pow_zk(g, resp, p)
    rhs: int = (commit * mod_pow_zk(pub, ch, p)) % p
    return lhs == rhs

def sigma_round(g: int, secret: int, r: int, p: int, q: int) -> Tuple[int, int, int]:
    commit: int = schnorr_commit(g, r, p)
    pub: int = mod_pow_zk(g, secret, p)
    ch: int = schnorr_challenge(commit, pub)
    resp: int = schnorr_response(r, ch, secret, q)
    return (commit, ch, resp)

def batch_verify_proofs(g: int, pubs: List[int], commits: List[int], chs: List[int], resps: List[int], p: int) -> bool:
    for i in range(len(pubs)):
        if not schnorr_verify(g, pubs[i], commits[i], chs[i], resps[i], p):
            return False
    return True
