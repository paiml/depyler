def to_upper(ch: str) -> str:
    code: int = ord(ch)
    if code >= 97 and code <= 122:
        return chr(code - 32)
    return ch

def soundex_code(ch: str) -> str:
    u: str = to_upper(ch)
    if u == "B" or u == "F" or u == "P" or u == "V":
        return "1"
    if u == "C" or u == "G" or u == "J" or u == "K" or u == "Q" or u == "S" or u == "X" or u == "Z":
        return "2"
    if u == "D" or u == "T":
        return "3"
    if u == "L":
        return "4"
    if u == "M" or u == "N":
        return "5"
    if u == "R":
        return "6"
    return "0"

def soundex(name: str) -> str:
    if len(name) == 0:
        return "0000"
    result: str = to_upper(name[0])
    prev: str = soundex_code(name[0])
    i: int = 1
    while i < len(name):
        sc: str = soundex_code(name[i])
        if sc != "0" and sc != prev:
            result = result + sc
        prev = sc
        if len(result) == 4:
            return result
        i = i + 1
    while len(result) < 4:
        result = result + "0"
    return result

def same_soundex(a: str, b: str) -> int:
    if soundex(a) == soundex(b):
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    if soundex("Robert") == "R163":
        passed = passed + 1
    if soundex("Rupert") == "R163":
        passed = passed + 1
    if same_soundex("Robert", "Rupert") == 1:
        passed = passed + 1
    if soundex("Ashcraft") == "A226":
        passed = passed + 1
    if soundex("A") == "A000":
        passed = passed + 1
    if soundex("") == "0000":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
