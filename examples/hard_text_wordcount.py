def split_words(text: str) -> list[str]:
    words: list[str] = []
    current: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        if ch == " ":
            if len(current) > 0:
                words.append(current)
                current = ""
        else:
            current = current + ch
        i = i + 1
    if len(current) > 0:
        words.append(current)
    return words

def count_words(text: str) -> int:
    words: list[str] = split_words(text)
    return len(words)

def count_unique(text: str) -> int:
    words: list[str] = split_words(text)
    seen: dict[str, int] = {}
    i: int = 0
    while i < len(words):
        w: str = words[i]
        seen[w] = 1
        i = i + 1
    return len(seen)

def word_frequency(text: str) -> dict[str, int]:
    words: list[str] = split_words(text)
    freq: dict[str, int] = {}
    i: int = 0
    while i < len(words):
        w: str = words[i]
        if w in freq:
            freq[w] = freq[w] + 1
        else:
            freq[w] = 1
        i = i + 1
    return freq

def test_module() -> int:
    passed: int = 0
    if count_words("hello world foo") == 3:
        passed = passed + 1
    if count_words("") == 0:
        passed = passed + 1
    if count_unique("a b a c b") == 3:
        passed = passed + 1
    freq: dict[str, int] = word_frequency("go go stop")
    if freq["go"] == 2:
        passed = passed + 1
    if freq["stop"] == 1:
        passed = passed + 1
    if count_words("single") == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
