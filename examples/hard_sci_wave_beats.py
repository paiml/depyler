"""Beat frequency and modulation computations using integer arithmetic.

Tests: beat frequency, envelope, modulation index, AM/FM basics.
Scale factor 1000 for fixed-point.
"""


def beat_frequency(freq1: int, freq2: int) -> int:
    """Beat frequency = |f1 - f2|."""
    diff: int = freq1 - freq2
    if diff < 0:
        diff = 0 - diff
    return diff


def beat_period(freq1: int, freq2: int) -> int:
    """Beat period = 1 / |f1 - f2|. Fixed-point scale 1000."""
    bf: int = beat_frequency(freq1, freq2)
    if bf == 0:
        return 0
    result: int = (1000 * 1000) // bf
    return result


def average_frequency(freq1: int, freq2: int) -> int:
    """Average (carrier) frequency = (f1 + f2) / 2."""
    return (freq1 + freq2) // 2


def envelope_amplitude(a1: int, a2: int, beat_phase: int) -> int:
    """Envelope amplitude at given beat phase.
    env = (a1 + a2) * |cos(beat_phase/2)|.
    cos approx: cos(x) ~ 1000 - x^2/2000 for small x.
    Fixed-point scale 1000."""
    half_phase: int = beat_phase // 2
    cos_val: int = 1000 - (half_phase * half_phase) // 2000
    if cos_val < 0:
        cos_val = 0 - cos_val
    result: int = ((a1 + a2) * cos_val) // 1000
    return result


def am_modulation_index(carrier_amp: int, mod_amp: int) -> int:
    """AM modulation index m = mod_amp / carrier_amp * 1000."""
    if carrier_amp == 0:
        return 0
    result: int = (mod_amp * 1000) // carrier_amp
    return result


def am_sideband_amplitude(carrier_amp: int, mod_index: int) -> int:
    """AM sideband amplitude = m * A_c / 2. Fixed-point scale 1000."""
    result: int = (mod_index * carrier_amp) // 2000
    return result


def am_power_ratio(mod_index: int) -> int:
    """AM total power / carrier power = 1 + m^2/2. Fixed-point scale 1000."""
    m_sq: int = (mod_index * mod_index) // 1000
    result: int = 1000 + m_sq // 2
    return result


def fm_deviation(sensitivity: int, mod_amplitude: int) -> int:
    """FM frequency deviation = k_f * A_m. Fixed-point scale 1000."""
    result: int = (sensitivity * mod_amplitude) // 1000
    return result


def fm_modulation_index(deviation: int, mod_freq: int) -> int:
    """FM modulation index beta = deviation / mod_freq * 1000."""
    if mod_freq == 0:
        return 0
    result: int = (deviation * 1000) // mod_freq
    return result


def fm_bandwidth_carson(deviation: int, mod_freq: int) -> int:
    """Carson's rule bandwidth = 2 * (deviation + mod_freq)."""
    return 2 * (deviation + mod_freq)


def test_module() -> int:
    """Test beat and modulation computations."""
    ok: int = 0
    bf: int = beat_frequency(440, 442)
    if bf == 2:
        ok = ok + 1
    bp: int = beat_period(440, 442)
    if bp == 500000:
        ok = ok + 1
    af: int = average_frequency(440, 442)
    if af == 441:
        ok = ok + 1
    mi: int = am_modulation_index(1000, 500)
    if mi == 500:
        ok = ok + 1
    sb: int = am_sideband_amplitude(1000, 500)
    if sb == 250:
        ok = ok + 1
    pr: int = am_power_ratio(1000)
    if pr == 1500:
        ok = ok + 1
    cb: int = fm_bandwidth_carson(75000, 15000)
    if cb == 180000:
        ok = ok + 1
    return ok
