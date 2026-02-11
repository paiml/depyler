def to_lower(ch: str) -> str:
    code: int = ord(ch)
    if code >= 65 and code <= 90:
        return chr(code + 32)
    return ch

def is_alpha(ch: str) -> int:
    code: int = ord(ch)
    if code >= 65 and code <= 90:
        return 1
    if code >= 97 and code <= 122:
        return 1
    return 0

def clean_string(text: str) -> str:
    result: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        if is_alpha(ch) == 1:
            result = result + to_lower(ch)
        i = i + 1
    return result

def is_palindrome(text: str) -> int:
    cleaned: str = clean_string(text)
    sz: int = len(cleaned)
    i: int = 0
    while i < sz // 2:
        j: int = sz - 1 - i
        if cleaned[i] != cleaned[j]:
            return 0
        i = i + 1
    return 1

def longest_palindrome_sub(text: str) -> int:
    sz: int = len(text)
    best: int = 0
    i: int = 0
    while i < sz:
        j: int = i
        while j <= sz:
            sub: str = text[i:j]
            slen: int = len(sub)
            if slen > best and is_palindrome(sub) == 1:
                best = slen
            j = j + 1
        i = i + 1
    return best

def test_module() -> int:
    passed: int = 0
    if is_palindrome("racecar") == 1:
        passed = passed + 1
    if is_palindrome("A man a plan a canal Panama") == 1:
        passed = passed + 1
    if is_palindrome("hello") == 0:
        passed = passed + 1
    if is_palindrome("") == 1:
        passed = passed + 1
    if is_palindrome("Was it a car or a cat I saw") == 1:
        passed = passed + 1
    if longest_palindrome_sub("abacaba") == 7:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
