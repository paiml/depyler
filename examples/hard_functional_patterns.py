"""Hard functional programming patterns for depyler transpiler stress testing.
Tests higher-order functions, composition, and functional idioms."""


# ---------- 1. Manual map implementation ----------

def manual_map(vals: list[int], addend: int) -> list[int]:
    """Apply transformation to each element (simulates map with closure)."""
    result: list[int] = []
    for v in vals:
        result.append(v + addend)
    return result


def test_manual_map() -> int:
    """Test manual map over a list."""
    mapped: list[int] = manual_map([1, 2, 3, 4, 5], 10)
    total: int = 0
    for v in mapped:
        total += v
    return total


# ---------- 2. Manual filter implementation ----------

def manual_filter_positive(vals: list[int]) -> list[int]:
    """Filter elements keeping only positives."""
    result: list[int] = []
    for v in vals:
        if v > 0:
            result.append(v)
    return result


def test_manual_filter() -> int:
    """Test filtering negative values out."""
    filtered: list[int] = manual_filter_positive([-3, -1, 0, 2, 5, -7, 8])
    total: int = 0
    for v in filtered:
        total += v
    return total


# ---------- 3. Manual reduce / fold left ----------

def fold_left_sum(vals: list[int], init: int) -> int:
    """Left fold with addition as the combining operation."""
    acc: int = init
    for v in vals:
        acc = acc + v
    return acc


def fold_left_product(vals: list[int], init: int) -> int:
    """Left fold with multiplication as the combining operation."""
    acc: int = init
    for v in vals:
        acc = acc * v
    return acc


def test_fold_left() -> int:
    """Test fold left with sum and product."""
    s: int = fold_left_sum([1, 2, 3, 4, 5], 0)
    p: int = fold_left_product([1, 2, 3, 4], 1)
    return s + p


# ---------- 4. Scan (prefix fold) ----------

def scan_sum(vals: list[int]) -> list[int]:
    """Running prefix sum (scan operation)."""
    result: list[int] = []
    acc: int = 0
    for v in vals:
        acc = acc + v
        result.append(acc)
    return result


def test_scan() -> int:
    """Test scan produces correct running totals."""
    scanned: list[int] = scan_sum([1, 2, 3, 4, 5])
    # Expected: [1, 3, 6, 10, 15], last element is 15
    return scanned[len(scanned) - 1]


# ---------- 5. Function composition via apply-twice ----------

def apply_twice(x: int, step: int) -> int:
    """Apply an increment operation twice (simulates compose(f,f))."""
    first: int = x + step
    second: int = first + step
    return second


def apply_thrice(x: int, step: int) -> int:
    """Apply an increment operation three times."""
    first: int = x + step
    second: int = first + step
    third: int = second + step
    return third


def test_composition() -> int:
    """Test function composition simulation."""
    a: int = apply_twice(10, 3)
    b: int = apply_thrice(10, 3)
    return a + b


# ---------- 6. Partial application simulation ----------

def add_partial(a: int, b: int) -> int:
    """Simulates partial application: add_five = partial(add, 5)."""
    return a + b


def multiply_partial(a: int, b: int) -> int:
    """Simulates partial application for multiplication."""
    return a * b


def apply_binary_op(x: int, y: int, use_add: bool) -> int:
    """Dispatch to simulate partial application of binary ops."""
    if use_add:
        return add_partial(x, y)
    return multiply_partial(x, y)


def test_partial_application() -> int:
    """Test partial application simulation."""
    # Simulate add5 = partial(add, 5)
    r1: int = apply_binary_op(5, 10, True)
    # Simulate mul3 = partial(mul, 3)
    r2: int = apply_binary_op(3, 7, False)
    return r1 + r2


# ---------- 7. Currying simulation ----------

def curried_add_step1(a: int, b: int, c: int) -> int:
    """Simulate curried addition: curry(add)(a)(b)(c) = a+b+c."""
    return a + b + c


