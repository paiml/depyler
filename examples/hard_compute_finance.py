"""Financial computations: compound interest, loan payment, NPV, IRR approximation.

All calculations use integer arithmetic (amounts in cents, rates in basis points).
Tests: compound_interest, loan_payment, npv, irr_approx, simple_interest.
"""


def simple_interest(principal: int, rate_bp: int, years: int) -> int:
    """Simple interest. Rate in basis points (100 bp = 1%). Returns total in cents."""
    interest: int = principal * rate_bp * years // 10000
    return principal + interest


def compound_interest(principal: int, rate_bp: int, years: int) -> int:
    """Compound interest (annual). Rate in basis points. Returns total in cents."""
    amount: int = principal
    yr: int = 0
    while yr < years:
        amount = amount + amount * rate_bp // 10000
        yr = yr + 1
    return amount


def future_value_monthly(principal: int, annual_rate_bp: int, months: int) -> int:
    """Future value with monthly compounding."""
    amount: int = principal
    monthly_rate: int = annual_rate_bp // 12
    m: int = 0
    while m < months:
        amount = amount + amount * monthly_rate // 10000
        m = m + 1
    return amount


def loan_total_payment(principal: int, annual_rate_bp: int, years: int) -> int:
    """Approximate total loan payment using iterative amortization."""
    monthly_rate: int = annual_rate_bp // 12
    num_payments: int = years * 12
    if monthly_rate == 0:
        return principal
    balance: int = principal * 10000
    total_paid: int = 0
    monthly_payment: int = balance * monthly_rate // (10000 - power_frac(10000, 10000 + monthly_rate, num_payments))
    if monthly_payment <= 0:
        monthly_payment = balance // num_payments
    p: int = 0
    while p < num_payments:
        interest_part: int = balance * monthly_rate // 10000
        total_paid = total_paid + monthly_payment
        balance = balance + interest_part - monthly_payment
        if balance < 0:
            balance = 0
        p = p + 1
    return total_paid // 10000


def power_frac(numerator: int, denominator: int, n: int) -> int:
    """Compute (numerator/denominator)^n as integer fraction * numerator^0 scale."""
    result: int = numerator
    i: int = 0
    while i < n:
        result = result * numerator // denominator
        i = i + 1
    return result


def npv_cents(rate_bp: int, cashflows: list[int]) -> int:
    """Net present value of cashflows in cents. Rate in basis points."""
    total: int = 0
    discount: int = 10000
    i: int = 0
    n: int = len(cashflows)
    while i < n:
        total = total + cashflows[i] * 10000 // discount
        discount = discount + discount * rate_bp // 10000
        i = i + 1
    return total


def irr_approx(cashflows: list[int]) -> int:
    """Approximate IRR in basis points using bisection method."""
    lo: int = 0
    hi: int = 10000
    iteration: int = 0
    while iteration < 100:
        mid: int = (lo + hi) // 2
        npv_val: int = npv_cents(mid, cashflows)
        if npv_val > 0:
            lo = mid + 1
        elif npv_val < 0:
            hi = mid - 1
        else:
            return mid
        iteration = iteration + 1
    return (lo + hi) // 2


def depreciation_straight_line(cost: int, salvage: int, life_years: int) -> int:
    """Annual straight-line depreciation amount."""
    if life_years <= 0:
        return 0
    return (cost - salvage) // life_years


def roi_bp(investment: int, gain: int) -> int:
    """Return on investment in basis points."""
    if investment == 0:
        return 0
    return gain * 10000 // investment


def test_module() -> int:
    """Test financial computations."""
    passed: int = 0

    si: int = simple_interest(10000, 500, 3)
    if si == 11500:
        passed = passed + 1

    ci: int = compound_interest(10000, 1000, 3)
    if ci == 13310:
        passed = passed + 1

    fv: int = future_value_monthly(10000, 1200, 12)
    if fv > 10000:
        passed = passed + 1

    dep: int = depreciation_straight_line(10000, 2000, 4)
    if dep == 2000:
        passed = passed + 1

    r: int = roi_bp(10000, 1500)
    if r == 1500:
        passed = passed + 1

    npv_val: int = npv_cents(500, [-10000, 3000, 3000, 3000, 3000])
    if npv_val > 0:
        passed = passed + 1

    if simple_interest(0, 500, 10) == 0:
        passed = passed + 1

    return passed
