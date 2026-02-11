"""Inductor computations using integer arithmetic.

Tests: inductance, energy, mutual inductance, solenoid.
Scale factor 1000 for fixed-point.
"""


def solenoid_inductance(turns: int, area: int, length: int, mu_r: int) -> int:
    """Solenoid inductance L = mu_r * mu_0 * N^2 * A / l.
    mu_0 = 1257 (scale 10^-6 * 10^9 = 10^3). Scale 1000."""
    if length == 0:
        return 0
    n_sq: int = (turns * turns) // 1000
    result: int = (mu_r * 1257 * n_sq * area) // (length * 1000 * 1000)
    return result


def inductor_energy_stored(inductance: int, current_val: int) -> int:
    """Energy E = 0.5*L*I^2. Scale 1000."""
    i_sq: int = (current_val * current_val) // 1000
    result: int = (inductance * i_sq) // 2000
    return result


def inductive_reactance_calc(inductance: int, frequency: int) -> int:
    """Inductive reactance XL = 2*pi*f*L. 2*pi ~ 6283. Scale 1000."""
    result: int = (6283 * frequency * inductance) // (1000 * 1000)
    return result


def mutual_inductance_calc(coupling_coeff: int, l1: int, l2: int) -> int:
    """Mutual inductance M = k*sqrt(L1*L2). Scale 1000."""
    product: int = (l1 * l2) // 1000
    if product <= 0:
        return 0
    guess: int = product
    iterations: int = 0
    target: int = product * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return (coupling_coeff * next_g) // 1000
        guess = next_g
        iterations = iterations + 1
    return (coupling_coeff * guess) // 1000


def series_inductance(inductances: list[int]) -> int:
    """Series inductance (no mutual): L = L1 + L2 + ..."""
    total: int = 0
    i: int = 0
    while i < len(inductances):
        val: int = inductances[i]
        total = total + val
        i = i + 1
    return total


def parallel_inductance_two(l1: int, l2: int) -> int:
    """Parallel inductance: L = L1*L2/(L1+L2). Scale 1000."""
    denom: int = l1 + l2
    if denom == 0:
        return 0
    result: int = (l1 * l2) // denom
    return result


def induced_emf(inductance: int, di_dt: int) -> int:
    """Induced EMF: V = L * dI/dt. Scale 1000."""
    result: int = (inductance * di_dt) // 1000
    return result


def magnetic_energy_density(mag_field: int, mu_r: int) -> int:
    """Magnetic energy density: u = B^2/(2*mu_r*mu_0).
    mu_0 = 1257. Scale 1000."""
    b_sq: int = (mag_field * mag_field) // 1000
    denom: int = (2 * mu_r * 1257) // 1000
    if denom == 0:
        return 0
    result: int = (b_sq * 1000) // denom
    return result


def time_constant_rl(inductance: int, resistance: int) -> int:
    """RL time constant tau = L/R. Scale 1000."""
    if resistance == 0:
        return 0
    result: int = (inductance * 1000) // resistance
    return result


def test_module() -> int:
    """Test inductor computations."""
    ok: int = 0
    e: int = inductor_energy_stored(2000, 3000)
    if e == 9000:
        ok = ok + 1
    sl: int = series_inductance([1000, 2000, 3000])
    if sl == 6000:
        ok = ok + 1
    pl: int = parallel_inductance_two(2000, 2000)
    if pl == 1000:
        ok = ok + 1
    emf: int = induced_emf(1000, 500)
    if emf == 500:
        ok = ok + 1
    tau: int = time_constant_rl(2000, 1000)
    if tau == 2000:
        ok = ok + 1
    pl_zero: int = parallel_inductance_two(0, 0)
    if pl_zero == 0:
        ok = ok + 1
    return ok
