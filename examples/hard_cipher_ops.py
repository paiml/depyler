def caesar_encrypt(data: list[int], shift: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(data):
        val: int = (data[i] + shift) % 26
        result.append(val)
        i = i + 1
    return result


def caesar_decrypt(data: list[int], shift: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(data):
        val: int = (data[i] - shift + 26) % 26
        result.append(val)
        i = i + 1
    return result


def vigenere_encrypt(data: list[int], key: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    kl: int = len(key)
    while i < len(data):
        shift: int = key[i % kl]
        val: int = (data[i] + shift) % 26
        result.append(val)
        i = i + 1
    return result


def vigenere_decrypt(data: list[int], key: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    kl: int = len(key)
    while i < len(data):
        shift: int = key[i % kl]
        val: int = (data[i] - shift + 26) % 26
        result.append(val)
        i = i + 1
    return result


def atbash(data: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(data):
        result.append(25 - data[i])
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    enc: list[int] = caesar_encrypt([0, 1, 2], 3)
    if enc == [3, 4, 5]:
        passed = passed + 1
    dec: list[int] = caesar_decrypt([3, 4, 5], 3)
    if dec == [0, 1, 2]:
        passed = passed + 1
    if caesar_encrypt([25], 1) == [0]:
        passed = passed + 1
    ve: list[int] = vigenere_encrypt([0, 1, 2, 3], [1, 2])
    if ve == [1, 3, 3, 5]:
        passed = passed + 1
    vd: list[int] = vigenere_decrypt([1, 3, 3, 5], [1, 2])
    if vd == [0, 1, 2, 3]:
        passed = passed + 1
    ab: list[int] = atbash([0, 25, 12])
    if ab == [25, 0, 13]:
        passed = passed + 1
    if atbash(atbash([5, 10, 15])) == [5, 10, 15]:
        passed = passed + 1
    return passed
