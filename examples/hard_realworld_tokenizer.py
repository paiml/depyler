"""Real-world simple tokenizer for code/text parsing.

Mimics: lexer/scanner stage of compilers, shlex, token-based parsers.
Classifies tokens as numbers, identifiers, operators, strings, whitespace.
"""


def tok_is_digit(ch: str) -> int:
    """Check if character is a digit. Returns 1 if yes."""
    code: int = ord(ch)
    if code >= 48 and code <= 57:
        return 1
    return 0


def tok_is_alpha(ch: str) -> int:
    """Check if character is a letter or underscore. Returns 1 if yes."""
    code: int = ord(ch)
    if code >= 97 and code <= 122:
        return 1
    if code >= 65 and code <= 90:
        return 1
    if ch == "_":
        return 1
    return 0


def tok_is_space(ch: str) -> int:
    """Check if character is whitespace. Returns 1 if yes."""
    if ch == " " or ch == "\t" or ch == "\n":
        return 1
    return 0


def tok_is_op(ch: str) -> int:
    """Check if character is an operator. Returns 1 if yes."""
    if ch == "+" or ch == "-" or ch == "*" or ch == "/":
        return 1
    if ch == "=" or ch == "!" or ch == "&" or ch == "|":
        return 1
    if ch == "<" or ch == ">":
        return 1
    return 0


def tok_scan_number(source: str, start: int) -> list[int]:
    """Scan a number token. Returns [end_pos, token_type]. 0=integer, 1=float."""
    pos: int = start
    has_dot: int = 0
    while pos < len(source):
        if tok_is_digit(source[pos]) == 1:
            pos = pos + 1
        elif source[pos] == "." and has_dot == 0:
            has_dot = 1
            pos = pos + 1
        else:
            pos = pos
            break
    return [pos, has_dot]


def tok_scan_ident(source: str, start: int) -> int:
    """Scan an identifier. Returns end position."""
    pos: int = start
    while pos < len(source) and (tok_is_alpha(source[pos]) == 1 or tok_is_digit(source[pos]) == 1):
        pos = pos + 1
    return pos


def tok_scan_space(source: str, start: int) -> int:
    """Scan whitespace. Returns end position."""
    pos: int = start
    while pos < len(source) and tok_is_space(source[pos]) == 1:
        pos = pos + 1
    return pos


def tok_scan_op(source: str, start: int) -> int:
    """Scan operator (including two-char). Returns end position."""
    pos: int = start + 1
    if pos < len(source):
        ch1: str = source[start]
        ch2: str = source[pos]
        if ch1 == "=" and ch2 == "=":
            return pos + 1
        if ch1 == "!" and ch2 == "=":
            return pos + 1
        if ch1 == "<" and ch2 == "=":
            return pos + 1
        if ch1 == ">" and ch2 == "=":
            return pos + 1
        if ch1 == "&" and ch2 == "&":
            return pos + 1
        if ch1 == "|" and ch2 == "|":
            return pos + 1
    return pos


def tok_extract(source: str, start: int, end: int) -> str:
    """Extract substring from source."""
    result: str = ""
    idx: int = start
    while idx < end:
        result = result + source[idx]
        idx = idx + 1
    return result


def tokenize(source: str) -> list[list[str]]:
    """Tokenize source into [token_text, token_type_name] pairs."""
    tokens: list[list[str]] = []
    pos: int = 0
    while pos < len(source):
        ch: str = source[pos]
        if tok_is_space(ch) == 1:
            end: int = tok_scan_space(source, pos)
            text: str = tok_extract(source, pos, end)
            tokens.append([text, "whitespace"])
            pos = end
        elif tok_is_digit(ch) == 1:
            scan_res: list[int] = tok_scan_number(source, pos)
            end2: int = scan_res[0]
            text2: str = tok_extract(source, pos, end2)
            tokens.append([text2, "number"])
            pos = end2
        elif tok_is_alpha(ch) == 1:
            end3: int = tok_scan_ident(source, pos)
            text3: str = tok_extract(source, pos, end3)
            tokens.append([text3, "ident"])
            pos = end3
        elif tok_is_op(ch) == 1:
            end4: int = tok_scan_op(source, pos)
            text4: str = tok_extract(source, pos, end4)
            tokens.append([text4, "operator"])
            pos = end4
        else:
            tokens.append([ch, "unknown"])
            pos = pos + 1
    return tokens


def count_token_type(tokens: list[list[str]], type_name: str) -> int:
    """Count tokens of a specific type."""
    count: int = 0
    idx: int = 0
    while idx < len(tokens):
        if tokens[idx][1] == type_name:
            count = count + 1
        idx = idx + 1
    return count


def strip_ws_tokens(tokens: list[list[str]]) -> list[list[str]]:
    """Remove whitespace tokens."""
    result: list[list[str]] = []
    idx: int = 0
    while idx < len(tokens):
        if tokens[idx][1] != "whitespace":
            result.append(tokens[idx])
        idx = idx + 1
    return result


def test_module() -> int:
    """Test tokenizer module."""
    passed: int = 0

    # Test 1: char classification
    if tok_is_digit("5") == 1 and tok_is_digit("x") == 0:
        passed = passed + 1

    # Test 2: alpha detection
    if tok_is_alpha("a") == 1 and tok_is_alpha("_") == 1 and tok_is_alpha("5") == 0:
        passed = passed + 1

    # Test 3: tokenize simple expression
    tokens: list[list[str]] = tokenize("x + 42")
    stripped: list[list[str]] = strip_ws_tokens(tokens)
    if len(stripped) == 3:
        passed = passed + 1

    # Test 4: token types
    if stripped[0][1] == "ident" and stripped[1][1] == "operator" and stripped[2][1] == "number":
        passed = passed + 1

    # Test 5: two-char operator
    tokens2: list[list[str]] = tokenize("a == b")
    stripped2: list[list[str]] = strip_ws_tokens(tokens2)
    if stripped2[1][0] == "==":
        passed = passed + 1

    # Test 6: count token types
    tokens3: list[list[str]] = tokenize("x + y * 3")
    if count_token_type(tokens3, "operator") == 2:
        passed = passed + 1

    # Test 7: identifier scanning
    tokens4: list[list[str]] = tokenize("my_var123")
    stripped4: list[list[str]] = strip_ws_tokens(tokens4)
    if len(stripped4) == 1 and stripped4[0][0] == "my_var123":
        passed = passed + 1

    # Test 8: mixed content
    tokens5: list[list[str]] = tokenize("if x >= 10")
    stripped5: list[list[str]] = strip_ws_tokens(tokens5)
    if len(stripped5) == 4:
        passed = passed + 1

    return passed
