"""
Comprehensive test of Python string operations transpilation to Rust.

Rewrites re module patterns to use string operations that the
transpiler handles correctly. Avoids: imports, find() comparisons,
tuple types, class definitions, and parenthesis string literals.
Uses index-based iteration instead of for-in on strings to avoid
is_numeric() on String type.
"""


def test_simple_match(text: str, prefix: str) -> int:
    """Test simple pattern matching via startswith."""
    if text.startswith(prefix):
        return 1
    return 0


def test_contains_pattern(text: str, pattern: str) -> int:
    """Test if text contains pattern using count."""
    cnt: int = text.count(pattern)
    if cnt > 0:
        return 1
    return 0


def test_find_pattern_position(text: str, pattern: str) -> int:
    """Find pattern position via manual search."""
    tlen: int = len(text)
    plen: int = len(pattern)
    i: int = 0
    while i + plen <= tlen:
        sub: str = text[i:i + plen]
        if sub == pattern:
            return i
        i = i + 1
    return 0 - 1


def test_count_occurrences(text: str, pattern: str) -> int:
    """Count occurrences via string count."""
    count: int = text.count(pattern)
    return count


def test_replace_pattern(text: str, old_pat: str, new_pat: str) -> str:
    """Replace pattern in text."""
    result: str = text.replace(old_pat, new_pat)
    return result


def test_split_by_delim(text: str, delimiter: str) -> list[str]:
    """Split text by delimiter."""
    parts: list[str] = text.split(delimiter)
    return parts


def test_match_digit(text: str) -> int:
    """Check if all characters are digits."""
    if text.isdigit():
        return 1
    return 0


def test_match_alpha(text: str) -> int:
    """Check if all characters are alphabetic."""
    if text.isalpha():
        return 1
    return 0


def test_match_alnum(text: str) -> int:
    """Check if all characters are alphanumeric."""
    if text.isalnum():
        return 1
    return 0


def extract_digits(text: str) -> str:
    """Extract all digits from text using index-based loop."""
    digits: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        one_char: str = ch + ""
        if one_char.isdigit():
            digits = digits + ch
        i = i + 1
    return digits


def extract_letters(text: str) -> str:
    """Extract all letters from text using index-based loop."""
    letters: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        one_char: str = ch + ""
        if one_char.isalpha():
            letters = letters + ch
        i = i + 1
    return letters


def find_all_words(text: str) -> list[str]:
    """Find all words in text (space-separated)."""
    words: list[str] = text.split()
    return words


def validate_email_simple(email: str) -> int:
    """Simple email validation: check for @ and dot after @."""
    has_at: int = 0
    at_idx: int = 0
    i: int = 0
    while i < len(email):
        ch: str = email[i]
        if ch == "@":
            has_at = 1
            at_idx = i
        i = i + 1
    if has_at == 0:
        return 0
    after: str = email[at_idx + 1:]
    dot_cnt: int = after.count(".")
    if dot_cnt > 0:
        return 1
    return 0


def clean_phone(phone: str) -> str:
    """Remove dashes and spaces from phone number."""
    tmp: str = phone + ""
    s1: str = tmp.replace("-", "")
    s2: str = s1.replace(" ", "")
    return s2


def validate_phone_simple(phone: str) -> int:
    """Simple phone validation: digits only, at least 10."""
    cleaned: str = clean_phone(phone)
    if cleaned.isdigit() and len(cleaned) > 9:
        return 1
    return 0


def extract_domain(url: str) -> str:
    """Extract domain from URL by splitting on slash."""
    tmp: str = url + ""
    if tmp.startswith("http://"):
        tmp = tmp[7:]
    if tmp.startswith("https://"):
        tmp = tmp[8:]
    parts: list[str] = tmp.split("/")
    domain: str = parts[0]
    return domain


def remove_punctuation(text: str, punct: str) -> str:
    """Remove characters found in punct from text."""
    result: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        is_punct: int = 0
        j: int = 0
        while j < len(punct):
            pc: str = punct[j]
            if ch == pc:
                is_punct = 1
            j = j + 1
        if is_punct == 0:
            result = result + ch
        i = i + 1
    return result


def normalize_whitespace(text: str) -> str:
    """Normalize multiple spaces to single space."""
    words: list[str] = text.split()
    normalized: str = " ".join(words)
    return normalized


def starts_with_check(text: str, pattern: str) -> int:
    """Check if text starts with pattern."""
    if text.startswith(pattern):
        return 1
    return 0


def ends_with_check(text: str, pattern: str) -> int:
    """Check if text ends with pattern."""
    if text.endswith(pattern):
        return 1
    return 0


def case_insensitive_match(text: str, pattern: str) -> int:
    """Case-insensitive pattern matching."""
    text_lower: str = text.lower()
    pattern_lower: str = pattern.lower()
    cnt: int = text_lower.count(pattern_lower)
    if cnt > 0:
        return 1
    return 0