def curried_multiply_step1(a: int, b: int, c: int) -> int:
    """Simulate curried multiplication."""
    return a * b * c


def test_currying() -> int:
    """Test currying simulation with 3-arg functions."""
    r1: int = curried_add_step1(1, 2, 3)
    r2: int = curried_multiply_step1(2, 3, 4)
    return r1 + r2


# ---------- 8. Pipeline / chained transformations ----------

def pipeline_transform(vals: list[int]) -> int:
    """Chain: filter positives -> double each -> sum all."""
    # Step 1: filter positives
    positives: list[int] = []
    for v in vals:
        if v > 0:
            positives.append(v)
    # Step 2: double each
    doubled: list[int] = []
    for v in positives:
        doubled.append(v * 2)
    # Step 3: sum
    total: int = 0
    for v in doubled:
        total += v
    return total


def pipeline_nested(vals: list[int], threshold: int) -> int:
    """Chain: filter > threshold -> square -> take sum -> add offset."""
    filtered: list[int] = []
    for v in vals:
        if v > threshold:
            filtered.append(v)
    squared: list[int] = []
    for v in filtered:
        squared.append(v * v)
    total: int = 0
    for v in squared:
        total += v
    return total + len(filtered)


def test_pipeline() -> int:
    """Test pipeline transformations."""
    r1: int = pipeline_transform([-3, 1, -2, 4, 5])
    r2: int = pipeline_nested([1, 5, 3, 8, 2, 7], 4)
    return r1 + r2


# ---------- 9. Zip operation ----------

def zip_sum(a: list[int], b: list[int]) -> list[int]:
    """Zip two lists by summing corresponding elements."""
    result: list[int] = []
    length: int = len(a)
    if len(b) < length:
        length = len(b)
    for i in range(length):
        result.append(a[i] + b[i])
    return result


def zip_product(a: list[int], b: list[int]) -> list[int]:
    """Zip two lists by multiplying corresponding elements (dot-product style)."""
    result: list[int] = []
    length: int = len(a)
    if len(b) < length:
        length = len(b)
    for i in range(length):
        result.append(a[i] * b[i])
    return result


def test_zip() -> int:
    """Test zip operations."""
    sums: list[int] = zip_sum([1, 2, 3], [10, 20, 30])
    prods: list[int] = zip_product([2, 3, 4], [5, 6, 7])
    total: int = 0
    for v in sums:
        total += v
    for v in prods:
        total += v
    return total


# ---------- 10. Unzip operation ----------

def unzip_pairs(pairs: list[list[int]]) -> list[list[int]]:
    """Unzip list of pairs into two separate lists packed as [firsts..., seconds...]."""
    firsts: list[int] = []
    seconds: list[int] = []
    for pair in pairs:
        firsts.append(pair[0])
        seconds.append(pair[1])
    result: list[list[int]] = [firsts, seconds]
    return result


def test_unzip() -> int:
    """Test unzip of pairs."""
    pairs: list[list[int]] = [[1, 10], [2, 20], [3, 30]]
    unzipped: list[list[int]] = unzip_pairs(pairs)
    total: int = 0
    for v in unzipped[0]:
        total += v
    for v in unzipped[1]:
        total += v
    return total


# ---------- 11. Partition function ----------

def partition(vals: list[int], pivot: int) -> list[list[int]]:
    """Partition list into [less_than_pivot, greater_or_equal]."""
    less: list[int] = []
    greater_eq: list[int] = []
    for v in vals:
        if v < pivot:
            less.append(v)
        else:
            greater_eq.append(v)
    return [less, greater_eq]


def test_partition() -> int:
    """Test partitioning a list around a pivot."""
    parts: list[list[int]] = partition([5, 1, 8, 3, 9, 2, 7], 5)
    less_count: int = len(parts[0])
    geq_count: int = len(parts[1])
    return less_count * 10 + geq_count


