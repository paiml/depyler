def caesar_encrypt(text: str, shift: int) -> str:
    result: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        code: int = ord(ch)
        if code >= 65 and code <= 90:
            new_code: int = ((code - 65 + shift) % 26) + 65
            result = result + chr(new_code)
        elif code >= 97 and code <= 122:
            new_code2: int = ((code - 97 + shift) % 26) + 97
            result = result + chr(new_code2)
        else:
            result = result + ch
        i = i + 1
    return result

def caesar_decrypt(text: str, shift: int) -> str:
    dshift: int = 26 - (shift % 26)
    result: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        code: int = ord(ch)
        if code >= 65 and code <= 90:
            new_code: int = ((code - 65 + dshift) % 26) + 65
            result = result + chr(new_code)
        elif code >= 97 and code <= 122:
            new_code2: int = ((code - 97 + dshift) % 26) + 97
            result = result + chr(new_code2)
        else:
            result = result + ch
        i = i + 1
    return result

def rot13(text: str) -> str:
    result: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        code: int = ord(ch)
        if code >= 65 and code <= 90:
            new_code: int = ((code - 65 + 13) % 26) + 65
            result = result + chr(new_code)
        elif code >= 97 and code <= 122:
            new_code2: int = ((code - 97 + 13) % 26) + 97
            result = result + chr(new_code2)
        else:
            result = result + ch
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    if caesar_encrypt("abc", 3) == "def":
        passed = passed + 1
    if caesar_decrypt("def", 3) == "abc":
        passed = passed + 1
    if rot13("hello") == "uryyb":
        passed = passed + 1
    if caesar_encrypt("xyz", 3) == "abc":
        passed = passed + 1
    if caesar_encrypt("A Z", 1) == "B A":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
