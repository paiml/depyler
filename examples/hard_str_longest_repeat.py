def longest_repeating_substring(s: str) -> int:
    n: int = len(s)
    best: int = 0
    length: int = n - 1
    while length >= 1:
        if length <= best:
            return best
        i: int = 0
        while i <= n - length:
            sub: str = s[i:i + length]
            j: int = i + 1
            while j <= n - length:
                if s[j:j + length] == sub:
                    if length > best:
                        best = length
                    j = n
                j = j + 1
            i = i + 1
        length = length - 1
    return best

def has_repeating(s: str, length: int) -> int:
    n: int = len(s)
    i: int = 0
    while i <= n - length:
        sub: str = s[i:i + length]
        j: int = i + 1
        while j <= n - length:
            if s[j:j + length] == sub:
                return 1
            j = j + 1
        i = i + 1
    return 0

def count_repeated_chars(s: str) -> int:
    freq: dict[str, int] = {}
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
        i = i + 1
    count: int = 0
    j: int = 0
    while j < len(s):
        ch2: str = s[j]
        if ch2 in freq:
            if freq[ch2] > 1:
                count = count + 1
                freq[ch2] = 0
        j = j + 1
    return count

def test_module() -> int:
    passed: int = 0
    if longest_repeating_substring("banana") == 3:
        passed = passed + 1
    if longest_repeating_substring("abcd") == 0:
        passed = passed + 1
    if has_repeating("abcabc", 3) == 1:
        passed = passed + 1
    if has_repeating("abcdef", 2) == 0:
        passed = passed + 1
    if count_repeated_chars("aabbc") == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