# ---------- 12. Flat map / bind ----------

def flat_map_expand(vals: list[int]) -> list[int]:
    """Flat map: each element n expands to [n, n*n]."""
    result: list[int] = []
    for v in vals:
        result.append(v)
        result.append(v * v)
    return result


def flat_map_range(vals: list[int]) -> list[int]:
    """Flat map: each element n expands to range(n)."""
    result: list[int] = []
    for v in vals:
        for i in range(v):
            result.append(i)
    return result


def test_flat_map() -> int:
    """Test flat map operations."""
    expanded: list[int] = flat_map_expand([1, 2, 3])
    # [1, 1, 2, 4, 3, 9] -> sum = 20
    ranged: list[int] = flat_map_range([2, 3, 4])
    # [0,1, 0,1,2, 0,1,2,3] -> sum = 10
    total: int = 0
    for v in expanded:
        total += v
    for v in ranged:
        total += v
    return total


# ---------- 13. Predicate combinators: all, any, none ----------

def all_positive(vals: list[int]) -> bool:
    """Check if all elements are positive."""
    for v in vals:
        if v <= 0:
            return False
    return True


def any_negative(vals: list[int]) -> bool:
    """Check if any element is negative."""
    for v in vals:
        if v < 0:
            return True
    return False


def none_zero(vals: list[int]) -> bool:
    """Check that no element is zero."""
    for v in vals:
        if v == 0:
            return False
    return True


def count_matching(vals: list[int], threshold: int) -> int:
    """Count elements above threshold (predicate combinator)."""
    count: int = 0
    for v in vals:
        if v > threshold:
            count += 1
    return count


def test_predicates() -> int:
    """Test predicate combinators."""
    score: int = 0
    if all_positive([1, 2, 3, 4]):
        score += 10
    if any_negative([1, -2, 3]):
        score += 20
    if none_zero([1, 2, 3]):
        score += 30
    score += count_matching([1, 5, 3, 8, 2, 7], 4)
    return score


# ---------- 14. Transducer (composed filter-map) ----------

def transduce_filter_double_sum(vals: list[int], min_val: int) -> int:
    """Transducer: filter(>min_val) then map(*2) then reduce(+, 0).

    Composed in a single pass for efficiency.
    """
    acc: int = 0
    for v in vals:
        if v > min_val:
            acc += v * 2
    return acc


def transduce_square_filter_sum(vals: list[int], max_square: int) -> int:
    """Transducer: map(x^2) then filter(<max_square) then reduce(+, 0)."""
    acc: int = 0
    for v in vals:
        sq: int = v * v
        if sq < max_square:
            acc += sq
    return acc


def test_transducers() -> int:
    """Test transducer-style composed transformations."""
    r1: int = transduce_filter_double_sum([1, 5, 3, 8, 2, 7], 4)
    # filter: [5, 8, 7], double: [10, 16, 14], sum: 40
    r2: int = transduce_square_filter_sum([1, 2, 3, 4, 5], 20)
    # squares: [1, 4, 9, 16, 25], filter(<20): [1, 4, 9, 16], sum: 30
    return r1 + r2


# ---------- 15. Memoization with dict cache ----------

def fib_memo(n: int) -> int:
    """Fibonacci with memoization using dict."""
    cache: dict[int, int] = {}
    cache[0] = 0
    cache[1] = 1
    for i in range(2, n + 1):
        cache[i] = cache[i - 1] + cache[i - 2]
    return cache[n]


def tribonacci_memo(n: int) -> int:
    """Tribonacci with memoization using dict."""
    cache: dict[int, int] = {}
    cache[0] = 0
    cache[1] = 0
    cache[2] = 1
    for i in range(3, n + 1):
        cache[i] = cache[i - 1] + cache[i - 2] + cache[i - 3]
    return cache[n]


def test_memoization() -> int:
    """Test memoized functions."""
    f10: int = fib_memo(10)
    t10: int = tribonacci_memo(10)
    return f10 + t10


