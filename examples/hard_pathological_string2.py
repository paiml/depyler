# Pathological string: String method chains (upper, lower, strip, replace)
# Tests: cascaded string transformations, method calls on results


def normalize_text(text: str) -> str:
    """Normalize: strip, lowercase, replace multiple spaces with single."""
    trimmed: str = text.strip()
    lowered: str = trimmed.lower()
    # Replace double spaces with single (iterate until stable)
    result: str = lowered
    prev: str = ""
    while result != prev:
        prev = result
        result = result.replace("  ", " ")
    return result


def title_case_word(word: str) -> str:
    """Convert first char to upper, rest to lower."""
    if len(word) == 0:
        return ""
    first: str = word[0]
    upper_first: str = first.upper()
    if len(word) == 1:
        return upper_first
    rest: str = word[1:]
    lower_rest: str = rest.lower()
    return upper_first + lower_rest


def clean_and_transform(text: str) -> str:
    """Strip whitespace, replace tabs with spaces, lowercase."""
    step1: str = text.strip()
    step2: str = step1.replace("\t", " ")
    step3: str = step2.lower()
    step4: str = step3.replace("  ", " ")
    return step4


def repeat_string(s: str, times: int) -> str:
    """Repeat a string n times."""
    result: str = ""
    i: int = 0
    while i < times:
        result = result + s
        i = i + 1
    return result


def pad_right(text: str, width: int) -> str:
    """Pad text with spaces on the right to reach width."""
    result: str = text
    while len(result) < width:
        result = result + " "
    return result


def test_module() -> int:
    passed: int = 0
    # Test 1: normalize
    if normalize_text("  Hello   World  ") == "hello world":
        passed = passed + 1
    # Test 2: title case
    if title_case_word("hELLO") == "Hello":
        passed = passed + 1
    # Test 3: empty title case
    if title_case_word("") == "":
        passed = passed + 1
    # Test 4: clean and transform
    if clean_and_transform("  HELLO  WORLD  ") == "hello world":
        passed = passed + 1
    # Test 5: repeat
    if repeat_string("ab", 3) == "ababab":
        passed = passed + 1
    # Test 6: pad right
    r: str = pad_right("hi", 5)
    if len(r) == 5:
        passed = passed + 1
    # Test 7: single char title
    if title_case_word("a") == "A":
        passed = passed + 1
    return passed
