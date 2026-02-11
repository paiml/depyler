"""Simple encoding: Caesar cipher, ROT13, run-length encoding on strings."""


def caesar_encode(text: str, shift: int) -> str:
    """Encode text with Caesar cipher."""
    result: str = ""
    i: int = 0
    while i < len(text):
        code: int = ord(text[i])
        if code >= 65 and code <= 90:
            code = ((code - 65 + shift) % 26) + 65
            result = result + chr(code)
        elif code >= 97 and code <= 122:
            code = ((code - 97 + shift) % 26) + 97
            result = result + chr(code)
        else:
            result = result + text[i]
        i = i + 1
    return result


def caesar_decode(text: str, shift: int) -> str:
    """Decode Caesar cipher."""
    return caesar_encode(text, 26 - (shift % 26))


def rot13(text: str) -> str:
    """Apply ROT13 encoding."""
    return caesar_encode(text, 13)


def rle_encode_str(s: str) -> str:
    """Run-length encode: 'aaabbc' -> 'a3b2c1'."""
    if len(s) == 0:
        return ""
    result: str = ""
    count: int = 1
    i: int = 1
    while i < len(s):
        if s[i] == s[i - 1]:
            count = count + 1
        else:
            result = result + s[i - 1]
            cnt_str: str = digit_to_str(count)
            result = result + cnt_str
            count = 1
        i = i + 1
    last_idx: int = len(s) - 1
    result = result + s[last_idx]
    final_cnt: str = digit_to_str(count)
    result = result + final_cnt
    return result


def digit_to_str(n: int) -> str:
    """Convert a small positive integer to string."""
    if n == 0:
        return "0"
    result: str = ""
    val: int = n
    while val > 0:
        d: int = val % 10
        result = chr(48 + d) + result
        val = val // 10
    return result


def rle_decode_str(encoded: str) -> str:
    """Decode run-length encoded string."""
    result: str = ""
    i: int = 0
    while i < len(encoded):
        ch: str = encoded[i]
        i = i + 1
        num_str: str = ""
        while i < len(encoded):
            code: int = ord(encoded[i])
            if code >= 48 and code <= 57:
                num_str = num_str + encoded[i]
                i = i + 1
            else:
                i = i
                break
        count: int = 0
        j: int = 0
        while j < len(num_str):
            count = count * 10 + (ord(num_str[j]) - 48)
            j = j + 1
        k: int = 0
        while k < count:
            result = result + ch
            k = k + 1
    return result


def atbash_cipher(text: str) -> str:
    """Atbash cipher: a->z, b->y, etc."""
    result: str = ""
    i: int = 0
    while i < len(text):
        code: int = ord(text[i])
        if code >= 65 and code <= 90:
            result = result + chr(90 - (code - 65))
        elif code >= 97 and code <= 122:
            result = result + chr(122 - (code - 97))
        else:
            result = result + text[i]
        i = i + 1
    return result


def simple_xor_str(text: str, mask: int) -> str:
    """XOR each character code with mask."""
    result: str = ""
    i: int = 0
    while i < len(text):
        code: int = ord(text[i])
        new_code: int = code ^ mask
        if new_code < 32:
            new_code = 32
        if new_code > 126:
            new_code = 126
        result = result + chr(new_code)
        i = i + 1
    return result


def test_module() -> int:
    """Test all encoding functions."""
    passed: int = 0
    enc: str = caesar_encode("hello", 3)
    if enc == "khoor":
        passed = passed + 1
    dec: str = caesar_decode(enc, 3)
    if dec == "hello":
        passed = passed + 1
    r13: str = rot13("hello")
    if r13 == "uryyb":
        passed = passed + 1
    r13_back: str = rot13(r13)
    if r13_back == "hello":
        passed = passed + 1
    rle: str = rle_encode_str("aaabbc")
    if rle == "a3b2c1":
        passed = passed + 1
    rle_dec: str = rle_decode_str("a3b2c1")
    if rle_dec == "aaabbc":
        passed = passed + 1
    atb: str = atbash_cipher("abc")
    if atb == "zyx":
        passed = passed + 1
    atb2: str = atbash_cipher(atb)
    if atb2 == "abc":
        passed = passed + 1
    if caesar_encode("", 5) == "":
        passed = passed + 1
    if rle_encode_str("x") == "x1":
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
