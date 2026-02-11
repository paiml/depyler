"""Pathological hashing, encoding, and simple crypto algorithm patterns.

Tests: bitwise operations, modular arithmetic, large integer constants,
nested loops, integer overflow handling, XOR chains, shift operations,
table-based lookups, rolling computations, bit array manipulation.
"""

from typing import Dict, List, Optional, Tuple


def djb2_hash(data: str) -> int:
    """DJB2 hash function by Dan Bernstein."""
    h: int = 5381
    for ch in data:
        h = ((h << 5) + h) + ord(ch)
        h = h & 0xFFFFFFFF
    return h


def sdbm_hash(data: str) -> int:
    """SDBM hash function used in gawk."""
    h: int = 0
    for ch in data:
        c: int = ord(ch)
        h = c + (h << 6) + (h << 16) - h
        h = h & 0xFFFFFFFF
    return h


def fnv1a_hash(data: str) -> int:
    """FNV-1a hash function with 32-bit offset basis and prime."""
    offset_basis: int = 2166136261
    fnv_prime: int = 16777619
    h: int = offset_basis
    for ch in data:
        h = h ^ ord(ch)
        h = (h * fnv_prime) & 0xFFFFFFFF
    return h


def jenkins_one_at_a_time(data: str) -> int:
    """Jenkins one-at-a-time hash function."""
    h: int = 0
    for ch in data:
        h = (h + ord(ch)) & 0xFFFFFFFF
        h = (h + (h << 10)) & 0xFFFFFFFF
        h = (h ^ (h >> 6)) & 0xFFFFFFFF
    h = (h + (h << 3)) & 0xFFFFFFFF
    h = (h ^ (h >> 11)) & 0xFFFFFFFF
    h = (h + (h << 15)) & 0xFFFFFFFF
    return h


def rabin_karp_search(text: str, pattern: str) -> List[int]:
    """Find all occurrences of pattern in text using Rabin-Karp rolling hash."""
    results: List[int] = []
    n: int = len(text)
    m: int = len(pattern)
    if m > n or m == 0:
        return results
    base: int = 256
    mod: int = 1000000007
    pat_hash: int = 0
    txt_hash: int = 0
    power: int = 1
    for i in range(m - 1):
        power = (power * base) % mod
    for i in range(m):
        pat_hash = (pat_hash * base + ord(pattern[i])) % mod
        txt_hash = (txt_hash * base + ord(text[i])) % mod
    for i in range(n - m + 1):
        if pat_hash == txt_hash:
            match: bool = True
            for j in range(m):
                if text[i + j] != pattern[j]:
                    match = False
                    break
            if match:
                results.append(i)
        if i < n - m:
            txt_hash = (txt_hash - ord(text[i]) * power % mod + mod) % mod
            txt_hash = (txt_hash * base + ord(text[i + m])) % mod
    return results


def caesar_encrypt(plaintext: str, shift: int) -> str:
    """Caesar cipher encryption on letters a-z and A-Z."""
    result: List[str] = []
    for ch in plaintext:
        if ch >= "a" and ch <= "z":
            shifted: int = (ord(ch) - ord("a") + shift) % 26 + ord("a")
            result.append(chr(shifted))
        elif ch >= "A" and ch <= "Z":
            shifted2: int = (ord(ch) - ord("A") + shift) % 26 + ord("A")
            result.append(chr(shifted2))
        else:
            result.append(ch)
    return "".join(result)


def caesar_decrypt(ciphertext: str, shift: int) -> str:
    """Caesar cipher decryption (reverse shift)."""
    return caesar_encrypt(ciphertext, 26 - (shift % 26))


def vigenere_encrypt(plaintext: str, key: str) -> str:
    """Vigenere cipher encryption using a repeating key."""
    result: List[str] = []
    key_len: int = len(key)
    key_idx: int = 0
    for ch in plaintext:
        if ch >= "a" and ch <= "z":
            k: int = ord(key[key_idx % key_len]) - ord("a")
            shifted: int = (ord(ch) - ord("a") + k) % 26 + ord("a")
            result.append(chr(shifted))
            key_idx += 1
        elif ch >= "A" and ch <= "Z":
            k2: int = ord(key[key_idx % key_len]) - ord("a")
            shifted2: int = (ord(ch) - ord("A") + k2) % 26 + ord("A")
            result.append(chr(shifted2))
            key_idx += 1
        else:
            result.append(ch)
    return "".join(result)


