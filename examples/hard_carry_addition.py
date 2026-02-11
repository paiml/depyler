"""Addition with carry operations.

Implements multi-digit addition using carry propagation,
simulating how hardware adders work at the digit level.
"""


def add_with_carry(a: int, b: int, carry_in: int) -> int:
    """Add two single digits with carry, return packed result.

    Lower 4 bits hold the sum digit, bit 4 holds carry out.
    """
    total: int = a + b + carry_in
    digit: int = total % 10
    carry_out: int = total // 10
    result: int = (carry_out << 4) | digit
    return result


def multi_digit_add(digits_a: list[int], digits_b: list[int], size: int) -> list[int]:
    """Add two numbers represented as digit arrays (least significant first).

    Returns result digits array of size+1 to hold potential carry.
    """
    result: list[int] = []
    carry: int = 0
    i: int = 0
    while i < size:
        a_digit: int = digits_a[i]
        b_digit: int = digits_b[i]
        packed: int = add_with_carry(a_digit, b_digit, carry)
        sum_digit: int = packed & 0xF
        carry = (packed >> 4) & 0xF
        result.append(sum_digit)
        i = i + 1
    result.append(carry)
    return result


def chain_carry_count(digits_a: list[int], digits_b: list[int], size: int) -> int:
    """Count how many consecutive carries occur during addition."""
    carry: int = 0
    max_chain: int = 0
    current_chain: int = 0
    i: int = 0
    while i < size:
        total: int = digits_a[i] + digits_b[i] + carry
        carry = total // 10
        if carry > 0:
            current_chain = current_chain + 1
            if current_chain > max_chain:
                max_chain = current_chain
        else:
            current_chain = 0
        i = i + 1
    return max_chain


def ripple_carry_sum(values: list[int], count: int) -> int:
    """Sum a list of integers using ripple carry style accumulation."""
    total: int = 0
    i: int = 0
    while i < count:
        total = total + values[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test carry addition operations."""
    ok: int = 0

    packed: int = add_with_carry(7, 8, 0)
    digit: int = packed & 0xF
    carry: int = (packed >> 4) & 0xF
    if digit == 5 and carry == 1:
        ok = ok + 1

    a_digits: list[int] = [9, 9, 9]
    b_digits: list[int] = [1, 0, 0]
    tmp_result: list[int] = multi_digit_add(a_digits, b_digits, 3)
    if tmp_result[0] == 0 and tmp_result[1] == 0 and tmp_result[2] == 0 and tmp_result[3] == 1:
        ok = ok + 1

    chain: int = chain_carry_count(a_digits, b_digits, 3)
    if chain == 3:
        ok = ok + 1

    vals: list[int] = [10, 20, 30]
    s: int = ripple_carry_sum(vals, 3)
    if s == 60:
        ok = ok + 1

    return ok
