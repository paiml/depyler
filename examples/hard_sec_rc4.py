from typing import List, Tuple

def rc4_init(key: List[int]) -> List[int]:
    s: List[int] = []
    for i in range(256):
        s.append(i)
    j: int = 0
    kl: int = len(key)
    for i in range(256):
        j = (j + s[i] + key[i % kl]) % 256
        temp: int = s[i]
        s[i] = s[j]
        s[j] = temp
    return s

def rc4_encrypt(key: List[int], plaintext: List[int]) -> List[int]:
    s: List[int] = []
    for i in range(256):
        s.append(i)
    j: int = 0
    kl: int = len(key)
    for i in range(256):
        j = (j + s[i] + key[i % kl]) % 256
        temp: int = s[i]
        s[i] = s[j]
        s[j] = temp
    ii: int = 0
    jj: int = 0
    ct: List[int] = []
    for k in range(len(plaintext)):
        ii = (ii + 1) % 256
        jj = (jj + s[ii]) % 256
        temp2: int = s[ii]
        s[ii] = s[jj]
        s[jj] = temp2
        idx: int = (s[ii] + s[jj]) % 256
        ct.append(plaintext[k] ^ s[idx])
    return ct

def rc4_decrypt(key: List[int], ct: List[int]) -> List[int]:
    s: List[int] = []
    for i in range(256):
        s.append(i)
    j: int = 0
    kl: int = len(key)
    for i in range(256):
        j = (j + s[i] + key[i % kl]) % 256
        temp: int = s[i]
        s[i] = s[j]
        s[j] = temp
    ii: int = 0
    jj: int = 0
    pt: List[int] = []
    for k in range(len(ct)):
        ii = (ii + 1) % 256
        jj = (jj + s[ii]) % 256
        temp2: int = s[ii]
        s[ii] = s[jj]
        s[jj] = temp2
        idx: int = (s[ii] + s[jj]) % 256
        pt.append(ct[k] ^ s[idx])
    return pt

def rc4_keystream(key: List[int], length: int) -> List[int]:
    s: List[int] = []
    for i in range(256):
        s.append(i)
    j: int = 0
    kl: int = len(key)
    for i in range(256):
        j = (j + s[i] + key[i % kl]) % 256
        temp: int = s[i]
        s[i] = s[j]
        s[j] = temp
    ii: int = 0
    jj: int = 0
    stream: List[int] = []
    for k in range(length):
        ii = (ii + 1) % 256
        jj = (jj + s[ii]) % 256
        temp2: int = s[ii]
        s[ii] = s[jj]
        s[jj] = temp2
        idx: int = (s[ii] + s[jj]) % 256
        stream.append(s[idx])
    return stream

def rc4_stats(key: List[int], count: int) -> List[int]:
    counts: List[int] = [0] * 256
    s: List[int] = []
    for i in range(256):
        s.append(i)
    j: int = 0
    kl: int = len(key)
    for i in range(256):
        j = (j + s[i] + key[i % kl]) % 256
        temp: int = s[i]
        s[i] = s[j]
        s[j] = temp
    ii: int = 0
    jj: int = 0
    for k in range(count):
        ii = (ii + 1) % 256
        jj = (jj + s[ii]) % 256
        temp2: int = s[ii]
        s[ii] = s[jj]
        s[jj] = temp2
        idx: int = (s[ii] + s[jj]) % 256
        counts[s[idx]] = counts[s[idx]] + 1
    return counts
