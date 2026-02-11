"""RL circuit computations using integer arithmetic.

Tests: time constant, current growth/decay, energy stored.
Scale factor 1000 for fixed-point.
"""


def rl_time_constant(inductance: int, resistance: int) -> int:
    """Time constant tau = L/R. Scale 1000."""
    if resistance == 0:
        return 0
    result: int = (inductance * 1000) // resistance
    return result


def rl_current_growth(i_max: int, elapsed: int, tau: int) -> int:
    """Current growth: I(t) = I_max*(1 - exp(-t/tau)).
    exp(-x) ~ 1 - x + x^2/2 for small x. Scale 1000."""
    if tau == 0:
        return i_max
    x: int = (elapsed * 1000) // tau
    if x > 5000:
        return i_max
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (i_max * (1000 - exp_val)) // 1000
    return result


def rl_current_decay(i_initial: int, elapsed: int, tau: int) -> int:
    """Current decay: I(t) = I0*exp(-t/tau). Scale 1000."""
    if tau == 0:
        return 0
    x: int = (elapsed * 1000) // tau
    if x > 5000:
        return 0
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (i_initial * exp_val) // 1000
    return result


def inductor_energy(inductance: int, current_val: int) -> int:
    """Energy stored in inductor: E = 0.5*L*I^2. Scale 1000."""
    i_sq: int = (current_val * current_val) // 1000
    result: int = (inductance * i_sq) // 2000
    return result


def back_emf(inductance: int, di_dt: int) -> int:
    """Back EMF: V = -L * dI/dt. Returns magnitude. Scale 1000."""
    result: int = (inductance * di_dt) // 1000
    if result < 0:
        result = 0 - result
    return result


def mutual_inductance(coupling: int, l1: int, l2: int) -> int:
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
            return (coupling * next_g) // 1000
        guess = next_g
        iterations = iterations + 1
    return (coupling * guess) // 1000


def inductive_reactance(inductance: int, frequency: int) -> int:
    """Inductive reactance: XL = 2*pi*f*L. 2*pi ~ 6283. Scale 1000."""
    result: int = (6283 * frequency * inductance) // (1000 * 1000)
    return result


def rl_impedance_magnitude(resistance: int, reactance: int) -> int:
    """RL impedance magnitude: Z = sqrt(R^2 + XL^2). Scale 1000."""
    r_sq: int = (resistance * resistance) // 1000
    x_sq: int = (reactance * reactance) // 1000
    sum_sq: int = r_sq + x_sq
    if sum_sq <= 0:
        return 0
    guess: int = sum_sq
    iterations: int = 0
    target: int = sum_sq * 1000
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return next_g
        guess = next_g
        iterations = iterations + 1
    return guess


def test_module() -> int:
    """Test RL circuit computations."""
    ok: int = 0
    tau: int = rl_time_constant(2000, 1000)
    if tau == 2000:
        ok = ok + 1
    i_grow: int = rl_current_growth(1000, 0, 2000)
    if i_grow == 0:
        ok = ok + 1
    i_full: int = rl_current_growth(1000, 20000, 2000)
    if i_full == 1000:
        ok = ok + 1
    i_dec: int = rl_current_decay(1000, 0, 2000)
    if i_dec == 1000:
        ok = ok + 1
    energy: int = inductor_energy(2000, 3000)
    if energy == 9000:
        ok = ok + 1
    bemf: int = back_emf(2000, 500)
    if bemf == 1000:
        ok = ok + 1
    return ok
