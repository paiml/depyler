def int_to_binary_str(val: int) -> str:
    if val == 0:
        return "0"
    result: str = ""
    v: int = val
    if v < 0:
        v = 0 - v
    while v > 0:
        bit: int = v % 2
        if bit == 1:
            result = "1" + result
        else:
            result = "0" + result
        v = v // 2
    return result

def int_to_hex_str(val: int) -> str:
    if val == 0:
        return "0"
    digits: str = "0123456789abcdef"
    result: str = ""
    v: int = val
    if v < 0:
        v = 0 - v
    while v > 0:
        rem: int = v % 16
        result = digits[rem] + result
        v = v // 16
    return result

def int_to_octal_str(val: int) -> str:
    if val == 0:
        return "0"
    result: str = ""
    v: int = val
    if v < 0:
        v = 0 - v
    while v > 0:
        rem: int = v % 8
        digits: str = "01234567"
        result = digits[rem] + result
        v = v // 8
    return result

def byte_to_bits(val: int) -> list[int]:
    result: list[int] = []
    i: int = 7
    while i >= 0:
        bit: int = (val // (1 * pow_2(i))) % 2
        result.append(bit)
        i = i - 1
    return result

def pow_2(exp: int) -> int:
    result: int = 1
    i: int = 0
    while i < exp:
        result = result * 2
        i = i + 1
    return result

def bits_to_byte(bits: list[int]) -> int:
    result: int = 0
    i: int = 0
    while i < 8:
        result = result * 2 + bits[i]
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    b: str = int_to_binary_str(10)
    if b == "1010":
        passed = passed + 1
    h: str = int_to_hex_str(255)
    if h == "ff":
        passed = passed + 1
    o: str = int_to_octal_str(8)
    if o == "10":
        passed = passed + 1
    p: int = pow_2(3)
    if p == 8:
        passed = passed + 1
    bits: list[int] = [0, 0, 0, 0, 1, 0, 1, 0]
    bv: int = bits_to_byte(bits)
    if bv == 10:
        passed = passed + 1
    return passed
