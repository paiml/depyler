"""Character frequency counting for Huffman encoding preparation."""


def count_frequencies(text: str) -> list[int]:
    """Count frequency of each ASCII char (0-127)."""
    freq: list[int] = []
    i: int = 0
    while i < 128:
        freq.append(0)
        i = i + 1
    i = 0
    n: int = len(text)
    while i < n:
        code: int = ord(text[i])
        if code < 128:
            freq[code] = freq[code] + 1
        i = i + 1
    return freq


def num_unique_chars(freq: list[int]) -> int:
    """Count number of unique characters."""
    count: int = 0
    i: int = 0
    while i < 128:
        if freq[i] > 0:
            count = count + 1
        i = i + 1
    return count


def most_frequent_code(freq: list[int]) -> int:
    """Return ASCII code of most frequent character."""
    best: int = 0
    best_count: int = 0
    i: int = 0
    while i < 128:
        if freq[i] > best_count:
            best_count = freq[i]
            best = i
        i = i + 1
    return best


def total_chars(freq: list[int]) -> int:
    """Return total character count."""
    total: int = 0
    i: int = 0
    while i < 128:
        total = total + freq[i]
        i = i + 1
    return total


def sorted_codes_by_freq(freq: list[int]) -> list[int]:
    """Return ASCII codes sorted by frequency descending."""
    codes: list[int] = []
    counts: list[int] = []
    i: int = 0
    while i < 128:
        if freq[i] > 0:
            codes.append(i)
            counts.append(freq[i])
        i = i + 1
    n: int = len(codes)
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if counts[j] > counts[i]:
                tc: int = counts[i]
                counts[i] = counts[j]
                counts[j] = tc
                tk: int = codes[i]
                codes[i] = codes[j]
                codes[j] = tk
            j = j + 1
        i = i + 1
    return codes


def entropy_estimate(freq: list[int], total: int) -> int:
    """Rough integer estimate of bits needed (sum of freq * log2-approx)."""
    bits: int = 0
    i: int = 0
    while i < 128:
        if freq[i] > 0:
            f: int = freq[i]
            log2: int = 0
            tmp: int = total // f
            while tmp > 1:
                tmp = tmp // 2
                log2 = log2 + 1
            bits = bits + f * log2
        i = i + 1
    return bits


def test_module() -> int:
    """Test huffman frequency operations."""
    passed: int = 0

    freq: list[int] = count_frequencies("aabbbcccc")
    if freq[97] == 2:
        passed = passed + 1

    if freq[98] == 3:
        passed = passed + 1

    if freq[99] == 4:
        passed = passed + 1

    if num_unique_chars(freq) == 3:
        passed = passed + 1

    if most_frequent_code(freq) == 99:
        passed = passed + 1

    if total_chars(freq) == 9:
        passed = passed + 1

    return passed
