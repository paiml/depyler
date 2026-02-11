# Shannon entropy, information gain using integer-scaled arithmetic


def int_log2_scaled(numerator: int, denominator: int, scale: int) -> int:
    # Compute log2(numerator / denominator) * scale
    # Uses repeated squaring: multiply num/den by 2 until >= 1, counting bits
    if numerator <= 0 or denominator <= 0:
        return 0
    # Handle num >= den (positive log)
    result: int = 0
    num: int = numerator
    den: int = denominator
    if num >= den:
        while num >= 2 * den:
            num = num // 2
            result = result + scale
        # Fractional part via binary search
        frac: int = scale // 2
        bit: int = 0
        while bit < 10:
            num = num * num
            den = den * den
            if num >= 2 * den:
                num = num // 2
                result = result + frac
            frac = frac // 2
            bit = bit + 1
        return result
    else:
        # num < den: negative log
        while den > num:
            den = den // 2
            result = result + scale
            if den <= 0:
                break
        return -result


def frequency_count(data: list[int], max_val: int) -> list[int]:
    counts: list[int] = []
    i: int = 0
    while i <= max_val:
        counts.append(0)
        i = i + 1
    j: int = 0
    while j < len(data):
        if data[j] >= 0 and data[j] <= max_val:
            counts[data[j]] = counts[data[j]] + 1
        j = j + 1
    return counts


def shannon_entropy_scaled(counts: list[int], scale: int) -> int:
    # H = -sum(p_i * log2(p_i)) * scale
    # p_i = counts[i] / total
    total: int = 0
    i: int = 0
    while i < len(counts):
        total = total + counts[i]
        i = i + 1
    if total == 0:
        return 0
    entropy: int = 0
    i = 0
    while i < len(counts):
        if counts[i] > 0:
            # p = counts[i]/total, log2(p) = log2(counts[i]/total)
            log_p: int = int_log2_scaled(counts[i], total, scale)
            # contribution = -p * log2(p) = -(counts[i]/total) * log2(p)
            # In scaled: -(counts[i] * log_p) / total
            entropy = entropy - counts[i] * log_p // total
        i = i + 1
    return entropy


def max_entropy_scaled(num_classes: int, scale: int) -> int:
    if num_classes <= 1:
        return 0
    return int_log2_scaled(num_classes, 1, scale)


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: frequency count
    data: list[int] = [0, 0, 1, 1, 2, 2]
    counts: list[int] = frequency_count(data, 2)
    if counts[0] == 2 and counts[1] == 2 and counts[2] == 2:
        passed = passed + 1

    # Test 2: entropy of uniform distribution > 0
    entropy: int = shannon_entropy_scaled(counts, scale)
    if entropy > 500:
        passed = passed + 1

    # Test 3: entropy of single class = 0
    single: list[int] = [5, 0, 0]
    e: int = shannon_entropy_scaled(single, scale)
    if e == 0:
        passed = passed + 1

    # Test 4: entropy of two equal classes ~= 1.0 * scale
    two_equal: list[int] = [5, 5]
    e2: int = shannon_entropy_scaled(two_equal, scale)
    if abs_val(e2 - 1000) < 200:
        passed = passed + 1

    # Test 5: max entropy of 2 classes ~= 1.0
    me: int = max_entropy_scaled(2, scale)
    if abs_val(me - 1000) < 200:
        passed = passed + 1

    # Test 6: frequency count with gaps
    data2: list[int] = [0, 0, 0, 3, 3]
    counts2: list[int] = frequency_count(data2, 3)
    if counts2[0] == 3 and counts2[1] == 0 and counts2[3] == 2:
        passed = passed + 1

    # Test 7: entropy of more uniform > entropy of less uniform
    uniform: list[int] = [3, 3, 3, 3]
    skewed: list[int] = [10, 1, 1, 1]
    e_uniform: int = shannon_entropy_scaled(uniform, scale)
    e_skewed: int = shannon_entropy_scaled(skewed, scale)
    if e_uniform > e_skewed:
        passed = passed + 1

    return passed
