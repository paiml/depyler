"""Wave and signal operations.

Implements discrete wave generation and signal processing
operations using integer arithmetic approximations.
"""


def generate_square_wave(amplitude: int, period: int, length: int) -> list[int]:
    """Generate a square wave signal as integer array.

    Alternates between +amplitude and -amplitude every half period.
    """
    result: list[int] = []
    i: int = 0
    while i < length:
        phase: int = i % period
        half: int = period // 2
        if phase < half:
            result.append(amplitude)
        else:
            result.append(-amplitude)
        i = i + 1
    return result


def generate_triangle_wave(amplitude: int, period: int, length: int) -> list[int]:
    """Generate a triangle wave using integer arithmetic."""
    result: list[int] = []
    i: int = 0
    while i < length:
        phase: int = i % period
        half: int = period // 2
        if half == 0:
            result.append(0)
        else:
            if phase <= half:
                value: int = (2 * amplitude * phase) // half - amplitude
                result.append(value)
            else:
                value2: int = amplitude - (2 * amplitude * (phase - half)) // half
                result.append(value2)
        i = i + 1
    return result


def signal_energy(signal: list[int], length: int) -> int:
    """Compute the energy of a signal (sum of squared values)."""
    energy: int = 0
    i: int = 0
    while i < length:
        val: int = signal[i]
        energy = energy + val * val
        i = i + 1
    return energy


def signal_crossings(signal: list[int], length: int) -> int:
    """Count zero crossings in the signal."""
    crossings: int = 0
    i: int = 1
    while i < length:
        prev: int = signal[i - 1]
        curr: int = signal[i]
        if (prev > 0 and curr <= 0) or (prev <= 0 and curr > 0):
            crossings = crossings + 1
        i = i + 1
    return crossings


def signal_peak_amplitude(signal: list[int], length: int) -> int:
    """Find the peak amplitude (max absolute value) in signal."""
    peak: int = 0
    i: int = 0
    while i < length:
        val: int = signal[i]
        abs_val: int = val
        if abs_val < 0:
            abs_val = -abs_val
        if abs_val > peak:
            peak = abs_val
        i = i + 1
    return peak


def test_module() -> int:
    """Test wave function operations."""
    ok: int = 0

    tmp_sq: list[int] = generate_square_wave(5, 4, 8)
    if tmp_sq[0] == 5 and tmp_sq[2] == -5 and tmp_sq[4] == 5:
        ok = ok + 1

    energy: int = signal_energy(tmp_sq, 8)
    if energy == 200:
        ok = ok + 1

    crossings: int = signal_crossings(tmp_sq, 8)
    if crossings == 4:
        ok = ok + 1

    peak: int = signal_peak_amplitude(tmp_sq, 8)
    if peak == 5:
        ok = ok + 1

    return ok