def vigenere_decrypt(ciphertext: str, key: str) -> str:
    """Vigenere cipher decryption using a repeating key."""
    result: List[str] = []
    key_len: int = len(key)
    key_idx: int = 0
    for ch in ciphertext:
        if ch >= "a" and ch <= "z":
            k: int = ord(key[key_idx % key_len]) - ord("a")
            shifted: int = (ord(ch) - ord("a") - k + 26) % 26 + ord("a")
            result.append(chr(shifted))
            key_idx += 1
        elif ch >= "A" and ch <= "Z":
            k2: int = ord(key[key_idx % key_len]) - ord("a")
            shifted2: int = (ord(ch) - ord("A") - k2 + 26) % 26 + ord("A")
            result.append(chr(shifted2))
            key_idx += 1
        else:
            result.append(ch)
    return "".join(result)


def xor_cipher_repeating(data: str, key: str) -> List[int]:
    """XOR cipher with a repeating multi-byte key; reversible."""
    result: List[int] = []
    key_len: int = len(key)
    for i in range(len(data)):
        result.append(ord(data[i]) ^ ord(key[i % key_len]))
    return result


def xor_decipher_repeating(data: List[int], key: str) -> str:
    """Reverse repeating-key XOR cipher to recover plaintext."""
    result: List[str] = []
    key_len: int = len(key)
    for i in range(len(data)):
        result.append(chr(data[i] ^ ord(key[i % key_len])))
    return "".join(result)


def base64_encode(data: str) -> str:
    """Manual base64 encoding implementation using bit manipulation."""
    table: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    result: List[str] = []
    bytes_list: List[int] = []
    for ch in data:
        bytes_list.append(ord(ch))
    i: int = 0
    n: int = len(bytes_list)
    while i < n:
        b0: int = bytes_list[i]
        b1: int = bytes_list[i + 1] if i + 1 < n else 0
        b2: int = bytes_list[i + 2] if i + 2 < n else 0
        triple: int = (b0 << 16) | (b1 << 8) | b2
        result.append(table[(triple >> 18) & 0x3F])
        result.append(table[(triple >> 12) & 0x3F])
        if i + 1 < n:
            result.append(table[(triple >> 6) & 0x3F])
        else:
            result.append("=")
        if i + 2 < n:
            result.append(table[triple & 0x3F])
        else:
            result.append("=")
        i += 3
    return "".join(result)


def base64_decode(encoded: str) -> str:
    """Manual base64 decoding implementation using bit manipulation."""
    table: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    lookup: Dict[str, int] = {}
    for idx in range(len(table)):
        lookup[table[idx]] = idx
    result: List[str] = []
    i: int = 0
    n: int = len(encoded)
    while i < n:
        v0: int = lookup[encoded[i]] if encoded[i] != "=" else 0
        v1: int = lookup[encoded[i + 1]] if encoded[i + 1] != "=" else 0
        v2: int = lookup[encoded[i + 2]] if encoded[i + 2] != "=" else 0
        v3: int = lookup[encoded[i + 3]] if encoded[i + 3] != "=" else 0
        triple: int = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3
        result.append(chr((triple >> 16) & 0xFF))
        if encoded[i + 2] != "=":
            result.append(chr((triple >> 8) & 0xFF))
        if encoded[i + 3] != "=":
            result.append(chr(triple & 0xFF))
        i += 4
    return "".join(result)


def run_length_encode(data: str) -> str:
    """Run-length encoding: compress consecutive repeated characters."""
    if len(data) == 0:
        return ""
    result: List[str] = []
    count: int = 1
    prev: str = data[0]
    for i in range(1, len(data)):
        if data[i] == prev:
            count += 1
        else:
            result.append(str(count))
            result.append(prev)
            prev = data[i]
            count = 1
    result.append(str(count))
    result.append(prev)
    return "".join(result)


def run_length_decode(encoded: str) -> str:
    """Run-length decoding: expand compressed representation."""
    result: List[str] = []
    i: int = 0
    n: int = len(encoded)
    while i < n:
        num_str: str = ""
        while i < n and encoded[i] >= "0" and encoded[i] <= "9":
            num_str += encoded[i]
            i += 1
        if i < n and len(num_str) > 0:
            count: int = int(num_str)
            ch: str = encoded[i]
            for j in range(count):
                result.append(ch)
            i += 1
    return "".join(result)


