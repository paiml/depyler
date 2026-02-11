def word_break(s: str, words: list[str]) -> int:
    n: int = len(s)
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    j: int = 1
    while j <= n:
        w: int = 0
        nw: int = len(words)
        while w < nw:
            word: str = words[w]
            wlen: int = len(word)
            if wlen <= j:
                prev_idx: int = j - wlen
                if dp[prev_idx] == 1:
                    sub: str = s[prev_idx:j]
                    if sub == word:
                        dp[j] = 1
            w = w + 1
        j = j + 1
    return dp[n]

def can_segment(s: str, words: list[str]) -> int:
    n: int = len(s)
    if n == 0:
        return 1
    memo: list[int] = []
    i: int = 0
    while i <= n:
        memo.append(0 - 1)
        i = i + 1
    return segment_helper(s, words, 0, memo)

def segment_helper(s: str, words: list[str], start: int, memo: list[int]) -> int:
    n: int = len(s)
    if start == n:
        return 1
    if memo[start] != 0 - 1:
        return memo[start]
    w: int = 0
    nw: int = len(words)
    while w < nw:
        word: str = words[w]
        wlen: int = len(word)
        end: int = start + wlen
        if end <= n:
            sub: str = s[start:end]
            if sub == word:
                rest: int = segment_helper(s, words, end, memo)
                if rest == 1:
                    memo[start] = 1
                    return 1
        w = w + 1
    memo[start] = 0
    return 0

def min_word_breaks(s: str, words: list[str]) -> int:
    n: int = len(s)
    dp: list[int] = []
    big: int = n + 1
    i: int = 0
    while i <= n:
        dp.append(big)
        i = i + 1
    dp[0] = 0
    j: int = 1
    while j <= n:
        w: int = 0
        nw: int = len(words)
        while w < nw:
            word: str = words[w]
            wlen: int = len(word)
            if wlen <= j:
                prev_idx: int = j - wlen
                sub: str = s[prev_idx:j]
                if sub == word:
                    candidate: int = dp[prev_idx] + 1
                    if candidate < dp[j]:
                        dp[j] = candidate
            w = w + 1
        j = j + 1
    if dp[n] >= big:
        return 0 - 1
    return dp[n]

def test_module() -> int:
    passed: int = 0
    r1: int = word_break("leetcode", ["leet", "code"])
    if r1 == 1:
        passed = passed + 1
    r2: int = word_break("catsandog", ["cats", "dog", "sand", "and", "cat"])
    if r2 == 0:
        passed = passed + 1
    r3: int = can_segment("applepenapple", ["apple", "pen"])
    if r3 == 1:
        passed = passed + 1
    r4: int = min_word_breaks("leetcode", ["leet", "code"])
    if r4 == 2:
        passed = passed + 1
    r5: int = word_break("", ["a", "b"])
    if r5 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
