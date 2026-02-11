"""Thermal expansion computations using integer arithmetic.

Tests: linear, area, volume expansion, thermal stress.
Scale factor 1000 for fixed-point.
"""


def linear_expansion(length: int, alpha: int, delta_t: int) -> int:
    """Change in length: dL = L * alpha * dT. Scale 1000.
    alpha is coefficient * 10^6, so divide by 10^6 extra."""
    result: int = (length * alpha * delta_t) // (1000 * 1000 * 1000)
    return result


def final_length(length: int, alpha: int, delta_t: int) -> int:
    """Final length after expansion: L' = L + dL. Scale 1000."""
    dl: int = linear_expansion(length, alpha, delta_t)
    return length + dl


def area_expansion(area: int, alpha: int, delta_t: int) -> int:
    """Change in area: dA = 2 * A * alpha * dT. Scale 1000."""
    result: int = (2 * area * alpha * delta_t) // (1000 * 1000 * 1000)
    return result


def volume_expansion(volume: int, beta: int, delta_t: int) -> int:
    """Change in volume: dV = V * beta * dT. Scale 1000.
    beta = 3*alpha typically."""
    result: int = (volume * beta * delta_t) // (1000 * 1000 * 1000)
    return result


def thermal_stress(modulus: int, alpha: int, delta_t: int) -> int:
    """Thermal stress sigma = E * alpha * dT. Scale 1000.
    alpha * 10^6 so divide extra."""
    result: int = (modulus * alpha * delta_t) // (1000 * 1000 * 1000)
    return result


def bimetallic_curvature(alpha1: int, alpha2: int, delta_t: int, thickness: int) -> int:
    """Bimetallic strip curvature: kappa ~ 6*(alpha2-alpha1)*dT / t.
    Scale 1000."""
    if thickness == 0:
        return 0
    d_alpha: int = alpha2 - alpha1
    if d_alpha < 0:
        d_alpha = 0 - d_alpha
    result: int = (6 * d_alpha * delta_t) // (thickness * 1000)
    return result


def gap_closure_temp(gap: int, length: int, alpha: int) -> int:
    """Temperature change to close gap: dT = gap / (L * alpha).
    Scale 1000. alpha in 10^-6."""
    denom: int = (length * alpha) // 1000
    if denom == 0:
        return 0
    result: int = (gap * 1000 * 1000) // denom
    return result


def apparent_expansion_liquid(beta_liquid: int, beta_container: int) -> int:
    """Apparent expansion coefficient = beta_liquid - beta_container."""
    return beta_liquid - beta_container


def density_after_expansion(density: int, beta: int, delta_t: int) -> int:
    """Density after expansion: rho' = rho / (1 + beta*dT).
    Approx: rho' ~ rho * (1 - beta*dT). Scale 1000."""
    correction: int = (beta * delta_t) // (1000 * 1000)
    result: int = (density * (1000 - correction)) // 1000
    return result


def test_module() -> int:
    """Test thermal expansion computations."""
    ok: int = 0
    dl: int = linear_expansion(1000000, 12, 100000)
    if dl > 1100 and dl < 1300:
        ok = ok + 1
    da: int = area_expansion(1000000, 12, 100000)
    if da > 2300 and da < 2500:
        ok = ok + 1
    dv: int = volume_expansion(1000000, 36, 100000)
    if dv > 3500 and dv < 3700:
        ok = ok + 1
    ae: int = apparent_expansion_liquid(1000, 36)
    if ae == 964:
        ok = ok + 1
    zero_gap: int = gap_closure_temp(0, 1000, 12)
    if zero_gap == 0:
        ok = ok + 1
    bmc: int = bimetallic_curvature(12, 24, 1000, 0)
    if bmc == 0:
        ok = ok + 1
    return ok
