"""Hard enum patterns: IntEnum, Flag, auto, custom init, classmethods, member access."""

from enum import Enum, IntEnum, Flag, auto
from typing import List, Dict, Optional, Tuple, Set


class Direction(Enum):
    """Basic enum with methods."""
    NORTH = "N"
    SOUTH = "S"
    EAST = "E"
    WEST = "W"

    def opposite(self) -> "Direction":
        opposites = {
            Direction.NORTH: Direction.SOUTH,
            Direction.SOUTH: Direction.NORTH,
            Direction.EAST: Direction.WEST,
            Direction.WEST: Direction.EAST,
        }
        return opposites[self]

    def is_vertical(self) -> bool:
        return self in (Direction.NORTH, Direction.SOUTH)

    def dx_dy(self) -> Tuple[int, int]:
        deltas: Dict[Direction, Tuple[int, int]] = {
            Direction.NORTH: (0, -1),
            Direction.SOUTH: (0, 1),
            Direction.EAST: (1, 0),
            Direction.WEST: (-1, 0),
        }
        return deltas[self]


class Priority(IntEnum):
    """IntEnum allowing integer comparisons and arithmetic."""
    CRITICAL = 0
    HIGH = 1
    MEDIUM = 2
    LOW = 3
    DEBUG = 4

    def is_urgent(self) -> bool:
        return self <= Priority.HIGH

    def escalated(self) -> "Priority":
        new_val = max(0, self.value - 1)
        return Priority(new_val)

    @classmethod
    def from_string(cls, name: str) -> "Priority":
        mapping: Dict[str, Priority] = {
            "critical": cls.CRITICAL,
            "high": cls.HIGH,
            "medium": cls.MEDIUM,
            "low": cls.LOW,
            "debug": cls.DEBUG,
        }
        lower = name.lower()
        if lower in mapping:
            return mapping[lower]
        return cls.MEDIUM


class Permission(Flag):
    """Flag enum for bitmask permissions."""
    NONE = 0
    READ = auto()
    WRITE = auto()
    EXECUTE = auto()
    DELETE = auto()
    ADMIN = READ | WRITE | EXECUTE | DELETE

    def can_read(self) -> bool:
        return bool(self & Permission.READ)

    def can_write(self) -> bool:
        return bool(self & Permission.WRITE)

    def can_execute(self) -> bool:
        return bool(self & Permission.EXECUTE)

    def description(self) -> str:
        parts: List[str] = []
        if self & Permission.READ:
            parts.append("r")
        if self & Permission.WRITE:
            parts.append("w")
        if self & Permission.EXECUTE:
            parts.append("x")
        if self & Permission.DELETE:
            parts.append("d")
        return "".join(parts) if parts else "-"


class TokenType(Enum):
    """Enum used in a lexer/parser context."""
    INTEGER = auto()
    FLOAT = auto()
    STRING = auto()
    IDENTIFIER = auto()
    PLUS = auto()
    MINUS = auto()
    MULTIPLY = auto()
    DIVIDE = auto()
    LPAREN = auto()
    RPAREN = auto()
    EOF = auto()

    def is_operator(self) -> bool:
        return self in (
            TokenType.PLUS,
            TokenType.MINUS,
            TokenType.MULTIPLY,
            TokenType.DIVIDE,
        )

    def is_literal(self) -> bool:
        return self in (TokenType.INTEGER, TokenType.FLOAT, TokenType.STRING)

    def precedence(self) -> int:
        prec_map: Dict[TokenType, int] = {
            TokenType.PLUS: 1,
            TokenType.MINUS: 1,
            TokenType.MULTIPLY: 2,
            TokenType.DIVIDE: 2,
        }
        return prec_map.get(self, 0)

    @classmethod
    def from_char(cls, ch: str) -> Optional["TokenType"]:
        char_map: Dict[str, TokenType] = {
            "+": cls.PLUS,
            "-": cls.MINUS,
            "*": cls.MULTIPLY,
            "/": cls.DIVIDE,
            "(": cls.LPAREN,
            ")": cls.RPAREN,
        }
        return char_map.get(ch)


