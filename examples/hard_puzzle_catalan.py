def catalan(n: int) -> int:
    if n <= 1:
        return 1
    dp: list[int] = []
    i: int = 0
    while i <= n:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    dp[1] = 1
    j: int = 2
    while j <= n:
        k: int = 0
        while k < j:
            dp[j] = dp[j] + dp[k] * dp[j - 1 - k]
            k = k + 1
        j = j + 1
    return dp[n]

def catalan_recursive(n: int) -> int:
    if n <= 1:
        return 1
    result: int = 0
    i: int = 0
    while i < n:
        result = result + catalan_recursive(i) * catalan_recursive(n - 1 - i)
        i = i + 1
    return result

def num_bst(n: int) -> int:
    return catalan(n)

def balanced_parens_count(n: int) -> int:
    return catalan(n)

def catalan_triangle(n: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i <= n:
        cv: int = catalan(i)
        result.append(cv)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    r1: int = catalan(0)
    if r1 == 1:
        passed = passed + 1
    r2: int = catalan(4)
    if r2 == 14:
        passed = passed + 1
    r3: int = catalan(5)
    if r3 == 42:
        passed = passed + 1
    r4: int = num_bst(3)
    if r4 == 5:
        passed = passed + 1
    r5: list[int] = catalan_triangle(4)
    if r5 == [1, 1, 2, 5, 14]:
        passed = passed + 1
    r6: int = balanced_parens_count(3)
    if r6 == 5:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