def huffman_sorted_pairs(data: str) -> List[Tuple[str, int]]:
    """Count character frequencies and return sorted by count ascending."""
    freq: Dict[str, int] = {}
    for ch in data:
        if ch in freq:
            freq[ch] += 1
        else:
            freq[ch] = 1
    pairs: List[Tuple[str, int]] = []
    for ch in freq:
        pairs.append((ch, freq[ch]))
    n: int = len(pairs)
    for i in range(n):
        for j in range(0, n - i - 1):
            if pairs[j][1] > pairs[j + 1][1]:
                temp: Tuple[str, int] = pairs[j]
                pairs[j] = pairs[j + 1]
                pairs[j + 1] = temp
    return pairs


def crc32_compute(data: str) -> int:
    """Compute CRC32 checksum with inline table generation."""
    poly: int = 0xEDB88320
    table: List[int] = []
    for i in range(256):
        crc: int = i
        for j in range(8):
            if (crc & 1) != 0:
                crc = (crc >> 1) ^ poly
            else:
                crc = crc >> 1
        table.append(crc & 0xFFFFFFFF)
    crc_val: int = 0xFFFFFFFF
    for ch in data:
        byte: int = ord(ch) & 0xFF
        idx: int = (crc_val ^ byte) & 0xFF
        crc_val = (crc_val >> 8) ^ table[idx]
    return (crc_val ^ 0xFFFFFFFF) & 0xFFFFFFFF


def md5_round_functions(b: int, c: int, d: int) -> Tuple[int, int, int, int]:
    """All four MD5 round functions: F, G, H, I."""
    f_val: int = ((b & c) | ((~b) & d)) & 0xFFFFFFFF
    g_val: int = ((b & d) | (c & (~d))) & 0xFFFFFFFF
    h_val: int = (b ^ c ^ d) & 0xFFFFFFFF
    i_val: int = (c ^ (b | (~d))) & 0xFFFFFFFF
    return (f_val, g_val, h_val, i_val)


def md5_mix_step(a: int, b: int, f_val: int, k: int,
                 m_val: int, s: int) -> int:
    """Single MD5 mixing step: add, left-rotate, add."""
    a = (a + f_val + k + m_val) & 0xFFFFFFFF
    rotated: int = ((a << s) | (a >> (32 - s))) & 0xFFFFFFFF
    return (rotated + b) & 0xFFFFFFFF


def bloom_filter_ops(size: int, items: List[str],
                     query: str) -> Tuple[List[int], bool]:
    """Create bloom filter, insert items, then query for membership."""
    bf: List[int] = []
    for i in range(size):
        bf.append(0)
    for item in items:
        h1: int = djb2_hash(item) % size
        h2: int = fnv1a_hash(item) % size
        h3: int = jenkins_one_at_a_time(item) % size
        bf[h1] = 1
        bf[h2] = 1
        bf[h3] = 1
    q1: int = djb2_hash(query) % size
    q2: int = fnv1a_hash(query) % size
    q3: int = jenkins_one_at_a_time(query) % size
    found: bool = bf[q1] == 1 and bf[q2] == 1 and bf[q3] == 1
    return (bf, found)


def consistent_hash_distribute(keys: List[str], nodes: List[int],
                               ring_size: int) -> Dict[int, List[str]]:
    """Distribute keys to nearest clockwise node on a hash ring."""
    distribution: Dict[int, List[str]] = {}
    for node in nodes:
        distribution[node] = []
    for key in keys:
        pos: int = fnv1a_hash(key) % ring_size
        best_node: int = nodes[0]
        best_dist: int = ring_size + 1
        for node_pos in nodes:
            dist: int = (node_pos - pos + ring_size) % ring_size
            if dist < best_dist:
                best_dist = dist
                best_node = node_pos
        distribution[best_node].append(key)
    return distribution


def perfect_hash_small(keys: List[str]) -> Dict[str, int]:
    """Build a minimal perfect hash for a small set via linear probing."""
    n: int = len(keys)
    if n == 0:
        return {}
    table_size: int = n * 2
    result: Dict[str, int] = {}
    used: List[int] = []
    for i in range(table_size):
        used.append(0)
    for key in keys:
        h: int = djb2_hash(key) % table_size
        attempt: int = 0
        while used[h] == 1 and attempt < table_size:
            h = (h + 1) % table_size
            attempt += 1
        used[h] = 1
        result[key] = h
    return result


