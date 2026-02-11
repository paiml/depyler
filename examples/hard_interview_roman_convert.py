def roman_to_int(s: str) -> int:
    result: int = 0
    n: int = len(s)
    i: int = 0
    while i < n:
        ch: str = s[i]
        val: int = roman_char_value(ch)
        if i + 1 < n:
            nxt_ch: str = s[i + 1]
            nxt_val: int = roman_char_value(nxt_ch)
            if val < nxt_val:
                result = result + nxt_val - val
                i = i + 2
            else:
                result = result + val
                i = i + 1
        else:
            result = result + val
            i = i + 1
    return result

def roman_char_value(ch: str) -> int:
    if ch == "I":
        return 1
    if ch == "V":
        return 5
    if ch == "X":
        return 10
    if ch == "L":
        return 50
    if ch == "C":
        return 100
    if ch == "D":
        return 500
    if ch == "M":
        return 1000
    return 0

def int_to_roman(num: int) -> str:
    result: str = ""
    vals: list[int] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]
    syms: list[str] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"]
    n: int = num
    i: int = 0
    nv: int = len(vals)
    while i < nv:
        v: int = vals[i]
        sym: str = syms[i]
        while n >= v:
            result = result + sym
            n = n - v
        i = i + 1
    return result

def is_valid_roman(s: str) -> int:
    n: int = len(s)
    i: int = 0
    while i < n:
        ch: str = s[i]
        v: int = roman_char_value(ch)
        if v == 0:
            return 0
        i = i + 1
    converted: int = roman_to_int(s)
    back: str = int_to_roman(converted)
    if back == s:
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    r1: int = roman_to_int("III")
    if r1 == 3:
        passed = passed + 1
    r2: int = roman_to_int("LVIII")
    if r2 == 58:
        passed = passed + 1
    r3: int = roman_to_int("MCMXCIV")
    if r3 == 1994:
        passed = passed + 1
    r4: str = int_to_roman(1994)
    if r4 == "MCMXCIV":
        passed = passed + 1
    r5: str = int_to_roman(58)
    if r5 == "LVIII":
        passed = passed + 1
    r6: int = is_valid_roman("XIV")
    if r6 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
