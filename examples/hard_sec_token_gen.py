from typing import List, Tuple

def hash_token(uid: int, ts: int, secret: int) -> int:
    h: int = uid * 0x5BD1E995
    h = (h + ts * 0x1B873593) & 0xFFFFFFFF
    h = h ^ secret
    h = ((h >> 16) ^ h) * 0x45D9F3B
    h = ((h >> 16) ^ h) & 0xFFFFFFFF
    return h

def gen_token(uid: int, ts: int, secret: int) -> List[int]:
    h: int = hash_token(uid, ts, secret)
    token: List[int] = []
    token.append(uid & 0xFF)
    token.append((uid >> 8) & 0xFF)
    token.append(ts & 0xFF)
    token.append((ts >> 8) & 0xFF)
    token.append((ts >> 16) & 0xFF)
    token.append((ts >> 24) & 0xFF)
    token.append(h & 0xFF)
    token.append((h >> 8) & 0xFF)
    token.append((h >> 16) & 0xFF)
    token.append((h >> 24) & 0xFF)
    return token

def verify_token(token: List[int], secret: int) -> bool:
    if len(token) < 10:
        return False
    uid: int = token[0] | (token[1] << 8)
    ts: int = token[2] | (token[3] << 8) | (token[4] << 16) | (token[5] << 24)
    eh: int = hash_token(uid, ts, secret)
    th: int = token[6] | (token[7] << 8) | (token[8] << 16) | (token[9] << 24)
    return eh == th

def is_expired(token: List[int], now: int, ttl: int) -> bool:
    if len(token) < 6:
        return True
    ts: int = token[2] | (token[3] << 8) | (token[4] << 16) | (token[5] << 24)
    return (now - ts) > ttl

def rotate_secret(old: int, factor: int) -> int:
    ns: int = old ^ factor
    ns = ((ns >> 16) ^ ns) * 0x45D9F3B
    return ns & 0xFFFFFFFF

def batch_gen(uids: List[int], ts: int, secret: int) -> List[List[int]]:
    tokens: List[List[int]] = []
    for uid in uids:
        tokens.append(gen_token(uid, ts, secret))
    return tokens
