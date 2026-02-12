from typing import List, Tuple

def compute_mac(key: List[int], msg: List[int]) -> List[int]:
    state: List[int] = [0] * 8
    for i in range(len(key)):
        state[i % 8] = state[i % 8] ^ key[i]
    for i in range(len(msg)):
        idx: int = i % 8
        state[idx] = ((state[idx] * 31 + msg[i]) ^ state[(idx + 1) % 8]) & 0xFF
    for r in range(4):
        for idx2 in range(8):
            state[idx2] = ((state[idx2] + state[(idx2 + 3) % 8]) * 17) & 0xFF
    return state

def verify_mac(key: List[int], msg: List[int], mac: List[int]) -> bool:
    state: List[int] = [0] * 8
    for i in range(len(key)):
        state[i % 8] = state[i % 8] ^ key[i]
    for i in range(len(msg)):
        idx: int = i % 8
        state[idx] = ((state[idx] * 31 + msg[i]) ^ state[(idx + 1) % 8]) & 0xFF
    for r in range(4):
        for idx2 in range(8):
            state[idx2] = ((state[idx2] + state[(idx2 + 3) % 8]) * 17) & 0xFF
    diff: int = 0
    for i in range(8):
        if i < len(mac):
            diff = diff | (state[i] ^ mac[i])
    return diff == 0

def mac_encrypt(km: List[int], ke: List[int], msg: List[int]) -> List[int]:
    enc: List[int] = []
    for i in range(len(msg)):
        enc.append(msg[i] ^ ke[i % len(ke)])
    return enc

def mac_decrypt(ke: List[int], ct: List[int]) -> List[int]:
    dec: List[int] = []
    for i in range(len(ct)):
        dec.append(ct[i] ^ ke[i % len(ke)])
    return dec

def batch_mac(key: List[int], msgs: List[List[int]]) -> List[List[int]]:
    results: List[List[int]] = []
    for m in msgs:
        state: List[int] = [0] * 8
        for i in range(len(key)):
            state[i % 8] = state[i % 8] ^ key[i]
        for i in range(len(m)):
            idx: int = i % 8
            state[idx] = ((state[idx] * 31 + m[i]) ^ state[(idx + 1) % 8]) & 0xFF
        results.append(state)
    return results
