"""Numerical methods: Signal processing primitives.

Tests: convolution, correlation, moving averages, filtering,
frequency domain approximations, window functions.
"""

from typing import List, Tuple


def moving_average(data: List[float], window: int) -> List[float]:
    """Compute simple moving average."""
    result: List[float] = []
    n: int = len(data)
    if window <= 0 or n == 0:
        return result
    i: int = 0
    while i <= n - window:
        total: float = 0.0
        j: int = 0
        while j < window:
            total = total + data[i + j]
            j += 1
        result.append(total / float(window))
        i += 1
    return result


def exponential_moving_avg(data: List[float], alpha: float) -> List[float]:
    """Compute exponential moving average."""
    result: List[float] = []
    if len(data) == 0:
        return result
    result.append(data[0])
    i: int = 1
    while i < len(data):
        ema: float = alpha * data[i] + (1.0 - alpha) * result[i - 1]
        result.append(ema)
        i += 1
    return result


def convolve_signals(signal: List[float], kern: List[float]) -> List[float]:
    """Linear convolution of signal with kernel."""
    ns: int = len(signal)
    nk: int = len(kern)
    if ns == 0 or nk == 0:
        return []
    n_out: int = ns + nk
    n_out = n_out - 1
    result: List[float] = []
    i: int = 0
    while i < n_out:
        result.append(0.0)
        i += 1
    i = 0
    while i < ns:
        j: int = 0
        while j < nk:
            result[i + j] = result[i + j] + signal[i] * kern[j]
            j += 1
        i += 1
    return result


def cross_correlate(a: List[float], b: List[float]) -> List[float]:
    """Cross-correlation of two signals."""
    na: int = len(a)
    nb: int = len(b)
    if na == 0 or nb == 0:
        return []
    n_out: int = na + nb
    n_out = n_out - 1
    result: List[float] = []
    i: int = 0
    while i < n_out:
        result.append(0.0)
        i += 1
    i = 0
    while i < na:
        j: int = 0
        while j < nb:
            result[i + j] = result[i + j] + a[i] * b[j]
            j += 1
        i += 1
    return result


def low_pass_filter(data: List[float], cutoff: float) -> List[float]:
    """Simple first-order IIR low-pass filter."""
    result: List[float] = []
    if len(data) == 0:
        return result
    result.append(data[0])
    i: int = 1
    while i < len(data):
        filtered: float = cutoff * data[i] + (1.0 - cutoff) * result[i - 1]
        result.append(filtered)
        i += 1
    return result


def high_pass_filter(data: List[float], cutoff: float) -> List[float]:
    """Simple first-order high-pass filter."""
    result: List[float] = []
    if len(data) == 0:
        return result
    result.append(data[0])
    i: int = 1
    while i < len(data):
        hp: float = (1.0 - cutoff) * (result[i - 1] + data[i] - data[i - 1])
        result.append(hp)
        i += 1
    return result


def autocorrelation(data: List[float], max_lag: int) -> List[float]:
    """Compute autocorrelation for lags 0 to max_lag."""
    n: int = len(data)
    result: List[float] = []
    m: float = 0.0
    for x in data:
        m = m + x
    m = m / float(n)
    lag: int = 0
    while lag <= max_lag and lag < n:
        total: float = 0.0
        i: int = 0
        while i < n - lag:
            total = total + (data[i] - m) * (data[i + lag] - m)
            i += 1
        result.append(total / float(n))
        lag += 1
    return result


def test_signal() -> bool:
    """Test signal processing functions."""
    ok: bool = True
    ma: List[float] = moving_average([1.0, 2.0, 3.0, 4.0, 5.0], 3)
    if len(ma) != 3:
        ok = False
    conv: List[float] = convolve_signals([1.0, 2.0, 3.0], [1.0, 1.0])
    if len(conv) != 4:
        ok = False
    lp: List[float] = low_pass_filter([1.0, 10.0, 1.0, 10.0], 0.5)
    if len(lp) != 4:
        ok = False
    return ok