class Season(Enum):
    """Enum with computed properties and complex methods."""
    SPRING = (3, 5)
    SUMMER = (6, 8)
    AUTUMN = (9, 11)
    WINTER = (12, 2)

    def __init__(self, start_month: int, end_month: int) -> None:
        self.start_month = start_month
        self.end_month = end_month

    def contains_month(self, month: int) -> bool:
        if self.start_month <= self.end_month:
            return self.start_month <= month <= self.end_month
        return month >= self.start_month or month <= self.end_month

    def duration_months(self) -> int:
        if self.start_month <= self.end_month:
            return self.end_month - self.start_month + 1
        return (12 - self.start_month + 1) + self.end_month

    @classmethod
    def from_month(cls, month: int) -> "Season":
        for season in cls:
            if season.contains_month(month):
                return season
        return cls.WINTER

    def next_season(self) -> "Season":
        order = [Season.SPRING, Season.SUMMER, Season.AUTUMN, Season.WINTER]
        idx = order.index(self)
        return order[(idx + 1) % 4]


class HttpStatus(IntEnum):
    """HTTP status codes as IntEnum."""
    OK = 200
    CREATED = 201
    NO_CONTENT = 204
    BAD_REQUEST = 400
    UNAUTHORIZED = 401
    FORBIDDEN = 403
    NOT_FOUND = 404
    INTERNAL_ERROR = 500
    BAD_GATEWAY = 502
    SERVICE_UNAVAILABLE = 503

    def is_success(self) -> bool:
        return 200 <= self.value < 300

    def is_client_error(self) -> bool:
        return 400 <= self.value < 500

    def is_server_error(self) -> bool:
        return 500 <= self.value < 600

    def category(self) -> str:
        if self.is_success():
            return "success"
        elif self.is_client_error():
            return "client_error"
        elif self.is_server_error():
            return "server_error"
        return "unknown"

    @classmethod
    def common_errors(cls) -> List["HttpStatus"]:
        return [cls.BAD_REQUEST, cls.NOT_FOUND, cls.INTERNAL_ERROR]


class Token:
    """Token class using TokenType enum."""

    def __init__(self, token_type: TokenType, value: str, position: int) -> None:
        self.token_type = token_type
        self.value = value
        self.position = position

    def __repr__(self) -> str:
        return f"Token({self.token_type.name}, {self.value!r}, pos={self.position})"


def tokenize_expression(expr: str) -> List[Token]:
    """Simple tokenizer using enum-based token types."""
    tokens: List[Token] = []
    i = 0
    while i < len(expr):
        ch = expr[i]
        if ch == " ":
            i += 1
            continue

        maybe_op = TokenType.from_char(ch)
        if maybe_op is not None:
            tokens.append(Token(maybe_op, ch, i))
            i += 1
        elif ch.isdigit():
            start = i
            has_dot = False
            while i < len(expr) and (expr[i].isdigit() or (expr[i] == "." and not has_dot)):
                if expr[i] == ".":
                    has_dot = True
                i += 1
            val = expr[start:i]
            tt = TokenType.FLOAT if has_dot else TokenType.INTEGER
            tokens.append(Token(tt, val, start))
        elif ch.isalpha() or ch == "_":
            start = i
            while i < len(expr) and (expr[i].isalnum() or expr[i] == "_"):
                i += 1
            tokens.append(Token(TokenType.IDENTIFIER, expr[start:i], start))
        else:
            i += 1

    tokens.append(Token(TokenType.EOF, "", len(expr)))
    return tokens


def count_token_types(tokens: List[Token]) -> Dict[str, int]:
    """Count occurrences of each token type."""
    counts: Dict[str, int] = {}
    for token in tokens:
        name = token.token_type.name
        counts[name] = counts.get(name, 0) + 1
    return counts


def check_permissions(user_perms: Permission, required: Permission) -> bool:
    """Check if user has all required permissions."""
    return (user_perms & required) == required


