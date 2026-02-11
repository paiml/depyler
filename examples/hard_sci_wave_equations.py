"""Wave equation computations using fixed-point integer arithmetic.

Tests: wave speed, wavelength, frequency, period, amplitude.
Scale factor 1000 used for fixed-point representation.
"""


def wave_speed(frequency: int, wavelength: int) -> int:
    """Compute wave speed = frequency * wavelength (fixed-point scale 1000)."""
    result: int = (frequency * wavelength) // 1000
    return result


def wavelength_from_speed(speed: int, frequency: int) -> int:
    """Compute wavelength = speed / frequency (fixed-point scale 1000)."""
    if frequency == 0:
        return 0
    result: int = (speed * 1000) // frequency
    return result


def frequency_from_period(period: int) -> int:
    """Compute frequency = 1 / period (fixed-point scale 1000)."""
    if period == 0:
        return 0
    result: int = (1000 * 1000) // period
    return result


def wave_number(wavelength: int) -> int:
    """Compute wave number k = 2*pi/lambda approx 6283/lambda (fixed-point)."""
    if wavelength == 0:
        return 0
    result: int = (6283 * 1000) // wavelength
    return result


def angular_frequency(frequency: int) -> int:
    """Compute omega = 2*pi*f approx 6283*f/1000 (fixed-point)."""
    result: int = (6283 * frequency) // 1000
    return result


def wave_displacement(amplitude: int, omega_t: int, kx: int) -> int:
    """Compute y = A * sin(omega*t - k*x) using integer sin approx.
    Uses a simple triangle-wave approximation for sin.
    All values in fixed-point scale 1000."""
    phase: int = omega_t - kx
    if phase < 0:
        phase = 0 - phase
    phase = phase % 6283
    if phase > 3141:
        phase = 6283 - phase
    sin_val: int = (4000 * phase * (3141 - phase)) // (3141 * 3141)
    result: int = (amplitude * sin_val) // 1000
    return result


def wave_energy_density(amplitude: int, omega: int, density: int) -> int:
    """Energy density = 0.5 * rho * omega^2 * A^2 (fixed-point scale 1000)."""
    omega_sq: int = (omega * omega) // 1000
    amp_sq: int = (amplitude * amplitude) // 1000
    result: int = (density * omega_sq) // 1000
    result = (result * amp_sq) // 2000
    return result


def superposition_amplitude(a1: int, a2: int, phase_diff: int) -> int:
    """Resultant amplitude from superposition of two waves.
    A = sqrt(A1^2 + A2^2 + 2*A1*A2*cos(phase_diff)).
    Uses cos approx: cos(x) ~ 1 - x^2/2 for small x (fixed-point)."""
    cos_val: int = 1000 - (phase_diff * phase_diff) // 2000
    a1_sq: int = (a1 * a1) // 1000
    a2_sq: int = (a2 * a2) // 1000
    cross: int = (2 * a1 * a2) // 1000
    cross = (cross * cos_val) // 1000
    sum_sq: int = a1_sq + a2_sq + cross
    if sum_sq < 0:
        return 0
    result: int = sum_sq
    guess: int = result
    iterations: int = 0
    while iterations < 50:
        if guess == 0:
            return 0
        next_guess: int = (guess + (sum_sq * 1000) // guess) // 2
        diff: int = next_guess - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return next_guess
        guess = next_guess
        iterations = iterations + 1
    return guess


def test_module() -> int:
    """Test wave equation computations."""
    ok: int = 0
    speed: int = wave_speed(500, 2000)
    if speed == 1000:
        ok = ok + 1
    wl: int = wavelength_from_speed(3000, 500)
    if wl == 6000:
        ok = ok + 1
    freq: int = frequency_from_period(2000)
    if freq == 500:
        ok = ok + 1
    wn: int = wave_number(1000)
    if wn == 6283:
        ok = ok + 1
    omega: int = angular_frequency(1000)
    if omega == 6283:
        ok = ok + 1
    e_dens: int = wave_energy_density(1000, 1000, 1000)
    if e_dens == 500:
        ok = ok + 1
    zero_wl: int = wavelength_from_speed(1000, 0)
    if zero_wl == 0:
        ok = ok + 1
    return ok
