"""Basic counter example using pure functions instead of classes."""


def counter_new(value: int) -> list[int]:
    """Create a new counter with the given initial value.

    Returns a list with a single element representing the counter state.
    """
    state: list[int] = [value]
    return state


def counter_increment(state: list[int]) -> list[int]:
    """Increment counter by 1 and return updated state."""
    current: int = state[0]
    new_val: int = current + 1
    result: list[int] = [new_val]
    return result


def counter_get_value(state: list[int]) -> int:
    """Get current counter value."""
    return state[0]


def counter_create_with_value(val: int) -> list[int]:
    """Create counter with initial value."""
    return counter_new(val)


def test_counter() -> int:
    """Test counter operations and return sum of results."""
    c: list[int] = counter_new(0)

    c = counter_increment(c)
    c = counter_increment(c)

    val: int = counter_get_value(c)

    c2: list[int] = counter_create_with_value(10)
    val2: int = counter_get_value(c2)

    return val + val2
