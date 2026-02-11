def sort_chars(s: str) -> str:
    chars: list[int] = []
    i: int = 0
    while i < len(s):
        chars.append(ord(s[i]))
        i = i + 1
    i = 0
    while i < len(chars):
        j: int = i + 1
        while j < len(chars):
            if chars[j] < chars[i]:
                tmp: int = chars[i]
                chars[i] = chars[j]
                chars[j] = tmp
            j = j + 1
        i = i + 1
    result: str = ""
    k: int = 0
    while k < len(chars):
        result = result + chr(chars[k])
        k = k + 1
    return result

def are_anagrams(a: str, b: str) -> int:
    ca: list[int] = []
    i: int = 0
    while i < len(a):
        ca.append(ord(a[i]))
        i = i + 1
    i = 0
    while i < len(ca):
        j: int = i + 1
        while j < len(ca):
            if ca[j] < ca[i]:
                tmp: int = ca[i]
                ca[i] = ca[j]
                ca[j] = tmp
            j = j + 1
        i = i + 1
    cb: list[int] = []
    i = 0
    while i < len(b):
        cb.append(ord(b[i]))
        i = i + 1
    i = 0
    while i < len(cb):
        j2: int = i + 1
        while j2 < len(cb):
            if cb[j2] < cb[i]:
                tmp2: int = cb[i]
                cb[i] = cb[j2]
                cb[j2] = tmp2
            j2 = j2 + 1
        i = i + 1
    if len(ca) != len(cb):
        return 0
    k: int = 0
    while k < len(ca):
        if ca[k] != cb[k]:
            return 0
        k = k + 1
    return 1

def count_anagram_groups(words: list[str]) -> int:
    sigs: list[str] = []
    i: int = 0
    while i < len(words):
        w: str = words[i]
        chars: list[int] = []
        j: int = 0
        while j < len(w):
            chars.append(ord(w[j]))
            j = j + 1
        j = 0
        while j < len(chars):
            k: int = j + 1
            while k < len(chars):
                if chars[k] < chars[j]:
                    tmp: int = chars[j]
                    chars[j] = chars[k]
                    chars[k] = tmp
                k = k + 1
            j = j + 1
        sig: str = ""
        m: int = 0
        while m < len(chars):
            sig = sig + chr(chars[m])
            m = m + 1
        sigs.append(sig)
        i = i + 1
    used: list[int] = []
    i = 0
    while i < len(words):
        used.append(0)
        i = i + 1
    groups: int = 0
    i = 0
    while i < len(words):
        if used[i] == 0:
            groups = groups + 1
            used[i] = 1
            j2: int = i + 1
            while j2 < len(words):
                if used[j2] == 0 and sigs[j2] == sigs[i]:
                    used[j2] = 1
                j2 = j2 + 1
        i = i + 1
    return groups

def test_module() -> int:
    passed: int = 0
    if are_anagrams("listen", "silent") == 1:
        passed = passed + 1
    if are_anagrams("abc", "abd") == 0:
        passed = passed + 1
    if sort_chars("cba") == "abc":
        passed = passed + 1
    if count_anagram_groups(["eat", "tea", "tan", "ate", "nat", "bat"]) == 3:
        passed = passed + 1
    if are_anagrams("", "") == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
