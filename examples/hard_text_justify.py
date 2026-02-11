def pad_right(text: str, width: int) -> str:
    result: str = text
    while len(result) < width:
        result = result + " "
    return result

def pad_left(text: str, width: int) -> str:
    result: str = text
    while len(result) < width:
        result = " " + result
    return result

def center_text(text: str, width: int) -> str:
    sz: int = len(text)
    if sz >= width:
        return text
    total_pad: int = width - sz
    left_pad: int = total_pad // 2
    right_pad: int = total_pad - left_pad
    result: str = ""
    i: int = 0
    while i < left_pad:
        result = result + " "
        i = i + 1
    result = result + text
    j: int = 0
    while j < right_pad:
        result = result + " "
        j = j + 1
    return result

def repeat_char(ch: str, count: int) -> str:
    result: str = ""
    i: int = 0
    while i < count:
        result = result + ch
        i = i + 1
    return result

def box_text(text: str) -> str:
    sz: int = len(text)
    border: str = "+" + repeat_char("-", sz + 2) + "+"
    middle: str = "| " + text + " |"
    return border + "\n" + middle + "\n" + border

def test_module() -> int:
    passed: int = 0
    if len(pad_right("hi", 5)) == 5:
        passed = passed + 1
    if len(pad_left("hi", 5)) == 5:
        passed = passed + 1
    if len(center_text("hi", 6)) == 6:
        passed = passed + 1
    if pad_right("ab", 4) == "ab  ":
        passed = passed + 1
    if pad_left("ab", 4) == "  ab":
        passed = passed + 1
    if repeat_char("x", 3) == "xxx":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