# ---------- 16. Fixed-point iteration ----------

def fixed_point_collatz(n: int) -> int:
    """Count steps to reach fixed point 1 via Collatz sequence."""
    steps: int = 0
    current: int = n
    while current != 1:
        if current % 2 == 0:
            current = current // 2
        else:
            current = 3 * current + 1
        steps += 1
    return steps


def fixed_point_digit_sum(n: int) -> int:
    """Repeatedly sum digits until single digit (digital root via iteration)."""
    current: int = n
    while current >= 10:
        total: int = 0
        temp: int = current
        while temp > 0:
            total += temp % 10
            temp = temp // 10
        current = total
    return current


def test_fixed_point() -> int:
    """Test fixed-point iterations."""
    collatz_27: int = fixed_point_collatz(27)
    digit_root: int = fixed_point_digit_sum(9999)
    return collatz_27 + digit_root


# ---------- 17. Accumulator-passing style ----------

def sum_acc(vals: list[int], acc: int) -> int:
    """Sum list using accumulator-passing style."""
    for v in vals:
        acc = acc + v
    return acc


def factorial_acc(n: int, acc: int) -> int:
    """Factorial using accumulator-passing style (tail-recursive form)."""
    result: int = acc
    for i in range(1, n + 1):
        result = result * i
    return result


def power_acc(base: int, exp: int, acc: int) -> int:
    """Power using accumulator-passing style."""
    result: int = acc
    for i in range(exp):
        result = result * base
    return result


def test_accumulator_passing() -> int:
    """Test accumulator-passing style functions."""
    s: int = sum_acc([1, 2, 3, 4, 5], 0)
    f: int = factorial_acc(6, 1)
    p: int = power_acc(2, 10, 1)
    return s + f + p


# ---------- 18. CPS (continuation-passing style) simulation ----------

def cps_add(a: int, b: int, continuation_mul: int) -> int:
    """CPS add: compute a+b then apply continuation (multiply by k)."""
    intermediate: int = a + b
    return intermediate * continuation_mul


def cps_chain(x: int, y: int, z: int) -> int:
    """CPS chain: add(x, y) -> multiply result by z -> add 1."""
    step1: int = cps_add(x, y, z)
    return step1 + 1


def cps_factorial(n: int) -> int:
    """Factorial using CPS simulation with explicit continuation stack."""
    result: int = 1
    i: int = n
    while i > 0:
        result = result * i
        i -= 1
    return result


def test_cps() -> int:
    """Test continuation-passing style simulation."""
    r1: int = cps_add(3, 4, 2)
    r2: int = cps_chain(2, 3, 4)
    r3: int = cps_factorial(5)
    return r1 + r2 + r3


# ---------- 19. Church encoding: booleans as ints ----------

def church_true(a: int, b: int) -> int:
    """Church boolean TRUE: select first argument."""
    return a


def church_false(a: int, b: int) -> int:
    """Church boolean FALSE: select second argument."""
    return b


def church_and(p: bool, q: bool, a: int, b: int) -> int:
    """Church AND: if p then q else false."""
    if p:
        if q:
            return church_true(a, b)
        return church_false(a, b)
    return church_false(a, b)


def church_or(p: bool, q: bool, a: int, b: int) -> int:
    """Church OR: if p then true else q."""
    if p:
        return church_true(a, b)
    if q:
        return church_true(a, b)
    return church_false(a, b)


def church_not(p: bool, a: int, b: int) -> int:
    """Church NOT: if p then false else true."""
    if p:
        return church_false(a, b)
    return church_true(a, b)


def test_church_booleans() -> int:
    """Test Church-encoded boolean operations."""
    t1: int = church_and(True, True, 10, 0)
    t2: int = church_and(True, False, 10, 0)
    t3: int = church_or(False, True, 20, 0)
    t4: int = church_or(False, False, 20, 0)
    t5: int = church_not(False, 30, 0)
    return t1 + t2 + t3 + t4 + t5