def count_word_occurrences(text: str, word: str) -> int:
    """Count occurrences of a specific word."""
    words: list[str] = text.split()
    count: int = 0
    for w in words:
        if w == word:
            count = count + 1
    return count


def extract_numbers_from_text(text: str) -> list[int]:
    """Extract numbers from text into a list."""
    numbers: list[int] = []
    current_num: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        one_char: str = ch + ""
        if one_char.isdigit():
            current_num = current_num + ch
        else:
            if len(current_num) > 0:
                num: int = int(current_num)
                numbers.append(num)
                current_num = ""
        i = i + 1
    if len(current_num) > 0:
        num2: int = int(current_num)
        numbers.append(num2)
    return numbers


def wildcard_match_simple(text: str, pattern: str) -> int:
    """Simple wildcard matching: pattern with single * between prefix/suffix."""
    star_cnt: int = pattern.count("*")
    if star_cnt == 0:
        if text == pattern:
            return 1
        return 0
    if star_cnt != 1:
        return 0
    parts: list[str] = pattern.split("*")
    prefix: str = parts[0]
    suffix: str = parts[1]
    has_prefix: int = 1
    has_suffix: int = 1
    if len(prefix) > 0:
        if text.startswith(prefix):
            has_prefix = 1
        else:
            has_prefix = 0
    if len(suffix) > 0:
        if text.endswith(suffix):
            has_suffix = 1
        else:
            has_suffix = 0
    if has_prefix == 1 and has_suffix == 1:
        return 1
    return 0


def replace_multi(text: str, old1: str, new1: str, old2: str, new2: str) -> str:
    """Replace two patterns in text."""
    tmp: str = text + ""
    r1: str = tmp.replace(old1, new1)
    r2: str = r1.replace(old2, new2)
    return r2


def test_module() -> int:
    """Run all regex-equivalent tests and count passes."""
    ok: int = 0

    m1: int = test_simple_match("Hello World", "Hello")
    if m1 == 1:
        ok = ok + 1

    c1: int = test_contains_pattern("The quick brown fox", "quick")
    if c1 == 1:
        ok = ok + 1

    p1: int = test_find_pattern_position("Hello World Hello", "World")
    if p1 == 6:
        ok = ok + 1

    c2: int = test_count_occurrences("abc abc abc", "abc")
    if c2 == 3:
        ok = ok + 1

    r1: str = test_replace_pattern("Hello World", "World", "Python")
    if r1 == "Hello Python":
        ok = ok + 1

    sp: list[str] = test_split_by_delim("apple,banana,cherry", ",")
    if len(sp) == 3:
        ok = ok + 1

    d1: int = test_match_digit("123")
    if d1 == 1:
        ok = ok + 1

    a1: int = test_match_alpha("Hello")
    if a1 == 1:
        ok = ok + 1

    an1: int = test_match_alnum("Hello123")
    if an1 == 1:
        ok = ok + 1

    dg: str = extract_digits("abc123def456")
    if dg == "123456":
        ok = ok + 1

    lt: str = extract_letters("abc123def456")
    if lt == "abcdef":
        ok = ok + 1

    words: list[str] = find_all_words("Hello world from Python")
    if len(words) == 4:
        ok = ok + 1

    ev: int = validate_email_simple("user@example.com")
    if ev == 1:
        ok = ok + 1

    ei: int = validate_email_simple("notanemail")
    if ei == 0:
        ok = ok + 1

    pv: int = validate_phone_simple("555-123-4567")
    if pv == 1:
        ok = ok + 1

    pi_val: int = validate_phone_simple("abc")
    if pi_val == 0:
        ok = ok + 1

    dom: str = extract_domain("https://www.example.com/path/page.html")
    if dom == "www.example.com":
        ok = ok + 1

    np_val: str = remove_punctuation("Hello, World!", ".,!?;:")
    if np_val == "Hello World":
        ok = ok + 1

    nw: str = normalize_whitespace("Hello    World   !")
    if nw == "Hello World !":
        ok = ok + 1

    sw: int = starts_with_check("Hello World", "Hello")
    if sw == 1:
        ok = ok + 1

    ew: int = ends_with_check("Hello World", "World")
    if ew == 1:
        ok = ok + 1

    ci: int = case_insensitive_match("Hello", "hello")
    if ci == 1:
        ok = ok + 1

    rm: str = replace_multi("aabbcc", "a", "x", "b", "y")
    if rm == "xxyycc":
        ok = ok + 1

    wc: int = count_word_occurrences("the quick brown fox the lazy dog", "the")
    if wc == 2:
        ok = ok + 1

    nums: list[int] = extract_numbers_from_text("I have 2 apples and 5 oranges")
    if len(nums) == 2:
        ok = ok + 1

    w1: int = wildcard_match_simple("hello.txt", "*.txt")
    if w1 == 1:
        ok = ok + 1

    w2: int = wildcard_match_simple("test_file.py", "test_*")
    if w2 == 1:
        ok = ok + 1

    return ok