def permission_audit(
    users: Dict[str, Permission],
) -> Dict[str, str]:
    """Audit all user permissions and return descriptions."""
    return {name: perm.description() for name, perm in users.items()}


# Untyped function 1: test inference with enum operations
def navigate_grid(start_x, start_y, directions, steps):
    x, y = start_x, start_y
    path = [(x, y)]
    for direction, step_count in zip(directions, steps):
        dx, dy = direction.dx_dy()
        for _ in range(step_count):
            x += dx
            y += dy
            path.append((x, y))
    return path


# Untyped function 2: test inference with Flag enum
def merge_permissions(perm_list):
    result = Permission.NONE
    for p in perm_list:
        result = result | p
    return result


def build_status_report(codes: List[HttpStatus]) -> Dict[str, List[int]]:
    """Group HTTP status codes by category."""
    report: Dict[str, List[int]] = {
        "success": [],
        "client_error": [],
        "server_error": [],
        "unknown": [],
    }
    for code in codes:
        cat = code.category()
        report[cat].append(code.value)
    return report


def seasonal_calendar(year: int) -> List[Tuple[int, str]]:
    """Build a month-to-season mapping for a year."""
    calendar: List[Tuple[int, str]] = []
    for month in range(1, 13):
        season = Season.from_month(month)
        calendar.append((month, season.name))
    return calendar


def priority_queue_sort(
    items: List[Tuple[str, Priority]],
) -> List[Tuple[str, Priority]]:
    """Sort items by priority (IntEnum allows direct comparison)."""
    n = len(items)
    result = list(items)
    for i in range(n):
        for j in range(i + 1, n):
            if result[j][1] < result[i][1]:
                result[i], result[j] = result[j], result[i]
    return result


def main() -> None:
    # Test Direction enum
    assert Direction.NORTH.opposite() == Direction.SOUTH
    assert Direction.EAST.is_vertical() is False
    dx, dy = Direction.NORTH.dx_dy()
    assert dx == 0 and dy == -1

    # Test Priority IntEnum
    assert Priority.CRITICAL < Priority.LOW
    assert Priority.HIGH.is_urgent()
    assert Priority.from_string("high") == Priority.HIGH
    assert Priority.LOW.escalated() == Priority.MEDIUM

    # Test Permission Flag
    rw = Permission.READ | Permission.WRITE
    assert rw.can_read()
    assert rw.can_write()
    assert not rw.can_execute()
    assert rw.description() == "rw"
    assert check_permissions(Permission.ADMIN, rw)

    # Test TokenType and tokenizer
    tokens = tokenize_expression("3 + 4.5 * x")
    assert tokens[0].token_type == TokenType.INTEGER
    assert tokens[2].token_type == TokenType.FLOAT
    counts = count_token_types(tokens)
    assert counts.get("PLUS", 0) == 1

    # Test Season
    assert Season.SUMMER.contains_month(7)
    assert Season.WINTER.contains_month(1)
    assert Season.from_month(7) == Season.SUMMER
    assert Season.SPRING.next_season() == Season.SUMMER

    # Test HttpStatus
    assert HttpStatus.OK.is_success()
    assert HttpStatus.NOT_FOUND.is_client_error()
    report = build_status_report([HttpStatus.OK, HttpStatus.NOT_FOUND, HttpStatus.INTERNAL_ERROR])
    assert len(report["success"]) == 1

    # Test untyped functions
    path = navigate_grid(0, 0, [Direction.NORTH, Direction.EAST], [3, 2])
    assert len(path) == 6

    merged = merge_permissions([Permission.READ, Permission.WRITE, Permission.EXECUTE])
    assert merged.can_read()
    assert merged.can_write()
    assert merged.can_execute()

    # Test seasonal calendar
    cal = seasonal_calendar(2024)
    assert len(cal) == 12

    # Test priority sorting
    items = [("task_a", Priority.LOW), ("task_b", Priority.CRITICAL), ("task_c", Priority.MEDIUM)]
    sorted_items = priority_queue_sort(items)
    assert sorted_items[0][1] == Priority.CRITICAL


if __name__ == "__main__":
    main()
