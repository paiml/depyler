def is_vowel(ch: str) -> int:
    if ch == "a" or ch == "e" or ch == "i" or ch == "o" or ch == "u":
        return 1
    if ch == "A" or ch == "E" or ch == "I" or ch == "O" or ch == "U":
        return 1
    return 0

def pig_latin_word(word: str) -> str:
    if len(word) == 0:
        return ""
    first: str = word[0]
    fv: int = is_vowel(first)
    if fv == 1:
        return word + "yay"
    consonants: str = ""
    idx: int = 0
    while idx < len(word):
        ch: str = word[idx]
        cv: int = is_vowel(ch)
        if cv == 1:
            return word[idx:] + consonants + "ay"
        consonants = consonants + ch
        idx = idx + 1
    return word + "ay"

def pig_latin_sentence(text: str) -> str:
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
    translated: list[str] = []
    j: int = 0
    while j < len(words):
        w: str = words[j]
        if len(w) == 0:
            translated.append("")
        else:
            f2: str = w[0]
            fv2: int = is_vowel(f2)
            if fv2 == 1:
                translated.append(w + "yay")
            else:
                cons: str = ""
                ki: int = 0
                found: int = 0
                while ki < len(w) and found == 0:
                    vc: int = is_vowel(w[ki])
                    if vc == 1:
                        translated.append(w[ki:] + cons + "ay")
                        found = 1
                    else:
                        cons = cons + w[ki]
                    ki = ki + 1
                if found == 0:
                    translated.append(w + "ay")
        j = j + 1
    result: str = ""
    k: int = 0
    while k < len(translated):
        if k > 0:
            result = result + " "
        result = result + translated[k]
        k = k + 1
    return result

def test_module() -> int:
    passed: int = 0
    if pig_latin_word("hello") == "ellohay":
        passed = passed + 1
    if pig_latin_word("apple") == "appleyay":
        passed = passed + 1
    if pig_latin_word("string") == "ingstray":
        passed = passed + 1
    if pig_latin_word("") == "":
        passed = passed + 1
    if pig_latin_word("eat") == "eatyay":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