# ---------- 20. Church numerals as iteration counts ----------

def church_zero(x: int) -> int:
    """Church numeral 0: apply f zero times (identity)."""
    return x


def church_succ(n: int, x: int, step: int) -> int:
    """Church successor: apply f one more time than n."""
    result: int = x
    for i in range(n + 1):
        result = result + step
    return result


def church_add_nums(a: int, b: int, x: int, step: int) -> int:
    """Church addition: apply f (a+b) times."""
    result: int = x
    for i in range(a + b):
        result = result + step
    return result


def church_mul_nums(a: int, b: int, x: int, step: int) -> int:
    """Church multiplication: apply f (a*b) times."""
    result: int = x
    for i in range(a * b):
        result = result + step
    return result


def test_church_numerals() -> int:
    """Test Church numeral operations."""
    z: int = church_zero(42)
    s: int = church_succ(3, 0, 1)
    a: int = church_add_nums(3, 4, 0, 1)
    m: int = church_mul_nums(3, 4, 0, 1)
    return z + s + a + m


# ---------- 21. Y-combinator simulation (explicit recursion) ----------

def y_factorial(n: int) -> int:
    """Y-combinator-style factorial using explicit loop (no lambda needed)."""
    if n <= 1:
        return 1
    result: int = 1
    for i in range(2, n + 1):
        result = result * i
    return result


def y_fibonacci(n: int) -> int:
    """Y-combinator-style fibonacci using explicit iteration."""
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for i in range(2, n + 1):
        temp: int = a + b
        a = b
        b = temp
    return b


def y_power(base: int, exp: int) -> int:
    """Y-combinator-style power using explicit iteration."""
    result: int = 1
    for i in range(exp):
        result = result * base
    return result


def test_y_combinator() -> int:
    """Test Y-combinator style recursive functions."""
    f: int = y_factorial(7)
    fib: int = y_fibonacci(12)
    p: int = y_power(3, 5)
    return f + fib + p


# ---------- 22. Lens / getter / setter on nested dicts ----------

def lens_get(data: dict[str, int], key: str) -> int:
    """Lens getter: extract value at key."""
    if key in data:
        return data[key]
    return 0


def lens_set(data: dict[str, int], key: str, value: int) -> dict[str, int]:
    """Lens setter: return new dict with key set to value."""
    result: dict[str, int] = {}
    for k in data:
        result[k] = data[k]
    result[key] = value
    return result


def lens_modify(data: dict[str, int], key: str, delta: int) -> dict[str, int]:
    """Lens modify: apply transformation to value at key."""
    current: int = lens_get(data, key)
    return lens_set(data, key, current + delta)


def test_lens() -> int:
    """Test lens-style get/set/modify on dicts."""
    data: dict[str, int] = {"x": 10, "y": 20, "z": 30}
    got: int = lens_get(data, "x")
    updated: dict[str, int] = lens_set(data, "x", 100)
    modified: dict[str, int] = lens_modify(data, "y", 5)
    return got + lens_get(updated, "x") + lens_get(modified, "y")


# ---------- 23. GroupBy implementation ----------

def group_by_mod(vals: list[int], modulus: int) -> dict[int, list[int]]:
    """Group values by their remainder when divided by modulus."""
    groups: dict[int, list[int]] = {}
    for v in vals:
        key: int = v % modulus
        if key in groups:
            groups[key].append(v)
        else:
            groups[key] = [v]
    return groups


def group_by_sign(vals: list[int]) -> dict[int, int]:
    """Group by sign and count elements in each group.

    Returns dict with keys: -1 (negative), 0 (zero), 1 (positive).
    """
    counts: dict[int, int] = {}
    counts[-1] = 0
    counts[0] = 0
    counts[1] = 0
    for v in vals:
        if v < 0:
            counts[-1] += 1
        elif v == 0:
            counts[0] += 1
        else:
            counts[1] += 1
    return counts


