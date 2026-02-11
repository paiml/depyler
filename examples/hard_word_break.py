"""Word break problem: determine if a string can be segmented into dictionary words.

Tests: segmentable strings, non-segmentable, empty string, single word, multiple ways.
"""


def word_break(s: str, word_list: list[str]) -> int:
    """Return 1 if s can be segmented into words from word_list, 0 otherwise."""
    n: int = len(s)
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    i = 1
    while i <= n:
        j: int = 0
        while j < i:
            if dp[j] == 1:
                substr: str = s[j:i]
                k: int = 0
                while k < len(word_list):
                    if word_list[k] == substr:
                        dp[i] = 1
                    k = k + 1
            j = j + 1
        i = i + 1
    return dp[n]


def count_word_breaks(s: str, word_list: list[str]) -> int:
    """Return number of ways to segment s into dictionary words."""
    n: int = len(s)
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    i = 1
    while i <= n:
        j: int = 0
        while j < i:
            if dp[j] > 0:
                substr: str = s[j:i]
                k: int = 0
                while k < len(word_list):
                    if word_list[k] == substr:
                        dp[i] = dp[i] + dp[j]
                    k = k + 1
            j = j + 1
        i = i + 1
    return dp[n]


def min_extra_chars(s: str, word_list: list[str]) -> int:
    """Return minimum number of characters left over after word break."""
    n: int = len(s)
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(i)
        i = i + 1
    i = 1
    while i <= n:
        dp[i] = dp[i - 1] + 1
        j: int = 0
        while j < i:
            substr: str = s[j:i]
            k: int = 0
            while k < len(word_list):
                if word_list[k] == substr:
                    if dp[j] < dp[i]:
                        dp[i] = dp[j]
                k = k + 1
            j = j + 1
        i = i + 1
    return dp[n]


def test_module() -> int:
    """Test word break problem."""
    ok: int = 0

    words1: list[str] = ["leet", "code"]
    if word_break("leetcode", words1) == 1:
        ok = ok + 1

    words2: list[str] = ["apple", "pen"]
    if word_break("applepenapple", words2) == 1:
        ok = ok + 1

    words3: list[str] = ["cats", "dog", "sand", "and", "cat"]
    if word_break("catsandog", words3) == 0:
        ok = ok + 1

    if word_break("", words1) == 1:
        ok = ok + 1

    words4: list[str] = ["a", "b"]
    if word_break("ab", words4) == 1:
        ok = ok + 1

    words5: list[str] = ["a", "aa", "aaa"]
    if count_word_breaks("aaa", words5) == 4:
        ok = ok + 1

    words6: list[str] = ["cat", "cats", "and", "sand"]
    if word_break("catsand", words6) == 1:
        ok = ok + 1

    if min_extra_chars("leetcode", words1) == 0:
        ok = ok + 1

    words7: list[str] = ["hello"]
    if min_extra_chars("helloworld", words7) == 5:
        ok = ok + 1

    if word_break("a", words4) == 1:
        ok = ok + 1

    return ok
