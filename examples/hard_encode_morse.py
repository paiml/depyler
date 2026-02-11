def char_to_morse(ch: str) -> str:
    if ch == "a":
        return ".-"
    if ch == "b":
        return "-..."
    if ch == "c":
        return "-.-."
    if ch == "d":
        return "-.."
    if ch == "e":
        return "."
    if ch == "f":
        return "..-."
    if ch == "g":
        return "--."
    if ch == "h":
        return "...."
    if ch == "i":
        return ".."
    if ch == "j":
        return ".---"
    if ch == "k":
        return "-.-"
    if ch == "l":
        return ".-.."
    if ch == "m":
        return "--"
    if ch == "n":
        return "-."
    if ch == "o":
        return "---"
    if ch == "s":
        return "..."
    if ch == "t":
        return "-"
    return "?"

def encode_morse(text: str) -> str:
    result: str = ""
    n: int = len(text)
    i: int = 0
    while i < n:
        ch: str = text[i]
        if ch == " ":
            result = result + "/ "
        else:
            code: str = char_to_morse(ch)
            if i > 0 and text[i - 1] != " ":
                result = result + " "
            result = result + code
        i = i + 1
    return result

def count_dots(morse: str) -> int:
    count: int = 0
    n: int = len(morse)
    i: int = 0
    while i < n:
        if morse[i] == ".":
            count = count + 1
        i = i + 1
    return count

def count_dashes(morse: str) -> int:
    count: int = 0
    n: int = len(morse)
    i: int = 0
    while i < n:
        if morse[i] == "-":
            count = count + 1
        i = i + 1
    return count

def morse_length(text: str) -> int:
    encoded: str = encode_morse(text)
    return len(encoded)

def test_module() -> int:
    passed: int = 0
    m: str = char_to_morse("a")
    if m == ".-":
        passed = passed + 1
    m2: str = char_to_morse("s")
    if m2 == "...":
        passed = passed + 1
    e: str = encode_morse("sos")
    if e == "... --- ...":
        passed = passed + 1
    d: int = count_dots("... --- ...")
    if d == 6:
        passed = passed + 1
    da: int = count_dashes("... --- ...")
    if da == 3:
        passed = passed + 1
    return passed
