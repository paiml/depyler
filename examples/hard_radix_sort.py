def get_max_val(arr: list[int]) -> int:
    if len(arr) == 0:
        return 0
    mx: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] > mx:
            mx = arr[i]
        i = i + 1
    return mx


def counting_sort_by_digit(arr: list[int], exp: int) -> list[int]:
    n: int = len(arr)
    output: list[int] = []
    count: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    i: int = 0
    while i < n:
        output.append(0)
        i = i + 1
    i = 0
    while i < n:
        idx: int = (arr[i] // exp) % 10
        count[idx] = count[idx] + 1
        i = i + 1
    i = 1
    while i < 10:
        count[i] = count[i] + count[i - 1]
        i = i + 1
    i = n - 1
    while i >= 0:
        idx = (arr[i] // exp) % 10
        count[idx] = count[idx] - 1
        output[count[idx]] = arr[i]
        i = i - 1
    return output


def radix_sort(arr: list[int]) -> list[int]:
    if len(arr) == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    mx: int = get_max_val(result)
    exp: int = 1
    while mx // exp > 0:
        result = counting_sort_by_digit(result, exp)
        exp = exp * 10
    return result


def count_digits(n: int) -> int:
    if n == 0:
        return 1
    count: int = 0
    val: int = n
    while val > 0:
        count = count + 1
        val = val // 10
    return count


def test_module() -> int:
    passed: int = 0
    if radix_sort([170, 45, 75, 90, 802, 24, 2, 66]) == [2, 24, 45, 66, 75, 90, 170, 802]:
        passed = passed + 1
    if radix_sort([]) == []:
        passed = passed + 1
    if radix_sort([5]) == [5]:
        passed = passed + 1
    if radix_sort([3, 1, 2]) == [1, 2, 3]:
        passed = passed + 1
    if get_max_val([3, 7, 2, 9]) == 9:
        passed = passed + 1
    if count_digits(12345) == 5:
        passed = passed + 1
    if count_digits(0) == 1:
        passed = passed + 1
    return passed
