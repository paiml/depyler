"""Semantic parity test: palindrome check."""


def is_palindrome(s: str) -> bool:
    left = 0
    right = len(s) - 1
    while left < right:
        if s[left] != s[right]:
            return False
        left = left + 1
        right = right - 1
    return True


def main() -> None:
    if is_palindrome("racecar"):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
