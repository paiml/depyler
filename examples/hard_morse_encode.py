"""Morse code encoding and decoding using lookup dictionaries."""


def char_to_morse(ch: str) -> str:
    """Convert a single character to morse code."""
    lookup: dict[str, int] = {
        "a": 1, "b": 2, "c": 3, "d": 4, "e": 5,
        "f": 6, "g": 7, "h": 8, "i": 9, "j": 10,
        "k": 11, "l": 12, "m": 13, "n": 14, "o": 15,
        "p": 16, "q": 17, "r": 18, "s": 19, "t": 20,
        "u": 21, "v": 22, "w": 23, "x": 24, "y": 25, "z": 26,
    }
    codes: list[str] = [
        "", ".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
        "....", "..", ".---", "-.-", ".-..", "--", "-.",
        "---", ".--.", "--.-", ".-.", "...", "-",
        "..-", "...-", ".--", "-..-", "-.--", "--..",
    ]
    lower: str = ch.lower()
    idx: int = lookup.get(lower, 0)
    if idx == 0:
        return ""
    return codes[idx]


def encode_morse(text: str) -> str:
    """Encode a string to morse code separated by spaces."""
    result: str = ""
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch == " ":
            result = result + " / "
        else:
            code: str = char_to_morse(ch)
            if code != "":
                if result != "" and not result.endswith("/ "):
                    result = result + " "
                result = result + code
        i = i + 1
    return result


def decode_morse(morse: str) -> str:
    """Decode morse code back to text."""
    codes: list[str] = [
        "", ".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
        "....", "..", ".---", "-.-", ".-..", "--", "-.",
        "---", ".--.", "--.-", ".-.", "...", "-",
        "..-", "...-", ".--", "-..-", "-.--", "--..",
    ]
    letters: str = " abcdefghijklmnopqrstuvwxyz"
    result: str = ""
    current: str = ""
    i: int = 0
    length: int = len(morse)
    while i < length:
        ch: str = morse[i]
        if ch == "/":
            result = result + " "
            i = i + 2
        elif ch == " ":
            if current != "":
                j: int = 1
                while j < 27:
                    if codes[j] == current:
                        result = result + letters[j]
                        break
                    j = j + 1
                current = ""
            i = i + 1
        else:
            current = current + ch
            i = i + 1
    if current != "":
        j2: int = 1
        while j2 < 27:
            if codes[j2] == current:
                result = result + letters[j2]
                break
            j2 = j2 + 1
    return result


def test_module() -> int:
    """Test morse code encoding and decoding."""
    passed: int = 0

    r1: str = char_to_morse("s")
    if r1 == "...":
        passed = passed + 1

    r2: str = char_to_morse("o")
    if r2 == "---":
        passed = passed + 1

    r3: str = encode_morse("sos")
    if r3 == "... --- ...":
        passed = passed + 1

    r4: str = encode_morse("hi")
    if r4 == ".... ..":
        passed = passed + 1

    r5: str = decode_morse("... --- ...")
    if r5 == "sos":
        passed = passed + 1

    r6: str = decode_morse(".... ..")
    if r6 == "hi":
        passed = passed + 1

    r7: str = char_to_morse("1")
    if r7 == "":
        passed = passed + 1

    r8: str = encode_morse("a b")
    if r8 == ".- / -...":
        passed = passed + 1

    return passed
