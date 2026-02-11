def sort_string(s: str) -> str:
    chars: list[int] = []
    i: int = 0
    n: int = len(s)
    while i < n:
        chars.append(ord(s[i]))
        i = i + 1
    a: int = 0
    while a < len(chars):
        b: int = a + 1
        while b < len(chars):
            if chars[b] < chars[a]:
                tmp: int = chars[a]
                chars[a] = chars[b]
                chars[b] = tmp
            b = b + 1
        a = a + 1
    result: str = ""
    k: int = 0
    while k < len(chars):
        result = result + chr(chars[k])
        k = k + 1
    return result

def is_anagram(s1: str, s2: str) -> int:
    if len(s1) != len(s2):
        return 0
    sorted1: str = sort_string(s1)
    sorted2: str = sort_string(s2)
    if sorted1 == sorted2:
        return 1
    return 0

def char_count_match(s1: str, s2: str) -> int:
    if len(s1) != len(s2):
        return 0
    counts: list[int] = []
    i: int = 0
    while i < 128:
        counts.append(0)
        i = i + 1
    j: int = 0
    while j < len(s1):
        idx: int = ord(s1[j])
        counts[idx] = counts[idx] + 1
        j = j + 1
    k: int = 0
    while k < len(s2):
        idx2: int = ord(s2[k])
        counts[idx2] = counts[idx2] - 1
        k = k + 1
    m: int = 0
    while m < 128:
        if counts[m] != 0:
            return 0
        m = m + 1
    return 1

def count_anagram_pairs(words: list[str]) -> int:
    n: int = len(words)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            w1: str = words[i]
            w2: str = words[j]
            chk: int = is_anagram(w1, w2)
            if chk == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count

def test_module() -> int:
    passed: int = 0
    r1: int = is_anagram("listen", "silent")
    if r1 == 1:
        passed = passed + 1
    r2: int = is_anagram("hello", "world")
    if r2 == 0:
        passed = passed + 1
    r3: int = char_count_match("anagram", "nagaram")
    if r3 == 1:
        passed = passed + 1
    r4: int = count_anagram_pairs(["eat", "tea", "ate", "bat"])
    if r4 == 3:
        passed = passed + 1
    r5: int = is_anagram("", "")
    if r5 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
