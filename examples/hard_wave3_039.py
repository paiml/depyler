"""Text processing: Morse code and phonetic encoding.

Tests: character-to-code mapping, encoding/decoding, dictionary lookup,
string building, delimiter handling.
"""

from typing import Dict, List, Tuple


def char_to_morse(ch: str) -> str:
    """Convert single character to Morse code."""
    c: int = ord(ch)
    if c == ord("a") or c == ord("A"):
        return ".-"
    elif c == ord("b") or c == ord("B"):
        return "-..."
    elif c == ord("c") or c == ord("C"):
        return "-.-."
    elif c == ord("d") or c == ord("D"):
        return "-.."
    elif c == ord("e") or c == ord("E"):
        return "."
    elif c == ord("f") or c == ord("F"):
        return "..-."
    elif c == ord("g") or c == ord("G"):
        return "--."
    elif c == ord("h") or c == ord("H"):
        return "...."
    elif c == ord("i") or c == ord("I"):
        return ".."
    elif c == ord("j") or c == ord("J"):
        return ".---"
    elif c == ord("k") or c == ord("K"):
        return "-.-"
    elif c == ord("l") or c == ord("L"):
        return ".-.."
    elif c == ord("m") or c == ord("M"):
        return "--"
    elif c == ord("n") or c == ord("N"):
        return "-."
    elif c == ord("o") or c == ord("O"):
        return "---"
    elif c == ord("p") or c == ord("P"):
        return ".--."
    elif c == ord("q") or c == ord("Q"):
        return "--.-"
    elif c == ord("r") or c == ord("R"):
        return ".-."
    elif c == ord("s") or c == ord("S"):
        return "..."
    elif c == ord("t") or c == ord("T"):
        return "-"
    elif c == ord("u") or c == ord("U"):
        return "..-"
    elif c == ord("v") or c == ord("V"):
        return "...-"
    elif c == ord("w") or c == ord("W"):
        return ".--"
    elif c == ord("x") or c == ord("X"):
        return "-..-"
    elif c == ord("y") or c == ord("Y"):
        return "-.--"
    elif c == ord("z") or c == ord("Z"):
        return "--.."
    return ""


def encode_morse(text: str) -> str:
    """Encode text to Morse code with space separators."""
    result: List[str] = []
    first: bool = True
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        if ch == " ":
            result.append(" / ")
            first = True
        else:
            mc: str = char_to_morse(ch)
            if len(mc) > 0:
                if not first:
                    result.append(" ")
                result.append(mc)
                first = False
        i += 1
    return "".join(result)


def soundex_code(ch: str) -> str:
    """Get Soundex code for a character."""
    c: int = ord(ch)
    if c == ord("b") or c == ord("f") or c == ord("p") or c == ord("v"):
        return "1"
    elif c == ord("c") or c == ord("g") or c == ord("j") or c == ord("k") or c == ord("q") or c == ord("s") or c == ord("x") or c == ord("z"):
        return "2"
    elif c == ord("d") or c == ord("t"):
        return "3"
    elif c == ord("l"):
        return "4"
    elif c == ord("m") or c == ord("n"):
        return "5"
    elif c == ord("r"):
        return "6"
    return "0"


def soundex(name: str) -> str:
    """Compute Soundex encoding of a name."""
    if len(name) == 0:
        return ""
    result: List[str] = []
    first: str = name[0]
    if first >= "a" and first <= "z":
        code: int = ord(first) - ord("a") + ord("A")
        result.append(chr(code))
    else:
        result.append(first)
    lower_first: str = name[0]
    if lower_first >= "A" and lower_first <= "Z":
        lower_first = chr(ord(lower_first) - ord("A") + ord("a"))
    prev_code: str = soundex_code(lower_first)
    i: int = 1
    while i < len(name) and len(result) < 4:
        lower_ch: str = name[i]
        if lower_ch >= "A" and lower_ch <= "Z":
            lower_ch = chr(ord(lower_ch) - ord("A") + ord("a"))
        sc: str = soundex_code(lower_ch)
        if sc != "0" and sc != prev_code:
            result.append(sc)
        prev_code = sc
        i += 1
    while len(result) < 4:
        result.append("0")
    return "".join(result)


def test_morse() -> bool:
    """Test Morse code encoding."""
    ok: bool = True
    sos: str = encode_morse("SOS")
    if sos != "... --- ...":
        ok = False
    sx: str = soundex("Robert")
    if sx != "R163":
        ok = False
    return ok
