"""Semantic parity test: rotate array left (simplified)."""


def rotate_left_element(a: int, b: int, c: int, d: int, e: int, idx: int) -> int:
    # Return element at index after rotation by 2
    # For [1,2,3,4,5] rotated left by 2 -> [3,4,5,1,2], element at 0 is 3
    if idx == 0:
        return c
    if idx == 1:
        return d
    if idx == 2:
        return e
    if idx == 3:
        return a
    return b


def main() -> None:
    print(rotate_left_element(1, 2, 3, 4, 5, 0))


if __name__ == "__main__":
    main()
