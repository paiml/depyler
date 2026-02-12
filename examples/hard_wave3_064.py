"""Cryptography: Hash functions (DJB2, FNV, SDBM).
Tests: hash computation, collision detection, hash combining.
"""
from typing import Dict, List, Tuple

def djb2_hash(data: str) -> int:
    h: int = 5381
    i: int = 0
    while i < len(data):
        h = ((h << 5) + h) + ord(data[i])
        h = h & 0xFFFFFFFF
        i += 1
    return h

def fnv1a_hash(data: str) -> int:
    h: int = 2166136261
    i: int = 0
    while i < len(data):
        h = h ^ ord(data[i])
        h = (h * 16777619) & 0xFFFFFFFF
        i += 1
    return h

def sdbm_hash(data: str) -> int:
    h: int = 0
    i: int = 0
    while i < len(data):
        c: int = ord(data[i])
        h = c + (h << 6) + (h << 16) - h
        h = h & 0xFFFFFFFF
        i += 1
    return h

def hash_combine(h1: int, h2: int) -> int:
    magic: int = 0x9E3779B9
    combined: int = h1 ^ (h2 + magic + (h1 << 6) + (h1 >> 2))
    return combined & 0xFFFFFFFF

def multi_hash_djb2(data: str) -> int:
    """Return DJB2 hash component for multi-hash."""
    h: int = 5381
    i: int = 0
    while i < len(data):
        h = ((h << 5) + h) + ord(data[i])
        h = h & 0xFFFFFFFF
        i += 1
    return h

def multi_hash_fnv(data: str) -> int:
    """Return FNV-1a hash component for multi-hash."""
    h: int = 2166136261
    i: int = 0
    while i < len(data):
        h = h ^ ord(data[i])
        h = (h * 16777619) & 0xFFFFFFFF
        i += 1
    return h

def multi_hash_sdbm(data: str) -> int:
    """Return SDBM hash component for multi-hash."""
    h: int = 0
    i: int = 0
    while i < len(data):
        c: int = ord(data[i])
        h = c + (h << 6) + (h << 16) - h
        h = h & 0xFFFFFFFF
        i += 1
    return h

def test_hashes() -> bool:
    ok: bool = True
    h1: int = djb2_hash("hello")
    if h1 == 0:
        ok = False
    h2: int = fnv1a_hash("hello")
    if h2 == 0:
        ok = False
    h3: int = sdbm_hash("hello")
    if h3 == 0:
        ok = False
    hc: int = hash_combine(h1, h2)
    if hc == 0:
        ok = False
    md: int = multi_hash_djb2("test")
    mf: int = multi_hash_fnv("test")
    ms: int = multi_hash_sdbm("test")
    if md == 0:
        ok = False
    if mf == 0:
        ok = False
    if ms == 0:
        ok = False
    return ok
