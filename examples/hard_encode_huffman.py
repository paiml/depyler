def char_frequency(text: str) -> dict[str, int]:
    freq: dict[str, int] = {}
    n: int = len(text)
    i: int = 0
    while i < n:
        ch: str = text[i]
        if ch in freq:
            old: int = freq[ch]
            freq[ch] = old + 1
        else:
            freq[ch] = 1
        i = i + 1
    return freq

def sorted_frequencies(text: str) -> list[int]:
    freq: dict[str, int] = char_frequency(text)
    vals: list[int] = []
    n: int = len(text)
    i: int = 0
    while i < n:
        ch: str = text[i]
        if ch in freq:
            vals.append(freq[ch])
            freq[ch] = 0 - 1
        i = i + 1
    result: list[int] = []
    j: int = 0
    nv: int = len(vals)
    while j < nv:
        v: int = vals[j]
        if v > 0:
            result.append(v)
        j = j + 1
    k: int = 0
    nr: int = len(result)
    while k < nr - 1:
        m: int = k + 1
        while m < nr:
            rk: int = result[k]
            rm: int = result[m]
            if rk > rm:
                result[k] = rm
                result[m] = rk
            m = m + 1
        k = k + 1
    return result

def unique_chars(text: str) -> int:
    seen: dict[str, int] = {}
    n: int = len(text)
    i: int = 0
    while i < n:
        ch: str = text[i]
        seen[ch] = 1
        i = i + 1
    count: int = 0
    j: int = 0
    while j < n:
        ch2: str = text[j]
        if ch2 in seen:
            v: int = seen[ch2]
            if v == 1:
                count = count + 1
                seen[ch2] = 0
        j = j + 1
    return count

def entropy_estimate(text: str) -> float:
    n: int = len(text)
    freq: dict[str, int] = char_frequency(text)
    ent: float = 0.0
    i: int = 0
    while i < n:
        ch: str = text[i]
        if ch in freq:
            f: int = freq[ch]
            if f > 0:
                p: float = f * 1.0 / (n * 1.0)
                log_p: float = (p - 1.0) - 0.5 * (p - 1.0) * (p - 1.0)
                ent = ent - p * log_p
                freq[ch] = 0 - 1
        i = i + 1
    return ent

def test_module() -> int:
    passed: int = 0
    f: dict[str, int] = char_frequency("aab")
    fa: int = f["a"]
    if fa == 2:
        passed = passed + 1
    fb: int = f["b"]
    if fb == 1:
        passed = passed + 1
    uc: int = unique_chars("hello")
    if uc == 4:
        passed = passed + 1
    sf: list[int] = sorted_frequencies("aabbc")
    sf0: int = sf[0]
    if sf0 == 1:
        passed = passed + 1
    e: float = entropy_estimate("aaaa")
    if e >= 0.0:
        passed = passed + 1
    return passed
