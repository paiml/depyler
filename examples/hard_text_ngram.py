def split_on_space(text: str) -> list[str]:
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

def char_ngrams(text: str, n: int) -> list[str]:
    result: list[str] = []
    sz: int = len(text)
    i: int = 0
    while i <= sz - n:
        result.append(text[i:i + n])
        i = i + 1
    return result

def word_ngrams(text: str, n: int) -> list[str]:
    words: list[str] = split_on_space(text)
    result: list[str] = []
    sz: int = len(words)
    i: int = 0
    while i <= sz - n:
        gram: str = words[i]
        j: int = 1
        while j < n:
            idx: int = i + j
            gram = gram + " " + words[idx]
            j = j + 1
        result.append(gram)
        i = i + 1
    return result

def count_ngrams(text: str, n: int) -> int:
    grams: list[str] = char_ngrams(text, n)
    return len(grams)

def test_module() -> int:
    passed: int = 0
    c2: list[str] = char_ngrams("abcd", 2)
    if len(c2) == 3:
        passed = passed + 1
    if c2[0] == "ab":
        passed = passed + 1
    w2: list[str] = word_ngrams("a b c d", 2)
    if len(w2) == 3:
        passed = passed + 1
    if w2[0] == "a b":
        passed = passed + 1
    if count_ngrams("hello", 3) == 3:
        passed = passed + 1
    c1: list[str] = char_ngrams("xy", 2)
    if len(c1) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