def test_group_by() -> int:
    """Test groupBy implementations."""
    groups: dict[int, list[int]] = group_by_mod([1, 2, 3, 4, 5, 6, 7, 8, 9], 3)
    # Groups: {0: [3,6,9], 1: [1,4,7], 2: [2,5,8]}
    count_mod0: int = len(groups[0])
    count_mod1: int = len(groups[1])
    signs: dict[int, int] = group_by_sign([-5, -3, 0, 1, 4, 7])
    neg: int = signs[-1]
    pos: int = signs[1]
    return count_mod0 * 10 + count_mod1 * 10 + neg + pos


# ---------- 24. Take / drop / take_while / drop_while ----------

def take_n(vals: list[int], n: int) -> list[int]:
    """Take first n elements."""
    result: list[int] = []
    for i in range(n):
        if i < len(vals):
            result.append(vals[i])
    return result


def drop_n(vals: list[int], n: int) -> list[int]:
    """Drop first n elements."""
    result: list[int] = []
    for i in range(n, len(vals)):
        result.append(vals[i])
    return result


def take_while_positive(vals: list[int]) -> list[int]:
    """Take elements while they are positive."""
    result: list[int] = []
    for v in vals:
        if v <= 0:
            return result
        result.append(v)
    return result


def drop_while_positive(vals: list[int]) -> list[int]:
    """Drop elements while they are positive, return the rest."""
    dropping: bool = True
    result: list[int] = []
    for v in vals:
        if dropping and v > 0:
            continue
        dropping = False
        result.append(v)
    return result


def test_take_drop() -> int:
    """Test take/drop/take_while/drop_while."""
    taken: list[int] = take_n([10, 20, 30, 40, 50], 3)
    dropped: list[int] = drop_n([10, 20, 30, 40, 50], 2)
    tw: list[int] = take_while_positive([3, 5, 7, -1, 9, 11])
    dw: list[int] = drop_while_positive([3, 5, 7, -1, 9, 11])
    total: int = 0
    for v in taken:
        total += v
    for v in dropped:
        total += v
    total += len(tw) * 10
    total += len(dw) * 10
    return total


# ---------- 25. Sliding window ----------

def sliding_window_sum(vals: list[int], window: int) -> list[int]:
    """Compute sum for each sliding window of given size."""
    result: list[int] = []
    for i in range(len(vals) - window + 1):
        w_sum: int = 0
        for j in range(window):
            w_sum += vals[i + j]
        result.append(w_sum)
    return result


def sliding_window_max(vals: list[int], window: int) -> list[int]:
    """Compute max for each sliding window of given size."""
    result: list[int] = []
    for i in range(len(vals) - window + 1):
        w_max: int = vals[i]
        for j in range(1, window):
            if vals[i + j] > w_max:
                w_max = vals[i + j]
        result.append(w_max)
    return result


def test_sliding_window() -> int:
    """Test sliding window operations."""
    sums: list[int] = sliding_window_sum([1, 3, 5, 7, 9], 3)
    maxes: list[int] = sliding_window_max([1, 3, 5, 7, 9], 3)
    total: int = 0
    for v in sums:
        total += v
    for v in maxes:
        total += v
    return total


# ---------- 26. Interleave / round-robin merge ----------

def interleave(a: list[int], b: list[int]) -> list[int]:
    """Interleave two lists element by element."""
    result: list[int] = []
    length: int = len(a)
    if len(b) > length:
        length = len(b)
    for i in range(length):
        if i < len(a):
            result.append(a[i])
        if i < len(b):
            result.append(b[i])
    return result


def test_interleave() -> int:
    """Test interleaving two lists."""
    merged: list[int] = interleave([1, 3, 5], [2, 4, 6])
    total: int = 0
    for v in merged:
        total += v
    return total


