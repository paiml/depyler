def count_primes(n: int) -> int:
    if n <= 2:
        return 0
    sieve: list[int] = []
    i: int = 0
    while i < n:
        sieve.append(1)
        i = i + 1
    sieve[0] = 0
    sieve[1] = 0
    p: int = 2
    while p * p < n:
        if sieve[p] == 1:
            m: int = p * p
            while m < n:
                sieve[m] = 0
                m = m + p
        p = p + 1
    cnt: int = 0
    j: int = 0
    while j < n:
        cnt = cnt + sieve[j]
        j = j + 1
    return cnt

def is_prime(n: int) -> int:
    if n < 2:
        return 0
    if n == 2:
        return 1
    if n % 2 == 0:
        return 0
    d: int = 3
    while d * d <= n:
        if n % d == 0:
            return 0
        d = d + 2
    return 1

def count_twin_primes(n: int) -> int:
    if n < 5:
        return 0
    sieve: list[int] = []
    i: int = 0
    while i < n:
        sieve.append(1)
        i = i + 1
    sieve[0] = 0
    sieve[1] = 0
    p: int = 2
    while p * p < n:
        if sieve[p] == 1:
            m: int = p * p
            while m < n:
                sieve[m] = 0
                m = m + p
        p = p + 1
    cnt: int = 0
    j: int = 2
    while j < n - 2:
        if sieve[j] == 1:
            twin: int = j + 2
            if twin < n and sieve[twin] == 1:
                cnt = cnt + 1
        j = j + 1
    return cnt

def nth_prime(n: int) -> int:
    if n <= 0:
        return 0
    cnt: int = 0
    num: int = 2
    while cnt < n:
        chk: int = is_prime(num)
        if chk == 1:
            cnt = cnt + 1
            if cnt == n:
                return num
        num = num + 1
    return 0

def test_module() -> int:
    passed: int = 0
    r1: int = count_primes(10)
    if r1 == 4:
        passed = passed + 1
    r2: int = count_primes(30)
    if r2 == 10:
        passed = passed + 1
    r3: int = is_prime(17)
    if r3 == 1:
        passed = passed + 1
    r4: int = count_twin_primes(20)
    if r4 == 4:
        passed = passed + 1
    r5: int = nth_prime(5)
    if r5 == 11:
        passed = passed + 1
    r6: int = is_prime(1)
    if r6 == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
