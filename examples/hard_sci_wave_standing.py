"""Standing wave computations using integer arithmetic.

Tests: harmonic frequencies, node positions, antinode positions, mode shapes.
Scale factor 1000 for fixed-point.
"""


def harmonic_frequency(fundamental: int, harmonic_num: int) -> int:
    """Compute nth harmonic frequency = n * f0."""
    return fundamental * harmonic_num


def string_fundamental(tension: int, linear_density: int, length: int) -> int:
    """Fundamental frequency of vibrating string.
    f = (1/(2L)) * sqrt(T/mu). Fixed-point scale 1000.
    Returns frequency * 1000."""
    if linear_density == 0 or length == 0:
        return 0
    ratio: int = (tension * 1000) // linear_density
    root: int = ratio
    guess: int = root
    iterations: int = 0
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + (ratio * 1000) // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            root = next_g
            break
        guess = next_g
        iterations = iterations + 1
    result: int = (root * 1000) // (2 * length)
    return result


def node_position(harmonic_num: int, length: int, node_idx: int) -> int:
    """Position of nth node in standing wave on string of given length.
    node_idx from 0 to harmonic_num. Fixed-point scale 1000."""
    if harmonic_num == 0:
        return 0
    result: int = (node_idx * length) // harmonic_num
    return result


def antinode_position(harmonic_num: int, length: int, antinode_idx: int) -> int:
    """Position of nth antinode in standing wave.
    Antinodes at (2n+1)*L/(2*harmonic). Fixed-point scale 1000."""
    if harmonic_num == 0:
        return 0
    numerator: int = (2 * antinode_idx + 1) * length
    result: int = numerator // (2 * harmonic_num)
    return result


def count_nodes(harmonic_num: int) -> int:
    """Number of nodes for nth harmonic (including endpoints)."""
    return harmonic_num + 1


def count_antinodes(harmonic_num: int) -> int:
    """Number of antinodes for nth harmonic."""
    return harmonic_num


def standing_wave_amplitude(incident: int, reflected: int) -> int:
    """Maximum amplitude of standing wave = incident + reflected."""
    return incident + reflected


def standing_wave_min_amplitude(incident: int, reflected: int) -> int:
    """Minimum amplitude of standing wave = |incident - reflected|."""
    diff: int = incident - reflected
    if diff < 0:
        diff = 0 - diff
    return diff


def resonant_wavelength(length: int, harmonic_num: int) -> int:
    """Resonant wavelength = 2L/n. Fixed-point scale 1000."""
    if harmonic_num == 0:
        return 0
    result: int = (2 * length) // harmonic_num
    return result


def test_module() -> int:
    """Test standing wave computations."""
    ok: int = 0
    freq2: int = harmonic_frequency(440, 2)
    if freq2 == 880:
        ok = ok + 1
    freq3: int = harmonic_frequency(440, 3)
    if freq3 == 1320:
        ok = ok + 1
    nodes: int = count_nodes(3)
    if nodes == 4:
        ok = ok + 1
    antinodes: int = count_antinodes(3)
    if antinodes == 3:
        ok = ok + 1
    node_pos: int = node_position(2, 1000, 1)
    if node_pos == 500:
        ok = ok + 1
    amp_max: int = standing_wave_amplitude(500, 500)
    if amp_max == 1000:
        ok = ok + 1
    amp_min: int = standing_wave_min_amplitude(500, 300)
    if amp_min == 200:
        ok = ok + 1
    rw: int = resonant_wavelength(1000, 2)
    if rw == 1000:
        ok = ok + 1
    return ok
