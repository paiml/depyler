def decode_ways(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    first_char: str = s[0]
    if first_char == "0":
        return 0
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 1
    j: int = 2
    while j <= n:
        one_digit: int = ord(s[j - 1]) - 48
        if one_digit >= 1:
            dp[j] = dp[j] + dp[j - 1]
        tens_char: int = ord(s[j - 2]) - 48
        two_digit: int = tens_char * 10 + one_digit
        if two_digit >= 10 and two_digit <= 26:
            dp[j] = dp[j] + dp[j - 2]
        j = j + 1
    return dp[n]

def decode_ways_memo(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    memo: list[int] = []
    i: int = 0
    while i <= n:
        memo.append(0 - 1)
        i = i + 1
    return decode_helper(s, 0, memo)

def decode_helper(s: str, idx: int, memo: list[int]) -> int:
    n: int = len(s)
    if idx == n:
        return 1
    if s[idx] == "0":
        return 0
    if memo[idx] != 0 - 1:
        return memo[idx]
    ways: int = decode_helper(s, idx + 1, memo)
    if idx + 1 < n:
        tens: int = ord(s[idx]) - 48
        ones: int = ord(s[idx + 1]) - 48
        two_digit: int = tens * 10 + ones
        if two_digit <= 26:
            ways = ways + decode_helper(s, idx + 2, memo)
    memo[idx] = ways
    return ways

def count_valid_encodings(num: int) -> int:
    if num <= 0:
        return 0
    s: str = ""
    n: int = num
    while n > 0:
        digit: int = n % 10
        s = chr(digit + 48) + s
        n = n // 10
    return decode_ways(s)

def test_module() -> int:
    passed: int = 0
    r1: int = decode_ways("12")
    if r1 == 2:
        passed = passed + 1
    r2: int = decode_ways("226")
    if r2 == 3:
        passed = passed + 1
    r3: int = decode_ways("06")
    if r3 == 0:
        passed = passed + 1
    r4: int = decode_ways_memo("226")
    if r4 == 3:
        passed = passed + 1
    r5: int = count_valid_encodings(12)
    if r5 == 2:
        passed = passed + 1
    r6: int = decode_ways("10")
    if r6 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