def detect_hash_collisions(strings: List[str]) -> List[Tuple[str, str]]:
    """Detect DJB2 hash collisions among a list of strings."""
    hash_map: Dict[int, str] = {}
    collisions: List[Tuple[str, str]] = []
    for s in strings:
        h: int = djb2_hash(s)
        if h in hash_map:
            collisions.append((hash_map[h], s))
        else:
            hash_map[h] = s
    return collisions


def multi_hash_fingerprint(data: str) -> Tuple[int, int, int, int]:
    """Compute a 4-hash fingerprint for stronger collision resistance."""
    h1: int = djb2_hash(data)
    h2: int = sdbm_hash(data)
    h3: int = fnv1a_hash(data)
    h4: int = jenkins_one_at_a_time(data)
    return (h1, h2, h3, h4)


def hash_combine_list(items: List[str]) -> int:
    """Hash a list of strings using Boost-style hash combine."""
    magic: int = 0x9E3779B9
    combined: int = 0
    for item in items:
        h: int = fnv1a_hash(item)
        combined = combined ^ (h + magic + (combined << 6) + (combined >> 2))
        combined = combined & 0xFFFFFFFF
    return combined


def bit_manipulation_suite(value: int) -> Tuple[int, int, int, int]:
    """Popcount, bit reversal, byte swap, and right rotate on 32-bit int."""
    v: int = value & 0xFFFFFFFF
    pop: int = 0
    tmp: int = v
    while tmp > 0:
        tmp = tmp & (tmp - 1)
        pop += 1
    rev: int = 0
    tmp2: int = v
    for i in range(32):
        rev = (rev << 1) | (tmp2 & 1)
        tmp2 = tmp2 >> 1
    rev = rev & 0xFFFFFFFF
    b0: int = (v >> 24) & 0xFF
    b1: int = (v >> 16) & 0xFF
    b2: int = (v >> 8) & 0xFF
    b3: int = v & 0xFF
    swapped: int = (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
    rotated: int = ((v >> 7) | (v << 25)) & 0xFFFFFFFF
    return (pop, rev, swapped, rotated)


def simple_checksum(data: str) -> int:
    """Internet-style ones-complement checksum over 16-bit words."""
    total: int = 0
    i: int = 0
    n: int = len(data)
    while i < n - 1:
        word: int = (ord(data[i]) << 8) | ord(data[i + 1])
        total = total + word
        i += 2
    if i < n:
        total = total + (ord(data[i]) << 8)
    while total > 0xFFFF:
        total = (total & 0xFFFF) + (total >> 16)
    return (~total) & 0xFFFF


def hash_to_hex(value: int) -> str:
    """Convert a 32-bit hash value to an 8-character hex string."""
    value = value & 0xFFFFFFFF
    hex_chars: str = "0123456789abcdef"
    result: List[str] = []
    for i in range(8):
        nibble: int = (value >> (28 - i * 4)) & 0xF
        result.append(hex_chars[nibble])
    return "".join(result)


def hex_to_hash(hex_str: str) -> int:
    """Convert an 8-character hex string to a 32-bit integer."""
    result: int = 0
    for ch in hex_str:
        result = result << 4
        if ch >= "0" and ch <= "9":
            result = result | (ord(ch) - ord("0"))
        elif ch >= "a" and ch <= "f":
            result = result | (ord(ch) - ord("a") + 10)
        elif ch >= "A" and ch <= "F":
            result = result | (ord(ch) - ord("A") + 10)
    return result & 0xFFFFFFFF


def hamming_distance_bytes(a: str, b: str) -> int:
    """Compute Hamming distance between equal-length strings at bit level."""
    if len(a) != len(b):
        return -1
    dist: int = 0
    for i in range(len(a)):
        xor_val: int = ord(a[i]) ^ ord(b[i])
        while xor_val > 0:
            dist += xor_val & 1
            xor_val = xor_val >> 1
    return dist


def sha_style_compress(a: int, b: int, c: int, d: int,
                       w: int, k: int) -> Tuple[int, int, int, int]:
    """One round of SHA-256 style compression with Ch, Maj, Sigma."""
    ch: int = ((a & b) ^ ((~a) & c)) & 0xFFFFFFFF
    maj: int = ((a & b) ^ (a & c) ^ (b & c)) & 0xFFFFFFFF
    s0: int = (((a >> 2) | (a << 30)) ^ ((a >> 13) | (a << 19))
               ^ ((a >> 22) | (a << 10))) & 0xFFFFFFFF
    s1: int = (((a >> 6) | (a << 26)) ^ ((a >> 11) | (a << 21))
               ^ ((a >> 25) | (a << 7))) & 0xFFFFFFFF
    t1: int = (d + s1 + ch + k + w) & 0xFFFFFFFF
    t2: int = (s0 + maj) & 0xFFFFFFFF
    new_a: int = (t1 + t2) & 0xFFFFFFFF
    new_d: int = (c + t1) & 0xFFFFFFFF
    return (new_a, b, c, new_d)


def test_all() -> bool:
    """Test all crypto and hash functions with known values."""
    ok: bool = True

    h1: int = djb2_hash("hello")
    if h1 == 0:
        ok = False

    h2: int = sdbm_hash("hello")
    if h2 == 0:
        ok = False

    h3: int = fnv1a_hash("hello")
    if h3 == 0:
        ok = False

    h4: int = jenkins_one_at_a_time("hello")
    if h4 == 0:
        ok = False

    positions: List[int] = rabin_karp_search("abcabcabc", "abc")
    if len(positions) != 3:
        ok = False

    enc: str = caesar_encrypt("hello", 3)
    dec: str = caesar_decrypt(enc, 3)
    if dec != "hello":
        ok = False

    venc: str = vigenere_encrypt("attackatdawn", "lemon")
    vdec: str = vigenere_decrypt(venc, "lemon")
    if vdec != "attackatdawn":
        ok = False

    xdata: List[int] = xor_cipher_repeating("secret", "key")
    xback: str = xor_decipher_repeating(xdata, "key")
    if xback != "secret":
        ok = False

    b64: str = base64_encode("Hello")
    decoded: str = base64_decode(b64)
    if decoded != "Hello":
        ok = False

    b64_2: str = base64_encode("Hi")
    decoded_2: str = base64_decode(b64_2)
    if decoded_2 != "Hi":
        ok = False

    rle: str = run_length_encode("aaabbc")
    if rle != "3a2b1c":
        ok = False

    rld: str = run_length_decode("3a2b1c")
    if rld != "aaabbc":
        ok = False

    pairs: List[Tuple[str, int]] = huffman_sorted_pairs("aabbc")
    if len(pairs) != 3:
        ok = False

    crc_val: int = crc32_compute("hello")
    if crc_val == 0:
        ok = False

    rounds: Tuple[int, int, int, int] = md5_round_functions(0xFF, 0x0F, 0xF0)
    if rounds[0] == 0:
        ok = False

    mix: int = md5_mix_step(0, 1, 2, 3, 4, 7)
    if mix == 0:
        ok = False

    bf_result: Tuple[List[int], bool] = bloom_filter_ops(
        64, ["hello", "world"], "hello")
    if not bf_result[1]:
        ok = False

    nodes: List[int] = [0, 90, 180, 270]
    keys: List[str] = ["a", "b", "c", "d", "e"]
    dist: Dict[int, List[str]] = consistent_hash_distribute(keys, nodes, 360)
    total_keys: int = 0
    for node in nodes:
        total_keys += len(dist[node])
    if total_keys != 5:
        ok = False

    phf: Dict[str, int] = perfect_hash_small(["cat", "dog", "fish"])
    if "cat" not in phf:
        ok = False

    collisions: List[Tuple[str, str]] = detect_hash_collisions(
        ["a", "b", "c", "d"])
    if len(collisions) < 0:
        ok = False

    fp: Tuple[int, int, int, int] = multi_hash_fingerprint("test")
    if fp[0] == 0 and fp[1] == 0:
        ok = False

    hl: int = hash_combine_list(["one", "two", "three"])
    if hl == 0:
        ok = False

    bits: Tuple[int, int, int, int] = bit_manipulation_suite(0xFF)
    if bits[0] != 8:
        ok = False

    cksum: int = simple_checksum("Hello World")
    if cksum == 0:
        ok = False

    hex_str: str = hash_to_hex(0xDEADBEEF)
    if hex_str != "deadbeef":
        ok = False

    back: int = hex_to_hash("deadbeef")
    if back != 0xDEADBEEF:
        ok = False

    hd: int = hamming_distance_bytes("abc", "axc")
    if hd < 1:
        ok = False

    sha_out: Tuple[int, int, int, int] = sha_style_compress(
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
        0x12345678, 0x428A2F98)
    if sha_out[0] == 0:
        ok = False

    return ok
