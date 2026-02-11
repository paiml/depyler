def reverse_words(s: str) -> str:
    words: list[str] = []
    n: int = len(s)
    i: int = 0
    current: str = ""
    while i < n:
        ch: str = s[i]
        if ch == " ":
            if len(current) > 0:
                words.append(current)
                current = ""
        else:
            current = current + ch
        i = i + 1
    if len(current) > 0:
        words.append(current)
    result: str = ""
    j: int = len(words) - 1
    while j >= 0:
        if j < len(words) - 1:
            result = result + " "
        result = result + words[j]
        j = j - 1
    return result

def reverse_each_word(s: str) -> str:
    words: list[str] = []
    n: int = len(s)
    i: int = 0
    current: str = ""
    while i < n:
        ch: str = s[i]
        if ch == " ":
            if len(current) > 0:
                words.append(current)
                current = ""
        else:
            current = current + ch
        i = i + 1
    if len(current) > 0:
        words.append(current)
    result: str = ""
    j: int = 0
    nw: int = len(words)
    while j < nw:
        if j > 0:
            result = result + " "
        w: str = words[j]
        k: int = len(w) - 1
        while k >= 0:
            result = result + w[k]
            k = k - 1
        j = j + 1
    return result

def reverse_vowels(s: str) -> str:
    vowels_str: str = "aeiouAEIOU"
    chars: list[str] = []
    n: int = len(s)
    i: int = 0
    while i < n:
        chars.append(s[i])
        i = i + 1
    left: int = 0
    right: int = n - 1
    while left < right:
        lch: str = chars[left]
        lv: int = is_vowel(lch, vowels_str)
        while left < right and lv == 0:
            left = left + 1
            lch = chars[left]
            lv = is_vowel(lch, vowels_str)
        rch: str = chars[right]
        rv: int = is_vowel(rch, vowels_str)
        while left < right and rv == 0:
            right = right - 1
            rch = chars[right]
            rv = is_vowel(rch, vowels_str)
        if left < right:
            tmp: str = chars[left]
            chars[left] = chars[right]
            chars[right] = tmp
            left = left + 1
            right = right - 1
    result: str = ""
    m: int = 0
    while m < n:
        result = result + chars[m]
        m = m + 1
    return result

def is_vowel(ch: str, vowels_str: str) -> int:
    i: int = 0
    n: int = len(vowels_str)
    while i < n:
        if ch == vowels_str[i]:
            return 1
        i = i + 1
    return 0

def test_module() -> int:
    passed: int = 0
    r1: str = reverse_words("the sky is blue")
    if r1 == "blue is sky the":
        passed = passed + 1
    r2: str = reverse_words("hello")
    if r2 == "hello":
        passed = passed + 1
    r3: str = reverse_each_word("hello world")
    if r3 == "olleh dlrow":
        passed = passed + 1
    r4: str = reverse_vowels("hello")
    if r4 == "holle":
        passed = passed + 1
    r5: str = reverse_words("  a  b  ")
    if r5 == "b a":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
