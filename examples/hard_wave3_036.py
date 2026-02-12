"""Text processing: Character histogram and frequency analysis.

Tests: frequency counting, entropy estimation, chi-squared statistic,
character distribution analysis, IC calculation.
"""

from typing import Dict, List, Tuple


def char_histogram(text: str) -> Dict[str, int]:
    """Build histogram of character frequencies."""
    freq: Dict[str, int] = {}
    for ch in text:
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
    return freq


def unique_char_count(text: str) -> int:
    """Count number of unique characters."""
    seen: Dict[str, int] = {}
    for ch in text:
        seen[ch] = 1
    count: int = 0
    for k in seen:
        count += 1
    return count


def entropy_estimate(text: str) -> float:
    """Estimate Shannon entropy of text (simplified)."""
    n: int = len(text)
    if n == 0:
        return 0.0
    freq: Dict[str, int] = char_histogram(text)
    entropy: float = 0.0
    for ch in freq:
        p: float = float(freq[ch]) / float(n)
        if p > 0.0:
            log_p: float = 0.0
            x: float = p
            term: float = (x - 1.0)
            power: float = term
            k: int = 1
            while k <= 20:
                log_p = log_p + power / float(k)
                power = power * term * (-1.0)
                k += 1
            entropy = entropy - p * log_p
    return entropy


def index_of_coincidence(text: str) -> float:
    """Compute index of coincidence for text."""
    n: int = len(text)
    if n <= 1:
        return 0.0
    freq: Dict[str, int] = char_histogram(text)
    sum_fi: float = 0.0
    for ch in freq:
        fi: float = float(freq[ch])
        sum_fi = sum_fi + fi * (fi - 1.0)
    return sum_fi / (float(n) * float(n - 1))


def letter_frequencies(text: str) -> List[float]:
    """Count frequencies of letters a-z as fractions."""
    counts: List[int] = []
    i: int = 0
    while i < 26:
        counts.append(0)
        i += 1
    total: int = 0
    for ch in text:
        if ch >= "a" and ch <= "z":
            counts[ord(ch) - ord("a")] = counts[ord(ch) - ord("a")] + 1
            total += 1
        elif ch >= "A" and ch <= "Z":
            counts[ord(ch) - ord("A")] = counts[ord(ch) - ord("A")] + 1
            total += 1
    result: List[float] = []
    if total == 0:
        i = 0
        while i < 26:
            result.append(0.0)
            i += 1
        return result
    i = 0
    while i < 26:
        result.append(float(counts[i]) / float(total))
        i += 1
    return result


def chi_squared_english(text: str) -> float:
    """Chi-squared statistic comparing letter frequencies to English."""
    expected: List[float] = [0.082, 0.015, 0.028, 0.043, 0.127, 0.022,
                             0.020, 0.061, 0.070, 0.002, 0.008, 0.040,
                             0.024, 0.067, 0.075, 0.019, 0.001, 0.060,
                             0.063, 0.091, 0.028, 0.010, 0.023, 0.002,
                             0.020, 0.001]
    observed: List[float] = letter_frequencies(text)
    chi2: float = 0.0
    i: int = 0
    while i < 26:
        if expected[i] > 0.0:
            diff: float = observed[i] - expected[i]
            chi2 = chi2 + (diff * diff) / expected[i]
        i += 1
    return chi2


def test_frequency() -> bool:
    """Test frequency analysis functions."""
    ok: bool = True
    hist: Dict[str, int] = char_histogram("aabbc")
    if hist["a"] != 2:
        ok = False
    uc: int = unique_char_count("hello")
    if uc != 4:
        ok = False
    freqs: List[float] = letter_frequencies("abcabc")
    if len(freqs) != 26:
        ok = False
    return ok
