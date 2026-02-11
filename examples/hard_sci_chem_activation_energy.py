"""Activation energy and Arrhenius equation computations.

Tests: Arrhenius equation, activation energy, frequency factor.
Scale factor 1000 for fixed-point.
"""


def arrhenius_rate_constant(freq_factor: int, ea_over_r: int, temp: int) -> int:
    """k = A * exp(-Ea/(R*T)). ea_over_r = Ea/R pre-computed.
    exp(-x) ~ 1 - x + x^2/2. Scale 1000."""
    if temp == 0:
        return 0
    x: int = (ea_over_r * 1000) // temp
    if x > 5000:
        return 0
    exp_val: int = 1000 - x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (freq_factor * exp_val) // 1000
    return result


def activation_energy_two_temps(k1: int, k2: int, t1: int, t2: int) -> int:
    """Ea = R * ln(k2/k1) / (1/T1 - 1/T2).
    R = 8314. ln(x) ~ (x-1000)/1000 for x near 1000. Scale 1000."""
    if t1 == 0 or t2 == 0 or k1 == 0:
        return 0
    r_const: int = 8314
    ratio: int = (k2 * 1000) // k1
    ln_ratio: int = ratio - 1000
    temp_diff: int = (1000 * 1000) // t1 - (1000 * 1000) // t2
    if temp_diff == 0:
        return 0
    result: int = (r_const * ln_ratio) // temp_diff
    return result


def rate_ratio_temp_change(ea_over_r: int, t1: int, t2: int) -> int:
    """Rate ratio k2/k1 = exp(Ea/R * (1/T1 - 1/T2)).
    exp(x) ~ 1 + x + x^2/2. Scale 1000."""
    if t1 == 0 or t2 == 0:
        return 1000
    inv_diff: int = (1000 * 1000) // t1 - (1000 * 1000) // t2
    x: int = (ea_over_r * inv_diff) // 1000
    exp_val: int = 1000 + x + (x * x) // 2000
    if exp_val < 0:
        exp_val = 0
    return exp_val


def transition_state_free_energy(temp: int, rate_constant: int) -> int:
    """dG_ts = -R*T*ln(k*h/(kB*T)).
    Simplified: returns R*T*correction. Scale 1000."""
    if temp == 0:
        return 0
    r_const: int = 8314
    result: int = (r_const * temp) // 1000
    return result


def catalyzed_rate(uncatalyzed_rate: int, ea_reduction: int, temp: int) -> int:
    """Rate with catalyst: k_cat = k * exp(Ea_reduction/(R*T)).
    R = 8314. exp(x) ~ 1 + x + x^2/2. Scale 1000."""
    if temp == 0:
        return uncatalyzed_rate
    r_const: int = 8314
    x: int = (ea_reduction * 1000) // (r_const * temp)
    exp_val: int = 1000 + x + (x * x) // 2000
    result: int = (uncatalyzed_rate * exp_val) // 1000
    return result


def temperature_coefficient(rate1: int, rate2: int) -> int:
    """Temperature coefficient = rate2/rate1 (usually for 10K increase). Scale 1000."""
    if rate1 == 0:
        return 0
    result: int = (rate2 * 1000) // rate1
    return result


def shelf_life_estimate(rate_constant: int, limit_fraction: int) -> int:
    """Shelf life for first order: t = -ln(1-f)/k.
    ln(1-f) ~ -f - f^2/2. Scale 1000."""
    if rate_constant == 0:
        return 0
    ln_val: int = 0 - limit_fraction - (limit_fraction * limit_fraction) // 2000
    t_val: int = (0 - ln_val * 1000) // rate_constant
    return t_val


def test_module() -> int:
    """Test activation energy computations."""
    ok: int = 0
    k_val: int = arrhenius_rate_constant(1000, 0, 300)
    if k_val == 1000:
        ok = ok + 1
    k_zero_t: int = arrhenius_rate_constant(1000, 1000, 0)
    if k_zero_t == 0:
        ok = ok + 1
    rr: int = rate_ratio_temp_change(0, 300, 310)
    if rr == 1000:
        ok = ok + 1
    cr: int = catalyzed_rate(1000, 0, 300)
    if cr == 1000:
        ok = ok + 1
    tc: int = temperature_coefficient(100, 200)
    if tc == 2000:
        ok = ok + 1
    sl: int = shelf_life_estimate(100, 100)
    if sl > 1000 and sl < 1100:
        ok = ok + 1
    return ok
