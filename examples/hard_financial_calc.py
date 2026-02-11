"""Financial calculations.

Tests: simple interest, compound interest, loan payment, depreciation.
"""


def simple_interest(principal: float, rate: float, years: int) -> float:
    """Calculate simple interest."""
    return principal * rate * float(years)


def compound_amount(principal: float, rate: float, periods: int) -> float:
    """Calculate compound amount: P * (1 + r)^n."""
    amount: float = principal
    i: int = 0
    while i < periods:
        amount = amount * (1.0 + rate)
        i = i + 1
    return amount


def future_value(payment: float, rate: float, periods: int) -> float:
    """Future value of annuity: PMT * ((1+r)^n - 1) / r."""
    if rate == 0.0:
        return payment * float(periods)
    total: float = 0.0
    i: int = 0
    while i < periods:
        total = (total + payment) * (1.0 + rate)
        i = i + 1
    return total


def straight_line_depreciation(cost: float, salvage: float, life: int) -> float:
    """Annual straight-line depreciation."""
    if life <= 0:
        return 0.0
    return (cost - salvage) / float(life)


def net_present_value(cashflows: list[float], rate: float) -> float:
    """Calculate NPV of cashflows at given discount rate."""
    npv: float = 0.0
    i: int = 0
    while i < len(cashflows):
        discount: float = 1.0
        j: int = 0
        while j < i:
            discount = discount * (1.0 + rate)
            j = j + 1
        npv = npv + cashflows[i] / discount
        i = i + 1
    return npv


def test_module() -> None:
    si: float = simple_interest(1000.0, 0.05, 3)
    assert si > 149.9 and si < 150.1
    ca: float = compound_amount(1000.0, 0.1, 2)
    assert ca > 1209.9 and ca < 1210.1
    fv: float = future_value(100.0, 0.0, 5)
    assert fv > 499.9 and fv < 500.1
    dep: float = straight_line_depreciation(10000.0, 2000.0, 8)
    assert dep > 999.9 and dep < 1000.1
    cfs: list[float] = [-1000.0, 500.0, 500.0, 500.0]
    npv: float = net_present_value(cfs, 0.1)
    assert npv > 243.0 and npv < 244.0
