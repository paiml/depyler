def manacher_odd(s: str) -> list[int]:
    n: int = len(s)
    p: list[int] = []
    i: int = 0
    while i < n:
        p.append(0)
        i = i + 1
    if n == 0:
        return p
    p[0] = 0
    center: int = 0
    right: int = 0
    i = 1
    while i < n:
        mirror: int = 2 * center - i
        if i < right and mirror >= 0:
            diff: int = right - i
            if p[mirror] < diff:
                p[i] = p[mirror]
            else:
                p[i] = diff
        lo: int = i - p[i] - 1
        hi: int = i + p[i] + 1
        while lo >= 0 and hi < n and s[lo] == s[hi]:
            p[i] = p[i] + 1
            lo = lo - 1
            hi = hi + 1
        if i + p[i] > right:
            center = i
            right = i + p[i]
        i = i + 1
    return p

def longest_palindrome_length(s: str) -> int:
    p: list[int] = manacher_odd(s)
    best: int = 0
    i: int = 0
    while i < len(p):
        val: int = 2 * p[i] + 1
        if val > best:
            best = val
        i = i + 1
    return best

def count_palindrome_centers(s: str) -> int:
    p: list[int] = manacher_odd(s)
    count: int = 0
    i: int = 0
    while i < len(p):
        if p[i] > 0:
            count = count + 1
        i = i + 1
    return count

def is_full_palindrome(s: str) -> int:
    if longest_palindrome_length(s) == len(s):
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    if longest_palindrome_length("abacaba") == 7:
        passed = passed + 1
    if longest_palindrome_length("abc") == 1:
        passed = passed + 1
    if is_full_palindrome("aba") == 1:
        passed = passed + 1
    if is_full_palindrome("ab") == 0:
        passed = passed + 1
    if count_palindrome_centers("aaa") == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
