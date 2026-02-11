# Type inference test: Chained function calls
# Strategy: Variables initialized from function returns, multi-step chains


def compute_a(x):
    """First in chain - type inferred from arithmetic."""
    return x * x + 1


def compute_b(x):
    """Second in chain - takes result of compute_a."""
    return x * 2 - 3


def compute_c(x):
    """Third in chain."""
    return x + 10


def full_chain(x):
    """Chain: compute_c(compute_b(compute_a(x)))."""
    step1 = compute_a(x)
    step2 = compute_b(step1)
    step3 = compute_c(step2)
    return step3


def parallel_chain(x, y):
    """Two parallel chains combined."""
    left = compute_a(x)
    right = compute_a(y)
    merged = left + right
    return compute_b(merged)


def reduce_chain(a, b, c):
    """Reduce three values through chained operations."""
    r1 = compute_a(a)
    r2 = compute_a(b)
    r3 = compute_a(c)
    sum_ab = r1 + r2
    sum_all = sum_ab + r3
    return compute_c(sum_all)


def iterative_chain(start, steps):
    """Repeatedly apply compute_b."""
    val = start
    i = 0
    while i < steps:
        val = compute_b(val)
        i = i + 1
    return val


def conditional_select(x, y, choose_first):
    """Select between two chain results based on flag."""
    if choose_first > 0:
        return compute_a(x)
    return compute_a(y)


def diverge_converge(x):
    """Fork into two paths then merge."""
    path1 = compute_a(x)
    path2 = compute_b(x)
    return path1 + path2


def pipeline_with_guard(x, lower, upper):
    """Chain with bounds checking at each step."""
    val = compute_a(x)
    if val < lower:
        val = lower
    if val > upper:
        val = upper
    val = compute_b(val)
    if val < lower:
        val = lower
    if val > upper:
        val = upper
    return val


def test_module() -> int:
    """Test chained function call inference."""
    total: int = 0

    # compute_a: x^2 + 1
    if compute_a(3) == 10:
        total = total + 1
    if compute_a(0) == 1:
        total = total + 1

    # compute_b: x*2 - 3
    if compute_b(5) == 7:
        total = total + 1

    # compute_c: x + 10
    if compute_c(5) == 15:
        total = total + 1

    # full_chain: c(b(a(x)))
    # a(3) = 10, b(10) = 17, c(17) = 27
    if full_chain(3) == 27:
        total = total + 1

    # parallel_chain: b(a(x) + a(y))
    # a(2)=5, a(3)=10, sum=15, b(15)=27
    if parallel_chain(2, 3) == 27:
        total = total + 1

    # reduce_chain: c(a(a) + a(b) + a(c))
    # a(1)=2, a(2)=5, a(3)=10, sum=17, c(17)=27
    if reduce_chain(1, 2, 3) == 27:
        total = total + 1

    # iterative_chain
    # start=10, b(10)=17, b(17)=31
    if iterative_chain(10, 2) == 31:
        total = total + 1

    # conditional_select
    if conditional_select(3, 5, 1) == 10:
        total = total + 1
    if conditional_select(3, 5, 0) == 26:
        total = total + 1

    # diverge_converge: a(x) + b(x)
    # a(3)=10, b(3)=3, sum=13
    if diverge_converge(3) == 13:
        total = total + 1

    # pipeline_with_guard
    r: int = pipeline_with_guard(3, 0, 50)
    if r == 17:
        total = total + 1

    return total