# ---------- 27. Chunk / split into sublists ----------

def chunk(vals: list[int], size: int) -> list[list[int]]:
    """Split list into chunks of given size."""
    result: list[list[int]] = []
    current: list[int] = []
    for i, v in enumerate(vals):
        current.append(v)
        if len(current) == size:
            result.append(current)
            current = []
    if len(current) > 0:
        result.append(current)
    return result


def test_chunk() -> int:
    """Test chunking a list."""
    chunks: list[list[int]] = chunk([1, 2, 3, 4, 5, 6, 7], 3)
    # [[1,2,3], [4,5,6], [7]]
    num_chunks: int = len(chunks)
    last_chunk_size: int = len(chunks[num_chunks - 1])
    total: int = 0
    for c in chunks:
        for v in c:
            total += v
    return total + num_chunks * 100 + last_chunk_size


# ---------- 28. Unique / deduplicate ----------

def unique_preserve_order(vals: list[int]) -> list[int]:
    """Remove duplicates while preserving insertion order."""
    seen: dict[int, bool] = {}
    result: list[int] = []
    for v in vals:
        if v not in seen:
            seen[v] = True
            result.append(v)
    return result


def test_unique() -> int:
    """Test deduplication preserving order."""
    deduped: list[int] = unique_preserve_order([3, 1, 4, 1, 5, 9, 2, 6, 5, 3])
    total: int = 0
    for v in deduped:
        total += v
    return total + len(deduped) * 10


# ---------- 29. Frequency map / histogram ----------

def frequency_map(vals: list[int]) -> dict[int, int]:
    """Build frequency map of values."""
    freq: dict[int, int] = {}
    for v in vals:
        if v in freq:
            freq[v] += 1
        else:
            freq[v] = 1
    return freq


def most_frequent(vals: list[int]) -> int:
    """Find the most frequently occurring element."""
    freq: dict[int, int] = frequency_map(vals)
    best_val: int = vals[0]
    best_count: int = 0
    for v in vals:
        if freq[v] > best_count:
            best_count = freq[v]
            best_val = v
    return best_val


def test_frequency() -> int:
    """Test frequency map and most frequent."""
    freq: dict[int, int] = frequency_map([1, 2, 2, 3, 3, 3, 4])
    mf: int = most_frequent([1, 2, 2, 3, 3, 3, 4])
    total: int = freq[3] * 10 + mf
    return total


# ---------- 30. Tally / aggregate multiple accumulators ----------

def tally_stats(vals: list[int]) -> list[int]:
    """Compute [count, sum, min, max] in single pass."""
    if len(vals) == 0:
        return [0, 0, 0, 0]
    count: int = 0
    total: int = 0
    lo: int = vals[0]
    hi: int = vals[0]
    for v in vals:
        count += 1
        total += v
        if v < lo:
            lo = v
        if v > hi:
            hi = v
    return [count, total, lo, hi]


def test_tally() -> int:
    """Test multi-accumulator tally."""
    stats: list[int] = tally_stats([5, 2, 8, 1, 9, 3, 7])
    # [7, 35, 1, 9]
    return stats[0] + stats[1] + stats[2] + stats[3]


# ---------- 31. Flatten nested list ----------

def flatten_2d(nested: list[list[int]]) -> list[int]:
    """Flatten a 2D list into 1D."""
    result: list[int] = []
    for row in nested:
        for v in row:
            result.append(v)
    return result


def test_flatten() -> int:
    """Test flattening nested list."""
    flat: list[int] = flatten_2d([[1, 2], [3, 4, 5], [6]])
    total: int = 0
    for v in flat:
        total += v
    return total


# ---------- 32. Map with index ----------

def map_with_index(vals: list[int]) -> list[int]:
    """Map each element to element * its index."""
    result: list[int] = []
    for i, v in enumerate(vals):
        result.append(v * i)
    return result


