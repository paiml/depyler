"""Thermal radiation computations using integer arithmetic.

Tests: Stefan-Boltzmann, Wien's law, emissive power, radiosity.
Scale factor 1000 for fixed-point.
"""


def stefan_boltzmann_power(emissivity: int, area: int, temp: int) -> int:
    """Radiated power P = eps*sigma*A*T^4.
    sigma = 5670 (scale 10^-5). Temp in relative units. Scale 1000."""
    t2: int = (temp * temp) // 1000
    t4: int = (t2 * t2) // 1000
    ea: int = (emissivity * area) // 1000
    sigma_ea: int = (5670 * ea) // 1000
    numer: int = (sigma_ea * t4) // 1000
    result: int = numer // 100
    return result


def net_radiation_exchange(eps: int, area: int, t_hot: int, t_cold: int) -> int:
    """Net radiation: Q = eps*sigma*A*(T_h^4 - T_c^4). Scale 1000."""
    th2: int = (t_hot * t_hot) // 1000
    th4: int = (th2 * th2) // 1000
    tc2: int = (t_cold * t_cold) // 1000
    tc4: int = (tc2 * tc2) // 1000
    diff_t4: int = th4 - tc4
    ea: int = (eps * area) // 1000
    sigma_ea: int = (5670 * ea) // 1000
    numer: int = (sigma_ea * diff_t4) // 1000
    result: int = numer // 100
    return result


def wiens_displacement(temp: int) -> int:
    """Wien's displacement law: lambda_max * T = 2898000.
    Returns lambda_max. Scale 1000 (wavelength in nm-like units)."""
    if temp == 0:
        return 0
    result: int = 2898000 // temp
    return result


def planck_peak_frequency(temp: int) -> int:
    """Peak frequency from Wien's law: f_max = 5.879 * 10^10 * T.
    Simplified: f_max = 5879 * T. Scale 1000 (freq in GHz-like)."""
    result: int = (5879 * temp) // 1000
    return result


def radiosity(emissivity: int, emissive_power: int, reflectivity: int, irradiation: int) -> int:
    """Radiosity J = eps*Eb + rho*G. Scale 1000."""
    emit: int = (emissivity * emissive_power) // 1000
    reflect: int = (reflectivity * irradiation) // 1000
    result: int = emit + reflect
    return result


def view_factor_parallel_plates(width: int, separation: int) -> int:
    """Approximate view factor for infinite parallel plates.
    F12 ~ w / sqrt(w^2 + d^2). Scale 1000."""
    if width == 0:
        return 0
    w2: int = (width * width) // 1000
    d2: int = (separation * separation) // 1000
    sum_sq: int = w2 + d2
    if sum_sq <= 0:
        return 1000
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
            return (width * 1000) // next_g
        guess = next_g
        iterations = iterations + 1
    return (width * 1000) // guess


def solar_constant_at_distance(luminosity: int, distance: int) -> int:
    """Solar irradiance at distance: S = L / (4*pi*d^2).
    pi ~ 3142. Scale 1000."""
    if distance == 0:
        return 0
    d_sq: int = (distance * distance) // 1000
    four_pi: int = 4 * 3142
    denom: int = (four_pi * d_sq) // 1000
    if denom == 0:
        return 0
    result: int = (luminosity * 1000) // denom
    return result


def absorptivity_from_kirchhoff(emissivity: int) -> int:
    """Kirchhoff's law: absorptivity = emissivity at thermal equilibrium."""
    return emissivity


def test_module() -> int:
    """Test thermal radiation computations."""
    ok: int = 0
    wl: int = wiens_displacement(6000)
    if wl > 480 and wl < 485:
        ok = ok + 1
    pf: int = planck_peak_frequency(6000)
    if pf > 35000 and pf < 35500:
        ok = ok + 1
    rad: int = radiosity(800, 1000, 200, 500)
    if rad == 900:
        ok = ok + 1
    ak: int = absorptivity_from_kirchhoff(900)
    if ak == 900:
        ok = ok + 1
    zero_w: int = wiens_displacement(0)
    if zero_w == 0:
        ok = ok + 1
    vf: int = view_factor_parallel_plates(1000, 0)
    if vf == 1000:
        ok = ok + 1
    return ok
