"""Chemical equilibrium computations using integer arithmetic.

Tests: equilibrium constant, Le Chatelier, reaction quotient, Kp.
Scale factor 1000 for fixed-point.
"""


def equilibrium_constant(products: list[int], reactants: list[int]) -> int:
    """Kc = product(concentrations of products) / product(concentrations of reactants).
    All values scale 1000. Returns Kc * 1000."""
    prod_val: int = 1000
    i: int = 0
    while i < len(products):
        val: int = products[i]
        prod_val = (prod_val * val) // 1000
        i = i + 1
    react_val: int = 1000
    j: int = 0
    while j < len(reactants):
        val2: int = reactants[j]
        react_val = (react_val * val2) // 1000
        j = j + 1
    if react_val == 0:
        return 0
    result: int = (prod_val * 1000) // react_val
    return result


def reaction_quotient(products: list[int], reactants: list[int]) -> int:
    """Reaction quotient Q - same formula as Kc but at non-equilibrium."""
    return equilibrium_constant(products, reactants)


def predict_direction(q_val: int, k_val: int) -> int:
    """Predict reaction direction: returns 1 (forward), -1 (reverse), 0 (equilibrium)."""
    if q_val < k_val:
        return 1
    if q_val > k_val:
        return 0 - 1
    return 0


def kp_from_kc(kc: int, delta_n: int, temp: int) -> int:
    """Kp = Kc * (RT)^delta_n. R = 8314 scale 1000.
    Simplified for small delta_n. Scale 1000."""
    r_const: int = 8314
    rt: int = (r_const * temp) // 1000
    if delta_n == 0:
        return kc
    result: int = kc
    count: int = delta_n
    if count < 0:
        count = 0 - count
    step: int = 0
    while step < count:
        if delta_n > 0:
            result = (result * rt) // 1000
        else:
            if rt == 0:
                return 0
            result = (result * 1000) // rt
        step = step + 1
    return result


def degree_of_dissociation(initial_conc: int, eq_conc: int) -> int:
    """Degree of dissociation alpha = (C0 - Ceq)/C0. Scale 1000."""
    if initial_conc == 0:
        return 0
    result: int = ((initial_conc - eq_conc) * 1000) // initial_conc
    return result


def ice_table_product(initial: int, change: int) -> int:
    """ICE table: equilibrium = initial + change."""
    return initial + change


def ice_table_reactant(initial: int, change: int) -> int:
    """ICE table: equilibrium = initial - change."""
    return initial - change


def van_hoff_k2(k1: int, delta_h: int, t1: int, t2: int) -> int:
    """Van't Hoff equation: ln(K2/K1) = (dH/R)*(1/T1 - 1/T2).
    R = 8314. Approx: K2 = K1 * exp(factor). Scale 1000."""
    if t1 == 0 or t2 == 0:
        return k1
    r_const: int = 8314
    inv_t1: int = (1000 * 1000) // t1
    inv_t2: int = (1000 * 1000) // t2
    factor: int = (delta_h * (inv_t1 - inv_t2)) // r_const
    exp_val: int = 1000 + factor + (factor * factor) // 2000
    if exp_val < 0:
        exp_val = 0
    result: int = (k1 * exp_val) // 1000
    return result


def test_module() -> int:
    """Test equilibrium computations."""
    ok: int = 0
    kc: int = equilibrium_constant([2000], [1000])
    if kc == 2000:
        ok = ok + 1
    q: int = reaction_quotient([1500], [1000])
    if q == 1500:
        ok = ok + 1
    d: int = predict_direction(1500, 2000)
    if d == 1:
        ok = ok + 1
    d2: int = predict_direction(2500, 2000)
    if d2 == 0 - 1:
        ok = ok + 1
    alpha: int = degree_of_dissociation(1000, 800)
    if alpha == 200:
        ok = ok + 1
    ice_p: int = ice_table_product(0, 500)
    if ice_p == 500:
        ok = ok + 1
    return ok
