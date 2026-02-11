"""Stoichiometry computations using integer arithmetic.

Tests: molar ratios, limiting reagent, theoretical yield, percent yield.
Scale factor 1000 for fixed-point.
"""


def moles_from_mass(mass: int, molar_mass: int) -> int:
    """Compute moles = mass / molar_mass. Scale 1000."""
    if molar_mass == 0:
        return 0
    result: int = (mass * 1000) // molar_mass
    return result


def mass_from_moles(moles: int, molar_mass: int) -> int:
    """Compute mass = moles * molar_mass. Scale 1000."""
    result: int = (moles * molar_mass) // 1000
    return result


def limiting_reagent(moles_a: int, coeff_a: int, moles_b: int, coeff_b: int) -> int:
    """Determine limiting reagent: returns 1 if A is limiting, 2 if B.
    Compare moles_A/coeff_A vs moles_B/coeff_B. Scale 1000."""
    if coeff_a == 0 or coeff_b == 0:
        return 0
    ratio_a: int = (moles_a * 1000) // coeff_a
    ratio_b: int = (moles_b * 1000) // coeff_b
    if ratio_a <= ratio_b:
        return 1
    return 2


def theoretical_yield(moles_limiting: int, coeff_limiting: int, coeff_product: int, molar_mass_product: int) -> int:
    """Theoretical yield = (moles_limiting/coeff_limiting)*coeff_product*MM_product.
    Scale 1000."""
    if coeff_limiting == 0:
        return 0
    moles_product: int = (moles_limiting * coeff_product) // coeff_limiting
    result: int = (moles_product * molar_mass_product) // 1000
    return result


def percent_yield(actual: int, theoretical: int) -> int:
    """Percent yield = actual/theoretical * 1000. Scale 1000."""
    if theoretical == 0:
        return 0
    result: int = (actual * 1000) // theoretical
    return result


def excess_reagent_remaining(moles_excess: int, coeff_excess: int, moles_limiting: int, coeff_limiting: int) -> int:
    """Moles of excess reagent remaining after reaction.
    remaining = moles_excess - (moles_limiting/coeff_limiting)*coeff_excess.
    Scale 1000."""
    if coeff_limiting == 0:
        return moles_excess
    consumed: int = (moles_limiting * coeff_excess) // coeff_limiting
    result: int = moles_excess - consumed
    if result < 0:
        result = 0
    return result


def empirical_ratio(element_a: int, element_b: int) -> int:
    """Simplify ratio a:b by GCD. Returns encoded ratio a*1000+b."""
    a: int = element_a
    b: int = element_b
    if a <= 0 or b <= 0:
        return 0
    temp_a: int = a
    temp_b: int = b
    while temp_b != 0:
        remainder: int = temp_a % temp_b
        temp_a = temp_b
        temp_b = remainder
    gcd_val: int = temp_a
    if gcd_val == 0:
        return 0
    ratio_a: int = a // gcd_val
    ratio_b: int = b // gcd_val
    return ratio_a * 1000 + ratio_b


def test_module() -> int:
    """Test stoichiometry computations."""
    ok: int = 0
    m: int = moles_from_mass(18000, 18000)
    if m == 1000:
        ok = ok + 1
    ms: int = mass_from_moles(1000, 18000)
    if ms == 18000:
        ok = ok + 1
    lr: int = limiting_reagent(1000, 1, 2000, 1)
    if lr == 1:
        ok = ok + 1
    lr2: int = limiting_reagent(3000, 1, 1000, 1)
    if lr2 == 2:
        ok = ok + 1
    py: int = percent_yield(800, 1000)
    if py == 800:
        ok = ok + 1
    er: int = excess_reagent_remaining(3000, 1, 1000, 1)
    if er == 2000:
        ok = ok + 1
    ratio: int = empirical_ratio(4, 2)
    if ratio == 2001:
        ok = ok + 1
    return ok
