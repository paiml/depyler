def compress(s: str) -> str:
    n: int = len(s)
    if n == 0:
        return ""
    result: str = ""
    i: int = 0
    while i < n:
        ch: str = s[i]
        cnt: int = 1
        while i + cnt < n:
            nxt: str = s[i + cnt]
            if nxt == ch:
                cnt = cnt + 1
            else:
                break
            nxt = ""
        result = result + ch + int_to_str(cnt)
        i = i + cnt
    return result

def int_to_str(num: int) -> str:
    if num == 0:
        return "0"
    result: str = ""
    n: int = num
    while n > 0:
        digit: int = n % 10
        result = chr(digit + 48) + result
        n = n // 10
    return result

def decompress(s: str) -> str:
    n: int = len(s)
    result: str = ""
    i: int = 0
    while i < n:
        ch: str = s[i]
        i = i + 1
        num_str: str = ""
        while i < n:
            c: int = ord(s[i])
            if c >= 48 and c <= 57:
                num_str = num_str + s[i]
                i = i + 1
            else:
                break
        cnt: int = str_to_int(num_str)
        j: int = 0
        while j < cnt:
            result = result + ch
            j = j + 1
    return result

def str_to_int(s: str) -> int:
    result: int = 0
    i: int = 0
    n: int = len(s)
    while i < n:
        c: int = ord(s[i])
        result = result * 10 + c - 48
        i = i + 1
    return result

def run_length_size(s: str) -> int:
    compressed: str = compress(s)
    return len(compressed)

def test_module() -> int:
    passed: int = 0
    r1: str = compress("aabbbcccc")
    if r1 == "a2b3c4":
        passed = passed + 1
    r2: str = compress("abc")
    if r2 == "a1b1c1":
        passed = passed + 1
    r3: str = decompress("a2b3c4")
    if r3 == "aabbbcccc":
        passed = passed + 1
    r4: str = compress("")
    if r4 == "":
        passed = passed + 1
    r5: int = run_length_size("aaaa")
    if r5 == 2:
        passed = passed + 1
    r6: str = decompress("x1y1z1")
    if r6 == "xyz":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
