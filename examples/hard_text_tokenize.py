def is_alpha_char(c: str) -> int:
    code: int = ord(c)
    if code >= 65 and code <= 90:
        return 1
    if code >= 97 and code <= 122:
        return 1
    return 0

def is_digit_char(c: str) -> int:
    code: int = ord(c)
    if code >= 48 and code <= 57:
        return 1
    return 0

def tokenize(text: str) -> list[str]:
    tokens: list[str] = []
    current: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        if is_alpha_char(ch) == 1 or is_digit_char(ch) == 1:
            current = current + ch
        else:
            if len(current) > 0:
                tokens.append(current)
                current = ""
        i = i + 1
    if len(current) > 0:
        tokens.append(current)
    return tokens

def count_tokens(text: str) -> int:
    tokens: list[str] = tokenize(text)
    return len(tokens)

def test_module() -> int:
    passed: int = 0
    t1: list[str] = tokenize("hello world")
    if len(t1) == 2:
        passed = passed + 1
    t2: list[str] = tokenize("one,two,three")
    if len(t2) == 3:
        passed = passed + 1
    t3: list[str] = tokenize("  spaces  everywhere  ")
    if len(t3) == 2:
        passed = passed + 1
    t4: list[str] = tokenize("")
    if len(t4) == 0:
        passed = passed + 1
    if count_tokens("a b c d e") == 5:
        passed = passed + 1
    t5: list[str] = tokenize("hello123world")
    if len(t5) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
