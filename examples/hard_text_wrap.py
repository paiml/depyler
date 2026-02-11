def split_words(text: str) -> list[str]:
    words: list[str] = []
    cur: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        if ch == " ":
            if len(cur) > 0:
                words.append(cur)
                cur = ""
        else:
            cur = cur + ch
        i = i + 1
    if len(cur) > 0:
        words.append(cur)
    return words

def word_wrap(text: str, width: int) -> list[str]:
    words: list[str] = split_words(text)
    lines: list[str] = []
    current_line: str = ""
    i: int = 0
    while i < len(words):
        w: str = words[i]
        if len(current_line) == 0:
            current_line = w
        elif len(current_line) + 1 + len(w) <= width:
            current_line = current_line + " " + w
        else:
            lines.append(current_line)
            current_line = w
        i = i + 1
    if len(current_line) > 0:
        lines.append(current_line)
    return lines

def count_lines(text: str, width: int) -> int:
    lines: list[str] = word_wrap(text, width)
    return len(lines)

def max_line_length(text: str, width: int) -> int:
    lines: list[str] = word_wrap(text, width)
    mx: int = 0
    i: int = 0
    while i < len(lines):
        ln: int = len(lines[i])
        if ln > mx:
            mx = ln
        i = i + 1
    return mx

def test_module() -> int:
    passed: int = 0
    lines: list[str] = word_wrap("the quick brown fox jumps", 10)
    if len(lines) == 3:
        passed = passed + 1
    if lines[0] == "the quick":
        passed = passed + 1
    if count_lines("hello world", 20) == 1:
        passed = passed + 1
    if count_lines("a b c d e f", 3) == 4:
        passed = passed + 1
    if max_line_length("hello world foo bar", 10) <= 10:
        passed = passed + 1
    if count_lines("", 10) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
