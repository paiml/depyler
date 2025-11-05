"""
Comprehensive test of Python enum module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's enum module
to Rust enums.

Expected Rust mappings:
- Enum -> enum with variants
- IntEnum -> enum with integer discriminants
- auto() -> automatic discriminant assignment

Note: Enum functionality may be simulated with integers for simplicity.
"""

from enum import Enum, IntEnum, auto
from typing import List


class Color(IntEnum):
    """Basic color enumeration"""
    RED = 1
    GREEN = 2
    BLUE = 3


class Status(IntEnum):
    """Status enumeration with auto()"""
    PENDING = auto()
    APPROVED = auto()
    REJECTED = auto()


class Direction(IntEnum):
    """Direction enumeration"""
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3


def test_enum_basic_access() -> int:
    """Test basic enum value access"""
    color: int = Color.RED
    return color


def test_enum_comparison() -> bool:
    """Test enum comparison"""
    color1: int = Color.RED
    color2: int = Color.GREEN

    are_equal: bool = color1 == color2
    are_different: bool = color1 != color2

    return are_different


def test_enum_to_name() -> str:
    """Test getting enum name (simplified)"""
    color: int = Color.BLUE

    # Simplified name mapping
    name: str = ""
    if color == Color.RED:
        name = "RED"
    elif color == Color.GREEN:
        name = "GREEN"
    elif color == Color.BLUE:
        name = "BLUE"

    return name


def test_enum_to_value() -> int:
    """Test getting enum value"""
    color: int = Color.RED
    value: int = color

    return value


def test_enum_from_value(value: int) -> int:
    """Test creating enum from value"""
    # Validate value is valid enum
    if value == Color.RED or value == Color.GREEN or value == Color.BLUE:
        result: int = value
    else:
        result: int = Color.RED  # Default

    return result


def test_status_enum() -> int:
    """Test status enumeration"""
    status: int = Status.PENDING

    # Change status
    if status == Status.PENDING:
        status = Status.APPROVED

    return status


def test_direction_enum() -> int:
    """Test direction enumeration"""
    current: int = Direction.NORTH

    # Rotate clockwise
    if current == Direction.NORTH:
        current = Direction.EAST
    elif current == Direction.EAST:
        current = Direction.SOUTH
    elif current == Direction.SOUTH:
        current = Direction.WEST
    elif current == Direction.WEST:
        current = Direction.NORTH

    return current


def rotate_direction(direction: int, clockwise: bool) -> int:
    """Rotate direction 90 degrees"""
    if clockwise:
        if direction == Direction.NORTH:
            return Direction.EAST
        elif direction == Direction.EAST:
            return Direction.SOUTH
        elif direction == Direction.SOUTH:
            return Direction.WEST
        else:
            return Direction.NORTH
    else:
        # Counter-clockwise
        if direction == Direction.NORTH:
            return Direction.WEST
        elif direction == Direction.WEST:
            return Direction.SOUTH
        elif direction == Direction.SOUTH:
            return Direction.EAST
        else:
            return Direction.NORTH


def opposite_direction(direction: int) -> int:
    """Get opposite direction"""
    if direction == Direction.NORTH:
        return Direction.SOUTH
    elif direction == Direction.SOUTH:
        return Direction.NORTH
    elif direction == Direction.EAST:
        return Direction.WEST
    else:
        return Direction.EAST


def is_horizontal(direction: int) -> bool:
    """Check if direction is horizontal"""
    return direction == Direction.EAST or direction == Direction.WEST


def is_vertical(direction: int) -> bool:
    """Check if direction is vertical"""
    return direction == Direction.NORTH or direction == Direction.SOUTH


def test_enum_iteration() -> List[int]:
    """Test iterating over enum values"""
    colors: List[int] = [Color.RED, Color.GREEN, Color.BLUE]

    return colors


def test_enum_count() -> int:
    """Test counting enum members"""
    colors: List[int] = [Color.RED, Color.GREEN, Color.BLUE]
    count: int = len(colors)

    return count


def color_to_rgb(color: int) -> tuple:
    """Convert color enum to RGB values"""
    if color == Color.RED:
        return (255, 0, 0)
    elif color == Color.GREEN:
        return (0, 255, 0)
    elif color == Color.BLUE:
        return (0, 0, 255)
    else:
        return (0, 0, 0)


def status_to_message(status: int) -> str:
    """Convert status enum to message"""
    if status == Status.PENDING:
        return "Waiting for approval"
    elif status == Status.APPROVED:
        return "Request approved"
    elif status == Status.REJECTED:
        return "Request rejected"
    else:
        return "Unknown status"


def process_by_status(status: int, value: int) -> int:
    """Process value based on status"""
    if status == Status.APPROVED:
        return value * 2
    elif status == Status.REJECTED:
        return 0
    else:
        return value


def test_enum_flags() -> bool:
    """Test enum as flags (bit operations)"""
    # Simplified flag operations
    READ: int = 1
    WRITE: int = 2
    EXECUTE: int = 4

    permissions: int = READ | WRITE

    # Check if has read permission
    has_read: bool = (permissions & READ) != 0

    # Check if has execute permission
    has_execute: bool = (permissions & EXECUTE) != 0

    return has_read and not has_execute


def test_enum_range() -> List[int]:
    """Test enum value ranges"""
    directions: List[int] = []

    for i in range(4):
        directions.append(i)

    return directions


def validate_enum_value(value: int, min_val: int, max_val: int) -> bool:
    """Validate if value is in enum range"""
    is_valid: bool = value >= min_val and value <= max_val
    return is_valid


def test_all_enum_features() -> None:
    """Run all enum module tests"""
    # Basic access
    color: int = test_enum_basic_access()

    # Comparison
    is_different: bool = test_enum_comparison()

    # Name and value
    name: str = test_enum_to_name()
    value: int = test_enum_to_value()
    from_value: int = test_enum_from_value(2)

    # Status tests
    status: int = test_status_enum()
    msg: str = status_to_message(status)

    # Direction tests
    direction: int = test_direction_enum()
    rotated: int = rotate_direction(Direction.NORTH, True)
    opposite: int = opposite_direction(Direction.NORTH)
    is_horiz: bool = is_horizontal(Direction.EAST)
    is_vert: bool = is_vertical(Direction.NORTH)

    # Iteration and counting
    colors: List[int] = test_enum_iteration()
    count: int = test_enum_count()

    # Conversions
    rgb: tuple = color_to_rgb(Color.RED)
    processed: int = process_by_status(Status.APPROVED, 10)

    # Flags
    has_perms: bool = test_enum_flags()

    # Range tests
    dir_range: List[int] = test_enum_range()
    is_valid: bool = validate_enum_value(2, 0, 3)

    print("All enum module tests completed successfully")
