"""Sort elements by frequency using dictionary-based counting."""


def frequency_count(arr: list[int]) -> dict[str, int]:
    """Build frequency map of array elements."""
    freq: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str in freq:
            freq[val_str] = freq[val_str] + 1
        else:
            freq[val_str] = 1
        idx = idx + 1
    return freq


def sort_by_frequency(arr: list[int]) -> list[int]:
    """Sort array by frequency (most frequent first). Stable within same frequency."""
    freq: dict[str, int] = frequency_count(arr)
    result: list[int] = []
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        result.append(arr[idx])
        idx = idx + 1
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            freq_i: int = freq[str(result[i])]
            freq_j: int = freq[str(result[j])]
            if freq_j > freq_i:
                temp: int = result[i]
                result[i] = result[j]
                result[j] = temp
            j = j + 1
        i = i + 1
    return result


def top_n_frequent(arr: list[int], n: int) -> list[int]:
    """Return the top n most frequent elements."""
    freq: dict[str, int] = frequency_count(arr)
    unique: list[int] = []
    seen: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str not in seen:
            unique.append(arr[idx])
            seen[val_str] = 1
        idx = idx + 1
    ui: int = 0
    ulen: int = len(unique)
    while ui < ulen:
        uj: int = ui + 1
        while uj < ulen:
            fi: int = freq[str(unique[ui])]
            fj: int = freq[str(unique[uj])]
            if fj > fi:
                temp: int = unique[ui]
                unique[ui] = unique[uj]
                unique[uj] = temp
            uj = uj + 1
        ui = ui + 1
    result: list[int] = []
    ri: int = 0
    while ri < n and ri < ulen:
        result.append(unique[ri])
        ri = ri + 1
    return result


def test_module() -> int:
    passed: int = 0

    freq: dict[str, int] = frequency_count([1, 2, 2, 3, 3, 3])
    if freq["3"] == 3:
        passed = passed + 1
    if freq["1"] == 1:
        passed = passed + 1

    sorted_arr: list[int] = sort_by_frequency([1, 2, 2, 3, 3, 3])
    if sorted_arr[0] == 3:
        passed = passed + 1

    topn: list[int] = top_n_frequent([1, 1, 2, 2, 2, 3], 2)
    if topn[0] == 2:
        passed = passed + 1
    if topn[1] == 1:
        passed = passed + 1

    if len(top_n_frequent([5, 5, 5], 1)) == 1:
        passed = passed + 1

    freq2: dict[str, int] = frequency_count([7, 7, 8])
    if freq2["7"] == 2:
        passed = passed + 1

    return passed
