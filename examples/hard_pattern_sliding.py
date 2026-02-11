"""Sliding window: max sum window, longest unique substring, anagram detection.

Tests: max_sum_k, longest_unique, count_anagrams, min_window_length.
"""


def max_sum_window(arr: list[int], k_size: int) -> int:
    """Maximum sum of any window of size k."""
    n: int = len(arr)
    if n < k_size:
        return 0
    if k_size <= 0:
        return 0
    window_sum: int = 0
    i: int = 0
    while i < k_size:
        window_sum = window_sum + arr[i]
        i = i + 1
    best: int = window_sum
    j: int = k_size
    while j < n:
        window_sum = window_sum + arr[j] - arr[j - k_size]
        if window_sum > best:
            best = window_sum
        j = j + 1
    return best


def longest_unique_substr_len(s: str) -> int:
    """Length of longest substring with all unique characters."""
    n: int = len(s)
    if n == 0:
        return 0
    last_seen: dict[str, int] = {}
    best: int = 0
    left: int = 0
    right: int = 0
    while right < n:
        ch: str = s[right]
        if ch in last_seen:
            prev: int = last_seen[ch]
            if prev >= left:
                left = prev + 1
        last_seen[ch] = right
        window_len: int = right - left + 1
        if window_len > best:
            best = window_len
        right = right + 1
    return best


def count_char_freq(s: str, start: int, end: int) -> dict[str, int]:
    """Count character frequencies in s[start:end]."""
    freq: dict[str, int] = {}
    i: int = start
    while i < end:
        ch: str = s[i]
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
        i = i + 1
    return freq


def dicts_equal(a: dict[str, int], b: dict[str, int], ref_str: str, ref_len: int) -> int:
    """Check if two frequency dicts are equal for chars in ref string."""
    i: int = 0
    while i < ref_len:
        ch: str = ref_str[i]
        a_val: int = 0
        b_val: int = 0
        if ch in a:
            a_val = a[ch]
        if ch in b:
            b_val = b[ch]
        if a_val != b_val:
            return 0
        i = i + 1
    return 1


def count_anagram_occurrences(text: str, word: str) -> int:
    """Count occurrences of anagrams of word in text using sliding window."""
    tlen: int = len(text)
    wlen: int = len(word)
    if wlen > tlen:
        return 0
    if wlen == 0:
        return 0
    word_freq: dict[str, int] = count_char_freq(word, 0, wlen)
    window_freq: dict[str, int] = count_char_freq(text, 0, wlen)
    count: int = 0
    chk: int = dicts_equal(word_freq, window_freq, word, wlen)
    if chk == 1:
        count = count + 1
    i: int = wlen
    while i < tlen:
        new_ch: str = text[i]
        if new_ch in window_freq:
            window_freq[new_ch] = window_freq[new_ch] + 1
        else:
            window_freq[new_ch] = 1
        old_idx: int = i - wlen
        old_ch: str = text[old_idx]
        if old_ch in window_freq:
            window_freq[old_ch] = window_freq[old_ch] - 1
        chk2: int = dicts_equal(word_freq, window_freq, word, wlen)
        if chk2 == 1:
            count = count + 1
        i = i + 1
    return count


def min_window_containing(text: str, chars: str) -> int:
    """Length of minimum window in text containing all chars. 0 if impossible."""
    tlen: int = len(text)
    clen: int = len(chars)
    if clen == 0:
        return 0
    need: dict[str, int] = count_char_freq(chars, 0, clen)
    have: dict[str, int] = {}
    required: int = clen
    formed: int = 0
    best: int = tlen + 1
    left: int = 0
    right: int = 0
    while right < tlen:
        ch: str = text[right]
        if ch in have:
            have[ch] = have[ch] + 1
        else:
            have[ch] = 1
        if ch in need:
            if have[ch] <= need[ch]:
                formed = formed + 1
        while formed == required:
            window_sz: int = right - left + 1
            if window_sz < best:
                best = window_sz
            lch: str = text[left]
            if lch in have:
                have[lch] = have[lch] - 1
            if lch in need:
                if have[lch] < need[lch]:
                    formed = formed - 1
            left = left + 1
        right = right + 1
    if best > tlen:
        return 0
    return best


def test_module() -> int:
    """Test sliding window algorithms."""
    passed: int = 0

    if max_sum_window([1, 4, 2, 10, 2, 3, 1, 0, 20], 4) == 24:
        passed = passed + 1

    if longest_unique_substr_len("abcabcbb") == 3:
        passed = passed + 1

    if longest_unique_substr_len("bbbbb") == 1:
        passed = passed + 1

    ca: int = count_anagram_occurrences("cbaebabacd", "abc")
    if ca == 2:
        passed = passed + 1

    mw: int = min_window_containing("ADOBECODEBANC", "ABC")
    if mw == 4:
        passed = passed + 1

    if max_sum_window([1, 2, 3], 5) == 0:
        passed = passed + 1

    return passed
