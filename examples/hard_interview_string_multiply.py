def multiply_strings(num1: str, num2: str) -> str:
    n1: int = len(num1)
    n2: int = len(num2)
    if n1 == 0 or n2 == 0:
        return "0"
    result: list[int] = []
    total: int = n1 + n2
    i: int = 0
    while i < total:
        result.append(0)
        i = i + 1
    i1: int = n1 - 1
    while i1 >= 0:
        d1: int = ord(num1[i1]) - 48
        i2: int = n2 - 1
        while i2 >= 0:
            d2: int = ord(num2[i2]) - 48
            prod: int = d1 * d2
            p1: int = i1 + i2
            p2: int = i1 + i2 + 1
            total_sum: int = prod + result[p2]
            result[p2] = total_sum % 10
            result[p1] = result[p1] + total_sum // 10
            i2 = i2 - 1
        i1 = i1 - 1
    out: str = ""
    k: int = 0
    while k < total:
        v: int = result[k]
        if len(out) == 0 and v == 0:
            k = k + 1
        else:
            out = out + chr(v + 48)
            k = k + 1
    if len(out) == 0:
        return "0"
    return out

def add_strings(num1: str, num2: str) -> str:
    i: int = len(num1) - 1
    j: int = len(num2) - 1
    carry: int = 0
    result: str = ""
    while i >= 0 or j >= 0 or carry > 0:
        total: int = carry
        if i >= 0:
            total = total + ord(num1[i]) - 48
            i = i - 1
        if j >= 0:
            total = total + ord(num2[j]) - 48
            j = j - 1
        result = chr(total % 10 + 48) + result
        carry = total // 10
    if len(result) == 0:
        return "0"
    return result

def subtract_strings(a: str, b: str) -> str:
    la: int = len(a)
    lb: int = len(b)
    if la < lb:
        return "0"
    i: int = la - 1
    j: int = lb - 1
    borrow: int = 0
    result: str = ""
    while i >= 0:
        da: int = ord(a[i]) - 48 - borrow
        db: int = 0
        if j >= 0:
            db = ord(b[j]) - 48
            j = j - 1
        diff: int = da - db
        if diff < 0:
            diff = diff + 10
            borrow = 1
        else:
            borrow = 0
        result = chr(diff + 48) + result
        i = i - 1
    out: str = ""
    k: int = 0
    n: int = len(result)
    started: int = 0
    while k < n:
        if started == 0 and result[k] == "0":
            k = k + 1
        else:
            started = 1
            out = out + result[k]
            k = k + 1
    if len(out) == 0:
        return "0"
    return out

def test_module() -> int:
    passed: int = 0
    r1: str = multiply_strings("123", "456")
    if r1 == "56088":
        passed = passed + 1
    r2: str = multiply_strings("0", "12345")
    if r2 == "0":
        passed = passed + 1
    r3: str = add_strings("999", "1")
    if r3 == "1000":
        passed = passed + 1
    r4: str = subtract_strings("1000", "1")
    if r4 == "999":
        passed = passed + 1
    r5: str = multiply_strings("2", "3")
    if r5 == "6":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