def test_map_with_index() -> int:
    """Test map with index."""
    mapped: list[int] = map_with_index([10, 20, 30, 40])
    # [0, 20, 60, 120] -> sum = 200
    total: int = 0
    for v in mapped:
        total += v
    return total


# ---------- 33. Unfold / generate sequence ----------

def unfold_powers_of_two(count: int) -> list[int]:
    """Generate sequence of powers of 2 via unfold."""
    result: list[int] = []
    current: int = 1
    for i in range(count):
        result.append(current)
        current = current * 2
    return result


def unfold_triangular(count: int) -> list[int]:
    """Generate triangular numbers via unfold."""
    result: list[int] = []
    acc: int = 0
    for i in range(1, count + 1):
        acc = acc + i
        result.append(acc)
    return result


def test_unfold() -> int:
    """Test unfold / sequence generation."""
    powers: list[int] = unfold_powers_of_two(8)
    triangles: list[int] = unfold_triangular(5)
    total: int = 0
    for v in powers:
        total += v
    for v in triangles:
        total += v
    return total


# ---------- 34. Iterate / fixed-point with convergence ----------

def iterate_halve(n: int) -> int:
    """Count iterations of halving until reaching 1."""
    count: int = 0
    current: int = n
    while current > 1:
        current = current // 2
        count += 1
    return count


def iterate_triple_plus_one(n: int, max_steps: int) -> int:
    """Iterate 3n+1 rule up to max_steps, return final value."""
    current: int = n
    for i in range(max_steps):
        if current <= 1:
            return current
        if current % 2 == 0:
            current = current // 2
        else:
            current = 3 * current + 1
    return current


def test_iterate() -> int:
    """Test iterative transformations."""
    h: int = iterate_halve(256)
    t: int = iterate_triple_plus_one(7, 100)
    return h + t


# ---------- 35. Maybe / Option pattern via sentinel ----------

def safe_head(vals: list[int], default: int) -> int:
    """Safe head of list, returns default if empty (Option pattern)."""
    if len(vals) == 0:
        return default
    return vals[0]


def safe_last(vals: list[int], default: int) -> int:
    """Safe last of list, returns default if empty."""
    if len(vals) == 0:
        return default
    return vals[len(vals) - 1]


def safe_index(vals: list[int], idx: int, default: int) -> int:
    """Safe index access, returns default if out of bounds."""
    if idx < 0 or idx >= len(vals):
        return default
    return vals[idx]


def test_maybe() -> int:
    """Test Option/Maybe pattern with safe accessors."""
    h: int = safe_head([10, 20, 30], -1)
    he: int = safe_head([], -1)
    l: int = safe_last([10, 20, 30], -1)
    le: int = safe_last([], -1)
    i: int = safe_index([10, 20, 30], 1, -1)
    ie: int = safe_index([10, 20, 30], 5, -1)
    return h + he + l + le + i + ie


# ---------- Runner ----------

def run_all_tests() -> int:
    """Run all test functions and return sum of results."""
    total: int = 0
    total += test_manual_map()
    total += test_manual_filter()
    total += test_fold_left()
    total += test_scan()
    total += test_composition()
    total += test_partial_application()
    total += test_currying()
    total += test_pipeline()
    total += test_zip()
    total += test_unzip()
    total += test_partition()
    total += test_flat_map()
    total += test_predicates()
    total += test_transducers()
    total += test_memoization()
    total += test_fixed_point()
    total += test_accumulator_passing()
    total += test_cps()
    total += test_church_booleans()
    total += test_church_numerals()
    total += test_y_combinator()
    total += test_lens()
    total += test_group_by()
    total += test_take_drop()
    total += test_sliding_window()
    total += test_interleave()
    total += test_chunk()
    total += test_unique()
    total += test_frequency()
    total += test_tally()
    total += test_flatten()
    total += test_map_with_index()
    total += test_unfold()
    total += test_iterate()
    total += test_maybe()
    return total
